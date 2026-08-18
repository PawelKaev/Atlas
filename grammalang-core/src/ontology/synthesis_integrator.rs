use super::synthesis_generator::*;
use super::target_ontology::*;
use std::collections::HashMap;

/// Интегратор синтеза - добавляет новое понятие в машину
#[derive(Debug, Clone)]
pub struct SynthesisIntegrator {
    /// Автоматически связывать синтез с родителями
    pub auto_connect: bool,
    
    /// Сохранять генеалогию
    pub preserve_genealogy: bool,
    
    /// Порог уверенности для интеграции
    pub confidence_threshold: f32,
}

impl Default for SynthesisIntegrator {
    fn default() -> Self {
        Self {
            auto_connect: true,
            preserve_genealogy: true,
            confidence_threshold: 0.5,
        }
    }
}

impl SynthesisIntegrator {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Интеграция синтеза в машину
    pub fn integrate(
        &self,
        machine: &mut MachineState,
        synthesis: &SynthesisResult,
        parents: &[NodeId],
    ) -> Result<IntegrationResult, IntegrationError> {
        // Проверяем уверенность
        if synthesis.confidence < self.confidence_threshold {
            return Err(IntegrationError::LowConfidence(synthesis.confidence));
        }
        
        // Создаем новый узел
        let node_id = format!("node_{}", machine.nodes.len());
        let node = MachineNode {
            id: node_id.clone(),
            name: synthesis.name.clone(),
            properties: synthesis.properties.clone(),
            genealogy: if self.preserve_genealogy {
                parents.to_vec()
            } else {
                Vec::new()
            },
            confidence: synthesis.confidence,
        };
        
        // Добавляем узел
        machine.nodes.push(node);
        
        // Автоматически связываем с родителями
        if self.auto_connect {
            for parent in parents {
                machine.edges.push(Edge {
                    from: parent.clone(),
                    to: node_id.clone(),
                    edge_type: EdgeType::Synthesizes,
                });
            }
        }
        
        // Обновляем метрики
        machine.recalculate_metrics();
        
        Ok(IntegrationResult {
            node_id,
            edges_created: parents.len(),
            metrics_after: machine.metrics.clone(),
        })
    }
}

/// Состояние машины
#[derive(Debug, Clone)]
pub struct MachineState {
    pub nodes: Vec<MachineNode>,
    pub edges: Vec<Edge>,
    pub metrics: MachineMetrics,
}

#[derive(Debug, Clone)]
pub struct MachineNode {
    pub id: NodeId,
    pub name: String,
    pub properties: Vec<String>,
    pub genealogy: Vec<NodeId>,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone)]
pub enum EdgeType {
    Synthesizes,
    Contradicts,
    Requires,
    Stabilizes,
}

#[derive(Debug, Clone, Default)]
pub struct MachineMetrics {
    pub stability_ratio: f32,
    pub contradiction_index: f32,
    pub node_count: usize,
    pub edge_count: usize,
}

impl MachineState {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            metrics: MachineMetrics::default(),
        }
    }
    
    pub fn add_node(&mut self, name: &str, properties: Vec<String>) -> NodeId {
        let id = format!("node_{}", self.nodes.len());
        self.nodes.push(MachineNode {
            id: id.clone(),
            name: name.to_string(),
            properties,
            genealogy: Vec::new(),
            confidence: 1.0,
        });
        self.recalculate_metrics();
        id
    }
    
    pub fn recalculate_metrics(&mut self) {
        self.metrics.node_count = self.nodes.len();
        self.metrics.edge_count = self.edges.len();
        
        // Простой расчет стабильности
        if self.nodes.is_empty() {
            self.metrics.stability_ratio = 1.0;
            self.metrics.contradiction_index = 0.0;
        } else {
            let avg_confidence: f32 = self.nodes.iter()
                .map(|n| n.confidence)
                .sum::<f32>() / self.nodes.len() as f32;
            
            self.metrics.stability_ratio = avg_confidence;
            self.metrics.contradiction_index = 1.0 - avg_confidence;
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntegrationResult {
    pub node_id: NodeId,
    pub edges_created: usize,
    pub metrics_after: MachineMetrics,
}

#[derive(Debug, Clone)]
pub enum IntegrationError {
    LowConfidence(f32),
    NodeNotFound(NodeId),
    InvalidParents,
}
