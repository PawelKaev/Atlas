# create_v08_phase1.py
import os

os.makedirs('grammalang-core/src/social', exist_ok=True)
os.makedirs('grammalang-core/tests/social', exist_ok=True)

# 1. Коннекторы для внешних баз знаний
connectors = '''use super::knowledge_base::*;
use serde_json::Value;
use std::collections::HashMap;

/// Коннектор для Wikidata
pub struct WikidataConnector {
    pub endpoint: String,
    pub cache: HashMap<String, KnowledgeNode>,
}

impl WikidataConnector {
    pub fn new() -> Self {
        Self {
            endpoint: "https://www.wikidata.org/wiki/Special:EntityData".to_string(),
            cache: HashMap::new(),
        }
    }
    
    /// Загрузка сущности по ID
    pub fn fetch_entity(&mut self, entity_id: &str) -> Option<KnowledgeNode> {
        // Проверяем кэш
        if let Some(node) = self.cache.get(entity_id) {
            return Some(node.clone());
        }
        
        // Имитация загрузки (в реальности - HTTP запрос)
        let node = KnowledgeNode {
            id: format!("wikidata:{}", entity_id),
            label: format!("Entity {}", entity_id),
            description: format!("Wikidata entity {}", entity_id),
            properties: {
                let mut props = HashMap::new();
                props.insert("source".to_string(), "wikidata".to_string());
                props.insert("entity_id".to_string(), entity_id.to_string());
                props
            },
            relations: vec![],
        };
        
        self.cache.insert(entity_id.to_string(), node.clone());
        Some(node)
    }
    
    /// Поиск по метке
    pub fn search(&self, query: &str) -> Vec<KnowledgeNode> {
        self.cache.values()
            .filter(|n| n.label.contains(query))
            .cloned()
            .collect()
    }
    
    /// Количество закэшированных
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

/// Коннектор для DBPedia
pub struct DBPediaConnector {
    pub endpoint: String,
    pub cache: HashMap<String, KnowledgeNode>,
}

impl DBPediaConnector {
    pub fn new() -> Self {
        Self {
            endpoint: "https://dbpedia.org/sparql".to_string(),
            cache: HashMap::new(),
        }
    }
    
    /// Загрузка ресурса
    pub fn fetch_resource(&mut self, resource: &str) -> Option<KnowledgeNode> {
        if let Some(node) = self.cache.get(resource) {
            return Some(node.clone());
        }
        
        let node = KnowledgeNode {
            id: format!("dbpedia:{}", resource),
            label: resource.to_string(),
            description: format!("DBPedia resource: {}", resource),
            properties: {
                let mut props = HashMap::new();
                props.insert("source".to_string(), "dbpedia".to_string());
                props.insert("resource".to_string(), resource.to_string());
                props
            },
            relations: vec![],
        };
        
        self.cache.insert(resource.to_string(), node.clone());
        Some(node)
    }
    
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

/// Коннектор для JSON-LD
pub struct JsonLdConnector {
    pub documents: Vec<Value>,
}

impl JsonLdConnector {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
        }
    }
    
    /// Загрузка JSON-LD документа
    pub fn load_document(&mut self, json: &str) -> Result<(), String> {
        let value: Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.documents.push(value);
        Ok(())
    }
    
    /// Извлечение узлов из документов
    pub fn extract_nodes(&self) -> Vec<KnowledgeNode> {
        let mut nodes = Vec::new();
        
        for doc in &self.documents {
            if let Some(items) = doc.get("@graph").and_then(|g| g.as_array()) {
                for item in items {
                    if let (Some(id), Some(label)) = (
                        item.get("@id").and_then(|v| v.as_str()),
                        item.get("name").and_then(|v| v.as_str()),
                    ) {
                        nodes.push(KnowledgeNode {
                            id: id.to_string(),
                            label: label.to_string(),
                            description: item.get("description")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            properties: HashMap::new(),
                            relations: vec![],
                        });
                    }
                }
            }
        }
        
        nodes
    }
    
    /// Количество загруженных документов
    pub fn document_count(&self) -> usize {
        self.documents.len()
    }
}

/// Менеджер коннекторов
pub struct KnowledgeBaseManager {
    pub wikidata: WikidataConnector,
    pub dbpedia: DBPediaConnector,
    pub jsonld: JsonLdConnector,
    pub bases: Vec<KnowledgeBase>,
}

impl KnowledgeBaseManager {
    pub fn new() -> Self {
        Self {
            wikidata: WikidataConnector::new(),
            dbpedia: DBPediaConnector::new(),
            jsonld: JsonLdConnector::new(),
            bases: Vec::new(),
        }
    }
    
    /// Создание базы знаний из Wikidata
    pub fn create_from_wikidata(&mut self, entity_ids: &[&str]) -> KnowledgeBase {
        let mut kb = KnowledgeBase::new(
            "wikidata_kb",
            "Wikidata Knowledge Base",
            KnowledgeBaseType::Wikidata,
        );
        
        for id in entity_ids {
            if let Some(node) = self.wikidata.fetch_entity(id) {
                kb.add_node(node);
            }
        }
        
        self.bases.push(kb.clone());
        kb
    }
    
    /// Создание базы знаний из DBPedia
    pub fn create_from_dbpedia(&mut self, resources: &[&str]) -> KnowledgeBase {
        let mut kb = KnowledgeBase::new(
            "dbpedia_kb",
            "DBPedia Knowledge Base",
            KnowledgeBaseType::DBPedia,
        );
        
        for resource in resources {
            if let Some(node) = self.dbpedia.fetch_resource(resource) {
                kb.add_node(node);
            }
        }
        
        self.bases.push(kb.clone());
        kb
    }
    
    /// Создание базы знаний из JSON-LD
    pub fn create_from_jsonld(&mut self, json: &str) -> Result<KnowledgeBase, String> {
        self.jsonld.load_document(json)?;
        
        let mut kb = KnowledgeBase::new(
            "jsonld_kb",
            "JSON-LD Knowledge Base",
            KnowledgeBaseType::JsonLd,
        );
        
        for node in self.jsonld.extract_nodes() {
            kb.add_node(node);
        }
        
        self.bases.push(kb.clone());
        Ok(kb)
    }
    
    /// Общее количество баз
    pub fn base_count(&self) -> usize {
        self.bases.len()
    }
}
'''

