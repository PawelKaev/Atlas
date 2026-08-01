// lib.rs — GrammaLang Core v0.4
// Rust-ядро для анализа воли (Парменид/Гераклит)
// Экспорт функций в Python через PyO3

mod error;
mod ontology;
mod trace;
mod will_markers;
mod syntax_modifiers;

use pyo3::prelude::*;

/// Модуль grammalang_core для Python
#[pymodule]
fn grammalang_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // v0.3 функции
    m.add_function(wrap_pyfunction!(will_markers::set_markers, m)?)?;
    m.add_function(wrap_pyfunction!(will_markers::load_markers_from_files, m)?)?;
    m.add_function(wrap_pyfunction!(will_markers::analyze_will, m)?)?;
    m.add_function(wrap_pyfunction!(will_markers::get_current_markers, m)?)?;
    // v0.4 функции
    m.add_function(wrap_pyfunction!(syntax_modifiers::apply_syntax_modifiers, m)?)?;
    Ok(())
}
