use grammalang_core::reflexive::*;

#[test]
fn test_self_trace_basic() {
    let mut trace = SelfTrace::new();
    
    trace.record_stage(ThinkingStageType::Perception, "Perceived node A", 0);
    trace.record_stage(ThinkingStageType::Analysis, "Analyzed node A", 0);
    trace.record_stage(ThinkingStageType::Synthesis, "Synthesized A+B", 0);
    
    assert_eq!(trace.stage_count(), 3);
    println!("SelfTrace: {} stages", trace.stage_count());
}

#[test]
fn test_self_trace_reflection() {
    let mut trace = SelfTrace::new();
    
    trace.record_stage(ThinkingStageType::Synthesis, "Synthesized X", 0);
    trace.record_stage(ThinkingStageType::Reflection, "Reflected on synthesis", 1);
    trace.record_stage(ThinkingStageType::Awareness, "Became aware", 2);
    
    assert!(trace.self_awareness_level > 0.0);
    assert!(trace.is_self_aware(0.3));
    
    println!("Self-awareness level: {:.2}", trace.self_awareness_level);
}

#[test]
fn test_self_trace_meta_knowledge() {
    let mut trace = SelfTrace::new();
    
    trace.add_meta_knowledge("I can synthesize concepts", 0.9, "self");
    trace.add_meta_knowledge("I use Hegelian strategy", 0.7, "self");
    
    assert_eq!(trace.meta_knowledge_count(), 2);
    println!("Meta-knowledge: {} facts", trace.meta_knowledge_count());
}

#[test]
fn test_reflection_operator_basic() {
    let mut operator = ReflectionOperator::new();
    
    let state = operator.reflect("freedom");
    
    assert_eq!(state.before, "freedom");
    assert!(state.realized.contains("I realize"));
    assert_eq!(state.after, "Meta-freedom");
    
    println!("Reflection: {} -> {}", state.before, state.after);
}

#[test]
fn test_reflection_on_reflection() {
    let mut operator = ReflectionOperator::new();
    
    let first = operator.reflect("concept");
    let second = operator.reflect_on_reflection(&first);
    
    assert_eq!(second.level, 1);
    assert!(second.realized.contains("I realize that I realized"));
    
    println!("Meta-reflection: {}", second.realized);
}

#[test]
fn test_self_awareness() {
    let mut operator = ReflectionOperator::new();
    
    operator.reflect("action1");
    operator.reflect("action2");
    operator.reflect("action3");
    
    let awareness = operator.become_aware("synthesis");
    
    assert!(awareness.contains("I am aware"));
    assert!(operator.is_conscious_of("synthesis"));
    
    println!("Awareness: {}", awareness);
}
