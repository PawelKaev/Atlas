use grammalang_core::reflexive::*;

#[test]
fn test_cognitive_state() {
    let mut trace = SelfTrace::new();
    
    trace.record_stage(ThinkingStageType::Perception, "Perceived A", 0);
    trace.record_stage(ThinkingStageType::Analysis, "Analyzed A", 0);
    trace.record_stage(ThinkingStageType::Synthesis, "Synthesized A+B", 0);
    trace.record_stage(ThinkingStageType::Reflection, "Reflected on synthesis", 1);
    
    let state = &trace.cognitive_state;
    
    assert_eq!(state.total_stages, 4);
    assert_eq!(state.reflection_count, 1);
    assert!(state.avg_reflection_level > 0.0);
    
    println!("Cognitive state: {} stages, {} reflections, avg level {:.2}", 
             state.total_stages, state.reflection_count, state.avg_reflection_level);
}

#[test]
fn test_metacognition() {
    let mut trace = SelfTrace::new();
    
    trace.record_stage(ThinkingStageType::Synthesis, "Synthesized X", 0);
    trace.record_stage(ThinkingStageType::Reflection, "Reflected", 1);
    trace.record_stage(ThinkingStageType::MetaCognition, "Thought about thinking", 2);
    
    assert_eq!(trace.cognitive_state.metacognition_count, 1);
    assert!(trace.self_awareness_level > 0.5);
    
    println!("Metacognition: {} act, awareness: {:.2}", 
             trace.cognitive_state.metacognition_count,
             trace.self_awareness_level);
}

#[test]
fn test_analysis_patterns() {
    let mut trace = SelfTrace::new();
    
    trace.record_stage(ThinkingStageType::Synthesis, "S1", 0);
    trace.record_stage(ThinkingStageType::Reflection, "R1", 1);
    trace.record_stage(ThinkingStageType::Reflection, "R2", 1);
    trace.record_action("a1", "conscious action", true);
    trace.record_action("a2", "unconscious action", false);
    
    let patterns = trace.analyze_patterns();
    
    assert!(!patterns.is_empty());
    println!("Patterns found: {}:", patterns.len());
    for pattern in &patterns {
        println!("  - {}", pattern);
    }
}

#[test]
fn test_reflection_history() {
    let mut trace = SelfTrace::new();
    
    trace.record_reflection(0, "concept A", "I understand A");
    trace.record_reflection(1, "concept A", "I understand that I understand A");
    trace.record_reflection(2, "concept A", "I understand my understanding of A");
    
    assert_eq!(trace.reflection_history_count(), 3);
    
    println!("Reflection history: {} entries", trace.reflection_history_count());
}

#[test]
fn test_self_awareness_report() {
    let mut trace = SelfTrace::new();
    
    trace.record_stage(ThinkingStageType::Synthesis, "S1", 0);
    trace.record_stage(ThinkingStageType::Reflection, "R1", 1);
    trace.record_stage(ThinkingStageType::Awareness, "A1", 2);
    trace.add_meta_knowledge("I can synthesize", 0.8, "self");
    
    let report = trace.self_awareness_report();
    
    assert!(report.contains("Self-awareness"));
    println!("{}", report);
}

#[test]
fn test_high_self_awareness() {
    let mut trace = SelfTrace::new();
    
    // Много рефлексий и метакогниций
    for i in 0..5 {
        trace.record_stage(ThinkingStageType::Reflection, &format!("R{}", i), 1);
    }
    for i in 0..3 {
        trace.record_stage(ThinkingStageType::MetaCognition, &format!("M{}", i), 2);
    }
    trace.record_stage(ThinkingStageType::Awareness, "Full awareness", 3);
    
    assert!(trace.is_self_aware(0.5));
    assert!(trace.self_awareness_level > 0.7);
    
    println!("High self-awareness: {:.2}", trace.self_awareness_level);
}
