// grammalang-core/src/monomorphize.rs
// Version 1.0 — generics monomorphization

use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Instantiation {
    pub name: String,
    pub type_args: Vec<Type>,
}

impl Instantiation {
    pub fn new(name: &str, type_args: Vec<Type>) -> Self {
        Instantiation { name: name.to_string(), type_args }
    }

    pub fn mono_name(&self) -> String {
        if self.type_args.is_empty() {
            return self.name.clone();
        }
        let args_str: Vec<String> = self.type_args.iter().map(type_to_mangled).collect();
        format!("{}_{}", self.name, args_str.join("_"))
    }
}

fn type_to_mangled(typ: &Type) -> String {
    match typ {
        Type::Primitive(p) => format!("{:?}", p).to_lowercase(),
        Type::Parameterized { name, params } => {
            let args: Vec<String> = params.iter().map(type_to_mangled).collect();
            format!("{}_{}", name.to_lowercase(), args.join("_"))
        }
        Type::Ref { mutable, llvm_type } => {
            let prefix = if *mutable { "mutref" } else { "ref" };
            format!("{}_{}", prefix, type_to_mangled(llvm_type))
        }
        Type::Variable(v) => v.clone(),
        Type::Void => "void".into(),
        Type::Array { llvm_type, size: Some(n) } => format!("arr{}_{}", n, type_to_mangled(llvm_type)),
        Type::Array { llvm_type, size: None } => format!("slice_{}", type_to_mangled(llvm_type)),
        _ => format!("{:?}", typ).to_lowercase(),
    }
}

pub struct Monomorphizer {
    monomorphized: HashMap<String, String>,
    generated_decls: Vec<Ast>,
}

impl Monomorphizer {
    pub fn new() -> Self {
        Monomorphizer { monomorphized: HashMap::new(), generated_decls: Vec::new() }
    }

    pub fn monomorphize(&mut self, ast: &Ast) -> Ast {
        let mut instantiations = self.collect_instantiations(ast);
        instantiations.sort_by(|a, b| a.mono_name().cmp(&b.mono_name()));
        instantiations.dedup_by(|a, b| a.mono_name() == b.mono_name());
        for inst in &instantiations {
            let mono_name = inst.mono_name();
            if !self.monomorphized.contains_key(&mono_name) {
                self.generate_mono_version(ast, inst);
            }
        }
        let mut result = ast.clone();
        self.substitute_calls(&mut result);
        if !self.generated_decls.is_empty() {
            if let Ast::Module { declarations, .. } = &mut result {
                declarations.append(&mut self.generated_decls);
            }
        }
        result
    }

    fn collect_instantiations(&self, ast: &Ast) -> Vec<Instantiation> {
        let mut insts = Vec::new();
        self.collect_from_node(ast, &mut insts);
        insts
    }

    fn collect_from_node(&self, node: &Ast, insts: &mut Vec<Instantiation>) {
        match node {
            Ast::Module { declarations, .. } => {
                for d in declarations { self.collect_from_node(d, insts); }
            }
            Ast::Call { function, arguments, .. } => {
                if let Ast::Variable { name, .. } = function.as_ref() {
                    if let Some(types) = self.extract_types(arguments) {
                        if !types.is_empty() {
                            insts.push(Instantiation::new(name, types));
                        }
                    }
                }
                for a in arguments { self.collect_from_node(a, insts); }
            }
            Ast::StructCons { name, llvm_type: Some(Type::Parameterized { params, .. }), .. } => {
                insts.push(Instantiation::new(name, params.clone()));
            }
            Ast::SumCons { name, llvm_type: Some(Type::Parameterized { params, .. }), .. } => {
                insts.push(Instantiation::new(name, params.clone()));
            }
            Ast::Let { value, .. } | Ast::Assign { value, .. } => {
                self.collect_from_node(value, insts);
            }
            Ast::BinExpr { left, right, .. } => {
                self.collect_from_node(left, insts);
                self.collect_from_node(right, insts);
            }
            Ast::If { condition, then, else_arm, .. } => {
                self.collect_from_node(condition, insts);
                self.collect_from_node(then, insts);
                if let Some(e) = else_arm { self.collect_from_node(e, insts); }
            }
            Ast::Match { value, arms, .. } => {
                self.collect_from_node(value, insts);
                for arm in arms { self.collect_from_node(&arm.body, insts); }
            }
            Ast::Block { expressions, .. } | Ast::ScopeBlock { expressions, .. } => {
                for e in expressions { self.collect_from_node(e, insts); }
            }
            Ast::LoopWhile { condition, body, .. } | Ast::While { condition, body, .. } => {
                self.collect_from_node(condition, insts);
                self.collect_from_node(body, insts);
            }
            Ast::StructUpdate { object, fields, .. } => {
                self.collect_from_node(object, insts);
                for (_, v) in fields { self.collect_from_node(v, insts); }
            }
            Ast::Return { value: Some(v), .. } => self.collect_from_node(v, insts),
            Ast::FieldAccess { object, .. } => self.collect_from_node(object, insts),
            _ => {}
        }
    }

