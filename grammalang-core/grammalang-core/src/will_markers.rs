// will_markers.rs — Лексический анализ маркеров воли (Парменид/Гераклит)
// GrammaLang Core v0.3/v0.4

use pyo3::prelude::*;

/// Хранилище маркеров
static mut PARMENIDES_MARKERS: Vec<String> = Vec::new();
static mut HERACLITUS_MARKERS: Vec<String> = Vec::new();

/// Загружает маркеры из Python-списков
#[pyfunction]
pub fn set_markers(
    parmenides: Vec<String>,
    heraclitus: Vec<String>,
) -> PyResult<()> {
    unsafe {
        PARMENIDES_MARKERS = parmenides;
        HERACLITUS_MARKERS = heraclitus;
    }
    Ok(())
}

/// Загружает маркеры из файлов
#[pyfunction]
pub fn load_markers_from_files(
    p_file: &str,
    h_file: &str,
) -> PyResult<()> {
    let p_content = std::fs::read_to_string(p_file).unwrap_or_default();
    let h_content = std::fs::read_to_string(h_file).unwrap_or_default();

    let p_markers: Vec<String> = p_content
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| l.trim().to_lowercase())
        .collect();

    let h_markers: Vec<String> = h_content
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| l.trim().to_lowercase())
        .collect();

    unsafe {
        PARMENIDES_MARKERS = p_markers;
        HERACLITUS_MARKERS = h_markers;
    }
    Ok(())
}

/// Возвращает текущие маркеры
#[pyfunction]
pub fn get_current_markers() -> PyResult<(Vec<String>, Vec<String>)> {
    unsafe {
        Ok((PARMENIDES_MARKERS.clone(), HERACLITUS_MARKERS.clone()))
    }
}

/// Анализирует список предложений и возвращает индексы воли
/// Каждое предложение: от -1.0 (Гераклит) до +1.0 (Парменид)
#[pyfunction]
pub fn analyze_will(sentences: Vec<String>) -> PyResult<Vec<f64>> {
    let (p_markers, h_markers) = unsafe {
        (PARMENIDES_MARKERS.clone(), HERACLITUS_MARKERS.clone())
    };

    // Маркеры по умолчанию (если не загружены извне)
    let p_default: Vec<&str> = vec![
        "create", "write", "set", "execute", "move", "copy", "call",
        "бытие", "истина", "единый", "абсолют", "вечный", "сущность",
        "должен", "надо", "обязан", "закон",
    ];
    let h_default: Vec<&str> = vec![
        "get", "check", "test", "compare", "wait", "sleep", "if",
        "становление", "изменение", "текучесть", "многое", "относительный",
        "может", "возможно", "случайно", "хаос",
    ];

    let p: Vec<&str> = if p_markers.is_empty() {
        p_default
    } else {
        p_markers.iter().map(|s| s.as_str()).collect()
    };
    let h: Vec<&str> = if h_markers.is_empty() {
        h_default
    } else {
        h_markers.iter().map(|s| s.as_str()).collect()
    };

    let mut indices = Vec::with_capacity(sentences.len());

    for sentence in &sentences {
        let lower = sentence.to_lowercase();
        let words: Vec<&str> = lower.split_whitespace().collect();

        let mut p_count: f64 = 0.0;
        let mut h_count: f64 = 0.0;

        for word in &words {
            for marker in &p {
                if word.contains(marker) {
                    p_count += 1.0;
                }
            }
            for marker in &h {
                if word.contains(marker) {
                    h_count += 1.0;
                }
            }
        }

        let total = p_count + h_count;
        let index: f64 = if total > 0.0 {
            (p_count - h_count) / total
        } else {
            0.0
        };

        // Ограничение диапазона [-1.0, 1.0]
        let clamped = if index < -1.0 {
            -1.0
        } else if index > 1.0 {
            1.0
        } else {
            index
        };

        indices.push(clamped);
    }

    Ok(indices)
}
