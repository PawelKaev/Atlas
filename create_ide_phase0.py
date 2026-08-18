# create_ide_phase0.py
import os

os.makedirs('ide', exist_ok=True)
os.makedirs('ide/src', exist_ok=True)
os.makedirs('ide/frontend', exist_ok=True)
os.makedirs('ide/frontend/js', exist_ok=True)
os.makedirs('ide/frontend/css', exist_ok=True)
os.makedirs('ide/tests', exist_ok=True)

# 1. Cargo.toml для IDE
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
print("Cargo.toml created")

# 2. lib.rs
lib_rs = '''// ide/src/lib.rs
use grammalang_core::ontology::*;
use grammalang_core::reflexive::*;
use grammalang_core::social::*;
use serde::{Serialize, Deserialize};

/// Состояние IDE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeState {
    pub machine: MachineState,
    pub reflexive_system: ReflexiveSystem,
    pub contradictions: Vec<Contradiction>,
    pub visual_state: VisualState,
    pub realtime_data: RealtimeData,
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

/// IDE Engine
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
            },
        }
    }
    
    /// Добавление узла
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
    
    /// Обновление real-time данных
    pub fn update_realtime(&mut self) {
        self.state.realtime_data.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.state.realtime_data.stability = self.state.machine.metrics.stability_ratio;
        self.state.realtime_data.contradiction_index = self.state.machine.metrics.contradiction_index;
        self.state.realtime_data.self_awareness = self.state.reflexive_system.state.self_awareness;
        
        // Пульс = частота изменений
        self.state.realtime_data.pulse_rate = 60.0 + 
            (self.state.machine.metrics.contradiction_index * 120.0);
    }
    
    /// Экспорт состояния в JSON
    pub fn export_json(&self) -> String {
        serde_json::to_string_pretty(&self.state).unwrap_or_default()
    }
    
    /// Получение сводки
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
        println!("IDE Engine created");
    }
    
    #[test]
    fn test_add_node() {
        let mut engine = IdeEngine::new();
        engine.add_node("freedom", vec!["abstract".to_string()]);
        
        assert_eq!(engine.state.visual_state.nodes.len(), 1);
        assert_eq!(engine.state.visual_state.nodes[0].label, "freedom");
        
        println!("Node added: {}", engine.state.visual_state.nodes[0].label);
    }
    
    #[test]
    fn test_realtime_update() {
        let mut engine = IdeEngine::new();
        
        engine.add_node("a", vec![]);
        engine.add_node("b", vec![]);
        
        engine.update_realtime();
        
        assert!(engine.state.realtime_data.timestamp > 0);
        
        println!("Realtime: stability {:.2}, pulse {:.1}", 
                 engine.state.realtime_data.stability,
                 engine.state.realtime_data.pulse_rate);
    }
    
    #[test]
    fn test_export_json() {
        let mut engine = IdeEngine::new();
        engine.add_node("test", vec![]);
        
        let json = engine.export_json();
        assert!(json.contains("test"));
        
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
print("lib.rs created")

# 3. Frontend - index.html
index_html = '''<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Atlas IDE - Полифонические поля</title>
    <link rel="stylesheet" href="css/style.css">
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
            </div>
        </header>
        
        <main>
            <div class="panel-left">
                <h2>Редактор</h2>
                <textarea id="editor" placeholder="Введите код Atlas..."></textarea>
                <button id="run-btn">▶ Запустить</button>
            </div>
            
            <div class="panel-center">
                <h2>Полифоническое поле</h2>
                <canvas id="field-canvas"></canvas>
            </div>
            
            <div class="panel-right">
                <h2>Кардиограмма</h2>
                <canvas id="cardio-canvas"></canvas>
                
                <h2>Рефлексивный каскад</h2>
                <div id="reflexive-view"></div>
            </div>
        </main>
        
        <footer>
            <div id="trace-console"></div>
        </footer>
    </div>
    
    <script src="js/main.js"></script>
</body>
</html>
'''

with open('ide/frontend/index.html', 'w', encoding='utf-8') as f:
    f.write(index_html)
print("index.html created")

# 4. CSS
css = '''* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Segoe UI', sans-serif;
    background: #1a1a2e;
    color: #e0e0e0;
    height: 100vh;
    overflow: hidden;
}

.container {
    display: flex;
    flex-direction: column;
    height: 100vh;
}

header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 20px;
    background: #16213e;
    border-bottom: 1px solid #0f3460;
}

h1 {
    font-size: 1.5em;
    color: #e94560;
}

.status-bar {
    display: flex;
    gap: 20px;
}

.status-bar span {
    font-size: 0.9em;
    padding: 5px 10px;
    background: #0f3460;
    border-radius: 5px;
}

main {
    display: flex;
    flex: 1;
    gap: 1px;
    background: #0f3460;
}

.panel-left {
    width: 30%;
    padding: 10px;
    background: #1a1a2e;
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.panel-center {
    width: 40%;
    padding: 10px;
    background: #1a1a2e;
}

.panel-right {
    width: 30%;
    padding: 10px;
    background: #1a1a2e;
    display: flex;
    flex-direction: column;
    gap: 10px;
}

h2 {
    font-size: 1.1em;
    margin-bottom: 10px;
    color: #e94560;
}

#editor {
    flex: 1;
    background: #16213e;
    color: #e0e0e0;
    border: 1px solid #0f3460;
    border-radius: 5px;
    padding: 10px;
    font-family: 'Consolas', monospace;
    font-size: 14px;
    resize: none;
}

button {
    padding: 10px 20px;
    background: #e94560;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    font-size: 1em;
}

button:hover {
    background: #ff6b81;
}

canvas {
    width: 100%;
    background: #16213e;
    border: 1px solid #0f3460;
    border-radius: 5px;
}

