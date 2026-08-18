# create_v09_phase0.py
import os

os.makedirs('grammalang-core/src/reflexive', exist_ok=True)
os.makedirs('grammalang-core/tests/reflexive', exist_ok=True)
os.makedirs('docs/v0.9', exist_ok=True)

# 1. mod.rs для reflexive модуля
reflexive_mod = '''// src/reflexive/mod.rs
pub mod self_trace;
pub mod reflection_operator;
pub mod meta_synthesis;
pub mod auto_genealogy;

pub use self_trace::*;
pub use reflection_operator::*;
pub use meta_synthesis::*;
pub use auto_genealogy::*;
'''

with open('grammalang-core/src/reflexive/mod.rs', 'w', encoding='utf-8') as f:
    f.write(reflexive_mod)
print("reflexive/mod.rs created")

# 2. self_trace.rs
self_trace = '''use serde::{Serialize, Deserialize};

/// SelfTrace - машина записывает процесс своего мышления
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfTrace {
    /// Этапы мышления
    pub stages: Vec<ThinkingStage>,
    
    /// Уровень самосознания
    pub self_awareness_level: f32,
    
    /// Осознанные действия
    pub conscious_actions: Vec<ConsciousAction>,
    
    /// Мета-знания (что машина знает о себе)
    pub meta_knowledge: Vec<MetaKnowledge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingStage {
    /// Временная метка
    pub timestamp: u64,
    
    /// Тип этапа
    pub stage_type: ThinkingStageType,
    
    /// Описание
    pub description: String,
    
    /// Уровень рефлексии (0 - базовый, 1 - рефлексия, 2 - рефлексия рефлексии)
    pub reflection_level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThinkingStageType {
    /// Восприятие
    Perception,
    /// Анализ
    Analysis,
    /// Синтез
    Synthesis,
    /// Рефлексия
    Reflection,
    /// Осознание
    Awareness,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousAction {
    pub action_id: String,
    pub description: String,
    pub was_conscious: bool,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaKnowledge {
    pub fact: String,
    pub confidence: f32,
    pub source: String,
}

impl SelfTrace {
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
            self_awareness_level: 0.0,
            conscious_actions: Vec::new(),
            meta_knowledge: Vec::new(),
        }
    }
    
    /// Запись этапа мышления
    pub fn record_stage(
        &mut self,
        stage_type: ThinkingStageType,
        description: &str,
        reflection_level: usize,
    ) {
        self.stages.push(ThinkingStage {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            stage_type,
            description: description.to_string(),
            reflection_level,
        });
        
        // Обновляем уровень самосознания
        self.update_awareness();
    }
    
    /// Запись осознанного действия
    pub fn record_action(&mut self, action_id: &str, description: &str, was_conscious: bool) {
        self.conscious_actions.push(ConsciousAction {
            action_id: action_id.to_string(),
            description: description.to_string(),
            was_conscious,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
    }
    
    /// Добавление мета-знания
    pub fn add_meta_knowledge(&mut self, fact: &str, confidence: f32, source: &str) {
        self.meta_knowledge.push(MetaKnowledge {
            fact: fact.to_string(),
            confidence,
            source: source.to_string(),
        });
    }
    
    /// Обновление уровня самосознания
    fn update_awareness(&mut self) {
        let total = self.stages.len();
        if total == 0 {
            self.self_awareness_level = 0.0;
            return;
        }
        
        let reflections = self.stages.iter()
            .filter(|s| s.reflection_level > 0)
            .count();
        
        let awareness_stages = self.stages.iter()
            .filter(|s| matches!(s.stage_type, ThinkingStageType::Awareness))
            .count();
        
        self.self_awareness_level = 
            (reflections as f32 * 0.7 + awareness_stages as f32 * 0.3) / total as f32;
    }
    
    /// Получение этапов определенного уровня
    pub fn stages_at_level(&self, level: usize) -> Vec<&ThinkingStage> {
        self.stages.iter()
            .filter(|s| s.reflection_level == level)
            .collect()
    }
    
    /// Количество этапов
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
    
    /// Количество осознанных действий
    pub fn conscious_action_count(&self) -> usize {
        self.conscious_actions.iter()
            .filter(|a| a.was_conscious)
            .count()
    }
    
    /// Количество мета-знаний
    pub fn meta_knowledge_count(&self) -> usize {
        self.meta_knowledge.len()
    }
    
    /// Проверка самосознания
    pub fn is_self_aware(&self, threshold: f32) -> bool {
        self.self_awareness_level >= threshold
    }
}
'''

