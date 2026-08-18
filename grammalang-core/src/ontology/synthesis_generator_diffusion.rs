use super::synthesis_generator::*;
use super::target_ontology::*;

pub struct DiffusionSynthesisGenerator {
    pub steps: usize,
    pub noise_scale: f32,
}

impl Default for DiffusionSynthesisGenerator {
    fn default() -> Self {
        Self {
            steps: 50,
            noise_scale: 0.1,
        }
    }
}

impl DiffusionSynthesisGenerator {
    pub fn new() -> Self {
        Self::default()
    }
    
    fn interpolate(&self, a: &str, b: &str, t: f32) -> String {
        let max_len = a.len().max(b.len());
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        
        let mut result = String::new();
        for i in 0..max_len {
            let ca = a_chars.get(i).copied().unwrap_or(' ');
            let cb = b_chars.get(i).copied().unwrap_or(' ');
            
            if ca == cb {
                result.push(ca);
            } else if t < 0.5 {
                result.push(ca);
            } else {
                result.push(cb);
            }
        }
        
        result
    }
}

impl SynthesisGenerator for DiffusionSynthesisGenerator {
    fn generate(&self, node_a: &str, node_b: &str, strategy: &SynthesisStrategy) -> Result<SynthesisResult, SynthesisError> {
        let name = self.interpolate(node_a, node_b, 0.5);
        
        Ok(SynthesisResult {
            name: format!("diffused_{}", name),
            description: format!("Diffusion synthesis of {} and {}", node_a, node_b),
            properties: vec![
                "diffusion_generated".to_string(),
                "interpolation_0.5".to_string(),
            ],
            strategy: strategy.clone(),
            confidence: 0.5,
        })
    }
}
