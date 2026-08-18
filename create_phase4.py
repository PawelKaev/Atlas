# create_phase4.py
import os

os.makedirs('grammalang-core/src/ontology', exist_ok=True)
os.makedirs('grammalang-core/tests/ontology', exist_ok=True)

# 1. synthesis_validator.rs
validator = '''use super::synthesis_integrator::*;
use super::synthesis_generator::*;
use super::target_ontology::*;

/// Валидатор синтеза - проверяет качество интеграции
#[derive(Debug, Clone)]
pub struct SynthesisValidator {
    /// Количество шагов симуляции
    pub simulation_steps: usize,
    
    /// Минимальная стабильность
    pub min_stability: f32,
    
    /// Максимальный индекс противоречия
    pub max_contradiction: f32,
    
    /// Минимальное улучшение стабильности
    pub min_stability_improvement: f32,
}

impl Default for SynthesisValidator {
    fn default() -> Self {
        Self {
            simulation_steps: 50,
            min_stability: 0.5,
            max_contradiction: 0.6,
            min_stability_improvement: 0.1,
        }
    }
}

impl SynthesisValidator {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Валидация синтеза
    pub fn validate(
        &self,
        machine: &MachineState,
        before: &MachineMetrics,
        after: &MachineMetrics,
    ) -> ValidationResult {
        // Проверка стабильности
        if after.stability_ratio < self.min_stability {
            return ValidationResult {
                valid: false,
                reason: Some(format!(
                    "Stability too low: {:.2} < {:.2}",
                    after.stability_ratio, self.min_stability
                )),
                metrics_after: after.clone(),
            };
        }
        
        // Проверка противоречия
        if after.contradiction_index > self.max_contradiction {
            return ValidationResult {
                valid: false,
                reason: Some(format!(
                    "Contradiction too high: {:.2} > {:.2}",
                    after.contradiction_index, self.max_contradiction
                )),
                metrics_after: after.clone(),
            };
        }
        
        // Проверка улучшения
        let improvement = after.stability_ratio - before.stability_ratio;
        if improvement < self.min_stability_improvement {
            return ValidationResult {
                valid: false,
                reason: Some(format!(
                    "Insufficient improvement: {:.2} < {:.2}",
                    improvement, self.min_stability_improvement
                )),
                metrics_after: after.clone(),
            };
        }
        
        ValidationResult {
            valid: true,
            reason: None,
            metrics_after: after.clone(),
        }
    }
    
    /// Симуляция после синтеза
    pub fn simulate(
        &self,
        machine: &mut MachineState,
        steps: Option<usize>,
    ) -> SimulationResult {
        let steps = steps.unwrap_or(self.simulation_steps);
        
        let mut stability_history = Vec::new();
        let mut contradiction_history = Vec::new();
        
        let initial_stability = machine.metrics.stability_ratio;
        let initial_contradiction = machine.metrics.contradiction_index;
        
        for step in 0..steps {
            // Имитация стабилизации
            let progress = step as f32 / steps as f32;
            let stability = initial_stability + (1.0 - initial_stability) * progress * 0.5;
            let contradiction = initial_contradiction * (1.0 - progress * 0.5);
            
            stability_history.push(stability);
            contradiction_history.push(contradiction);
            
            // Обновляем метрики машины
            machine.metrics.stability_ratio = stability;
            machine.metrics.contradiction_index = contradiction;
        }
        
        SimulationResult {
            steps_completed: steps,
            final_stability: machine.metrics.stability_ratio,
            final_contradiction: machine.metrics.contradiction_index,
            stability_history,
            contradiction_history,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub reason: Option<String>,
    pub metrics_after: MachineMetrics,
}

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub steps_completed: usize,
    pub final_stability: f32,
    pub final_contradiction: f32,
    pub stability_history: Vec<f32>,
    pub contradiction_history: Vec<f32>,
}
'''

with open('grammalang-core/src/ontology/synthesis_validator.rs', 'w', encoding='utf-8') as f:
    f.write(validator)
print("synthesis_validator.rs created")

# 2. synthesis_rollback.rs
rollback = '''use super::synthesis_integrator::*;

/// Механизм отката синтеза
#[derive(Debug, Clone)]
pub struct SynthesisRollback {
    /// История снапшотов машины
    pub history: Vec<MachineSnapshot>,
}

#[derive(Debug, Clone)]
pub struct MachineSnapshot {
    pub nodes: Vec<MachineNode>,
    pub edges: Vec<Edge>,
    pub metrics: MachineMetrics,
}

impl SynthesisRollback {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    
    /// Создание снапшота
    pub fn snapshot(&mut self, machine: &MachineState) {
        self.history.push(MachineSnapshot {
            nodes: machine.nodes.clone(),
            edges: machine.edges.clone(),
            metrics: machine.metrics.clone(),
        });
    }
    
    /// Откат к последнему снапшоту
    pub fn rollback(&mut self, machine: &mut MachineState) -> Result<(), RollbackError> {
        if let Some(snapshot) = self.history.pop() {
            machine.nodes = snapshot.nodes;
            machine.edges = snapshot.edges;
            machine.metrics = snapshot.metrics;
            Ok(())
        } else {
            Err(RollbackError::NoSnapshot)
        }
    }
    
    /// Откат к конкретному снапшоту
    pub fn rollback_to(&mut self, machine: &mut MachineState, index: usize) -> Result<(), RollbackError> {
        if index < self.history.len() {
            let snapshot = &self.history[index];
            machine.nodes = snapshot.nodes.clone();
            machine.edges = snapshot.edges.clone();
            machine.metrics = snapshot.metrics.clone();
            
            // Удаляем снапшоты после index
            self.history.truncate(index);
            
            Ok(())
        } else {
            Err(RollbackError::InvalidIndex(index))
        }
    }
    
    /// Очистка истории
    pub fn clear(&mut self) {
        self.history.clear();
    }
    
    /// Количество снапшотов
    pub fn len(&self) -> usize {
        self.history.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }
}

#[derive(Debug, Clone)]
pub enum RollbackError {
    NoSnapshot,
    InvalidIndex(usize),
}
'''

