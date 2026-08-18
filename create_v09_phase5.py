# create_v09_phase5.py
import os

os.makedirs('grammalang-core/src/reflexive', exist_ok=True)
os.makedirs('grammalang-core/tests/reflexive', exist_ok=True)

# 1. Интеграция рефлексивной системы с v0.7 и v0.8
reflexive_integration = '''use serde::{Serialize, Deserialize};
use crate::ontology::*;
use crate::social::*;
use super::reflection::*;
use super::self_trace::*;
use super::meta_synthesis::*;
use super::auto_genealogy::*;

/// Полная интеграция рефлексивной системы с v0.7 (ontology) и v0.8 (social)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflexiveIntegration {
    pub reflexive_system: ReflexiveSystem,
    pub machine: MachineState,
    pub social_integration: SocialIntegration,
    pub state: IntegrationState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationState {
    /// Общее количество рефлексивных синтезов
    pub total_reflexive_syntheses: usize,
    
    /// Общее количество социальных обменов
    pub total_social_exchanges: usize,
    
    /// Уровень интеграции
    pub integration_level: usize,
}

impl ReflexiveIntegration {
    pub fn new() -> Self {
        Self {
            reflexive_system: ReflexiveSystem::new(),
            machine: MachineState::new(),
            social_integration: SocialIntegration::new(),
            state: IntegrationState {
                total_reflexive_syntheses: 0,
                total_social_exchanges: 0,
                integration_level: 0,
            },
        }
    }
    
    /// Рефлексивный синтез с осознанием
    pub fn reflexive_synthesize(
        &mut self,
        node_a: &str,
        node_b: &str,
        strategy: SynthesisStrategy,
    ) -> Result<SynthesisResult, SynthesisError> {
        // 1. Рефлексия над узлами
        self.reflexive_system.reflect(node_a);
        self.reflexive_system.reflect(node_b);
        
        // 2. Синтез через социальную интеграцию
        let synthesis = self.social_integration.social_synthesis(
            node_a, node_b, strategy,
        )?;
        
        // 3. Осознание синтеза
        self.reflexive_system.become_aware(&synthesis.name);
        
        // 4. Самопорождение в генеалогии
        self.reflexive_system.auto_genealogy.self_generate(
            &format!("reflexive_{}", synthesis.name),
            "Generated through reflexive synthesis",
        );
        
        // 5. Обновление состояния
        self.state.total_reflexive_syntheses += 1;
        self.state.integration_level = self.state.total_reflexive_syntheses / 3;
        
        Ok(synthesis)
    }
    
    /// Социальный обмен с рефлексией
    pub fn reflective_exchange(
        &mut self,
        from: &str,
        to: &str,
        node_id: &str,
    ) {
        // Рефлексия над обменом
        self.reflexive_system.reflect(&format!("exchange {} with {}", node_id, to));
        
        // Выполнение обмена
        self.social_integration.federated_exchange(from, to, node_id);
        
        // Осознание
        self.reflexive_system.become_aware(&format!("exchange_{}", node_id));
        
        self.state.total_social_exchanges += 1;
    }
    
    /// Полный рефлексивно-социальный цикл
    pub fn full_reflexive_social_cycle(
        &mut self,
        node_a: &str,
        node_b: &str,
    ) -> ReflexiveSocialReport {
        // Рефлексивный синтез
        let synthesis = self.reflexive_synthesize(
            node_a, node_b, SynthesisStrategy::Hegelian,
        ).unwrap_or_else(|_| SynthesisResult {
            name: format!("failed_{}_{}", node_a, node_b),
            description: "Synthesis failed".to_string(),
            properties: vec![],
            strategy: SynthesisStrategy::Hegelian,
            confidence: 0.0,
        });
        
        // Полный цикл рефлексии
        let reflex_report = self.reflexive_system.full_cycle(&synthesis.name);
        
        ReflexiveSocialReport {
            synthesis_name: synthesis.name,
            reflection: reflex_report.reflection,
            awareness: reflex_report.awareness,
            final_awareness: reflex_report.final_awareness,
            total_reflexive_syntheses: self.state.total_reflexive_syntheses,
            total_social_exchanges: self.state.total_social_exchanges,
        }
    }
    
    /// Получение сводки интеграции
    pub fn integration_summary(&self) -> String {
        format!(
            "Integration:\\n  Reflexive syntheses: {}\\n  Social exchanges: {}\\n  Integration level: {}\\n  Self-awareness: {:.2}",
            self.state.total_reflexive_syntheses,
            self.state.total_social_exchanges,
            self.state.integration_level,
            self.reflexive_system.state.self_awareness,
        )
    }
}

#[derive(Debug, Clone)]
pub struct ReflexiveSocialReport {
    pub synthesis_name: String,
    pub reflection: String,
    pub awareness: String,
    pub final_awareness: f32,
    pub total_reflexive_syntheses: usize,
    pub total_social_exchanges: usize,
}
'''

with open('grammalang-core/src/reflexive/reflexive_integration.rs', 'w', encoding='utf-8') as f:
    f.write(reflexive_integration)
print("reflexive_integration.rs created")

# 2. Обновляем mod.rs
mod_content = '''// src/reflexive/mod.rs
pub mod self_trace;
pub mod reflection_operator;
pub mod meta_synthesis;
pub mod auto_genealogy;
pub mod reflection;
pub mod reflexive_integration;

pub use self_trace::*;
pub use reflection_operator::*;
pub use meta_synthesis::*;
pub use auto_genealogy::*;
pub use reflection::*;
pub use reflexive_integration::*;
'''

with open('grammalang-core/src/reflexive/mod.rs', 'w', encoding='utf-8') as f:
    f.write(mod_content)
print("mod.rs updated")

# 3. Тесты Фазы 5
tests = '''use grammalang_core::reflexive::*;
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
'''

with open('grammalang-core/tests/reflexive/phase5_tests.rs', 'w', encoding='utf-8') as f:
    f.write(tests)
print("phase5_tests.rs created")

# 4. Обновляем тестовый mod.rs
test_mod = '''pub mod phase0_tests;
pub mod phase1_tests;
pub mod phase2_tests;
pub mod phase3_tests;
pub mod phase4_tests;
pub mod phase5_tests;
'''

with open('grammalang-core/tests/reflexive/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll v0.9 Phase 5 files created!")
