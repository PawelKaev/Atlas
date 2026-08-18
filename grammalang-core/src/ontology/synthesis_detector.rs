use serde::{Serialize, Deserialize};
use super::target_ontology::*;
use super::contradiction::*;
use std::collections::HashMap;

/// Детектор синтеза - обнаруживает противоречия, готовые к синтезу
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisDetector {
    /// Порог противоречия (по умолчанию 0.6)
    pub threshold_contradiction: f32,
    
    /// Порог падения стабильности за такт (по умолчанию 0.05)
    pub threshold_stability_drop: f32,
    
    /// Минимальная длительность напряжения в тактах (по умолчанию 3)
    pub min_tension_duration: usize,
    
    /// Минимальное количество узлов-посредников
    pub min_mediators: usize,
    
    /// Включить обнаружение опосредованных противоречий
    pub detect_mediated: bool,
    
    /// Включить обнаружение рекурсивных противоречий
    pub detect_recursive: bool,
}

impl Default for SynthesisDetector {
    fn default() -> Self {
        Self {
            threshold_contradiction: 0.6,
            threshold_stability_drop: 0.05,
            min_tension_duration: 3,
            min_mediators: 1,
            detect_mediated: true,
            detect_recursive: true,
        }
    }
}

impl SynthesisDetector {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Обнаружение противоречий, готовых к синтезу
    pub fn detect(&self, contradictions: &[Contradiction]) -> Vec<SynthesisCandidate> {
        let mut candidates = Vec::new();
        
        for contradiction in contradictions {
            // Проверяем базовую готовность
            if !contradiction.is_ready_for_synthesis(self.threshold_contradiction) {
                continue;
            }
            
            // Проверяем длительность напряжения
            if contradiction.severity_history.len() < self.min_tension_duration {
                continue;
            }
            
            // Определяем тип противоречия
            let contradiction_type = self.determine_contradiction_type(contradiction);
            
            // Выбираем стратегию синтеза
            let strategy_hint = self.select_strategy(contradiction, &contradiction_type);
            
            // Создаем кандидата
            let candidate = SynthesisCandidate {
                source_nodes: vec![contradiction.node_a.clone(), contradiction.node_b.clone()],
                contradiction_type,
                strategy_hint,
                metrics_before: MetricsSnapshot {
                    stability_ratio: self.calculate_stability(contradiction),
                    contradiction_index: contradiction.contradiction_index,
                },
                mediators: self.find_mediators(contradiction),
                genealogy: contradiction.genealogy.clone(),
                tension_duration: contradiction.severity_history.len(),
            };
            
            candidates.push(candidate);
        }
        
        // Сортируем по индексу противоречия (от высокого к низкому)
        candidates.sort_by(|a, b| {
            b.metrics_before.contradiction_index
                .partial_cmp(&a.metrics_before.contradiction_index)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        candidates
    }
    
    /// Определение типа противоречия
    fn determine_contradiction_type(&self, c: &Contradiction) -> ContradictionType {
        match c.kind {
            ContradictionKind::Recursive if self.detect_recursive => {
                ContradictionType::Recursive
            }
            ContradictionKind::Structural if self.detect_mediated => {
                if !c.resolution_candidates.is_empty() {
                    ContradictionType::Mediated {
                        mediator: c.resolution_candidates[0].clone(),
                    }
                } else {
                    ContradictionType::Direct
                }
            }
            _ => ContradictionType::Direct,
        }
    }
    
    /// Выбор стратегии синтеза
    fn select_strategy(
        &self,
        _contradiction: &Contradiction,
        contradiction_type: &ContradictionType,
    ) -> SynthesisStrategy {
        match contradiction_type {
            ContradictionType::Direct => SynthesisStrategy::Hegelian,
            ContradictionType::Mediated { .. } => SynthesisStrategy::Pragmatic,
            ContradictionType::Recursive => SynthesisStrategy::Plotinian,
        }
    }
    
    /// Расчет текущей стабильности
    fn calculate_stability(&self, c: &Contradiction) -> f32 {
        if c.severity_history.is_empty() {
            return 1.0;
        }
        c.severity_history.last().unwrap().stability
    }
    
    /// Поиск узлов-посредников
    fn find_mediators(&self, c: &Contradiction) -> Vec<NodeId> {
        if !self.detect_mediated {
            return Vec::new();
        }
        c.resolution_candidates
            .iter()
            .take(self.min_mediators)
            .cloned()
            .collect()
    }
    
    /// Расширенное обнаружение с анализом трендов
    pub fn detect_with_trends(&self, contradictions: &[Contradiction]) -> Vec<SynthesisCandidate> {
        let mut candidates = self.detect(contradictions);
        
        for candidate in &mut candidates {
            if let Some(contradiction) = contradictions.iter().find(|c| {
                c.node_a == candidate.source_nodes[0] && 
                c.node_b == candidate.source_nodes[1]
            }) {
                candidate.metrics_before.stability_ratio = 
                    self.calculate_stability_trend(contradiction);
            }
        }
        
        candidates
    }
    
    /// Расчет тренда стабильности
    fn calculate_stability_trend(&self, c: &Contradiction) -> f32 {
        if c.severity_history.len() < 2 {
            return 0.0;
        }
        
        let recent: Vec<&SeverityRecord> = 
            c.severity_history.iter().rev().take(5).collect();
        
        if recent.len() < 2 {
            return 0.0;
        }
        
        let mut total_drop = 0.0;
        let mut count = 0;
        
        for i in 1..recent.len() {
            let drop = recent[i].stability - recent[i-1].stability;
            if drop < 0.0 {
                total_drop += drop.abs();
                count += 1;
            }
        }
        
        if count > 0 {
            total_drop / count as f32
        } else {
            0.0
        }
    }
}

/// Кандидат на синтез
#[derive(Debug, Clone)]
pub struct SynthesisCandidate {
    pub source_nodes: Vec<NodeId>,
    pub contradiction_type: ContradictionType,
    pub strategy_hint: SynthesisStrategy,
    pub metrics_before: MetricsSnapshot,
    pub mediators: Vec<NodeId>,
    pub genealogy: Vec<String>,
    pub tension_duration: usize,
}

/// Снимок метрик машины
#[derive(Debug, Clone, Default)]
pub struct MetricsSnapshot {
    pub stability_ratio: f32,
    pub contradiction_index: f32,
}

/// Расширенный детектор с контекстным анализом
pub struct ContextAwareDetector {
    base_detector: SynthesisDetector,
    context_weights: HashMap<String, f32>,
}

impl ContextAwareDetector {
    pub fn new(context_weights: HashMap<String, f32>) -> Self {
        Self {
            base_detector: SynthesisDetector::new(),
            context_weights,
        }
    }
    
    pub fn detect_with_context(&self, contradictions: &[Contradiction]) -> Vec<SynthesisCandidate> {
        let mut candidates = self.base_detector.detect(contradictions);
        
        for candidate in &mut candidates {
            if let Some(weight) = self.context_weights.get("hegelian_bias") {
                if *weight > 0.7 {
                    candidate.strategy_hint = SynthesisStrategy::Hegelian;
                }
            }
            
            if let Some(weight) = self.context_weights.get("plotinian_bias") {
                if *weight > 0.7 {
                    candidate.strategy_hint = SynthesisStrategy::Plotinian;
                }
            }
        }
        
        candidates
    }
}
