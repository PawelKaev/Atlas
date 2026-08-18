use grammalang_core::social::*;
use grammalang_core::ontology::*;

#[test]
fn test_import_from_kb() {
    let mut integration = SocialIntegration::new();
    
    // Добавляем узлы в KB
    let node = KnowledgeNode {
        id: "kb_node1".to_string(),
        label: "Knowledge Node".to_string(),
        description: "From KB".to_string(),
        properties: std::collections::HashMap::new(),
        relations: vec![],
    };
    integration.knowledge_base.add_node(node);
    
    // Импортируем в машину
    let imported = integration.import_from_kb();
    
    assert_eq!(imported, 1);
    assert_eq!(integration.machine.nodes.len(), 1);
    
    println!("Imported {} nodes from KB", imported);
}

#[test]
fn test_export_to_kb() {
    let mut integration = SocialIntegration::new();
    
    // Добавляем узлы в машину
    integration.machine.add_node("machine_node", vec!["prop1".to_string()]);
    
    // Экспортируем в KB
    let exported = integration.export_to_kb();
    
    assert_eq!(exported, 1);
    assert_eq!(integration.knowledge_base.metadata.node_count, 1);
    
    println!("Exported {} nodes to KB", exported);
}

#[test]
fn test_social_synthesis() {
    let mut integration = SocialIntegration::new();
    
    let result = integration.social_synthesis(
        "freedom",
        "security",
        SynthesisStrategy::Hegelian,
    ).unwrap();
    
    assert!(!result.name.is_empty());
    assert!(integration.collective_trace.event_count() > 0);
    
    println!("Social synthesis: {}", result.name);
}

#[test]
fn test_federated_exchange() {
    let mut integration = SocialIntegration::new();
    
    integration.federated_exchange("m1", "m2", "node1");
    
    assert_eq!(integration.federation.exchange_count(), 1);
    assert!(integration.collective_trace.event_count() > 0);
    
    println!("Federated exchange completed");
}

#[test]
fn test_integration_stats() {
    let mut integration = SocialIntegration::new();
    
    integration.machine.add_node("node1", vec![]);
    integration.knowledge_base.add_node(KnowledgeNode {
        id: "kb1".to_string(),
        label: "KB Node".to_string(),
        description: "test".to_string(),
        properties: std::collections::HashMap::new(),
        relations: vec![],
    });
    
    let stats = integration.integration_stats();
    
    assert_eq!(stats.machine_nodes, 1);
    assert_eq!(stats.kb_nodes, 1);
    
    println!("Stats: {} machine nodes, {} KB nodes", 
             stats.machine_nodes, stats.kb_nodes);
}

#[test]
fn test_social_bridge() {
    let mut bridge = SocialBridge::new();
    
    // Добавляем узлы
    bridge.integration.machine.add_node("freedom", vec![]);
    bridge.integration.machine.add_node("security", vec![]);
    bridge.integration.machine.metrics.stability_ratio = 0.3;
    bridge.integration.machine.metrics.contradiction_index = 0.8;
    
    // Создаем противоречие
    let mut c = Contradiction::new(
        "freedom".to_string(),
        "security".to_string(),
        ContradictionKind::Logical,
    );
    c.update_severity(0.8, 0.5);
    c.update_severity(0.8, 0.4);
    c.update_severity(0.8, 0.3);
    
    // Запускаем Платона с социальным контекстом
    let result = bridge.run_plato_social(&[c]);
    
    assert!(result.iterations > 0);
    println!("Social bridge: {} iterations", result.iterations);
}
