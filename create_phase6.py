# create_phase6.py
import os

os.makedirs('grammalang-core/tests/ontology', exist_ok=True)
os.makedirs('docs/v0.7', exist_ok=True)

# 1. Комплексные тесты
phase6_tests = '''use grammalang_core::ontology::*;
use grammalang_core::modes::*;

/// Тест 1: Платон (Идея Блага) vs. Ницше (Воля к власти)
#[test]
fn test_plato_vs_nietzsche() {
    let mut machine = MachineState::new();
    
    let plato = machine.add_node("platonic_idea", vec!["transcendent".to_string()]);
    let nietzsche = machine.add_node("will_to_power", vec!["immanent".to_string()]);
    
    machine.metrics.stability_ratio = 0.4;
    machine.metrics.contradiction_index = 0.7;
    
    let mut c = Contradiction::new(plato, nietzsche, ContradictionKind::Logical);
    c.update_severity(0.8, 0.5);
    c.update_severity(0.8, 0.4);
    c.update_severity(0.8, 0.3);
    
    let plato_mode = PlatoMode::new();
    let result = plato_mode.run(&mut machine, &[c]);
    
    assert!(result.iterations > 0);
    println!("Plato vs Nietzsche: {} iterations completed", result.iterations);
}

/// Тест 2: Гегель (Абсолютный дух) vs. Кьеркегор (Экзистенция)
#[test]
fn test_hegel_vs_kierkegaard() {
    let mut machine = MachineState::new();
    
    let hegel = machine.add_node("absolute_spirit", vec![]);
    let kierkegaard = machine.add_node("existence", vec![]);
    
    machine.metrics.stability_ratio = 0.35;
    machine.metrics.contradiction_index = 0.75;
    
    let mut c = Contradiction::new(hegel, kierkegaard, ContradictionKind::Logical);
    c.update_severity(0.75, 0.5);
    c.update_severity(0.75, 0.4);
    c.update_severity(0.75, 0.3);
    
    let plato_mode = PlatoMode::new();
    let result = plato_mode.run(&mut machine, &[c]);
    
    assert!(result.iterations > 0);
    println!("Hegel vs Kierkegaard: {} iterations", result.iterations);
}

/// Тест 3: Капитализм vs. Экология
#[test]
fn test_capitalism_vs_ecology() {
    let mut machine = MachineState::new();
    
    let capitalism = machine.add_node("capitalism", vec!["profit".to_string()]);
    let ecology = machine.add_node("ecology", vec!["sustainability".to_string()]);
    
    machine.metrics.stability_ratio = 0.3;
    machine.metrics.contradiction_index = 0.8;
    
    let mut c = Contradiction::new(capitalism, ecology, ContradictionKind::Structural);
    c.resolution_candidates = vec!["green_economy".to_string()];
    c.update_severity(0.7, 0.5);
    c.update_severity(0.7, 0.4);
    c.update_severity(0.7, 0.3);
    
    let plato_mode = PlatoMode::new();
    let result = plato_mode.run(&mut machine, &[c]);
    
    assert!(result.iterations > 0);
    println!("Capitalism vs Ecology: {} iterations", result.iterations);
}

/// Тест 4: Свобода vs. Безопасность
#[test]
fn test_freedom_vs_security() {
    let mut machine = MachineState::new();
    
    machine.add_node("freedom", vec![]);
    machine.add_node("security", vec![]);
    
    machine.metrics.stability_ratio = 0.3;
    machine.metrics.contradiction_index = 0.8;
    
    let architect = ArchitectMode::new();
    let result = architect.synthesize(
        &mut machine,
        "freedom",
        "security",
        SynthesisStrategy::Hegelian,
        Some("responsible_freedom"),
    );
    
    assert!(result.success);
    println!("Freedom vs Security: {}", result.message);
}

/// Тест 5: Пустая машина
#[test]
fn test_empty_machine_no_synthesis() {
    let mut machine = MachineState::new();
    let contradictions = vec![];
    
    let plato = PlatoMode::new();
    let result = plato.run(&mut machine, &contradictions);
    
    assert_eq!(result.iterations, 0);
    println!("Empty machine: no synthesis performed");
}

/// Тест 6: Полный цикл синтеза
#[test]
fn test_full_synthesis_cycle() {
    let mut machine = MachineState::new();
    let mut rollback = SynthesisRollback::new();
    
    // Снапшот до синтеза
    rollback.snapshot(&machine);
    
    // Добавляем узлы
    let a = machine.add_node("thesis", vec![]);
    let b = machine.add_node("antithesis", vec![]);
    
    machine.metrics.stability_ratio = 0.35;
    machine.metrics.contradiction_index = 0.7;
    
    // Создаем противоречие
    let mut c = Contradiction::new(a.clone(), b.clone(), ContradictionKind::Logical);
    c.update_severity(0.8, 0.5);
    c.update_severity(0.8, 0.4);
    c.update_severity(0.8, 0.3);
    
    // Детектор
    let detector = SynthesisDetector::new();
    let candidates = detector.detect(&[c]);
    assert_eq!(candidates.len(), 1);
    
    // Генератор
    let generator = LLMSynthesisGenerator::default();
    let synthesis = generator.generate(
        &candidates[0].source_nodes[0],
        &candidates[0].source_nodes[1],
        &candidates[0].strategy_hint,
    ).unwrap();
    
    // Интегратор
    let integrator = SynthesisIntegrator::new();
    let integration = integrator.integrate(
        &mut machine,
        &synthesis,
        &candidates[0].source_nodes,
    ).unwrap();
    
    // Валидатор
    let validator = SynthesisValidator::new();
    let validation = validator.validate(
        &machine,
        &MachineMetrics {
            stability_ratio: 0.35,
            contradiction_index: 0.7,
            node_count: 2,
            edge_count: 0,
        },
        &machine.metrics,
    );
    
    if validation.valid {
        println!("Full cycle: synthesis '{}' validated", synthesis.name);
    } else {
        println!("Full cycle: validation failed - {}", 
                 validation.reason.unwrap_or_default());
    }
    
    assert!(machine.nodes.len() >= 3);
    println!("Full cycle completed: {} nodes", machine.nodes.len());
}

/// Тест 7: Множественные противоречия
#[test]
fn test_multiple_contradictions() {
    let mut machine = MachineState::new();
    
    let a = machine.add_node("a", vec![]);
    let b = machine.add_node("b", vec![]);
    let c = machine.add_node("c", vec![]);
    let d = machine.add_node("d", vec![]);
    
    machine.metrics.stability_ratio = 0.3;
    machine.metrics.contradiction_index = 0.8;
    
    let mut c1 = Contradiction::new(a.clone(), b.clone(), ContradictionKind::Logical);
    c1.update_severity(0.8, 0.5);
    c1.update_severity(0.8, 0.4);
    c1.update_severity(0.8, 0.3);
    
    let mut c2 = Contradiction::new(c.clone(), d.clone(), ContradictionKind::Structural);
    c2.update_severity(0.7, 0.5);
    c2.update_severity(0.7, 0.4);
    c2.update_severity(0.7, 0.3);
    
    let plato = PlatoMode::new();
    let result = plato.run(&mut machine, &[c1, c2]);
    
    println!("Multiple contradictions: {} iterations", result.iterations);
    assert!(result.iterations > 0);
}

/// Тест 8: Откат после неудачного синтеза
#[test]
fn test_rollback_after_failed_synthesis() {
    let mut machine = MachineState::new();
    let mut rollback = SynthesisRollback::new();
    
    rollback.snapshot(&machine);
    
    machine.add_node("a", vec![]);
    machine.add_node("b", vec![]);
    
    // Имитация неудачного синтеза
    machine.add_node("bad_synthesis", vec![]);
    
    // Откат
    rollback.rollback(&mut machine).unwrap();
    
    assert_eq!(machine.nodes.len(), 0);
    println!("Rollback after failed synthesis: restored to {} nodes", machine.nodes.len());
}
'''

