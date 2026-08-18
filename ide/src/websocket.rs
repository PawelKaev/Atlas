// ide/src/websocket.rs
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
            assert!(message.data.message.contains("test_synthesis"));
            println!("Synthesis message received");
        }
    }
}
