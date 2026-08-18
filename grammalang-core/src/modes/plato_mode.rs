use crate::ontology::*;

/// Режим "Платон" - автоматический синтез
pub struct PlatoMode {
    pub detector: SynthesisDetector,
    pub generator: LLMSynthesisGenerator,
    pub integrator: SynthesisIntegrator,
    pub validator: SynthesisValidator,
    pub max_iterations: usize,
}

impl Default for PlatoMode {
    fn default() -> Self {
        Self {
            detector: SynthesisDetector::new(),
            generator: LLMSynthesisGenerator::default(),
            integrator: SynthesisIntegrator::new(),
            validator: SynthesisValidator::new(),
            max_iterations: 10,
        }
    }
}

impl PlatoMode {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Автоматический запуск синтеза
    pub fn run(&self, machine: &mut MachineState, contradictions: &[Contradiction]) -> PlatoResult {
        let mut events = Vec::new();
        let mut iterations = 0;
        
        // Обнаружение кандидатов
        let candidates = self.detector.detect(contradictions);
        
        for candidate in candidates.iter().take(self.max_iterations) {
            iterations += 1;
            
            // Снапшот для отката
            let before = machine.metrics.clone();
            
            // Генерация синтеза
            let synthesis = match self.generator.generate(
                &candidate.source_nodes[0],
                &candidate.source_nodes[1],
                &candidate.strategy_hint,
            ) {
                Ok(s) => s,
                Err(e) => {
                    events.push(PlatoEvent::GenerationFailed(format!("{:?}", e)));
                    continue;
                }
            };
            
            // Интеграция
            match self.integrator.integrate(machine, &synthesis, &candidate.source_nodes) {
                Ok(result) => {
                    // Валидация
                    let validation = self.validator.validate(
                        machine,
                        &before,
                        &machine.metrics,
                    );
                    
                    if validation.valid {
                        events.push(PlatoEvent::SynthesisCompleted {
                            name: synthesis.name.clone(),
                            confidence: synthesis.confidence,
                            node_id: result.node_id,
                        });
                    } else {
                        events.push(PlatoEvent::ValidationFailed(
                            validation.reason.unwrap_or_default()
                        ));
                    }
                }
                Err(e) => {
                    events.push(PlatoEvent::IntegrationFailed(format!("{:?}", e)));
                }
            }
        }
        
        PlatoResult {
            iterations,
            events,
            final_metrics: machine.metrics.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlatoEvent {
    SynthesisCompleted {
        name: String,
        confidence: f32,
        node_id: String,
    },
    GenerationFailed(String),
    IntegrationFailed(String),
    ValidationFailed(String),
}

#[derive(Debug, Clone)]
pub struct PlatoResult {
    pub iterations: usize,
    pub events: Vec<PlatoEvent>,
    pub final_metrics: MachineMetrics,
}