    fn extract_types(&self, args: &[Ast]) -> Option<Vec<Type>> {
        let mut types = Vec::new();
        for arg in args {
            types.push(self.get_node_type(arg)?);
        }
        if types.is_empty() { None } else { Some(types) }
    }

    fn get_node_type(&self, node: &Ast) -> Option<Type> {
        match node {
            Ast::Variable { llvm_type, .. }
            | Ast::BinExpr { llvm_type, .. }
            | Ast::Call { llvm_type, .. }
            | Ast::If { llvm_type, .. }
            | Ast::StructCons { llvm_type, .. }
            | Ast::SumCons { llvm_type, .. } => llvm_type.clone(),
            Ast::Literal { value, .. } => Some(match value {
                Value::Int(_) => Type::Primitive(PrimitiveType::Int),
                Value::Float(_) => Type::Primitive(PrimitiveType::Float),
                Value::Bool(_) => Type::Primitive(PrimitiveType::Bool),
                Value::String(_) => Type::Primitive(PrimitiveType::String),
                Value::Char(_) => Type::Primitive(PrimitiveType::Char),
                Value::Nil => Type::Void,
            }),
            _ => None,
        }
    }

    fn generate_mono_version(&mut self, ast: &Ast, inst: &Instantiation) {
        let mono_name = inst.mono_name();
        let original = self.find_generic_decl(ast, &inst.name);
        if let Some(decl) = original {
            let substitution = self.build_substitution(&decl, inst);
            let mut mono_decl = decl.clone();
            self.apply_type_substitution(&mut mono_decl, &substitution);
            self.rename_decl(&mut mono_decl, &mono_name);
            self.generated_decls.push(mono_decl);
            self.monomorphized.insert(mono_name.clone(), mono_name);
        }
    }

    fn find_generic_decl(&self, ast: &Ast, name: &str) -> Option<Ast> {
        if let Ast::Module { declarations, .. } = ast {
            for d in declarations {
                match d {
                    Ast::FnDecl { name: n, type_params, .. }
                        if n == name && !type_params.is_empty() => return Some(d.clone()),
                    Ast::StructDecl { name: n, type_params, .. }
                        if n == name && !type_params.is_empty() => return Some(d.clone()),
                    Ast::SumDecl { name: n, type_params, .. }
                        if n == name && !type_params.is_empty() => return Some(d.clone()),
                    _ => {}
                }
            }
        }
        None
    }

    fn build_substitution(&self, decl: &Ast, inst: &Instantiation) -> HashMap<String, Type> {
        let mut map = HashMap::new();
        let type_params: Vec<String> = match decl {
            Ast::FnDecl { type_params, .. }
            | Ast::StructDecl { type_params, .. }
            | Ast::SumDecl { type_params, .. } => {
                type_params.iter().map(|p| p.name.clone()).collect()
            }
            _ => Vec::new(),
        };
        for (i, name) in type_params.iter().enumerate() {
            if i < inst.type_args.len() {
                map.insert(name.clone(), inst.type_args[i].clone());
            }
        }
        map
    }

