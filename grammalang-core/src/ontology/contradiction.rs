// src/ontology/contradiction.rs
use serde::{Serialize, Deserialize};
use super::target_ontology::NodeId;

/// РЎС‚СЂСѓРєС‚СѓСЂР° РїСЂРѕС‚РёРІРѕСЂРµС‡РёСЏ РјРµР¶РґСѓ СѓР·Р»Р°РјРё
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contradiction {
    /// РџРµСЂРІС‹Р№ СѓР·РµР» РїСЂРѕС‚РёРІРѕСЂРµС‡РёСЏ
    pub node_a: NodeId,
    
    /// Р’С‚РѕСЂРѕР№ СѓР·РµР» РїСЂРѕС‚РёРІРѕСЂРµС‡РёСЏ
    pub node_b: NodeId,
    
    /// РЎС‚РµРїРµРЅСЊ РїСЂРѕС‚РёРІРѕСЂРµС‡РёСЏ (0.0 - 1.0)
    pub severity: f32,
    
    /// РўРёРї РїСЂРѕС‚РёРІРѕСЂРµС‡РёСЏ
    pub kind: ContradictionKind,
    
    /// Р“РµРЅРµР°Р»РѕРіРёСЏ РїСЂРѕС‚РёРІРѕСЂРµС‡РёСЏ
    pub genealogy: Vec<String>,
    
    /// РљР°РЅРґРёРґР°С‚С‹ РґР»СЏ СЃРёРЅС‚РµР·Р°
    pub resolution_candidates: Vec<NodeId>,
    
    /// РСЃС‚РѕСЂРёСЏ РёР·РјРµРЅРµРЅРёР№ severity
    pub severity_history: Vec<SeverityRecord>,
    
    /// РРЅРґРµРєСЃ РїСЂРѕС‚РёРІРѕСЂРµС‡РёСЏ (РєРѕРјРїР»РµРєСЃРЅС‹Р№ РїРѕРєР°Р·Р°С‚РµР»СЊ)
    pub contradiction_index: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContradictionKind {
    /// Р›РѕРіРёС‡РµСЃРєРѕРµ: A Рё В¬A
    Logical,
    
    /// РЎС‚СЂСѓРєС‚СѓСЂРЅРѕРµ: A Рё B РЅРµСЃРѕРІРјРµСЃС‚РёРјС‹ РїРѕ РёРЅРІР°СЂРёР°РЅС‚Р°Рј
    Structural,
    
    /// РўРµРјРїРѕСЂР°Р»СЊРЅРѕРµ: A С‚СЂРµР±СѓРµС‚ B, РЅРѕ B СѓР¶Рµ РЅРµР°РєС‚РёРІРµРЅ
    Temporal,
    
    /// Р РµРєСѓСЂСЃРёРІРЅРѕРµ: A РїСЂРѕС‚РёРІРѕСЂРµС‡РёС‚ СЃР°РјРѕРјСѓ СЃРµР±Рµ
    Recursive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityRecord {
    pub timestamp: u64,
    pub severity: f32,
    pub stability: f32,
}

impl Contradiction {
    /// РЎРѕР·РґР°РЅРёРµ РЅРѕРІРѕРіРѕ РїСЂРѕС‚РёРІРѕСЂРµС‡РёСЏ
    pub fn new(node_a: NodeId, node_b: NodeId, kind: ContradictionKind) -> Self {
        Self {
            node_a,
            node_b,
            severity: 0.0,
            kind,
            genealogy: Vec::new(),
            resolution_candidates: Vec::new(),
            severity_history: Vec::new(),
            contradiction_index: 0.0,
        }
    }
    
    /// РћР±РЅРѕРІР»РµРЅРёРµ severity Рё РёСЃС‚РѕСЂРёРё
    pub fn update_severity(&mut self, new_severity: f32, stability: f32) {
        self.severity = new_severity;
        self.severity_history.push(SeverityRecord {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            severity: new_severity,
            stability,
        });
        
        // РћР±РЅРѕРІР»РµРЅРёРµ contradiction_index
        self.contradiction_index = self.calculate_contradiction_index();
    }
    
    /// Р Р°СЃС‡РµС‚ РєРѕРјРїР»РµРєСЃРЅРѕРіРѕ РёРЅРґРµРєСЃР° РїСЂРѕС‚РёРІРѕСЂРµС‡РёСЏ
    fn calculate_contradiction_index(&self) -> f32 {
        // Р‘Р°Р·РѕРІС‹Р№ РєРѕРјРїРѕРЅРµРЅС‚ - severity
        let severity_component = self.severity;
        
        // РљРѕРјРїРѕРЅРµРЅС‚ СЃС‚Р°Р±РёР»СЊРЅРѕСЃС‚Рё (РЅР° РѕСЃРЅРѕРІРµ РёСЃС‚РѕСЂРёРё)
        let stability_component = if self.severity_history.len() >= 3 {
            let recent: Vec<&SeverityRecord> = 
                self.severity_history.iter().rev().take(3).collect();
            
            // РџСЂРѕРІРµСЂСЏРµРј РїР°РґРµРЅРёРµ СЃС‚Р°Р±РёР»СЊРЅРѕСЃС‚Рё
            if recent.len() == 3 {
                let drop = recent[0].stability - recent[2].stability;
                if drop < -0.05 { 1.0 } else { 0.0 }
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        // Р’Р·РІРµС€РµРЅРЅР°СЏ СЃСѓРјРјР°
        0.7 * severity_component + 0.3 * stability_component
    }
    
    /// РџСЂРѕРІРµСЂРєР° РіРѕС‚РѕРІРЅРѕСЃС‚Рё Рє СЃРёРЅС‚РµР·Сѓ
    pub fn is_ready_for_synthesis(&self, threshold: f32) -> bool {
        self.contradiction_index > threshold && 
        self.severity_history.len() >= 3
    }
}

