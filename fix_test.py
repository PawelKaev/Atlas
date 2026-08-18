# fix_test.py
content = '''use grammalang_core::ontology::*;
use grammalang_core::modes::*;

#[test]
fn test_plato_mode() {
    let mut machine = MachineState::new();
    
    let a = machine.add_node("freedom", vec![]);
    let b = machine.add_node("security", vec![]);
    
    machine.metrics.stability_ratio = 0.4;
    machine.metrics.contradiction_index = 0.7;
    
    let mut c = Contradiction::new(a.clone(), b.clone(), ContradictionKind::Logical);
    c.update_severity(0.8, 0.5);
    c.update_severity(0.8, 0.4);
    c.update_severity(0.8, 0.3);
    
    let plato = PlatoMode::new();
    let result = plato.run(&mut machine, &[c]);
    
    assert!(result.iterations > 0);
    println!("Plato mode: {} iterations", result.iterations);
}

#[test]
fn test_architect_mode() {
    let mut machine = MachineState::new();
    
    machine.add_node("freedom", vec![]);
    machine.add_node("security", vec![]);
    
    machine.metrics.stability_ratio = 0.3;
    machine.metrics.contradiction_index = 0.8;
    
    let architect = ArchitectMode::new();
    let result = architect.synthesize(
        &mut machine,
        "freedom",
        "security",
        SynthesisStrategy::Hegelian,
        Some("responsible_freedom"),
    );
    
    println!("Result: {} (success: {})", result.message, result.success);
    assert!(result.success);
}

#[test]
fn test_architect_list_contradictions() {
    let architect = ArchitectMode::new();
    let mut c = Contradiction::new("a".to_string(), "b".to_string(), ContradictionKind::Logical);
    c.update_severity(0.7, 0.5);
    c.update_severity(0.7, 0.4);
    c.update_severity(0.7, 0.3);
    
    let list = architect.list_contradictions(&[c]);
    assert_eq!(list.len(), 1);
}

#[test]
fn test_plato_empty_machine() {
    let mut machine = MachineState::new();
    let contradictions = vec![];
    let plato = PlatoMode::new();
    let result = plato.run(&mut machine, &contradictions);
    assert_eq!(result.iterations, 0);
}
'''

with open('grammalang-core/tests/ontology/phase5_tests.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print('phase5_tests.rs updated')
