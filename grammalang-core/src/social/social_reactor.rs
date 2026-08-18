use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Социальный реактор - обработка коллективных противоречий
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialReactor {
    pub collective_threshold: f32,
    pub active_contradictions: Vec<SocialContradiction>,
    pub reaction_history: Vec<Reaction>,
    pub resolution_strategies: HashMap<String, ResolutionStrategy>,
    pub metrics: ReactorMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialContradiction {
    pub source_a: String,
    pub source_b: String,
    pub severity: f32,
    pub kind: SocialContradictionKind,
    pub context: Option<ContradictionContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialContradictionKind {
    KnowledgeConflict,
    MachineConflict,
    GenealogyConflict,
    TemporalConflict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContradictionContext {
    pub domain: String,
    pub participants: Vec<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    /// Приоритет первому источнику
    PreferFirst,
    /// Приоритет второму источнику
    PreferSecond,
    /// Слияние обоих
    Merge,
    /// Создание нового
    Synthesize,
    /// Отклонить оба
    RejectBoth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub action: ReactionAction,
    pub result: ReactionResult,
    pub strategy_used: ResolutionStrategy,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReactionAction {
    Merge,
    Resolve,
    Reject,
    Defer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReactionResult {
    Success,
    Failure,
    Pending,
    PartialSuccess,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactorMetrics {
    pub total_contradictions: usize,
    pub resolved: usize,
    pub failed: usize,
    pub pending: usize,
    pub avg_resolution_time: f32,
}

impl SocialReactor {
    pub fn new() -> Self {
        Self {
            collective_threshold: 0.6,
            active_contradictions: Vec::new(),
            reaction_history: Vec::new(),
            resolution_strategies: HashMap::new(),
            metrics: ReactorMetrics::default(),
        }
    }
    
    /// Добавление противоречия
    pub fn add_contradiction(&mut self, contradiction: SocialContradiction) {
        if contradiction.severity >= self.collective_threshold {
            self.active_contradictions.push(contradiction);
            self.metrics.total_contradictions += 1;
        }
    }
    
    /// Добавление стратегии разрешения
    pub fn add_strategy(&mut self, domain: &str, strategy: ResolutionStrategy) {
        self.resolution_strategies.insert(domain.to_string(), strategy);
    }
    
    /// Обработка всех активных противоречий
    pub fn process(&mut self) -> usize {
        let count = self.active_contradictions.len();
        
        while let Some(contradiction) = self.active_contradictions.pop() {
            let strategy = self.select_strategy(&contradiction);
            let reaction = self.resolve(&contradiction, &strategy);
            self.reaction_history.push(reaction);
        }
        
        count
    }
    
    /// Выбор стратегии на основе контекста
    fn select_strategy(&self, contradiction: &SocialContradiction) -> ResolutionStrategy {
        // Проверяем доменную стратегию
        if let Some(context) = &contradiction.context {
            if let Some(strategy) = self.resolution_strategies.get(&context.domain) {
                return strategy.clone();
            }
        }
        
        // Стратегия по умолчанию на основе типа
        match contradiction.kind {
            SocialContradictionKind::KnowledgeConflict => ResolutionStrategy::Merge,
            SocialContradictionKind::MachineConflict => ResolutionStrategy::Synthesize,
            SocialContradictionKind::GenealogyConflict => ResolutionStrategy::PreferFirst,
            SocialContradictionKind::TemporalConflict => ResolutionStrategy::PreferSecond,
        }
    }
    
    /// Разрешение противоречия
    fn resolve(
        &mut self,
        contradiction: &SocialContradiction,
        strategy: &ResolutionStrategy,
    ) -> Reaction {
        match strategy {
            ResolutionStrategy::Merge => {
                self.metrics.resolved += 1;
                Reaction {
                    action: ReactionAction::Merge,
                    result: ReactionResult::Success,
                    strategy_used: strategy.clone(),
                    description: format!("Merged {} and {}", 
                        contradiction.source_a, contradiction.source_b),
                }
            }
            ResolutionStrategy::Synthesize => {
                self.metrics.resolved += 1;
                Reaction {
                    action: ReactionAction::Resolve,
                    result: ReactionResult::Success,
                    strategy_used: strategy.clone(),
                    description: format!("Synthesized from {} and {}", 
                        contradiction.source_a, contradiction.source_b),
                }
            }
            ResolutionStrategy::RejectBoth => {
                self.metrics.failed += 1;
                Reaction {
                    action: ReactionAction::Reject,
                    result: ReactionResult::Failure,
                    strategy_used: strategy.clone(),
                    description: "Rejected both sources".to_string(),
                }
            }
            _ => {
                self.metrics.resolved += 1;
                Reaction {
                    action: ReactionAction::Resolve,
                    result: ReactionResult::Success,
                    strategy_used: strategy.clone(),
                    description: "Resolved with preference".to_string(),
                }
            }
        }
    }
    
    /// Обработка одного противоречия
    pub fn process_one(&mut self, contradiction: SocialContradiction) -> Reaction {
        let strategy = self.select_strategy(&contradiction);
        let reaction = self.resolve(&contradiction, &strategy);
        self.reaction_history.push(reaction.clone());
        reaction
    }
    
    /// Получение метрик
    pub fn get_metrics(&self) -> ReactorMetrics {
        self.metrics.clone()
    }
    
    /// Количество активных
    pub fn active_count(&self) -> usize {
        self.active_contradictions.len()
    }
    
    /// Количество обработанных
    pub fn processed_count(&self) -> usize {
        self.reaction_history.len()
    }
    
    /// Очистка истории
    pub fn clear_history(&mut self) {
        self.reaction_history.clear();
    }
}

/// Распределенный реактор для нескольких машин
pub struct DistributedReactor {
    pub reactors: HashMap<String, SocialReactor>,
}

impl DistributedReactor {
    pub fn new() -> Self {
        Self {
            reactors: HashMap::new(),
        }
    }
    
    /// Добавление реактора для машины
    pub fn add_reactor(&mut self, machine_id: &str, reactor: SocialReactor) {
        self.reactors.insert(machine_id.to_string(), reactor);
    }
    
    /// Обработка противоречия на конкретной машине
    pub fn process_on(&mut self, machine_id: &str, contradiction: SocialContradiction) -> Option<Reaction> {
        self.reactors.get_mut(machine_id)
            .map(|reactor| reactor.process_one(contradiction))
    }
    
    /// Общая статистика
    pub fn total_stats(&self) -> HashMap<String, ReactorMetrics> {
        self.reactors.iter()
            .map(|(id, reactor)| (id.clone(), reactor.get_metrics()))
            .collect()
    }
    
    /// Количество реакторов
    pub fn reactor_count(&self) -> usize {
        self.reactors.len()
    }
}
