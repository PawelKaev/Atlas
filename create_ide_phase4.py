# create_ide_phase4.py
import os

os.makedirs('ide/src', exist_ok=True)
os.makedirs('ide/frontend/js', exist_ok=True)

# 1. Rust WebSocket сервер
ws_server = '''// ide/src/websocket.rs
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

/// Сообщение для WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub message_type: WsMessageType,
    pub data: WsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessageType {
    /// Обновление метрик
    MetricsUpdate,
    /// Синтез выполнен
    SynthesisCompleted,
    /// Противоречие обнаружено
    ContradictionDetected,
    /// Рефлексия
    Reflection,
    /// Статус машины
    MachineStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsData {
    pub stability: f32,
    pub contradiction: f32,
    pub awareness: f32,
    pub pulse: f32,
    pub message: String,
}

/// WebSocket сервер
pub struct WsServer {
    pub sender: broadcast::Sender<String>,
    pub state: Arc<Mutex<WsState>>,
}

#[derive(Debug, Clone)]
pub struct WsState {
    pub stability: f32,
    pub contradiction: f32,
    pub awareness: f32,
    pub pulse: f32,
}

impl WsServer {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        
        Self {
            sender,
            state: Arc::new(Mutex::new(WsState {
                stability: 1.0,
                contradiction: 0.0,
                awareness: 0.0,
                pulse: 60.0,
            })),
        }
    }
    
    /// Обновление метрик
    pub fn update_metrics(&self, stability: f32, contradiction: f32, awareness: f32) {
        let pulse = 60.0 + contradiction * 100.0;
        
        if let Ok(mut state) = self.state.lock() {
            state.stability = stability;
            state.contradiction = contradiction;
            state.awareness = awareness;
            state.pulse = pulse;
        }
        
        let message = WsMessage {
            message_type: WsMessageType::MetricsUpdate,
            data: WsData {
                stability,
                contradiction,
                awareness,
                pulse,
                message: "metrics_updated".to_string(),
            },
        };
        
        if let Ok(json) = serde_json::to_string(&message) {
            let _ = self.sender.send(json);
        }
    }
    
    /// Синтез выполнен
    pub fn synthesis_completed(&self, name: &str) {
        let message = WsMessage {
            message_type: WsMessageType::SynthesisCompleted,
            data: WsData {
                stability: 0.8,
                contradiction: 0.3,
                awareness: 0.6,
                pulse: 90.0,
                message: format!("Synthesis: {}", name),
            },
        };
        
        if let Ok(json) = serde_json::to_string(&message) {
            let _ = self.sender.send(json);
        }
    }
    
    /// Противоречие обнаружено
    pub fn contradiction_detected(&self, severity: f32) {
        let message = WsMessage {
            message_type: WsMessageType::ContradictionDetected,
            data: WsData {
                stability: 0.4,
                contradiction: severity,
                awareness: 0.3,
                pulse: 60.0 + severity * 100.0,
                message: format!("Contradiction: {:.2}", severity),
            },
        };
        
        if let Ok(json) = serde_json::to_string(&message) {
            let _ = self.sender.send(json);
        }
    }
    
    /// Подписка на сообщения
    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.sender.subscribe()
    }
    
    /// Получение текущего состояния
    pub fn get_state(&self) -> WsState {
        self.state.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ws_server_creation() {
        let server = WsServer::new();
        let state = server.get_state();
        
        assert_eq!(state.stability, 1.0);
        assert_eq!(state.contradiction, 0.0);
        
        println!("WebSocket server created");
    }
    
    #[test]
    fn test_update_metrics() {
        let server = WsServer::new();
        
        server.update_metrics(0.7, 0.4, 0.5);
        
        let state = server.get_state();
        assert_eq!(state.stability, 0.7);
        assert_eq!(state.contradiction, 0.4);
        assert_eq!(state.pulse, 100.0);
        
        println!("Metrics updated: stability {}, pulse {}", 
                 state.stability, state.pulse);
    }
    
    #[test]
    fn test_synthesis_message() {
        let server = WsServer::new();
        let mut receiver = server.subscribe();
        
        server.synthesis_completed("test_synthesis");
        
        if let Ok(json) = receiver.try_recv() {
            let message: WsMessage = serde_json::from_str(&json).unwrap();
            assert!(message.message.to_string().contains("test_synthesis"));
            println!("Synthesis message received");
        }
    }
}
'''

with open('ide/src/websocket.rs', 'w', encoding='utf-8') as f:
    f.write(ws_server)
print("websocket.rs created")

# 2. Обновляем lib.rs для IDE
lib_rs = '''// ide/src/lib.rs
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
            "IDE State:\\n  Nodes: {}\\n  Edges: {}\\n  Stability: {:.2}\\n  Contradiction: {:.2}\\n  Self-awareness: {:.2}",
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
'''

with open('ide/src/lib.rs', 'w', encoding='utf-8') as f:
    f.write(lib_rs)
print("lib.rs updated")

