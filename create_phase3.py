# create_phase3.py
import os

os.makedirs('grammalang-core/src/ontology', exist_ok=True)
os.makedirs('grammalang-core/tests/ontology', exist_ok=True)

# 1. synthesis_integrator.rs
integrator = '''use super::synthesis_generator::*;
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
'''

with open('grammalang-core/src/ontology/synthesis_integrator.rs', 'w', encoding='utf-8') as f:
    f.write(integrator)
print("synthesis_integrator.rs created")

# 2. axis_proposer.rs
axis_proposer = '''use super::synthesis_integrator::*;
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
'''

with open('grammalang-core/src/ontology/axis_proposer.rs', 'w', encoding='utf-8') as f:
    f.write(axis_proposer)
print("axis_proposer.rs created")

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
'''

with open('grammalang-core/src/ontology/mod.rs', 'w', encoding='utf-8') as f:
    f.write(mod_content)
print("mod.rs updated")

# 4. Тесты Фазы 3
phase3_tests = '''use grammalang_core::ontology::*;

#[test]
fn test_machine_state() {
    let mut machine = MachineState::new();
    
    let a = machine.add_node("freedom", vec!["abstract".to_string()]);
    let b = machine.add_node("security", vec!["concrete".to_string()]);
    
    assert_eq!(machine.nodes.len(), 2);
    assert_eq!(machine.metrics.node_count, 2);
    
    println!("Machine: {} nodes, stability: {:.2}", 
             machine.metrics.node_count, 
             machine.metrics.stability_ratio);
}

#[test]
fn test_synthesis_integration() {
    let mut machine = MachineState::new();
    
    let a = machine.add_node("freedom", vec![]);
    let b = machine.add_node("security", vec![]);
    
    let integrator = SynthesisIntegrator::new();
    
    let synthesis = SynthesisResult {
        name: "responsible_freedom".to_string(),
        description: "Synthesis of freedom and security".to_string(),
        properties: vec!["balanced".to_string()],
        strategy: SynthesisStrategy::Hegelian,
        confidence: 0.8,
    };
    
    let result = integrator.integrate(
        &mut machine,
        &synthesis,
        &[a, b],
    ).unwrap();
    
    assert_eq!(machine.nodes.len(), 3);
    assert_eq!(result.edges_created, 2);
    
    println!("Integrated: {} ({} edges)", result.node_id, result.edges_created);
}

#[test]
fn test_axis_proposal() {
    let mut machine = MachineState::new();
    
    let a = machine.add_node("good", vec![]);
    let b = machine.add_node("evil", vec![]);
    
    let integrator = SynthesisIntegrator::new();
    let synthesis = SynthesisResult {
        name: "moral_axis".to_string(),
        description: "Moral axis".to_string(),
        properties: vec!["axis_candidate".to_string()],
        strategy: SynthesisStrategy::Hegelian,
        confidence: 0.9,
    };
    
    let result = integrator.integrate(&mut machine, &synthesis, &[a, b]).unwrap();
    
    let proposer = AxisProposer::new();
    let candidate = machine.nodes.iter()
        .find(|n| n.id == result.node_id)
        .unwrap();
    
    if let Some(proposal) = proposer.propose(&machine, candidate) {
        println!("Axis proposed: {} (gain: {:.2})", 
                 proposal.axis_name, proposal.expected_gain);
        assert!(!proposal.axis_name.is_empty());
    }
}

#[test]
fn test_low_confidence_rejection() {
    let mut machine = MachineState::new();
    let a = machine.add_node("a", vec![]);
    let b = machine.add_node("b", vec![]);
    
    let integrator = SynthesisIntegrator::new();
    let synthesis = SynthesisResult {
        name: "weak_synthesis".to_string(),
        description: "Low confidence".to_string(),
        properties: vec![],
        strategy: SynthesisStrategy::Hegelian,
        confidence: 0.3,
    };
    
    let result = integrator.integrate(&mut machine, &synthesis, &[a, b]);
    
    assert!(result.is_err());
    println!("Low confidence rejected correctly");
}
'''

with open('grammalang-core/tests/ontology/phase3_tests.rs', 'w', encoding='utf-8') as f:
    f.write(phase3_tests)
print("phase3_tests.rs created")

# 5. Обновляем тестовый mod.rs
test_mod = '''pub mod target_ontology_tests;
pub mod contradiction_tests;
pub mod synthesis_detector_tests;
pub mod phase1_tests;
pub mod phase2_tests;
pub mod phase3_tests;
pub mod integration_test;
'''

with open('grammalang-core/tests/ontology/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll Phase 3 files created!")
