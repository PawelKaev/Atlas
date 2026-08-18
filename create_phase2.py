# create_phase2.py
import os

os.makedirs('grammalang-core/src/ontology', exist_ok=True)
os.makedirs('grammalang-core/tests/ontology', exist_ok=True)

# 1. synthesis_generator.rs
generator_trait = '''use super::target_ontology::*;
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
'''

with open('grammalang-core/src/ontology/synthesis_generator.rs', 'w', encoding='utf-8') as f:
    f.write(generator_trait)
print("synthesis_generator.rs created")

# 2. synthesis_generator_llm.rs
llm_generator = '''use super::synthesis_generator::*;
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
'''

with open('grammalang-core/src/ontology/synthesis_generator_llm.rs', 'w', encoding='utf-8') as f:
    f.write(llm_generator)
print("synthesis_generator_llm.rs created")

# 3. synthesis_generator_diffusion.rs
diffusion_generator = '''use super::synthesis_generator::*;
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
'''

with open('grammalang-core/src/ontology/synthesis_generator_diffusion.rs', 'w', encoding='utf-8') as f:
    f.write(diffusion_generator)
print("synthesis_generator_diffusion.rs created")

# 4. synthesis_generator_evolutionary.rs
evolutionary_generator = '''use super::synthesis_generator::*;
use super::target_ontology::*;
use rand::Rng;

pub struct EvolutionarySynthesisGenerator {
    pub population_size: usize,
    pub generations: usize,
    pub mutation_rate: f32,
}

impl Default for EvolutionarySynthesisGenerator {
    fn default() -> Self {
        Self {
            population_size: 50,
            generations: 50,
            mutation_rate: 0.1,
        }
    }
}

impl EvolutionarySynthesisGenerator {
    pub fn new() -> Self {
        Self::default()
    }
    
    fn mutate(&self, s: &str) -> String {
        let mut rng = rand::thread_rng();
        let mut chars: Vec<char> = s.chars().collect();
        
        for i in 0..chars.len() {
            if rng.gen::<f32>() < self.mutation_rate {
                chars[i] = (b'a' + rng.gen_range(0..26)) as char;
            }
        }
        
        chars.into_iter().collect()
    }
    
    fn crossover(&self, a: &str, b: &str) -> String {
        let mut rng = rand::thread_rng();
        let mid = rng.gen_range(0..a.len().min(b.len()));
        let a_part = &a[..mid];
        let b_part = &b[mid..];
        format!("{}{}", a_part, b_part)
    }
    
    fn similarity(&self, a: &str, b: &str) -> f32 {
        let max_len = a.len().max(b.len());
        if max_len == 0 {
            return 1.0;
        }
        
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let mut matches = 0;
        
        for i in 0..max_len {
            if a_chars.get(i) == b_chars.get(i) {
                matches += 1;
            }
        }
        
        matches as f32 / max_len as f32
    }
    
    fn fitness(&self, candidate: &str, a: &str, b: &str) -> f32 {
        let sim_a = self.similarity(candidate, a);
        let sim_b = self.similarity(candidate, b);
        (sim_a + sim_b) / 2.0
    }
}

impl SynthesisGenerator for EvolutionarySynthesisGenerator {
    fn generate(&self, node_a: &str, node_b: &str, strategy: &SynthesisStrategy) -> Result<SynthesisResult, SynthesisError> {
        let mut rng = rand::thread_rng();
        
        let mut population: Vec<String> = Vec::new();
        for _ in 0..self.population_size {
            let candidate = self.crossover(node_a, node_b);
            population.push(self.mutate(&candidate));
        }
        
        for _ in 0..self.generations {
            let mut fitnesses: Vec<(f32, String)> = population
                .iter()
                .map(|c| (self.fitness(c, node_a, node_b), c.clone()))
                .collect();
            
            fitnesses.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            
            let best: Vec<String> = fitnesses
                .iter()
                .take(self.population_size / 2)
                .map(|(_, c)| c.clone())
                .collect();
            
            population = best.clone();
            while population.len() < self.population_size {
                let parent_a = &best[rng.gen_range(0..best.len())];
                let parent_b = &best[rng.gen_range(0..best.len())];
                let child = self.crossover(parent_a, parent_b);
                population.push(self.mutate(&child));
            }
        }
        
        let best = population
            .iter()
            .max_by(|a, b| {
                self.fitness(a, node_a, node_b)
                    .partial_cmp(&self.fitness(b, node_a, node_b))
                    .unwrap()
            })
            .unwrap()
            .clone();
        
        Ok(SynthesisResult {
            name: format!("evolved_{}", best),
            description: format!("Evolutionary synthesis of {} and {}", node_a, node_b),
            properties: vec![
                "evolutionary_generated".to_string(),
                format!("generations_{}", self.generations),
            ],
            strategy: strategy.clone(),
            confidence: 0.6,
        })
    }
}
'''