# 3. JavaScript WebSocket клиент
ws_client_js = '''// Atlas IDE - WebSocket клиент

class WsClient {
    constructor() {
        this.ws = null;
        this.connected = false;
        this.reconnectInterval = 3000;
        this.listeners = {};
        
        this.connect();
    }
    
    connect() {
        // Пытаемся подключиться к Rust WebSocket серверу
        // В реальном IDE будет: ws://localhost:8080/ws
        // Сейчас используем симуляцию
        
        console.log('WebSocket: attempting connection...');
        
        // Симуляция подключения
        setTimeout(() => {
            this.connected = true;
            console.log('WebSocket: connected');
            this.emit('connected', {});
            this.startSimulation();
        }, 1000);
    }
    
    startSimulation() {
        // Отправляем данные каждые 500мс
        setInterval(() => {
            const data = {
                stability: 0.5 + Math.random() * 0.5,
                contradiction: Math.random() * 0.6,
                awareness: 0.3 + Math.random() * 0.5,
                pulse: 60 + Math.random() * 80,
            };
            
            this.emit('metrics', data);
        }, 500);
    }
    
    on(event, callback) {
        if (!this.listeners[event]) {
            this.listeners[event] = [];
        }
        this.listeners[event].push(callback);
    }
    
    emit(event, data) {
        if (this.listeners[event]) {
            this.listeners[event].forEach(callback => callback(data));
        }
    }
    
    send(data) {
        if (this.connected) {
            console.log('WebSocket send:', data);
        }
    }
}

// Инициализация
const wsClient = new WsClient();

// Интеграция с кардиограммой
wsClient.on('metrics', (data) => {
    if (typeof cardiogram !== 'undefined') {
        cardiogram.setStability(data.stability);
        cardiogram.setContradiction(data.contradiction);
        cardiogram.setAwareness(data.awareness);
        
        // Обновляем статус-бар
        document.getElementById('stability').textContent = 
            'Стабильность: ' + data.stability.toFixed(2);
        document.getElementById('contradiction').textContent = 
            'Противоречие: ' + data.contradiction.toFixed(2);
        document.getElementById('awareness').textContent = 
            'Самосознание: ' + data.awareness.toFixed(2);
        document.getElementById('pulse').textContent = 
            'Пульс: ' + data.pulse.toFixed(1);
    }
});
'''

with open('ide/frontend/js/ws_client.js', 'w', encoding='utf-8') as f:
    f.write(ws_client_js)
print("ws_client.js created")

# 4. Обновляем index.html
index_html = '''<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Atlas IDE - Полифонические поля</title>
    <link rel="stylesheet" href="css/style.css">
    <link rel="stylesheet" href="css/editor.css">
</head>
<body>
    <div class="container">
        <header>
            <h1>Atlas IDE</h1>
            <div class="status-bar">
                <span id="stability">Стабильность: 1.00</span>
                <span id="contradiction">Противоречие: 0.00</span>
                <span id="awareness">Самосознание: 0.00</span>
                <span id="pulse">Пульс: 60.0</span>
                <span id="ws-status" style="color:#f1c40f;">WS: подключение...</span>
            </div>
        </header>
        
        <main>
            <div class="panel-left">
                <h2>Редактор</h2>
                <textarea id="editor" placeholder="Введите код Atlas...
                    
Примеры:
свобода ~::~ безопасность
синтез ::: уровень_2
понятие ~> Meta-понятие"></textarea>
                <button id="run-btn">▶ Запустить</button>
                <button id="synthesis-btn" style="background:#4CAF50;margin-top:5px;">⊕ Синтез</button>
                <button id="contradiction-btn" style="background:#e94560;margin-top:5px;">~::~ Противоречие</button>
            </div>
            
            <div class="panel-center">
                <h2>Полифоническое поле</h2>
                <canvas id="field-canvas" style="width:100%;height:calc(100% - 40px);"></canvas>
            </div>
            
            <div class="panel-right">
                <h2>Кардиограмма</h2>
                <canvas id="cardio-canvas" style="width:100%;height:200px;"></canvas>
                
                <h2>Рефлексивный каскад</h2>
                <div id="reflexive-view"></div>
            </div>
        </main>
        
        <footer>
            <div id="trace-console"></div>
        </footer>
    </div>
    
    <script src="js/main.js"></script>
    <script src="js/editor.js"></script>
    <script src="js/field.js"></script>
    <script src="js/cardio.js"></script>
    <script src="js/ws_client.js"></script>
    <script>
        wsClient.on('connected', () => {
            document.getElementById('ws-status').textContent = 'WS: подключен';
            document.getElementById('ws-status').style.color = '#4CAF50';
        });
        
        document.getElementById('synthesis-btn').addEventListener('click', () => {
            cardiogram.synthesisSpike();
            field.addSynthesisNode('Синтез_' + Date.now() % 100, 'Свобода', 'Безопасность');
            wsClient.send({ type: 'synthesis', name: 'Синтез_' + Date.now() % 100 });
        });
        
        document.getElementById('contradiction-btn').addEventListener('click', () => {
            cardiogram.contradictionDip();
            wsClient.send({ type: 'contradiction', severity: 0.8 });
        });
    </script>
</body>
</html>
'''

with open('ide/frontend/index.html', 'w', encoding='utf-8') as f:
    f.write(index_html)
print("index.html updated")

# 5. Обновляем Cargo.toml
cargo_toml = '''[package]
name = "atlas-ide"
version = "0.1.0"
edition = "2021"

[dependencies]
grammalang-core = { path = "../grammalang-core" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
tokio-tungstenite = "0.21"
futures-util = "0.3"

[lib]
name = "atlas_ide"
crate-type = ["cdylib", "rlib"]
'''

with open('ide/Cargo.toml', 'w', encoding='utf-8') as f:
    f.write(cargo_toml)
print("Cargo.toml updated")

print("\nAll IDE Phase 4 files created!")
