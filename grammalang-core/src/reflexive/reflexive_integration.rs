use serde::{Serialize, Deserialize};
use crate::ontology::*;
use crate::social::*;
use super::reflection::*;
use super::self_trace::*;
use super::meta_synthesis::*;
use super::auto_genealogy::*;

/// Полная интеграция рефлексивной системы с v0.7 (ontology) и v0.8 (social)
pub struct ReflexiveIntegration {
    pub reflexive_system: ReflexiveSystem,
    pub machine: MachineState,
    pub social_integration: SocialIntegration,
    pub state: IntegrationState,
}

#[derive(Debug, Clone)]
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
            "Integration:\n  Reflexive syntheses: {}\n  Social exchanges: {}\n  Integration level: {}\n  Self-awareness: {:.2}",
            self.state.total_reflexive_syntheses,
            self.state.total_social_exchanges,
            self.state.integration_level,
            self.reflexive_system.state.self_awareness,
        )
    }
}

#[derive(Debug)]
pub struct ReflexiveSocialReport {
    pub synthesis_name: String,
    pub reflection: String,
    pub awareness: String,
    pub final_awareness: f32,
    pub total_reflexive_syntheses: usize,
    pub total_social_exchanges: usize,
}
