// grammalang-core/src/types.rs

use crate::ast::Тип;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Генератор свежих переменных типа
static NEXT_VAR_ID: AtomicUsize = AtomicUsize::new(0);

pub fn fresh_var() -> Тип {
    let id = NEXT_VAR_ID.fetch_add(1, Ordering::SeqCst);
    Тип::Переменная(format!("α{}", id))
}

pub fn fresh_effect_var() -> Тип {
    let id = NEXT_VAR_ID.fetch_add(1, Ordering::SeqCst);
    Тип::Переменная(format!("ε{}", id))
}

/// Подстановка типов
#[derive(Debug, Clone, Default)]
pub struct Substitution {
    map: HashMap<String, Тип>,
}

impl Substitution {
    pub fn new() -> Self {
        Substitution { map: HashMap::new() }
    }

    pub fn singleton(var: &str, typ: Тип) -> Self {
        let mut map = HashMap::new();
        map.insert(var.to_string(), typ);
        Substitution { map }
    }

    pub fn apply(&self, typ: &Тип) -> Тип {
        match typ {
            Тип::Переменная(name) => {
                self.map.get(name).cloned().unwrap_or_else(|| typ.clone())
            }
            Тип::Параметризованный { имя, параметры } => {
                let params = параметры.iter().map(|p| self.apply(p)).collect();
                Тип::Параметризованный {
                    имя: имя.clone(),
                    параметры: params,
                }
            }
            Тип::Функция { аргументы, результат } => {
                let args = аргументы.iter().map(|a| self.apply(a)).collect();
                let ret = self.apply(результат);
                Тип::Функция {
                    аргументы: args,
                    результат: Box::new(ret),
                }
            }
            Тип::Запись(поля) => {
                let fields = поля.iter().map(|(n, t)| (n.clone(), self.apply(t))).collect();
                Тип::Запись(fields)
            }
            Тип::Сумма(варианты) => {
                let vars = варианты.iter().map(|(n, t)| {
                    (n.clone(), t.as_ref().map(|typ| self.apply(typ)))
                }).collect();
                Тип::Сумма(vars)
            }
            Тип::Ссылка { изменяемая, тип } => {
                Тип::Ссылка {
                    изменяемая: *изменяемая,
                    тип: Box::new(self.apply(тип)),
                }
            }
            Тип::Эффект { эффект, тип } => {
                Тип::Эффект {
                    эффект: эффект.clone(),
                    тип: Box::new(self.apply(тип)),
                }
            }
            _ => typ.clone(),
        }
    }

    pub fn compose(&self, other: &Substitution) -> Substitution {
        let mut result = other.map.clone();
        for (var, typ) in &self.map {
            result.insert(var.clone(), other.apply(typ));
        }
        Substitution { map: result }
    }
}

/// Ограничение типов
#[derive(Debug, Clone)]
pub enum Constraint {
    Равенство(Тип, Тип),
    Подтип(Тип, Тип),
    Концепт(Тип, String),
}