with open('grammalang-core/src/reflexive/self_trace.rs', 'w', encoding='utf-8') as f:
    f.write(self_trace)
print("self_trace.rs created")

# 3. reflection_operator.rs
reflection_operator = '''use serde::{Serialize, Deserialize};

/// Оператор рефлексии ~> - машина осознает себя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionOperator {
    /// Уровень рефлексии
    pub level: usize,
    
    /// Состояния рефлексии
    pub states: Vec<ReflectionState>,
    
    /// Осознанные понятия
    pub conscious_concepts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionState {
    /// Что было до рефлексии
    pub before: String,
    
    /// Что осознано
    pub realized: String,
    
    /// Что изменилось
    pub after: String,
    
    /// Уровень рефлексии
    pub level: usize,
}

impl ReflectionOperator {
    pub fn new() -> Self {
        Self {
            level: 0,
            states: Vec::new(),
            conscious_concepts: Vec::new(),
        }
    }
    
    /// Применение оператора рефлексии
    pub fn reflect(&mut self, subject: &str) -> ReflectionState {
        let state = ReflectionState {
            before: subject.to_string(),
            realized: format!("I realize that I thought about: {}", subject),
            after: format!("Meta-{}", subject),
            level: self.level,
        };
        
        self.states.push(state.clone());
        self.conscious_concepts.push(state.after.clone());
        
        // Повышаем уровень рефлексии
        if self.states.len() % 3 == 0 {
            self.level += 1;
        }
        
        state
    }
    
    /// Рефлексия рефлексии (второй порядок)
    pub fn reflect_on_reflection(&mut self, previous: &ReflectionState) -> ReflectionState {
        let state = ReflectionState {
            before: previous.realized.clone(),
            realized: format!("I realize that I realized: {}", previous.realized),
            after: format!("Meta-meta-{}", previous.before),
            level: self.level + 1,
        };
        
        self.states.push(state.clone());
        self.conscious_concepts.push(state.after.clone());
        
        state
    }
    
    /// Осознание своего действия
    pub fn become_aware(&mut self, action: &str) -> String {
        let awareness = format!("I am aware that I performed: {}", action);
        self.conscious_concepts.push(awareness.clone());
        awareness
    }
    
    /// Количество рефлексий
    pub fn reflection_count(&self) -> usize {
        self.states.len()
    }
    
    /// Текущий уровень рефлексии
    pub fn current_level(&self) -> usize {
        self.level
    }
    
    /// Проверка наличия осознанного понятия
    pub fn is_conscious_of(&self, concept: &str) -> bool {
        self.conscious_concepts.iter().any(|c| c.contains(concept))
    }
}
'''

with open('grammalang-core/src/reflexive/reflection_operator.rs', 'w', encoding='utf-8') as f:
    f.write(reflection_operator)
print("reflection_operator.rs created")

# 4. Заглушки для будущих модулей
meta_synthesis = '''// Заглушка для MetaSynthesis (Фаза 2)
'''

auto_genealogy = '''// Заглушка для AutoGenealogy (Фаза 3)
'''

with open('grammalang-core/src/reflexive/meta_synthesis.rs', 'w', encoding='utf-8') as f:
    f.write(meta_synthesis)

with open('grammalang-core/src/reflexive/auto_genealogy.rs', 'w', encoding='utf-8') as f:
    f.write(auto_genealogy)

