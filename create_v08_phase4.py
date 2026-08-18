# create_v08_phase4.py
import os

os.makedirs('grammalang-core/src/social', exist_ok=True)
os.makedirs('grammalang-core/tests/social', exist_ok=True)

# 1. Расширенный Federation
federation = '''use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Федерация - протокол обмена между машинами
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Federation {
    pub members: Vec<FederationMember>,
    pub exchanged_nodes: Vec<ExchangedNode>,
    pub status: FederationStatus,
    pub sync_queue: Vec<SyncRequest>,
    pub consensus: ConsensusState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationMember {
    pub machine_id: String,
    pub address: String,
    pub connected: bool,
    pub capabilities: Vec<MemberCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemberCapability {
    /// Может генерировать синтез
    Synthesis,
    /// Может хранить знания
    KnowledgeStorage,
    /// Может обрабатывать противоречия
    ContradictionProcessing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangedNode {
    pub node_id: String,
    pub from: String,
    pub to: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FederationStatus {
    Active,
    Inactive,
    Syncing,
    ConsensusBuilding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequest {
    pub from: String,
    pub to: String,
    pub nodes: Vec<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsensusState {
    /// Текущий раунд консенсуса
    pub round: usize,
    /// Голоса участников
    pub votes: HashMap<String, bool>,
    /// Достигнут ли консенсус
    pub consensus_reached: bool,
}

impl Federation {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
            exchanged_nodes: Vec::new(),
            status: FederationStatus::Inactive,
            sync_queue: Vec::new(),
            consensus: ConsensusState::default(),
        }
    }
    
    /// Добавление участника
    pub fn add_member(&mut self, member: FederationMember) {
        self.members.push(member);
        self.status = FederationStatus::Active;
    }
    
    /// Обмен узлами
    pub fn exchange(&mut self, node_id: &str, from: &str, to: &str) {
        self.exchanged_nodes.push(ExchangedNode {
            node_id: node_id.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
    }
    
    /// Добавление запроса на синхронизацию
    pub fn request_sync(&mut self, from: &str, to: &str, nodes: Vec<String>) {
        self.sync_queue.push(SyncRequest {
            from: from.to_string(),
            to: to.to_string(),
            nodes,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        self.status = FederationStatus::Syncing;
    }
    
    /// Обработка очереди синхронизации
    pub fn process_sync_queue(&mut self) -> usize {
        let count = self.sync_queue.len();
        
        while let Some(request) = self.sync_queue.pop() {
            for node in &request.nodes {
                self.exchange(node, &request.from, &request.to);
            }
        }
        
        self.status = FederationStatus::Active;
        count
    }
    
    /// Начало голосования
    pub fn start_consensus(&mut self) {
        self.status = FederationStatus::ConsensusBuilding;
        self.consensus.round += 1;
        self.consensus.votes.clear();
        self.consensus.consensus_reached = false;
    }
    
    /// Голосование участника
    pub fn vote(&mut self, machine_id: &str, approve: bool) {
        self.consensus.votes.insert(machine_id.to_string(), approve);
        
        // Проверяем консенсус
        let total = self.members.len();
        let voted = self.consensus.votes.len();
        
        if total > 0 && voted >= total {
            let approvals = self.consensus.votes.values().filter(|&&v| v).count();
            self.consensus.consensus_reached = approvals as f32 / total as f32 > 0.5;
            self.status = FederationStatus::Active;
        }
    }
    
    /// Проверка консенсуса
    pub fn has_consensus(&self) -> bool {
        self.consensus.consensus_reached
    }
    
    /// Получение члена по ID
    pub fn get_member(&self, machine_id: &str) -> Option<&FederationMember> {
        self.members.iter().find(|m| m.machine_id == machine_id)
    }
    
    /// Проверка возможностей
    pub fn has_capability(&self, machine_id: &str, capability: &MemberCapability) -> bool {
        self.get_member(machine_id)
            .map(|m| m.capabilities.iter().any(|c| 
                std::mem::discriminant(c) == std::mem::discriminant(capability)))
            .unwrap_or(false)
    }
    
    /// Количество участников
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
    
    /// Количество обменов
    pub fn exchange_count(&self) -> usize {
        self.exchanged_nodes.len()
    }
    
    /// Количество в очереди
    pub fn sync_queue_size(&self) -> usize {
        self.sync_queue.len()
    }
    
    /// Статистика федерации
    pub fn stats(&self) -> FederationStats {
        FederationStats {
            members: self.members.len(),
            exchanges: self.exchanged_nodes.len(),
            sync_queue: self.sync_queue.len(),
            consensus_round: self.consensus.round,
            consensus_reached: self.consensus.consensus_reached,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FederationStats {
    pub members: usize,
    pub exchanges: usize,
    pub sync_queue: usize,
    pub consensus_round: usize,
    pub consensus_reached: bool,
}

/// Протокол обмена сообщениями
pub struct MessageProtocol {
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub message_type: MessageType,
    pub payload: String,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    /// Запрос узла
    NodeRequest,
    /// Ответ с узлом
    NodeResponse,
    /// Объявление
    Announcement,
    /// Голосование
    Vote,
}

impl MessageProtocol {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    
    /// Отправка сообщения
    pub fn send(&mut self, from: &str, to: &str, message_type: MessageType, payload: &str) {
        self.messages.push(Message {
            from: from.to_string(),
            to: to.to_string(),
            message_type,
            payload: payload.to_string(),
        });
    }
    
    /// Получение сообщений для машины
    pub fn receive(&self, machine_id: &str) -> Vec<&Message> {
        self.messages.iter()
            .filter(|m| m.to == machine_id)
            .collect()
    }
    
    /// Количество сообщений
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}
'''

