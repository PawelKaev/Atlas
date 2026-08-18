// grammalang-core/src/ontology/engine.rs
#![allow(dead_code, unused_variables, unused_imports)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Entity category in the ontology
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Concept,
    Relation,
    Attribute,
    Instance,
    Axiom,
}

/// Ontology entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub category: Category,
    pub properties: Vec<Property>,
}

/// Entity property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub key: String,
    pub value: PropertyValue,
}

/// Property value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Timestamp(String),
    Reference(String),  // reference to another entity
}

/// Ontology engine — encapsulated draft specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyEngine {
    entities: BTreeMap<String, Entity>,
}

impl OntologyEngine {
    pub fn new() -> Self {
        OntologyEngine { entities: BTreeMap::new() }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.name.clone(), entity);
    }

    pub fn find_entity(&self, name: &str) -> Option<&Entity> {
        self.entities.get(name)
    }

    pub fn entities_by_category(&self, category: &Category) -> Vec<&Entity> {
        self.entities.values().filter(|e| &e.category == category).collect()
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}
