use grammalang_core::reflexive::*;

#[test]
fn test_auto_genealogy_basic() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.add_root("origin");
    genealogy.generate("origin", "synthesis_1", GenerationType::Synthesis, "First synthesis");
    
    assert_eq!(genealogy.generation_count(), 1);
    assert_eq!(genealogy.tree.roots.len(), 1);
    
    println!("AutoGenealogy: {} generations, {} roots", 
             genealogy.generation_count(), genealogy.tree.roots.len());
}

#[test]
fn test_generation_tree() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.add_root("root");
    genealogy.generate("root", "child1", GenerationType::Synthesis, "Synthesis 1");
    genealogy.generate("root", "child2", GenerationType::Synthesis, "Synthesis 2");
    genealogy.generate("child1", "grandchild", GenerationType::MetaSynthesis, "Meta");
    
    let descendants = genealogy.descendants("root");
    assert_eq!(descendants.len(), 2);
    
    let lineage = genealogy.lineage("grandchild");
    assert_eq!(lineage.len(), 3);
    
    println!("Lineage of grandchild: {:?}", lineage);
}

#[test]
fn test_self_generation() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.self_generate("self_concept_1", "I created this about myself");
    genealogy.self_generate("self_concept_2", "Another self-created concept");
    
    assert_eq!(genealogy.self_generated_count(), 2);
    assert_eq!(genealogy.auto_level(), 2);
    assert!(genealogy.is_self_generating());
    
    println!("Self-generated: {} concepts (level {})", 
             genealogy.self_generated_count(), genealogy.auto_level());
}

#[test]
fn test_tree_depth() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.add_root("level_0");
    genealogy.generate("level_0", "level_1", GenerationType::Synthesis, "S");
    genealogy.generate("level_1", "level_2", GenerationType::MetaSynthesis, "M");
    genealogy.generate("level_2", "level_3", GenerationType::Reflection, "R");
    
    assert_eq!(genealogy.tree.depth, 3);
    println!("Tree depth: {}", genealogy.tree.depth);
}

#[test]
fn test_analyze_self() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.add_root("root1");
    genealogy.add_root("root2");
    genealogy.generate("root1", "child1", GenerationType::Synthesis, "S");
    genealogy.self_generate("self1", "self");
    
    let analysis = genealogy.analyze_self();
    assert!(analysis.contains("2 roots"));
    
    println!("{}", analysis);
}

#[test]
fn test_generation_types() {
    let mut genealogy = AutoGenealogy::new();
    
    genealogy.add_root("root");
    genealogy.generate("root", "synth", GenerationType::Synthesis, "S");
    genealogy.generate("synth", "meta", GenerationType::MetaSynthesis, "M");
    genealogy.generate("meta", "refl", GenerationType::Reflection, "R");
    genealogy.generate("refl", "aware", GenerationType::Awareness, "A");
    genealogy.generate("aware", "self", GenerationType::SelfGeneration, "SG");
    
    assert_eq!(genealogy.generation_count(), 5);
    println!("All generation types: {} generations", genealogy.generation_count());
}
