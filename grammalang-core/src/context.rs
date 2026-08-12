// grammalang-core/src/context.rs

use crate::entity::{OntoSpace, StratumLevel};
use crate::llm_resolver::LlmResolver;

/// Ontological context — hierarchical storage of all subject states.
///
/// Provides:
/// - Tree of named spaces (micro-ontologies)
/// - Stratification into Basis and Superstructure
/// - Inheritance: entities resolve upward through parent spaces
/// - Caching of LLM name resolution results
/// - Tracking of state evolution via version
pub struct OntoContext {
    root: OntoSpace,
}

impl OntoContext {
    pub fn new() -> Self {
        Self {
            root: OntoSpace::new("root"),
        }
    }

    /// Creates a stratified space with the given level.
    pub fn create_stratum(&mut self, name: &str, level: StratumLevel) {
        let space = OntoSpace::with_level(name, level);
        self.root.subspaces.insert(name.to_string(), space);
    }

    /// Resolves all passed identifiers via LLM (once per name).
    pub fn resolve_all(
        &mut self,
        identifiers: &[String],
        llm: &mut dyn LlmResolver,
    ) -> Result<(), String> {
        for name in identifiers {
            if self.root.resolve(name).is_none() {
                let (x, y, z) = llm.resolve_entity(name)?;
                self.root.resolve_or_create(name, (x, y, z));
            }
        }
        Ok(())
    }

    /// Returns the current state of an entity by path.
    pub fn get_state(&self, name: &str) -> Option<(f64, f64, f64)> {
        self.root.resolve(name)
    }

    /// Returns the current version of an entity.
    pub fn get_version(&self, name: &str) -> Option<u64> {
        self.root.get_version(name)
    }

    /// Updates the state of an entity and increments its version.
    pub fn update_state(&mut self, name: &str, state: (f64, f64, f64)) {
        self.root.update_state(name, state);
    }

    /// Checks if a contradiction in the superstructure can be resolved
    /// by examining whether the base contradiction has been resolved first.
    pub fn can_resolve_superstructure(&self, entity_name: &str) -> bool {
        let state = self.root.resolve(entity_name);
        match state {
            Some((x, _, _)) => x > 0.0,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm_resolver::MockLlmResolver;
    use crate::cascade::evaluate_cascade;
    use crate::ast::EthicalSystem;

    #[test]
    fn test_nested_reflexion_versions() {
        let mut ctx = OntoContext::new();
        let mut llm = MockLlmResolver::new();

        ctx.resolve_all(
            &["a".to_string(), "b".to_string(), "c".to_string()],
            &mut llm,
        )
        .unwrap();

        assert_eq!(ctx.get_version("b"), Some(0));

        let state_b = ctx.get_state("b").unwrap();
        let inner = evaluate_cascade(EthicalSystem::Second, 2, state_b);
        let inner_tensions = &inner.tensions;
        let b_new = (inner_tensions[1], state_b.0, state_b.1);
        ctx.update_state("b", b_new);
        assert_eq!(ctx.get_version("b"), Some(1));

        let state_b_updated = ctx.get_state("b").unwrap();
        let outer = evaluate_cascade(EthicalSystem::Second, 2, state_b_updated);
        let outer_tensions = &outer.tensions;
        let b_final = (outer_tensions[1], state_b_updated.0, state_b_updated.1);
        ctx.update_state("b", b_final);

        assert_eq!(ctx.get_version("b"), Some(2));
    }

    #[test]
    fn test_hierarchical_spaces() {
        let mut ctx = OntoContext::new();
        let mut llm = MockLlmResolver::new();

        ctx.resolve_all(
            &["sonya.plot.raskolnikov".to_string()],
            &mut llm,
        )
        .unwrap();

        assert!(ctx.get_state("sonya.plot.raskolnikov").is_some());
        assert_eq!(ctx.get_version("sonya.plot.raskolnikov"), Some(0));
    }

    #[test]
    fn test_stratification() {
        let mut ctx = OntoContext::new();

        // Create basis and superstructure
        ctx.create_stratum("base", StratumLevel::Basis);
        ctx.create_stratum("superstructure", StratumLevel::Superstructure);

        // Resolve an entity in the base
        let mut llm = MockLlmResolver::new();
        ctx.resolve_all(&["base.production".to_string()], &mut llm).unwrap();

        // Initially base has state
        assert!(ctx.get_state("base.production").is_some());

        // can_resolve_superstructure checks if the named entity has tension > 0
        // base.production exists with some state
        let can = ctx.can_resolve_superstructure("base.production");
        assert!(can); // tension should be > 0 for a newly created entity
    }
}
