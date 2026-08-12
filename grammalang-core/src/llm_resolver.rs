// grammalang-core/src/llm_resolver.rs

/// Interface for resolving ontological entities via LLM.
/// In production — calls Qwen 3 16B, in tests — mock.
pub trait LlmResolver {
    /// Returns the initial state (x, y, z) for an entity by name.
    /// All values ∈ [0, 1].
    fn resolve_entity(&mut self, name: &str) -> Result<(f64, f64, f64), String>;
}

/// Mock resolver for tests.
/// Returns fixed values depending on the name.
pub struct MockLlmResolver;

impl MockLlmResolver {
    pub fn new() -> Self {
        Self
    }
}

impl LlmResolver for MockLlmResolver {
    fn resolve_entity(&mut self, name: &str) -> Result<(f64, f64, f64), String> {
        match name {
            "Raskolnikov" => Ok((0.8, 0.9, 0.1)),
            "Crone" => Ok((0.2, 0.1, 0.9)),
            "Sonya" => Ok((0.9, 0.3, 0.7)),
            "sonya.plot.raskolnikov" => Ok((0.8, 0.9, 0.1)),
            "sonya" => Ok((0.9, 0.3, 0.7)),
            "sonya.plot" => Ok((0.5, 0.5, 0.5)),
            "base.production" => Ok((0.7, 0.5, 0.3)),
            "a" => Ok((0.8, 0.5, 0.3)),
            "b" => Ok((0.6, 0.4, 0.2)),
            "c" => Ok((0.9, 0.7, 0.1)),
            _ => Err(format!("Unknown entity: {}", name)),
        }
    }
}
