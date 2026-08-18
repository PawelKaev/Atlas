use crate::ontology::*;
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