with open('grammalang-core/src/social/kb_connectors.rs', 'w', encoding='utf-8') as f:
    f.write(connectors)
print("kb_connectors.rs created")

# 2. Обновляем mod.rs
mod_content = '''// src/social/mod.rs
pub mod knowledge_base;
pub mod kb_connectors;
pub mod collective_trace;
pub mod social_reactor;
pub mod federation;

pub use knowledge_base::*;
pub use kb_connectors::*;
pub use collective_trace::*;
pub use social_reactor::*;
pub use federation::*;
'''

with open('grammalang-core/src/social/mod.rs', 'w', encoding='utf-8') as f:
    f.write(mod_content)
print("mod.rs updated")

# 3. Тесты Фазы 1
tests = '''use grammalang_core::social::*;

#[test]
fn test_wikidata_connector() {
    let mut connector = WikidataConnector::new();
    
    let node = connector.fetch_entity("Q42").unwrap();
    assert_eq!(node.id, "wikidata:Q42");
    assert_eq!(connector.cache_size(), 1);
    
    // Повторная загрузка из кэша
    let cached = connector.fetch_entity("Q42").unwrap();
    assert_eq!(cached.id, node.id);
    
    println!("Wikidata: fetched {} (cache: {})", node.label, connector.cache_size());
}

#[test]
fn test_dbpedia_connector() {
    let mut connector = DBPediaConnector::new();
    
    let node = connector.fetch_resource("Berlin").unwrap();
    assert_eq!(node.id, "dbpedia:Berlin");
    assert_eq!(connector.cache_size(), 1);
    
    println!("DBPedia: fetched {} (cache: {})", node.label, connector.cache_size());
}

#[test]
fn test_jsonld_connector() {
    let mut connector = JsonLdConnector::new();
    
    let json = r#"{
        "@context": {"name": "http://schema.org/name"},
        "@graph": [
            {"@id": "http://example.org/1", "name": "Node 1", "description": "First node"},
            {"@id": "http://example.org/2", "name": "Node 2", "description": "Second node"}
        ]
    }"#;
    
    connector.load_document(json).unwrap();
    
    let nodes = connector.extract_nodes();
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].label, "Node 1");
    assert_eq!(nodes[1].label, "Node 2");
    
    println!("JSON-LD: loaded {} documents, extracted {} nodes", 
             connector.document_count(), nodes.len());
}

#[test]
fn test_knowledge_base_manager() {
    let mut manager = KnowledgeBaseManager::new();
    
    // Из Wikidata
    let wikidata_kb = manager.create_from_wikidata(&["Q42", "Q43"]);
    assert_eq!(wikidata_kb.metadata.node_count, 2);
    
    // Из DBPedia
    let dbpedia_kb = manager.create_from_dbpedia(&["Berlin", "Paris"]);
    assert_eq!(dbpedia_kb.metadata.node_count, 2);
    
    // Из JSON-LD
    let json = r#"{"@graph": [{"@id": "1", "name": "Test"}]}"#;
    let jsonld_kb = manager.create_from_jsonld(json).unwrap();
    assert_eq!(jsonld_kb.metadata.node_count, 1);
    
    assert_eq!(manager.base_count(), 3);
    
    println!("Manager: {} knowledge bases created", manager.base_count());
}
'''

with open('grammalang-core/tests/social/phase1_tests.rs', 'w', encoding='utf-8') as f:
    f.write(tests)
print("phase1_tests.rs created")

# 4. Обновляем тестовый mod.rs
test_mod = '''pub mod phase0_tests;
pub mod phase1_tests;
'''

with open('grammalang-core/tests/social/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll v0.8 Phase 1 files created!")