with open('grammalang-core/src/ontology/synthesis_generator_evolutionary.rs', 'w', encoding='utf-8') as f:
    f.write(evolutionary_generator)
print("synthesis_generator_evolutionary.rs created")

# 5. Обновляем mod.rs
mod_content = '''// src/ontology/mod.rs
pub mod engine;
pub mod target_ontology;
pub mod contradiction;
pub mod synthesis_detector;
pub mod synthesis_strategy_selector;
pub mod synthesis_generator;
pub mod synthesis_generator_llm;
pub mod synthesis_generator_diffusion;
pub mod synthesis_generator_evolutionary;
pub mod synthesis_integrator;
pub mod axis_proposer;
pub mod synthesis_validator;
pub mod synthesis_rollback;

pub use engine::*;
pub use target_ontology::*;
pub use contradiction::*;
pub use synthesis_detector::*;
pub use synthesis_strategy_selector::*;
pub use synthesis_generator::*;
pub use synthesis_generator_llm::*;
pub use synthesis_generator_diffusion::*;
pub use synthesis_generator_evolutionary::*;
'''

with open('grammalang-core/src/ontology/mod.rs', 'w', encoding='utf-8') as f:
    f.write(mod_content)
print("mod.rs updated")

# 6. Тесты
phase2_tests = '''use grammalang_core::ontology::*;

#[test]
fn test_llm_generator() {
    let generator = LLMSynthesisGenerator::default();
    let result = generator.generate("freedom", "security", &SynthesisStrategy::Hegelian).unwrap();
    assert!(!result.name.is_empty());
    assert!(result.confidence > 0.0);
    println!("LLM: {} (confidence: {:.2})", result.name, result.confidence);
}

#[test]
fn test_diffusion_generator() {
    let generator = DiffusionSynthesisGenerator::new();
    let result = generator.generate("thesis", "antithesis", &SynthesisStrategy::Plotinian).unwrap();
    assert!(!result.name.is_empty());
    println!("Diffusion: {}", result.name);
}

#[test]
fn test_evolutionary_generator() {
    let generator = EvolutionarySynthesisGenerator::new();
    let result = generator.generate("capitalism", "ecology", &SynthesisStrategy::Pragmatic).unwrap();
    assert!(!result.name.is_empty());
    println!("Evolutionary: {}", result.name);
}

#[test]
fn test_all_generators() {
    let llm = LLMSynthesisGenerator::new("qwen-32b");
    let diffusion = DiffusionSynthesisGenerator::new();
    let evolutionary = EvolutionarySynthesisGenerator::new();
    
    let strategies = vec![
        SynthesisStrategy::Hegelian,
        SynthesisStrategy::Plotinian,
        SynthesisStrategy::Pragmatic,
    ];
    
    for strategy in &strategies {
        let r1 = llm.generate("a", "b", strategy).unwrap();
        let r2 = diffusion.generate("a", "b", strategy).unwrap();
        let r3 = evolutionary.generate("a", "b", strategy).unwrap();
        
        assert!(!r1.name.is_empty());
        assert!(!r2.name.is_empty());
        assert!(!r3.name.is_empty());
    }
    
    println!("All generators work for all strategies");
}
'''

with open('grammalang-core/tests/ontology/phase2_tests.rs', 'w', encoding='utf-8') as f:
    f.write(phase2_tests)
print("phase2_tests.rs created")

# 7. Обновляем тестовый mod.rs
test_mod = '''pub mod target_ontology_tests;
pub mod contradiction_tests;
pub mod synthesis_detector_tests;
pub mod phase1_tests;
pub mod phase2_tests;
pub mod integration_test;
'''

with open('grammalang-core/tests/ontology/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll Phase 2 files created!")
