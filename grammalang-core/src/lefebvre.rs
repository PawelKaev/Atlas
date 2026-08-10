// grammalang-core/src/lefebvre.rs

use crate::ast::EthicalSystem;

// ============ Reactor Phases ============

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactorPhase {
    /// Phase 1: Traditional grammatical form is injected
    Injektion,
    /// Phase 2: Form undergoes etymological/decompositional irradiation
    Bestrahlung,
    /// Phase 3: Grammatically unstable environment — neither noun nor verb
    Plasma,
    /// Phase 4: Meaning crystallizes into a new, but shaken, grammatical form
    Niederschlag,
    /// Critical point: second derivative of intention passes through zero
    HoldBreak,
}

// ============ Algebra of Conscience ============

/// Base algebra of conscience: operation of combining meanings.
/// First system: a + b - ab (compromise = evil)
/// Second system: min(1, a + b) (compromise = good)
pub trait AlgebraOfConscience {
    fn combine(a: f64, b: f64) -> f64;
    fn golden_threshold() -> f64;
}

/// Object-safe version for dynamic dispatch (benchmarks only).
pub trait AlgebraOfConscienceDyn {
    fn combine(&self, a: f64, b: f64) -> f64;
    fn golden_threshold(&self) -> f64;
}

impl<T: AlgebraOfConscience> AlgebraOfConscienceDyn for T {
    fn combine(&self, a: f64, b: f64) -> f64 {
        T::combine(a, b)
    }
    fn golden_threshold(&self) -> f64 {
        T::golden_threshold()
    }
}

/// Trait extension: links algebra with the enum variant for dispatching
pub trait Ethics: AlgebraOfConscience {
    const SYSTEM: EthicalSystem;
}

// ----- First ethical system -----

pub struct FirstEthics;

impl AlgebraOfConscience for FirstEthics {
    fn combine(a: f64, b: f64) -> f64 {
        a + b - a * b
    }
    fn golden_threshold() -> f64 {
        (5.0_f64.sqrt() - 1.0) / 2.0 // ≈ 0.618
    }
}

impl Ethics for FirstEthics {
    const SYSTEM: EthicalSystem = EthicalSystem::First;
}

// ----- Second ethical system -----

pub struct SecondEthics;

impl AlgebraOfConscience for SecondEthics {
    fn combine(a: f64, b: f64) -> f64 {
        (a + b).min(1.0)
    }
    fn golden_threshold() -> f64 {
        0.5
    }
}

impl Ethics for SecondEthics {
    const SYSTEM: EthicalSystem = EthicalSystem::Second;
}

// ----- Computational functions -----

/// Computes tension for one step of reflection.
pub fn calculate_tension<E: AlgebraOfConscience>(x: f64, y: f64, z: f64) -> f64 {
    let x = x.clamp(0.0, 1.0);
    let y = y.clamp(0.0, 1.0);
    let z = z.clamp(0.0, 1.0);
    let term1 = E::combine(x, 1.0 - y);
    let term2 = E::combine(y, 1.0 - z);
    let tension = term1 + term2;

    let threshold = E::golden_threshold();
    if (tension - threshold).abs() < 0.05 {
        #[cfg(debug_assertions)]
        eprintln!(
            "[GrammaLang] Ontological harmony: tension={:.4}, threshold={:.4}",
            tension, threshold
        );
    }

    tension
}

/// Determines the reactor phase based on tension, derivative, and second derivative.
pub fn determine_phase(
    tension: f64,
    derivative: Option<f64>,
    second_derivative: Option<f64>,
) -> ReactorPhase {
    match (derivative, second_derivative) {
        // First step — injection
        (None, _) => ReactorPhase::Injektion,
        // Second step — irradiation (etymological decomposition)
        (Some(_), None) => ReactorPhase::Bestrahlung,
        // Later steps — check for plasma, holdbreak, or crystallization
        (Some(dy), Some(d2y)) => {
            // HoldBreak: second derivative crosses zero while first derivative is non-zero
            if d2y.abs() < 1e-6 && dy.abs() > 1e-6 {
                ReactorPhase::HoldBreak
            }
            // Plasma: high tension and high absolute derivative (grammatical instability)
            else if tension > 1.0 && dy.abs() > 0.2 {
                ReactorPhase::Plasma
            }
            // Otherwise — crystallization
            else {
                ReactorPhase::Niederschlag
            }
        }
    }
}
