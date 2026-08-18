use grammalang_core::ontology::*;

#[test]
fn test_basic_ontology() {
    // Тест Contradiction
    let mut c = Contradiction::new(
        "a".to_string(),
        "b".to_string(),
        ContradictionKind::Logical,
    );
    
    c.update_severity(0.8, 0.5);
    c.update_severity(0.8, 0.4);
    c.update_severity(0.8, 0.3);
    
    assert!(c.is_ready_for_synthesis(0.6));
    println!("✓ Contradiction index: {:.2}", c.contradiction_index);
    
    // Тест TargetOntology
    let o = TargetOntology::new(
        vec!["a".to_string(), "b".to_string()],
        ContradictionType::Direct,
        SynthesisStrategy::Hegelian,
    );
    
    assert!(o.is_ready());
    println!("✓ Ontology ready: {}", o.is_ready());
    
    // Тест старого API
    let mut engine = OntologyEngine::new();
    let entity = Entity {
        name: "test".to_string(),
        category: Category::Concept,
        properties: vec![],
    };
    engine.add_entity(entity);
    assert_eq!(engine.entity_count(), 1);
    println!("✓ Engine entities: {}", engine.entity_count());
}

#[test]
fn test_detector() {
    let detector = SynthesisDetector::new();
    
    let mut c = Contradiction::new(
        "x".to_string(),
        "y".to_string(),
        ContradictionKind::Structural,
    );
    
    c.update_severity(0.7, 0.5);
    c.update_severity(0.7, 0.4);
    c.update_severity(0.7, 0.3);
    
    let candidates = detector.detect(&[c]);
    assert_eq!(candidates.len(), 1);
    println!("✓ Candidates found: {}", candidates.len());
}

#[test]
fn test_strategies() {
    let strategies = vec![
        SynthesisStrategy::Hegelian,
        SynthesisStrategy::Plotinian,
        SynthesisStrategy::Pragmatic,
    ];
    
    for strategy in strategies {
        let o = TargetOntology::new(
            vec!["a".to_string(), "b".to_string()],
            ContradictionType::Direct,
            strategy,
        );
        assert!(o.is_ready());
    }
    println!("✓ All strategies work");
}

#[test]
fn test_contradiction_kinds() {
    let kinds = vec![
        ContradictionKind::Logical,
        ContradictionKind::Structural,
        ContradictionKind::Temporal,
        ContradictionKind::Recursive,
    ];
    
    for kind in kinds {
        let c = Contradiction::new(
            "a".to_string(),
            "b".to_string(),
            kind,
        );
        assert_eq!(c.severity, 0.0);
    }
    println!("✓ All contradiction kinds work");
}

#[test]
fn test_axis_spec() {
    let axis = AxisSpec {
        axis_id: "moral_axis".to_string(),
        axis_name: "Moral axis".to_string(),
        poles: ("good".to_string(), "evil".to_string()),
        transformation_type: AxisTransformation::Create,
    };
    
    let o = TargetOntology::new(
        vec!["a".to_string(), "b".to_string()],
        ContradictionType::Direct,
        SynthesisStrategy::Hegelian,
    ).with_target_axis(axis);
    
    assert!(o.target_axis.is_some());
    println!("✓ Axis spec works");
}

#[test]
fn test_contradiction_index_calculation() {
    let mut contradiction = Contradiction::new(
        "freedom".to_string(),
        "security".to_string(),
        ContradictionKind::Logical,
    );
    
    // Без падения стабильности
    contradiction.update_severity(0.8, 0.6);
    contradiction.update_severity(0.8, 0.6);
    contradiction.update_severity(0.8, 0.6);
    
    let index_without_drop = contradiction.contradiction_index;
    println!("Index without stability drop: {:.2}", index_without_drop);
    
    // С падением стабильности
    let mut contradiction2 = Contradiction::new(
        "freedom".to_string(),
        "security".to_string(),
        ContradictionKind::Logical,
    );
    
    contradiction2.update_severity(0.8, 0.6);
    contradiction2.update_severity(0.8, 0.5);
    contradiction2.update_severity(0.8, 0.4);
    
    let index_with_drop = contradiction2.contradiction_index;
    println!("Index with stability drop: {:.2}", index_with_drop);
    
    assert!(index_with_drop > index_without_drop);
    println!("✓ Stability drop increases contradiction index");
}
