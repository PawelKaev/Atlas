// grammalang-core/src/infer.rs
// Version 2.0 — full pattern checking (patcheck)

use std::collections::{HashMap, HashSet};
use crate::ast::*;
use crate::error::{Diagnostic, DiagnosticKind};
use crate::token::Span;
use crate::types::{Constraint, Substitution, fresh_var, occurs_in};

type TypeContext = HashMap<String, Type>;

#[derive(Debug, Clone, Default)]
pub struct BindingsMap {
    bindings: Vec<(String, Type)>,
}

impl BindingsMap {
    pub fn new() -> Self { BindingsMap { bindings: Vec::new() } }
    pub fn singleton(name: String, typ: Type) -> Self { BindingsMap { bindings: vec![(name, typ)] } }
    pub fn empty() -> Self { BindingsMap::new() }
    
    pub fn insert(&mut self, name: String, typ: Type) {
        self.bindings.push((name, typ));
    }
    
    pub fn merge(&mut self, other: BindingsMap) {
        self.bindings.extend(other.bindings);
    }
    
    pub fn names(&self) -> Vec<String> {
        self.bindings.iter().map(|(n, _)| n.clone()).collect()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &(String, Type)> {
        self.bindings.iter()
    }
    
    pub fn into_context(self, ctx: &mut TypeContext) {
        for (name, typ) in self.bindings {
            ctx.insert(name, typ);
        }
    }
}

pub struct Inferrer {
    context: TypeContext,
    constraints: Vec<Constraint>,
    errors: Vec<Diagnostic>,
    expected_return_type: Option<Type>,
    concepts: HashMap<String, Vec<Type>>,
    struct_schemas: HashMap<String, Vec<(String, Type)>>,
    sum_schemas: HashMap<String, Vec<(String, Option<Type>)>>,
}

impl Inferrer {
    pub fn new() -> Self {
        let mut concepts = HashMap::new();
        concepts.insert("Num".to_string(), vec![
            Type::Primitive(PrimitiveType::Int),
            Type::Primitive(PrimitiveType::Float),
        ]);
        concepts.insert("Comparable".to_string(), vec![
            Type::Primitive(PrimitiveType::Int),
            Type::Primitive(PrimitiveType::Float),
            Type::Primitive(PrimitiveType::String),
            Type::Primitive(PrimitiveType::Bool),
            Type::Primitive(PrimitiveType::Char),
        ]);
        concepts.insert("Stringable".to_string(), vec![
            Type::Primitive(PrimitiveType::String),
        ]);
        concepts.insert("Iterable".to_string(), vec![
            Type::Array { llvm_type: Box::new(Type::Variable("T".to_string())), size: None },
            Type::Slice { llvm_type: Box::new(Type::Variable("T".to_string())) },
            Type::Range,
        ]);
        Inferrer {
            context: HashMap::new(),
            constraints: Vec::new(),
            errors: Vec::new(),
            expected_return_type: None,
            concepts,
            struct_schemas: HashMap::new(),
            sum_schemas: HashMap::new(),
        }
    }

    pub fn register_struct_schema(&mut self, name: &str, fields: Vec<(String, Type)>) {
        self.struct_schemas.insert(name.to_string(), fields);
    }

    pub fn register_sum_schema(&mut self, name: &str, variants: Vec<(String, Option<Type>)>) {
        self.sum_schemas.insert(name.to_string(), variants);
    }

    pub fn infer(&mut self, ast: &Ast) -> (Option<Ast>, Vec<Diagnostic>) {
        self.collect_schemas(ast);
        let typed = self.infer_node(ast);
        let mut typed = match typed {
            Some(ast) => ast,
            None => return (None, std::mem::take(&mut self.errors)),
        };
        match self.solve() {
            Ok(mut sub) => {
                sub.compress_all();
                apply_substitution_to_ast(&mut typed, &sub);
                (Some(typed), std::mem::take(&mut self.errors))
            }
            Err(mut solve_errors) => {
                self.errors.append(&mut solve_errors);
                (Some(typed), std::mem::take(&mut self.errors))
            }
        }
    }

