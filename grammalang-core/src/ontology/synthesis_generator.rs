use super::target_ontology::*;
use super::synthesis_detector::*;

#[derive(Debug, Clone)]
pub enum SynthesisError {
    GenerationFailed(String),
    LLMUnavailable(String),
    InsufficientData(String),
    Timeout,
}

#[derive(Debug, Clone)]
pub struct SynthesisResult {
    pub name: String,
    pub description: String,
    pub properties: Vec<String>,
    pub strategy: SynthesisStrategy,
    pub confidence: f32,
}

pub trait SynthesisGenerator {
    fn generate(&self, node_a: &str, node_b: &str, strategy: &SynthesisStrategy) -> Result<SynthesisResult, SynthesisError>;
}
