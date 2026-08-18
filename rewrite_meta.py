# rewrite_meta.py
content = '''use serde::{Serialize, Deserialize};
use crate::ontology::{SynthesisStrategy, SynthesisResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaSynthesis {
    pub first_order_syntheses: Vec<SynthesisResult>,
    pub meta_syntheses: Vec<MetaSynthesisResult>,
    pub level: usize,
    pub history: Vec<MetaSynthesisRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaSynthesisResult {
    pub name: String,
    pub description: String,
    pub source_syntheses: Vec<String>,
    pub level: usize,
    pub realization: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaSynthesisRecord {
    pub timestamp: u64,
    pub level: usize,
    pub description: String,
}

impl MetaSynthesis {
    pub fn new() -> Self {
        Self {
            first_order_syntheses: Vec::new(),
            meta_syntheses: Vec::new(),
            level: 0,
            history: Vec::new(),
        }
    }
    
    pub fn add_first_order(&mut self, synthesis: SynthesisResult) {
        self.first_order_syntheses.push(synthesis);
    }
    
    pub fn synthesize(
        &mut self,
        synthesis_a: &SynthesisResult,
        synthesis_b: &SynthesisResult,
    ) -> MetaSynthesisResult {
        let meta_name = format!("meta_{}_{}", synthesis_a.name, synthesis_b.name);
        
        let result = MetaSynthesisResult {
            name: meta_name.clone(),
            description: format!(
                "Meta-synthesis of '{}' and '{}'",
                synthesis_a.name, synthesis_b.name
            ),
            source_syntheses: vec![
                synthesis_a.name.clone(),
                synthesis_b.name.clone(),
            ],
            level: self.level + 1,
            realization: format!(
                "I synthesized '{}' from syntheses '{}' and '{}'",
                meta_name, synthesis_a.name, synthesis_b.name
            ),
        };
        
        self.meta_syntheses.push(result.clone());
        self.history.push(MetaSynthesisRecord {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            level: result.level,
            description: result.realization.clone(),
        });
        
        self.level = self.level.max(result.level);
        
        result
    }
    
    pub fn synthesize_meta(
        &mut self,
        meta_a: &MetaSynthesisResult,
        meta_b: &MetaSynthesisResult,
    ) -> MetaSynthesisResult {
        let meta_name = format!("meta_meta_{}_{}", meta_a.name, meta_b.name);
        
        let result = MetaSynthesisResult {
            name: meta_name.clone(),
            description: format!(
                "Third-order synthesis of '{}' and '{}'",
                meta_a.name, meta_b.name
            ),
            source_syntheses: vec![
                meta_a.name.clone(),
                meta_b.name.clone(),
            ],
            level: 3,
            realization: format!(
                "I realized that I synthesized meta-concepts '{}' and '{}' into '{}'",
                meta_a.name, meta_b.name, meta_name
            ),
        };
        
        self.meta_syntheses.push(result.clone());
        self.history.push(MetaSynthesisRecord {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            level: 3,
            description: result.realization.clone(),
        });
        
        self.level = 3;
        
        result
    }
    
    pub fn synthesize_all(&mut self) -> Option<MetaSynthesisResult> {
        if self.first_order_syntheses.len() < 2 {
            return None;
        }
        
        let first = self.first_order_syntheses[0].clone();
        let last = self.first_order_syntheses[self.first_order_syntheses.len() - 1].clone();
        
        Some(self.synthesize(&first, &last))
    }
    
    pub fn meta_syntheses_at_level(&self, level: usize) -> Vec<&MetaSynthesisResult> {
        self.meta_syntheses.iter()
            .filter(|m| m.level == level)
            .collect()
    }
    
    pub fn first_order_count(&self) -> usize {
        self.first_order_syntheses.len()
    }
    
    pub fn meta_count(&self) -> usize {
        self.meta_syntheses.len()
    }
    
    pub fn current_level(&self) -> usize {
        self.level
    }
    
    pub fn has_meta_level(&self, level: usize) -> bool {
        self.level >= level
    }
}
'''

with open('grammalang-core/src/reflexive/meta_synthesis.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print("meta_synthesis.rs restored")