    fn apply_type_substitution(&self, node: &mut Ast, sub: &HashMap<String, Type>) {
        match node {
            Ast::FnDecl { type_params, params, return_type, body, .. } => {
                type_params.clear();
                for p in params { p.llvm_type = self.subst_type(&p.llvm_type, sub); }
                if let Some(ref mut rt) = return_type { *rt = self.subst_type(rt, sub); }
                self.apply_type_substitution(body, sub);
            }
            Ast::StructDecl { type_params, fields, .. } => {
                type_params.clear();
                for (_, t) in fields { *t = self.subst_type(t, sub); }
            }
            Ast::SumDecl { type_params, variants, .. } => {
                type_params.clear();
                for v in variants {
                    if let Some(ref mut t) = v.data_type { *t = self.subst_type(t, sub); }
                }
            }
            Ast::Block { expressions, .. } | Ast::ScopeBlock { expressions, .. } => {
                for e in expressions { self.apply_type_substitution(e, sub); }
            }
            Ast::Let { type_annotation, value, .. }
            | Ast::Assign { type_annotation, value, .. } => {
                if let Some(ref mut t) = type_annotation { *t = self.subst_type(t, sub); }
                self.apply_type_substitution(value, sub);
            }
            Ast::Call { function, arguments, llvm_type, .. } => {
                self.apply_type_substitution(function, sub);
                for a in arguments { self.apply_type_substitution(a, sub); }
                if let Some(ref mut t) = llvm_type { *t = self.subst_type(t, sub); }
            }
            Ast::BinExpr { left, right, llvm_type, .. } => {
                self.apply_type_substitution(left, sub);
                self.apply_type_substitution(right, sub);
                if let Some(ref mut t) = llvm_type { *t = self.subst_type(t, sub); }
            }
            Ast::If { condition, then, else_arm, llvm_type, .. } => {
                self.apply_type_substitution(condition, sub);
                self.apply_type_substitution(then, sub);
                if let Some(ref mut e) = else_arm { self.apply_type_substitution(e, sub); }
                if let Some(ref mut t) = llvm_type { *t = self.subst_type(t, sub); }
            }
            Ast::Match { value, arms, llvm_type, .. } => {
                self.apply_type_substitution(value, sub);
                for arm in arms { self.apply_type_substitution(&mut arm.body, sub); }
                if let Some(ref mut t) = llvm_type { *t = self.subst_type(t, sub); }
            }
            Ast::Variable { llvm_type, .. } => {
                if let Some(ref mut t) = llvm_type { *t = self.subst_type(t, sub); }
            }
            Ast::StructCons { fields, llvm_type, .. } => {
                if let Some(ref mut t) = llvm_type { *t = self.subst_type(t, sub); }
                for (_, v) in fields { self.apply_type_substitution(v, sub); }
            }
            Ast::SumCons { llvm_type, value, .. } => {
                if let Some(ref mut t) = llvm_type { *t = self.subst_type(t, sub); }
                if let Some(ref mut v) = value { self.apply_type_substitution(v, sub); }
            }
            Ast::FieldAccess { object, llvm_type, .. } => {
                self.apply_type_substitution(object, sub);
                if let Some(ref mut t) = llvm_type { *t = self.subst_type(t, sub); }
            }
            Ast::StructUpdate { object, fields, llvm_type, .. } => {
                self.apply_type_substitution(object, sub);
                for (_, v) in fields { self.apply_type_substitution(v, sub); }
                if let Some(ref mut t) = llvm_type { *t = self.subst_type(t, sub); }
            }
            Ast::Return { value, .. } => {
                if let Some(ref mut v) = value { self.apply_type_substitution(v, sub); }
            }
            Ast::LoopWhile { condition, body, .. } | Ast::While { condition, body, .. } => {
                self.apply_type_substitution(condition, sub);
                self.apply_type_substitution(body, sub);
            }
            _ => {}
        }
    }

