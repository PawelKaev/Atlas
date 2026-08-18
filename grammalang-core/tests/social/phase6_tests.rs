use grammalang_core::social::*;
use grammalang_core::ontology::*;

/// Сценарий 1: Полный цикл социального синтеза
#[test]
fn test_full_social_cycle() {
    let mut integration = SocialIntegration::new();
    
    // 1. Загружаем знания из внешней базы
    let node1 = KnowledgeNode {
        id: "kb_plato".to_string(),
        label: "Platonic Idea".to_string(),
        description: "Transcendent form".to_string(),
        properties: std::collections::HashMap::new(),
        relations: vec![],
    };
    integration.knowledge_base.add_node(node1);
    
    // 2. Импортируем в машину
    let imported = integration.import_from_kb();
    assert_eq!(imported, 1);
    
    // 3. Добавляем второй узел
    integration.machine.add_node("Nietzschean Will", vec![]);
    integration.machine.metrics.stability_ratio = 0.3;
    integration.machine.metrics.contradiction_index = 0.8;
    
    // 4. Создаем противоречие
    let mut c = Contradiction::new(
        "Platonic Idea".to_string(),
        "Nietzschean Will".to_string(),
        ContradictionKind::Logical,
    );
    c.update_severity(0.8, 0.5);
    c.update_severity(0.8, 0.4);
    c.update_severity(0.8, 0.3);
    
    // 5. Социальный синтез
    let synthesis = integration.social_synthesis(
        "Platonic Idea",
        "Nietzschean Will",
        SynthesisStrategy::Hegelian,
    ).unwrap();
    
    assert!(!synthesis.name.is_empty());
    println!("Full social cycle: {}", synthesis.name);
}

/// Сценарий 2: Распределенный синтез через федерацию
#[test]
fn test_distributed_synthesis() {
    let mut integration = SocialIntegration::new();
    
    // Добавляем участников федерации
    integration.federation.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![MemberCapability::Synthesis],
    });
    
    integration.federation.add_member(FederationMember {
        machine_id: "m2".to_string(),
        address: "localhost:8002".to_string(),
        connected: true,
        capabilities: vec![MemberCapability::KnowledgeStorage],
    });
    
    // Обмен узлами
    integration.federated_exchange("m1", "m2", "synthesis_node");
    
    // Проверяем консенсус
    integration.federation.start_consensus();
    integration.federation.vote("m1", true);
    integration.federation.vote("m2", true);
    
    assert!(integration.federation.has_consensus());
    assert_eq!(integration.federation.exchange_count(), 1);
    
    println!("Distributed synthesis: consensus reached, {} exchanges", 
             integration.federation.exchange_count());
}

/// Сценарий 3: Конфликт знаний между базами
#[test]
fn test_knowledge_conflict_resolution() {
    let mut integration = SocialIntegration::new();
    
    // Противоречие между двумя базами
    let contradiction = SocialContradiction {
        source_a: "wikidata:Q42".to_string(),
        source_b: "dbpedia:Berlin".to_string(),
        severity: 0.75,
        kind: SocialContradictionKind::KnowledgeConflict,
        context: None,
    };
    
    let reaction = integration.social_reactor.process_one(contradiction);
    
    assert!(matches!(reaction.result, ReactionResult::Success));
    
    println!("Knowledge conflict: {:?} -> {:?}", 
             reaction.strategy_used, reaction.result);
}

/// Сценарий 4: Коллективный trace с множеством участников
#[test]
fn test_collective_trace_multiple_participants() {
    let mut trace = CollectiveTrace::new();
    
    // Добавляем 3 участника
    for i in 1..=3 {
        trace.add_participant(Participant {
            machine_id: format!("m{}", i),
            name: format!("Machine {}", i),
            role: if i == 1 { ParticipantRole::Leader } else { ParticipantRole::Member },
            contribution: i * 5,
        });
    }
    
    // Каждый добавляет узлы
    for i in 1..=3 {
        trace.record_event(
            &format!("m{}", i),
            TraceEventType::NodeAdded,
            &format!("node_from_m{}", i),
        );
    }
    
    // Слияние trace от другой машины
    let mut other_trace = CollectiveTrace::new();
    other_trace.protocol_version = trace.protocol_version.clone();
    other_trace.add_participant(Participant {
        machine_id: "m4".to_string(),
        name: "Machine 4".to_string(),
        role: ParticipantRole::Observer,
        contribution: 1,
    });
    other_trace.record_event("m4", TraceEventType::NodeAdded, "node_from_m4");
    
    let merged = trace.merge(&other_trace).unwrap();
    
    assert_eq!(trace.participant_count(), 4);
    assert!(merged > 0);
    
    println!("Collective trace: {} participants, {} events after merge", 
             trace.participant_count(), trace.event_count());
}

/// Сценарий 5: Социальный мост с режимом Архитектор
#[test]
fn test_architect_social_bridge() {
    let mut bridge = SocialBridge::new();
    
    bridge.integration.machine.add_node("freedom", vec![]);
    bridge.integration.machine.add_node("security", vec![]);
    bridge.integration.machine.metrics.stability_ratio = 0.3;
    bridge.integration.machine.metrics.contradiction_index = 0.8;
    
    let result = bridge.run_architect_social(
        "freedom",
        "security",
        SynthesisStrategy::Hegelian,
        Some("responsible_freedom"),
    );
    
    assert!(result.success);
    println!("Architect social: {}", result.message);
}

/// Сценарий 6: Массовый обмен через федерацию
#[test]
fn test_mass_exchange() {
    let mut integration = SocialIntegration::new();
    
    // Добавляем 5 участников
    for i in 1..=5 {
        integration.federation.add_member(FederationMember {
            machine_id: format!("m{}", i),
            address: format!("localhost:800{}", i),
            connected: true,
            capabilities: vec![],
        });
    }
    
    // Массовый обмен
    for i in 1..=5 {
        integration.federated_exchange(
            &format!("m{}", i),
            "m1",
            &format!("node_{}", i),
        );
    }
    
    assert_eq!(integration.federation.exchange_count(), 5);
    assert_eq!(integration.federation.member_count(), 5);
    
    println!("Mass exchange: {} exchanges among {} members", 
             integration.federation.exchange_count(),
             integration.federation.member_count());
}

/// Сценарий 7: Синхронизация большого количества узлов
#[test]
fn test_large_sync() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![],
    });
    
    fed.add_member(FederationMember {
        machine_id: "m2".to_string(),
        address: "localhost:8002".to_string(),
        connected: true,
        capabilities: vec![],
    });
    
    // 100 узлов на синхронизацию
    let nodes: Vec<String> = (0..100).map(|i| format!("node_{}", i)).collect();
    
    fed.request_sync("m1", "m2", nodes);
    
    assert_eq!(fed.sync_queue_size(), 1);
    
    let processed = fed.process_sync_queue();
    
    assert_eq!(processed, 1);
    assert_eq!(fed.exchange_count(), 100);
    
    println!("Large sync: {} nodes exchanged", fed.exchange_count());
}
