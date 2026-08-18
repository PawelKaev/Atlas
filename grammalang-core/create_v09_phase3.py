# create_v09_phase3.py
import os

os.makedirs('grammalang-core/src/reflexive', exist_ok=True)
os.makedirs('grammalang-core/tests/reflexive', exist_ok=True)

# 1. AutoGenealogy
auto_genealogy = '''use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// AutoGenealogy - машина порождает и анализирует свою историю
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoGenealogy {
    /// Генеалогическое дерево
    pub tree: GenealogyTree,
    
    /// История порождений
    pub generation_history: Vec<GenerationRecord>,
    
    /// Самопорожденные понятия
    pub self_generated: Vec<SelfGeneratedConcept>,
    
    /// Уровень автогенеалогии
    pub auto_level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenealogyTree {
    /// Корневые узлы (начала)
    pub roots: Vec<String>,
    
    /// Связи родитель -> потомки
    pub relations: HashMap<String, Vec<String>>,
    
    /// Глубина дерева
    pub depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRecord {
    pub timestamp: u64,
    pub parent: String,
    pub child: String,
    pub description: String,
    pub generation_type: GenerationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationType {
    /// Синтез
    Synthesis,
    /// Мета-синтез
    MetaSynthesis,
    /// Рефлексия
    Reflection,
    /// Осознание
    Awareness,
    /// Самопорождение
    SelfGeneration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfGeneratedConcept {
    pub name: String,
    pub origin: String,
    pub timestamp: u64,
}

impl AutoGenealogy {
    pub fn new() -> Self {
        Self {
            tree: GenealogyTree {
                roots: Vec::new(),
                relations: HashMap::new(),
                depth: 0,
            },
            generation_history: Vec::new(),
            self_generated: Vec::new(),
            auto_level: 0,
        }
    }
    
    /// Добавление корневого узла
    pub fn add_root(&mut self, name: &str) {
        self.tree.roots.push(name.to_string());
        self.tree.relations.entry(name.to_string()).or_insert_with(Vec::new);
    }
    
    /// Порождение потомка
    pub fn generate(
        &mut self,
        parent: &str,
        child: &str,
        generation_type: GenerationType,
        description: &str,
    ) {
        self.tree.relations
            .entry(parent.to_string())
            .or_insert_with(Vec::new)
            .push(child.to_string());
        
        self.tree.relations
            .entry(child.to_string())
            .or_insert_with(Vec::new);
        
        self.generation_history.push(GenerationRecord {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            parent: parent.to_string(),
            child: child.to_string(),
            description: description.to_string(),
            generation_type,
        });
        
        self.update_depth();
    }
    
    /// Самопорождение понятия
    pub fn self_generate(&mut self, name: &str, origin: &str) {
        self.self_generated.push(SelfGeneratedConcept {
            name: name.to_string(),
            origin: origin.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        
        self.auto_level += 1;
    }
    
    /// Обновление глубины дерева
    fn update_depth(&mut self) {
        let mut max_depth = 0;
        
        for root in &self.tree.roots {
            let depth = self.calculate_depth(root, 0, &mut Vec::new());
            max_depth = max_depth.max(depth);
        }
        
        self.tree.depth = max_depth;
    }
    
    /// Рекурсивный расчет глубины
    fn calculate_depth(&self, node: &str, current: usize, visited: &mut Vec<String>) -> usize {
        if visited.contains(&node.to_string()) {
            return current;
        }
        visited.push(node.to_string());
        
        let children = self.tree.relations.get(node)
            .map(|v| v.clone())
            .unwrap_or_default();
        
        if children.is_empty() {
            return current;
        }
        
        let mut max_child_depth = current;
        for child in &children {
            let depth = self.calculate_depth(child, current + 1, visited);
            max_child_depth = max_child_depth.max(depth);
        }
        
        max_child_depth
    }
    
    /// Получение потомков узла
    pub fn descendants(&self, node: &str) -> Vec<&String> {
        self.tree.relations.get(node)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    
    /// Получение всей линии узла
    pub fn lineage(&self, node: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = node.to_string();
        
        // Идем вверх по дереву
        while let Some((parent, _)) = self.tree.relations.iter()
            .find(|(_, children)| children.contains(&current)) {
            result.push(parent.clone());
            current = parent.clone();
        }
        
        result.reverse();
        result.push(node.to_string());
        result
    }
    
    /// Анализ своей истории
    pub fn analyze_self(&self) -> String {
        format!(
            "I have {} roots, {} generations, {} self-generated concepts, tree depth {}",
            self.tree.roots.len(),
            self.generation_history.len(),
            self.self_generated.len(),
            self.tree.depth
        )
    }
    
    /// Количество порождений
    pub fn generation_count(&self) -> usize {
        self.generation_history.len()
    }
    
    /// Количество самопорожденных
    pub fn self_generated_count(&self) -> usize {
        self.self_generated.len()
    }
    
    /// Уровень автогенеалогии
    pub fn auto_level(&self) -> usize {
        self.auto_level
    }
    
    /// Проверка самопорождения
    pub fn is_self_generating(&self) -> bool {
        self.auto_level > 0
    }
}
'''

