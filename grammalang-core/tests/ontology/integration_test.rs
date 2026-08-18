use grammalang_core::ontology::*;

#[test]
fn test_full_synthesis_pipeline_stub() {
    // Создаем противоречие
    let mut contradiction = Contradiction::new(
        "hegel_spirit".to_string(),
        "kierkegaard_existence".to_string(),
        ContradictionKind::Logical,
    );
    
    // Имитируем накопление напряжения
    contradiction.update_severity(0.6, 0.7);
    contradiction.update_severity(0.7, 0.6);
    contradiction.update_severity(0.8, 0.5);
    contradiction.update_severity(0.9, 0.4);
    
    // Проверяем готовность
    assert!(contradiction.is_ready_for_synthesis(0.6));
    
    // Создаем детектор
    let detector = SynthesisDetector::new();
    let candidates = detector.detect(&[contradiction]);
    
    assert_eq!(candidates.len(), 1);
    
    // Создаем TargetOntology
    let ontology = TargetOntology::new(
        candidates[0].source_nodes.clone(),
        candidates[0].contradiction_type.clone(),
        SynthesisStrategy::Hegelian,
    );
    
    assert!(ontology.is_ready());
    
    // Проверяем выбранную стратегию
    let selector = HeuristicSelector;
    let strategy = selector.select(&candidates[0]);
    
    match strategy {
        SynthesisStrategy::Hegelian => assert!(true),
        _ => assert!(false, "Expected Hegelian strategy"),
    }
}

#[test]
fn test_philosophical_scenarios() {
    // Сценарий 1: Платон vs Ницше
    let mut c1 = Contradiction::new(
        "platonic_idea".to_string(),
        "nietzschean_will".to_string(),
        ContradictionKind::Logical,
    );
    c1.update_severity(0.8, 0.6);
    c1.update_severity(0.8, 0.5);
    c1.update_severity(0.8, 0.4);
    assert!(c1.is_ready_for_synthesis(0.6));
    
    // Сценарий 2: Капитализм vs Экология
    let mut c2 = Contradiction::new(
        "capitalism".to_string(),
        "ecology".to_string(),
        ContradictionKind::Structural,
    );
    c2.update_severity(0.7, 0.5);
    c2.update_severity(0.7, 0.4);
    c2.update_severity(0.7, 0.3);
    assert!(c2.is_ready_for_synthesis(0.6));
    
    // Сценарий 3: Свобода vs Безопасность
    let mut c3 = Contradiction::new(
        "freedom".to_string(),
        "security".to_string(),
        ContradictionKind::Logical,
    );
    c3.update_severity(0.9, 0.5);
    c3.update_severity(0.9, 0.4);
    c3.update_severity(0.9, 0.3);
    assert!(c3.is_ready_for_synthesis(0.6));
    
    // Сценарий 4: Пустая машина
    let empty_contradictions: Vec<Contradiction> = vec![];
    let detector = SynthesisDetector::new();
    let candidates = detector.detect(&empty_contradictions);
    assert!(candidates.is_empty());
}
