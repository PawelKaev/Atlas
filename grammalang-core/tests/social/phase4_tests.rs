use grammalang_core::social::*;

#[test]
fn test_federation_basic() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![MemberCapability::Synthesis],
    });
    
    fed.add_member(FederationMember {
        machine_id: "m2".to_string(),
        address: "localhost:8002".to_string(),
        connected: true,
        capabilities: vec![MemberCapability::KnowledgeStorage],
    });
    
    assert_eq!(fed.member_count(), 2);
    println!("Federation: {} members", fed.member_count());
}

#[test]
fn test_node_exchange() {
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
    
    fed.exchange("node1", "m1", "m2");
    fed.exchange("node2", "m2", "m1");
    
    assert_eq!(fed.exchange_count(), 2);
    println!("Exchanges: {}", fed.exchange_count());
}

#[test]
fn test_sync_queue() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![],
    });
    
    fed.request_sync("m1", "m2", vec!["node1".to_string(), "node2".to_string()]);
    
    assert_eq!(fed.sync_queue_size(), 1);
    
    let processed = fed.process_sync_queue();
    
    assert_eq!(processed, 1);
    assert_eq!(fed.exchange_count(), 2);
    
    println!("Sync: {} requests processed, {} exchanges", 
             processed, fed.exchange_count());
}

#[test]
fn test_consensus() {
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
    
    fed.start_consensus();
    fed.vote("m1", true);
    fed.vote("m2", true);
    
    assert!(fed.has_consensus());
    println!("Consensus reached in round {}", fed.consensus.round);
}

#[test]
fn test_capabilities() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![MemberCapability::Synthesis, MemberCapability::KnowledgeStorage],
    });
    
    assert!(fed.has_capability("m1", &MemberCapability::Synthesis));
    assert!(fed.has_capability("m1", &MemberCapability::KnowledgeStorage));
    assert!(!fed.has_capability("m1", &MemberCapability::ContradictionProcessing));
    
    println!("Capabilities checked correctly");
}

#[test]
fn test_message_protocol() {
    let mut protocol = MessageProtocol::new();
    
    protocol.send("m1", "m2", MessageType::NodeRequest, "node1");
    protocol.send("m2", "m1", MessageType::NodeResponse, "node1_data");
    protocol.send("m1", "all", MessageType::Announcement, "new_node_available");
    
    assert_eq!(protocol.message_count(), 3);
    
    let for_m2 = protocol.receive("m2");
    assert_eq!(for_m2.len(), 1);
    
    let for_m1 = protocol.receive("m1");
    assert_eq!(for_m1.len(), 1);
    
    println!("Message protocol: {} messages", protocol.message_count());
}

#[test]
fn test_federation_stats() {
    let mut fed = Federation::new();
    
    fed.add_member(FederationMember {
        machine_id: "m1".to_string(),
        address: "localhost:8001".to_string(),
        connected: true,
        capabilities: vec![],
    });
    
    fed.exchange("node1", "m1", "m2");
    fed.request_sync("m1", "m2", vec!["node2".to_string()]);
    
    let stats = fed.stats();
    
    assert_eq!(stats.members, 1);
    assert_eq!(stats.exchanges, 1);
    assert_eq!(stats.sync_queue, 1);
    
    println!("Stats: {} members, {} exchanges, {} in queue", 
             stats.members, stats.exchanges, stats.sync_queue);
}
