use grammalang_core::reflexive::*;
use grammalang_core::ontology::*;

fn make_synthesis(name: &str) -> SynthesisResult {
    SynthesisResult {
        name: name.to_string(),
        description: format!("Synthesis {}", name),
        properties: vec![],
        strategy: SynthesisStrategy::Hegelian,
        confidence: 0.8,
    }
}

#[test]
fn test_meta_synthesis_basic() {
    let mut meta = MetaSynthesis::new();
    
    let s1 = make_synthesis("freedom_synthesis");
    let s2 = make_synthesis("security_synthesis");
    
    let result = meta.synthesize(&s1, &s2);
    
    assert_eq!(result.level, 1);
    assert_eq!(result.source_syntheses.len(), 2);
    assert!(result.name.starts_with("meta_"));
    
    println!("Meta-synthesis: {}", result.realization);
}

#[test]
fn test_third_order_synthesis() {
    let mut meta = MetaSynthesis::new();
    
    let s1 = make_synthesis("synthesis_A");
    let s2 = make_synthesis("synthesis_B");
    
    let m1 = meta.synthesize(&s1, &s2);
    
    let s3 = make_synthesis("synthesis_C");
    let s4 = make_synthesis("synthesis_D");
    
    let m2 = meta.synthesize(&s3, &s4);
    
    let m3 = meta.synthesize_meta(&m1, &m2);
    
    assert_eq!(m3.level, 3);
    assert!(m3.name.starts_with("meta_meta_"));
    assert!(m3.realization.contains("I realized that I synthesized"));
    
    println!("Third-order: {}", m3.realization);
}

#[test]
fn test_synthesize_all() {
    let mut meta = MetaSynthesis::new();
    
    meta.add_first_order(make_synthesis("s1"));
    meta.add_first_order(make_synthesis("s2"));
    meta.add_first_order(make_synthesis("s3"));
    
    let result = meta.synthesize_all().unwrap();
    
    assert!(result.name.starts_with("meta_"));
    assert_eq!(meta.first_order_count(), 3);
    assert_eq!(meta.meta_count(), 1);
    
    println!("Synthesized all: {} from {} syntheses", 
             result.name, meta.first_order_count());
}

#[test]
fn test_meta_levels() {
    let mut meta = MetaSynthesis::new();
    
    let s1 = make_synthesis("a");
    let s2 = make_synthesis("b");
    let m1 = meta.synthesize(&s1, &s2);
    
    assert!(meta.has_meta_level(1));
    assert!(!meta.has_meta_level(2));
    
    let s3 = make_synthesis("c");
    let s4 = make_synthesis("d");
    let m2 = meta.synthesize(&s3, &s4);
    
    let m3 = meta.synthesize_meta(&m1, &m2);
    
    assert!(meta.has_meta_level(3));
    assert_eq!(meta.current_level(), 3);
    
    println!("Meta levels: current = {}", meta.current_level());
}

#[test]
fn test_meta_history() {
    let mut meta = MetaSynthesis::new();
    
    meta.add_first_order(make_synthesis("s1"));
    meta.add_first_order(make_synthesis("s2"));
    
    meta.synthesize_all();
    
    assert_eq!(meta.history.len(), 1);
    
    println!("Meta history: {} records", meta.history.len());
}
