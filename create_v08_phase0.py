# create_v08_phase0.py
import os

os.makedirs('grammalang-core/src/social', exist_ok=True)
os.makedirs('grammalang-core/tests/social', exist_ok=True)
os.makedirs('docs/v0.8', exist_ok=True)

# 1. mod.rs для social модуля
social_mod = '''// src/social/mod.rs
pub mod knowledge_base;
pub mod collective_trace;
pub mod social_reactor;
pub mod federation;

pub use knowledge_base::*;
pub use collective_trace::*;
pub use social_reactor::*;
pub use federation::*;
'''

with open('grammalang-core/src/social/mod.rs', 'w', encoding='utf-8') as f:
    f.write(social_mod)
print("social/mod.rs created")

# 2. knowledge_base.rs
knowledge_base = '''use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Внешняя база знаний
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    /// Идентификатор базы
    pub id: String,
    
    /// Название
    pub name: String,
    
    /// Тип базы
    pub kb_type: KnowledgeBaseType,
    
    /// Узлы знаний
    pub nodes: Vec<KnowledgeNode>,
    
    /// Метаданные
    pub metadata: KnowledgeBaseMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnowledgeBaseType {
    /// Wikidata
    Wikidata,
    /// DBPedia
    DBPedia,
    /// Локальная JSON-LD
    JsonLd,
    /// Пользовательская
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    /// Идентификатор узла
    pub id: String,
    
    /// Название
    pub label: String,
    
    /// Описание
    pub description: String,
    
    /// Свойства
    pub properties: HashMap<String, String>,
    
    /// Связи с другими узлами
    pub relations: Vec<KnowledgeRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeRelation {
    /// Целевой узел
    pub target: String,
    
    /// Тип связи
    pub relation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBaseMetadata {
    /// Версия базы
    pub version: String,
    
    /// Дата обновления
    pub updated_at: String,
    
    /// Количество узлов
    pub node_count: usize,
    
    /// Источник
    pub source_url: Option<String>,
}

impl KnowledgeBase {
    pub fn new(id: &str, name: &str, kb_type: KnowledgeBaseType) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            kb_type,
            nodes: Vec::new(),
            metadata: KnowledgeBaseMetadata {
                version: "0.1".to_string(),
                updated_at: String::new(),
                node_count: 0,
                source_url: None,
            },
        }
    }
    
    /// Добавление узла
    pub fn add_node(&mut self, node: KnowledgeNode) {
        self.nodes.push(node);
        self.metadata.node_count = self.nodes.len();
    }
    
    /// Поиск узла по ID
    pub fn find_node(&self, id: &str) -> Option<&KnowledgeNode> {
        self.nodes.iter().find(|n| n.id == id)
    }
    
    /// Поиск узлов по свойству
    pub fn find_by_property(&self, key: &str, value: &str) -> Vec<&KnowledgeNode> {
        self.nodes.iter()
            .filter(|n| n.properties.get(key) == Some(&value.to_string()))
            .collect()
    }
    
    /// Получение всех связей узла
    pub fn get_relations(&self, node_id: &str) -> Vec<&KnowledgeRelation> {
        self.nodes.iter()
            .filter(|n| n.id == node_id)
            .flat_map(|n| n.relations.iter())
            .collect()
    }
}
'''

with open('grammalang-core/src/social/knowledge_base.rs', 'w', encoding='utf-8') as f:
    f.write(knowledge_base)
print("knowledge_base.rs created")

