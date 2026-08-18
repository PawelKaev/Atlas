// ide/src/lib.rs
pub mod websocket;

use grammalang_core::ontology::*;
use grammalang_core::reflexive::*;
use serde::{Serialize, Deserialize};

pub struct IdeState {
    pub machine: MachineState,
    pub reflexive_system: ReflexiveSystem,
    pub contradictions: Vec<Contradiction>,
    pub visual_state: VisualState,
    pub realtime_data: RealtimeData,
    pub ws_server: websocket::WsServer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualState {
    pub nodes: Vec<VisualNode>,
    pub edges: Vec<VisualEdge>,
    pub field_intensity: f32,
    pub polyphony_level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualNode {
    pub id: String,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: String,
    pub contradiction_level: f32,
    pub centrality: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualEdge {
    pub from: String,
    pub to: String,
    pub edge_type: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeData {
    pub timestamp: u64,
    pub stability: f32,
    pub contradiction_index: f32,
    pub self_awareness: f32,
    pub pulse_rate: f32,
}

pub struct IdeEngine {
    pub state: IdeState,
}

impl IdeEngine {
    pub fn new() -> Self {
        Self {
            state: IdeState {
                machine: MachineState::new(),
                reflexive_system: ReflexiveSystem::new(),
                contradictions: Vec::new(),
                visual_state: VisualState {
                    nodes: Vec::new(),
                    edges: Vec::new(),
                    field_intensity: 0.0,
                    polyphony_level: 0,
                },
                realtime_data: RealtimeData {
                    timestamp: 0,
                    stability: 1.0,
                    contradiction_index: 0.0,
                    self_awareness: 0.0,
                    pulse_rate: 60.0,
                },
                ws_server: websocket::WsServer::new(),
            },
        }
    }
    
    pub fn add_node(&mut self, label: &str, properties: Vec<String>) {
        let id = self.state.machine.add_node(label, properties);
        
        self.state.visual_state.nodes.push(VisualNode {
            id: id.clone(),
            label: label.to_string(),
            x: 0.0,
            y: 0.0,
            size: 10.0,
            color: "#4CAF50".to_string(),
            contradiction_level: 0.0,
            centrality: 0.0,
        });
    }
    
    pub fn update_realtime(&mut self) {
        self.state.realtime_data.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.state.realtime_data.stability = self.state.machine.metrics.stability_ratio;
        self.state.realtime_data.contradiction_index = self.state.machine.metrics.contradiction_index;
        self.state.realtime_data.self_awareness = self.state.reflexive_system.state.self_awareness;
        self.state.realtime_data.pulse_rate = 60.0 + 
            (self.state.machine.metrics.contradiction_index * 120.0);
        
        // Отправляем через WebSocket
        self.state.ws_server.update_metrics(
            self.state.realtime_data.stability,
            self.state.realtime_data.contradiction_index,
            self.state.realtime_data.self_awareness,
        );
    }
    
    pub fn export_json(&self) -> String {
        format!("nodes: {}", self.state.visual_state.nodes.len())
    }
    
    pub fn summary(&self) -> String {
        format!(
            "IDE State:\n  Nodes: {}\n  Edges: {}\n  Stability: {:.2}\n  Contradiction: {:.2}\n  Self-awareness: {:.2}",
            self.state.visual_state.nodes.len(),
            self.state.visual_state.edges.len(),
            self.state.realtime_data.stability,
            self.state.realtime_data.contradiction_index,
            self.state.realtime_data.self_awareness,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ide_engine_creation() {
        let engine = IdeEngine::new();
        assert_eq!(engine.state.visual_state.nodes.len(), 0);
        println!("IDE Engine created with WebSocket");
    }
    
    #[test]
    fn test_add_node() {
        let mut engine = IdeEngine::new();
        engine.add_node("freedom", vec!["abstract".to_string()]);
        
        assert_eq!(engine.state.visual_state.nodes.len(), 1);
        println!("Node added: {}", engine.state.visual_state.nodes[0].label);
    }
    
    #[test]
    fn test_realtime_update() {
        let mut engine = IdeEngine::new();
        
        engine.add_node("a", vec![]);
        engine.add_node("b", vec![]);
        
        engine.update_realtime();
        
        assert!(engine.state.realtime_data.timestamp > 0);
        println!("Realtime updated via WebSocket");
    }
    
    #[test]
    fn test_export_json() {
        let mut engine = IdeEngine::new();
        engine.add_node("test", vec![]);
        
        let json = engine.export_json();
        assert!(json.contains("nodes: 1"));
        println!("JSON export works");
    }
    
    #[test]
    fn test_summary() {
        let mut engine = IdeEngine::new();
        engine.add_node("node1", vec![]);
        
        let summary = engine.summary();
        assert!(summary.contains("Nodes: 1"));
        println!("{}", summary);
    }
}
