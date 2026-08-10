use std::time::Instant;

mod lefebvre {
    pub trait AlgebraOfConscience {
        fn combine(a: f64, b: f64) -> f64;
    }
    pub trait AlgebraOfConscienceDyn {
        fn combine(&self, a: f64, b: f64) -> f64;
    }
    impl<T: AlgebraOfConscience> AlgebraOfConscienceDyn for T {
        fn combine(&self, a: f64, b: f64) -> f64 { T::combine(a, b) }
    }
    pub trait Ethics: AlgebraOfConscience {}
    pub struct FirstEthics;
    impl AlgebraOfConscience for FirstEthics {
        fn combine(a: f64, b: f64) -> f64 { a + b - a * b }
    }
    impl Ethics for FirstEthics {}
    pub struct SecondEthics;
    impl AlgebraOfConscience for SecondEthics {
        fn combine(a: f64, b: f64) -> f64 { (a + b).min(1.0) }
    }
    impl Ethics for SecondEthics {}
    pub fn calculate_tension<E: AlgebraOfConscience>(x: f64, y: f64, z: f64) -> f64 {
        E::combine(x.clamp(0.0, 1.0), 1.0 - y.clamp(0.0, 1.0))
            + E::combine(y.clamp(0.0, 1.0), 1.0 - z.clamp(0.0, 1.0))
    }
}
use lefebvre::*;

fn run_cascade<E: Ethics, const DEPTH: usize>(mut state: (f64, f64, f64)) -> [f64; DEPTH] {
    let mut history = [0.0; DEPTH];
    for i in 0..DEPTH {
        history[i] = calculate_tension::<E>(state.0, state.1, state.2);
        state = (history[i], state.0, state.1);
    }
    history
}

fn run_cascade_dyn(system: &dyn AlgebraOfConscienceDyn, depth: usize, mut state: (f64, f64, f64)) -> Vec<f64> {
    let mut history = Vec::with_capacity(depth);
    for _ in 0..depth {
        history.push(system.combine(state.0, 1.0 - state.1) + system.combine(state.1, 1.0 - state.2));
        state = (history[history.len()-1], state.0, state.1);
    }
    history
}

#[test]
fn bench_static_vs_dyn() {
    const N: usize = 1_000_000;
    let state = (0.8, 0.9, 0.1);
    
    let start = Instant::now();
    for _ in 0..N {
        std::hint::black_box(run_cascade::<SecondEthics, 3>(std::hint::black_box(state)));
    }
    let static_time = start.elapsed();
    
    let system = &SecondEthics as &dyn AlgebraOfConscienceDyn;
    let start = Instant::now();
    for _ in 0..N {
        std::hint::black_box(run_cascade_dyn(std::hint::black_box(system), 3, std::hint::black_box(state)));
    }
    let dyn_time = start.elapsed();
    
    println!("Static dispatch: {:?}", static_time);
    println!("Dynamic dispatch: {:?}", dyn_time);
    println!("Ratio (dyn/static): {:.2}x", dyn_time.as_secs_f64() / static_time.as_secs_f64());
    
    // Утверждаем, что статика не медленнее динамики
    assert!(static_time <= dyn_time);
}
