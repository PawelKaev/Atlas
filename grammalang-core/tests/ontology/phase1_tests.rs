use grammalang_core::ontology::*;
use std::collections::HashMap;

#[test]
fn test_detect_mediated_contradiction() {
    let detector = SynthesisDetector::new();
    
    let mut c = Contradiction::new(
        "capitalism".to_string(),
        "ecology".to_string(),
        ContradictionKind::Structural,
    );
    
    c.resolution_candidates = vec!["green_economy".to_string()];
    
    c.update_severity(0.7, 0.5);
    c.update_severity(0.7, 0.4);
    c.update_severity(0.7, 0.3);
    
    let candidates = detector.detect(&[c]);
    
    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].mediators.len(), 1);
    
    match &candidates[0].contradiction_type {
        ContradictionType::Mediated { mediator } => {
            assert_eq!(mediator, "green_economy");
        }
        _ => panic!("Expected Mediated type"),
    }
    
    println!("✓ Mediated contradiction detected");
}

#[test]
fn test_detect_recursive_contradiction() {
    let detector = SynthesisDetector::new();
    
    let mut c = Contradiction::new(
        "self".to_string(),
        "self".to_string(),
        ContradictionKind::Recursive,
    );
    
    c.update_severity(0.8, 0.5);
    c.update_severity(0.8, 0.4);
    c.update_severity(0.8, 0.3);
    
    let candidates = detector.detect(&[c]);
    
    assert_eq!(candidates.len(), 1);
    
    match candidates[0].strategy_hint {
        SynthesisStrategy::Plotinian => assert!(true),
        _ => panic!("Expected Plotinian for recursive"),
    }
    
    println!("✓ Recursive contradiction detected");
}

#[test]
fn test_sorting_by_contradiction_index() {
    let detector = SynthesisDetector::new();
    
    let mut c1 = Contradiction::new(
        "a".to_string(), "b".to_string(),
        ContradictionKind::Logical,
    );
    c1.update_severity(0.9, 0.5);
    c1.update_severity(0.9, 0.4);
    c1.update_severity(0.9, 0.3);
    
    let mut c2 = Contradiction::new(
        "c".to_string(), "d".to_string(),
        ContradictionKind::Logical,
    );
    c2.update_severity(0.7, 0.5);
    c2.update_severity(0.7, 0.4);
    c2.update_severity(0.7, 0.3);
    
    let candidates = detector.detect(&[c1, c2]);
    
    assert_eq!(candidates.len(), 2);
    assert!(candidates[0].metrics_before.contradiction_index > 
            candidates[1].metrics_before.contradiction_index);
    
    println!("✓ Candidates sorted");
}

#[test]
fn test_context_aware_detection() {
    let mut weights = HashMap::new();
    weights.insert("hegelian_bias".to_string(), 0.9);
    
    let detector = ContextAwareDetector::new(weights);
    
    let mut c = Contradiction::new(
        "thesis".to_string(), "antithesis".to_string(),
        ContradictionKind::Logical,
    );
    
    c.update_severity(0.8, 0.5);
    c.update_severity(0.8, 0.4);
    c.update_severity(0.8, 0.3);
    
    let candidates = detector.detect_with_context(&[c]);
    
    assert_eq!(candidates.len(), 1);
    
    match candidates[0].strategy_hint {
        SynthesisStrategy::Hegelian => assert!(true),
        _ => panic!("Expected Hegelian due to context"),
    }
    
    println!("✓ Context-aware detection works");
}

#[test]
fn test_adaptive_selector() {
    let mut selector = AdaptiveSelector::new();
    
    selector.record_result(SynthesisStrategy::Hegelian, true, 0.3);
    selector.record_result(SynthesisStrategy::Hegelian, true, 0.2);
    selector.record_result(SynthesisStrategy::Pragmatic, false, -0.1);
    
    let candidate = SynthesisCandidate {
        source_nodes: vec!["a".to_string(), "b".to_string()],
        contradiction_type: ContradictionType::Direct,
        strategy_hint: SynthesisStrategy::Pragmatic,
        metrics_before: MetricsSnapshot::default(),
        mediators: vec![],
        genealogy: vec![],
        tension_duration: 3,
    };
    
    match selector.select(&candidate) {
        SynthesisStrategy::Hegelian => assert!(true),
        _ => panic!("Expected Hegelian due to history"),
    }
    
    println!("✓ Adaptive selector works");
}
