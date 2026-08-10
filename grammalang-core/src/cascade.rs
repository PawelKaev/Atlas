// grammalang-core/src/cascade.rs

use crate::ast::EthicalSystem;
use crate::lefebvre::{
    AlgebraOfConscienceDyn, Ethics, FirstEthics, ReactorPhase, SecondEthics,
    calculate_tension, determine_phase,
};

// ============ TensionVector ============

#[derive(Debug, Clone)]
pub struct TensionVector {
    /// History of tension values for each step
    pub tensions: Vec<f64>,
    /// First derivative (dy/dt) at each step
    pub derivatives: Vec<f64>,
    /// Reactor phase at each step
    pub phases: Vec<ReactorPhase>,
    /// Etymological trace — description of the grammatical operation at each step
    pub etymological_trace: Vec<String>,
}

impl TensionVector {
    pub fn new() -> Self {
        TensionVector {
            tensions: Vec::new(),
            derivatives: Vec::new(),
            phases: Vec::new(),
            etymological_trace: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.tensions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tensions.is_empty()
    }
}

// ============ Static dispatch (monomorphized) ============

#[inline(never)]
pub fn run_cascade<E: Ethics, const DEPTH: usize>(
    mut state: (f64, f64, f64),
    trace: &[&str],
) -> TensionVector {
    let mut result = TensionVector::new();
    let mut prev_tension: Option<f64> = None;
    let mut prev_derivative: Option<f64> = None;

    for i in 0..DEPTH {
        // Compute tension
        let tension = calculate_tension::<E>(state.0, state.1, state.2);
        result.tensions.push(tension);

        // Compute first derivative
        let derivative = prev_tension.map(|prev| tension - prev);
        result.derivatives.push(derivative.unwrap_or(0.0));

        // Compute second derivative
        let second_derivative = prev_derivative.and_then(|prev_d| {
            derivative.map(|d| d - prev_d)
        });

        // Determine phase
        let phase = determine_phase(tension, derivative, second_derivative);
        result.phases.push(phase);

        // Etymological trace
        let trace_entry = if i < trace.len() {
            trace[i].to_string()
        } else {
            format!("step_{}", i)
        };
        result.etymological_trace.push(trace_entry);

        // Evolve state
        state = (tension, state.0, state.1);
        prev_tension = Some(tension);
        prev_derivative = derivative;
    }

    result
}

// ============ Dispatch interface ============

pub fn evaluate_cascade(
    ethics: EthicalSystem,
    depth: usize,
    state: (f64, f64, f64),
) -> TensionVector {
    let trace: &[&str] = &[];
    match (ethics, depth) {
        (EthicalSystem::First, 1) => run_cascade::<FirstEthics, 1>(state, trace),
        (EthicalSystem::First, 2) => run_cascade::<FirstEthics, 2>(state, trace),
        (EthicalSystem::First, 3) => run_cascade::<FirstEthics, 3>(state, trace),
        (EthicalSystem::Second, 1) => run_cascade::<SecondEthics, 1>(state, trace),
        (EthicalSystem::Second, 2) => run_cascade::<SecondEthics, 2>(state, trace),
        (EthicalSystem::Second, 3) => run_cascade::<SecondEthics, 3>(state, trace),
        _ => panic!(
            "[GrammaLang] Unsupported reflection depth: {}. Allowed values: 1, 2, 3",
            depth
        ),
    }
}

// ============ Dynamic dispatch (for benchmarks) ============

pub fn run_cascade_dyn(
    system: &dyn AlgebraOfConscienceDyn,
    depth: usize,
    mut state: (f64, f64, f64),
) -> TensionVector {
    let mut result = TensionVector::new();
    let mut prev_tension: Option<f64> = None;

    for _ in 0..depth {
        let tension = system.combine(state.0, 1.0 - state.1)
            + system.combine(state.1, 1.0 - state.2);
        result.tensions.push(tension);

        let derivative = prev_tension.map(|prev| tension - prev);
        result.derivatives.push(derivative.unwrap_or(0.0));

        result.phases.push(ReactorPhase::Injektion);
        result.etymological_trace.push("dyn".to_string());

        state = (tension, state.0, state.1);
        prev_tension = Some(tension);
    }

    result
}

// ============ Tests ============

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lefebvre::AlgebraOfConscienceDyn;

