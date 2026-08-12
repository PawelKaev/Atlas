// grammalang-core/src/evaluator.rs

use crate::ast::*;
use crate::cascade::{evaluate_cascade, TensionVector};
use crate::context::OntoContext;
use crate::lefebvre::ReactorPhase;

/// Result of evaluating a node.
#[derive(Debug, Clone)]
pub enum EvaluationResult {
    /// Final result — value is ready.
    Complete(f64),
    /// Recontextualization requested (hermeneutic loop).
    RequiresRecontextualization {
        tensions: TensionVector,
        subject_name: String,
        context_name: String,
        iteration: usize,
        max_iterations: usize,
    },
    /// Praxis verification: synthesis passed or failed material test.
    PraxisVerdict {
        synthesis_name: String,
        passed: bool,
        reason: String,
        tension_after: f64,
    },
}

/// Evaluates an AST node.
pub fn evaluate(
    node: &Ast,
    onto: &mut OntoContext,
    iteration: usize,
    max_iterations: usize,
) -> Option<EvaluationResult> {
    match node {
        Ast::ReflexiveCascade { subject, context, ethics, depth, .. } => {
            let subj_name = extract_identifier(subject)?;
            let ctx_name = extract_identifier(context)?;
            let subj_state = onto.get_state(&subj_name)?;
            let history = evaluate_cascade(*ethics, *depth, subj_state);
            let tensions = &history.tensions;
            let last = *tensions.last()?;
            let new_state = (last, subj_state.0, subj_state.1);
            onto.update_state(&subj_name, new_state);

            let has_holdbreak = history.phases.iter().any(|p| *p == ReactorPhase::HoldBreak);
            let has_plasma = history.phases.iter().any(|p| *p == ReactorPhase::Plasma);

            if (has_holdbreak || has_plasma) && iteration < max_iterations {
                Some(EvaluationResult::RequiresRecontextualization {
                    tensions: history,
                    subject_name: subj_name,
                    context_name: ctx_name,
                    iteration,
                    max_iterations,
                })
            } else {
                Some(EvaluationResult::Complete(last))
            }
        }

        Ast::PraxisBinding { synthesis, context, .. } => {
            let synth_name = extract_identifier(synthesis).unwrap_or_else(|| "synthesis".to_string());
            let ctx_name = extract_identifier(context).unwrap_or_else(|| "context".to_string());
            let synth_state = onto.get_state(&synth_name);
            let ctx_state = onto.get_state(&ctx_name);

            match (synth_state, ctx_state) {
                (Some(ss), Some(cs)) => {
                    let resistance = ss.0 - cs.2;
                    let passed = resistance > 0.0;
                    let new_tension = if passed {
                        (ss.0 * 0.8 + 0.2).min(2.0)
                    } else {
                        (ss.0 * 0.3).max(0.0)
                    };
                    onto.update_state(&synth_name, (new_tension, ss.1, ss.2));

                    Some(EvaluationResult::PraxisVerdict {
                        synthesis_name: synth_name,
                        passed,
                        reason: if passed {
                            format!("Synthesis holds: tension {:.3} > pressure {:.3}", ss.0, cs.2)
                        } else {
                            format!("Synthesis collapses: tension {:.3} < pressure {:.3}", ss.0, cs.2)
                        },
                        tension_after: new_tension,
                    })
                }
                _ => {
                    Some(EvaluationResult::PraxisVerdict {
                        synthesis_name: synth_name,
                        passed: false,
                        reason: "Missing state for synthesis or context".to_string(),
                        tension_after: 0.0,
                    })
                }
            }
        }

        Ast::RevolutionBinding { old_field, new_quality, .. } => {
            let old_name = extract_identifier(old_field).unwrap_or_else(|| "old_field".to_string());
            let new_name = extract_identifier(new_quality).unwrap_or_else(|| "new_quality".to_string());
            let old_state = onto.get_state(&old_name);
            let new_state = onto.get_state(&new_name);

            match (old_state, new_state) {
                (Some(os), Some(ns)) => {
                    // Destroy old field
                    onto.update_state(&old_name, (0.0, 0.0, 0.0));
                    // Elevate new quality — enhanced by the tension of the destroyed old field
                    let elevated = ns.0.max(os.0);
                    onto.update_state(&new_name, (elevated, ns.1, ns.2));

                    let reason = format!(
                        "Revolution: '{}' destroyed (tension {:.3}), '{}' elevated (tension {:.3})",
                        old_name, os.0, new_name, elevated
                    );

                    Some(EvaluationResult::PraxisVerdict {
                        synthesis_name: new_name,
                        passed: true,
                        reason,
                        tension_after: elevated,
                    })
                }
                _ => {
                    Some(EvaluationResult::PraxisVerdict {
                        synthesis_name: new_name,
                        passed: false,
                        reason: "Missing state for old field or new quality".to_string(),
                        tension_after: 0.0,
                    })
                }
            }
        }

        _ => None,
    }
}

fn extract_identifier(node: &Ast) -> Option<String> {
    match node {
        Ast::Variable { name, .. } => Some(name.clone()),
        _ => None,
    }
}
