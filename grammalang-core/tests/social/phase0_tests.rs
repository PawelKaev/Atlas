use grammalang_core::social::*;

#[test]
fn test_knowledge_base_creation() {
    let mut kb = KnowledgeBase::new("kb1", "Test KB", KnowledgeBaseType::Custom);
    
    let node = KnowledgeNode {
        id: "node1".to_string(),
        label: "Test Node".to_string(),
        description: "A test node".to_string(),
        properties: std::collections::HashMap::new(),
        relations: vec![],
    };
    
    kb.add_node(node);
    
    assert_eq!(kb.metadata.node_count, 1);
    assert!(kb.find_node("node1").is_some());
    println!("KnowledgeBase: {} nodes", kb.metadata.node_count);
}

#[test]
fn test_collective_trace() {
    let mut trace = CollectiveTrace::new();
    
    trace.add_participant(Participant {
        machine_id: "m1".to_string(),
        name: "Machine 1".to_string(),
        role: ParticipantRole::Leader,
        contribution: 10,
    });
    
    trace.record_event("m1", TraceEventType::Synthesis, "Test synthesis");
    
    assert_eq!(trace.participant_count(), 1);
    assert_eq!(trace.event_count(), 1);
    println!("CollectiveTrace: {} participants, {} events", 
             trace.participant_count(), trace.event_count());
}

#[test]
fn test_social_reactor() {
    let mut reactor = SocialReactor::new();
    
    reactor.add_contradiction(SocialContradiction {
        source_a: "kb1".to_string(),
        source_b: "kb2".to_string(),
        severity: 0.8,
        kind: SocialContradictionKind::KnowledgeConflict,
        context: None,
    });
    
    assert_eq!(reactor.active_count(), 1);
    
    let processed = reactor.process();
    assert_eq!(processed, 1);
    assert_eq!(reactor.processed_count(), 1);
    
    println!("SocialReactor: {} processed contradictions", processed);
}

#[test]
fn test_federation() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8000".to_string(),
        connected: true,
        capabilities: vec![],
    });
    
    fed.exchange("node1", "m1", "m2");
    
    assert_eq!(fed.member_count(), 1);
    assert_eq!(fed.exchange_count(), 1);
    
    println!("Federation: {} members, {} exchanges", 
             fed.member_count(), fed.exchange_count());
}
