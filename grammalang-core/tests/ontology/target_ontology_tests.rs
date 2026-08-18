// tests/ontology/target_ontology_tests.rs
use grammalang_core::ontology::*;

#[test]
fn test_target_ontology_creation() {
    let ontology = TargetOntology::new(
        vec!["idea_good".to_string(), "will_to_power".to_string()],
        ContradictionType::Direct,
        SynthesisStrategy::Hegelian,
    );
    
    assert!(ontology.is_ready());
    assert_eq!(ontology.source_nodes.len(), 2);
}

#[test]
fn test_contradiction_index() {
    let mut contradiction = Contradiction::new(
        "a".to_string(),
        "b".to_string(),
        ContradictionKind::Logical,
    );
    
    // Имитация падения стабильности
    contradiction.update_severity(0.7, 0.5);
    contradiction.update_severity(0.8, 0.4);
    contradiction.update_severity(0.9, 0.3);
    
    assert!(contradiction.is_ready_for_synthesis(0.6));
    assert!(contradiction.contradiction_index > 0.6);
}

#[test]
fn test_synthesis_strategies() {
    let strategies = vec![
        SynthesisStrategy::Hegelian,
        SynthesisStrategy::Plotinian,
        SynthesisStrategy::Pragmatic,
    ];
    
    for strategy in strategies {
        let ontology = TargetOntology::new(
            vec!["a".to_string(), "b".to_string()],
            ContradictionType::Direct,
            strategy,
        );
        
        assert!(ontology.is_ready());
    }
}