# 3. collective_trace.rs
collective_trace = '''use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Коллективный trace - объединенная генеалогия от множества машин
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveTrace {
    /// Участники (машины)
    pub participants: Vec<Participant>,
    
    /// Общие события
    pub events: Vec<TraceEvent>,
    
    /// Объединенная генеалогия
    pub genealogy: HashMap<String, Vec<String>>,
    
    /// Версия протокола
    pub protocol_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// ID машины
    pub machine_id: String,
    
    /// Имя машины
    pub name: String,
    
    /// Роль в коллективе
    pub role: ParticipantRole,
    
    /// Вклад (количество узлов)
    pub contribution: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantRole {
    /// Ведущая машина
    Leader,
    /// Участник
    Member,
    /// Наблюдатель
    Observer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEvent {
    /// Временная метка
    pub timestamp: u64,
    
    /// Машина-источник
    pub source: String,
    
    /// Тип события
    pub event_type: TraceEventType,
    
    /// Описание
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceEventType {
    /// Добавление узла
    NodeAdded,
    /// Синтез
    Synthesis,
    /// Противоречие
    Contradiction,
    /// Слияние
    Merge,
}

impl CollectiveTrace {
    pub fn new() -> Self {
        Self {
            participants: Vec::new(),
            events: Vec::new(),
            genealogy: HashMap::new(),
            protocol_version: "0.8.0".to_string(),
        }
    }
    
    /// Добавление участника
    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }
    
    /// Запись события
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
    
    /// Добавление генеалогической связи
    pub fn add_genealogy(&mut self, node: &str, ancestors: Vec<String>) {
        self.genealogy.insert(node.to_string(), ancestors);
    }
    
    /// Получение истории узла
    pub fn get_history(&self, node: &str) -> Option<&Vec<String>> {
        self.genealogy.get(node)
    }
    
    /// Общее количество событий
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
    
    /// Количество участников
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }
}
'''

with open('grammalang-core/src/social/collective_trace.rs', 'w', encoding='utf-8') as f:
    f.write(collective_trace)
print("collective_trace.rs created")

# 4. social_reactor.rs
social_reactor = '''use serde::{Serialize, Deserialize};

/// Социальный реактор - обработка коллективных противоречий
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialReactor {
    /// Порог коллективного противоречия
    pub collective_threshold: f32,
    
    /// Активные противоречия
    pub active_contradictions: Vec<SocialContradiction>,
    
    /// История реакций
    pub reaction_history: Vec<Reaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialContradiction {
    /// Первый источник
    pub source_a: String,
    
    /// Второй источник
    pub source_b: String,
    
    /// Степень противоречия
    pub severity: f32,
    
    /// Тип противоречия
    pub kind: SocialContradictionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialContradictionKind {
    /// Противоречие между базами знаний
    KnowledgeConflict,
    /// Противоречие между машинами
    MachineConflict,
    /// Противоречие в генеалогии
    GenealogyConflict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    /// Реакция на противоречие
    pub action: ReactionAction,
    
    /// Результат
    pub result: ReactionResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReactionAction {
    /// Слияние
    Merge,
    /// Разрешение
    Resolve,
    /// Отклонение
    Reject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReactionResult {
    /// Успешно
    Success,
    /// Неудача
    Failure,
    /// Отложено
    Pending,
}

impl SocialReactor {
    pub fn new() -> Self {
        Self {
            collective_threshold: 0.6,
            active_contradictions: Vec::new(),
            reaction_history: Vec::new(),
        }
    }
    
    /// Добавление противоречия
    pub fn add_contradiction(&mut self, contradiction: SocialContradiction) {
        if contradiction.severity >= self.collective_threshold {
            self.active_contradictions.push(contradiction);
        }
    }
    
    /// Обработка всех активных противоречий
    pub fn process(&mut self) -> usize {
        let count = self.active_contradictions.len();
        
        for _ in 0..count {
            if let Some(contradiction) = self.active_contradictions.pop() {
                // Обработка противоречия
                let reaction = Reaction {
                    action: ReactionAction::Resolve,
                    result: ReactionResult::Success,
                };
                self.reaction_history.push(reaction);
            }
        }
        
        count
    }
    
    /// Количество активных противоречий
    pub fn active_count(&self) -> usize {
        self.active_contradictions.len()
    }
    
    /// Количество обработанных
    pub fn processed_count(&self) -> usize {
        self.reaction_history.len()
    }
}
'''

with open('grammalang-core/src/social/social_reactor.rs', 'w', encoding='utf-8') as f:
    f.write(social_reactor)
print("social_reactor.rs created")

