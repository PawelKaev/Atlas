// grammalang-core/src/types.rs
// Version 1.4 — support for new types (Array, Slice, Range, Tuple, Pointer)

use crate::ast::Type;
use crate::token::Span;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

// ============ Fresh variable generators ============

static NEXT_VAR_ID: AtomicUsize = AtomicUsize::new(0);

pub fn fresh_var() -> Type {
    let id = NEXT_VAR_ID.fetch_add(1, Ordering::SeqCst);
    Type::Variable(format!("α{}", id))
}

pub fn fresh_effect_var() -> Type {
    let id = NEXT_VAR_ID.fetch_add(1, Ordering::SeqCst);
    Type::Variable(format!("ε{}", id))
}

pub fn reset_fresh_vars() {
    NEXT_VAR_ID.store(0, Ordering::SeqCst);
}

// ============ Substitution ============

#[derive(Debug, Clone, Default)]
pub struct Substitution {
    map: HashMap<String, Type>,
}

impl Substitution {
    pub fn new() -> Self {
        Substitution {
            map: HashMap::new(),
        }
    }

    pub fn singleton(var: &str, typ: Type) -> Self {
        let mut map = HashMap::new();
        map.insert(var.to_string(), typ);
        Substitution { map }
    }

    pub fn insert(&mut self, var: String, typ: Type) {
        self.map.insert(var, typ);
    }

