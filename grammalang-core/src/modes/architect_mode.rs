use crate::ontology::*;

/// Режим "Архитектор" - ручной синтез
pub struct ArchitectMode {
    pub generator: LLMSynthesisGenerator,
    pub integrator: SynthesisIntegrator,
    pub validator: SynthesisValidator,
}

impl Default for ArchitectMode {
    fn default() -> Self {
        Self {
            generator: LLMSynthesisGenerator::default(),
            integrator: SynthesisIntegrator::new(),
            validator: SynthesisValidator::new(),
        }
    }
}

impl ArchitectMode {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Ручной синтез с указанием параметров
    pub fn synthesize(
        &self,
        machine: &mut MachineState,
        node_a: &str,
        node_b: &str,
        strategy: SynthesisStrategy,
        custom_name: Option<&str>,
    ) -> ArchitectResult {
        // Снапшот для отката
        let before = machine.metrics.clone();
        
        // Генерация
        let mut synthesis = match self.generator.generate(node_a, node_b, &strategy) {
            Ok(s) => s,
            Err(e) => {
                return ArchitectResult {
                    success: false,
                    message: format!("Generation failed: {:?}", e),
                    node_id: None,
                }
            }
        };
        
        // Переопределение имени
        if let Some(name) = custom_name {
            synthesis.name = name.to_string();
        }
        
        // Интеграция
        let parents = vec![node_a.to_string(), node_b.to_string()];
        
        match self.integrator.integrate(machine, &synthesis, &parents) {
            Ok(result) => {
                // Валидация
                let validation = self.validator.validate(
                    machine,
                    &before,
                    &machine.metrics,
                );
                
                if validation.valid {
                    ArchitectResult {
                        success: true,
                        message: format!("Synthesis '{}' integrated successfully", synthesis.name),
                        node_id: Some(result.node_id),
                    }
                } else {
                    ArchitectResult {
                        success: false,
                        message: format!("Validation failed: {}", 
                            validation.reason.unwrap_or_default()),
                        node_id: None,
                    }
                }
            }
            Err(e) => {
                ArchitectResult {
                    success: false,
                    message: format!("Integration failed: {:?}", e),
                    node_id: None,
                }
            }
        }
    }
    
    /// Просмотр противоречий
    pub fn list_contradictions(&self, contradictions: &[Contradiction]) -> Vec<String> {
        contradictions.iter()
            .map(|c| format!("{} <-> {} (index: {:.2})", 
                c.node_a, c.node_b, c.contradiction_index))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ArchitectResult {
    pub success: bool,
    pub message: String,
    pub node_id: Option<String>,
}
