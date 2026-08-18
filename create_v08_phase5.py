# create_v08_phase5.py
import os

os.makedirs('grammalang-core/src/social', exist_ok=True)
os.makedirs('grammalang-core/tests/social', exist_ok=True)

# 1. Интеграционный слой между v0.7 и v0.8
integration = '''use crate::ontology::*;
use super::knowledge_base::*;
use super::collective_trace::*;
use super::social_reactor::*;
use super::federation::*;

/// Интеграция v0.7 (ontology) с v0.8 (social)
pub struct SocialIntegration {
    /// Связь с машиной v0.7
    pub machine: MachineState,
    /// База знаний
    pub knowledge_base: KnowledgeBase,
    /// Коллективный trace
    pub collective_trace: CollectiveTrace,
    /// Социальный реактор
    pub social_reactor: SocialReactor,
    /// Федерация
    pub federation: Federation,
}

impl SocialIntegration {
    pub fn new() -> Self {
        Self {
            machine: MachineState::new(),
            knowledge_base: KnowledgeBase::new(
                "default",
                "Default Knowledge Base",
                KnowledgeBaseType::Custom,
            ),
            collective_trace: CollectiveTrace::new(),
            social_reactor: SocialReactor::new(),
            federation: Federation::new(),
        }
    }
    
    /// Импорт узлов из базы знаний в машину
    pub fn import_from_kb(&mut self) -> usize {
        let mut imported = 0;
        
        for node in &self.knowledge_base.nodes {
            let node_id = self.machine.add_node(
                &node.label,
                node.properties.values().cloned().collect(),
            );
            
            self.collective_trace.add_genealogy(
                &node_id,
                vec![format!("kb:{}", self.knowledge_base.id)],
            );
            
            imported += 1;
        }
        
        imported
    }
    
    /// Экспорт узлов из машины в базу знаний
    pub fn export_to_kb(&mut self) -> usize {
        let mut exported = 0;
        
        for node in &self.machine.nodes {
            let kb_node = KnowledgeNode {
                id: node.id.clone(),
                label: node.name.clone(),
                description: format!("Exported from machine"),
                properties: {
                    let mut props = std::collections::HashMap::new();
                    props.insert("source".to_string(), "machine".to_string());
                    for (i, prop) in node.properties.iter().enumerate() {
                        props.insert(format!("prop_{}", i), prop.clone());
                    }
                    props
                },
                relations: vec![],
            };
            
            self.knowledge_base.add_node(kb_node);
            exported += 1;
        }
        
        exported
    }
    
    /// Синтез с учетом социального контекста
    pub fn social_synthesis(
        &mut self,
        node_a: &str,
        node_b: &str,
        strategy: SynthesisStrategy,
    ) -> Result<SynthesisResult, SynthesisError> {
        // Проверяем социальные противоречия
        let contradiction = SocialContradiction {
            source_a: node_a.to_string(),
            source_b: node_b.to_string(),
            severity: 0.7,
            kind: SocialContradictionKind::MachineConflict,
            context: None,
        };
        
        // Обрабатываем через социальный реактор
        let reaction = self.social_reactor.process_one(contradiction);
        
        // Генерируем синтез
        let generator = LLMSynthesisGenerator::default();
        let synthesis = generator.generate(node_a, node_b, &strategy)?;
        
        // Записываем в trace
        self.collective_trace.record_event(
            "machine",
            TraceEventType::Synthesis,
            &synthesis.name,
        );
        
        // Добавляем генеалогию
        self.collective_trace.add_genealogy(
            &synthesis.name,
            vec![node_a.to_string(), node_b.to_string()],
        );
        
        Ok(synthesis)
    }
    
    /// Обмен узлами через федерацию
    pub fn federated_exchange(
        &mut self,
        from: &str,
        to: &str,
        node_id: &str,
    ) {
        self.federation.exchange(node_id, from, to);
        self.collective_trace.record_event(
            from,
            TraceEventType::Merge,
            &format!("Exchanged {} with {}", node_id, to),
        );
    }
    
    /// Полная интеграционная статистика
    pub fn integration_stats(&self) -> IntegrationStats {
        IntegrationStats {
            machine_nodes: self.machine.nodes.len(),
            kb_nodes: self.knowledge_base.metadata.node_count,
            trace_events: self.collective_trace.event_count(),
            reactor_processed: self.social_reactor.processed_count(),
            federation_members: self.federation.member_count(),
            federation_exchanges: self.federation.exchange_count(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntegrationStats {
    pub machine_nodes: usize,
    pub kb_nodes: usize,
    pub trace_events: usize,
    pub reactor_processed: usize,
    pub federation_members: usize,
    pub federation_exchanges: usize,
}

/// Мост между v0.7 и v0.8
pub struct SocialBridge {
    pub integration: SocialIntegration,
    pub plato_mode: crate::modes::PlatoMode,
    pub architect_mode: crate::modes::ArchitectMode,
}

impl SocialBridge {
    pub fn new() -> Self {
        Self {
            integration: SocialIntegration::new(),
            plato_mode: crate::modes::PlatoMode::new(),
            architect_mode: crate::modes::ArchitectMode::new(),
        }
    }
    
    /// Запуск Платона с социальным контекстом
    pub fn run_plato_social(
        &mut self,
        contradictions: &[Contradiction],
    ) -> crate::modes::PlatoResult {
        self.plato_mode.run(&mut self.integration.machine, contradictions)
    }
    
    /// Ручной синтез с социальным контекстом
    pub fn run_architect_social(
        &mut self,
        node_a: &str,
        node_b: &str,
        strategy: SynthesisStrategy,
        name: Option<&str>,
    ) -> crate::modes::ArchitectResult {
        self.architect_mode.synthesize(
            &mut self.integration.machine,
            node_a,
            node_b,
            strategy,
            name,
        )
    }
}
'''