    pub fn get(&self, var: &str) -> Option<&Type> {
        self.map.get(var)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    // ========== apply (immutable, no compression) ==========

    pub fn apply(&self, typ: &Type) -> Type {
        match typ {
            Type::Variable(name) => match self.map.get(name) {
                Some(resolved) => self.apply(resolved),
                None => typ.clone(),
            },
            Type::Parameterized { name, params } => {
                let params = params.iter().map(|p| self.apply(p)).collect();
                Type::Parameterized {
                    name: name.clone(),
                    params,
                }
            }
            Type::Fn { arguments, result } => {
                let args = arguments.iter().map(|a| self.apply(a)).collect();
                let ret = self.apply(result);
                Type::Fn {
                    arguments: args,
                    result: Box::new(ret),
                }
            }
            Type::Record(fields) => {
                let fields = fields
                    .iter()
                    .map(|(n, t)| (n.clone(), self.apply(t)))
                    .collect();
                Type::Record(fields)
            }
            Type::Sum(variants) => {
                let vars = variants
                    .iter()
                    .map(|(n, t)| (n.clone(), t.as_ref().map(|typ| self.apply(typ))))
                    .collect();
                Type::Sum(vars)
            }
            Type::Ref { mutable, llvm_type } => Type::Ref {
                mutable: *mutable,
                llvm_type: Box::new(self.apply(llvm_type)),
            },
            Type::Effect { effect, llvm_type } => Type::Effect {
                effect: effect.clone(),
                llvm_type: Box::new(self.apply(llvm_type)),
            },
            Type::Refined { base, condition } => Type::Refined {
                base: Box::new(self.apply(base)),
                condition: condition.clone(),
            },
            Type::Array { llvm_type, size } => Type::Array {
                llvm_type: Box::new(self.apply(llvm_type)),
                size: *size,
            },
            Type::Slice { llvm_type } => Type::Slice {
                llvm_type: Box::new(self.apply(llvm_type)),
            },
            Type::Tuple(types) => {
                let types: Vec<Type> = types.iter().map(|t| self.apply(t)).collect();
                Type::Tuple(types)
            },
            Type::Pointer { mutable, llvm_type } => Type::Pointer {
                mutable: *mutable,
                llvm_type: Box::new(self.apply(llvm_type)),
            },
            Type::Range => Type::Range,
            Type::Primitive(_) | Type::Unit | Type::Void => typ.clone(),
        }
    }

    // ========== apply_mut (with Path Compression) ==========

    pub fn apply_mut(&mut self, typ: &Type) -> Type {
        match typ {
            Type::Variable(name) => {
                let mut current: &str = name.as_str();
                loop {
                    match self.map.get(current) {
                        Some(Type::Variable(next)) if next.as_str() != current => {
                            current = next.as_str();
                            continue;
                        }
                        Some(resolved) => return resolved.clone(),
                        None => {
                            if current == name.as_str() {
                                return typ.clone();
                            } else {
                                return Type::Variable(current.to_string());
                            }
                        }
                    }
                }
            }
            Type::Parameterized { name, params } => {
                let params: Vec<Type> = params.iter().map(|p| self.apply_mut(p)).collect();
                Type::Parameterized { name: name.clone(), params }
            }
            Type::Fn { arguments, result } => {
                let args: Vec<Type> = arguments.iter().map(|a| self.apply_mut(a)).collect();
                let ret = self.apply_mut(result);
                Type::Fn { arguments: args, result: Box::new(ret) }
            }
            Type::Record(fields) => {
                let fields: Vec<(String, Type)> = fields
                    .iter()
                    .map(|(n, t)| (n.clone(), self.apply_mut(t)))
                    .collect();
                Type::Record(fields)
            }
            Type::Sum(variants) => {
                let vars: Vec<(String, Option<Type>)> = variants
                    .iter()
                    .map(|(n, t)| (n.clone(), t.as_ref().map(|typ| self.apply_mut(typ))))
                    .collect();
                Type::Sum(vars)
            }
            Type::Ref { mutable, llvm_type } => Type::Ref {
                mutable: *mutable,
                llvm_type: Box::new(self.apply_mut(llvm_type)),
            },
            Type::Effect { effect, llvm_type } => Type::Effect {
                effect: effect.clone(),
                llvm_type: Box::new(self.apply_mut(llvm_type)),
            },
            Type::Refined { base, condition } => Type::Refined {
                base: Box::new(self.apply_mut(base)),
                condition: condition.clone(),
            },
            Type::Array { llvm_type, size } => Type::Array {
                llvm_type: Box::new(self.apply_mut(llvm_type)),
                size: *size,
            },
            Type::Slice { llvm_type } => Type::Slice {
                llvm_type: Box::new(self.apply_mut(llvm_type)),
            },
            Type::Tuple(types) => {
                let types: Vec<Type> = types.iter().map(|t| self.apply_mut(t)).collect();
                Type::Tuple(types)
            },
            Type::Pointer { mutable, llvm_type } => Type::Pointer {
                mutable: *mutable,
                llvm_type: Box::new(self.apply_mut(llvm_type)),
            },
            Type::Range => Type::Range,
            _ => typ.clone(),
        }
    }

    pub fn compress_all(&mut self) {
        let keys: Vec<String> = self.map.keys().cloned().collect();
        for key in keys {
            let resolved = self.apply_mut(&Type::Variable(key.clone()));
            self.map.insert(key, resolved);
        }
    }

    // ========== compose ==========

    pub fn compose(self, other: Substitution) -> Substitution {
        let mut result = other.map;
        for (var, typ) in self.map {
            let applied = Self::apply_to_map(&result, &typ);
            result.insert(var, applied);
        }
        Substitution { map: result }
    }

    fn apply_to_map(map: &HashMap<String, Type>, typ: &Type) -> Type {
        match typ {
            Type::Variable(name) => match map.get(name) {
                Some(resolved) => Self::apply_to_map(map, resolved),
                None => typ.clone(),
            },
            Type::Parameterized { name, params } => {
                let params = params.iter().map(|p| Self::apply_to_map(map, p)).collect();
                Type::Parameterized { name: name.clone(), params }
            }
            Type::Fn { arguments, result } => {
                let args = arguments.iter().map(|a| Self::apply_to_map(map, a)).collect();
                let ret = Self::apply_to_map(map, result);
                Type::Fn { arguments: args, result: Box::new(ret) }
            }
            Type::Record(fields) => {
                let fields = fields.iter().map(|(n, t)| (n.clone(), Self::apply_to_map(map, t))).collect();
                Type::Record(fields)
            }
            Type::Sum(variants) => {
                let vars = variants.iter().map(|(n, t)| (n.clone(), t.as_ref().map(|typ| Self::apply_to_map(map, typ)))).collect();
                Type::Sum(vars)
            }
            Type::Ref { mutable, llvm_type } => Type::Ref {
                mutable: *mutable,
                llvm_type: Box::new(Self::apply_to_map(map, llvm_type)),
            },
            Type::Effect { effect, llvm_type } => Type::Effect {
                effect: effect.clone(),
                llvm_type: Box::new(Self::apply_to_map(map, llvm_type)),
            },
            Type::Refined { base, condition } => Type::Refined {
                base: Box::new(Self::apply_to_map(map, base)),
                condition: condition.clone(),
            },
            Type::Array { llvm_type, size } => Type::Array {
                llvm_type: Box::new(Self::apply_to_map(map, llvm_type)),
                size: *size,
            },
            Type::Slice { llvm_type } => Type::Slice {
                llvm_type: Box::new(Self::apply_to_map(map, llvm_type)),
            },
            Type::Tuple(types) => {
                let types: Vec<Type> = types.iter().map(|t| Self::apply_to_map(map, t)).collect();
                Type::Tuple(types)
            },
            Type::Pointer { mutable, llvm_type } => Type::Pointer {
                mutable: *mutable,
                llvm_type: Box::new(Self::apply_to_map(map, llvm_type)),
            },
            Type::Range => Type::Range,
            _ => typ.clone(),
        }
    }

    // ========== merge (with compatibility check) ==========

    pub fn merge(&mut self, other: &Substitution) -> Result<(), (String, Type, Type)> {
        for (var, typ) in &other.map {
            let applied = self.apply(typ);
            if let Some(existing) = self.map.get(var) {
                let existing_applied = self.apply(existing);
                if existing_applied != applied {
                    return Err((var.clone(), existing_applied, applied));
                }
            } else {
                self.map.insert(var.clone(), applied);
            }
        }
        Ok(())
    }
}

// ============ Constraints ============

#[derive(Debug, Clone)]
pub enum Constraint {
    Equality(Type, Type, Span),
    Subtype(Type, Type, Span),
    Concept(Type, String, Span),
}

impl Constraint {
    pub fn equality(t1: Type, t2: Type, span: Span) -> Self {
        Constraint::Equality(t1, t2, span)
    }