with open('grammalang-core/src/social/federation.rs', 'w', encoding='utf-8') as f:
    f.write(federation)
print("federation.rs updated")

# 2. Тесты Фазы 4
tests = '''use grammalang_core::social::*;

#[test]
fn test_federation_basic() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![MemberCapability::Synthesis],
    });
    
    fed.add_member(FederationMember {
        machine_id: "m2".to_string(),
        address: "localhost:8002".to_string(),
        connected: true,
        capabilities: vec![MemberCapability::KnowledgeStorage],
    });
    
    assert_eq!(fed.member_count(), 2);
    println!("Federation: {} members", fed.member_count());
}

#[test]
fn test_node_exchange() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![],
    });
    
    fed.add_member(FederationMember {
        machine_id: "m2".to_string(),
        address: "localhost:8002".to_string(),
        connected: true,
        capabilities: vec![],
    });
    
    fed.exchange("node1", "m1", "m2");
    fed.exchange("node2", "m2", "m1");
    
    assert_eq!(fed.exchange_count(), 2);
    println!("Exchanges: {}", fed.exchange_count());
}

#[test]
fn test_sync_queue() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![],
    });
    
    fed.request_sync("m1", "m2", vec!["node1".to_string(), "node2".to_string()]);
    
    assert_eq!(fed.sync_queue_size(), 1);
    
    let processed = fed.process_sync_queue();
    
    assert_eq!(processed, 1);
    assert_eq!(fed.exchange_count(), 2);
    
    println!("Sync: {} requests processed, {} exchanges", 
             processed, fed.exchange_count());
}

#[test]
fn test_consensus() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![],
    });
    
    fed.add_member(FederationMember {
        machine_id: "m2".to_string(),
        address: "localhost:8002".to_string(),
        connected: true,
        capabilities: vec![],
    });
    
    fed.start_consensus();
    fed.vote("m1", true);
    fed.vote("m2", true);
    
    assert!(fed.has_consensus());
    println!("Consensus reached in round {}", fed.consensus.round);
}

#[test]
fn test_capabilities() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![MemberCapability::Synthesis, MemberCapability::KnowledgeStorage],
    });
    
    assert!(fed.has_capability("m1", &MemberCapability::Synthesis));
    assert!(fed.has_capability("m1", &MemberCapability::KnowledgeStorage));
    assert!(!fed.has_capability("m1", &MemberCapability::ContradictionProcessing));
    
    println!("Capabilities checked correctly");
}

#[test]
fn test_message_protocol() {
    let mut protocol = MessageProtocol::new();
    
    protocol.send("m1", "m2", MessageType::NodeRequest, "node1");
    protocol.send("m2", "m1", MessageType::NodeResponse, "node1_data");
    protocol.send("m1", "all", MessageType::Announcement, "new_node_available");
    
    assert_eq!(protocol.message_count(), 3);
    
    let for_m2 = protocol.receive("m2");
    assert_eq!(for_m2.len(), 1);
    
    let for_m1 = protocol.receive("m1");
    assert_eq!(for_m1.len(), 1);
    
    println!("Message protocol: {} messages", protocol.message_count());
}

#[test]
fn test_federation_stats() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![],
    });
    
    fed.exchange("node1", "m1", "m2");
    fed.request_sync("m1", "m2", vec!["node2".to_string()]);
    
    let stats = fed.stats();
    
    assert_eq!(stats.members, 1);
    assert_eq!(stats.exchanges, 1);
    assert_eq!(stats.sync_queue, 1);
    
    println!("Stats: {} members, {} exchanges, {} in queue", 
             stats.members, stats.exchanges, stats.sync_queue);
}
'''

with open('grammalang-core/tests/social/phase4_tests.rs', 'w', encoding='utf-8') as f:
    f.write(tests)
print("phase4_tests.rs created")

# 3. Обновляем тестовый mod.rs
test_mod = '''pub mod phase0_tests;
pub mod phase1_tests;
pub mod phase2_tests;
pub mod phase3_tests;
pub mod phase4_tests;
'''

with open('grammalang-core/tests/social/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll v0.8 Phase 4 files created!")
