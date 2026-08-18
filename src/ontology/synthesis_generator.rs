use super::target_ontology::*;
use super::synthesis_detector::*;

/// Ошибка генерации синтеза
#[derive(Debug, Clone)]
pub enum SynthesisError {
    /// Генерация не удалась
    GenerationFailed(String),
    /// LLM недоступен
    LLMUnavailable(String),
    /// Недостаточно данных
    InsufficientData(String),
    /// Таймаут
    Timeout,
}

/// Результат генерации синтеза
#[derive(Debug, Clone)]
pub struct SynthesisResult {
    /// Название нового понятия
    pub name: String,
    /// Описание
    pub description: String,
    /// Свойства
    pub properties: Vec<String>,
    /// Стратегия синтеза
    pub strategy: SynthesisStrategy,
    /// Уверенность (0.0 - 1.0)
    pub confidence: f32,
}

/// Трейт для генераторов синтеза
pub trait SynthesisGenerator {
    fn generate(&self, node_a: &str, node_b: &str, strategy: &SynthesisStrategy) -> Result<SynthesisResult, SynthesisError>;
}
