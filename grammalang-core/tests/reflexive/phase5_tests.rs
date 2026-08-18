use grammalang_core::reflexive::*;
use grammalang_core::ontology::*;

#[test]
fn test_reflexive_synthesize() {
    let mut integration = ReflexiveIntegration::new();
    
    integration.machine.add_node("freedom", vec![]);
    integration.machine.add_node("security", vec![]);
    integration.machine.metrics.stability_ratio = 0.3;
    integration.machine.metrics.contradiction_index = 0.8;
    
    let result = integration.reflexive_synthesize(
        "freedom",
        "security",
        SynthesisStrategy::Hegelian,
    ).unwrap();
    
    assert!(!result.name.is_empty());
    assert_eq!(integration.state.total_reflexive_syntheses, 1);
    
    println!("Reflexive synthesis: {}", result.name);
}

#[test]
fn test_reflective_exchange() {
    let mut integration = ReflexiveIntegration::new();
    
    integration.reflective_exchange("m1", "m2", "node1");
    
    assert_eq!(integration.state.total_social_exchanges, 1);
    
    println!("Reflective exchange completed");
}

#[test]
fn test_full_reflexive_social_cycle() {
    let mut integration = ReflexiveIntegration::new();
    
    integration.machine.add_node("thesis", vec![]);
    integration.machine.add_node("antithesis", vec![]);
    integration.machine.metrics.stability_ratio = 0.3;
    integration.machine.metrics.contradiction_index = 0.8;
    
    let report = integration.full_reflexive_social_cycle("thesis", "antithesis");
    
    assert!(!report.synthesis_name.is_empty());
    assert!(!report.reflection.is_empty());
    assert!(report.final_awareness > 0.0);
    
    println!("Full cycle:");
    println!("  Synthesis: {}", report.synthesis_name);
    println!("  Reflection: {}", report.reflection);
    println!("  Awareness: {:.2}", report.final_awareness);
}

#[test]
fn test_integration_summary() {
    let mut integration = ReflexiveIntegration::new();
    
    integration.machine.add_node("a", vec![]);
    integration.machine.add_node("b", vec![]);
    integration.machine.metrics.stability_ratio = 0.3;
    
    integration.reflexive_synthesize("a", "b", SynthesisStrategy::Hegelian).unwrap();
    integration.reflective_exchange("m1", "m2", "node");
    
    let summary = integration.integration_summary();
    
    assert!(summary.contains("Reflexive syntheses: 1"));
    assert!(summary.contains("Social exchanges: 1"));
    
    println!("{}", summary);
}

#[test]
fn test_multiple_reflexive_syntheses() {
    let mut integration = ReflexiveIntegration::new();
    
    for i in 0..3 {
        integration.machine.add_node(&format!("a{}", i), vec![]);
        integration.machine.add_node(&format!("b{}", i), vec![]);
        integration.machine.metrics.stability_ratio = 0.3;
    }
    
    integration.reflexive_synthesize("a0", "b0", SynthesisStrategy::Hegelian).unwrap();
    integration.reflexive_synthesize("a1", "b1", SynthesisStrategy::Hegelian).unwrap();
    integration.reflexive_synthesize("a2", "b2", SynthesisStrategy::Hegelian).unwrap();
    
    assert_eq!(integration.state.total_reflexive_syntheses, 3);
    assert_eq!(integration.state.integration_level, 1);
    
    println!("Multiple syntheses: {} (level {})", 
             integration.state.total_reflexive_syntheses,
             integration.state.integration_level);
}