with open('grammalang-core/src/reflexive/auto_genealogy.rs', 'w', encoding='utf-8') as f:
    f.write(auto_genealogy)
print("auto_genealogy.rs updated")

# 2. Тесты Фазы 3
tests = '''use grammalang_core::reflexive::*;

#[test]
fn test_auto_genealogy_basic() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.add_root("origin");
    genealogy.generate("origin", "synthesis_1", GenerationType::Synthesis, "First synthesis");
    
    assert_eq!(genealogy.generation_count(), 1);
    assert_eq!(genealogy.tree.roots.len(), 1);
    
    println!("AutoGenealogy: {} generations, {} roots", 
             genealogy.generation_count(), genealogy.tree.roots.len());
}

#[test]
fn test_generation_tree() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.add_root("root");
    genealogy.generate("root", "child1", GenerationType::Synthesis, "Synthesis 1");
    genealogy.generate("root", "child2", GenerationType::Synthesis, "Synthesis 2");
    genealogy.generate("child1", "grandchild", GenerationType::MetaSynthesis, "Meta");
    
    let descendants = genealogy.descendants("root");
    assert_eq!(descendants.len(), 2);
    
    let lineage = genealogy.lineage("grandchild");
    assert_eq!(lineage.len(), 3);
    assert_eq!(lineage[0], "root");
    assert_eq!(lineage[1], "child1");
    assert_eq!(lineage[2], "grandchild");
    
    println!("Lineage of grandchild: {:?}", lineage);
}

#[test]
fn test_self_generation() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.self_generate("self_concept_1", "I created this about myself");
    genealogy.self_generate("self_concept_2", "Another self-created concept");
    
    assert_eq!(genealogy.self_generated_count(), 2);
    assert_eq!(genealogy.auto_level(), 2);
    assert!(genealogy.is_self_generating());
    
    println!("Self-generated: {} concepts (level {})", 
             genealogy.self_generated_count(), genealogy.auto_level());
}

#[test]
fn test_tree_depth() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.add_root("level_0");
    genealogy.generate("level_0", "level_1", GenerationType::Synthesis, "S");
    genealogy.generate("level_1", "level_2", GenerationType::MetaSynthesis, "M");
    genealogy.generate("level_2", "level_3", GenerationType::Reflection, "R");
    
    assert_eq!(genealogy.tree.depth, 3);
    
    println!("Tree depth: {}", genealogy.tree.depth);
}

#[test]
fn test_analyze_self() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.add_root("root1");
    genealogy.add_root("root2");
    genealogy.generate("root1", "child1", GenerationType::Synthesis, "S");
    genealogy.self_generate("self1", "self");
    
    let analysis = genealogy.analyze_self();
    
    assert!(analysis.contains("2 roots"));
    assert!(analysis.contains("1 generations"));
    assert!(analysis.contains("1 self-generated"));
    
    println!("{}", analysis);
}

#[test]
fn test_generation_types() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.add_root("root");
    genealogy.generate("root", "synth", GenerationType::Synthesis, "S");
    genealogy.generate("synth", "meta", GenerationType::MetaSynthesis, "M");
    genealogy.generate("meta", "refl", GenerationType::Reflection, "R");
    genealogy.generate("refl", "aware", GenerationType::Awareness, "A");
    genealogy.generate("aware", "self", GenerationType::SelfGeneration, "SG");
    
    assert_eq!(genealogy.generation_count(), 5);
    println!("All generation types work: {} generations", genealogy.generation_count());
}
'''

with open('grammalang-core/tests/reflexive/phase3_tests.rs', 'w', encoding='utf-8') as f:
    f.write(tests)
print("phase3_tests.rs created")

# 3. Обновляем тестовый mod.rs
test_mod = '''pub mod phase0_tests;
pub mod phase1_tests;
pub mod phase2_tests;
pub mod phase3_tests;
'''

with open('grammalang-core/tests/reflexive/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll v0.9 Phase 3 files created!")
