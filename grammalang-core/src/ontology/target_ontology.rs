// src/ontology/target_ontology.rs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Оператор TargetOntology - механизм синтеза новых понятий
/// из противоречащих узлов машины
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetOntology {
    /// Узлы-источники противоречия
    pub source_nodes: Vec<NodeId>,
    
    /// Тип противоречия
    pub contradiction_type: ContradictionType,
    
    /// Стратегия синтеза
    pub synthesis_strategy: SynthesisStrategy,
    
    /// Целевая ось (если требуется перестройка)
    pub target_axis: Option<AxisSpec>,
    
    /// Метаданные синтеза
    pub metadata: SynthesisMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContradictionType {
    /// Прямое противоречие: A и ¬A
    Direct,
    
    /// Опосредованное: A и B через посредника C
    Mediated { mediator: NodeId },
    
    /// Рекурсивное: A противоречит самому себе
    Recursive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynthesisStrategy {
    /// Гегелевское снятие: тезис + антитезис → синтез
    Hegelian,
    
    /// Плотиновская эманация: из Единого → множественность
    Plotinian,
    
    /// Прагматическая абдукция: поиск наилучшего объяснения
    Pragmatic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxisSpec {
    pub axis_id: AxisId,
    pub axis_name: String,
    pub poles: (NodeId, NodeId),
    pub transformation_type: AxisTransformation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AxisTransformation {
    /// Создание новой оси
    Create,
    
    /// Модификация существующей
    Modify,
    
    /// Удаление устаревшей
    Remove,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisMetadata {
    /// Временная метка синтеза
    pub timestamp: u64,
    
    /// Версия машины до синтеза
    pub version_before: String,
    
    /// Инициатор синтеза (Plato/Achitect/System)
    pub initiator: SynthesisInitiator,
    
    /// Параметры симуляции
    pub simulation_params: SimulationParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynthesisInitiator {
    /// Автоматический режим
    Plato,
    
    /// Ручной режим
    Architect { user_id: String },
    
    /// Системный триггер
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationParams {
    pub steps: usize,
    pub stability_threshold: f32,
    pub contradiction_threshold: f32,
}

// Базовые типы
pub type NodeId = String;
pub type AxisId = String;

impl TargetOntology {
    /// Создание нового оператора TargetOntology
    pub fn new(
        source_nodes: Vec<NodeId>,
        contradiction_type: ContradictionType,
        strategy: SynthesisStrategy,
    ) -> Self {
        Self {
            source_nodes,
            contradiction_type,
            synthesis_strategy: strategy,
            target_axis: None,
            metadata: SynthesisMetadata {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                version_before: "v0.6".to_string(),
                initiator: SynthesisInitiator::System,
                simulation_params: SimulationParams {
                    steps: 50,
                    stability_threshold: 0.5,
                    contradiction_threshold: 0.6,
                },
            },
        }
    }
    
    /// Установка целевой оси
    pub fn with_target_axis(mut self, axis: AxisSpec) -> Self {
        self.target_axis = Some(axis);
        self
    }
    
    /// Проверка готовности к синтезу
    pub fn is_ready(&self) -> bool {
        self.source_nodes.len() >= 2 && 
        self.metadata.simulation_params.steps > 0
    }
}
