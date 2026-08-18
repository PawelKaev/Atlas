use super::target_ontology::*;
use super::synthesis_detector::*;

/// Селектор стратегии синтеза
pub trait SynthesisStrategySelector {
    fn select(&self, candidate: &SynthesisCandidate) -> SynthesisStrategy;
}

/// Эвристический селектор
pub struct HeuristicSelector;

impl SynthesisStrategySelector for HeuristicSelector {
    fn select(&self, candidate: &SynthesisCandidate) -> SynthesisStrategy {
        candidate.strategy_hint.clone()
    }
}

/// Контекстный селектор
pub struct ContextualSelector {
    pub hegelian_weight: f32,
    pub plotinian_weight: f32,
    pub pragmatic_weight: f32,
}

impl Default for ContextualSelector {
    fn default() -> Self {
        Self {
            hegelian_weight: 0.4,
            plotinian_weight: 0.3,
            pragmatic_weight: 0.3,
        }
    }
}

impl SynthesisStrategySelector for ContextualSelector {
    fn select(&self, candidate: &SynthesisCandidate) -> SynthesisStrategy {
        let base = candidate.strategy_hint.clone();
        
        match base {
            SynthesisStrategy::Hegelian if self.hegelian_weight > 0.5 => {
                SynthesisStrategy::Hegelian
            }
            SynthesisStrategy::Plotinian if self.plotinian_weight > 0.5 => {
                SynthesisStrategy::Plotinian
            }
            SynthesisStrategy::Pragmatic if self.pragmatic_weight > 0.5 => {
                SynthesisStrategy::Pragmatic
            }
            _ => {
                if self.hegelian_weight >= self.plotinian_weight && 
                   self.hegelian_weight >= self.pragmatic_weight {
                    SynthesisStrategy::Hegelian
                } else if self.plotinian_weight >= self.pragmatic_weight {
                    SynthesisStrategy::Plotinian
                } else {
                    SynthesisStrategy::Pragmatic
                }
            }
        }
    }
}

/// Адаптивный селектор на основе истории
pub struct AdaptiveSelector {
    success_history: Vec<StrategySuccess>,
}

#[derive(Debug, Clone)]
pub struct StrategySuccess {
    pub strategy: SynthesisStrategy,
    pub success: bool,
    pub stability_gain: f32,
}

impl AdaptiveSelector {
    pub fn new() -> Self {
        Self {
            success_history: Vec::new(),
        }
    }
    
    pub fn record_result(&mut self, strategy: SynthesisStrategy, success: bool, gain: f32) {
        self.success_history.push(StrategySuccess {
            strategy,
            success,
            stability_gain: gain,
        });
    }
    
    fn calculate_success_rate(&self, strategy: &SynthesisStrategy) -> f32 {
        let relevant: Vec<&StrategySuccess> = self.success_history
            .iter()
            .filter(|s| std::mem::discriminant(&s.strategy) == std::mem::discriminant(strategy))
            .collect();
        
        if relevant.is_empty() {
            return 0.5;
        }
        
        let successes = relevant.iter().filter(|s| s.success).count();
        successes as f32 / relevant.len() as f32
    }
}

impl SynthesisStrategySelector for AdaptiveSelector {
    fn select(&self, candidate: &SynthesisCandidate) -> SynthesisStrategy {
        let strategies = vec![
            SynthesisStrategy::Hegelian,
            SynthesisStrategy::Plotinian,
            SynthesisStrategy::Pragmatic,
        ];
        
        let mut best_strategy = candidate.strategy_hint.clone();
        let mut best_rate = self.calculate_success_rate(&best_strategy);
        
        for strategy in strategies {
            let rate = self.calculate_success_rate(&strategy);
            if rate > best_rate {
                best_rate = rate;
                best_strategy = strategy;
            }
        }
        
        best_strategy
    }
}
