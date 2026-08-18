use grammalang_core::ontology::*;

#[test]
fn test_validator_success() {
    let validator = SynthesisValidator::new();
    
    let before = MachineMetrics {
        stability_ratio: 0.5,
        contradiction_index: 0.7,
        node_count: 2,
        edge_count: 0,
    };
    
    let after = MachineMetrics {
        stability_ratio: 0.8,
        contradiction_index: 0.3,
        node_count: 3,
        edge_count: 2,
    };
    
    let result = validator.validate(
        &MachineState::new(),
        &before,
        &after,
    );
    
    assert!(result.valid);
    println!("Validation passed: stability improved to {:.2}", after.stability_ratio);
}

#[test]
fn test_validator_low_stability() {
    let validator = SynthesisValidator::new();
    
    let before = MachineMetrics {
        stability_ratio: 0.4,
        contradiction_index: 0.5,
        node_count: 2,
        edge_count: 0,
    };
    
    let after = MachineMetrics {
        stability_ratio: 0.3,
        contradiction_index: 0.5,
        node_count: 3,
        edge_count: 2,
    };
    
    let result = validator.validate(
        &MachineState::new(),
        &before,
        &after,
    );
    
    assert!(!result.valid);
    println!("Low stability rejected: {}", result.reason.unwrap());
}

#[test]
fn test_simulation() {
    let validator = SynthesisValidator::new();
    let mut machine = MachineState::new();
    
    machine.add_node("a", vec![]);
    machine.add_node("b", vec![]);
    
    let result = validator.simulate(&mut machine, Some(20));
    
    assert_eq!(result.steps_completed, 20);
    assert!(result.final_stability > 0.0);
    assert!(result.stability_history.len() == 20);
    
    println!("Simulation: {} steps, final stability: {:.2}", 
             result.steps_completed, result.final_stability);
}

#[test]
fn test_rollback() {
    let mut machine = MachineState::new();
    let mut rollback = SynthesisRollback::new();
    
    // Снапшот до изменений
    rollback.snapshot(&machine);
    
    // Добавляем узлы
    machine.add_node("a", vec![]);
    machine.add_node("b", vec![]);
    machine.add_node("c", vec![]);
    
    assert_eq!(machine.nodes.len(), 3);
    
    // Откат
    rollback.rollback(&mut machine).unwrap();
    
    assert_eq!(machine.nodes.len(), 0);
    println!("Rollback successful: restored to {} nodes", machine.nodes.len());
}

#[test]
fn test_rollback_multiple() {
    let mut machine = MachineState::new();
    let mut rollback = SynthesisRollback::new();
    
    // Снапшот 0
    rollback.snapshot(&machine);
    
    machine.add_node("a", vec![]);
    rollback.snapshot(&machine);
    
    machine.add_node("b", vec![]);
    rollback.snapshot(&machine);
    
    machine.add_node("c", vec![]);
    
    assert_eq!(machine.nodes.len(), 3);
    assert_eq!(rollback.len(), 3);
    
    // Откат к снапшоту 1 (после добавления a)
    rollback.rollback_to(&mut machine, 1).unwrap();
    
    assert_eq!(machine.nodes.len(), 1);
    println!("Rollback to snapshot 1: {} nodes", machine.nodes.len());
}
