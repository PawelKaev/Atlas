# create_v09_phase2.py
import os

os.makedirs('grammalang-core/src/reflexive', exist_ok=True)
os.makedirs('grammalang-core/tests/reflexive', exist_ok=True)

# 1. MetaSynthesis
meta_synthesis = '''use serde::{Serialize, Deserialize};
use crate::ontology::{SynthesisStrategy, SynthesisResult};

/// MetaSynthesis - синтез из синтезов (второй порядок)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaSynthesis {
    /// Синтезы первого порядка
    pub first_order_syntheses: Vec<SynthesisResult>,
    
    /// Синтезы второго порядка (мета-синтезы)
    pub meta_syntheses: Vec<MetaSynthesisResult>,
    
    /// Уровень мета-синтеза
    pub level: usize,
    
    /// История мета-синтезов
    pub history: Vec<MetaSynthesisRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaSynthesisResult {
    /// Название мета-понятия
    pub name: String,
    
    /// Описание
    pub description: String,
    
    /// Исходные синтезы
    pub source_syntheses: Vec<String>,
    
    /// Уровень мета-синтеза
    pub level: usize,
    
    /// Осознание процесса
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
    
    /// Добавление синтеза первого порядка
    pub fn add_first_order(&mut self, synthesis: SynthesisResult) {
        self.first_order_syntheses.push(synthesis);
    }
    
    /// Создание мета-синтеза из двух синтезов
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
    
    /// Мета-синтез третьего порядка (синтез мета-синтезов)
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
        
        result
    }
    
    /// Синтез всех накопленных синтезов
    pub fn synthesize_all(&mut self) -> Option<MetaSynthesisResult> {
        if self.first_order_syntheses.len() < 2 {
            return None;
        }
        
        let first = &self.first_order_syntheses[0];
        let last = &self.first_order_syntheses[self.first_order_syntheses.len() - 1];
        
        Some(self.synthesize(first, last))
    }
    
    /// Получение мета-синтезов определенного уровня
    pub fn meta_syntheses_at_level(&self, level: usize) -> Vec<&MetaSynthesisResult> {
        self.meta_syntheses.iter()
            .filter(|m| m.level == level)
            .collect()
    }
    
    /// Количество синтезов
    pub fn first_order_count(&self) -> usize {
        self.first_order_syntheses.len()
    }
    
    /// Количество мета-синтезов
    pub fn meta_count(&self) -> usize {
        self.meta_syntheses.len()
    }
    
    /// Текущий уровень
    pub fn current_level(&self) -> usize {
        self.level
    }
    
    /// Проверка достижения мета-уровня
    pub fn has_meta_level(&self, level: usize) -> bool {
        self.level >= level
    }
}
'''

with open('grammalang-core/src/reflexive/meta_synthesis.rs', 'w', encoding='utf-8') as f:
    f.write(meta_synthesis)
print("meta_synthesis.rs updated")

# 2. Тесты Фазы 2
tests = '''use grammalang_core::reflexive::*;
use grammalang_core::ontology::*;

fn make_synthesis(name: &str) -> SynthesisResult {
    SynthesisResult {
        name: name.to_string(),
        description: format!("Synthesis {}", name),
        properties: vec![],
        strategy: SynthesisStrategy::Hegelian,
        confidence: 0.8,
    }
}

#[test]
fn test_meta_synthesis_basic() {
    let mut meta = MetaSynthesis::new();
    
    let s1 = make_synthesis("freedom_synthesis");
    let s2 = make_synthesis("security_synthesis");
    
    let result = meta.synthesize(&s1, &s2);
    
    assert_eq!(result.level, 1);
    assert_eq!(result.source_syntheses.len(), 2);
    assert!(result.name.starts_with("meta_"));
    
    println!("Meta-synthesis: {}", result.realization);
}

#[test]
fn test_third_order_synthesis() {
    let mut meta = MetaSynthesis::new();
    
    let s1 = make_synthesis("synthesis_A");
    let s2 = make_synthesis("synthesis_B");
    
    let m1 = meta.synthesize(&s1, &s2);
    
    let s3 = make_synthesis("synthesis_C");
    let s4 = make_synthesis("synthesis_D");
    
    let m2 = meta.synthesize(&s3, &s4);
    
    let m3 = meta.synthesize_meta(&m1, &m2);
    
    assert_eq!(m3.level, 3);
    assert!(m3.name.starts_with("meta_meta_"));
    assert!(m3.realization.contains("I realized that I synthesized"));
    
    println!("Third-order: {}", m3.realization);
}

#[test]
fn test_synthesize_all() {
    let mut meta = MetaSynthesis::new();
    
    meta.add_first_order(make_synthesis("s1"));
    meta.add_first_order(make_synthesis("s2"));
    meta.add_first_order(make_synthesis("s3"));
    
    let result = meta.synthesize_all().unwrap();
    
    assert!(result.name.starts_with("meta_"));
    assert_eq!(meta.first_order_count(), 3);
    assert_eq!(meta.meta_count(), 1);
    
    println!("Synthesized all: {} from {} syntheses", 
             result.name, meta.first_order_count());
}

#[test]
fn test_meta_levels() {
    let mut meta = MetaSynthesis::new();
    
    let s1 = make_synthesis("a");
    let s2 = make_synthesis("b");
    let m1 = meta.synthesize(&s1, &s2);
    
    assert!(meta.has_meta_level(1));
    assert!(!meta.has_meta_level(2));
    
    let s3 = make_synthesis("c");
    let s4 = make_synthesis("d");
    let m2 = meta.synthesize(&s3, &s4);
    
    let m3 = meta.synthesize_meta(&m1, &m2);
    
    assert!(meta.has_meta_level(3));
    assert_eq!(meta.current_level(), 3);
    
    println!("Meta levels: current = {}", meta.current_level());
}

#[test]
fn test_meta_history() {
    let mut meta = MetaSynthesis::new();
    
    meta.add_first_order(make_synthesis("s1"));
    meta.add_first_order(make_synthesis("s2"));
    
    meta.synthesize_all();
    
    assert_eq!(meta.history.len(), 1);
    
    println!("Meta history: {} records", meta.history.len());
}
'''

with open('grammalang-core/tests/reflexive/phase2_tests.rs', 'w', encoding='utf-8') as f:
    f.write(tests)
print("phase2_tests.rs created")

# 3. Обновляем тестовый mod.rs
test_mod = '''pub mod phase0_tests;
pub mod phase1_tests;
pub mod phase2_tests;
'''

with open('grammalang-core/tests/reflexive/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll v0.9 Phase 2 files created!")
