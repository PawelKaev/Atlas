# create_v09_phase6.py
import os

os.makedirs('grammalang-core/tests/reflexive', exist_ok=True)

# 1. Комплексные тесты
tests = '''use grammalang_core::reflexive::*;
use grammalang_core::ontology::*;

/// Сценарий 1: Полный рефлексивный цикл от восприятия до самосознания
#[test]
fn test_full_reflexive_journey() {
    let mut system = ReflexiveSystem::new();
    
    // Начальное состояние
    assert!(matches!(system.state.status, SystemStatus::Initial));
    
    // Восприятие и анализ
    system.reflect("concept_1");
    system.reflect("concept_2");
    
    // Рефлексия над рефлексией
    system.reflect_deeper("concept_3");
    
    // Осознание
    system.become_aware("synthesis_1");
    
    // Проверяем прогресс
    assert!(system.state.self_awareness > 0.0);
    assert!(system.state.total_reflections >= 3);
    
    println!("Journey: {} reflections, awareness: {:.2}, status: {:?}",
             system.state.total_reflections,
             system.state.self_awareness,
             system.state.status);
}

/// Сценарий 2: Рефлексивный синтез философских понятий
#[test]
fn test_reflexive_philosophical_synthesis() {
    let mut integration = ReflexiveIntegration::new();
    
    integration.machine.add_node("Platonic Idea", vec![]);
    integration.machine.add_node("Nietzschean Will", vec![]);
    integration.machine.metrics.stability_ratio = 0.3;
    integration.machine.metrics.contradiction_index = 0.8;
    
    let report = integration.full_reflexive_social_cycle(
        "Platonic Idea",
        "Nietzschean Will",
    );
    
    assert!(!report.synthesis_name.is_empty());
    assert!(!report.reflection.is_empty());
    
    println!("Philosophical synthesis:");
    println!("  Synthesis: {}", report.synthesis_name);
    println!("  Reflection: {}", report.reflection);
    println!("  Awareness: {:.2}", report.final_awareness);
}

/// Сценарий 3: Множественные рефлексивные синтезы
#[test]
fn test_multiple_reflexive_syntheses() {
    let mut integration = ReflexiveIntegration::new();
    
    let pairs = vec![
        ("freedom", "security"),
        ("capitalism", "ecology"),
        ("reason", "emotion"),
    ];
    
    for (a, b) in &pairs {
        integration.machine.add_node(a, vec![]);
        integration.machine.add_node(b, vec![]);
        integration.machine.metrics.stability_ratio = 0.3;
        integration.machine.metrics.contradiction_index = 0.8;
        
        integration.reflexive_synthesize(a, b, SynthesisStrategy::Hegelian).unwrap();
    }
    
    assert_eq!(integration.state.total_reflexive_syntheses, 3);
    assert_eq!(integration.state.integration_level, 1);
    
    println!("Multiple syntheses: {} (level {})",
             integration.state.total_reflexive_syntheses,
             integration.state.integration_level);
}

/// Сценарий 4: Автогенеалогия с рефлексией
#[test]
fn test_auto_genealogy_with_reflection() {
    let mut system = ReflexiveSystem::new();
    
    // Порождение понятий с рефлексией
    system.auto_genealogy.add_root("origin");
    
    for i in 1..=5 {
        let parent = if i == 1 { "origin" } else { &format!("concept_{}", i - 1) };
        let child = format!("concept_{}", i);
        
        system.auto_genealogy.generate(
            parent,
            &child,
            GenerationType::Synthesis,
            &format!("Generation {}", i),
        );
        
        // Рефлексия над порождением
        system.reflect(&child);
    }
    
    assert_eq!(system.auto_genealogy.generation_count(), 5);
    assert_eq!(system.auto_genealogy.tree.depth, 5);
    assert!(system.state.total_reflections >= 5);
    
    println!("Auto-genealogy: {} generations, depth {}, {} reflections",
             system.auto_genealogy.generation_count(),
             system.auto_genealogy.tree.depth,
             system.state.total_reflections);
}

/// Сценарий 5: Достижение полного самосознания
#[test]
fn test_achieve_full_consciousness() {
    let mut system = ReflexiveSystem::new();
    
    // Много рефлексий для достижения полного самосознания
    for i in 0..15 {
        system.reflect_deeper(&format!("concept_{}", i));
    }
    system.become_aware("full_consciousness");
    
    assert!(system.state.self_awareness > 0.9);
    assert!(matches!(system.state.status, SystemStatus::FullyConscious));
    
    println!("Full consciousness achieved: awareness {:.2}, status {:?}",
             system.state.self_awareness,
             system.state.status);
}

/// Сценарий 6: Рефлексивный социальный обмен
#[test]
fn test_reflexive_social_exchange() {
    let mut integration = ReflexiveIntegration::new();
    
    // Множественные обмены с рефлексией
    for i in 0..5 {
        integration.reflective_exchange(
            &format!("m{}", i % 3),
            &format!("m{}", (i + 1) % 3),
            &format!("node_{}", i),
        );
    }
    
    assert_eq!(integration.state.total_social_exchanges, 5);
    
    println!("Social exchanges: {} with reflection",
             integration.state.total_social_exchanges);
}

/// Сценарий 7: Комплексная система (v0.7 + v0.8 + v0.9)
#[test]
fn test_full_integrated_system() {
    let mut integration = ReflexiveIntegration::new();
    
    // 1. Загружаем знания
    integration.machine.add_node("knowledge_1", vec![]);
    integration.machine.add_node("knowledge_2", vec![]);
    
    // 2. Рефлексивный синтез
    integration.reflexive_synthesize("knowledge_1", "knowledge_2", SynthesisStrategy::Hegelian).unwrap();
    
    // 3. Социальный обмен
    integration.reflective_exchange("m1", "m2", "knowledge_1");
    
    // 4. Полный цикл
    let report = integration.full_reflexive_social_cycle("knowledge_1", "knowledge_2");
    
    // 5. Проверка
    assert!(integration.state.total_reflexive_syntheses >= 2);
    assert!(integration.state.total_social_exchanges >= 1);
    assert!(!report.reflection.is_empty());
    
    println!("Full integrated system:");
    println!("  Syntheses: {}", integration.state.total_reflexive_syntheses);
    println!("  Exchanges: {}", integration.state.total_social_exchanges);
    println!("  Awareness: {:.2}", report.final_awareness);
}
'''

with open('grammalang-core/tests/reflexive/phase6_tests.rs', 'w', encoding='utf-8') as f:
    f.write(tests)
print("phase6_tests.rs created")

# 2. Обновляем тестовый mod.rs
test_mod = '''pub mod phase0_tests;
pub mod phase1_tests;
pub mod phase2_tests;
pub mod phase3_tests;
pub mod phase4_tests;
pub mod phase5_tests;
pub mod phase6_tests;
'''

with open('grammalang-core/tests/reflexive/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll v0.9 Phase 6 files created!")