#field-canvas {
    height: 70%;
}

#cardio-canvas {
    height: 150px;
}

#reflexive-view {
    flex: 1;
    background: #16213e;
    border: 1px solid #0f3460;
    border-radius: 5px;
    padding: 10px;
    overflow-y: auto;
}

footer {
    padding: 10px 20px;
    background: #16213e;
    border-top: 1px solid #0f3460;
}

#trace-console {
    font-family: 'Consolas', monospace;
    font-size: 12px;
    color: #4CAF50;
    max-height: 100px;
    overflow-y: auto;
}
'''

with open('ide/frontend/css/style.css', 'w', encoding='utf-8') as f:
    f.write(css)
print("style.css created")

# 5. JavaScript
js = '''// Atlas IDE - Main JavaScript

class AtlasIDE {
    constructor() {
        this.state = {
            stability: 1.0,
            contradiction: 0.0,
            awareness: 0.0,
            pulse: 60.0,
        };
        
        this.nodes = [];
        this.cardioData = [];
        
        this.init();
    }
    
    init() {
        this.fieldCanvas = document.getElementById('field-canvas');
        this.cardioCanvas = document.getElementById('cardio-canvas');
        this.editor = document.getElementById('editor');
        this.traceConsole = document.getElementById('trace-console');
        
        document.getElementById('run-btn').addEventListener('click', () => this.run());
        
        this.startCardiogram();
        this.log('Atlas IDE initialized');
    }
    
    run() {
        const code = this.editor.value;
        this.log('Running: ' + code.substring(0, 50) + '...');
        
        // Имитация выполнения
        this.updateState({
            stability: Math.random() * 0.5 + 0.5,
            contradiction: Math.random() * 0.5,
            awareness: Math.random() * 0.7,
        });
    }
    
    updateState(newState) {
        Object.assign(this.state, newState);
        
        document.getElementById('stability').textContent = 
            'Стабильность: ' + this.state.stability.toFixed(2);
        document.getElementById('contradiction').textContent = 
            'Противоречие: ' + this.state.contradiction.toFixed(2);
        document.getElementById('awareness').textContent = 
            'Самосознание: ' + this.state.awareness.toFixed(2);
        document.getElementById('pulse').textContent = 
            'Пульс: ' + this.state.pulse.toFixed(1);
        
        this.drawField();
    }
    
    startCardiogram() {
        setInterval(() => {
            this.state.pulse = 60 + this.state.contradiction * 120;
            this.cardioData.push({
                stability: this.state.stability,
                contradiction: this.state.contradiction,
            });
            
            if (this.cardioData.length > 100) {
                this.cardioData.shift();
            }
            
            this.drawCardiogram();
        }, 500);
    }
    
    drawField() {
        const ctx = this.fieldCanvas.getContext('2d');
        const w = this.fieldCanvas.width;
        const h = this.fieldCanvas.height;
        
        ctx.clearRect(0, 0, w, h);
        
        // Рисуем фоновое поле
        const gradient = ctx.createRadialGradient(w/2, h/2, 0, w/2, h/2, w/2);
        gradient.addColorStop(0, '#1a1a2e');
        gradient.addColorStop(1, '#0f3460');
        ctx.fillStyle = gradient;
        ctx.fillRect(0, 0, w, h);
        
        // Рисуем узлы
        this.nodes.forEach((node, i) => {
            const x = w/2 + Math.cos(i * 2 * Math.PI / this.nodes.length) * w/4;
            const y = h/2 + Math.sin(i * 2 * Math.PI / this.nodes.length) * h/4;
            
            ctx.beginPath();
            ctx.arc(x, y, node.size || 10, 0, 2 * Math.PI);
            ctx.fillStyle = node.color || '#4CAF50';
            ctx.fill();
            
            ctx.fillStyle = '#fff';
            ctx.font = '12px Arial';
            ctx.textAlign = 'center';
            ctx.fillText(node.label || '', x, y - 15);
        });
    }
    
    drawCardiogram() {
        const ctx = this.cardioCanvas.getContext('2d');
        const w = this.cardioCanvas.width;
        const h = this.cardioCanvas.height;
        
        ctx.clearRect(0, 0, w, h);
        ctx.fillStyle = '#16213e';
        ctx.fillRect(0, 0, w, h);
        
        // Рисуем линию стабильности
        ctx.beginPath();
        ctx.strokeStyle = '#4CAF50';
        ctx.lineWidth = 2;
        
        this.cardioData.forEach((data, i) => {
            const x = (i / 100) * w;
            const y = h - data.stability * h;
            
            if (i === 0) {
                ctx.moveTo(x, y);
            } else {
                ctx.lineTo(x, y);
            }
        });
        
        ctx.stroke();
        
        // Рисуем линию противоречия
        ctx.beginPath();
        ctx.strokeStyle = '#e94560';
        ctx.lineWidth = 1;
        
        this.cardioData.forEach((data, i) => {
            const x = (i / 100) * w;
            const y = h - data.contradiction * h;
            
            if (i === 0) {
                ctx.moveTo(x, y);
            } else {
                ctx.lineTo(x, y);
            }
        });
        
        ctx.stroke();
    }
    
    log(message) {
        const entry = document.createElement('div');
        entry.textContent = new Date().toLocaleTimeString() + ' - ' + message;
        this.traceConsole.appendChild(entry);
        this.traceConsole.scrollTop = this.traceConsole.scrollHeight;
    }
}

// Запуск IDE
const ide = new AtlasIDE();
'''

with open('ide/frontend/js/main.js', 'w', encoding='utf-8') as f:
    f.write(js)
print("main.js created")

print("\nAll IDE Phase 0 files created!")

