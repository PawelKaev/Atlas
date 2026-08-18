use super::synthesis_generator::*;
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