    pub fn concept(typ: Type, concept: &str, span: Span) -> Self {
        Constraint::Concept(typ, concept.to_string(), span)
    }

    pub fn is_equality(&self) -> bool {
        matches!(self, Constraint::Equality(_, _, _))
    }
}

// ============ Occurs check ============

pub fn occurs_in(var: &str, typ: &Type) -> bool {
    match typ {
        Type::Variable(v) => v == var,
        Type::Parameterized { params, .. } => params.iter().any(|p| occurs_in(var, p)),
        Type::Fn { arguments, result } => {
            arguments.iter().any(|a| occurs_in(var, a)) || occurs_in(var, result)
        }
        Type::Record(fields) => fields.iter().any(|(_, t)| occurs_in(var, t)),
        Type::Sum(variants) => variants.iter().any(|(_, t)| {
            t.as_ref().map_or(false, |typ| occurs_in(var, typ))
        }),
        Type::Ref { llvm_type, .. } => occurs_in(var, llvm_type),
        Type::Effect { llvm_type, .. } => occurs_in(var, llvm_type),
        Type::Refined { base, .. } => occurs_in(var, base),
        Type::Array { llvm_type, .. } => occurs_in(var, llvm_type),
        Type::Slice { llvm_type } => occurs_in(var, llvm_type),
        Type::Tuple(types) => types.iter().any(|t| occurs_in(var, t)),
        Type::Pointer { llvm_type, .. } => occurs_in(var, llvm_type),
        Type::Range => false,
        Type::Primitive(_) | Type::Unit | Type::Void => false,
    }
}

// ============ Helper functions for new types ============

pub fn array_type(element_type: Type, size: usize) -> Type {
    Type::Array {
        llvm_type: Box::new(element_type),
        size: Some(size),
    }
}

pub fn slice_type(element_type: Type) -> Type {
    Type::Slice {
        llvm_type: Box::new(element_type),
    }
}

pub fn tuple_type(types: Vec<Type>) -> Type {
    Type::Tuple(types)
}

pub fn pointer_type(mutable: bool, target: Type) -> Type {
    Type::Pointer {
        mutable,
        llvm_type: Box::new(target),
    }
}

// ============ Tests ============

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::PrimitiveType;

