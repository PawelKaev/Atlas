use grammalang_core::ontology::*;

#[test]
fn test_machine_state() {
    let mut machine = MachineState::new();
    
    let a = machine.add_node("freedom", vec!["abstract".to_string()]);
    let b = machine.add_node("security", vec!["concrete".to_string()]);
    
    assert_eq!(machine.nodes.len(), 2);
    assert_eq!(machine.metrics.node_count, 2);
    
    println!("Machine: {} nodes, stability: {:.2}", 
             machine.metrics.node_count, 
             machine.metrics.stability_ratio);
}

#[test]
fn test_synthesis_integration() {
    let mut machine = MachineState::new();
    
    let a = machine.add_node("freedom", vec![]);
    let b = machine.add_node("security", vec![]);
    
    let integrator = SynthesisIntegrator::new();
    
    let synthesis = SynthesisResult {
        name: "responsible_freedom".to_string(),
        description: "Synthesis of freedom and security".to_string(),
        properties: vec!["balanced".to_string()],
        strategy: SynthesisStrategy::Hegelian,
        confidence: 0.8,
    };
    
    let result = integrator.integrate(
        &mut machine,
        &synthesis,
        &[a, b],
    ).unwrap();
    
    assert_eq!(machine.nodes.len(), 3);
    assert_eq!(result.edges_created, 2);
    
    println!("Integrated: {} ({} edges)", result.node_id, result.edges_created);
}

#[test]
fn test_axis_proposal() {
    let mut machine = MachineState::new();
    
    let a = machine.add_node("good", vec![]);
    let b = machine.add_node("evil", vec![]);
    
    let integrator = SynthesisIntegrator::new();
    let synthesis = SynthesisResult {
        name: "moral_axis".to_string(),
        description: "Moral axis".to_string(),
        properties: vec!["axis_candidate".to_string()],
        strategy: SynthesisStrategy::Hegelian,
        confidence: 0.9,
    };
    
    let result = integrator.integrate(&mut machine, &synthesis, &[a, b]).unwrap();
    
    let proposer = AxisProposer::new();
    let candidate = machine.nodes.iter()
        .find(|n| n.id == result.node_id)
        .unwrap();
    
    if let Some(proposal) = proposer.propose(&machine, candidate) {
        println!("Axis proposed: {} (gain: {:.2})", 
                 proposal.axis_name, proposal.expected_gain);
        assert!(!proposal.axis_name.is_empty());
    }
}

#[test]
fn test_low_confidence_rejection() {
    let mut machine = MachineState::new();
    let a = machine.add_node("a", vec![]);
    let b = machine.add_node("b", vec![]);
    
    let integrator = SynthesisIntegrator::new();
    let synthesis = SynthesisResult {
        name: "weak_synthesis".to_string(),
        description: "Low confidence".to_string(),
        properties: vec![],
        strategy: SynthesisStrategy::Hegelian,
        confidence: 0.3,
    };
    
    let result = integrator.integrate(&mut machine, &synthesis, &[a, b]);
    
    assert!(result.is_err());
    println!("Low confidence rejected correctly");
}