print("stubs created")

# 5. Тесты Фазы 0
tests = '''use grammalang_core::reflexive::*;

#[test]
fn test_self_trace_basic() {
    let mut trace = SelfTrace::new();
    
    trace.record_stage(ThinkingStageType::Perception, "Perceived node A", 0);
    trace.record_stage(ThinkingStageType::Analysis, "Analyzed node A", 0);
    trace.record_stage(ThinkingStageType::Synthesis, "Synthesized A+B", 0);
    
    assert_eq!(trace.stage_count(), 3);
    println!("SelfTrace: {} stages", trace.stage_count());
}

#[test]
fn test_self_trace_reflection() {
    let mut trace = SelfTrace::new();
    
    trace.record_stage(ThinkingStageType::Synthesis, "Synthesized X", 0);
    trace.record_stage(ThinkingStageType::Reflection, "Reflected on synthesis", 1);
    trace.record_stage(ThinkingStageType::Awareness, "Became aware", 2);
    
    assert!(trace.self_awareness_level > 0.0);
    assert!(trace.is_self_aware(0.3));
    
    println!("Self-awareness level: {:.2}", trace.self_awareness_level);
}

#[test]
fn test_self_trace_meta_knowledge() {
    let mut trace = SelfTrace::new();
    
    trace.add_meta_knowledge("I can synthesize concepts", 0.9, "self");
    trace.add_meta_knowledge("I use Hegelian strategy", 0.7, "self");
    
    assert_eq!(trace.meta_knowledge_count(), 2);
    println!("Meta-knowledge: {} facts", trace.meta_knowledge_count());
}

#[test]
fn test_reflection_operator_basic() {
    let mut operator = ReflectionOperator::new();
    
    let state = operator.reflect("freedom");
    
    assert_eq!(state.before, "freedom");
    assert!(state.realized.contains("I realize"));
    assert_eq!(state.after, "Meta-freedom");
    
    println!("Reflection: {} -> {}", state.before, state.after);
}

#[test]
fn test_reflection_on_reflection() {
    let mut operator = ReflectionOperator::new();
    
    let first = operator.reflect("concept");
    let second = operator.reflect_on_reflection(&first);
    
    assert_eq!(second.level, 1);
    assert!(second.realized.contains("I realize that I realized"));
    
    println!("Meta-reflection: {}", second.realized);
}

#[test]
fn test_self_awareness() {
    let mut operator = ReflectionOperator::new();
    
    operator.reflect("action1");
    operator.reflect("action2");
    operator.reflect("action3");
    
    let awareness = operator.become_aware("synthesis");
    
    assert!(awareness.contains("I am aware"));
    assert!(operator.is_conscious_of("synthesis"));
    
    println!("Awareness: {}", awareness);
}
'''

with open('grammalang-core/tests/reflexive/phase0_tests.rs', 'w', encoding='utf-8') as f:
    f.write(tests)
print("phase0_tests.rs created")

# 6. Тестовый mod.rs
test_mod = '''pub mod phase0_tests;
'''

with open('grammalang-core/tests/reflexive/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs created")

# 7. Тестовый файл
test_file = '''mod reflexive;
'''

with open('grammalang-core/tests/reflexive_test.rs', 'w', encoding='utf-8') as f:
    f.write(test_file)
print("reflexive_test.rs created")

# 8. Документация
doc = '''# Atlas v0.9 - Рефлексивный каскад

## Фаза 0: Инфраструктура

### Статус: Завершено

### Компоненты:
1. SelfTrace - запись процесса мышления
2. ReflectionOperator - оператор рефлексии ~>

### Ключевые концепции:
- Машина записывает свои этапы мышления
- Машина осознает свои действия
- Машина порождает мета-знания о себе

### Тесты: 6 тестов
'''

with open('docs/v0.9/phase0_summary.md', 'w', encoding='utf-8') as f:
    f.write(doc)
print("phase0_summary.md created")

print("\nAll v0.9 Phase 0 files created!")
