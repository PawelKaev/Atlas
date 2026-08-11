// grammalang-core/src/entity.rs

use std::collections::HashMap;

/// Unique identifier of an ontological entity in the state pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OntoEntityId(pub usize);

/// State of a subject in terms of Lefebvre mathematics.
///
/// - `x`: utility of the alternative (desire)
/// - `y`: intention (readiness to choose)
/// - `z`: image of world pressure (perceived pressure)
/// - `version`: change counter (for the cardiogram)
#[derive(Debug, Clone, Copy)]
pub struct OntoState {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub version: u64,
}

impl OntoState {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, version: 0 }
    }

    pub fn as_tuple(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn update(&mut self, state: (f64, f64, f64)) {
        self.x = state.0;
        self.y = state.1;
        self.z = state.2;
        self.version += 1;
    }
}

/// Unique identifier for a voice in the polyphonic field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoiceId(pub usize);

/// Hierarchical ontological space — a tree of named subspaces.
///
/// Each space has:
/// - `entities`: entities directly defined in this space
/// - `subspaces`: child spaces (e.g., "участок" inside "Соня")
/// - `states`: shared state pool for this space
///
/// Lookup traverses upward: if an entity is not found in the current space,
/// it searches the parent space (inheritance).
#[derive(Debug, Clone)]
pub struct OntoSpace {
    pub name: String,
    pub entities: HashMap<String, OntoEntityId>,
    pub subspaces: HashMap<String, OntoSpace>,
    pub states: Vec<OntoState>,
    parent: Option<Box<OntoSpace>>,
}

impl OntoSpace {
    pub fn new(name: &str) -> Self {
        OntoSpace {
            name: name.to_string(),
            entities: HashMap::new(),
            subspaces: HashMap::new(),
            states: Vec::new(),
            parent: None,
        }
    }

    pub fn with_parent(name: &str, parent: OntoSpace) -> Self {
        OntoSpace {
            name: name.to_string(),
            entities: HashMap::new(),
            subspaces: HashMap::new(),
            states: Vec::new(),
            parent: Some(Box::new(parent)),
        }
    }

    /// Resolves an entity by traversing the hierarchy upward.
    /// First checks current space, then subspaces, then parent.
    pub fn resolve(&self, path: &str) -> Option<(f64, f64, f64)> {
        let parts: Vec<&str> = path.split('.').collect();
        self.resolve_parts(&parts)
    }

    fn resolve_parts(&self, parts: &[&str]) -> Option<(f64, f64, f64)> {
        if parts.is_empty() {
            return None;
        }

        let head = parts[0];
        let tail = &parts[1..];

        if tail.is_empty() {
            // Last segment — look for entity in this space or parent
            if let Some(id) = self.entities.get(head) {
                return Some(self.states[id.0].as_tuple());
            }
            // Try parent
            if let Some(ref parent) = self.parent {
                return parent.resolve_parts(parts);
            }
            return None;
        }

        // Has more segments — look for subspace
        if let Some(subspace) = self.subspaces.get(head) {
            return subspace.resolve_parts(tail);
        }

        // Try parent
        if let Some(ref parent) = self.parent {
            return parent.resolve_parts(parts);
        }

        None
    }

    /// Resolves an entity ID, creating it in the deepest subspace if not found.
    pub fn resolve_or_create(
        &mut self,
        path: &str,
        default_state: (f64, f64, f64),
    ) -> OntoEntityId {
        let parts: Vec<&str> = path.split('.').collect();
        self.resolve_or_create_parts(&parts, default_state)
    }

    fn resolve_or_create_parts(
        &mut self,
        parts: &[&str],
        default_state: (f64, f64, f64),
    ) -> OntoEntityId {
        if parts.is_empty() {
            panic!("Empty path in resolve_or_create");
        }

        let head = parts[0];
        let tail = &parts[1..];

        if tail.is_empty() {
            // Last segment — get or create entity in this space
            if let Some(id) = self.entities.get(head) {
                return *id;
            }
            let id = OntoEntityId(self.states.len());
            self.entities.insert(head.to_string(), id);
            self.states.push(OntoState::new(
                default_state.0,
                default_state.1,
                default_state.2,
            ));
            return id;
        }

        // Has more segments — get or create subspace
        let subspace = self
            .subspaces
            .entry(head.to_string())
            .or_insert_with(|| OntoSpace::new(head));

        subspace.resolve_or_create_parts(tail, default_state)
    }

    /// Gets the version of an entity by path.
    pub fn get_version(&self, path: &str) -> Option<u64> {
        let parts: Vec<&str> = path.split('.').collect();
        self.get_version_parts(&parts)
    }

    fn get_version_parts(&self, parts: &[&str]) -> Option<u64> {
        if parts.is_empty() {
            return None;
        }

        let head = parts[0];
        let tail = &parts[1..];

        if tail.is_empty() {
            if let Some(id) = self.entities.get(head) {
                return Some(self.states[id.0].version);
            }
            if let Some(ref parent) = self.parent {
                return parent.get_version_parts(parts);
            }
            return None;
        }

        if let Some(subspace) = self.subspaces.get(head) {
            return subspace.get_version_parts(tail);
        }

        if let Some(ref parent) = self.parent {
            return parent.get_version_parts(parts);
        }

        None
    }

    /// Updates the state of an entity by path.
    pub fn update_state(&mut self, path: &str, state: (f64, f64, f64)) {
        let parts: Vec<&str> = path.split('.').collect();
        self.update_state_parts(&parts, state);
    }

    fn update_state_parts(&mut self, parts: &[&str], state: (f64, f64, f64)) {
        if parts.len() <= 1 {
            let head = parts[0];
            if let Some(id) = self.entities.get(head) {
                self.states[id.0].update(state);
            }
            return;
        }

        let head = parts[0];
        let tail = &parts[1..];

        if let Some(subspace) = self.subspaces.get_mut(head) {
            subspace.update_state_parts(tail, state);
        }
    }
}

// ============ Operational Theory of the Ideal (Pivovarov) ============

/// Unique identifier for an operation schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaId(pub usize);

/// Unique identifier for an act node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActId(pub usize);

/// Unique identifier for a symbolic node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

/// A step within an operation schema.
#[derive(Debug, Clone)]
pub struct SchemaStep {
    pub action: String,
    pub role: SchemaRole,
    pub precondition: Option<String>,
    pub postcondition: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaRole {
    Subject,
    Object,
    Tool,
    Mediator,
}

/// An operational schema of activity — the ideal as an executable invariant.
#[derive(Debug, Clone)]
pub struct OperationSchema {
    pub id: SchemaId,
    pub name: String,
    pub steps: Vec<SchemaStep>,
    pub invariant: String,
    pub symbol: Option<String>,
    pub owner: VoiceId,
}

/// A concrete act, produced by applying <<execute>> to a schema.
#[derive(Debug, Clone)]
pub struct ActNode {
    pub id: ActId,
    pub schema: SchemaId,
    pub subject: VoiceId,
    pub object: String,
    pub tool: String,
    pub result: ActResult,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum ActResult {
    Success,
    Failure(String),
    Contradiction(SchemaId),
}

/// A sign in which a schema of activity is encoded.
#[derive(Debug, Clone)]
pub struct SymbolicNode {
    pub id: SymbolId,
    pub schema: SchemaId,
    pub form: String,
    pub encoded_by: VoiceId,
}
