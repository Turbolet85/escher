//! Handler coverage: the handlers a set of kept checks never exercised, read from the UI graph.
//!
//! Each check is replayed on a fresh session of its app, through [`Check::replay`]'s own path.
//! The UI graph is read at boot and after every replayed step, and every handler it shows on a
//! node is seen, `mounted` left out, as the one actionable rule leaves it out. Meanwhile the
//! document's reached-handler record ([`DioxusDocument::record_reached`]) collects the handlers
//! each dispatch reached. A handler is unexercised where it was seen and no replay reached it.
//!
//! The count reads the graph's DOM listeners on agent-visible nodes, so the handlers a canvas
//! dispatches inside one listener are not counted, and a handler on no screen a replay read is
//! not listed. A node a canvas declares carries no handler in the graph, yet where its canvas
//! carries a `click` listener the node's own `click` is listed, under its id and owned by the
//! canvas, and reads exercised where a replayed step is an accepted `click` naming that id; the
//! dispatch reaches the canvas's listener, counted under the canvas's id. A click on the canvas's
//! own id that lands inside a node's region counts the canvas's listener alone. A bubbling
//! dispatch nothing stopped reaches every ancestor listener of its event;
//! one something stopped reaches its innermost listener alone, so where an ancestor stops it, the
//! listeners between that ancestor and the target read unexercised although they ran; a
//! non-bubbling event reaches its target alone. The reader holds ids, owner ids and event names only, never a
//! `NodeId`, a control value or typed text.
//!
//! [`DioxusDocument::record_reached`]: dioxus_native_dom::DioxusDocument::record_reached

use dioxus_native_dom::VIRTUAL_KEY_SEPARATOR;

use crate::command::ArgValue;
use crate::error::SessionError;
use crate::recording::Check;
use crate::replay::Replay;
use crate::session::Session;

/// One handler of a [`Coverage`]: the graph node carrying it and its event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandlerRef {
    /// The stable id of the graph node carrying the handler.
    pub id: String,
    /// The stable id of the element whose handler serves the node, where that is another
    /// element: a select for its options and groups, a canvas for the nodes it declares.
    pub owner: Option<String>,
    /// The handler's event name, without its `on`.
    pub event: String,
}

/// What [`handler_coverage`] read over a set of kept checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coverage {
    /// The handlers a replay's screen showed and no replay reached, in the order first seen.
    pub unexercised: Vec<HandlerRef>,
    /// How many checks were read.
    pub checks: usize,
    /// How many of their replays held.
    pub held: usize,
}

/// Adds every handler the session's screen shows now, `mounted` left out, to `seen`, once each,
/// and the `click` of each virtual node whose canvas carries a `click` listener.
fn see(session: &Session, seen: &mut Vec<HandlerRef>) {
    let nodes = session.harness().doc.graph().nodes;
    let mut add = |id: &str, owner: Option<&String>, event: &str| {
        if event != "mounted"
            && !seen
                .iter()
                .any(|known| known.id == id && known.event == event)
        {
            seen.push(HandlerRef {
                id: id.to_string(),
                owner: owner.cloned(),
                event: event.to_string(),
            });
        }
    };
    for node in &nodes {
        for handler in &node.handlers {
            add(&node.id, node.owner.as_ref(), &handler.event);
        }
        if !node.id.contains(VIRTUAL_KEY_SEPARATOR) {
            continue;
        }
        let Some(owner) = node.owner.as_ref() else {
            continue;
        };
        let canvas_clicks = nodes.iter().any(|canvas| {
            &canvas.id == owner && canvas.handlers.iter().any(|h| h.event == "click")
        });
        if canvas_clicks {
            add(&node.id, Some(owner), "click");
        }
    }
}

/// The ids of virtual nodes an accepted replayed `click` named: of the check's steps the first
/// [`Replay::ran`], each accepted on replay.
fn clicked_virtual(check: &Check, replay: &Replay) -> Vec<String> {
    let changed = replay.change().map(|change| change.step());
    check
        .steps()
        .iter()
        .take(replay.ran())
        .enumerate()
        .filter(|(index, step)| match changed {
            Some(at) if at == *index => replay.change().is_some_and(|c| c.accepted_now()),
            _ => step.accepted(),
        })
        .filter(|(_, step)| step.call().verb == "click")
        .filter_map(|(_, step)| {
            step.call()
                .args
                .iter()
                .find_map(|(name, value)| match value {
                    ArgValue::Text(id) if name == "id" && id.contains(VIRTUAL_KEY_SEPARATOR) => {
                        Some(id.clone())
                    }
                    _ => None,
                })
        })
        .collect()
}

/// Replays each of `checks` on a fresh session `boot` starts and lists the handlers none of the
/// replays exercised.
///
/// A check whose app is not the booted session's label runs nothing, contributes nothing and
/// does not hold. A boot that fails ends the reading with its error.
pub fn handler_coverage(
    checks: &[Check],
    mut boot: impl FnMut() -> Result<Session, SessionError>,
) -> Result<Coverage, SessionError> {
    let mut seen = Vec::new();
    let mut reached = Vec::new();
    let mut held = 0;
    for check in checks {
        let mut session = boot()?;
        if session.label() != check.app() {
            continue;
        }
        session.harness_mut().doc.record_reached();
        see(&session, &mut seen);
        let replay = check.replay_each(&mut session, |session| see(session, &mut seen));
        if replay.holds() {
            held += 1;
        }
        reached.extend(
            session
                .harness()
                .doc
                .reached_handlers()
                .into_iter()
                .map(|hit| (hit.id, hit.event)),
        );
        reached.extend(
            clicked_virtual(check, &replay)
                .into_iter()
                .map(|id| (id, "click".to_string())),
        );
    }
    let unexercised = seen
        .into_iter()
        .filter(|handler| {
            !reached
                .iter()
                .any(|(id, event)| *id == handler.id && *event == handler.event)
        })
        .collect();
    Ok(Coverage {
        unexercised,
        checks: checks.len(),
        held,
    })
}
