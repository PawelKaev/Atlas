// grammalang-core/src/codegen.rs
// Версия 2.11.0 — ОбновлениеСтруктуры + исправленный field_access + новые образцы match

use crate::ast::*;
use crate::error::Diagnostic;
use crate::resolve::SymbolTable;
use std::collections::HashMap;

// ============ LlvmIr ============

#[derive(Debug, Clone)]
pub enum LlvmIr {
    Модуль { имя: String, функции: Vec<LlvmFunction>, строковые_константы: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct LlvmFunction {
    pub имя: String, pub параметры: Vec<(String, LlvmType)>,
    pub возвращаемый_тип: LlvmType, pub базовые_блоки: Vec<BasicBlock>,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub метка: String, pub инструкции: Vec<LlvmInstruction>, pub терминатор: Terminator,
}

#[derive(Debug, Clone)]
pub enum LlvmInstruction {
    Add { результат: String, тип: LlvmType, левый: String, правый: String },
    Sub { результат: String, тип: LlvmType, левый: String, правый: String },
    Mul { результат: String, тип: LlvmType, левый: String, правый: String },
    SDiv { результат: String, левый: String, правый: String },
    UDiv { результат: String, левый: String, правый: String },
    SRem { результат: String, левый: String, правый: String },
    FAdd { результат: String, левый: String, правый: String },
    FSub { результат: String, левый: String, правый: String },
    FMul { результат: String, левый: String, правый: String },
    FDiv { результат: String, левый: String, правый: String },
    And { результат: String, левый: String, правый: String },
    Or { результат: String, левый: String, правый: String },
    Xor { результат: String, левый: String, правый: String },
    Shl { результат: String, левый: String, правый: String },
    LShr { результат: String, левый: String, правый: String },
    AShr { результат: String, левый: String, правый: String },
    Icmp { результат: String, условие: String, тип: LlvmType, левый: String, правый: String },
    Fcmp { результат: String, условие: String, левый: String, правый: String },
    Load { результат: String, тип: LlvmType, указатель: String },
    Store { тип: LlvmType, значение: String, указатель: String },
    Alloca { результат: String, тип: LlvmType },
    Call { результат: Option<String>, функция: String, аргументы: Vec<(LlvmType, String)> },
    Ret { значение: Option<(LlvmType, String)> },
    Br { метка: String },
    CondBr { условие: String, истина: String, ложь: String },
    Phi { результат: String, тип: LlvmType, входящие: Vec<(String, String)> },
    GetElementPtr { результат: String, тип: LlvmType, указатель: String, индексы: Vec<(LlvmType, String)> },
    Bitcast { результат: String, значение: String, из: LlvmType, в: LlvmType },
    ZExt { результат: String, значение: String, из: LlvmType, в: LlvmType },
    SExt { результат: String, значение: String, из: LlvmType, в: LlvmType },
    Select { результат: String, условие: String, истина: String, ложь: String, тип: LlvmType },
    Unreachable,
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Ret(Option<(LlvmType, String)>), Br(String),
    CondBr { условие: String, истина: String, ложь: String }, Unreachable,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LlvmType {
    Void, I1, I8, I32, I64, Double,
    Ptr(Box<LlvmType>), Array(Box<LlvmType>, usize), Struct(Vec<LlvmType>), Named(String),
}

impl LlvmType {
    pub fn to_llvm_string(&self) -> String {
        match self {
            LlvmType::Void => "void".to_string(), LlvmType::I1 => "i1".to_string(), LlvmType::I8 => "i8".to_string(),
            LlvmType::I32 => "i32".to_string(), LlvmType::I64 => "i64".to_string(), LlvmType::Double => "double".to_string(),
            LlvmType::Ptr(t) => format!("{}*", t.to_llvm_string()),
            LlvmType::Array(t, n) => format!("[{} x {}]", n, t.to_llvm_string()),
            LlvmType::Struct(f) => format!("{{ {} }}", f.iter().map(|x| x.to_llvm_string()).collect::<Vec<_>>().join(", ")),
            LlvmType::Named(n) => format!("%{}", n),
        }
    }
    pub fn bit_width(&self) -> usize { match self { LlvmType::I1=>1, LlvmType::I8=>8, LlvmType::I32=>32, LlvmType::I64|LlvmType::Double=>64, _=>64 } }
}

// ============ FnGenContext ============

struct FnGenContext {
    blocks: Vec<BasicBlock>, current_label: String,
    variables: HashMap<String, (String, LlvmType)>,
    last_value: Option<(LlvmType, String)>,
    entry_label: String, function_name: String, params: Vec<(String, LlvmType)>,
}

impl FnGenContext {
    fn new(entry_label: String, function_name: String, params: Vec<(String, LlvmType)>) -> Self {
        FnGenContext { blocks: Vec::new(), current_label: entry_label.clone(), variables: HashMap::new(), last_value: None, entry_label, function_name, params }
    }
    fn start_new_block(&mut self, label: String) { self.current_label = label; }
    fn get_or_create_block(&mut self, label: &str) -> &mut BasicBlock {
        if let Some(pos) = self.blocks.iter().position(|b| b.метка == label) { &mut self.blocks[pos] }
        else { self.blocks.push(BasicBlock { метка: label.to_string(), инструкции: Vec::new(), терминатор: Terminator::Unreachable }); self.blocks.last_mut().unwrap() }
    }
    fn emit(&mut self, instr: LlvmInstruction) { self.get_or_create_block(&self.current_label.clone()).инструкции.push(instr); }
    fn set_terminator(&mut self, term: Terminator) { self.get_or_create_block(&self.current_label.clone()).терминатор = term; }
    fn finish_with_br(&mut self, target: &str) {
        let current = self.current_label.clone();
        let block = self.get_or_create_block(&current);
        if matches!(block.терминатор, Terminator::Unreachable) { block.терминатор = Terminator::Br(target.to_string()); }
    }
    fn finalize(self) -> Vec<BasicBlock> { self.blocks }
}

// ============ Codegen ============

pub struct Codegen {
    module_name: String, functions: Vec<LlvmFunction>, string_constants: Vec<String>,
    errors: Vec<Diagnostic>, next_reg: usize, next_block: usize, next_string: usize,
    fn_ctx: Option<FnGenContext>, symbol_table: Option<SymbolTable>,
    struct_schemas: HashMap<String, Vec<(String, LlvmType)>>,
    monomorphized: HashMap<(String, Vec<LlvmType>), String>, pending_mono: Vec<(String, Ast)>,
}

impl Codegen {
    pub fn new(module_name: &str) -> Self {
        Codegen { module_name: module_name.to_string(), functions: Vec::new(), string_constants: Vec::new(), errors: Vec::new(), next_reg: 0, next_block: 0, next_string: 0, fn_ctx: None, symbol_table: None, struct_schemas: HashMap::new(), monomorphized: HashMap::new(), pending_mono: Vec::new() }
    }
    pub fn set_symbol_table(&mut self, table: SymbolTable) { self.symbol_table = Some(table); }
    pub fn register_struct(&mut self, name: &str, fields: Vec<(String, LlvmType)>) { self.struct_schemas.insert(name.to_string(), fields); }

    pub fn generate(&mut self, ast: &Ast) -> (Option<LlvmIr>, Vec<Diagnostic>) {
        self.generate_node(ast);
        self.generate_pending_monomorphizations();
        let ir = LlvmIr::Модуль { имя: self.module_name.clone(), функции: std::mem::take(&mut self.functions), строковые_константы: std::mem::take(&mut self.string_constants) };
        (Some(ir), std::mem::take(&mut self.errors))
    }

    fn fresh_reg(&mut self) -> String { let id = self.next_reg; self.next_reg += 1; format!("%reg{}", id) }
    fn fresh_block(&mut self) -> String { let id = self.next_block; self.next_block += 1; format!("%block{}", id) }
    fn fresh_string_name(&mut self) -> String { let id = self.next_string; self.next_string += 1; format!("@.str.{}", id) }

    fn add_string_constant(&mut self, s: &str) -> String {
        let name = self.fresh_string_name();
        self.string_constants.push(format!("{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"", name, s.len() + 1, s));
        name
    }

    fn to_llvm_name(&self, atlas_name: &str) -> String {
        if atlas_name == "главная" { "main".to_string() } else { atlas_name.to_string() }
    }

    fn coerce_to(&mut self, value: &str, from: &LlvmType, to: &LlvmType, _signed: bool) -> (LlvmType, String) {
        if from == to { return (to.clone(), value.to_string()); }
        let reg = self.fresh_reg();
        let mut ctx = self.fn_ctx.take().unwrap();
        match (from, to) {
            (LlvmType::I1, LlvmType::I64) => ctx.emit(LlvmInstruction::ZExt { результат: reg.clone(), значение: value.to_string(), из: from.clone(), в: to.clone() }),
            (LlvmType::I32, LlvmType::I64) => ctx.emit(LlvmInstruction::SExt { результат: reg.clone(), значение: value.to_string(), из: from.clone(), в: to.clone() }),
            _ => {}
        }
        self.fn_ctx = Some(ctx);
        (to.clone(), reg)
    }

    fn generate_node(&mut self, node: &Ast) {
        match node {
            Ast::Модуль { объявления, .. } => { for d in объявления { self.generate_node(d); } }
            Ast::ОбъявлениеСтруктуры { имя, поля, .. } => { let fields: Vec<_> = поля.iter().map(|(n,t)| (n.clone(), self.type_to_llvm(t))).collect(); self.register_struct(имя, fields); }
            Ast::ОбъявлениеФункции { имя, параметры, возвращаемый_тип, тело, .. } => {
                let entry_label = self.fresh_block();
                let llvm_params: Vec<_> = параметры.iter().map(|p| (p.имя.clone(), self.type_to_llvm(&p.тип))).collect();
                let llvm_name = self.to_llvm_name(имя);
                let mut ctx = FnGenContext::new(entry_label.clone(), llvm_name.clone(), llvm_params.clone());
                for param in параметры {
                    let lt = self.type_to_llvm(&param.тип);
                    let ptr = self.fresh_reg();
                    ctx.emit(LlvmInstruction::Alloca { результат: ptr.clone(), тип: lt.clone() });
                    ctx.emit(LlvmInstruction::Store { тип: lt.clone(), значение: format!("%{}", param.имя), указатель: ptr.clone() });
                    ctx.variables.insert(param.имя.clone(), (ptr, lt));
                }
                self.fn_ctx = Some(ctx);
                self.generate_node(тело);
                if let Some(ctx) = self.fn_ctx.take() {
                    let blocks = ctx.finalize();
                    self.functions.push(LlvmFunction { имя: llvm_name, параметры: llvm_params, возвращаемый_тип: self.type_to_llvm(&возвращаемый_тип.clone().unwrap_or(Тип::Пустой)), базовые_блоки: blocks });
                }
            }
            Ast::Блок { выражения, .. } => { for e in выражения { self.generate_expr(e); } }
            Ast::БлокОбласти { выражения, последнее, .. } => { for e in выражения { self.generate_expr(e); } if let Some(last) = последнее { self.generate_expr(last); } }
            Ast::Возврат { значение, .. } => {
                if let Some(inner) = значение { self.generate_expr(inner); }
                let val = значение.as_ref().and_then(|v| self.eval_to_reg(v));
                let mut ctx = self.fn_ctx.take().unwrap();
                ctx.set_terminator(Terminator::Ret(val));
                self.fn_ctx = Some(ctx);
            }
            _ => {}
        }
    }

    fn generate_expr(&mut self, node: &Ast) {
        match node {
            Ast::Пусть { имя, значение, .. } | Ast::Присваивание { имя, значение, .. } => {
                if let Some((vt, v)) = self.eval_to_reg(значение) {
                    let mut ctx = self.fn_ctx.take().unwrap();
                    if let Some((existing_ptr, _)) = ctx.variables.get(имя).cloned() {
                        ctx.emit(LlvmInstruction::Store { тип: vt.clone(), значение: v, указатель: existing_ptr });
                    } else {
                        let ptr = self.fresh_reg();
                        ctx.emit(LlvmInstruction::Alloca { результат: ptr.clone(), тип: vt.clone() });
                        ctx.emit(LlvmInstruction::Store { тип: vt.clone(), значение: v, указатель: ptr.clone() });
                        ctx.variables.insert(имя.clone(), (ptr, vt));
                    }
                    self.fn_ctx = Some(ctx);
                }
            }
            Ast::ПрисваиваниеСОперацией { имя, оператор, значение, .. } => {
                if let (Some((lt, lv)), Some((_rt, rv))) = (
                    self.eval_to_reg(&Ast::Переменная { имя: имя.clone(), тип: None, span: crate::token::Span { line: 0, column: 0, offset: 0 } }),
                    self.eval_to_reg(значение),
                ) {
                    let mut ctx = self.fn_ctx.take().unwrap();
                    let res = self.fresh_reg();
                    match оператор {
                        БинарныйОператор::Сложение => ctx.emit(LlvmInstruction::Add { результат: res.clone(), тип: lt.clone(), левый: lv, правый: rv }),
                        БинарныйОператор::Вычитание => ctx.emit(LlvmInstruction::Sub { результат: res.clone(), тип: lt.clone(), левый: lv, правый: rv }),
                        БинарныйОператор::Умножение => ctx.emit(LlvmInstruction::Mul { результат: res.clone(), тип: lt.clone(), левый: lv, правый: rv }),
                        _ => {}
                    };
                    if let Some((ptr, _)) = ctx.variables.get(имя).cloned() {
                        ctx.emit(LlvmInstruction::Store { тип: lt.clone(), значение: res.clone(), указатель: ptr });
                    }
                    ctx.last_value = Some((lt, res));
                    self.fn_ctx = Some(ctx);
                }
            }
            Ast::Если { условие, то, иначе, .. } => self.generate_if(условие, то, иначе.as_deref()),
            Ast::Пока { условие, тело, .. } => self.generate_while(условие, тело),
            Ast::ЦиклПока { условие, тело, .. } => self.generate_while(условие, тело),
            Ast::Цикл { тело, .. } => {
                let true_val = Ast::Литерал { значение: Значение::Булево(true), span: crate::token::Span { line: 0, column: 0, offset: 0 } };
                self.generate_while(&true_val, тело);
            }
            Ast::ЦиклДля { переменная, итератор, тело, .. } => self.generate_for(переменная, итератор, тело),
            Ast::Прервать { .. } => { let mut ctx = self.fn_ctx.take().unwrap(); ctx.set_terminator(Terminator::Br("%exit_loop".to_string())); self.fn_ctx = Some(ctx); }
            Ast::Продолжить { .. } => { let mut ctx = self.fn_ctx.take().unwrap(); ctx.set_terminator(Terminator::Br("%continue_loop".to_string())); self.fn_ctx = Some(ctx); }
            Ast::Сопоставление { значение, ветки, .. } => self.generate_match(значение, ветки),
            Ast::Вызов { функция, аргументы, .. } => self.generate_call(функция, аргументы),
            Ast::ДвоичноеВыражение { левое, оператор, правое, тип, .. } => self.generate_binary(левое, оператор, правое, тип),
            Ast::КонструкторСтруктуры { имя, поля, .. } => self.generate_struct_construction(имя, поля),
            Ast::ДоступКПолю { объект, поле, .. } => self.generate_field_access(объект, поле),
            Ast::ОбновлениеСтруктуры { объект, поля, тип, span: _ } => {
                let поля_типа = match тип {
                    Some(Тип::Запись(fields)) => fields.clone(),
                    _ => {
                        self.errors.push(Diagnostic {
                            kind: crate::error::DiagnosticKind::Ошибка,
                            message: "Обновление структуры: ожидался тип записи".to_string(),
                            span: crate::token::Span { line: 0, column: 0, offset: 0 },
                            hint: None,
                        });
                        return;
                    }
                };
                let (_, исходный_reg) = match self.eval_to_reg(объект) { Some(v) => v, None => return };
                let llvm_типы: Vec<LlvmType> = поля_типа.iter().map(|(_, t)| self.type_to_llvm(t)).collect();
                let struct_type = LlvmType::Struct(llvm_типы);
                let mut обновления: Vec<(String, (LlvmType, String))> = Vec::new();
                for (имя, значение) in поля {
                    if let Some(val) = self.eval_to_reg(значение) { обновления.push((имя.clone(), val)); }
                }
                let mut ctx = self.fn_ctx.take().unwrap();
                let новый_ptr = self.fresh_reg();
                ctx.emit(LlvmInstruction::Alloca { результат: новый_ptr.clone(), тип: struct_type.clone() });
                for (i, (имя_поля, тип_поля)) in поля_типа.iter().enumerate() {
                    let llvm_тип = self.type_to_llvm(тип_поля);
                    let (тип_знач, рег_знач) = if let Some((_, val)) = обновления.iter().find(|(n, _)| n == имя_поля) {
                        val.clone()
                    } else {
                        let src_ptr = self.fresh_reg();
                        ctx.emit(LlvmInstruction::GetElementPtr {
                            результат: src_ptr.clone(), тип: struct_type.clone(), указатель: исходный_reg.clone(),
                            индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, i.to_string())],
                        });
                        let loaded = self.fresh_reg();
                        ctx.emit(LlvmInstruction::Load { результат: loaded.clone(), тип: llvm_тип.clone(), указатель: src_ptr });
                        (llvm_тип, loaded)
                    };
                    let dst_ptr = self.fresh_reg();
                    ctx.emit(LlvmInstruction::GetElementPtr {
                        результат: dst_ptr.clone(), тип: struct_type.clone(), указатель: новый_ptr.clone(),
                        индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, i.to_string())],
                    });
                    ctx.emit(LlvmInstruction::Store { тип: тип_знач, значение: рег_знач, указатель: dst_ptr });
                }
                ctx.last_value = Some((LlvmType::Ptr(Box::new(struct_type)), новый_ptr));
                self.fn_ctx = Some(ctx);
            }
            Ast::КонструкторСуммы { имя, значение, .. } => {
                self.generate_sum_construction(имя, значение.as_deref());
            }
            Ast::Блок { выражения, .. } => { for e in выражения { self.generate_expr(e); } }
            _ => {}
        }
    }

    fn generate_for(&mut self, переменная: &str, _итератор: &Ast, тело: &Ast) {
        let header_l = self.fresh_block(); let body_l = self.fresh_block(); let exit_l = self.fresh_block();
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.get_or_create_block(&header_l); ctx.get_or_create_block(&body_l); ctx.get_or_create_block(&exit_l);
        ctx.finish_with_br(&header_l); self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(header_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        let cond = self.fresh_reg();
        ctx.emit(LlvmInstruction::Icmp { результат: cond.clone(), условие: "ne".into(), тип: LlvmType::I64, левый: "0".to_string(), правый: "1".to_string() });
        ctx.set_terminator(Terminator::CondBr { условие: cond, истина: body_l.clone(), ложь: exit_l.clone() }); self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(body_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        let elem_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::Alloca { результат: elem_ptr.clone(), тип: LlvmType::I64 });
        ctx.variables.insert(переменная.to_string(), (elem_ptr, LlvmType::I64)); self.fn_ctx = Some(ctx);
        self.generate_expr(тело);
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.finish_with_br(&header_l); self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(exit_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.set_terminator(Terminator::Ret(None)); self.fn_ctx = Some(ctx);
    }

    fn generate_if(&mut self, условие: &Ast, то: &Ast, иначе: Option<&Ast>) {
        let then_l = self.fresh_block(); let else_l = self.fresh_block(); let merge_l = self.fresh_block();
        let cond = self.eval_to_reg(условие).map(|(_, v)| v).unwrap_or_else(|| "0".to_string());

        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.get_or_create_block(&then_l); ctx.get_or_create_block(&else_l); ctx.get_or_create_block(&merge_l);
        ctx.set_terminator(Terminator::CondBr { условие: cond, истина: then_l.clone(), ложь: else_l.clone() }); self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(then_l.clone());
        self.generate_expr(то);
        let mut ctx = self.fn_ctx.take().unwrap(); let tv = ctx.last_value.take(); let t_end = ctx.current_label.clone();
        ctx.finish_with_br(&merge_l); self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(else_l.clone());
        let ev = if let Some(e) = иначе { self.generate_expr(e); let mut ctx = self.fn_ctx.take().unwrap(); let val = ctx.last_value.take(); self.fn_ctx = Some(ctx); val } else { None };
        let mut ctx = self.fn_ctx.take().unwrap(); let e_end = ctx.current_label.clone();
        ctx.finish_with_br(&merge_l); self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(merge_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        match (&tv, &ev) {
            (Some((tt, t)), Some((_, e))) => {
                let phi = self.fresh_reg();
                ctx.emit(LlvmInstruction::Phi { результат: phi.clone(), тип: tt.clone(), входящие: vec![(t.clone(), t_end), (e.clone(), e_end)] });
                ctx.last_value = Some((tt.clone(), phi));
            }
            _ => ctx.last_value = None,
        }
        self.fn_ctx = Some(ctx);
    }

    fn generate_while(&mut self, условие: &Ast, тело: &Ast) {
        let header_l = self.fresh_block(); let body_l = self.fresh_block(); let exit_l = self.fresh_block();
        
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.get_or_create_block(&header_l); ctx.get_or_create_block(&body_l); ctx.get_or_create_block(&exit_l);
        ctx.finish_with_br(&header_l); self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(header_l.clone());
        let cond = self.eval_to_reg(условие).map(|(_, v)| v).unwrap_or_else(|| "0".to_string());
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.set_terminator(Terminator::CondBr { условие: cond, истина: body_l.clone(), ложь: exit_l.clone() }); self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(body_l.clone());
        self.generate_expr(тело);
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.finish_with_br(&header_l); self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(exit_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.set_terminator(Terminator::Ret(None)); self.fn_ctx = Some(ctx);
    }

    fn generate_match(&mut self, значение: &Ast, ветки: &[ВеткаСопоставления]) {
        let merge_l = self.fresh_block();
        let (val_type, val_reg) = match self.eval_to_reg(значение) { Some(v) => v, None => return };
        let mut branch_vals: Vec<(String, Option<(LlvmType, String)>)> = Vec::new();
        for (i, ветка) in ветки.iter().enumerate() {
            let is_last = i == ветки.len() - 1;
            let next_test = if is_last { merge_l.clone() } else { self.fresh_block() };
            let body_l = self.fresh_block();
            self.generate_pattern_test(&ветка.образец, &val_type, &val_reg, &body_l, &next_test);
            self.fn_ctx.as_mut().unwrap().start_new_block(body_l.clone());
            self.bind_pattern_vars(&ветка.образец, &val_type, &val_reg);
            if let Some(ref условие) = ветка.условие {
                let guard_val = self.eval_to_reg(условие).map(|(_, v)| v).unwrap_or_else(|| "0".to_string());
                let guard_pass = self.fresh_block();
                let guard_fail = if is_last { merge_l.clone() } else { next_test.clone() };
                let mut ctx = self.fn_ctx.take().unwrap();
                ctx.set_terminator(Terminator::CondBr { условие: guard_val, истина: guard_pass.clone(), ложь: guard_fail }); self.fn_ctx = Some(ctx);
                self.fn_ctx.as_mut().unwrap().start_new_block(guard_pass);
            }
            self.generate_expr(&ветка.тело);
            let mut ctx = self.fn_ctx.take().unwrap(); let bv = ctx.last_value.take(); let b_end = ctx.current_label.clone();
            ctx.finish_with_br(&merge_l); branch_vals.push((b_end, bv)); self.fn_ctx = Some(ctx);
            if !is_last { self.fn_ctx.as_mut().unwrap().start_new_block(next_test.clone()); }
        }
        self.fn_ctx.as_mut().unwrap().start_new_block(merge_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        if !branch_vals.is_empty() && branch_vals.iter().all(|(_, v)| v.is_some()) {
            if let Some((phi_type, _)) = branch_vals[0].1.as_ref() {
                if *phi_type != LlvmType::Void {
                    let phi_reg = self.fresh_reg();
                    let incoming: Vec<_> = branch_vals.iter().map(|(l, v)| { let (_, vv) = v.as_ref().unwrap(); (vv.clone(), l.clone()) }).collect();
                    ctx.emit(LlvmInstruction::Phi { результат: phi_reg.clone(), тип: phi_type.clone(), входящие: incoming });
                    ctx.last_value = Some((phi_type.clone(), phi_reg));
                } else { ctx.last_value = None; }
            }
        } else { ctx.last_value = None; }
        ctx.set_terminator(Terminator::Ret(None));
        self.fn_ctx = Some(ctx);
    }

    fn generate_pattern_test(&mut self, образец: &Образец, _val_type: &LlvmType, val_reg: &str, match_l: &str, next_l: &str) {
        let mut ctx = self.fn_ctx.take().unwrap();
        match образец {
            Образец::Подчёркивание | Образец::Переменная(_) => ctx.set_terminator(Terminator::Br(match_l.to_string())),
            Образец::Литерал(значение) => {
                let (lit_type, lit_val) = self.literal_to_llvm_imm(значение);
                let cmp = self.fresh_reg();
                ctx.emit(LlvmInstruction::Icmp { результат: cmp.clone(), условие: "eq".into(), тип: lit_type, левый: val_reg.to_string(), правый: lit_val });
                ctx.set_terminator(Terminator::CondBr { условие: cmp, истина: match_l.to_string(), ложь: next_l.to_string() });
            }
            Образец::Конструктор { .. } => {
                let sum_type = LlvmType::Struct(vec![LlvmType::I32, LlvmType::I64]);
                let tag_ptr = self.fresh_reg();
                ctx.emit(LlvmInstruction::GetElementPtr {
                    результат: tag_ptr.clone(), тип: sum_type, указатель: val_reg.to_string(),
                    индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, "0".to_string())],
                });
                let loaded_tag = self.fresh_reg();
                ctx.emit(LlvmInstruction::Load { результат: loaded_tag.clone(), тип: LlvmType::I32, указатель: tag_ptr });
                let cmp = self.fresh_reg();
                ctx.emit(LlvmInstruction::Icmp {
                    результат: cmp.clone(), условие: "eq".into(), тип: LlvmType::I32,
                    левый: loaded_tag, правый: "0".to_string(),
                });
                ctx.set_terminator(Terminator::CondBr {
                    условие: cmp, истина: match_l.to_string(), ложь: next_l.to_string(),
                });
            }
            Образец::Структура { .. } => ctx.set_terminator(Terminator::Br(match_l.to_string())),
            Образец::Или(left, right) => {
                let or_next = self.fresh_block();
                self.fn_ctx = Some(ctx);
                self.generate_pattern_test(left, _val_type, val_reg, match_l, &or_next);
                self.fn_ctx.as_mut().unwrap().start_new_block(or_next.clone());
                self.generate_pattern_test(right, _val_type, val_reg, match_l, next_l);
                return;
            }
            Образец::Привязка { образец, .. } => {
                self.fn_ctx = Some(ctx);
                self.generate_pattern_test(образец, _val_type, val_reg, match_l, next_l);
                return;
            }
            _ => ctx.set_terminator(Terminator::Br(next_l.to_string())),
        }
        self.fn_ctx = Some(ctx);
    }

    fn bind_pattern_vars(&mut self, образец: &Образец, val_type: &LlvmType, val_reg: &str) {
        let mut ctx = self.fn_ctx.take().unwrap();
        match образец {
            Образец::Переменная(name) => {
                let ptr = self.fresh_reg();
                ctx.emit(LlvmInstruction::Alloca { результат: ptr.clone(), тип: val_type.clone() });
                ctx.emit(LlvmInstruction::Store { тип: val_type.clone(), значение: val_reg.to_string(), указатель: ptr.clone() });
                ctx.variables.insert(name.clone(), (ptr, val_type.clone()));
            }
            Образец::Привязка { имя, образец } => {
                let ptr = self.fresh_reg();
                ctx.emit(LlvmInstruction::Alloca { результат: ptr.clone(), тип: val_type.clone() });
                ctx.emit(LlvmInstruction::Store { тип: val_type.clone(), значение: val_reg.to_string(), указатель: ptr.clone() });
                ctx.variables.insert(имя.clone(), (ptr, val_type.clone()));
                self.fn_ctx = Some(ctx);
                self.bind_pattern_vars(образец, val_type, val_reg);
                return;
            }
            Образец::Структура { поля, .. } => {
                if let LlvmType::Ptr(inner) = val_type {
                    if let LlvmType::Struct(field_types) = inner.as_ref() {
                        for (i, (_, field_pattern)) in поля.iter().enumerate() {
                            let field_ptr = self.fresh_reg();
                            ctx.emit(LlvmInstruction::GetElementPtr {
                                результат: field_ptr.clone(), тип: *inner.clone(), указатель: val_reg.to_string(),
                                индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, i.to_string())],
                            });
                            let field_type = field_types.get(i).cloned().unwrap_or(LlvmType::I64);
                            let loaded = self.fresh_reg();
                            ctx.emit(LlvmInstruction::Load { результат: loaded.clone(), тип: field_type.clone(), указатель: field_ptr });
                            self.fn_ctx = Some(ctx);
                            self.bind_pattern_vars(field_pattern, &field_type, &loaded);
                            ctx = self.fn_ctx.take().unwrap();
                        }
                    }
                }
            }
            Образец::Конструктор { вложенный, .. } => {
                if let Some(inner_pattern) = вложенный {
                    let sum_type = LlvmType::Struct(vec![LlvmType::I32, LlvmType::I64]);
                    let data_ptr = self.fresh_reg();
                    ctx.emit(LlvmInstruction::GetElementPtr {
                        результат: data_ptr.clone(), тип: sum_type, указатель: val_reg.to_string(),
                        индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, "1".to_string())],
                    });
                    let loaded_data = self.fresh_reg();
                    ctx.emit(LlvmInstruction::Load { результат: loaded_data.clone(), тип: LlvmType::I64, указатель: data_ptr });
                    self.fn_ctx = Some(ctx);
                    self.bind_pattern_vars(inner_pattern, &LlvmType::I64, &loaded_data);
                    return;
                }
            }
            Образец::Или(left, _) => {
                self.fn_ctx = Some(ctx);
                self.bind_pattern_vars(left, val_type, val_reg);
                return;
            }
            _ => {}
        }
        self.fn_ctx = Some(ctx);
    }