# 5. federation.rs
federation = '''use serde::{Serialize, Deserialize};

/// Федерация - протокол обмена между машинами
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Federation {
    /// Участники федерации
    pub members: Vec<FederationMember>,
    
    /// Обмененные узлы
    pub exchanged_nodes: Vec<ExchangedNode>,
    
    /// Статус федерации
    pub status: FederationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationMember {
    /// ID машины
    pub machine_id: String,
    
    /// Адрес
    pub address: String,
    
    /// Статус подключения
    pub connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangedNode {
    /// Узел
    pub node_id: String,
    
    /// Отправитель
    pub from: String,
    
    /// Получатель
    pub to: String,
    
    /// Время обмена
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FederationStatus {
    /// Активна
    Active,
    /// Неактивна
    Inactive,
    /// Синхронизация
    Syncing,
}

impl Federation {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
            exchanged_nodes: Vec::new(),
            status: FederationStatus::Inactive,
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
    
    /// Количество участников
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
    
    /// Количество обменов
    pub fn exchange_count(&self) -> usize {
        self.exchanged_nodes.len()
    }
}
'''

with open('grammalang-core/src/social/federation.rs', 'w', encoding='utf-8') as f:
    f.write(federation)
print("federation.rs created")

# 6. Тесты Фазы 0
tests = '''use grammalang_core::social::*;

#[test]
fn test_knowledge_base_creation() {
    let mut kb = KnowledgeBase::new("kb1", "Test KB", KnowledgeBaseType::Custom);
    
    let node = KnowledgeNode {
        id: "node1".to_string(),
        label: "Test Node".to_string(),
        description: "A test node".to_string(),
        properties: std::collections::HashMap::new(),
        relations: vec![],
    };
    
    kb.add_node(node);
    
    assert_eq!(kb.metadata.node_count, 1);
    assert!(kb.find_node("node1").is_some());
    println!("KnowledgeBase: {} nodes", kb.metadata.node_count);
}

#[test]
fn test_collective_trace() {
    let mut trace = CollectiveTrace::new();
    
    trace.add_participant(Participant {
        machine_id: "m1".to_string(),
        name: "Machine 1".to_string(),
        role: ParticipantRole::Leader,
        contribution: 10,
    });
    
    trace.record_event("m1", TraceEventType::Synthesis, "Test synthesis");
    
    assert_eq!(trace.participant_count(), 1);
    assert_eq!(trace.event_count(), 1);
    println!("CollectiveTrace: {} participants, {} events", 
             trace.participant_count(), trace.event_count());
}

#[test]
fn test_social_reactor() {
    let mut reactor = SocialReactor::new();
    
    reactor.add_contradiction(SocialContradiction {
        source_a: "kb1".to_string(),
        source_b: "kb2".to_string(),
        severity: 0.8,
        kind: SocialContradictionKind::KnowledgeConflict,
    });
    
    assert_eq!(reactor.active_count(), 1);
    
    let processed = reactor.process();
    assert_eq!(processed, 1);
    assert_eq!(reactor.processed_count(), 1);
    
    println!("SocialReactor: {} processed contradictions", processed);
}

#[test]
fn test_federation() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8000".to_string(),
        connected: true,
    });
    
    fed.exchange("node1", "m1", "m2");
    
    assert_eq!(fed.member_count(), 1);
    assert_eq!(fed.exchange_count(), 1);
    
    println!("Federation: {} members, {} exchanges", 
             fed.member_count(), fed.exchange_count());
}
'''

with open('grammalang-core/tests/social/phase0_tests.rs', 'w', encoding='utf-8') as f:
    f.write(tests)
print("phase0_tests.rs created")

# 7. mod.rs для тестов
test_mod = '''pub mod phase0_tests;
'''

with open('grammalang-core/tests/social/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs created")

# 8. Тестовый файл
test_file = '''mod social;
'''

with open('grammalang-core/tests/social_test.rs', 'w', encoding='utf-8') as f:
    f.write(test_file)
print("social_test.rs created")

# 9. Документация
doc = '''# Atlas v0.8 - Социальный реактор

## Фаза 0: Подготовка инфраструктуры

### Статус: Завершено

### Компоненты:
1. KnowledgeBase - внешние базы знаний
2. CollectiveTrace - коллективный trace
3. SocialReactor - обработка противоречий
4. Federation - протокол обмена

### Тесты: 4 теста
'''

with open('docs/v0.8/phase0_summary.md', 'w', encoding='utf-8') as f:
    f.write(doc)
print("phase0_summary.md created")

print("\\nAll v0.8 Phase 0 files created!")
