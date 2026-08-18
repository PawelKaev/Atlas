use grammalang_core::social::*;

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