    fn collect_schemas(&mut self, ast: &Ast) {
        match ast {
            Ast::Module { declarations, .. } => {
                for d in declarations {
                    self.collect_schemas(d);
                }
            }
            Ast::StructDecl { name, fields, .. } => {
                self.register_struct_schema(name, fields.clone());
            }
            Ast::SumDecl { name, variants, .. } => {
                let vars: Vec<(String, Option<Type>)> = variants
                    .iter()
                    .map(|v| (v.name.clone(), v.data_type.clone()))
                    .collect();
                self.register_sum_schema(name, vars);
            }
            _ => {}
        }
    }

    fn solve(&mut self) -> Result<Substitution, Vec<Diagnostic>> {
        let mut sub = Substitution::new();
        for constraint in std::mem::take(&mut self.constraints) {
            match constraint {
                Constraint::Equality(t1, t2, span) => {
                    let t1 = sub.apply_mut(&t1);
                    let t2 = sub.apply_mut(&t2);
                    if let Err(mut errors) = self.unify(&t1, &t2, &mut sub) {
                        for err in &mut errors {
                            if err.span.line == 0 {
                                err.span = span;
                            }
                        }
                        return Err(errors);
                    }
                }
                Constraint::Concept(typ, concept, span) => {
                    let resolved = sub.apply_mut(&typ);
                    if !self.check_concept(&resolved, &concept) {
                        return Err(vec![Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!(
                                "Type '{}' does not satisfy concept '{}'",
                                self.type_to_string(&resolved),
                                concept
                            ),
                            span,
                            hint: Some(format!(
                                "Concept '{}' requires: {}",
                                concept,
                                self.concept_types_string(&concept)
                            )),
                        }]);
                    }
                }
                _ => continue,
            }
        }
        Ok(sub)
    }

