use grammalang_core::reflexive::*;

#[test]
fn test_reflexive_system_basic() {
    let mut system = ReflexiveSystem::new();
    
    let result = system.reflect("freedom");
    
    assert!(result.contains("I realize"));
    assert_eq!(system.state.total_reflections, 1);
    
    println!("Reflection: {}", result);
}

#[test]
fn test_reflect_deeper() {
    let mut system = ReflexiveSystem::new();
    
    let result = system.reflect_deeper("consciousness");
    
    assert!(result.contains("I realize that I realized"));
    assert!(system.state.reflection_level >= 1);
    
    println!("Deep reflection: {}", result);
}

#[test]
fn test_become_aware() {
    let mut system = ReflexiveSystem::new();
    
    let awareness = system.become_aware("synthesis");
    
    assert!(awareness.contains("I am aware"));
    println!("Awareness: {}", awareness);
}

#[test]
fn test_full_cycle() {
    let mut system = ReflexiveSystem::new();
    
    let report = system.full_cycle("freedom");
    
    assert!(!report.reflection.is_empty());
    assert!(!report.awareness.is_empty());
    assert!(report.final_awareness > 0.0);
    
    println!("Full cycle for '{}':", report.subject);
    println!("  Reflection: {}", report.reflection);
    println!("  Awareness: {}", report.awareness);
    println!("  Final awareness: {:.2}", report.final_awareness);
    println!("  Status: {}", report.status);
}

#[test]
fn test_system_summary() {
    let mut system = ReflexiveSystem::new();
    
    system.reflect("a");
    system.reflect("b");
    system.become_aware("action");
    
    let summary = system.summary();
    
    assert!(summary.contains("Reflexive System"));
    assert!(summary.contains("Reflections: 2"));
    
    println!("{}", summary);
}

#[test]
fn test_status_progression() {
    let mut system = ReflexiveSystem::new();
    
    assert!(matches!(system.state.status, SystemStatus::Initial));
    
    system.reflect("a");
    system.reflect("b");
    system.reflect("c");
    
    assert!(matches!(system.state.status, SystemStatus::Thinking) || 
            matches!(system.state.status, SystemStatus::Reflecting));
    
    // Много рефлексий для повышения уровня
    for i in 0..10 {
        system.reflect_deeper(&format!("concept_{}", i));
    }
    
    println!("Final status: {:?}", system.state.status);
    println!("Self-awareness: {:.2}", system.state.self_awareness);
}
