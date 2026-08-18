use grammalang_core::ontology::*;

#[test]
fn test_llm_resolver_integration() {
    let integration = LLMResolverIntegration::new();
    
    let result = integration.generate_with_resolver(
        "freedom",
        "security",
        &SynthesisStrategy::Hegelian,
    ).unwrap();
    
    assert!(result.description.contains("resolved via LLM"));
    println!("LLM Resolver: {}", result.description);
}

#[test]
fn test_temporal_map_recording() {
    let mut temporal = TemporalMapIntegration::new();
    
    temporal.record_synthesis("responsible_freedom", 0.8);
    temporal.record_validation(true);
    
    assert_eq!(temporal.event_count(), 2);
    println!("TemporalMap: {} events recorded", temporal.event_count());
}

#[test]
fn test_full_integration_cycle() {
    let mut integration = IntegrationLayer::new();
    let mut machine = MachineState::new();
    
    machine.add_node("freedom", vec![]);
    machine.add_node("security", vec![]);
    machine.metrics.stability_ratio = 0.3;
    machine.metrics.contradiction_index = 0.8;
    
    let result = integration.full_cycle(
        &mut machine,
        "freedom",
        "security",
        &SynthesisStrategy::Hegelian,
    );
    
    assert!(result.is_ok());
    assert!(integration.temporal_map.event_count() >= 2);
    
    println!("Full integration cycle: {} events", 
             integration.temporal_map.event_count());
}

#[test]
fn test_genealogy_preservation() {
    let integration = LLMResolverIntegration::new();
    
    let synthesis = SynthesisResult {
        name: "synthesis".to_string(),
        description: "test".to_string(),
        properties: vec![],
        strategy: SynthesisStrategy::Hegelian,
        confidence: 0.8,
    };
    
    let genealogy = integration.preserve_genealogy(
        &synthesis,
        &["parent_a".to_string(), "parent_b".to_string()],
    );
    
    assert_eq!(genealogy.len(), 3);
    assert_eq!(genealogy[0], "parent_a");
    assert_eq!(genealogy[1], "parent_b");
    assert_eq!(genealogy[2], "synthesis");
    
    println!("Genealogy preserved: {:?}", genealogy);
}

#[test]
fn test_temporal_events_history() {
    let mut temporal = TemporalMapIntegration::new();
    
    temporal.record_synthesis("a", 0.7);
    temporal.record_validation(true);
    temporal.record_synthesis("b", 0.8);
    temporal.record_validation(false);
    temporal.record_rollback();
    
    let history = temporal.history();
    assert_eq!(history.len(), 5);
    
    println!("Temporal history:");
    for event in history {
        println!("  [{:?}] {}", event.event_type, event.description);
    }
}