with open('grammalang-core/src/ontology/synthesis_rollback.rs', 'w', encoding='utf-8') as f:
    f.write(rollback)
print("synthesis_rollback.rs created")

# 3. Обновляем mod.rs
mod_content = '''// src/ontology/mod.rs
pub mod engine;
pub mod target_ontology;
pub mod contradiction;
pub mod synthesis_detector;
pub mod synthesis_strategy_selector;
pub mod synthesis_generator;
pub mod synthesis_generator_llm;
pub mod synthesis_generator_diffusion;
pub mod synthesis_generator_evolutionary;
pub mod synthesis_integrator;
pub mod axis_proposer;
pub mod synthesis_validator;
pub mod synthesis_rollback;

pub use engine::*;
pub use target_ontology::*;
pub use contradiction::*;
pub use synthesis_detector::*;
pub use synthesis_strategy_selector::*;
pub use synthesis_generator::*;
pub use synthesis_generator_llm::*;
pub use synthesis_generator_diffusion::*;
pub use synthesis_generator_evolutionary::*;
pub use synthesis_integrator::*;
pub use axis_proposer::*;
pub use synthesis_validator::*;
pub use synthesis_rollback::*;
'''

with open('grammalang-core/src/ontology/mod.rs', 'w', encoding='utf-8') as f:
    f.write(mod_content)
print("mod.rs updated")

# 4. Тесты Фазы 4
phase4_tests = '''use grammalang_core::ontology::*;

#[test]
fn test_validator_success() {
    let validator = SynthesisValidator::new();
    
    let before = MachineMetrics {
        stability_ratio: 0.5,
        contradiction_index: 0.7,
        node_count: 2,
        edge_count: 0,
    };
    
    let after = MachineMetrics {
        stability_ratio: 0.8,
        contradiction_index: 0.3,
        node_count: 3,
        edge_count: 2,
    };
    
    let result = validator.validate(
        &MachineState::new(),
        &before,
        &after,
    );
    
    assert!(result.valid);
    println!("Validation passed: stability improved to {:.2}", after.stability_ratio);
}

#[test]
fn test_validator_low_stability() {
    let validator = SynthesisValidator::new();
    
    let before = MachineMetrics {
        stability_ratio: 0.4,
        contradiction_index: 0.5,
        node_count: 2,
        edge_count: 0,
    };
    
    let after = MachineMetrics {
        stability_ratio: 0.3,
        contradiction_index: 0.5,
        node_count: 3,
        edge_count: 2,
    };
    
    let result = validator.validate(
        &MachineState::new(),
        &before,
        &after,
    );
    
    assert!(!result.valid);
    println!("Low stability rejected: {}", result.reason.unwrap());
}

#[test]
fn test_simulation() {
    let validator = SynthesisValidator::new();
    let mut machine = MachineState::new();
    
    machine.add_node("a", vec![]);
    machine.add_node("b", vec![]);
    
    let result = validator.simulate(&mut machine, Some(20));
    
    assert_eq!(result.steps_completed, 20);
    assert!(result.final_stability > 0.0);
    assert!(result.stability_history.len() == 20);
    
    println!("Simulation: {} steps, final stability: {:.2}", 
             result.steps_completed, result.final_stability);
}

#[test]
fn test_rollback() {
    let mut machine = MachineState::new();
    let mut rollback = SynthesisRollback::new();
    
    // Снапшот до изменений
    rollback.snapshot(&machine);
    
    // Добавляем узлы
    machine.add_node("a", vec![]);
    machine.add_node("b", vec![]);
    machine.add_node("c", vec![]);
    
    assert_eq!(machine.nodes.len(), 3);
    
    // Откат
    rollback.rollback(&mut machine).unwrap();
    
    assert_eq!(machine.nodes.len(), 0);
    println!("Rollback successful: restored to {} nodes", machine.nodes.len());
}

#[test]
fn test_rollback_multiple() {
    let mut machine = MachineState::new();
    let mut rollback = SynthesisRollback::new();
    
    // Снапшот 0
    rollback.snapshot(&machine);
    
    machine.add_node("a", vec![]);
    rollback.snapshot(&machine);
    
    machine.add_node("b", vec![]);
    rollback.snapshot(&machine);
    
    machine.add_node("c", vec![]);
    
    assert_eq!(machine.nodes.len(), 3);
    assert_eq!(rollback.len(), 3);
    
    // Откат к снапшоту 1 (после добавления a)
    rollback.rollback_to(&mut machine, 1).unwrap();
    
    assert_eq!(machine.nodes.len(), 1);
    println!("Rollback to snapshot 1: {} nodes", machine.nodes.len());
}
'''

with open('grammalang-core/tests/ontology/phase4_tests.rs', 'w', encoding='utf-8') as f:
    f.write(phase4_tests)
print("phase4_tests.rs created")

# 5. Обновляем тестовый mod.rs
test_mod = '''pub mod target_ontology_tests;
pub mod contradiction_tests;
pub mod synthesis_detector_tests;
pub mod phase1_tests;
pub mod phase2_tests;
pub mod phase3_tests;
pub mod phase4_tests;
pub mod integration_test;
'''

with open('grammalang-core/tests/ontology/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll Phase 4 files created!")
