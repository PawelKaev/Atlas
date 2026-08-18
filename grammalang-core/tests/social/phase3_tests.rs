use grammalang_core::social::*;

#[test]
fn test_social_reactor_basic() {
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
    
    println!("Basic reactor: {} processed", processed);
}

#[test]
fn test_resolution_strategies() {
    let mut reactor = SocialReactor::new();
    
    // Добавляем доменную стратегию
    reactor.add_strategy("philosophy", ResolutionStrategy::Synthesize);
    
    let contradiction = SocialContradiction {
        source_a: "plato".to_string(),
        source_b: "nietzsche".to_string(),
        severity: 0.7,
        kind: SocialContradictionKind::MachineConflict,
        context: Some(ContradictionContext {
            domain: "philosophy".to_string(),
            participants: vec!["m1".to_string()],
            timestamp: 0,
        }),
    };
    
    let reaction = reactor.process_one(contradiction);
    
    match reaction.strategy_used {
        ResolutionStrategy::Synthesize => assert!(true),
        _ => panic!("Expected Synthesize strategy"),
    }
    
    println!("Strategy used: {:?}", reaction.strategy_used);
}

#[test]
fn test_default_strategies() {
    let mut reactor = SocialReactor::new();
    
    // KnowledgeConflict -> Merge
    let c1 = SocialContradiction {
        source_a: "a".to_string(),
        source_b: "b".to_string(),
        severity: 0.7,
        kind: SocialContradictionKind::KnowledgeConflict,
        context: None,
    };
    let r1 = reactor.process_one(c1);
    assert!(matches!(r1.strategy_used, ResolutionStrategy::Merge));
    
    // MachineConflict -> Synthesize
    let c2 = SocialContradiction {
        source_a: "a".to_string(),
        source_b: "b".to_string(),
        severity: 0.7,
        kind: SocialContradictionKind::MachineConflict,
        context: None,
    };
    let r2 = reactor.process_one(c2);
    assert!(matches!(r2.strategy_used, ResolutionStrategy::Synthesize));
    
    println!("Default strategies work correctly");
}

#[test]
fn test_reactor_metrics() {
    let mut reactor = SocialReactor::new();
    
    reactor.add_contradiction(SocialContradiction {
        source_a: "a".to_string(),
        source_b: "b".to_string(),
        severity: 0.8,
        kind: SocialContradictionKind::KnowledgeConflict,
        context: None,
    });
    
    reactor.add_contradiction(SocialContradiction {
        source_a: "c".to_string(),
        source_b: "d".to_string(),
        severity: 0.7,
        kind: SocialContradictionKind::MachineConflict,
        context: None,
    });
    
    reactor.process();
    
    let metrics = reactor.get_metrics();
    
    assert_eq!(metrics.total_contradictions, 2);
    assert_eq!(metrics.resolved, 2);
    
    println!("Metrics: {} total, {} resolved", 
             metrics.total_contradictions, metrics.resolved);
}

#[test]
fn test_distributed_reactor() {
    let mut distributed = DistributedReactor::new();
    
    distributed.add_reactor("m1", SocialReactor::new());
    distributed.add_reactor("m2", SocialReactor::new());
    
    assert_eq!(distributed.reactor_count(), 2);
    
    let contradiction = SocialContradiction {
        source_a: "a".to_string(),
        source_b: "b".to_string(),
        severity: 0.8,
        kind: SocialContradictionKind::KnowledgeConflict,
        context: None,
    };
    
    let reaction = distributed.process_on("m1", contradiction).unwrap();
    assert!(matches!(reaction.result, ReactionResult::Success));
    
    let stats = distributed.total_stats();
    assert_eq!(stats.len(), 2);
    
    println!("Distributed: {} reactors, processed on m1", 
             distributed.reactor_count());
}

#[test]
fn test_threshold_filtering() {
    let mut reactor = SocialReactor::new();
    reactor.collective_threshold = 0.7;
    
    // Ниже порога - не добавится
    reactor.add_contradiction(SocialContradiction {
        source_a: "a".to_string(),
        source_b: "b".to_string(),
        severity: 0.5,
        kind: SocialContradictionKind::KnowledgeConflict,
        context: None,
    });
    
    assert_eq!(reactor.active_count(), 0);
    
    // Выше порога - добавится
    reactor.add_contradiction(SocialContradiction {
        source_a: "c".to_string(),
        source_b: "d".to_string(),
        severity: 0.9,
        kind: SocialContradictionKind::KnowledgeConflict,
        context: None,
    });
    
    assert_eq!(reactor.active_count(), 1);
    
    println!("Threshold filtering works: {} active", reactor.active_count());
}