    fn unify(&mut self, t1: &Type, t2: &Type, sub: &mut Substitution) -> Result<(), Vec<Diagnostic>> {
        let t1 = sub.apply_mut(t1);
        let t2 = sub.apply_mut(t2);
        if t1 == t2 {
            return Ok(());
        }
        match (&t1, &t2) {
            (Type::Variable(v), other) | (other, Type::Variable(v)) => {
                if occurs_in(v, other) {
                    return Err(vec![Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: format!(
                            "Infinite type: '{}' contains '{}'",
                            v,
                            self.type_to_string(other)
                        ),
                        span: Span { line: 0, column: 0, offset: 0 },
                        hint: None,
                    }]);
                }
                sub.insert(v.clone(), other.clone());
                Ok(())
            }
            (Type::Primitive(p1), Type::Primitive(p2)) => {
                if p1 == p2 {
                    Ok(())
                } else {
                    Err(vec![self.type_mismatch(&t1, &t2)])
                }
            }
            (Type::Fn { arguments: a1, result: r1 }, Type::Fn { arguments: a2, result: r2 }) => {
                if a1.len() != a2.len() {
                    return Err(vec![Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: format!("Arity mismatch: {} vs {}", a1.len(), a2.len()),
                        span: Span { line: 0, column: 0, offset: 0 },
                        hint: None,
                    }]);
                }
                for (x, y) in a1.iter().zip(a2) {
                    self.unify(x, y, sub)?;
                }
                self.unify(r1, r2, sub)
            }
            (Type::Record(f1), Type::Record(f2)) => {
                if f1.len() != f2.len() {
                    return Err(vec![Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: format!("Record size mismatch: {} vs {}", f1.len(), f2.len()),
                        span: Span { line: 0, column: 0, offset: 0 },
                        hint: None,
                    }]);
                }
                for ((n1, t1), (n2, t2)) in f1.iter().zip(f2) {
                    if n1 != n2 {
                        return Err(vec![Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!("Field '{}' vs '{}'", n1, n2),
                            span: Span { line: 0, column: 0, offset: 0 },
                            hint: None,
                        }]);
                    }
                    self.unify(t1, t2, sub)?;
                }
                Ok(())
            }
            _ => Err(vec![self.type_mismatch(&t1, &t2)]),
        }
    }

    fn check_concept(&self, typ: &Type, concept: &str) -> bool {
        self.concepts
            .get(concept)
            .map_or(true, |allowed| allowed.iter().any(|a| self.types_match(typ, a)))
    }

    fn types_match(&self, t1: &Type, t2: &Type) -> bool {
        match (t1, t2) {
            (Type::Primitive(p1), Type::Primitive(p2)) => p1 == p2,
            (Type::Variable(_), _) | (_, Type::Variable(_)) => true,
            _ => false,
        }
    }

    fn concept_types_string(&self, c: &str) -> String {
        self.concepts
            .get(c)
            .map_or("any".into(), |t| {
                t.iter()
                    .map(|x| self.type_to_string(x))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
    }

    // ==================== check_pattern ====================

    fn check_pattern(
        &mut self,
        pattern: &Pattern,
        expected_type: &Type,
        span: Span,
    ) -> Result<BindingsMap, Vec<Diagnostic>> {
        match pattern {
            Pattern::Variable(name) => {
                Ok(BindingsMap::singleton(name.clone(), expected_type.clone()))
            }

            Pattern::Wildcard => Ok(BindingsMap::empty()),

            Pattern::Literal(val) => {
                let lit_type = self.literal_type(val);
                match self.unify_silent(expected_type, &lit_type) {
                    Ok(_) => Ok(BindingsMap::empty()),
                    Err(e) => Err(vec![e]),
                }
            }

            Pattern::Constructor { name, nested } => {
                let variants = self.resolve_sum_variants(expected_type);

                let variant = variants
                    .iter()
                    .find(|(n, _)| n == name)
                    .ok_or_else(|| vec![Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: format!(
                            "Variant '{}' not found in type '{}'",
                            name,
                            self.type_to_string(expected_type)
                        ),
                        span,
                        hint: None,
                    }])?;

                match (&variant.1, nested) {
                    (Some(var_type), Some(inner_pattern)) => {
                        self.check_pattern(inner_pattern, var_type, span)
                    }
                    (None, None) => Ok(BindingsMap::empty()),
                    (Some(_), None) => Err(vec![Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: format!("Variant '{}' requires a nested pattern", name),
                        span,
                        hint: None,
                    }]),
                    (None, Some(_)) => Err(vec![Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: format!("Variant '{}' does not accept a nested pattern", name),
                        span,
                        hint: None,
                    }]),
                }
            }

            Pattern::Struct { name, fields, open: _ } => {
                let struct_fields = self.resolve_struct_fields(expected_type, name);

                let mut bindings = BindingsMap::empty();

                for (field_name, field_pattern) in fields {
                    let field_type = struct_fields
                        .iter()
                        .find(|(n, _)| n == field_name)
                        .map(|(_, t)| t.clone())
                        .ok_or_else(|| vec![Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!(
                                "Field '{}' not found in struct '{}'",
                                field_name, name
                            ),
                            span,
                            hint: None,
                        }])?;

                    bindings.merge(self.check_pattern(field_pattern, &field_type, span)?);
                }

                Ok(bindings)
            }

            Pattern::Or(left, right) => {
                let left_bindings = self.check_pattern(left, expected_type, span)?;
                let _ = self.check_pattern(right, expected_type, span);
                Ok(left_bindings)
            }

            Pattern::Binding { name, pattern } => {
                let mut bindings = self.check_pattern(pattern, expected_type, span)?;
                bindings.insert(name.clone(), expected_type.clone());
                Ok(bindings)
            }

            Pattern::Tuple(elements) => {
                let tuple_types = match expected_type {
                    Type::Tuple(types) => types.clone(),
                    _ => return Err(vec![Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: "Tuple pattern requires a tuple type".to_string(),
                        span,
                        hint: None,
                    }]),
                };

                if elements.len() != tuple_types.len() {
                    return Err(vec![Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: format!(
                            "Tuple length mismatch: {} vs {}",
                            elements.len(),
                            tuple_types.len()
                        ),
                        span,
                        hint: None,
                    }]);
                }

                let mut bindings = BindingsMap::empty();
                for (elem_pattern, elem_type) in elements.iter().zip(tuple_types.iter()) {
                    bindings.merge(self.check_pattern(elem_pattern, elem_type, span)?);
                }
                Ok(bindings)
            }

            _ => Err(vec![Diagnostic {
                kind: DiagnosticKind::Error,
                message: format!("Unsupported pattern: {:?}", pattern),
                span,
                hint: None,
            }]),
        }
    }

    fn resolve_sum_variants(&self, expected_type: &Type) -> Vec<(String, Option<Type>)> {
        match expected_type {
            Type::Sum(v) => v.clone(),
            Type::Variable(name) => self.sum_schemas.get(name).cloned().unwrap_or_default(),
            Type::Parameterized { name, .. } => self.sum_schemas.get(name).cloned().unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    fn resolve_struct_fields(&self, expected_type: &Type, _struct_name: &str) -> Vec<(String, Type)> {
        match expected_type {
            Type::Record(f) => f.clone(),
            Type::Variable(name) => self.struct_schemas.get(name).cloned().unwrap_or_default(),
            Type::Parameterized { name, .. } => self.struct_schemas.get(name).cloned().unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    fn unify_silent(&self, t1: &Type, t2: &Type) -> Result<(), Diagnostic> {
        if t1 == t2 {
            return Ok(());
        }
        match (t1, t2) {
            (Type::Variable(_), _) | (_, Type::Variable(_)) => Ok(()),
            (Type::Primitive(p1), Type::Primitive(p2)) if p1 == p2 => Ok(()),
            _ => Err(self.type_mismatch(t1, t2)),
        }
    }

    fn literal_type(&self, val: &Value) -> Type {
        match val {
            Value::Int(_) => Type::Primitive(PrimitiveType::Int),
            Value::Float(_) => Type::Primitive(PrimitiveType::Float),
            Value::String(_) => Type::Primitive(PrimitiveType::String),
            Value::Bool(_) => Type::Primitive(PrimitiveType::Bool),
            Value::Char(_) => Type::Primitive(PrimitiveType::Char),
            Value::Nil => Type::Void,
        }
    }

    // ==================== infer_node ====================

    fn infer_node(&mut self, node: &Ast) -> Option<Ast> {
        match node {
            Ast::Module { name, declarations, span } => {
                Some(Ast::Module {
                    name: name.clone(),
                    declarations: declarations.iter().filter_map(|d| self.infer_node(d)).collect(),
                    span: *span,
                })
            }
            Ast::FnDecl { name, type_params, params, return_type, body, public, span } => {
                let saved = self.expected_return_type.clone();
                self.expected_return_type = return_type.clone();
                let mut type_param_map = HashMap::new();
                for tp in type_params {
                    let v = fresh_var();
                    type_param_map.insert(tp.name.clone(), v.clone());
                    self.context.insert(tp.name.clone(), v);
                }
                let resolved_params: Vec<Parameter> = params
                    .iter()
                    .map(|p| Parameter {
                        name: p.name.clone(),
                        llvm_type: substitute_type_vars(&p.llvm_type, &type_param_map),
                        mutable: p.mutable,
                    })
                    .collect();
                let mut saved_vars = Vec::new();
                for p in &resolved_params {
                    let t = if p.llvm_type != Type::Void {
                        p.llvm_type.clone()
                    } else {
                        fresh_var()
                    };
                    saved_vars.push((p.name.clone(), self.context.insert(p.name.clone(), t)));
                }
                let typed_body = self.infer_node(body)?;
                for (name, old) in saved_vars {
                    if let Some(t) = old {
                        self.context.insert(name, t);
                    } else {
                        self.context.remove(&name);
                    }
                }
                self.expected_return_type = saved;
                Some(Ast::FnDecl {
                    name: name.clone(),
                    type_params: type_params.clone(),
                    params: resolved_params,
                    return_type: return_type.clone(),
                    body: Box::new(typed_body),
                    public: *public,
                    span: *span,
                })
            }
            Ast::Block { expressions, span } => {
                Some(Ast::Block {
                    expressions: expressions.iter().filter_map(|e| self.infer_node(e)).collect(),
                    span: *span,
                })
            }
            Ast::Let { name, type_annotation, mutable, value, span }
            | Ast::Assign { name, type_annotation, mutable, value, span } => {
                let typed_val = self.infer_node(value)?;
                let _vt = self.get_type(&typed_val).unwrap_or_else(fresh_var);
                if let (Some(annot), Some(ref vtp)) = (type_annotation, &self.get_type(&typed_val)) {
                    self.constraints.push(Constraint::Equality(vtp.clone(), annot.clone(), *span));
                }
                let ft = type_annotation
                    .clone()
                    .or_else(|| self.get_type(&typed_val))
                    .unwrap_or_else(fresh_var);
                self.context.insert(name.clone(), ft.clone());
                Some(Ast::Let {
                    name: name.clone(),
                    type_annotation: Some(ft.clone()),
                    mutable: *mutable,
                    value: Box::new(typed_val),
                    span: *span,
                })
            }
            Ast::OpAssign { name, operator, value, span } => {
                let typed_val = self.infer_node(value)?;
                let vt = self.get_type(&typed_val).unwrap_or_else(fresh_var);
                if let Some(var_type) = self.context.get(name) {
                    self.constraints.push(Constraint::Equality(var_type.clone(), vt, *span));
                }
                Some(Ast::OpAssign {
                    name: name.clone(),
                    operator: operator.clone(),
                    value: Box::new(typed_val),
                    span: *span,
                })
            }
            Ast::PatternAssign { pattern, value, span } => {
                let typed_val = self.infer_node(value)?;
                let vt = self.get_type(&typed_val).unwrap_or_else(fresh_var);
                if let Ok(bindings) = self.check_pattern(pattern, &vt, *span) {
                    bindings.into_context(&mut self.context);
                }
                Some(Ast::PatternAssign {
                    pattern: pattern.clone(),
                    value: Box::new(typed_val),
                    span: *span,
                })
            }

            Ast::StructUpdate { object, fields, span, .. } => {
                let typed_obj = self.infer_node(object)?;
                let obj_type = self.get_type(&typed_obj).unwrap_or_else(fresh_var);
                if let Type::Record(existing_fields) = &obj_type {
                    for (name, _) in fields {
                        if !existing_fields.iter().any(|(n, _)| n == name) {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: format!("Field '{}' not found in struct", name),
                                span: *span,
                                hint: None,
                            });
                            return None;
                        }
                    }
                    let typed_fields: Vec<(String, Ast)> = fields
                        .iter()
                        .map(|(n, v)| self.infer_node(v).map(|ast| (n.clone(), ast)))
                        .collect::<Option<Vec<_>>>()?;
                    Some(Ast::StructUpdate {
                        object: Box::new(typed_obj),
                        fields: typed_fields,
                        llvm_type: Some(obj_type.clone()),
                        span: *span,
                    })
                } else {
                    self.errors.push(Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: "Struct update is only possible for records".to_string(),
                        span: *span,
                        hint: None,
                    });
                    None
                }
            }

            Ast::Match { value, arms, span, .. } => {
                let typed_val = self.infer_node(value)?;
                let val_type = self.get_type(&typed_val).unwrap_or_else(fresh_var);

                let mut typed_branches = Vec::new();
                let mut result_type: Option<Type> = None;

                for arm in arms {
                    match self.check_pattern(&arm.pattern, &val_type, *span) {
                        Ok(bindings) => {
                            let saved_context = self.context.clone();
                            bindings.into_context(&mut self.context);

                            let typed_guard = arm
                                .condition
                                .as_ref()
                                .and_then(|g| self.infer_node(g))
                                .map(Box::new);

                            let typed_body = self.infer_node(&arm.body)?;
                            let body_type = self.get_type(&typed_body).unwrap_or_else(fresh_var);

                            match &result_type {
                                Some(first) => {
                                    self.constraints.push(Constraint::Equality(
                                        body_type.clone(),
                                        first.clone(),
                                        *span,
                                    ));
                                }
                                None => {
                                    result_type = Some(body_type);
                                }
                            }

                            self.context = saved_context;
                            typed_branches.push(MatchArm {
                                pattern: arm.pattern.clone(),
                                condition: typed_guard,
                                body: Box::new(typed_body),
                            });
                        }
                        Err(mut errs) => {
                            self.errors.append(&mut errs);
                            return None;
                        }
                    }
                }

                let final_type = result_type.unwrap_or(Type::Void);
                Some(Ast::Match {
                    value: Box::new(typed_val),
                    arms: typed_branches,
                    llvm_type: Some(final_type.clone()),
                    span: *span,
                })
            }

            Ast::BinExpr { left, operator, right, span, .. } => {
                let tl = self.infer_node(left)?;
                let tr = self.infer_node(right)?;
                let lt = self.get_type(&tl).unwrap_or_else(fresh_var);
                let rt = self.get_type(&tr).unwrap_or_else(fresh_var);
                let result = match operator {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Rem => {
                        self.constraints.push(Constraint::Equality(lt.clone(), rt.clone(), *span));
                        lt.clone()
                    }
                    BinOp::Eq | BinOp::Neq | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => {
                        self.constraints.push(Constraint::Equality(lt.clone(), rt.clone(), *span));
                        Type::Primitive(PrimitiveType::Bool)
                    }
                    BinOp::And | BinOp::Or => Type::Primitive(PrimitiveType::Bool),
                    _ => lt.clone(),
                };
                Some(Ast::BinExpr {
                    left: Box::new(tl),
                    operator: operator.clone(),
                    right: Box::new(tr),
                    llvm_type: Some(result),
                    span: *span,
                })
            }
            Ast::If { condition, then, else_arm, span, .. } => {
                let tc = self.infer_node(condition)?;
                let tt = self.infer_node(then)?;
                let te = else_arm.as_ref().and_then(|e| self.infer_node(e));
                let ct = self.get_type(&tc).unwrap_or_else(fresh_var);
                self.constraints.push(Constraint::Equality(
                    ct,
                    Type::Primitive(PrimitiveType::Bool),
                    *span,
                ));
                let tt_type = self.get_type(&tt).unwrap_or_else(fresh_var);
                let result = if let Some(ref te_ast) = te {
                    let et = self.get_type(te_ast).unwrap_or_else(fresh_var);
                    self.constraints.push(Constraint::Equality(tt_type.clone(), et, *span));
                    tt_type
                } else {
                    Type::Void
                };
                Some(Ast::If {
                    condition: Box::new(tc),
                    then: Box::new(tt),
                    else_arm: te.map(Box::new),
                    llvm_type: Some(result),
                    span: *span,
                })
            }
            Ast::While { condition, body, span }
            | Ast::LoopWhile { condition, body, span, .. } => {
                let tc = self.infer_node(condition)?;
                let tb = self.infer_node(body)?;
                self.constraints.push(Constraint::Equality(
                    self.get_type(&tc).unwrap_or_else(fresh_var),
                    Type::Primitive(PrimitiveType::Bool),
                    *span,
                ));
                Some(Ast::While {
                    condition: Box::new(tc),
                    body: Box::new(tb),
                    span: *span,
                })
            }
            Ast::Call { function, arguments, span, .. } => {
                let tf = self.infer_node(function)?;
                let ta: Vec<Ast> = arguments.iter().filter_map(|a| self.infer_node(a)).collect();
                let result = fresh_var();
                let arg_types: Vec<Type> = ta
                    .iter()
                    .map(|a| self.get_type(a).unwrap_or_else(fresh_var))
                    .collect();
                let ft = Type::Fn {
                    arguments: arg_types,
                    result: Box::new(result.clone()),
                };
                self.constraints.push(Constraint::Equality(
                    self.get_type(&tf).unwrap_or_else(fresh_var),
                    ft,
                    *span,
                ));
                Some(Ast::Call {
                    function: Box::new(tf),
                    arguments: ta,
                    llvm_type: Some(result),
                    span: *span,
                })
            }
            Ast::Variable { name, span, .. } => {
                let t = self.context.get(name).cloned().unwrap_or_else(fresh_var);
                Some(Ast::Variable {
                    name: name.clone(),
                    llvm_type: Some(t),
                    span: *span,
                })
            }
            Ast::Literal { value, span } => {
                Some(Ast::Literal { value: value.clone(), span: *span })
            }
            Ast::StructCons { name, fields, span, .. } => {
                let tf: Vec<(String, Ast)> = fields
                    .iter()
                    .filter_map(|(n, v)| self.infer_node(v).map(|ast| (n.clone(), ast)))
                    .collect();
                let ft: Vec<(String, Type)> = tf
                    .iter()
                    .map(|(n, v)| (n.clone(), self.get_type(v).unwrap_or_else(fresh_var)))
                    .collect();
                Some(Ast::StructCons {
                    name: name.clone(),
                    fields: tf,
                    llvm_type: Some(Type::Record(ft)),
                    span: *span,
                })
            }
            Ast::SumCons { name, value, span, .. } => {
                let typed_val = value.as_ref().and_then(|v| self.infer_node(v));
                let inner_type = typed_val.as_ref().and_then(|v| self.get_type(v));
                let sum_type = Type::Sum(vec![(name.clone(), inner_type.clone())]);
                Some(Ast::SumCons {
                    name: name.clone(),
                    value: typed_val.map(Box::new),
                    llvm_type: Some(sum_type),
                    span: *span,
                })
            }
            _ => Some(node.clone()),
        }
    }

    fn get_type(&self, node: &Ast) -> Option<Type> {
        match node {
            Ast::BinExpr { llvm_type, .. }
            | Ast::UnaryExpr { llvm_type, .. }
            | Ast::Call { llvm_type, .. }
            | Ast::If { llvm_type, .. }
            | Ast::Variable { llvm_type, .. }
            | Ast::StructCons { llvm_type, .. }
            | Ast::SumCons { llvm_type, .. }
            | Ast::FieldAccess { llvm_type, .. }
            | Ast::StructUpdate { llvm_type, .. }
            | Ast::Match { llvm_type, .. } => llvm_type.clone(),
            Ast::Literal { value, .. } => Some(self.literal_type(value)),
            _ => None,
        }
    }

    fn type_mismatch(&self, expected: &Type, found: &Type) -> Diagnostic {
        Diagnostic {
            kind: DiagnosticKind::Error,
            message: format!(
                "Type mismatch: expected '{}', found '{}'",
                self.type_to_string(expected),
                self.type_to_string(found)
            ),
            span: Span { line: 0, column: 0, offset: 0 },
            hint: None,
        }
    }

    fn type_to_string(&self, typ: &Type) -> String {
        match typ {
            Type::Primitive(PrimitiveType::Int) => "Int".into(),
            Type::Primitive(PrimitiveType::Float) => "Float".into(),
            Type::Primitive(PrimitiveType::Bool) => "Bool".into(),
            Type::Primitive(PrimitiveType::String) => "String".into(),
            Type::Variable(v) => v.clone(),
            Type::Fn { arguments, result } => format!(
                "({}) -> {}",
                arguments
                    .iter()
                    .map(|a| self.type_to_string(a))
                    .collect::<Vec<_>>()
                    .join(", "),
                self.type_to_string(result)
            ),
            Type::Record(fields) => format!(
                "{{ {} }}",
                fields
                    .iter()
                    .map(|(n, t)| format!("{}: {}", n, self.type_to_string(t)))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Type::Sum(variants) => format!(
                "enum {{ {} }}",
                variants
                    .iter()
                    .map(|(n, t)| match t {
                        Some(tt) => format!("{}({})", n, self.type_to_string(tt)),
                        None => n.clone(),
                    })
                    .collect::<Vec<_>>()
                    .join(" | ")
            ),
            Type::Void => "Void".into(),
            Type::Tuple(types) => format!(
                "({})",
                types
                    .iter()
                    .map(|t| self.type_to_string(t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            _ => format!("{:?}", typ),
        }
    }
}

pub fn substitute_type_vars(typ: &Type, map: &HashMap<String, Type>) -> Type {
    match typ {
        Type::Variable(name) => map.get(name).cloned().unwrap_or(typ.clone()),
        Type::Fn { arguments, result } => Type::Fn {
            arguments: arguments
                .iter()
                .map(|a| substitute_type_vars(a, map))
                .collect(),
            result: Box::new(substitute_type_vars(result, map)),
        },
        Type::Record(fields) => Type::Record(
            fields
                .iter()
                .map(|(n, t)| (n.clone(), substitute_type_vars(t, map)))
                .collect(),
        ),
        _ => typ.clone(),
    }
}

pub fn apply_substitution_to_ast(ast: &mut Ast, sub: &Substitution) {
    match ast {
        Ast::Module { declarations, .. } => {
            for d in declarations {
                apply_substitution_to_ast(d, sub);
            }
        }
        Ast::FnDecl { params, return_type, body, .. } => {
            for p in params {
                p.llvm_type = sub.apply(&p.llvm_type);
            }
            if let Some(ref mut r) = return_type {
                *r = sub.apply(r);
            }
            apply_substitution_to_ast(body, sub);
        }
        Ast::Block { expressions, .. } => {
            for e in expressions {
                apply_substitution_to_ast(e, sub);
            }
        }
        Ast::Let { type_annotation, value, .. }
        | Ast::Assign { type_annotation, value, .. } => {
            if let Some(ref mut t) = type_annotation {
                *t = sub.apply(t);
            }
            apply_substitution_to_ast(value, sub);
        }
        Ast::OpAssign { value, .. } => {
            apply_substitution_to_ast(value, sub);
        }
        Ast::PatternAssign { value, .. } => {
            apply_substitution_to_ast(value, sub);
        }
        Ast::StructUpdate { object, fields, llvm_type, .. } => {
            apply_substitution_to_ast(object, sub);
            for (_, v) in fields {
                apply_substitution_to_ast(v, sub);
            }
            if let Some(ref mut t) = llvm_type {
                *t = sub.apply(t);
            }
        }
        Ast::Match { value, arms, llvm_type, .. } => {
            apply_substitution_to_ast(value, sub);
            for arm in arms {
                if let Some(ref mut g) = arm.condition {
                    apply_substitution_to_ast(g, sub);
                }
                apply_substitution_to_ast(&mut arm.body, sub);
            }
            if let Some(ref mut t) = llvm_type {
                *t = sub.apply(t);
            }
        }
        Ast::BinExpr { left, right, llvm_type, .. } => {
            if let Some(ref mut t) = llvm_type {
                *t = sub.apply(t);
            }
            apply_substitution_to_ast(left, sub);
            apply_substitution_to_ast(right, sub);
        }
        Ast::Call { function, arguments, llvm_type, .. } => {
            if let Some(ref mut t) = llvm_type {
                *t = sub.apply(t);
            }
            apply_substitution_to_ast(function, sub);
            for a in arguments {
                apply_substitution_to_ast(a, sub);
            }
        }
        Ast::If { condition, then, else_arm, llvm_type, .. } => {
            if let Some(ref mut t) = llvm_type {
                *t = sub.apply(t);
            }
            apply_substitution_to_ast(condition, sub);
            apply_substitution_to_ast(then, sub);
            if let Some(ref mut e) = else_arm {
                apply_substitution_to_ast(e, sub);
            }
        }
        Ast::While { condition, body, .. } => {
            apply_substitution_to_ast(condition, sub);
            apply_substitution_to_ast(body, sub);
        }
        Ast::LoopWhile { condition, body, .. } => {
            apply_substitution_to_ast(condition, sub);
            apply_substitution_to_ast(body, sub);
        }
        Ast::Variable { llvm_type, .. } => {
            if let Some(ref mut t) = llvm_type {
                *t = sub.apply(t);
            }
        }
        Ast::StructCons { fields, llvm_type, .. } => {
            if let Some(ref mut t) = llvm_type {
                *t = sub.apply(t);
            }
            for (_, v) in fields {
                apply_substitution_to_ast(v, sub);
            }
        }
        Ast::SumCons { llvm_type, value, .. } => {
            if let Some(ref mut t) = llvm_type {
                *t = sub.apply(t);
            }
            if let Some(ref mut v) = value {
                apply_substitution_to_ast(v, sub);
            }
        }
        Ast::FieldAccess { object, llvm_type, .. } => {
            if let Some(ref mut t) = llvm_type {
                *t = sub.apply(t);
            }
            apply_substitution_to_ast(object, sub);
        }
        _ => {}
    }
}
