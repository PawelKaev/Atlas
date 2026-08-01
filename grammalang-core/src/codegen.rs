// grammalang-core/src/codegen.rs

use crate::ast::*;
use crate::error::Diagnostic;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum LlvmIr {
    Модуль { имя: String, функции: Vec<LlvmFunction>, строковые_константы: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct LlvmFunction {
    pub имя: String,
    pub параметры: Vec<(String, LlvmType)>,
    pub возвращаемый_тип: LlvmType,
    pub базовые_блоки: Vec<BasicBlock>,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub метка: String,
    pub инструкции: Vec<LlvmInstruction>,
    pub терминатор: Terminator,
}

#[derive(Debug, Clone)]
pub enum LlvmInstruction {
    Add { результат: String, левый: String, правый: String },
    Sub { результат: String, левый: String, правый: String },
    Mul { результат: String, левый: String, правый: String },
    Icmp { результат: String, условие: String, левый: String, правый: String },
    Load { результат: String, тип: LlvmType, указатель: String },
    Store { значение: String, указатель: String },
    Alloca { результат: String, тип: LlvmType },
    Call { результат: Option<String>, функция: String, аргументы: Vec<(LlvmType, String)> },
    Ret { значение: Option<(LlvmType, String)> },
    Br { метка: String },
    CondBr { условие: String, истина: String, ложь: String },
    GetElementPtr { результат: String, тип: LlvmType, указатель: String, индексы: Vec<(LlvmType, String)> },
    Bitcast { результат: String, значение: String, из: LlvmType, в: LlvmType },
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Ret(Option<(LlvmType, String)>),
    Br(String),
    CondBr { условие: String, истина: String, ложь: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LlvmType {
    Void,
    I1,
    I8,
    I32,
    I64,
    Double,
    Ptr(Box<LlvmType>),
    Array(Box<LlvmType>, usize),
    Struct(Vec<LlvmType>),
    Named(String),
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
            LlvmType::Struct(fields) => {
                let fs: Vec<String> = fields.iter().map(|f| f.to_llvm_string()).collect();
                format!("{{ {} }}", fs.join(", "))
            }
            LlvmType::Named(name) => format!("%{}", name),
        }
    }
}

pub struct Codegen {
    module_name: String,
    functions: Vec<LlvmFunction>,
    string_constants: Vec<String>,
    errors: Vec<Diagnostic>,
    next_reg: usize,
    next_block: usize,
    next_string: usize,
    current_function: Option<LlvmFunction>,
    variables: HashMap<String, (String, LlvmType)>,
    last_value: Option<(LlvmType, String)>,
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
            current_function: None,
            variables: HashMap::new(),
            last_value: None,
        }
    }

    pub fn generate(&mut self, ast: &Ast) -> (Option<LlvmIr>, Vec<Diagnostic>) {
        self.generate_node(ast);
        let ir = LlvmIr::Модуль {
            имя: self.module_name.clone(),
            функции: std::mem::take(&mut self.functions),
            строковые_константы: std::mem::take(&mut self.string_constants),
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
        format!("block{}", id)
    }

    fn fresh_string_name(&mut self) -> String {
        let id = self.next_string;
        self.next_string += 1;
        format!("@.str.{}", id)
    }

    fn add_string_constant(&mut self, s: &str) -> String {
        let name = self.fresh_string_name();
        self.string_constants.push(format!("{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"", name, s.len() + 1, s));
        name
    }

    fn generate_node(&mut self, node: &Ast) {
        match node {
            Ast::Модуль { объявления, .. } => {
                for decl in объявления {
                    self.generate_node(decl);
                }
            }

            Ast::ОбъявлениеФункции { имя, параметры, возвращаемый_тип, тело, .. } => {
                self.variables.clear();
                let func = LlvmFunction {
                    имя: имя.clone(),
                    параметры: параметры.iter().map(|p| (p.имя.clone(), self.type_to_llvm(&p.тип))).collect(),
                    возвращаемый_тип: self.type_to_llvm(&возвращаемый_тип.clone().unwrap_or(Тип::Пустой)),
                    базовые_блоки: Vec::new(),
                };
                self.current_function = Some(func);
                self.generate_node(тело);
                if let Some(f) = self.current_function.take() {
                    self.functions.push(f);
                }
            }

            Ast::Блок { выражения, .. } => {
                let entry_block = self.fresh_block();
                let mut block = BasicBlock {
                    метка: entry_block.clone(),
                    инструкции: Vec::new(),
                    терминатор: Terminator::Ret(None),
                };

                for expr in выражения {
                    self.generate_expr(expr, &mut block);
                }

                if let Some(ref mut func) = self.current_function {
                    func.базовые_блоки.push(block);
                }
            }

            Ast::Возврат { значение, .. } => {
                let val = значение.as_ref().and_then(|v| self.eval_standalone(v));
                if let Some(ref mut func) = self.current_function {
                    if let Some(last_block) = func.базовые_блоки.last_mut() {
                        last_block.терминатор = Terminator::Ret(val);
                    }
                }
            }

            _ => {}
        }
    }

    fn generate_expr(&mut self, node: &Ast, block: &mut BasicBlock) {
        match node {
            Ast::Присваивание { имя, значение, .. } => {
                if let Some((val_type, val)) = self.eval_to_reg(значение, block) {
                    let ptr = self.fresh_reg();
                    block.инструкции.push(LlvmInstruction::Alloca {
                        результат: ptr.clone(),
                        тип: val_type.clone(),
                    });
                    block.инструкции.push(LlvmInstruction::Store {
                        значение: val.clone(),
                        указатель: ptr.clone(),
                    });
                    self.variables.insert(имя.clone(), (ptr, val_type));
                }
            }

            Ast::Вызов { функция, аргументы, .. } => {
                if let Ast::Переменная { имя, .. } = функция.as_ref() {
                    if имя == "написать" || имя == "консоль.написать" {
                        if let Some(arg) = аргументы.first() {
                            match arg {
                                Ast::Литерал { значение: Значение::Строка(s), .. } => {
                                    let str_name = self.add_string_constant(s);
                                    let ptr = self.fresh_reg();
                                    block.инструкции.push(LlvmInstruction::GetElementPtr {
                                        результат: ptr.clone(),
                                        тип: LlvmType::Array(Box::new(LlvmType::I8), s.len() + 1),
                                        указатель: str_name.clone(),
                                        индексы: vec![
                                            (LlvmType::I32, "0".to_string()),
                                            (LlvmType::I32, "0".to_string()),
                                        ],
                                    });
                                    let result = self.fresh_reg();
                                    block.инструкции.push(LlvmInstruction::Call {
                                        результат: Some(result),
                                        функция: "printf".to_string(),
                                        аргументы: vec![(LlvmType::Ptr(Box::new(LlvmType::I8)), ptr)],
                                    });
                                }
                                _ => {
                                    if let Some((val_type, val)) = self.eval_to_reg(arg, block) {
                                        let format_str = self.add_string_constant("%d\\n");
                                        let fmt_ptr = self.fresh_reg();
                                        block.инструкции.push(LlvmInstruction::GetElementPtr {
                                            результат: fmt_ptr.clone(),
                                            тип: LlvmType::Array(Box::new(LlvmType::I8), 4),
                                            указатель: format_str,
                                            индексы: vec![
                                                (LlvmType::I32, "0".to_string()),
                                                (LlvmType::I32, "0".to_string()),
                                            ],
                                        });
                                        let result = self.fresh_reg();
                                        block.инструкции.push(LlvmInstruction::Call {
                                            результат: Some(result),
                                            функция: "printf".to_string(),
                                            аргументы: vec![
                                                (LlvmType::Ptr(Box::new(LlvmType::I8)), fmt_ptr),
                                                (val_type, val),
                                            ],
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Ast::ДвоичноеВыражение { левое, оператор, правое, .. } => {
                let left_val = self.eval_to_reg(левое, block);
                let right_val = self.eval_to_reg(правое, block);

                if let (Some((_, l)), Some((_, r))) = (left_val, right_val) {
                    let result = self.fresh_reg();
                    let instr = match оператор {
                        БинарныйОператор::Сложение => LlvmInstruction::Add { результат: result.clone(), левый: l, правый: r },
                        БинарныйОператор::Вычитание => LlvmInstruction::Sub { результат: result.clone(), левый: l, правый: r },
                        БинарныйОператор::Умножение => LlvmInstruction::Mul { результат: result.clone(), левый: l, правый: r },
                        _ => return,
                    };
                    block.инструкции.push(instr);
                    self.last_value = Some((LlvmType::I64, result));
                }
            }

            _ => {}
        }
    }

    fn eval_to_reg(&mut self, node: &Ast, block: &mut BasicBlock) -> Option<(LlvmType, String)> {
        match node {
            Ast::Литерал { значение, .. } => match значение {
                Значение::Целое(n) => Some((LlvmType::I64, n.to_string())),
                Значение::Десятичное(f) => Some((LlvmType::Double, f.to_string())),
                Значение::Булево(b) => Some((LlvmType::I1, if *b { "1".to_string() } else { "0".to_string() })),
                Значение::Строка(_) => None,
                Значение::Ничего => None,
            },
            Ast::Переменная { имя, .. } => {
                if let Some((ptr, typ)) = self.variables.get(имя) {
                    let ptr_clone = ptr.clone();
                    let typ_clone = typ.clone();
                    drop(ptr);
                    drop(typ);
                    let reg = self.fresh_reg();
                    block.инструкции.push(LlvmInstruction::Load {
                        результат: reg.clone(),
                        тип: typ_clone.clone(),
                        указатель: ptr_clone,
                    });
                    Some((typ_clone, reg))
                } else {
                    None
                }
            }
            Ast::ДвоичноеВыражение { .. } => {
                self.generate_expr(node, block);
                self.last_value.take()
            }
            _ => None,
        }
    }

    fn eval_standalone(&self, node: &Ast) -> Option<(LlvmType, String)> {
        match node {
            Ast::Литерал { значение, .. } => match значение {
                Значение::Целое(n) => Some((LlvmType::I64, n.to_string())),
                Значение::Десятичное(f) => Some((LlvmType::Double, f.to_string())),
                Значение::Булево(b) => Some((LlvmType::I1, if *b { "1".to_string() } else { "0".to_string() })),
                Значение::Строка(_) => None,
                Значение::Ничего => None,
            },
            Ast::Переменная { имя, .. } => {
                if let Some((ptr, typ)) = self.variables.get(имя) {
                    Some((typ.clone(), ptr.clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn type_to_llvm(&self, typ: &Тип) -> LlvmType {
        match typ {
            Тип::Примитивный(p) => match p {
                ПримитивныйТип::Целое => LlvmType::I64,
                ПримитивныйТип::Десятичное => LlvmType::Double,
                ПримитивныйТип::Булево => LlvmType::I1,
                ПримитивныйТип::Строка => LlvmType::Ptr(Box::new(LlvmType::I8)),
            },
            Тип::Ссылка { тип, .. } => LlvmType::Ptr(Box::new(self.type_to_llvm(тип))),
            Тип::Пустой => LlvmType::Void,
            _ => LlvmType::I64,
        }
    }

    pub fn emit_llvm_text(ir: &LlvmIr) -> String {
        let mut output = String::new();

        let LlvmIr::Модуль { имя, функции, строковые_константы } = ir;

        output.push_str(&format!("; Модуль: {}\n\n", имя));
        output.push_str("declare i32 @printf(i8*, ...) #0\n\n");

        for s in строковые_константы {
            output.push_str(&format!("{}\n", s));
        }
        if !строковые_константы.is_empty() {
            output.push('\n');
        }

        for func in функции {
            output.push_str(&Codegen::emit_function(func));
            output.push('\n');
        }

        output.push_str("attributes #0 = { nounwind }\n");
        output
    }

    fn emit_function(func: &LlvmFunction) -> String {
        let mut out = String::new();
        let params: Vec<String> = func.параметры.iter()
            .map(|(name, typ)| format!("{} {}", typ.to_llvm_string(), name))
            .collect();

        out.push_str(&format!(
            "define {} @{}({}) {{\n",
            func.возвращаемый_тип.to_llvm_string(),
            func.имя,
            params.join(", ")
        ));

        for block in &func.базовые_блоки {
            out.push_str(&format!("{}:\n", block.метка));
            for instr in &block.инструкции {
                out.push_str(&format!("  {}\n", Codegen::emit_instruction(instr)));
            }
            out.push_str(&format!("  {}\n", Codegen::emit_terminator(&block.терминатор)));
        }

        out.push_str("}\n");
        out
    }

    fn emit_instruction(instr: &LlvmInstruction) -> String {
        match instr {
            LlvmInstruction::Add { результат, левый, правый } =>
                format!("{} = add i64 {}, {}", результат, левый, правый),
            LlvmInstruction::Sub { результат, левый, правый } =>
                format!("{} = sub i64 {}, {}", результат, левый, правый),
            LlvmInstruction::Mul { результат, левый, правый } =>
                format!("{} = mul i64 {}, {}", результат, левый, правый),
            LlvmInstruction::Icmp { результат, условие, левый, правый } =>
                format!("{} = icmp {} i64 {}, {}", результат, условие, левый, правый),
            LlvmInstruction::Load { результат, тип, указатель } =>
                format!("{} = load {}, {}* {}", результат, тип.to_llvm_string(), тип.to_llvm_string(), указатель),
            LlvmInstruction::Store { значение, указатель } =>
                format!("store i64 {}, i64* {}", значение, указатель),
            LlvmInstruction::Alloca { результат, тип } =>
                format!("{} = alloca {}", результат, тип.to_llvm_string()),
            LlvmInstruction::Call { результат, функция, аргументы } => {
                let res = if let Some(r) = результат { format!("{} = ", r) } else { String::new() };
                let args_str: Vec<String> = аргументы.iter()
                    .map(|(t, v)| format!("{} {}", t.to_llvm_string(), v))
                    .collect();
                format!("{}call i32 @{}({})", res, функция, args_str.join(", "))
            }
            LlvmInstruction::Ret { значение } => {
                if let Some((t, v)) = значение {
                    format!("ret {} {}", t.to_llvm_string(), v)
                } else {
                    "ret void".to_string()
                }
            }
            LlvmInstruction::Br { метка } => format!("br label %{}", метка),
            LlvmInstruction::CondBr { условие, истина, ложь } =>
                format!("br i1 {}, label %{}, label %{}", условие, истина, ложь),
            LlvmInstruction::GetElementPtr { результат, тип, указатель, индексы } => {
                let idx_str: Vec<String> = индексы.iter()
                    .map(|(t, v)| format!("{} {}", t.to_llvm_string(), v))
                    .collect();
                format!("{} = getelementptr {}, {}* {}, {}", результат, тип.to_llvm_string(), тип.to_llvm_string(), указатель, idx_str.join(", "))
            }
            LlvmInstruction::Bitcast { результат, значение, из, в } =>
                format!("{} = bitcast {} {} to {}", результат, из.to_llvm_string(), значение, в.to_llvm_string()),
        }
    }

    fn emit_terminator(term: &Terminator) -> String {
        match term {
            Terminator::Ret(val) => {
                if let Some((t, v)) = val {
                    format!("ret {} {}", t.to_llvm_string(), v)
                } else {
                    "ret void".to_string()
                }
            }
            Terminator::Br(label) => format!("br label %{}", label),
            Terminator::CondBr { условие, истина, ложь } =>
                format!("br i1 {}, label %{}, label %{}", условие, истина, ложь),
        }
    }
}
