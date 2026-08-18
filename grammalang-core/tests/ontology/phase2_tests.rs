use grammalang_core::ontology::*;

#[test]
fn test_llm_generator() {
    let generator = LLMSynthesisGenerator::default();
    let result = generator.generate("freedom", "security", &SynthesisStrategy::Hegelian).unwrap();
    assert!(!result.name.is_empty());
    assert!(result.confidence > 0.0);
    println!("LLM: {} (confidence: {:.2})", result.name, result.confidence);
}

#[test]
fn test_diffusion_generator() {
    let generator = DiffusionSynthesisGenerator::new();
    let result = generator.generate("thesis", "antithesis", &SynthesisStrategy::Plotinian).unwrap();
    assert!(!result.name.is_empty());
    println!("Diffusion: {}", result.name);
}

#[test]
fn test_evolutionary_generator() {
    let generator = EvolutionarySynthesisGenerator::new();
    let result = generator.generate("capitalism", "ecology", &SynthesisStrategy::Pragmatic).unwrap();
    assert!(!result.name.is_empty());
    println!("Evolutionary: {}", result.name);
}

#[test]
fn test_all_generators() {
    let llm = LLMSynthesisGenerator::new("qwen-32b");
    let diffusion = DiffusionSynthesisGenerator::new();
    let evolutionary = EvolutionarySynthesisGenerator::new();
    
    let strategies = vec![
        SynthesisStrategy::Hegelian,
        SynthesisStrategy::Plotinian,
        SynthesisStrategy::Pragmatic,
    ];
    
    for strategy in &strategies {
        let r1 = llm.generate("a", "b", strategy).unwrap();
        let r2 = diffusion.generate("a", "b", strategy).unwrap();
        let r3 = evolutionary.generate("a", "b", strategy).unwrap();
        
        assert!(!r1.name.is_empty());
        assert!(!r2.name.is_empty());
        assert!(!r3.name.is_empty());
    }
    
    println!("All generators work for all strategies");
}
