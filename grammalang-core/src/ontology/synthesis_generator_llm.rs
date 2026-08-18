use super::synthesis_generator::*;
use super::target_ontology::*;

pub struct LLMSynthesisGenerator {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: usize,
}

impl Default for LLMSynthesisGenerator {
    fn default() -> Self {
        Self {
            model: "qwen-32b".to_string(),
            temperature: 0.3,
            max_tokens: 500,
        }
    }
}

impl LLMSynthesisGenerator {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.to_string(),
            ..Default::default()
        }
    }
}

impl SynthesisGenerator for LLMSynthesisGenerator {
    fn generate(&self, node_a: &str, node_b: &str, strategy: &SynthesisStrategy) -> Result<SynthesisResult, SynthesisError> {
        let prompt = format!(
            "Given two concepts: '{}' and '{}'. They contradict each other. Propose a new concept that resolves this contradiction (Aufhebung).",
            node_a, node_b
        );
        
        let name = match strategy {
            SynthesisStrategy::Hegelian => format!("synthesis_{}_{}", node_a, node_b),
            SynthesisStrategy::Plotinian => format!("emanation_{}", node_a),
            SynthesisStrategy::Pragmatic => format!("pragmatic_{}_{}", node_a, node_b),
        };
        
        Ok(SynthesisResult {
            name,
            description: prompt,
            properties: vec![
                format!("inherits_from_{}", node_a),
                format!("inherits_from_{}", node_b),
            ],
            strategy: strategy.clone(),
            confidence: 0.7,
        })
    }
}
