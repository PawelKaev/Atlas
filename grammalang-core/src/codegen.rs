// grammalang-core/src/codegen.rs
// Version 2.11.0 — StructUpdate + fixed field_access + new match patterns

use crate::ast::*;
use crate::error::Diagnostic;
use crate::resolve::SymbolTable;
use std::collections::HashMap;

// ============ LlvmIr ============

#[derive(Debug, Clone)]
pub enum LlvmIr {
    Module { name: String, functions: Vec<LlvmFunction>, string_constants: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct LlvmFunction {
    pub name: String,
    pub params: Vec<(String, LlvmType)>,
    pub return_type: LlvmType,
    pub basic_blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<LlvmInstruction>,
    pub terminator: Terminator,
}

#[derive(Debug, Clone)]
pub enum LlvmInstruction {
    Add { result: String, llvm_type: LlvmType, left: String, right: String },
    Sub { result: String, llvm_type: LlvmType, left: String, right: String },
    Mul { result: String, llvm_type: LlvmType, left: String, right: String },
    SDiv { result: String, left: String, right: String },
    UDiv { result: String, left: String, right: String },
    SRem { result: String, left: String, right: String },
    FAdd { result: String, left: String, right: String },
    FSub { result: String, left: String, right: String },
    FMul { result: String, left: String, right: String },
    FDiv { result: String, left: String, right: String },
    And { result: String, left: String, right: String },
    Or { result: String, left: String, right: String },
    Xor { result: String, left: String, right: String },
    Shl { result: String, left: String, right: String },
    LShr { result: String, left: String, right: String },
    AShr { result: String, left: String, right: String },
    Icmp { result: String, condition: String, llvm_type: LlvmType, left: String, right: String },
    Fcmp { result: String, condition: String, left: String, right: String },
    Load { result: String, llvm_type: LlvmType, pointer: String },
    Store { llvm_type: LlvmType, value: String, pointer: String },
    Alloca { result: String, llvm_type: LlvmType },
    Call { result: Option<String>, function: String, arguments: Vec<(LlvmType, String)> },
    Ret { value: Option<(LlvmType, String)> },
    Br { label: String },
    CondBr { condition: String, true_label: String, false_label: String },
    Phi { result: String, llvm_type: LlvmType, incoming: Vec<(String, String)> },
    GetElementPtr { result: String, llvm_type: LlvmType, pointer: String, indices: Vec<(LlvmType, String)> },
    Bitcast { result: String, value: String, from: LlvmType, to: LlvmType },
    ZExt { result: String, value: String, from: LlvmType, to: LlvmType },
    SExt { result: String, value: String, from: LlvmType, to: LlvmType },
    Select { result: String, condition: String, true_label: String, false_label: String, llvm_type: LlvmType },
    Unreachable,
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Ret(Option<(LlvmType, String)>),
    Br(String),
    CondBr { condition: String, true_label: String, false_label: String },
    Unreachable,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LlvmType {
    Void, I1, I8, I32, I64, Double,
    Ptr(Box<LlvmType>), Array(Box<LlvmType>, usize), Struct(Vec<LlvmType>), Named(String),
}

impl LlvmType {
    pub fn to_llvm_string(&self) -> String {
        match self {
            LlvmType::Void => "void".to_string(),
            LlvmType::I1 => "i1".to_string(),
            LlvmType::I8 => "i8".to_string(),
            LlvmType::I32 => "i32".to_string(),
            LlvmType::I64 => "i64".to_string(),
            LlvmType::Double => "double".to_string(),
            LlvmType::Ptr(t) => format!("{}*", t.to_llvm_string()),
            LlvmType::Array(t, n) => format!("[{} x {}]", n, t.to_llvm_string()),
            LlvmType::Struct(f) => format!("{{ {} }}", f.iter().map(|x| x.to_llvm_string()).collect::<Vec<_>>().join(", ")),
            LlvmType::Named(n) => format!("%{}", n),
        }
    }

    pub fn bit_width(&self) -> usize {
        match self {
            LlvmType::I1 => 1,
            LlvmType::I8 => 8,
            LlvmType::I32 => 32,
            LlvmType::I64 | LlvmType::Double => 64,
            _ => 64,
        }
    }
}

// ============ FnGenContext ============

struct FnGenContext {
    blocks: Vec<BasicBlock>,
    current_label: String,
    variables: HashMap<String, (String, LlvmType)>,
    last_value: Option<(LlvmType, String)>,
    entry_label: String,
    function_name: String,
    params: Vec<(String, LlvmType)>,
}

impl FnGenContext {
    fn new(entry_label: String, function_name: String, params: Vec<(String, LlvmType)>) -> Self {
        FnGenContext {
            blocks: Vec::new(),
            current_label: entry_label.clone(),
            variables: HashMap::new(),
            last_value: None,
            entry_label,
            function_name,
            params,
        }
    }

    fn start_new_block(&mut self, label: String) {
        self.current_label = label;
    }

    fn get_or_create_block(&mut self, label: &str) -> &mut BasicBlock {
        if let Some(pos) = self.blocks.iter().position(|b| b.label == label) {
            &mut self.blocks[pos]
        } else {
            self.blocks.push(BasicBlock {
                label: label.to_string(),
                instructions: Vec::new(),
                terminator: Terminator::Unreachable,
            });
            self.blocks.last_mut().unwrap()
        }
    }

    fn emit(&mut self, instr: LlvmInstruction) {
        self.get_or_create_block(&self.current_label.clone()).instructions.push(instr);
    }

    fn set_terminator(&mut self, term: Terminator) {
        self.get_or_create_block(&self.current_label.clone()).terminator = term;
    }

    fn finish_with_br(&mut self, target: &str) {
        let current = self.current_label.clone();
        let block = self.get_or_create_block(&current);
        if matches!(block.terminator, Terminator::Unreachable) {
            block.terminator = Terminator::Br(target.to_string());
        }
    }

    fn finalize(self) -> Vec<BasicBlock> {
        self.blocks
    }
}

// ============ Codegen ============

pub struct Codegen {
    module_name: String,
    functions: Vec<LlvmFunction>,
    string_constants: Vec<String>,
    errors: Vec<Diagnostic>,
    next_reg: usize,
    next_block: usize,
    next_string: usize,
    fn_ctx: Option<FnGenContext>,
    symbol_table: Option<SymbolTable>,
    struct_schemas: HashMap<String, Vec<(String, LlvmType)>>,
    monomorphized: HashMap<(String, Vec<LlvmType>), String>,
    pending_mono: Vec<(String, Ast)>,
}

impl Codegen {
    pub fn new(module_name: &str) -> Self {
        Codegen {
            module_name: module_name.to_string(),
            functions: Vec::new(),
            string_constants: Vec::new(),
            errors: Vec::new(),
            next_reg: 0,
            next_block: 0,
            next_string: 0,
            fn_ctx: None,
            symbol_table: None,
            struct_schemas: HashMap::new(),
            monomorphized: HashMap::new(),
            pending_mono: Vec::new(),
        }
    }

    pub fn set_symbol_table(&mut self, table: SymbolTable) {
        self.symbol_table = Some(table);
    }

    pub fn register_struct(&mut self, name: &str, fields: Vec<(String, LlvmType)>) {
        self.struct_schemas.insert(name.to_string(), fields);
    }

    pub fn generate(&mut self, ast: &Ast) -> (Option<LlvmIr>, Vec<Diagnostic>) {
        self.generate_node(ast);
        self.generate_pending_monomorphizations();
        let ir = LlvmIr::Module {
            name: self.module_name.clone(),
            functions: std::mem::take(&mut self.functions),
            string_constants: std::mem::take(&mut self.string_constants),
        };
        (Some(ir), std::mem::take(&mut self.errors))
    }

    fn fresh_reg(&mut self) -> String {
        let id = self.next_reg;
        self.next_reg += 1;
        format!("%reg{}", id)
    }

    fn fresh_block(&mut self) -> String {
        let id = self.next_block;
        self.next_block += 1;
        format!("%block{}", id)
    }

    fn fresh_string_name(&mut self) -> String {
        let id = self.next_string;
        self.next_string += 1;
        format!("@.str.{}", id)
    }

    fn add_string_constant(&mut self, s: &str) -> String {
        let name = self.fresh_string_name();
        self.string_constants.push(format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"",
            name,
            s.len() + 1,
            s
        ));
        name
    }

    fn to_llvm_name(&self, atlas_name: &str) -> String {
        if atlas_name == "main" {
            "main".to_string()
        } else {
            atlas_name.to_string()
        }
    }

    fn coerce_to(&mut self, value: &str, from: &LlvmType, to: &LlvmType, _signed: bool) -> (LlvmType, String) {
        if from == to {
            return (to.clone(), value.to_string());
        }
        let reg = self.fresh_reg();
        let mut ctx = self.fn_ctx.take().unwrap();
        match (from, to) {
            (LlvmType::I1, LlvmType::I64) => ctx.emit(LlvmInstruction::ZExt {
                result: reg.clone(),
                value: value.to_string(),
                from: from.clone(),
                to: to.clone(),
            }),
            (LlvmType::I32, LlvmType::I64) => ctx.emit(LlvmInstruction::SExt {
                result: reg.clone(),
                value: value.to_string(),
                from: from.clone(),
                to: to.clone(),
            }),
            _ => {}
        }
        self.fn_ctx = Some(ctx);
        (to.clone(), reg)
    }

    fn generate_node(&mut self, node: &Ast) {
        match node {
            Ast::Module { declarations, .. } => {
                for d in declarations {
                    self.generate_node(d);
                }
            }
            Ast::StructDecl { name, fields, .. } => {
                let fields: Vec<_> = fields
                    .iter()
                    .map(|(n, t)| (n.clone(), self.type_to_llvm(t)))
                    .collect();
                self.register_struct(name, fields);
            }
            Ast::FnDecl { name, params, return_type, body, .. } => {
                let entry_label = self.fresh_block();
                let llvm_params: Vec<_> = params
                    .iter()
                    .map(|p| (p.name.clone(), self.type_to_llvm(&p.llvm_type)))
                    .collect();
                let llvm_name = self.to_llvm_name(name);
                let mut ctx = FnGenContext::new(entry_label.clone(), llvm_name.clone(), llvm_params.clone());
                for param in params {
                    let lt = self.type_to_llvm(&param.llvm_type);
                    let ptr = self.fresh_reg();
                    ctx.emit(LlvmInstruction::Alloca {
                        result: ptr.clone(),
                        llvm_type: lt.clone(),
                    });
                    ctx.emit(LlvmInstruction::Store {
                        llvm_type: lt.clone(),
                        value: format!("%{}", param.name),
                        pointer: ptr.clone(),
                    });
                    ctx.variables.insert(param.name.clone(), (ptr, lt));
                }
                self.fn_ctx = Some(ctx);
                self.generate_node(body);
                if let Some(ctx) = self.fn_ctx.take() {
                    let blocks = ctx.finalize();
                    self.functions.push(LlvmFunction {
                        name: llvm_name,
                        params: llvm_params,
                        return_type: self.type_to_llvm(&return_type.clone().unwrap_or(Type::Unit)),
                        basic_blocks: blocks,
                    });
                }
            }
            Ast::Block { expressions, .. } => {
                for e in expressions {
                    self.generate_expr(e);
                }
            }
            Ast::ScopeBlock { expressions, last, .. } => {
                for e in expressions {
                    self.generate_expr(e);
                }
                if let Some(last) = last {
                    self.generate_expr(last);
                }
            }
            Ast::Return { value, .. } => {
                if let Some(inner) = value {
                    self.generate_expr(inner);
                }
                let val = value.as_ref().and_then(|v| self.eval_to_reg(v));
                let mut ctx = self.fn_ctx.take().unwrap();
                ctx.set_terminator(Terminator::Ret(val));
                self.fn_ctx = Some(ctx);
            }
            _ => {}
        }
    }

    fn generate_expr(&mut self, node: &Ast) {
        match node {
            Ast::Let { name, value, .. } | Ast::Assign { name, value, .. } => {
                if let Some((vt, v)) = self.eval_to_reg(value) {
                    let mut ctx = self.fn_ctx.take().unwrap();
                    if let Some((existing_ptr, _)) = ctx.variables.get(name).cloned() {
                        ctx.emit(LlvmInstruction::Store {
                            llvm_type: vt.clone(),
                            value: v,
                            pointer: existing_ptr,
                        });
                    } else {
                        let ptr = self.fresh_reg();
                        ctx.emit(LlvmInstruction::Alloca {
                            result: ptr.clone(),
                            llvm_type: vt.clone(),
                        });
                        ctx.emit(LlvmInstruction::Store {
                            llvm_type: vt.clone(),
                            value: v,
                            pointer: ptr.clone(),
                        });
                        ctx.variables.insert(name.clone(), (ptr, vt));
                    }
                    self.fn_ctx = Some(ctx);
                }
            }
            Ast::OpAssign { name, operator, value, .. } => {
                if let (Some((lt, lv)), Some((_rt, rv))) = (
                    self.eval_to_reg(&Ast::Variable {
                        name: name.clone(),
                        llvm_type: None,
                        span: crate::token::Span { line: 0, column: 0, offset: 0 },
                    }),
                    self.eval_to_reg(value),
                ) {
                    let mut ctx = self.fn_ctx.take().unwrap();
                    let res = self.fresh_reg();
                    match operator {
                        BinOp::Add => ctx.emit(LlvmInstruction::Add {
                            result: res.clone(),
                            llvm_type: lt.clone(),
                            left: lv,
                            right: rv,
                        }),
                        BinOp::Sub => ctx.emit(LlvmInstruction::Sub {
                            result: res.clone(),
                            llvm_type: lt.clone(),
                            left: lv,
                            right: rv,
                        }),
                        BinOp::Mul => ctx.emit(LlvmInstruction::Mul {
                            result: res.clone(),
                            llvm_type: lt.clone(),
                            left: lv,
                            right: rv,
                        }),
                        _ => {}
                    };
                    if let Some((ptr, _)) = ctx.variables.get(name).cloned() {
                        ctx.emit(LlvmInstruction::Store {
                            llvm_type: lt.clone(),
                            value: res.clone(),
                            pointer: ptr,
                        });
                    }
                    ctx.last_value = Some((lt, res));
                    self.fn_ctx = Some(ctx);
                }
            }
            Ast::If { condition, then, else_arm, .. } => {
                self.generate_if(condition, then, else_arm.as_deref());
            }
            Ast::While { condition, body, .. } => {
                self.generate_while(condition, body);
            }
            Ast::LoopWhile { condition, body, .. } => {
                self.generate_while(condition, body);
            }
            Ast::Loop { body, .. } => {
                let true_val = Ast::Literal {
                    value: Value::Bool(true),
                    span: crate::token::Span { line: 0, column: 0, offset: 0 },
                };
                self.generate_while(&true_val, body);
            }
            Ast::ForLoop { variable, iterator, body, .. } => {
                self.generate_for(variable, iterator, body);
            }
            Ast::Break { .. } => {
                let mut ctx = self.fn_ctx.take().unwrap();
                ctx.set_terminator(Terminator::Br("%exit_loop".to_string()));
                self.fn_ctx = Some(ctx);
            }
            Ast::Continue { .. } => {
                let mut ctx = self.fn_ctx.take().unwrap();
                ctx.set_terminator(Terminator::Br("%continue_loop".to_string()));
                self.fn_ctx = Some(ctx);
            }
            Ast::Match { value, arms, .. } => {
                self.generate_match(value, arms);
            }
            Ast::Call { function, arguments, .. } => {
                self.generate_call(function, arguments);
            }
            Ast::BinExpr { left, operator, right, llvm_type, .. } => {
                self.generate_binary(left, operator, right, llvm_type);
            }
            Ast::StructCons { name, fields, .. } => {
                self.generate_struct_construction(name, fields);
            }
            Ast::FieldAccess { object, field, .. } => {
                self.generate_field_access(object, field);
            }
            Ast::StructUpdate { object, fields, llvm_type, span: _ } => {
                let field_types = match llvm_type {
                    Some(Type::Record(fields)) => fields.clone(),
                    _ => {
                        self.errors.push(Diagnostic {
                            kind: crate::error::DiagnosticKind::Error,
                            message: "Struct update: expected record type".to_string(),
                            span: crate::token::Span { line: 0, column: 0, offset: 0 },
                            hint: None,
                        });
                        return;
                    }
                };
                let (_, source_reg) = match self.eval_to_reg(object) {
                    Some(v) => v,
                    None => return,
                };
                let llvm_types: Vec<LlvmType> = field_types
                    .iter()
                    .map(|(_, t)| self.type_to_llvm(t))
                    .collect();
                let struct_type = LlvmType::Struct(llvm_types);
                let mut updates: Vec<(String, (LlvmType, String))> = Vec::new();
                for (name, value) in fields {
                    if let Some(val) = self.eval_to_reg(value) {
                        updates.push((name.clone(), val));
                    }
                }
                let mut ctx = self.fn_ctx.take().unwrap();
                let new_ptr = self.fresh_reg();
                ctx.emit(LlvmInstruction::Alloca {
                    result: new_ptr.clone(),
                    llvm_type: struct_type.clone(),
                });
                for (i, (field_name, field_type)) in field_types.iter().enumerate() {
                    let llvm_field_type = self.type_to_llvm(field_type);
                    let (val_type, val_reg) = if let Some((_, val)) = updates.iter().find(|(n, _)| n == field_name) {
                        val.clone()
                    } else {
                        let src_ptr = self.fresh_reg();
                        ctx.emit(LlvmInstruction::GetElementPtr {
                            result: src_ptr.clone(),
                            llvm_type: struct_type.clone(),
                            pointer: source_reg.clone(),
                            indices: vec![
                                (LlvmType::I32, "0".to_string()),
                                (LlvmType::I32, i.to_string()),
                            ],
                        });
                        let loaded = self.fresh_reg();
                        ctx.emit(LlvmInstruction::Load {
                            result: loaded.clone(),
                            llvm_type: llvm_field_type.clone(),
                            pointer: src_ptr,
                        });
                        (llvm_field_type, loaded)
                    };
                    let dst_ptr = self.fresh_reg();
                    ctx.emit(LlvmInstruction::GetElementPtr {
                        result: dst_ptr.clone(),
                        llvm_type: struct_type.clone(),
                        pointer: new_ptr.clone(),
                        indices: vec![
                            (LlvmType::I32, "0".to_string()),
                            (LlvmType::I32, i.to_string()),
                        ],
                    });
                    ctx.emit(LlvmInstruction::Store {
                        llvm_type: val_type,
                        value: val_reg,
                        pointer: dst_ptr,
                    });
                }
                ctx.last_value = Some((LlvmType::Ptr(Box::new(struct_type)), new_ptr));
                self.fn_ctx = Some(ctx);
            }
            Ast::SumCons { name, value, .. } => {
                self.generate_sum_construction(name, value.as_deref());
            }
            Ast::Block { expressions, .. } => {
                for e in expressions {
                    self.generate_expr(e);
                }
            }
            _ => {}
        }
    }

    fn generate_for(&mut self, variable: &str, _iterator: &Ast, body: &Ast) {
        let header_l = self.fresh_block();
        let body_l = self.fresh_block();
        let exit_l = self.fresh_block();
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.get_or_create_block(&header_l);
        ctx.get_or_create_block(&body_l);
        ctx.get_or_create_block(&exit_l);
        ctx.finish_with_br(&header_l);
        self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(header_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        let cond = self.fresh_reg();
        ctx.emit(LlvmInstruction::Icmp {
            result: cond.clone(),
            condition: "ne".into(),
            llvm_type: LlvmType::I64,
            left: "0".to_string(),
            right: "1".to_string(),
        });
        ctx.set_terminator(Terminator::CondBr {
            condition: cond,
            true_label: body_l.clone(),
            false_label: exit_l.clone(),
        });
        self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(body_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        let elem_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::Alloca {
            result: elem_ptr.clone(),
            llvm_type: LlvmType::I64,
        });
        ctx.variables.insert(variable.to_string(), (elem_ptr, LlvmType::I64));
        self.fn_ctx = Some(ctx);
        self.generate_expr(body);
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.finish_with_br(&header_l);
        self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(exit_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.set_terminator(Terminator::Ret(None));
        self.fn_ctx = Some(ctx);
    }

    fn generate_if(&mut self, condition: &Ast, then: &Ast, else_arm: Option<&Ast>) {
        let then_l = self.fresh_block();
        let else_l = self.fresh_block();
        let merge_l = self.fresh_block();
        let cond = self
            .eval_to_reg(condition)
            .map(|(_, v)| v)
            .unwrap_or_else(|| "0".to_string());

        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.get_or_create_block(&then_l);
        ctx.get_or_create_block(&else_l);
        ctx.get_or_create_block(&merge_l);
        ctx.set_terminator(Terminator::CondBr {
            condition: cond,
            true_label: then_l.clone(),
            false_label: else_l.clone(),
        });
        self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(then_l.clone());
        self.generate_expr(then);
        let mut ctx = self.fn_ctx.take().unwrap();
        let tv = ctx.last_value.take();
        let t_end = ctx.current_label.clone();
        ctx.finish_with_br(&merge_l);
        self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(else_l.clone());
        let ev = if let Some(e) = else_arm {
            self.generate_expr(e);
            let mut ctx = self.fn_ctx.take().unwrap();
            let val = ctx.last_value.take();
            self.fn_ctx = Some(ctx);
            val
        } else {
            None
        };
        let mut ctx = self.fn_ctx.take().unwrap();
        let e_end = ctx.current_label.clone();
        ctx.finish_with_br(&merge_l);
        self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(merge_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        match (&tv, &ev) {
            (Some((tt, t)), Some((_, e))) => {
                let phi = self.fresh_reg();
                ctx.emit(LlvmInstruction::Phi {
                    result: phi.clone(),
                    llvm_type: tt.clone(),
                    incoming: vec![(t.clone(), t_end), (e.clone(), e_end)],
                });
                ctx.last_value = Some((tt.clone(), phi));
            }
            _ => ctx.last_value = None,
        }
        self.fn_ctx = Some(ctx);
    }

    fn generate_while(&mut self, condition: &Ast, body: &Ast) {
        let header_l = self.fresh_block();
        let body_l = self.fresh_block();
        let exit_l = self.fresh_block();

        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.get_or_create_block(&header_l);
        ctx.get_or_create_block(&body_l);
        ctx.get_or_create_block(&exit_l);
        ctx.finish_with_br(&header_l);
        self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(header_l.clone());
        let cond = self
            .eval_to_reg(condition)
            .map(|(_, v)| v)
            .unwrap_or_else(|| "0".to_string());
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.set_terminator(Terminator::CondBr {
            condition: cond,
            true_label: body_l.clone(),
            false_label: exit_l.clone(),
        });
        self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(body_l.clone());
        self.generate_expr(body);
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.finish_with_br(&header_l);
        self.fn_ctx = Some(ctx);

        self.fn_ctx.as_mut().unwrap().start_new_block(exit_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.set_terminator(Terminator::Ret(None));
        self.fn_ctx = Some(ctx);
    }

    fn generate_match(&mut self, value: &Ast, arms: &[MatchArm]) {
        let merge_l = self.fresh_block();
        let (val_type, val_reg) = match self.eval_to_reg(value) {
            Some(v) => v,
            None => return,
        };
        let mut branch_vals: Vec<(String, Option<(LlvmType, String)>)> = Vec::new();
        for (i, arm) in arms.iter().enumerate() {
            let is_last = i == arms.len() - 1;
            let next_test = if is_last {
                merge_l.clone()
            } else {
                self.fresh_block()
            };
            let body_l = self.fresh_block();
            self.generate_pattern_test(&arm.pattern, &val_type, &val_reg, &body_l, &next_test);
            self.fn_ctx.as_mut().unwrap().start_new_block(body_l.clone());
            self.bind_pattern_vars(&arm.pattern, &val_type, &val_reg);
            if let Some(ref condition) = arm.condition {
                let guard_val = self
                    .eval_to_reg(condition)
                    .map(|(_, v)| v)
                    .unwrap_or_else(|| "0".to_string());
                let guard_pass = self.fresh_block();
                let guard_fail = if is_last {
                    merge_l.clone()
                } else {
                    next_test.clone()
                };
                let mut ctx = self.fn_ctx.take().unwrap();
                ctx.set_terminator(Terminator::CondBr {
                    condition: guard_val,
                    true_label: guard_pass.clone(),
                    false_label: guard_fail,
                });
                self.fn_ctx = Some(ctx);
                self.fn_ctx.as_mut().unwrap().start_new_block(guard_pass);
            }
            self.generate_expr(&arm.body);
            let mut ctx = self.fn_ctx.take().unwrap();
            let bv = ctx.last_value.take();
            let b_end = ctx.current_label.clone();
            ctx.finish_with_br(&merge_l);
            branch_vals.push((b_end, bv));
            self.fn_ctx = Some(ctx);
            if !is_last {
                self.fn_ctx.as_mut().unwrap().start_new_block(next_test.clone());
            }
        }
        self.fn_ctx.as_mut().unwrap().start_new_block(merge_l.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        if !branch_vals.is_empty() && branch_vals.iter().all(|(_, v)| v.is_some()) {
            if let Some((phi_type, _)) = branch_vals[0].1.as_ref() {
                if *phi_type != LlvmType::Void {
                    let phi_reg = self.fresh_reg();
                    let incoming: Vec<_> = branch_vals
                        .iter()
                        .map(|(l, v)| {
                            let (_, vv) = v.as_ref().unwrap();
                            (vv.clone(), l.clone())
                        })
                        .collect();
                    ctx.emit(LlvmInstruction::Phi {
                        result: phi_reg.clone(),
                        llvm_type: phi_type.clone(),
                        incoming,
                    });
                    ctx.last_value = Some((phi_type.clone(), phi_reg));
                } else {
                    ctx.last_value = None;
                }
            }
        } else {
            ctx.last_value = None;
        }
        ctx.set_terminator(Terminator::Ret(None));
        self.fn_ctx = Some(ctx);
    }

    fn generate_pattern_test(
        &mut self,
        pattern: &Pattern,
        _val_type: &LlvmType,
        val_reg: &str,
        match_l: &str,
        next_l: &str,
    ) {
        let mut ctx = self.fn_ctx.take().unwrap();
        match pattern {
            Pattern::Wildcard | Pattern::Variable(_) => {
                ctx.set_terminator(Terminator::Br(match_l.to_string()));
            }
            Pattern::Literal(value) => {
                let (lit_type, lit_val) = self.literal_to_llvm_imm(value);
                let cmp = self.fresh_reg();
                ctx.emit(LlvmInstruction::Icmp {
                    result: cmp.clone(),
                    condition: "eq".into(),
                    llvm_type: lit_type,
                    left: val_reg.to_string(),
                    right: lit_val,
                });
                ctx.set_terminator(Terminator::CondBr {
                    condition: cmp,
                    true_label: match_l.to_string(),
                    false_label: next_l.to_string(),
                });
            }
            Pattern::Constructor { .. } => {
                let sum_type = LlvmType::Struct(vec![LlvmType::I32, LlvmType::I64]);
                let tag_ptr = self.fresh_reg();
                ctx.emit(LlvmInstruction::GetElementPtr {
                    result: tag_ptr.clone(),
                    llvm_type: sum_type,
                    pointer: val_reg.to_string(),
                    indices: vec![
                        (LlvmType::I32, "0".to_string()),
                        (LlvmType::I32, "0".to_string()),
                    ],
                });
                let loaded_tag = self.fresh_reg();
                ctx.emit(LlvmInstruction::Load {
                    result: loaded_tag.clone(),
                    llvm_type: LlvmType::I32,
                    pointer: tag_ptr,
                });
                let cmp = self.fresh_reg();
                ctx.emit(LlvmInstruction::Icmp {
                    result: cmp.clone(),
                    condition: "eq".into(),
                    llvm_type: LlvmType::I32,
                    left: loaded_tag,
                    right: "0".to_string(),
                });
                ctx.set_terminator(Terminator::CondBr {
                    condition: cmp,
                    true_label: match_l.to_string(),
                    false_label: next_l.to_string(),
                });
            }
            Pattern::Struct { .. } => {
                ctx.set_terminator(Terminator::Br(match_l.to_string()));
            }
            Pattern::Or(left, right) => {
                let or_next = self.fresh_block();
                self.fn_ctx = Some(ctx);
                self.generate_pattern_test(left, _val_type, val_reg, match_l, &or_next);
                self.fn_ctx.as_mut().unwrap().start_new_block(or_next.clone());
                self.generate_pattern_test(right, _val_type, val_reg, match_l, next_l);
                return;
            }
            Pattern::Binding { pattern, .. } => {
                self.fn_ctx = Some(ctx);
                self.generate_pattern_test(pattern, _val_type, val_reg, match_l, next_l);
                return;
            }
            _ => ctx.set_terminator(Terminator::Br(next_l.to_string())),
        }
        self.fn_ctx = Some(ctx);
    }

    fn bind_pattern_vars(&mut self, pattern: &Pattern, val_type: &LlvmType, val_reg: &str) {
        let mut ctx = self.fn_ctx.take().unwrap();
        match pattern {
            Pattern::Variable(name) => {
                let ptr = self.fresh_reg();
                ctx.emit(LlvmInstruction::Alloca {
                    result: ptr.clone(),
                    llvm_type: val_type.clone(),
                });
                ctx.emit(LlvmInstruction::Store {
                    llvm_type: val_type.clone(),
                    value: val_reg.to_string(),
                    pointer: ptr.clone(),
                });
                ctx.variables.insert(name.clone(), (ptr, val_type.clone()));
            }
            Pattern::Binding { name, pattern } => {
                let ptr = self.fresh_reg();
                ctx.emit(LlvmInstruction::Alloca {
                    result: ptr.clone(),
                    llvm_type: val_type.clone(),
                });
                ctx.emit(LlvmInstruction::Store {
                    llvm_type: val_type.clone(),
                    value: val_reg.to_string(),
                    pointer: ptr.clone(),
                });
                ctx.variables.insert(name.clone(), (ptr, val_type.clone()));
                self.fn_ctx = Some(ctx);
                self.bind_pattern_vars(pattern, val_type, val_reg);
                return;
            }
            Pattern::Struct { fields, .. } => {
                if let LlvmType::Ptr(inner) = val_type {
                    if let LlvmType::Struct(field_types) = inner.as_ref() {
                        for (i, (_, field_pattern)) in fields.iter().enumerate() {
                            let field_ptr = self.fresh_reg();
                            ctx.emit(LlvmInstruction::GetElementPtr {
                                result: field_ptr.clone(),
                                llvm_type: *inner.clone(),
                                pointer: val_reg.to_string(),
                                indices: vec![
                                    (LlvmType::I32, "0".to_string()),
                                    (LlvmType::I32, i.to_string()),
                                ],
                            });
                            let field_type =
                                field_types.get(i).cloned().unwrap_or(LlvmType::I64);
                            let loaded = self.fresh_reg();
                            ctx.emit(LlvmInstruction::Load {
                                result: loaded.clone(),
                                llvm_type: field_type.clone(),
                                pointer: field_ptr,
                            });
                            self.fn_ctx = Some(ctx);
                            self.bind_pattern_vars(field_pattern, &field_type, &loaded);
                            ctx = self.fn_ctx.take().unwrap();
                        }
                    }
                }
            }
            Pattern::Constructor { nested, .. } => {
                if let Some(inner_pattern) = nested {
                    let sum_type = LlvmType::Struct(vec![LlvmType::I32, LlvmType::I64]);
                    let data_ptr = self.fresh_reg();
                    ctx.emit(LlvmInstruction::GetElementPtr {
                        result: data_ptr.clone(),
                        llvm_type: sum_type,
                        pointer: val_reg.to_string(),
                        indices: vec![
                            (LlvmType::I32, "0".to_string()),
                            (LlvmType::I32, "1".to_string()),
                        ],
                    });
                    let loaded_data = self.fresh_reg();
                    ctx.emit(LlvmInstruction::Load {
                        result: loaded_data.clone(),
                        llvm_type: LlvmType::I64,
                        pointer: data_ptr,
                    });
                    self.fn_ctx = Some(ctx);
                    self.bind_pattern_vars(inner_pattern, &LlvmType::I64, &loaded_data);
                    return;
                }
            }
            Pattern::Or(left, _) => {
                self.fn_ctx = Some(ctx);
                self.bind_pattern_vars(left, val_type, val_reg);
                return;
            }
            _ => {}
        }
        self.fn_ctx = Some(ctx);
    }

    fn generate_call(&mut self, function: &Ast, arguments: &[Ast]) {
        if let Ast::Variable { name, .. } = function {
            if name == "print" || name == "console.print" {
                self.generate_print(arguments);
                return;
            }
        }
        let func_name = match function {
            Ast::Variable { name, .. } => self.to_llvm_name(name),
            _ => return,
        };
        let mut args_buf: Vec<(LlvmType, String)> = Vec::new();
        for arg in arguments {
            if let Some(v) = self.eval_to_reg(arg) {
                args_buf.push(v);
            }
        }
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.emit(LlvmInstruction::Call {
            result: None,
            function: func_name,
            arguments: args_buf,
        });
        ctx.last_value = Some((LlvmType::Void, "void".to_string()));
        self.fn_ctx = Some(ctx);
    }

    fn generate_print(&mut self, arguments: &[Ast]) {
        let mut buf: Vec<(LlvmType, String)> = Vec::new();
        for arg in arguments {
            if let Some((vt, v)) = self.eval_to_reg(arg) {
                match &vt {
                    LlvmType::Ptr(_) => buf.push((LlvmType::Ptr(Box::new(LlvmType::I8)), v)),
                    LlvmType::I64 => {
                        let fmt_str = "%lld";
                        let fmt_name = self.add_string_constant(fmt_str);
                        let mut ctx = self.fn_ctx.take().unwrap();
                        let fp = self.fresh_reg();
                        ctx.emit(LlvmInstruction::GetElementPtr {
                            result: fp.clone(),
                            llvm_type: LlvmType::Array(Box::new(LlvmType::I8), fmt_str.len() + 1),
                            pointer: fmt_name,
                            indices: vec![
                                (LlvmType::I32, "0".to_string()),
                                (LlvmType::I32, "0".to_string()),
                            ],
                        });
                        self.fn_ctx = Some(ctx);
                        buf.push((LlvmType::Ptr(Box::new(LlvmType::I8)), fp));
                        buf.push((vt, v));
                    }
                    LlvmType::Double => {
                        let fmt_str = "%f";
                        let fmt_name = self.add_string_constant(fmt_str);
                        let mut ctx = self.fn_ctx.take().unwrap();
                        let fp = self.fresh_reg();
                        ctx.emit(LlvmInstruction::GetElementPtr {
                            result: fp.clone(),
                            llvm_type: LlvmType::Array(Box::new(LlvmType::I8), fmt_str.len() + 1),
                            pointer: fmt_name,
                            indices: vec![
                                (LlvmType::I32, "0".to_string()),
                                (LlvmType::I32, "0".to_string()),
                            ],
                        });
                        self.fn_ctx = Some(ctx);
                        buf.push((LlvmType::Ptr(Box::new(LlvmType::I8)), fp));
                        buf.push((vt, v));
                    }
                    _ => {}
                }
            }
        }
        let mut ctx = self.fn_ctx.take().unwrap();
        ctx.emit(LlvmInstruction::Call {
            result: None,
            function: "printf".to_string(),
            arguments: buf,
        });
        ctx.last_value = Some((LlvmType::Void, "void".to_string()));
        self.fn_ctx = Some(ctx);
    }

    fn generate_struct_construction(&mut self, _name: &str, fields: &[(String, Ast)]) {
        let field_types: Vec<LlvmType> = fields
            .iter()
            .map(|(_, v)| self.eval_to_reg(v).map(|(t, _)| t).unwrap_or(LlvmType::I64))
            .collect();
        let struct_type = LlvmType::Struct(field_types.clone());
        let mut ctx = self.fn_ctx.take().unwrap();
        let struct_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::Alloca {
            result: struct_ptr.clone(),
            llvm_type: struct_type.clone(),
        });
        for (i, (_, value)) in fields.iter().enumerate() {
            if let Some((val_type, val_reg)) = self.eval_to_reg(value) {
                let field_ptr = self.fresh_reg();
                ctx.emit(LlvmInstruction::GetElementPtr {
                    result: field_ptr.clone(),
                    llvm_type: struct_type.clone(),
                    pointer: struct_ptr.clone(),
                    indices: vec![
                        (LlvmType::I32, "0".to_string()),
                        (LlvmType::I32, i.to_string()),
                    ],
                });
                ctx.emit(LlvmInstruction::Store {
                    llvm_type: val_type,
                    value: val_reg,
                    pointer: field_ptr,
                });
            }
        }
        ctx.last_value = Some((LlvmType::Ptr(Box::new(struct_type)), struct_ptr));
        self.fn_ctx = Some(ctx);
    }

    fn generate_sum_construction(&mut self, variant_name: &str, value: Option<&Ast>) {
        let tag = 0i64;
        let inner_val = if let Some(val_ast) = value {
            self.eval_to_reg(val_ast)
                .map(|(_, v)| v)
                .unwrap_or_else(|| "0".to_string())
        } else {
            "0".to_string()
        };
        let sum_type = LlvmType::Struct(vec![LlvmType::I32, LlvmType::I64]);
        let mut ctx = self.fn_ctx.take().unwrap();
        let sum_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::Alloca {
            result: sum_ptr.clone(),
            llvm_type: sum_type.clone(),
        });
        let tag_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::GetElementPtr {
            result: tag_ptr.clone(),
            llvm_type: sum_type.clone(),
            pointer: sum_ptr.clone(),
            indices: vec![
                (LlvmType::I32, "0".to_string()),
                (LlvmType::I32, "0".to_string()),
            ],
        });
        ctx.emit(LlvmInstruction::Store {
            llvm_type: LlvmType::I32,
            value: format!("{}", tag),
            pointer: tag_ptr,
        });
        let data_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::GetElementPtr {
            result: data_ptr.clone(),
            llvm_type: sum_type.clone(),
            pointer: sum_ptr.clone(),
            indices: vec![
                (LlvmType::I32, "0".to_string()),
                (LlvmType::I32, "1".to_string()),
            ],
        });
        ctx.emit(LlvmInstruction::Store {
            llvm_type: LlvmType::I64,
            value: inner_val,
            pointer: data_ptr,
        });
        ctx.last_value = Some((LlvmType::Ptr(Box::new(sum_type)), sum_ptr));
        self.fn_ctx = Some(ctx);
    }

    fn generate_field_access(&mut self, object: &Ast, field: &str) {
        let (obj_type, obj_reg) = match self.eval_to_reg(object) {
            Some(v) => v,
            None => return,
        };
        let struct_type = match &obj_type {
            LlvmType::Ptr(inner) => *inner.clone(),
            _ => return,
        };
        let field_types = match &struct_type {
            LlvmType::Struct(fields) => fields.clone(),
            _ => return,
        };
        let field_index = match self
            .struct_schemas
            .values()
            .find_map(|schema| schema.iter().position(|(name, _)| name == field))
        {
            Some(idx) => idx,
            None => {
                self.errors.push(Diagnostic {
                    kind: crate::error::DiagnosticKind::Error,
                    message: format!("Field '{}' not found", field),
                    span: crate::token::Span { line: 0, column: 0, offset: 0 },
                    hint: None,
                });
                return;
            }
        };
        let field_type = field_types.get(field_index).cloned().unwrap_or(LlvmType::I64);
        let mut ctx = self.fn_ctx.take().unwrap();
        let field_ptr = self.fresh_reg();
        ctx.emit(LlvmInstruction::GetElementPtr {
            result: field_ptr.clone(),
            llvm_type: struct_type.clone(),
            pointer: obj_reg,
            indices: vec![
                (LlvmType::I32, "0".to_string()),
                (LlvmType::I32, field_index.to_string()),
            ],
        });
        let loaded = self.fresh_reg();
        ctx.emit(LlvmInstruction::Load {
            result: loaded.clone(),
            llvm_type: field_type.clone(),
            pointer: field_ptr,
        });
        ctx.last_value = Some((field_type, loaded));
        self.fn_ctx = Some(ctx);
    }

    fn generate_binary(
        &mut self,
        left: &Ast,
        operator: &BinOp,
        right: &Ast,
        _annot_type: &Option<Type>,
    ) {
        let lv = self.eval_to_reg(left);
        let rv = self.eval_to_reg(right);
        if let (Some((lt, l)), Some((_rt, r))) = (lv, rv) {
            let target = lt.clone();
            let (_, lc) = self.coerce_to(&l, &lt, &target, true);
            let (_, rc) = self.coerce_to(&r, &_rt, &target, true);
            let mut ctx = self.fn_ctx.take().unwrap();
            let res = self.fresh_reg();
            let is_cmp = matches!(
                operator,
                BinOp::Eq | BinOp::Neq | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge
            );
            if is_cmp {
                let cond = match operator {
                    BinOp::Eq => "eq",
                    BinOp::Neq => "ne",
                    BinOp::Lt => "slt",
                    BinOp::Gt => "sgt",
                    BinOp::Le => "sle",
                    BinOp::Ge => "sge",
                    _ => "eq",
                };
                ctx.emit(LlvmInstruction::Icmp {
                    result: res.clone(),
                    condition: cond.into(),
                    llvm_type: target.clone(),
                    left: lc,
                    right: rc,
                });
                ctx.last_value = Some((LlvmType::I1, res));
            } else {
                match (operator, &target) {
                    (BinOp::Add, LlvmType::I64) => ctx.emit(LlvmInstruction::Add {
                        result: res.clone(),
                        llvm_type: LlvmType::I64,
                        left: lc,
                        right: rc,
                    }),
                    (BinOp::Sub, LlvmType::I64) => ctx.emit(LlvmInstruction::Sub {
                        result: res.clone(),
                        llvm_type: LlvmType::I64,
                        left: lc,
                        right: rc,
                    }),
                    (BinOp::Mul, LlvmType::I64) => ctx.emit(LlvmInstruction::Mul {
                        result: res.clone(),
                        llvm_type: LlvmType::I64,
                        left: lc,
                        right: rc,
                    }),
                    (BinOp::Div, LlvmType::I64) => ctx.emit(LlvmInstruction::SDiv {
                        result: res.clone(),
                        left: lc,
                        right: rc,
                    }),
                    _ => {}
                }
                ctx.last_value = Some((target, res));
            }
            self.fn_ctx = Some(ctx);
        }
    }

    fn eval_to_reg(&mut self, node: &Ast) -> Option<(LlvmType, String)> {
        match node {
            Ast::Literal { value, .. } => {
                let (t, v) = self.literal_to_llvm_imm(value);
                if let Value::String(s) = value {
                    let mut ctx = self.fn_ctx.take().unwrap();
                    let reg = self.fresh_reg();
                    ctx.emit(LlvmInstruction::GetElementPtr {
                        result: reg.clone(),
                        llvm_type: LlvmType::Array(Box::new(LlvmType::I8), s.len() + 1),
                        pointer: v,
                        indices: vec![
                            (LlvmType::I32, "0".to_string()),
                            (LlvmType::I32, "0".to_string()),
                        ],
                    });
                    self.fn_ctx = Some(ctx);
                    Some((t, reg))
                } else {
                    Some((t, v))
                }
            }
            Ast::Variable { name, .. } => {
                let mut ctx = self.fn_ctx.take()?;
                let (ptr, typ) = ctx.variables.get(name)?.clone();
                let reg = self.fresh_reg();
                ctx.emit(LlvmInstruction::Load {
                    result: reg.clone(),
                    llvm_type: typ.clone(),
                    pointer: ptr,
                });
                self.fn_ctx = Some(ctx);
                Some((typ, reg))
            }
            Ast::BinExpr { .. }
            | Ast::If { .. }
            | Ast::Match { .. }
            | Ast::StructCons { .. }
            | Ast::SumCons { .. }
            | Ast::FieldAccess { .. }
            | Ast::StructUpdate { .. } => {
                self.generate_expr(node);
                let ctx = self.fn_ctx.take()?;
                let val = ctx.last_value.clone();
                self.fn_ctx = Some(ctx);
                val
            }
            _ => None,
        }
    }

    fn literal_to_llvm_imm(&mut self, value: &Value) -> (LlvmType, String) {
        match value {
            Value::Int(n) => (LlvmType::I64, n.to_string()),
            Value::Float(f) => (LlvmType::Double, format!("{}", f)),
            Value::Bool(b) => (LlvmType::I1, if *b { "1".into() } else { "0".into() }),
            Value::String(s) => {
                let name = self.add_string_constant(s);
                (LlvmType::Ptr(Box::new(LlvmType::I8)), name)
            }
            Value::Char(c) => (LlvmType::I32, (*c as u32).to_string()),
            Value::Nil => (LlvmType::Void, "void".into()),
        }
    }

    fn type_to_llvm(&self, typ: &Type) -> LlvmType {
        match typ {
            Type::Primitive(p) => match p {
                PrimitiveType::Int => LlvmType::I64,
                PrimitiveType::Float => LlvmType::Double,
                PrimitiveType::Bool => LlvmType::I1,
                PrimitiveType::String => LlvmType::Ptr(Box::new(LlvmType::I8)),
                _ => LlvmType::I64,
            },
            Type::Ref { llvm_type, .. } => LlvmType::Ptr(Box::new(self.type_to_llvm(llvm_type))),
            Type::Record(fields) => {
                LlvmType::Struct(fields.iter().map(|(_, t)| self.type_to_llvm(t)).collect())
            }
            Type::Unit => LlvmType::Void,
            _ => LlvmType::I64,
        }
    }

    fn generate_pending_monomorphizations(&mut self) {
        let pending = std::mem::take(&mut self.pending_mono);
        for (_, ast) in pending {
            self.generate_node(&ast);
        }
    }

    pub fn emit_llvm_text(ir: &LlvmIr) -> String {
        let LlvmIr::Module { name, functions, string_constants } = ir;
        let mut out = format!("; Module: {}\n\n", name);
        out.push_str("declare i32 @printf(i8*, ...) #0\n\n");
        for s in string_constants {
            out.push_str(&format!("{}\n", s));
        }
        if !string_constants.is_empty() {
            out.push('\n');
        }
        for f in functions {
            out.push_str(&Self::emit_function(f));
            out.push('\n');
        }
        out.push_str("attributes #0 = { nounwind }\n");
        out
    }

    fn emit_function(f: &LlvmFunction) -> String {
        let params: Vec<String> = f
            .params
            .iter()
            .map(|(n, t)| format!("{} %{}", t.to_llvm_string(), n))
            .collect();
        let mut out = format!(
            "define {} @{}({}) {{\n",
            f.return_type.to_llvm_string(),
            f.name,
            params.join(", ")
        );
        for b in &f.basic_blocks {
            out.push_str(&format!("{}:\n", b.label.trim_start_matches('%')));
            for i in &b.instructions {
                out.push_str(&format!("  {}\n", Self::emit_instruction(i)));
            }
            out.push_str(&format!("  {}\n", Self::emit_terminator(&b.terminator)));
        }
        out.push_str("}\n");
        out
    }

    fn emit_instruction(i: &LlvmInstruction) -> String {
        match i {
            LlvmInstruction::Add { result, llvm_type, left, right } => {
                format!("{} = add {} {}, {}", result, llvm_type.to_llvm_string(), left, right)
            }
            LlvmInstruction::Sub { result, llvm_type, left, right } => {
                format!("{} = sub {} {}, {}", result, llvm_type.to_llvm_string(), left, right)
            }
            LlvmInstruction::Mul { result, llvm_type, left, right } => {
                format!("{} = mul {} {}, {}", result, llvm_type.to_llvm_string(), left, right)
            }
            LlvmInstruction::SDiv { result, left, right } => {
                format!("{} = sdiv i64 {}, {}", result, left, right)
            }
            LlvmInstruction::SRem { result, left, right } => {
                format!("{} = srem i64 {}, {}", result, left, right)
            }
            LlvmInstruction::Icmp { result, condition, llvm_type, left, right } => {
                format!(
                    "{} = icmp {} {} {}, {}",
                    result,
                    condition,
                    llvm_type.to_llvm_string(),
                    left,
                    right
                )
            }
            LlvmInstruction::Load { result, llvm_type, pointer } => {
                format!(
                    "{} = load {}, {}* {}",
                    result,
                    llvm_type.to_llvm_string(),
                    llvm_type.to_llvm_string(),
                    pointer
                )
            }
            LlvmInstruction::Store { llvm_type, value, pointer } => {
                format!(
                    "store {} {}, {}* {}",
                    llvm_type.to_llvm_string(),
                    value,
                    llvm_type.to_llvm_string(),
                    pointer
                )
            }
            LlvmInstruction::Alloca { result, llvm_type } => {
                format!("{} = alloca {}", result, llvm_type.to_llvm_string())
            }
            LlvmInstruction::Call { result, function, arguments } => {
                let r = result.as_ref().map(|x| format!("{} = ", x)).unwrap_or_default();
                let a: Vec<String> = arguments
                    .iter()
                    .map(|(t, v)| format!("{} {}", t.to_llvm_string(), v))
                    .collect();
                format!("{}call i32 @{}({})", r, function, a.join(", "))
            }
            LlvmInstruction::Ret { value } => match value {
                Some((t, v)) => format!("ret {} {}", t.to_llvm_string(), v),
                None => "ret void".into(),
            },
            LlvmInstruction::Br { label } => {
                format!("br label {}", ensure_label_prefix(label))
            }
            LlvmInstruction::CondBr { condition, true_label, false_label } => {
                format!(
                    "br i1 {}, label {}, label {}",
                    condition,
                    ensure_label_prefix(true_label),
                    ensure_label_prefix(false_label)
                )
            }
            LlvmInstruction::GetElementPtr { result, llvm_type, pointer, indices } => {
                let idx: Vec<String> = indices
                    .iter()
                    .map(|(t, v)| format!("{} {}", t.to_llvm_string(), v))
                    .collect();
                format!(
                    "{} = getelementptr {}, {}* {}, {}",
                    result,
                    llvm_type.to_llvm_string(),
                    llvm_type.to_llvm_string(),
                    pointer,
                    idx.join(", ")
                )
            }
            LlvmInstruction::Bitcast { result, value, from, to } => {
                format!(
                    "{} = bitcast {} {} to {}",
                    result,
                    from.to_llvm_string(),
                    value,
                    to.to_llvm_string()
                )
            }
            _ => "".into(),
        }
    }

    fn emit_terminator(t: &Terminator) -> String {
        match t {
            Terminator::Ret(v) => match v {
                Some((tp, val)) => format!("ret {} {}", tp.to_llvm_string(), val),
                None => "ret void".into(),
            },
            Terminator::Br(l) => format!("br label {}", ensure_label_prefix(l)),
            Terminator::CondBr { condition, true_label, false_label } => {
                format!(
                    "br i1 {}, label {}, label {}",
                    condition,
                    ensure_label_prefix(true_label),
                    ensure_label_prefix(false_label)
                )
            }
            Terminator::Unreachable => "unreachable".into(),
        }
    }
}

fn ensure_label_prefix(label: &str) -> String {
    if label.starts_with('%') {
        label.to_string()
    } else {
        format!("%{}", label)
    }
}
