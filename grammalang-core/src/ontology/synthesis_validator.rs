use super::synthesis_integrator::*;
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