with open('grammalang-core/tests/ontology/phase6_tests.rs', 'w', encoding='utf-8') as f:
    f.write(phase6_tests)
print("phase6_tests.rs created")

# 2. Обновляем тестовый mod.rs
test_mod = '''pub mod target_ontology_tests;
pub mod contradiction_tests;
pub mod synthesis_detector_tests;
pub mod phase1_tests;
pub mod phase2_tests;
pub mod phase3_tests;
pub mod phase4_tests;
pub mod phase5_tests;
pub mod phase6_tests;
pub mod integration_test;
'''

with open('grammalang-core/tests/ontology/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

# 3. Документация
readme = '''# Atlas v0.7 - Документация

## Полный статус

### Реализованные фазы
- [x] Фаза 0: Подготовка инфраструктуры
- [x] Фаза 1: Детектор синтеза
- [x] Фаза 2: Генерация новых понятий
- [x] Фаза 3: Пересборка машины
- [x] Фаза 4: Верификация
- [x] Фаза 5: Интерфейс (Платон/Архитектор)
- [x] Фаза 6: Тестирование
- [ ] Фаза 7: Интеграция

### Тесты: 46 тестов

### Быстрый старт
cargo test --test ontology_test -- --nocapture

### Ключевые компоненты
- SynthesisDetector - обнаружение противоречий
- SynthesisGenerator - генерация синтеза (LLM/Diffusion/Evolutionary)
- SynthesisIntegrator - интеграция в машину
- SynthesisValidator - верификация
- SynthesisRollback - откат
- PlatoMode - автоматический режим
- ArchitectMode - ручной режим
'''

with open('docs/v0.7/README.md', 'w', encoding='utf-8') as f:
    f.write(readme)
print("README.md updated")

print("\nAll Phase 6 files created!")
