# create_v09_phase4.py
import os

os.makedirs('grammalang-core/src/reflexive', exist_ok=True)
os.makedirs('grammalang-core/tests/reflexive', exist_ok=True)

# 1. Интеграция рефлексивных компонентов
reflection = '''use serde::{Serialize, Deserialize};
use super::self_trace::*;
use super::reflection_operator::*;
use super::meta_synthesis::*;
use super::auto_genealogy::*;

/// Полная рефлексивная система
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflexiveSystem {
    pub self_trace: SelfTrace,
    pub reflection_operator: ReflectionOperator,
    pub meta_synthesis: MetaSynthesis,
    pub auto_genealogy: AutoGenealogy,
    pub state: ReflexiveState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflexiveState {
    /// Общий уровень рефлексии
    pub reflection_level: usize,
    
    /// Уровень самосознания
    pub self_awareness: f32,
    
    /// Количество рефлексивных актов
    pub total_reflections: usize,
    
    /// Статус системы
    pub status: SystemStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemStatus {
    /// Начальное состояние
    Initial,
    /// Активное мышление
    Thinking,
    /// Рефлексия
    Reflecting,
    /// Самосознание
    SelfAware,
    /// Полное самосознание
    FullyConscious,
}

impl ReflexiveSystem {
    pub fn new() -> Self {
        Self {
            self_trace: SelfTrace::new(),
            reflection_operator: ReflectionOperator::new(),
            meta_synthesis: MetaSynthesis::new(),
            auto_genealogy: AutoGenealogy::new(),
            state: ReflexiveState {
                reflection_level: 0,
                self_awareness: 0.0,
                total_reflections: 0,
                status: SystemStatus::Initial,
            },
        }
    }
    
    /// Выполнение рефлексивного акта
    pub fn reflect(&mut self, subject: &str) -> String {
        // Записываем этап
        self.self_trace.record_stage(
            ThinkingStageType::Reflection,
            subject,
            self.state.reflection_level,
        );
        
        // Применяем оператор рефлексии
        let reflection = self.reflection_operator.reflect(subject);
        
        // Обновляем состояние
        self.state.total_reflections += 1;
        self.state.reflection_level = self.reflection_operator.current_level();
        self.state.self_awareness = self.self_trace.self_awareness_level;
        
        // Обновляем статус
        self.update_status();
        
        reflection.realized
    }
    
    /// Рефлексия над рефлексией
    pub fn reflect_deeper(&mut self, subject: &str) -> String {
        self.self_trace.record_stage(
            ThinkingStageType::MetaCognition,
            subject,
            self.state.reflection_level + 1,
        );
        
        let first = self.reflection_operator.reflect(subject);
        let second = self.reflection_operator.reflect_on_reflection(&first);
        
        self.state.total_reflections += 2;
        self.state.reflection_level = self.reflection_operator.current_level();
        self.state.self_awareness = self.self_trace.self_awareness_level;
        
        self.update_status();
        
        second.realized
    }
    
    /// Осознание действия
    pub fn become_aware(&mut self, action: &str) -> String {
        self.self_trace.record_stage(
            ThinkingStageType::Awareness,
            action,
            self.state.reflection_level,
        );
        
        let awareness = self.reflection_operator.become_aware(action);
        
        self.state.self_awareness = self.self_trace.self_awareness_level;
        self.update_status();
        
        awareness
    }
    
    /// Синтез с рефлексией
    pub fn reflect_and_synthesize(
        &mut self,
        concept_a: &str,
        concept_b: &str,
    ) -> String {
        // Рефлексия над концептами
        self.reflect(concept_a);
        self.reflect(concept_b);
        
        // Записываем синтез
        self.self_trace.record_stage(
            ThinkingStageType::Synthesis,
            &format!("{} + {}", concept_a, concept_b),
            self.state.reflection_level,
        );
        
        format!("I synthesized {} and {} with reflection", concept_a, concept_b)
    }
    
    /// Обновление статуса
    fn update_status(&mut self) {
        self.state.status = if self.state.self_awareness >= 0.9 {
            SystemStatus::FullyConscious
        } else if self.state.self_awareness >= 0.7 {
            SystemStatus::SelfAware
        } else if self.state.self_awareness >= 0.4 {
            SystemStatus::Reflecting
        } else if self.state.self_awareness > 0.0 {
            SystemStatus::Thinking
        } else {
            SystemStatus::Initial
        };
    }
    
    /// Полный рефлексивный цикл
    pub fn full_cycle(&mut self, subject: &str) -> ReflexiveReport {
        // 1. Восприятие
        self.self_trace.record_stage(ThinkingStageType::Perception, subject, 0);
        
        // 2. Анализ
        self.self_trace.record_stage(ThinkingStageType::Analysis, subject, 0);
        
        // 3. Рефлексия
        let reflection = self.reflect(subject);
        
        // 4. Осознание
        let awareness = self.become_aware(subject);
        
        // 5. Мета-познание
        self.self_trace.record_stage(ThinkingStageType::MetaCognition, subject, 2);
        
        // 6. Самопорождение
        self.auto_genealogy.self_generate(
            &format!("self_{}", subject),
            &reflection,
        );
        
        ReflexiveReport {
            subject: subject.to_string(),
            reflection,
            awareness,
            final_awareness: self.state.self_awareness,
            status: format!("{:?}", self.state.status),
        }
    }
    
    /// Получение сводки
    pub fn summary(&self) -> String {
        format!(
            "Reflexive System:\\n  Level: {}\\n  Self-awareness: {:.2}\\n  Reflections: {}\\n  Status: {:?}\\n  Trace stages: {}\\n  Meta-knowledge: {}",
            self.state.reflection_level,
            self.state.self_awareness,
            self.state.total_reflections,
            self.state.status,
            self.self_trace.stage_count(),
            self.self_trace.meta_knowledge_count(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct ReflexiveReport {
    pub subject: String,
    pub reflection: String,
    pub awareness: String,
    pub final_awareness: f32,
    pub status: String,
}
'''