    fn subst_type(&self, typ: &Type, sub: &HashMap<String, Type>) -> Type {
        match typ {
            Type::Variable(name) => sub.get(name).cloned().unwrap_or_else(|| typ.clone()),
            Type::Parameterized { name, params } => {
                let new_params: Vec<Type> = params.iter().map(|p| self.subst_type(p, sub)).collect();
                Type::Parameterized { name: name.clone(), params: new_params }
            }
            Type::Fn { arguments, result } => Type::Fn {
                arguments: arguments.iter().map(|a| self.subst_type(a, sub)).collect(),
                result: Box::new(self.subst_type(result, sub)),
            },
            Type::Record(fields) => Type::Record(
                fields.iter().map(|(n, t)| (n.clone(), self.subst_type(t, sub))).collect()
            ),
            Type::Sum(variants) => Type::Sum(
                variants.iter().map(|(n, t)| (n.clone(), t.as_ref().map(|tt| self.subst_type(tt, sub)))).collect()
            ),
            Type::Ref { mutable, llvm_type } => Type::Ref {
                mutable: *mutable,
                llvm_type: Box::new(self.subst_type(llvm_type, sub)),
            },
            Type::Array { llvm_type, size } => Type::Array {
                llvm_type: Box::new(self.subst_type(llvm_type, sub)),
                size: *size,
            },
            Type::Slice { llvm_type } => Type::Slice {
                llvm_type: Box::new(self.subst_type(llvm_type, sub)),
            },
            Type::Tuple(types) => Type::Tuple(
                types.iter().map(|t| self.subst_type(t, sub)).collect()
            ),
            _ => typ.clone(),
        }
    }

    fn rename_decl(&self, decl: &mut Ast, new_name: &str) {
        match decl {
            Ast::FnDecl { name, .. }
            | Ast::StructDecl { name, .. }
            | Ast::SumDecl { name, .. } => *name = new_name.to_string(),
            _ => {}
        }
    }

    fn substitute_calls(&mut self, node: &mut Ast) {
        match node {
            Ast::Module { declarations, .. } => {
                for d in declarations { self.substitute_calls(d); }
            }
            Ast::Call { function, arguments, .. } => {
                for a in arguments.iter_mut() {
                    self.substitute_calls(a);
                }
                if let Ast::Variable { name, .. } = function.as_ref() {
                    let types: Option<Vec<Type>> = arguments.iter()
                        .map(|a| self.get_node_type(a))
                        .collect();
                    if let Some(types) = types {
                        if !types.is_empty() {
                            let inst = Instantiation::new(name, types);
                            let mono_name = inst.mono_name();
                            if self.monomorphized.contains_key(&mono_name) {
                                if let Ast::Variable { name: ref mut n, .. } = function.as_mut() {
                                    *n = mono_name;
                                }
                            }
                        }
                    }
                }
                self.substitute_calls(function);
            }
            Ast::StructCons { name, llvm_type: Some(Type::Parameterized { params, .. }), .. } => {
                let inst = Instantiation::new(name, params.clone());
                let mono_name = inst.mono_name();
                if self.monomorphized.contains_key(&mono_name) { *name = mono_name; }
            }
            Ast::SumCons { name, llvm_type: Some(Type::Parameterized { params, .. }), .. } => {
                let inst = Instantiation::new(name, params.clone());
                let mono_name = inst.mono_name();
                if self.monomorphized.contains_key(&mono_name) { *name = mono_name; }
            }
            Ast::Block { expressions, .. } | Ast::ScopeBlock { expressions, .. } => {
                for e in expressions { self.substitute_calls(e); }
            }
            Ast::Let { value, .. } | Ast::Assign { value, .. } => self.substitute_calls(value),
            Ast::BinExpr { left, right, .. } => {
                self.substitute_calls(left); self.substitute_calls(right);
            }
            Ast::If { condition, then, else_arm, .. } => {
                self.substitute_calls(condition); self.substitute_calls(then);
                if let Some(e) = else_arm { self.substitute_calls(e); }
            }
            Ast::Match { value, arms, .. } => {
                self.substitute_calls(value);
                for arm in arms { self.substitute_calls(&mut arm.body); }
            }
            Ast::Return { value: Some(v), .. } => self.substitute_calls(v),
            Ast::StructUpdate { object, fields, .. } => {
                self.substitute_calls(object);
                for (_, v) in fields { self.substitute_calls(v); }
            }
            Ast::LoopWhile { condition, body, .. } | Ast::While { condition, body, .. } => {
                self.substitute_calls(condition); self.substitute_calls(body);
            }
            Ast::FnDecl { body, .. } => self.substitute_calls(body),
            _ => {}
        }
    }
}
