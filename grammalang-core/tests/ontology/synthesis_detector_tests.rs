use grammalang_core::ontology::*;

#[test]
fn test_detector_empty() {
    let detector = SynthesisDetector::new();
    let contradictions = vec![];
    
    let candidates = detector.detect(&contradictions);
    assert!(candidates.is_empty());
}

#[test]
fn test_detector_no_ready_contradictions() {
    let detector = SynthesisDetector::new();
    
    let mut contradiction = Contradiction::new(
        "a".to_string(),
        "b".to_string(),
        ContradictionKind::Logical,
    );
    
    // Низкая severity
    contradiction.update_severity(0.3, 0.8);
    contradiction.update_severity(0.3, 0.8);
    contradiction.update_severity(0.3, 0.8);
    
    let contradictions = vec![contradiction];
    let candidates = detector.detect(&contradictions);
    
    assert!(candidates.is_empty());
}

#[test]
fn test_detector_ready_contradictions() {
    let detector = SynthesisDetector::new();
    
    let mut contradiction = Contradiction::new(
        "plato_idea".to_string(),
        "nietzsche_will".to_string(),
        ContradictionKind::Logical,
    );
    
    // Высокая severity и падение стабильности
    contradiction.update_severity(0.8, 0.6);
    contradiction.update_severity(0.8, 0.5);
    contradiction.update_severity(0.8, 0.4);
    
    let contradictions = vec![contradiction];
    let candidates = detector.detect(&contradictions);
    
    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].source_nodes.len(), 2);
    assert_eq!(candidates[0].source_nodes[0], "plato_idea");
    assert_eq!(candidates[0].source_nodes[1], "nietzsche_will");
}

#[test]
fn test_detector_multiple_contradictions() {
    let detector = SynthesisDetector::new();
    
    let mut contradictions = Vec::new();
    
    // Готовое противоречие
    let mut c1 = Contradiction::new(
        "capitalism".to_string(),
        "ecology".to_string(),
        ContradictionKind::Structural,
    );
    c1.update_severity(0.7, 0.5);
    c1.update_severity(0.7, 0.4);
    c1.update_severity(0.7, 0.3);
    contradictions.push(c1);
    
    // Не готовое противоречие
    let mut c2 = Contradiction::new(
        "freedom".to_string(),
        "security".to_string(),
        ContradictionKind::Logical,
    );
    c2.update_severity(0.4, 0.7);
    c2.update_severity(0.4, 0.7);
    c2.update_severity(0.4, 0.7);
    contradictions.push(c2);
    
    let candidates = detector.detect(&contradictions);
    
    assert_eq!(candidates.len(), 1);
}

#[test]
fn test_detector_threshold_configuration() {
    let mut detector = SynthesisDetector::new();
    detector.threshold_contradiction = 0.8;
    
    let mut contradiction = Contradiction::new(
        "a".to_string(),
        "b".to_string(),
        ContradictionKind::Logical,
    );
    
    contradiction.update_severity(0.7, 0.5);
    contradiction.update_severity(0.7, 0.4);
    contradiction.update_severity(0.7, 0.3);
    
    let contradictions = vec![contradiction];
    let candidates = detector.detect(&contradictions);
    
    assert!(candidates.is_empty());
}