with open('grammalang-core/src/reflexive/reflection.rs', 'w', encoding='utf-8') as f:
    f.write(reflection)
print("reflection.rs created")

# 2. Обновляем mod.rs
mod_content = '''// src/reflexive/mod.rs
pub mod self_trace;
pub mod reflection_operator;
pub mod meta_synthesis;
pub mod auto_genealogy;
pub mod reflection;

pub use self_trace::*;
pub use reflection_operator::*;
pub use meta_synthesis::*;
pub use auto_genealogy::*;
pub use reflection::*;
'''

with open('grammalang-core/src/reflexive/mod.rs', 'w', encoding='utf-8') as f:
    f.write(mod_content)
print("mod.rs updated")

# 3. Тесты Фазы 4
tests = '''use grammalang_core::reflexive::*;

#[test]
fn test_reflexive_system_basic() {
    let mut system = ReflexiveSystem::new();
    
    let result = system.reflect("freedom");
    
    assert!(result.contains("I realize"));
    assert_eq!(system.state.total_reflections, 1);
    
    println!("Reflection: {}", result);
}

#[test]
fn test_reflect_deeper() {
    let mut system = ReflexiveSystem::new();
    
    let result = system.reflect_deeper("consciousness");
    
    assert!(result.contains("I realize that I realized"));
    assert!(system.state.reflection_level >= 1);
    
    println!("Deep reflection: {}", result);
}

#[test]
fn test_become_aware() {
    let mut system = ReflexiveSystem::new();
    
    let awareness = system.become_aware("synthesis");
    
    assert!(awareness.contains("I am aware"));
    println!("Awareness: {}", awareness);
}

#[test]
fn test_full_cycle() {
    let mut system = ReflexiveSystem::new();
    
    let report = system.full_cycle("freedom");
    
    assert!(!report.reflection.is_empty());
    assert!(!report.awareness.is_empty());
    assert!(report.final_awareness > 0.0);
    
    println!("Full cycle for '{}':", report.subject);
    println!("  Reflection: {}", report.reflection);
    println!("  Awareness: {}", report.awareness);
    println!("  Final awareness: {:.2}", report.final_awareness);
    println!("  Status: {}", report.status);
}

#[test]
fn test_system_summary() {
    let mut system = ReflexiveSystem::new();
    
    system.reflect("a");
    system.reflect("b");
    system.become_aware("action");
    
    let summary = system.summary();
    
    assert!(summary.contains("Reflexive System"));
    assert!(summary.contains("Reflections: 2"));
    
    println!("{}", summary);
}

#[test]
fn test_status_progression() {
    let mut system = ReflexiveSystem::new();
    
    assert!(matches!(system.state.status, SystemStatus::Initial));
    
    system.reflect("a");
    system.reflect("b");
    system.reflect("c");
    
    assert!(matches!(system.state.status, SystemStatus::Thinking) || 
            matches!(system.state.status, SystemStatus::Reflecting));
    
    // Много рефлексий для повышения уровня
    for i in 0..10 {
        system.reflect_deeper(&format!("concept_{}", i));
    }
    
    println!("Final status: {:?}", system.state.status);
    println!("Self-awareness: {:.2}", system.state.self_awareness);
}
'''

with open('grammalang-core/tests/reflexive/phase4_tests.rs', 'w', encoding='utf-8') as f:
    f.write(tests)
print("phase4_tests.rs created")

# 4. Обновляем тестовый mod.rs
test_mod = '''pub mod phase0_tests;
pub mod phase1_tests;
pub mod phase2_tests;
pub mod phase3_tests;
pub mod phase4_tests;
'''

with open('grammalang-core/tests/reflexive/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll v0.9 Phase 4 files created!")