with open('grammalang-core/src/social/integration.rs', 'w', encoding='utf-8') as f:
    f.write(integration)
print("integration.rs created")

# 2. Обновляем mod.rs
mod_content = '''// src/social/mod.rs
pub mod knowledge_base;
pub mod kb_connectors;
pub mod collective_trace;
pub mod social_reactor;
pub mod federation;
pub mod integration;

pub use knowledge_base::*;
pub use kb_connectors::*;
pub use collective_trace::*;
pub use social_reactor::*;
pub use federation::*;
pub use integration::*;
'''

with open('grammalang-core/src/social/mod.rs', 'w', encoding='utf-8') as f:
    f.write(mod_content)
print("mod.rs updated")

# 3. Тесты Фазы 5
tests = '''use grammalang_core::social::*;
use grammalang_core::ontology::*;

#[test]
fn test_import_from_kb() {
    let mut integration = SocialIntegration::new();
    
    // Добавляем узлы в KB
    let node = KnowledgeNode {
        id: "kb_node1".to_string(),
        label: "Knowledge Node".to_string(),
        description: "From KB".to_string(),
        properties: std::collections::HashMap::new(),
        relations: vec![],
    };
    integration.knowledge_base.add_node(node);
    
    // Импортируем в машину
    let imported = integration.import_from_kb();
    
    assert_eq!(imported, 1);
    assert_eq!(integration.machine.nodes.len(), 1);
    
    println!("Imported {} nodes from KB", imported);
}

#[test]
fn test_export_to_kb() {
    let mut integration = SocialIntegration::new();
    
    // Добавляем узлы в машину
    integration.machine.add_node("machine_node", vec!["prop1".to_string()]);
    
    // Экспортируем в KB
    let exported = integration.export_to_kb();
    
    assert_eq!(exported, 1);
    assert_eq!(integration.knowledge_base.metadata.node_count, 1);
    
    println!("Exported {} nodes to KB", exported);
}

#[test]
fn test_social_synthesis() {
    let mut integration = SocialIntegration::new();
    
    let result = integration.social_synthesis(
        "freedom",
        "security",
        SynthesisStrategy::Hegelian,
    ).unwrap();
    
    assert!(!result.name.is_empty());
    assert!(integration.collective_trace.event_count() > 0);
    
    println!("Social synthesis: {}", result.name);
}

#[test]
fn test_federated_exchange() {
    let mut integration = SocialIntegration::new();
    
    integration.federated_exchange("m1", "m2", "node1");
    
    assert_eq!(integration.federation.exchange_count(), 1);
    assert!(integration.collective_trace.event_count() > 0);
    
    println!("Federated exchange completed");
}

#[test]
fn test_integration_stats() {
    let mut integration = SocialIntegration::new();
    
    integration.machine.add_node("node1", vec![]);
    integration.knowledge_base.add_node(KnowledgeNode {
        id: "kb1".to_string(),
        label: "KB Node".to_string(),
        description: "test".to_string(),
        properties: std::collections::HashMap::new(),
        relations: vec![],
    });
    
    let stats = integration.integration_stats();
    
    assert_eq!(stats.machine_nodes, 1);
    assert_eq!(stats.kb_nodes, 1);
    
    println!("Stats: {} machine nodes, {} KB nodes", 
             stats.machine_nodes, stats.kb_nodes);
}

#[test]
fn test_social_bridge() {
    let mut bridge = SocialBridge::new();
    
    // Добавляем узлы
    bridge.integration.machine.add_node("freedom", vec![]);
    bridge.integration.machine.add_node("security", vec![]);
    bridge.integration.machine.metrics.stability_ratio = 0.3;
    bridge.integration.machine.metrics.contradiction_index = 0.8;
    
    // Создаем противоречие
    let mut c = Contradiction::new(
        "freedom".to_string(),
        "security".to_string(),
        ContradictionKind::Logical,
    );
    c.update_severity(0.8, 0.5);
    c.update_severity(0.8, 0.4);
    c.update_severity(0.8, 0.3);
    
    // Запускаем Платона с социальным контекстом
    let result = bridge.run_plato_social(&[c]);
    
    assert!(result.iterations > 0);
    println!("Social bridge: {} iterations", result.iterations);
}
'''

with open('grammalang-core/tests/social/phase5_tests.rs', 'w', encoding='utf-8') as f:
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

with open('grammalang-core/tests/social/mod.rs', 'w', encoding='utf-8') as f:
    f.write(test_mod)
print("test mod.rs updated")

print("\nAll v0.8 Phase 5 files created!")
