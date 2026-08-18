use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoGenealogy {
    pub tree: GenealogyTree,
    pub generation_history: Vec<GenerationRecord>,
    pub self_generated: Vec<SelfGeneratedConcept>,
    pub auto_level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenealogyTree {
    pub roots: Vec<String>,
    pub relations: HashMap<String, Vec<String>>,
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
    Synthesis,
    MetaSynthesis,
    Reflection,
    Awareness,
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
    
    pub fn add_root(&mut self, name: &str) {
        self.tree.roots.push(name.to_string());
        self.tree.relations.entry(name.to_string()).or_insert_with(Vec::new);
    }
    
    pub fn generate(&mut self, parent: &str, child: &str, generation_type: GenerationType, description: &str) {
        self.tree.relations.entry(parent.to_string()).or_insert_with(Vec::new).push(child.to_string());
        self.tree.relations.entry(child.to_string()).or_insert_with(Vec::new);
        self.generation_history.push(GenerationRecord {
            timestamp: 0,
            parent: parent.to_string(),
            child: child.to_string(),
            description: description.to_string(),
            generation_type,
        });
        self.update_depth();
    }
    
    pub fn self_generate(&mut self, name: &str, origin: &str) {
        self.self_generated.push(SelfGeneratedConcept {
            name: name.to_string(),
            origin: origin.to_string(),
            timestamp: 0,
        });
        self.auto_level += 1;
    }
    
    fn update_depth(&mut self) {
        let mut max_depth = 0;
        for root in &self.tree.roots {
            let depth = self.calculate_depth(root, 0, &mut Vec::new());
            max_depth = max_depth.max(depth);
        }
        self.tree.depth = max_depth;
    }
    
    fn calculate_depth(&self, node: &str, current: usize, visited: &mut Vec<String>) -> usize {
        if visited.contains(&node.to_string()) { return current; }
        visited.push(node.to_string());
        let children = self.tree.relations.get(node).map(|v| v.clone()).unwrap_or_default();
        if children.is_empty() { return current; }
        let mut max_child = current;
        for child in &children {
            let d = self.calculate_depth(child, current + 1, visited);
            max_child = max_child.max(d);
        }
        max_child
    }
    
    pub fn descendants(&self, node: &str) -> Vec<&String> {
        self.tree.relations.get(node).map(|v| v.iter().collect()).unwrap_or_default()
    }
    
    pub fn lineage(&self, node: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = node.to_string();
        while let Some((parent, _)) = self.tree.relations.iter().find(|(_, children)| children.contains(&current)) {
            result.push(parent.clone());
            current = parent.clone();
        }
        result.reverse();
        result.push(node.to_string());
        result
    }
    
    pub fn analyze_self(&self) -> String {
        format!("I have {} roots, {} generations, {} self-generated concepts, tree depth {}",
            self.tree.roots.len(), self.generation_history.len(), self.self_generated.len(), self.tree.depth)
    }
    
    pub fn generation_count(&self) -> usize { self.generation_history.len() }
    pub fn self_generated_count(&self) -> usize { self.self_generated.len() }
    pub fn auto_level(&self) -> usize { self.auto_level }
    pub fn is_self_generating(&self) -> bool { self.auto_level > 0 }
}
