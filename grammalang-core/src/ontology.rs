// grammalang-core/src/ontology.rs

use serde::{Deserialize, Serialize};
use crate::error::Diagnostic;

/// Онтологическая категория
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Сущность,
    Свойство,
    Отношение,
    Событие,
    Качество,
    Количество,
}

/// Онтологическая сущность
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub category: Category,
    pub properties: Vec<Property>,
}

/// Свойство сущности
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
}

/// Значение свойства
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Reference(String),
}

/// Онтологический движок
pub struct OntologyEngine {
    entities: Vec<Entity>,
}

impl OntologyEngine {
    pub fn new() -> Self {
        OntologyEngine {
            entities: Vec::new(),
        }
    }

    /// Добавить сущность
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    /// Найти сущность по имени
    pub fn find_entity(&self, name: &str) -> Option<&Entity> {
        self.entities.iter().find(|e| e.name == name)
    }

    /// Получить все сущности заданной категории
    pub fn entities_by_category(&self, category: &Category) -> Vec<&Entity> {
        self.entities.iter().filter(|e| &e.category == category).collect()
    }

    /// Количество сущностей
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}
