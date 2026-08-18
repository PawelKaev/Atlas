use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Внешняя база знаний
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    /// Идентификатор базы
    pub id: String,
    
    /// Название
    pub name: String,
    
    /// Тип базы
    pub kb_type: KnowledgeBaseType,
    
    /// Узлы знаний
    pub nodes: Vec<KnowledgeNode>,
    
    /// Метаданные
    pub metadata: KnowledgeBaseMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnowledgeBaseType {
    /// Wikidata
    Wikidata,
    /// DBPedia
    DBPedia,
    /// Локальная JSON-LD
    JsonLd,
    /// Пользовательская
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    /// Идентификатор узла
    pub id: String,
    
    /// Название
    pub label: String,
    
    /// Описание
    pub description: String,
    
    /// Свойства
    pub properties: HashMap<String, String>,
    
    /// Связи с другими узлами
    pub relations: Vec<KnowledgeRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeRelation {
    /// Целевой узел
    pub target: String,
    
    /// Тип связи
    pub relation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBaseMetadata {
    /// Версия базы
    pub version: String,
    
    /// Дата обновления
    pub updated_at: String,
    
    /// Количество узлов
    pub node_count: usize,
    
    /// Источник
    pub source_url: Option<String>,
}

impl KnowledgeBase {
    pub fn new(id: &str, name: &str, kb_type: KnowledgeBaseType) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            kb_type,
            nodes: Vec::new(),
            metadata: KnowledgeBaseMetadata {
                version: "0.1".to_string(),
                updated_at: String::new(),
                node_count: 0,
                source_url: None,
            },
        }
    }
    
    /// Добавление узла
    pub fn add_node(&mut self, node: KnowledgeNode) {
        self.nodes.push(node);
        self.metadata.node_count = self.nodes.len();
    }
    
    /// Поиск узла по ID
    pub fn find_node(&self, id: &str) -> Option<&KnowledgeNode> {
        self.nodes.iter().find(|n| n.id == id)
    }
    
    /// Поиск узлов по свойству
    pub fn find_by_property(&self, key: &str, value: &str) -> Vec<&KnowledgeNode> {
        self.nodes.iter()
            .filter(|n| n.properties.get(key) == Some(&value.to_string()))
            .collect()
    }
    
    /// Получение всех связей узла
    pub fn get_relations(&self, node_id: &str) -> Vec<&KnowledgeRelation> {
        self.nodes.iter()
            .filter(|n| n.id == node_id)
            .flat_map(|n| n.relations.iter())
            .collect()
    }
}
