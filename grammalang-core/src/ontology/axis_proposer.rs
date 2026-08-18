use super::synthesis_integrator::*;
use super::target_ontology::*;
use std::collections::HashMap;

/// Предложение новой оси на основе синтеза
#[derive(Debug, Clone)]
pub struct AxisProposer {
    /// Порог центральности для предложения
    pub centrality_threshold: f32,
    
    /// Порог прироста стабильности
    pub stability_gain_threshold: f32,
}

impl Default for AxisProposer {
    fn default() -> Self {
        Self {
            centrality_threshold: 0.3,
            stability_gain_threshold: 0.1,
        }
    }
}

impl AxisProposer {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Предложение новой оси
    pub fn propose(
        &self,
        machine: &MachineState,
        candidate: &MachineNode,
    ) -> Option<AxisProposal> {
        // Вычисляем центральность кандидата
        let centrality = self.calculate_centrality(machine, &candidate.id);
        
        // Проверяем порог
        if centrality < self.centrality_threshold {
            return None;
        }
        
        // Вычисляем ожидаемый прирост стабильности
        let stability_gain = self.calculate_stability_gain(machine, candidate);
        
        if stability_gain < self.stability_gain_threshold {
            return None;
        }
        
        Some(AxisProposal {
            axis_name: candidate.name.clone(),
            axis_poles: self.find_poles(machine, &candidate.id),
            expected_gain: stability_gain,
            centrality,
        })
    }
    
    /// Вычисление центральности узла
    fn calculate_centrality(&self, machine: &MachineState, node_id: &str) -> f32 {
        let total_edges = machine.edges.len();
        if total_edges == 0 {
            return 0.0;
        }
        
        let connected: usize = machine.edges.iter()
            .filter(|e| e.from == node_id || e.to == node_id)
            .count();
        
        connected as f32 / total_edges as f32
    }
    
    /// Вычисление прироста стабильности
    fn calculate_stability_gain(&self, machine: &MachineState, candidate: &MachineNode) -> f32 {
        let current = machine.metrics.stability_ratio;
        let potential = (current + candidate.confidence) / 2.0;
        potential - current
    }
    
    /// Поиск полюсов оси
    fn find_poles(&self, machine: &MachineState, node_id: &str) -> (NodeId, NodeId) {
        let mut connected: Vec<&Edge> = machine.edges.iter()
            .filter(|e| e.from == node_id || e.to == node_id)
            .collect();
        
        connected.sort_by(|a, b| {
            let a_type = match a.edge_type {
                EdgeType::Contradicts => 0,
                _ => 1,
            };
            let b_type = match b.edge_type {
                EdgeType::Contradicts => 0,
                _ => 1,
            };
            a_type.cmp(&b_type)
        });
        
        if connected.len() >= 2 {
            let pole_a = if connected[0].from == node_id {
                connected[0].to.clone()
            } else {
                connected[0].from.clone()
            };
            let pole_b = if connected[1].from == node_id {
                connected[1].to.clone()
            } else {
                connected[1].from.clone()
            };
            (pole_a, pole_b)
        } else {
            (node_id.to_string(), node_id.to_string())
        }
    }
}

#[derive(Debug, Clone)]
pub struct AxisProposal {
    pub axis_name: String,
    pub axis_poles: (NodeId, NodeId),
    pub expected_gain: f32,
    pub centrality: f32,
}