    fn dummy_span() -> Span {
        Span { line: 1, column: 1, offset: 0 }
    }

    #[test]
    fn test_apply_simple() {
        let sub = Substitution::singleton("α0", Type::Primitive(PrimitiveType::Int));
        assert_eq!(
            sub.apply(&Type::Variable("α0".to_string())),
            Type::Primitive(PrimitiveType::Int)
        );
    }

    #[test]
    fn test_apply_chain() {
        let mut sub = Substitution::new();
        sub.insert("α1".to_string(), Type::Primitive(PrimitiveType::Int));
        sub.insert("α0".to_string(), Type::Variable("α1".to_string()));
        assert_eq!(
            sub.apply(&Type::Variable("α0".to_string())),
            Type::Primitive(PrimitiveType::Int)
        );
    }

    #[test]
    fn test_apply_mut_compression() {
        let mut sub = Substitution::new();
        sub.insert("α2".to_string(), Type::Primitive(PrimitiveType::Int));
        sub.insert("α1".to_string(), Type::Variable("α2".to_string()));
        sub.insert("α0".to_string(), Type::Variable("α1".to_string()));
        let result = sub.apply_mut(&Type::Variable("α0".to_string()));
        assert_eq!(result, Type::Primitive(PrimitiveType::Int));
    }

    #[test]
    fn test_merge_conflict() {
        let mut sub = Substitution::singleton("α0", Type::Primitive(PrimitiveType::Int));
        let other = Substitution::singleton("α0", Type::Primitive(PrimitiveType::String));
        let result = sub.merge(&other);
        assert!(result.is_err());
    }

    #[test]
    fn test_occurs_in_self_ref() {
        let self_ref = Type::Parameterized {
            name: "List".to_string(),
            params: vec![Type::Variable("α0".to_string())],
        };
        assert!(occurs_in("α0", &self_ref));
    }

    #[test]
    fn test_fresh_var_uniqueness() {
        reset_fresh_vars();
        let v1 = fresh_var();
        let v2 = fresh_var();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_array_type() {
        let arr = array_type(Type::Primitive(PrimitiveType::Int), 5);
        assert_eq!(
            arr,
            Type::Array {
                llvm_type: Box::new(Type::Primitive(PrimitiveType::Int)),
                size: Some(5),
            }
        );
    }

    #[test]
    fn test_slice_type() {
        let slice = slice_type(Type::Primitive(PrimitiveType::String));
        assert_eq!(
            slice,
            Type::Slice {
                llvm_type: Box::new(Type::Primitive(PrimitiveType::String)),
            }
        );
    }

    #[test]
    fn test_apply_array() {
        let sub = Substitution::singleton("α0", Type::Primitive(PrimitiveType::Int));
        let arr = Type::Array {
            llvm_type: Box::new(Type::Variable("α0".to_string())),
            size: Some(3),
        };
        let result = sub.apply(&arr);
        assert_eq!(
            result,
            Type::Array {
                llvm_type: Box::new(Type::Primitive(PrimitiveType::Int)),
                size: Some(3),
            }
        );
    }

    #[test]
    fn test_occurs_in_array() {
        let arr = Type::Array {
            llvm_type: Box::new(Type::Variable("α0".to_string())),
            size: None,
        };
        assert!(occurs_in("α0", &arr));
    }

    #[test]
    fn test_occurs_in_tuple() {
        let tuple = Type::Tuple(vec![
            Type::Primitive(PrimitiveType::Int),
            Type::Variable("α0".to_string()),
        ]);
        assert!(occurs_in("α0", &tuple));
    }
}
