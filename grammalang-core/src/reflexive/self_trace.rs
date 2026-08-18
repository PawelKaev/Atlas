use serde::{Serialize, Deserialize};

/// SelfTrace - машина записывает процесс своего мышления
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfTrace {
    pub stages: Vec<ThinkingStage>,
    pub self_awareness_level: f32,
    pub conscious_actions: Vec<ConsciousAction>,
    pub meta_knowledge: Vec<MetaKnowledge>,
    pub reflection_history: Vec<ReflectionRecord>,
    pub cognitive_state: CognitiveState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingStage {
    pub timestamp: u64,
    pub stage_type: ThinkingStageType,
    pub description: String,
    pub reflection_level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThinkingStageType {
    Perception,
    Analysis,
    Synthesis,
    Reflection,
    Awareness,
    MetaCognition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousAction {
    pub action_id: String,
    pub description: String,
    pub was_conscious: bool,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaKnowledge {
    pub fact: String,
    pub confidence: f32,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionRecord {
    pub timestamp: u64,
    pub reflection_level: usize,
    pub subject: String,
    pub insight: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CognitiveState {
    pub total_stages: usize,
    pub reflection_count: usize,
    pub awareness_count: usize,
    pub metacognition_count: usize,
    pub avg_reflection_level: f32,
}

impl SelfTrace {
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
            self_awareness_level: 0.0,
            conscious_actions: Vec::new(),
            meta_knowledge: Vec::new(),
            reflection_history: Vec::new(),
            cognitive_state: CognitiveState::default(),
        }
    }
    
    pub fn record_stage(
        &mut self,
        stage_type: ThinkingStageType,
        description: &str,
        reflection_level: usize,
    ) {
        self.stages.push(ThinkingStage {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            stage_type,
            description: description.to_string(),
            reflection_level,
        });
        
        self.update_cognitive_state();
        self.update_awareness();
    }
    
    pub fn record_action(&mut self, action_id: &str, description: &str, was_conscious: bool) {
        self.conscious_actions.push(ConsciousAction {
            action_id: action_id.to_string(),
            description: description.to_string(),
            was_conscious,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
    }
    
    pub fn add_meta_knowledge(&mut self, fact: &str, confidence: f32, source: &str) {
        self.meta_knowledge.push(MetaKnowledge {
            fact: fact.to_string(),
            confidence,
            source: source.to_string(),
        });
    }
    
    /// Запись рефлексии
    pub fn record_reflection(
        &mut self,
        reflection_level: usize,
        subject: &str,
        insight: &str,
    ) {
        self.reflection_history.push(ReflectionRecord {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            reflection_level,
            subject: subject.to_string(),
            insight: insight.to_string(),
        });
    }
    
    /// Обновление когнитивного состояния
    fn update_cognitive_state(&mut self) {
        let total = self.stages.len();
        let reflections = self.stages.iter()
            .filter(|s| matches!(s.stage_type, ThinkingStageType::Reflection))
            .count();
        let awareness = self.stages.iter()
            .filter(|s| matches!(s.stage_type, ThinkingStageType::Awareness))
            .count();
        let metacognition = self.stages.iter()
            .filter(|s| matches!(s.stage_type, ThinkingStageType::MetaCognition))
            .count();
        
        let total_level: usize = self.stages.iter()
            .map(|s| s.reflection_level)
            .sum();
        
        self.cognitive_state = CognitiveState {
            total_stages: total,
            reflection_count: reflections,
            awareness_count: awareness,
            metacognition_count: metacognition,
            avg_reflection_level: if total > 0 {
                total_level as f32 / total as f32
            } else {
                0.0
            },
        };
    }
    
    fn update_awareness(&mut self) {
        let total = self.stages.len();
        if total == 0 {
            self.self_awareness_level = 0.0;
            return;
        }
        
        let weighted = self.stages.iter().map(|s| {
            let weight = match s.stage_type {
                ThinkingStageType::Perception => 0.1,
                ThinkingStageType::Analysis => 0.2,
                ThinkingStageType::Synthesis => 0.3,
                ThinkingStageType::Reflection => 0.6,
                ThinkingStageType::Awareness => 0.8,
                ThinkingStageType::MetaCognition => 1.0,
            };
            weight * (1.0 + s.reflection_level as f32 * 0.5)
        }).sum::<f32>();
        
        let max_weight = self.stages.len() as f32 * 1.5;
        self.self_awareness_level = (weighted / max_weight).min(1.0);
    }
    
    /// Анализ паттернов мышления
    pub fn analyze_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        
        if self.cognitive_state.reflection_count > 0 {
            patterns.push(format!(
                "Reflection pattern: {} reflections (avg level {:.1})",
                self.cognitive_state.reflection_count,
                self.cognitive_state.avg_reflection_level
            ));
        }
        
        if self.cognitive_state.metacognition_count > 0 {
            patterns.push(format!(
                "Metacognition pattern: {} meta-cognitive acts",
                self.cognitive_state.metacognition_count
            ));
        }
        
        if self.conscious_action_count() > 0 {
            patterns.push(format!(
                "Conscious action pattern: {} of {} actions were conscious",
                self.conscious_action_count(),
                self.conscious_actions.len()
            ));
        }
        
        patterns
    }
    
    /// Получение сводки самосознания
    pub fn self_awareness_report(&self) -> String {
        format!(
            "Self-awareness: {:.2}\nStages: {}\nReflections: {}\nMetacognition: {}\nMeta-knowledge: {}",
            self.self_awareness_level,
            self.cognitive_state.total_stages,
            self.cognitive_state.reflection_count,
            self.cognitive_state.metacognition_count,
            self.meta_knowledge.len()
        )
    }
    
    pub fn stages_at_level(&self, level: usize) -> Vec<&ThinkingStage> {
        self.stages.iter()
            .filter(|s| s.reflection_level == level)
            .collect()
    }
    
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
    
    pub fn conscious_action_count(&self) -> usize {
        self.conscious_actions.iter()
            .filter(|a| a.was_conscious)
            .count()
    }
    
    pub fn meta_knowledge_count(&self) -> usize {
        self.meta_knowledge.len()
    }
    
    pub fn reflection_history_count(&self) -> usize {
        self.reflection_history.len()
    }
    
    pub fn is_self_aware(&self, threshold: f32) -> bool {
        self.self_awareness_level >= threshold
    }
}