    fn generate_call(&mut self, функция: &Ast, аргументы: &[Ast]) {
        if let Ast::Переменная { имя, .. } = функция {
            if имя == "написать" || имя == "консоль.написать" { self.generate_print(аргументы); return; }
        }
        let func_name = match функция { Ast::Переменная { имя, .. } => self.to_llvm_name(имя), _ => return };
        let mut args_buf: Vec<(LlvmType, String)> = Vec::new();
        for arg in аргументы { if let Some(v) = self.eval_to_reg(arg) { args_buf.push(v); } }
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.emit(LlvmInstruction::Call { результат: None, функция: func_name, аргументы: args_buf });
        ctx.last_value = Some((LlvmType::Void, "void".to_string())); self.fn_ctx = Some(ctx);
    }

    fn generate_print(&mut self, аргументы: &[Ast]) {
        let mut buf: Vec<(LlvmType, String)> = Vec::new();
        for arg in аргументы {
            if let Some((vt, v)) = self.eval_to_reg(arg) {
                match &vt {
                    LlvmType::Ptr(_) => buf.push((LlvmType::Ptr(Box::new(LlvmType::I8)), v)),
                    LlvmType::I64 => {
                        let fmt_str = "%lld";
                        let fmt_name = self.add_string_constant(fmt_str);
                        let mut ctx = self.fn_ctx.take().unwrap();
                        let fp = self.fresh_reg();
                        ctx.emit(LlvmInstruction::GetElementPtr {
                            результат: fp.clone(),
                            тип: LlvmType::Array(Box::new(LlvmType::I8), fmt_str.len() + 1),
                            указатель: fmt_name,
                            индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, "0".to_string())],
                        });
                        self.fn_ctx = Some(ctx);
                        buf.push((LlvmType::Ptr(Box::new(LlvmType::I8)), fp)); buf.push((vt, v));
                    }
                    LlvmType::Double => {
                        let fmt_str = "%f";
                        let fmt_name = self.add_string_constant(fmt_str);
                        let mut ctx = self.fn_ctx.take().unwrap();
                        let fp = self.fresh_reg();
                        ctx.emit(LlvmInstruction::GetElementPtr {
                            результат: fp.clone(),
                            тип: LlvmType::Array(Box::new(LlvmType::I8), fmt_str.len() + 1),
                            указатель: fmt_name,
                            индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, "0".to_string())],
                        });
                        self.fn_ctx = Some(ctx);
                        buf.push((LlvmType::Ptr(Box::new(LlvmType::I8)), fp)); buf.push((vt, v));
                    }
                    _ => {}
                }
            }
        }
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.emit(LlvmInstruction::Call { результат: None, функция: "printf".to_string(), аргументы: buf });
        ctx.last_value = Some((LlvmType::Void, "void".to_string())); self.fn_ctx = Some(ctx);
    }

    fn generate_struct_construction(&mut self, _имя: &str, поля: &[(String, Ast)]) {
        let field_types: Vec<LlvmType> = поля.iter().map(|(_, v)| self.eval_to_reg(v).map(|(t, _)| t).unwrap_or(LlvmType::I64)).collect();
        let struct_type = LlvmType::Struct(field_types.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        let struct_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::Alloca { результат: struct_ptr.clone(), тип: struct_type.clone() });
        for (i, (_, value)) in поля.iter().enumerate() {
            if let Some((val_type, val_reg)) = self.eval_to_reg(value) {
                let field_ptr = self.fresh_reg();
                ctx.emit(LlvmInstruction::GetElementPtr { результат: field_ptr.clone(), тип: struct_type.clone(), указатель: struct_ptr.clone(), индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, i.to_string())] });
                ctx.emit(LlvmInstruction::Store { тип: val_type, значение: val_reg, указатель: field_ptr });
            }
        }
        ctx.last_value = Some((LlvmType::Ptr(Box::new(struct_type)), struct_ptr)); self.fn_ctx = Some(ctx);
    }
    fn generate_sum_construction(&mut self, имя_варианта: &str, значение: Option<&Ast>) {
        let tag = 0i64;
        let inner_val = if let Some(val_ast) = значение {
            self.eval_to_reg(val_ast).map(|(_, v)| v).unwrap_or_else(|| "0".to_string())
        } else {
            "0".to_string()
        };
        let sum_type = LlvmType::Struct(vec![LlvmType::I32, LlvmType::I64]);
        let mut ctx = self.fn_ctx.take().unwrap();
        let sum_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::Alloca { результат: sum_ptr.clone(), тип: sum_type.clone() });
        let tag_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::GetElementPtr {
            результат: tag_ptr.clone(), тип: sum_type.clone(), указатель: sum_ptr.clone(),
            индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, "0".to_string())],
        });
        ctx.emit(LlvmInstruction::Store { тип: LlvmType::I32, значение: format!("{}", tag), указатель: tag_ptr });
        let data_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::GetElementPtr {
            результат: data_ptr.clone(), тип: sum_type.clone(), указатель: sum_ptr.clone(),
            индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, "1".to_string())],
        });
        ctx.emit(LlvmInstruction::Store { тип: LlvmType::I64, значение: inner_val, указатель: data_ptr });
        ctx.last_value = Some((LlvmType::Ptr(Box::new(sum_type)), sum_ptr));
        self.fn_ctx = Some(ctx);
    }

    fn generate_field_access(&mut self, объект: &Ast, поле: &str) {
        let (obj_type, obj_reg) = match self.eval_to_reg(объект) { Some(v) => v, None => return };
        let struct_type = match &obj_type { LlvmType::Ptr(inner) => *inner.clone(), _ => return };
        let field_types = match &struct_type { LlvmType::Struct(fields) => fields.clone(), _ => return };
        let field_index = match self.struct_schemas.values().find_map(|schema| schema.iter().position(|(name, _)| name == поле)) {
            Some(idx) => idx,
            None => {
                self.errors.push(Diagnostic {
                    kind: crate::error::DiagnosticKind::Ошибка,
                    message: format!("Поле '{}' не найдено", поле),
                    span: crate::token::Span { line: 0, column: 0, offset: 0 },
                    hint: None,
                });
                return;
            }
        };
        let field_type = field_types.get(field_index).cloned().unwrap_or(LlvmType::I64);
        let mut ctx = self.fn_ctx.take().unwrap();
        let field_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::GetElementPtr { результат: field_ptr.clone(), тип: struct_type.clone(), указатель: obj_reg, индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, field_index.to_string())] });
        let loaded = self.fresh_reg();
        ctx.emit(LlvmInstruction::Load { результат: loaded.clone(), тип: field_type.clone(), указатель: field_ptr });
        ctx.last_value = Some((field_type, loaded)); self.fn_ctx = Some(ctx);
    }

    fn generate_binary(&mut self, левое: &Ast, оператор: &БинарныйОператор, правое: &Ast, _аннот_тип: &Option<Тип>) {
        let lv = self.eval_to_reg(левое); let rv = self.eval_to_reg(правое);
        if let (Some((lt, l)), Some((_rt, r))) = (lv, rv) {
            let target = lt.clone();
            let (_, lc) = self.coerce_to(&l, &lt, &target, true); let (_, rc) = self.coerce_to(&r, &_rt, &target, true);
            let mut ctx = self.fn_ctx.take().unwrap(); let res = self.fresh_reg();
            let is_cmp = matches!(оператор, БинарныйОператор::Равно | БинарныйОператор::НеРавно | БинарныйОператор::Меньше | БинарныйОператор::Больше | БинарныйОператор::МеньшеРавно | БинарныйОператор::БольшеРавно);
            if is_cmp {
                let cond = match оператор { БинарныйОператор::Равно => "eq", БинарныйОператор::НеРавно => "ne", БинарныйОператор::Меньше => "slt", БинарныйОператор::Больше => "sgt", БинарныйОператор::МеньшеРавно => "sle", БинарныйОператор::БольшеРавно => "sge", _ => "eq" };
                ctx.emit(LlvmInstruction::Icmp { результат: res.clone(), условие: cond.into(), тип: target.clone(), левый: lc, правый: rc });
                ctx.last_value = Some((LlvmType::I1, res));
            } else {
                match (оператор, &target) {
                    (БинарныйОператор::Сложение, LlvmType::I64) => ctx.emit(LlvmInstruction::Add { результат: res.clone(), тип: LlvmType::I64, левый: lc, правый: rc }),
                    (БинарныйОператор::Вычитание, LlvmType::I64) => ctx.emit(LlvmInstruction::Sub { результат: res.clone(), тип: LlvmType::I64, левый: lc, правый: rc }),
                    (БинарныйОператор::Умножение, LlvmType::I64) => ctx.emit(LlvmInstruction::Mul { результат: res.clone(), тип: LlvmType::I64, левый: lc, правый: rc }),
                    (БинарныйОператор::Деление, LlvmType::I64) => ctx.emit(LlvmInstruction::SDiv { результат: res.clone(), левый: lc, правый: rc }),
                    _ => {}
                }
                ctx.last_value = Some((target, res));
            }
            self.fn_ctx = Some(ctx);
        }
    }

    fn eval_to_reg(&mut self, node: &Ast) -> Option<(LlvmType, String)> {
        match node {
            Ast::Литерал { значение, .. } => {
                let (t, v) = self.literal_to_llvm_imm(значение);
                if let Значение::Строка(s) = значение {
                    let mut ctx = self.fn_ctx.take().unwrap(); let reg = self.fresh_reg();
                    ctx.emit(LlvmInstruction::GetElementPtr { результат: reg.clone(), тип: LlvmType::Array(Box::new(LlvmType::I8), s.len() + 1), указатель: v, индексы: vec![(LlvmType::I32, "0".to_string()), (LlvmType::I32, "0".to_string())] });
                    self.fn_ctx = Some(ctx); Some((t, reg))
                } else { Some((t, v)) }
            }
            Ast::Переменная { имя, .. } => {
                let mut ctx = self.fn_ctx.take()?; let (ptr, typ) = ctx.variables.get(имя)?.clone();
                let reg = self.fresh_reg();
                ctx.emit(LlvmInstruction::Load { результат: reg.clone(), тип: typ.clone(), указатель: ptr });
                self.fn_ctx = Some(ctx); Some((typ, reg))
            }
            Ast::ДвоичноеВыражение { .. } | Ast::Если { .. } | Ast::Сопоставление { .. } | Ast::КонструкторСтруктуры { .. } | Ast::КонструкторСуммы { .. } | Ast::ДоступКПолю { .. } | Ast::ОбновлениеСтруктуры { .. } => {
                self.generate_expr(node); let ctx = self.fn_ctx.take()?; let val = ctx.last_value.clone(); self.fn_ctx = Some(ctx); val
            }
            _ => None,
        }
    }

    fn literal_to_llvm_imm(&mut self, значение: &Значение) -> (LlvmType, String) {
        match значение {
            Значение::Целое(n) => (LlvmType::I64, n.to_string()),
            Значение::Десятичное(f) => (LlvmType::Double, format!("{}", f)),
            Значение::Булево(b) => (LlvmType::I1, if *b { "1".into() } else { "0".into() }),
            Значение::Строка(s) => { let name = self.add_string_constant(s); (LlvmType::Ptr(Box::new(LlvmType::I8)), name) }
            Значение::Символ(c) => (LlvmType::I32, (*c as u32).to_string()),
            Значение::Ничего => (LlvmType::Void, "void".into()),
        }
    }

    fn type_to_llvm(&self, typ: &Тип) -> LlvmType {
        match typ {
            Тип::Примитивный(p) => match p { ПримитивныйТип::Целое => LlvmType::I64, ПримитивныйТип::Десятичное => LlvmType::Double, ПримитивныйТип::Булево => LlvmType::I1, ПримитивныйТип::Строка => LlvmType::Ptr(Box::new(LlvmType::I8)), _ => LlvmType::I64 },
            Тип::Ссылка { тип, .. } => LlvmType::Ptr(Box::new(self.type_to_llvm(тип))),
            Тип::Запись(поля) => LlvmType::Struct(поля.iter().map(|(_, t)| self.type_to_llvm(t)).collect()),
            Тип::Пустой => LlvmType::Void, _ => LlvmType::I64,
        }
    }

    fn generate_pending_monomorphizations(&mut self) { let pending = std::mem::take(&mut self.pending_mono); for (_, ast) in pending { self.generate_node(&ast); } }

    pub fn emit_llvm_text(ir: &LlvmIr) -> String {
        let LlvmIr::Модуль { имя, функции, строковые_константы } = ir;
        let mut out = format!("; Module: {}\n\n", имя);
        out.push_str("declare i32 @printf(i8*, ...) #0\n\n");
        for s in строковые_константы { out.push_str(&format!("{}\n", s)); }
        if !строковые_константы.is_empty() { out.push('\n'); }
        for f in функции { out.push_str(&Self::emit_function(f)); out.push('\n'); }
        out.push_str("attributes #0 = { nounwind }\n"); out
    }

    fn emit_function(f: &LlvmFunction) -> String {
        let params: Vec<String> = f.параметры.iter().map(|(n, t)| format!("{} %{}", t.to_llvm_string(), n)).collect();
        let mut out = format!("define {} @{}({}) {{\n", f.возвращаемый_тип.to_llvm_string(), f.имя, params.join(", "));
        for b in &f.базовые_блоки {
            out.push_str(&format!("{}:\n", b.метка.trim_start_matches('%')));
            for i in &b.инструкции { out.push_str(&format!("  {}\n", Self::emit_instruction(i))); }
            out.push_str(&format!("  {}\n", Self::emit_terminator(&b.терминатор)));
        }
        out.push_str("}\n"); out
    }

    fn emit_instruction(i: &LlvmInstruction) -> String {
        match i {
            LlvmInstruction::Add { результат, тип, левый, правый } => format!("{} = add {} {}, {}", результат, тип.to_llvm_string(), левый, правый),
            LlvmInstruction::Sub { результат, тип, левый, правый } => format!("{} = sub {} {}, {}", результат, тип.to_llvm_string(), левый, правый),
            LlvmInstruction::Mul { результат, тип, левый, правый } => format!("{} = mul {} {}, {}", результат, тип.to_llvm_string(), левый, правый),
            LlvmInstruction::SDiv { результат, левый, правый } => format!("{} = sdiv i64 {}, {}", результат, левый, правый),
            LlvmInstruction::SRem { результат, левый, правый } => format!("{} = srem i64 {}, {}", результат, левый, правый),
            LlvmInstruction::Icmp { результат, условие, тип, левый, правый } => format!("{} = icmp {} {} {}, {}", результат, условие, тип.to_llvm_string(), левый, правый),
            LlvmInstruction::Load { результат, тип, указатель } => format!("{} = load {}, {}* {}", результат, тип.to_llvm_string(), тип.to_llvm_string(), указатель),
            LlvmInstruction::Store { тип, значение, указатель } => format!("store {} {}, {}* {}", тип.to_llvm_string(), значение, тип.to_llvm_string(), указатель),
            LlvmInstruction::Alloca { результат, тип } => format!("{} = alloca {}", результат, тип.to_llvm_string()),
            LlvmInstruction::Call { результат, функция, аргументы } => {
                let r = результат.as_ref().map(|x| format!("{} = ", x)).unwrap_or_default();
                let a: Vec<String> = аргументы.iter().map(|(t, v)| format!("{} {}", t.to_llvm_string(), v)).collect();
                format!("{}call i32 @{}({})", r, функция, a.join(", "))
            }
            LlvmInstruction::Ret { значение } => match значение { Some((t, v)) => format!("ret {} {}", t.to_llvm_string(), v), None => "ret void".into() },
            LlvmInstruction::Br { метка } => format!("br label {}", ensure_label_prefix(метка)),
            LlvmInstruction::CondBr { условие, истина, ложь } => format!("br i1 {}, label {}, label {}", условие, ensure_label_prefix(истина), ensure_label_prefix(ложь)),
            LlvmInstruction::GetElementPtr { результат, тип, указатель, индексы } => {
                let idx: Vec<String> = индексы.iter().map(|(t, v)| format!("{} {}", t.to_llvm_string(), v)).collect();
                format!("{} = getelementptr {}, {}* {}, {}", результат, тип.to_llvm_string(), тип.to_llvm_string(), указатель, idx.join(", "))
            }
            LlvmInstruction::Bitcast { результат, значение, из, в } => format!("{} = bitcast {} {} to {}", результат, из.to_llvm_string(), значение, в.to_llvm_string()),
            _ => "".into()
        }
    }

    fn emit_terminator(t: &Terminator) -> String {
        match t {
            Terminator::Ret(v) => match v { Some((tp, val)) => format!("ret {} {}", tp.to_llvm_string(), val), None => "ret void".into() },
            Terminator::Br(l) => format!("br label {}", ensure_label_prefix(l)),
            Terminator::CondBr { условие, истина, ложь } => format!("br i1 {}, label {}, label {}", условие, ensure_label_prefix(истина), ensure_label_prefix(ложь)),
            Terminator::Unreachable => "unreachable".into(),
        }
    }
}

fn ensure_label_prefix(label: &str) -> String { if label.starts_with('%') { label.to_string() } else { format!("%{}", label) } }