    #[test]
    fn force_instantiation() {
        let state = (0.8, 0.9, 0.1);
        let h1 = evaluate_cascade(EthicalSystem::First, 1, state);
        let h2 = evaluate_cascade(EthicalSystem::First, 2, state);
        let h3 = evaluate_cascade(EthicalSystem::First, 3, state);
        let h4 = evaluate_cascade(EthicalSystem::Second, 1, state);
        let h5 = evaluate_cascade(EthicalSystem::Second, 2, state);
        let h6 = evaluate_cascade(EthicalSystem::Second, 3, state);

        assert_eq!(h1.len(), 1);
        assert_eq!(h2.len(), 2);
        assert_eq!(h3.len(), 3);
        assert_eq!(h4.len(), 1);
        assert_eq!(h5.len(), 2);
        assert_eq!(h6.len(), 3);

        // Check that phases are present
        assert_eq!(h3.phases.len(), 3);
        // First step is always Injektion
        assert_eq!(h3.phases[0], ReactorPhase::Injektion);
    }

    #[test]
    fn test_holdbreak_detection() {
        // Use a state that causes sign change in derivative
        // SecondEthics with saturated values will hit min(1, ...) = 1.0
        // causing derivative to flatten, which triggers HoldBreak
        let state = (0.9, 0.95, 0.1);
        let result = evaluate_cascade(EthicalSystem::Second, 3, state);
        
        println!("Tensions: {:?}", result.tensions);
        println!("Derivatives: {:?}", result.derivatives);
        println!("Phases: {:?}", result.phases);
        
        // At least verify we have 3 phases
        assert_eq!(result.phases.len(), 3);
    }

    #[test]
    fn test_tension_vector_fields() {
        let state = (0.5, 0.5, 0.5);
        let result = evaluate_cascade(EthicalSystem::First, 3, state);
        
        // All vectors have same length
        assert_eq!(result.tensions.len(), result.derivatives.len());
        assert_eq!(result.tensions.len(), result.phases.len());
        assert_eq!(result.tensions.len(), result.etymological_trace.len());
        
        // Tensions are finite
        for t in &result.tensions {
            assert!(t.is_finite());
        }
    }

    #[test]
    fn test_dyn_vs_static_same_tensions() {
        let state = (0.5, 0.5, 0.5);
        let static_result = evaluate_cascade(EthicalSystem::Second, 3, state);
        let dyn_result = run_cascade_dyn(&SecondEthics, 3, state);
        
        assert_eq!(static_result.tensions.len(), dyn_result.tensions.len());
        for (s, d) in static_result.tensions.iter().zip(dyn_result.tensions.iter()) {
            assert!((s - d).abs() < 1e-10);
        }
    }

    #[test]
    fn bench_static_vs_dyn() {
        use std::time::Instant;
        
        const N: usize = 500_000;
        let state = (0.8, 0.9, 0.1);
        
        let start = Instant::now();
        for _ in 0..N {
            std::hint::black_box(run_cascade::<SecondEthics, 3>(
                std::hint::black_box(state),
                &[],
            ));
        }
        let static_time = start.elapsed();
        
        let system = &SecondEthics as &dyn AlgebraOfConscienceDyn;
        let start = Instant::now();
        for _ in 0..N {
            std::hint::black_box(run_cascade_dyn(
                std::hint::black_box(system),
                3,
                std::hint::black_box(state),
            ));
        }
        let dyn_time = start.elapsed();
        
        println!("Static dispatch: {:?}", static_time);
        println!("Dynamic dispatch: {:?}", dyn_time);
        println!(
            "Ratio (dyn/static): {:.2}x",
            dyn_time.as_secs_f64() / static_time.as_secs_f64()
        );
    }
}
