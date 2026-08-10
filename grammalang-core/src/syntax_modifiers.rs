// syntax_modifiers.rs — Синтаксические модификаторы для GrammaLang v0.4
// Применяет поправки к лексическим индексам на основе синтаксической структуры

use pyo3::prelude::*;

/// Применяет синтаксические модификаторы к лексическим индексам воли.
#[pyfunction]
pub fn apply_syntax_modifiers(
    lexical_indices: Vec<f64>,
    syntax_types: Vec<String>,
) -> PyResult<Vec<f64>> {
    let mut result = Vec::with_capacity(lexical_indices.len());

    for (lex, syn_type) in lexical_indices.iter().zip(syntax_types.iter()) {
        let modifier = match syn_type.as_str() {
            "imperative"          => 0.4,
            "definition"          => 0.3,
            "assertion"           => 0.0,
            "open_question"       => -0.1,
            "real_conditional"    => -0.15,
            "rhetorical_question" => -0.3,
            "optative"            => -0.3,
            "counterfactual"      => -0.35,
            _                     => 0.0,
        };

        let final_idx = lex + modifier;
        // Ограничение диапазона [-1.0, 1.0]
        let clamped = if final_idx < -1.0 {
            -1.0
        } else if final_idx > 1.0 {
            1.0
        } else {
            final_idx
        };

        result.push(clamped);
    }

    Ok(result)
}
