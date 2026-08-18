# create_v08_phase2.py
import os

os.makedirs('grammalang-core/src/social', exist_ok=True)
os.makedirs('grammalang-core/tests/social', exist_ok=True)

# 1. Расширенный CollectiveTrace
collective_trace = '''use serde::{Serialize, Deserialize};
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
'''

with open('grammalang-core/src/social/collective_trace.rs', 'w', encoding='utf-8') as f:
    f.write(collective_trace)
print("collective_trace.rs updated")

# 2. Тесты Фазы 2
tests = '''use grammalang_core::social::*;

#[test]
fn test_collective_trace_basic() {
    let mut trace = CollectiveTrace::new();
    
    trace.add_participant(Participant {
        machine_id: "m1".to_string(),
        name: "Machine 1".to_string(),
        role: ParticipantRole::Leader,
        contribution: 10,
    });
    
    trace.record_event("m1", TraceEventType::NodeAdded, "node_a");
    trace.record_event("m1", TraceEventType::Synthesis, "synthesis_1");
    
    assert_eq!(trace.participant_count(), 1);
    assert_eq!(trace.event_count(), 2);
    
    println!("Basic trace: {} participants, {} events", 
             trace.participant_count(), trace.event_count());
}

#[test]
fn test_trace_merge() {
    let mut trace1 = CollectiveTrace::new();
    let mut trace2 = CollectiveTrace::new();
    
    trace1.add_participant(Participant {
        machine_id: "m1".to_string(),
        name: "Machine 1".to_string(),
        role: ParticipantRole::Leader,
        contribution: 5,
    });
    
    trace2.add_participant(Participant {
        machine_id: "m2".to_string(),
        name: "Machine 2".to_string(),
        role: ParticipantRole::Member,
        contribution: 3,
    });
    
    trace2.record_event("m2", TraceEventType::NodeAdded, "node_from_m2");
    trace2.add_genealogy("node_from_m2", vec!["parent_a".to_string()]);
    
    let merged = trace1.merge(&trace2).unwrap();
    
    assert!(merged > 0);
    assert_eq!(trace1.participant_count(), 2);
    
    println!("Merged: {} items from trace2", merged);
    println!("Total participants: {}", trace1.participant_count());
}

#[test]
fn test_trace_genealogy() {
    let mut trace = CollectiveTrace::new();
    
    trace.add_genealogy("synthesis_1", vec!["thesis".to_string(), "antithesis".to_string()]);
    trace.add_genealogy("thesis", vec!["origin".to_string()]);
    
    let history = trace.get_history("synthesis_1").unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0], "thesis");
    assert_eq!(history[1], "antithesis");
    
    println!("Genealogy of synthesis_1: {:?}", history);
}

#[test]
fn test_trace_sync() {
    let mut trace = CollectiveTrace::new();
    
    trace.start_sync();
    assert!(trace.sync_state.is_syncing);
    
    trace.record_event("m1", TraceEventType::NodeAdded, "during_sync");
    trace.sync_state.pending_changes += 1;
    
    trace.finish_sync();
    assert!(!trace.sync_state.is_syncing);
    assert_eq!(trace.sync_state.pending_changes, 1);
    
    println!("Sync completed with {} pending changes", trace.sync_state.pending_changes);
}

#[test]
fn test_trace_stats() {
    let mut trace = CollectiveTrace::new();
    
    trace.add_participant(Participant {
        machine_id: "m1".to_string(),
        name: "M1".to_string(),
        role: ParticipantRole::Leader,
        contribution: 7,
    });
    
    trace.record_event("m1", TraceEventType::NodeAdded, "node1");
    trace.record_event("m1", TraceEventType::Synthesis, "synth1");
    trace.add_genealogy("synth1", vec!["node1".to_string()]);
    
    let stats = trace.stats();
    
    assert_eq!(stats.participants, 1);
    assert_eq!(stats.events, 2);
    assert_eq!(stats.genealogy_entries, 1);
    
    println!("Stats: {} participants, {} events, {} genealogy entries", 
             stats.participants, stats.events, stats.genealogy_entries);
}

#[test]
fn test_find_events_by_type() {
    let mut trace = CollectiveTrace::new();
    
    trace.record_event("m1", TraceEventType::NodeAdded, "node1");
    trace.record_event("m1", TraceEventType::Synthesis, "synth1");
    trace.record_event("m2", TraceEventType::NodeAdded, "node2");
    
    let node_events = trace.find_events(&TraceEventType::NodeAdded);
    assert_eq!(node_events.len(), 2);
    
    let synthesis_events = trace.find_events(&TraceEventType::Synthesis);
    assert_eq!(synthesis_events.len(), 1);
    
    println!("Found {} node events, {} synthesis events", 
             node_events.len(), synthesis_events.len());
}

#[test]
fn test_protocol_mismatch() {
    let mut trace1 = CollectiveTrace::new();
    let mut trace2 = CollectiveTrace::new();
    trace2.protocol_version = "0.7.0".to_string();
    
    let result = trace1.merge(&trace2);
    assert!(result.is_err());
    
    println!("Protocol mismatch detected correctly");
}
'''

with open('grammalang-core/tests/social/phase2_tests.rs', 'w', encoding='utf-8') as f:
    f.write(tests)
print("phase2_tests.rs created")

# 3. Обновляем тестовый mod.rs
test_mod = '''pub mod phase0_tests;
pub mod phase1_tests;
pub mod phase2_tests;
'''

with open('grammalang-core/tests/social/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll v0.8 Phase 2 files created!")
