use grammalang_core::ontology::*;

#[test]
fn test_contradiction_creation() {
    let contradiction = Contradiction::new(
        "thesis".to_string(),
        "antithesis".to_string(),
        ContradictionKind::Logical,
    );
    
    assert_eq!(contradiction.node_a, "thesis");
    assert_eq!(contradiction.node_b, "antithesis");
    assert_eq!(contradiction.severity, 0.0);
    assert_eq!(contradiction.contradiction_index, 0.0);
    assert!(contradiction.resolution_candidates.is_empty());
    assert!(contradiction.genealogy.is_empty());
}

#[test]
fn test_contradiction_severity_update() {
    let mut contradiction = Contradiction::new(
        "a".to_string(),
        "b".to_string(),
        ContradictionKind::Structural,
    );
    
    contradiction.update_severity(0.7, 0.5);
    contradiction.update_severity(0.8, 0.4);
    contradiction.update_severity(0.9, 0.3);
    
    assert_eq!(contradiction.severity, 0.9);
    assert_eq!(contradiction.severity_history.len(), 3);
    assert!(contradiction.contradiction_index > 0.6);
}

#[test]
fn test_contradiction_readiness() {
    let mut contradiction = Contradiction::new(
        "capitalism".to_string(),
        "ecology".to_string(),
        ContradictionKind::Structural,
    );
    
    // Не готов к синтезу (мало истории)
    contradiction.update_severity(0.9, 0.5);
    assert!(!contradiction.is_ready_for_synthesis(0.6));
    
    // Добавляем больше записей с падением стабильности
    contradiction.update_severity(0.9, 0.4);
    contradiction.update_severity(0.9, 0.3);
    contradiction.update_severity(0.9, 0.2);
    
    assert!(contradiction.is_ready_for_synthesis(0.6));
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
    
    // Индекс должен быть около 0.56 (0.7 * 0.8 + 0.3 * 0)
    assert!(contradiction.contradiction_index < 0.6);
    assert!((contradiction.contradiction_index - 0.56).abs() < 0.001);
    
    // С падением стабильности
    let mut contradiction2 = Contradiction::new(
        "freedom".to_string(),
        "security".to_string(),
        ContradictionKind::Logical,
    );
    
    contradiction2.update_severity(0.8, 0.6);
    contradiction2.update_severity(0.8, 0.5);
    contradiction2.update_severity(0.8, 0.4);
    
    // Индекс должен быть выше 0.6
    assert!(contradiction2.contradiction_index > 0.6);
    assert!((contradiction2.contradiction_index - 0.86).abs() < 0.001);
}

#[test]
fn test_contradiction_kinds() {
    let logical = ContradictionKind::Logical;
    let structural = ContradictionKind::Structural;
    let temporal = ContradictionKind::Temporal;
    let recursive = ContradictionKind::Recursive;
    
    // Проверяем, что все варианты создаются
    match logical {
        ContradictionKind::Logical => assert!(true),
        _ => assert!(false, "Expected Logical"),
    }
    
    match structural {
        ContradictionKind::Structural => assert!(true),
        _ => assert!(false, "Expected Structural"),
    }
    
    match temporal {
        ContradictionKind::Temporal => assert!(true),
        _ => assert!(false, "Expected Temporal"),
    }
    
    match recursive {
        ContradictionKind::Recursive => assert!(true),
        _ => assert!(false, "Expected Recursive"),
    }
}

#[test]
fn test_severity_history_tracking() {
    let mut contradiction = Contradiction::new(
        "a".to_string(),
        "b".to_string(),
        ContradictionKind::Logical,
    );
    
    // Добавляем записи
    for i in 0..5 {
        contradiction.update_severity(0.5 + i as f32 * 0.1, 0.8 - i as f32 * 0.1);
    }
    
    // Проверяем историю
    assert_eq!(contradiction.severity_history.len(), 5);
    assert_eq!(contradiction.severity_history[0].severity, 0.5);
    assert_eq!(contradiction.severity_history[4].severity, 0.9);
    
    // Проверяем, что severity обновилась
    assert_eq!(contradiction.severity, 0.9);
}
