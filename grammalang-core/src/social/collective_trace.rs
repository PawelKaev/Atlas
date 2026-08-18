use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Коллективный trace - объединенная генеалогия от множества машин
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveTrace {
    pub participants: Vec<Participant>,
    pub events: Vec<TraceEvent>,
    pub genealogy: HashMap<String, Vec<String>>,
    pub protocol_version: String,
    pub merge_history: Vec<MergeRecord>,
    pub sync_state: SyncState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub machine_id: String,
    pub name: String,
    pub role: ParticipantRole,
    pub contribution: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantRole {
    Leader,
    Member,
    Observer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEvent {
    pub timestamp: u64,
    pub source: String,
    pub event_type: TraceEventType,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceEventType {
    NodeAdded,
    Synthesis,
    Contradiction,
    Merge,
    Sync,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRecord {
    pub timestamp: u64,
    pub from_participant: String,
    pub to_participant: String,
    pub nodes_merged: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    pub last_sync: u64,
    pub pending_changes: usize,
    pub is_syncing: bool,
}

impl CollectiveTrace {
    pub fn new() -> Self {
        Self {
            participants: Vec::new(),
            events: Vec::new(),
            genealogy: HashMap::new(),
            protocol_version: "0.8.0".to_string(),
            merge_history: Vec::new(),
            sync_state: SyncState {
                last_sync: 0,
                pending_changes: 0,
                is_syncing: false,
            },
        }
    }
    
    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }
    
    pub fn record_event(&mut self, source: &str, event_type: TraceEventType, description: &str) {
        self.events.push(TraceEvent {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            source: source.to_string(),
            event_type,
            description: description.to_string(),
        });
    }
    
    pub fn add_genealogy(&mut self, node: &str, ancestors: Vec<String>) {
        self.genealogy.insert(node.to_string(), ancestors);
    }
    
    pub fn get_history(&self, node: &str) -> Option<&Vec<String>> {
        self.genealogy.get(node)
    }
    
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
    
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }
    
    /// Слияние trace от другого участника
    pub fn merge(&mut self, other: &CollectiveTrace) -> Result<usize, String> {
        let mut merged_count = 0;
        
        // Проверка совместимости протокола
        if self.protocol_version != other.protocol_version {
            return Err(format!(
                "Protocol mismatch: {} vs {}",
                self.protocol_version, other.protocol_version
            ));
        }
        
        // Слияние участников
        for participant in &other.participants {
            if !self.participants.iter().any(|p| p.machine_id == participant.machine_id) {
                self.participants.push(participant.clone());
            }
        }
        
        // Слияние событий
        for event in &other.events {
            self.events.push(event.clone());
            merged_count += 1;
        }
        
        // Слияние генеалогии
        for (node, ancestors) in &other.genealogy {
            if !self.genealogy.contains_key(node) {
                self.genealogy.insert(node.clone(), ancestors.clone());
                merged_count += 1;
            }
        }
        
        // Запись слияния
        self.merge_history.push(MergeRecord {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            from_participant: other.participants.first()
                .map(|p| p.machine_id.clone())
                .unwrap_or_default(),
            to_participant: "self".to_string(),
            nodes_merged: other.genealogy.keys().cloned().collect(),
        });
        
        Ok(merged_count)
    }
    
    /// Начало синхронизации
    pub fn start_sync(&mut self) {
        self.sync_state.is_syncing = true;
        self.sync_state.pending_changes = 0;
        self.record_event("system", TraceEventType::Sync, "Sync started");
    }
    
    /// Завершение синхронизации
    pub fn finish_sync(&mut self) {
        self.sync_state.is_syncing = false;
        self.sync_state.last_sync = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.record_event("system", TraceEventType::Sync, "Sync completed");
    }
    
    /// Получение статистики
    pub fn stats(&self) -> TraceStats {
        TraceStats {
            participants: self.participants.len(),
            events: self.events.len(),
            genealogy_entries: self.genealogy.len(),
            merges: self.merge_history.len(),
            pending_changes: self.sync_state.pending_changes,
        }
    }
    
    /// Поиск событий по типу
    pub fn find_events(&self, event_type: &TraceEventType) -> Vec<&TraceEvent> {
        self.events.iter()
            .filter(|e| std::mem::discriminant(&e.event_type) == std::mem::discriminant(event_type))
            .collect()
    }
    
    /// Получение всех узлов от участника
    pub fn nodes_from_participant(&self, machine_id: &str) -> Vec<String> {
        self.events.iter()
            .filter(|e| e.source == machine_id)
            .filter(|e| matches!(e.event_type, TraceEventType::NodeAdded))
            .map(|e| e.description.clone())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct TraceStats {
    pub participants: usize,
    pub events: usize,
    pub genealogy_entries: usize,
    pub merges: usize,
    pub pending_changes: usize,
}
