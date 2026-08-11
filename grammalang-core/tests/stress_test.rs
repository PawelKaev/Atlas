use grammalang_core::cascade::{evaluate_cascade, TensionVector};
use grammalang_core::ast::EthicalSystem;
use grammalang_core::lefebvre::ReactorPhase;

use std::fs;
use std::path::PathBuf;

fn load_fixture(filename: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(filename);
    fs::read_to_string(&path).expect(&format!("Failed to read fixture: {:?}", path))
}

fn split_paragraphs(text: &str) -> Vec<String> {
    text.split("\r\n\r\n")
        .flat_map(|p| p.split("\n\n"))
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect()
}

fn paragraph_to_state(text: &str) -> (f64, f64, f64) {
    let len = text.len() as f64;
    let x = (len % 100.0) / 100.0 + 0.5;
    let y = ((len * 3.0) % 100.0) / 100.0 + 0.3;
    let z = ((len * 7.0) % 100.0) / 100.0 + 0.1;
    (x.clamp(0.0, 1.0), y.clamp(0.0, 1.0), z.clamp(0.0, 1.0))
}

#[test]
fn stress_test_chapter_vs_paragraphs() {
    let text = load_fixture("chapter_sample.txt");
    let paragraphs = split_paragraphs(&text);

    println!("\n========== STRESS TEST: Global vs Paragraph-level ==========");
    println!("Paragraphs: {}", paragraphs.len());

    // --- Global run ---
    let global_state = paragraph_to_state(&text);
    println!("\n--- GLOBAL ---");
    println!("State: ({:.3}, {:.3}, {:.3})", global_state.0, global_state.1, global_state.2);

    let global_result = evaluate_cascade(EthicalSystem::Second, 3, global_state);
    print_tension_vector("GLOBAL", &global_result);

    // --- Paragraph-level run ---
    println!("\n--- PARAGRAPH-LEVEL ---");
    let mut paragraph_results = Vec::new();

    for (i, para) in paragraphs.iter().enumerate() {
        let state = paragraph_to_state(para);
        let result = evaluate_cascade(EthicalSystem::Second, 3, state);
        print_tension_vector(&format!("PARA[{}]", i), &result);
        paragraph_results.push((i, para.clone(), result));
    }

    // --- Comparison ---
    println!("\n========== COMPARISON ==========");

    let global_holdbreaks: Vec<usize> = global_result
        .phases
        .iter()
        .enumerate()
        .filter(|(_, p)| **p == ReactorPhase::HoldBreak)
        .map(|(i, _)| i)
        .collect();

    println!("Global HoldBreak steps: {:?}", global_holdbreaks);

    let para_holdbreaks: Vec<(usize, usize)> = paragraph_results
        .iter()
        .flat_map(|(i, _, r)| {
            r.phases
                .iter()
                .enumerate()
                .filter(|(_, p)| **p == ReactorPhase::HoldBreak)
                .map(move |(step, _)| (*i, step))
        })
        .collect();

    println!("Paragraph HoldBreak steps (para, step): {:?}", para_holdbreaks);

    if !global_holdbreaks.is_empty() && !para_holdbreaks.is_empty() {
        println!(
            "\nHYPOTHESIS: Does micro-analysis preserve macro-HoldBreak? \
            Global HoldBreak at step {:?}. {} paragraph-level HoldBreaks found.",
            global_holdbreaks,
            para_holdbreaks.len()
        );
    }

    // Assertions
    assert_eq!(global_result.tensions.len(), 3);
    for (_, _, r) in &paragraph_results {
        assert_eq!(r.tensions.len(), 3);
    }

    // Save trace to JSON
    let output = serde_json::json!({
        "test": "chapter_vs_paragraphs",
        "paragraph_count": paragraphs.len(),
        "global": {
            "tensions": global_result.tensions,
            "derivatives": global_result.derivatives,
            "phases": global_result.phases.iter().map(|p| format!("{:?}", p)).collect::<Vec<_>>(),
            "holdbreak_steps": global_holdbreaks,
        },
        "paragraphs": paragraph_results.iter().map(|(i, text, r)| {
            serde_json::json!({
                "index": i,
                "text_preview": &text[..text.len().min(80)],
                "tensions": r.tensions,
                "derivatives": r.derivatives,
                "phases": r.phases.iter().map(|p| format!("{:?}", p)).collect::<Vec<_>>(),
            })
        }).collect::<Vec<_>>(),
    });

    let out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("stress_test_output.json");
    fs::write(&out_path, serde_json::to_string_pretty(&output).unwrap())
        .expect("Failed to write output");
    println!("\nTrace saved to {:?}", out_path);
}

fn print_tension_vector(label: &str, tv: &TensionVector) {
    println!("[{}]", label);
    for i in 0..tv.tensions.len() {
        println!(
            "  step {}: tension={:.4}, deriv={:.4}, phase={:?}, trace=\"{}\"",
            i,
            tv.tensions[i],
            tv.derivatives[i],
            tv.phases[i],
            tv.etymological_trace.get(i).unwrap_or(&"".to_string())
        );
    }
}
