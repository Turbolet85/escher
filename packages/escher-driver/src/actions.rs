//! What each actionable node of the screen can take now: the verbs of the table that name an
//! element, each with the refusal a call of it would meet, or none.

use std::collections::HashMap;

use dioxus_native_dom::NodeId;

use crate::command::{ArgValue, Call, validate};
use crate::execute::{check_target, target_of};
use crate::refusal::Cause;
use crate::schema::{ArgKind, KEY_NAMES, Level, VERBS, VerbSpec};
use crate::session::Session;

/// One actionable node and what a call naming it would meet now. Built by
/// [`Session::actions`]. It holds ids, the table's verb words, closed causes and a bool: no
/// `NodeId`, text, name or value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionNode {
    /// The node's stable element id.
    pub id: String,
    /// The stable id of the element whose handler serves this node — a select for its options
    /// and groups — or `None` where the node is its own.
    pub owner: Option<String>,
    /// Whether the id is an author key rather than a positional path.
    pub keyed: bool,
    /// One reading per verb of the table that names an element, in the table's order.
    pub verbs: Vec<VerbReading>,
}

/// One verb a call can name a node by, and what that call would meet now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerbReading {
    /// The verb's word, as the table spells it.
    pub verb: &'static str,
    /// The cause the call would be refused with; `None` where it would be accepted.
    pub cause: Option<Cause>,
}

impl Session {
    /// Reads the screen of the held instance once and returns each actionable node on it, in
    /// snapshot pre-order (`DioxusDocument::actionable_nodes`: focusable, an interactive role,
    /// a current handler other than `mounted`, or served by an owner), with every instance verb
    /// of [`VERBS`] that takes an element id.
    ///
    /// For each verb it builds the call naming the node, any other argument the verb requires
    /// given a fixed value within its kind's bound, and decides it as [`Session::run`] does
    /// before acting: through [`validate`], then the one check that verb's arm meets once its
    /// target resolves. So a listed cause is the cause that call would return now, and a verb
    /// with none would be accepted — accepted, not that it would change anything.
    ///
    /// It runs no step, dispatches nothing and changes nothing; the ids it reads join the
    /// session's record of ids read, as a `snapshot`'s do.
    pub fn actions(&mut self) -> Vec<ActionNode> {
        let (harness, seen) = (&self.harness, &mut self.seen);
        let screen = harness.doc.snapshot();
        for node in screen.nodes() {
            seen.record(&node.id);
        }
        let mut elements: HashMap<String, NodeId> = HashMap::new();
        for (node, id) in harness.doc.element_ids() {
            elements.entry(id).or_insert(node);
        }
        let verbs: Vec<&'static VerbSpec> = target_verbs().collect();
        harness
            .doc
            .actionable_nodes()
            .into_iter()
            .map(|node| {
                let target = screen.get(&node.id).and_then(|listed| {
                    let element = elements.get(&node.id)?;
                    target_of(harness, listed, *element)
                });
                let verbs = verbs
                    .iter()
                    .map(|spec| {
                        let call = call_naming(spec, &node.id);
                        let cause = match validate(&call) {
                            Err(refusal) => Some(refusal.cause()),
                            // Every id of the screen was just recorded, so a call naming one that
                            // does not resolve would read it stale.
                            Ok(command) => match &target {
                                None => Some(Cause::Stale),
                                Some(target) => check_target(harness, &command, target)
                                    .err()
                                    .map(|refusal| refusal.cause()),
                            },
                        };
                        VerbReading {
                            verb: spec.name,
                            cause,
                        }
                    })
                    .collect();
                ActionNode {
                    id: node.id,
                    owner: node.owner,
                    keyed: node.keyed,
                    verbs,
                }
            })
            .collect()
    }
}

/// The instance verbs of the table that take an element id, in its order.
fn target_verbs() -> impl Iterator<Item = &'static VerbSpec> {
    VERBS
        .iter()
        .filter(|spec| spec.level == Level::Instance)
        .filter(|spec| spec.args.iter().any(|arg| arg.kind == ArgKind::Id))
}

/// The call of `verb` naming `id` by its id argument, every other required argument given a
/// fixed value within its kind's bound.
fn call_naming(verb: &VerbSpec, id: &str) -> Call {
    let args = verb
        .args
        .iter()
        .filter(|arg| arg.kind == ArgKind::Id || arg.required)
        .map(|arg| {
            let value = match arg.kind {
                ArgKind::Id => ArgValue::Text(id.to_string()),
                ArgKind::Text => ArgValue::Text(String::new()),
                ArgKind::Key => ArgValue::Text(KEY_NAMES[0].to_string()),
                ArgKind::Flag => ArgValue::Flag(false),
                ArgKind::Milliseconds | ArgKind::Part => ArgValue::Number(1),
                ArgKind::Name => ArgValue::Text("a".to_string()),
            };
            (arg.name.to_string(), value)
        })
        .collect();
    Call {
        verb: verb.name.to_string(),
        args,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_call_named_for_a_target_verb_validates() {
        let verbs: Vec<&str> = target_verbs().map(|spec| spec.name).collect();
        assert_eq!(verbs, ["click", "type", "scroll", "screenshot"]);
        for spec in target_verbs() {
            assert!(validate(&call_naming(spec, "go")).is_ok(), "{}", spec.name);
        }
    }

    #[test]
    fn an_id_past_its_bound_reads_malformed() {
        let long = "x".repeat(crate::schema::MAX_ID_BYTES + 1);
        let click = crate::schema::verb("click").expect("click is a verb");
        let refused = validate(&call_naming(click, &long)).expect_err("refused");
        assert_eq!(refused.cause(), Cause::Malformed);
    }
}
