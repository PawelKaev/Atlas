# fix_integration.py
content = '''use super::synthesis_generator::*;
use super::target_ontology::*;
use super::synthesis_detector::*;
use super::synthesis_generator_llm::LLMSynthesisGenerator;

pub struct LLMResolverIntegration {
    pub generator: LLMSynthesisGenerator,
}

impl LLMResolverIntegration {
    pub fn new() -> Self {
        Self {
            generator: LLMSynthesisGenerator::default(),
        }
    }
    
    pub fn generate_with_resolver(
        &self,
        node_a: &str,
        node_b: &str,
        strategy: &SynthesisStrategy,
    ) -> Result<SynthesisResult, SynthesisError> {
        let mut result = self.generator.generate(node_a, node_b, strategy)?;
        result.description = format!(
            "{} (resolved via LLM)",
            result.description
        );
        Ok(result)
    }
    
    pub fn preserve_genealogy(
        &self,
        synthesis: &SynthesisResult,
        parents: &[String],
    ) -> Vec<String> {
        let mut genealogy = parents.to_vec();
        genealogy.push(synthesis.name.clone());
        genealogy
    }
}

pub struct TemporalMapIntegration {
    pub events: Vec<TemporalEvent>,
}

#[derive(Debug, Clone)]
pub struct TemporalEvent {
    pub timestamp: u64,
    pub event_type: TemporalEventType,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum TemporalEventType {
    Synthesis,
    Validation,
    Rollback,
    AxisProposal,
}

impl TemporalMapIntegration {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }
    
    pub fn record_synthesis(&mut self, name: &str, confidence: f32) {
        self.events.push(TemporalEvent {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_type: TemporalEventType::Synthesis,
            description: format!("Synthesis: {} (confidence: {:.2})", name, confidence),
        });
    }
    
    pub fn record_validation(&mut self, valid: bool) {
        self.events.push(TemporalEvent {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_type: TemporalEventType::Validation,
            description: format!("Validation: {}", if valid { "passed" } else { "failed" }),
        });
    }
    
    pub fn record_rollback(&mut self) {
        self.events.push(TemporalEvent {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_type: TemporalEventType::Rollback,
            description: "Rollback performed".to_string(),
        });
    }
    
    pub fn history(&self) -> &[TemporalEvent] {
        &self.events
    }
    
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

pub struct IntegrationLayer {
    pub llm_resolver: LLMResolverIntegration,
    pub temporal_map: TemporalMapIntegration,
}

impl IntegrationLayer {
    pub fn new() -> Self {
        Self {
            llm_resolver: LLMResolverIntegration::new(),
            temporal_map: TemporalMapIntegration::new(),
        }
    }
    
    pub fn full_cycle(
        &mut self,
        machine: &mut super::synthesis_integrator::MachineState,
        node_a: &str,
        node_b: &str,
        strategy: &SynthesisStrategy,
    ) -> Result<SynthesisResult, SynthesisError> {
        let synthesis = self.llm_resolver.generate_with_resolver(node_a, node_b, strategy)?;
        
        self.temporal_map.record_synthesis(&synthesis.name, synthesis.confidence);
        
        let integrator = super::synthesis_integrator::SynthesisIntegrator::new();
        let parents = vec![node_a.to_string(), node_b.to_string()];
        
        match integrator.integrate(machine, &synthesis, &parents) {
            Ok(_) => {
                self.temporal_map.record_validation(true);
                Ok(synthesis)
            }
            Err(_) => {
                self.temporal_map.record_validation(false);
                self.temporal_map.record_rollback();
                Err(SynthesisError::GenerationFailed("Integration failed".to_string()))
            }
        }
    }
}
'''

with open('grammalang-core/src/ontology/integration_layer.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print('File rewritten successfully')
