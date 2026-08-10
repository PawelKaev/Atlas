// grammalang-core/src/evaluator.rs

use crate::ast::*;
use crate::cascade::{evaluate_cascade, TensionVector};
use crate::context::OntoContext;

/// Result of evaluating a ReflexiveCascade node.
#[derive(Debug, Clone)]
pub enum EvaluationResult {
    /// Final result — cascade completed, value is ready.
    Complete(f64),
    /// The evaluator requests recontextualization from the host (Atlas/Qwen).
    /// The host should:
    /// 1. Extract the TensionVector
    /// 2. Send it to the LLM for semantic analysis
    /// 3. Update the OntoContext with new state
    /// 4. Re-evaluate the same node
    RequiresRecontextualization {
        tensions: TensionVector,
        subject_name: String,
        context_name: String,
        iteration: usize,
        max_iterations: usize,
    },
}

/// Evaluates an AST node, handling ReflexiveCascade via Lefebvre math.
/// Supports the hermeneutic loop: if recontextualization is needed,
/// returns RequiresRecontextualization instead of the final value.
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

            // Update subject state after cascade
            let new_state = (last, subj_state.0, subj_state.1);
            onto.update_state(&subj_name, new_state);

            // Check if we need recontextualization
            let has_holdbreak = history.phases.iter().any(|p| *p == crate::lefebvre::ReactorPhase::HoldBreak);
            let has_plasma = history.phases.iter().any(|p| *p == crate::lefebvre::ReactorPhase::Plasma);

            if (has_holdbreak || has_plasma) && iteration < max_iterations {
                // Request recontextualization — the host should consult LLM and re-evaluate
                Some(EvaluationResult::RequiresRecontextualization {
                    tensions: history,
                    subject_name: subj_name,
                    context_name: ctx_name,
                    iteration,
                    max_iterations,
                })
            } else {
                // Final result
                Some(EvaluationResult::Complete(last))
            }
        }
        _ => None,
    }
}

/// Extracts identifier name from an Ast node.
fn extract_identifier(node: &Ast) -> Option<String> {
    match node {
        Ast::Variable { name, .. } => Some(name.clone()),
        _ => None,
    }
}
