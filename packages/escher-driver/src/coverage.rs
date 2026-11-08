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
//! not listed. A bubbling dispatch reaches every ancestor listener of its event, a
//! `stopPropagation` unread. The reader holds ids, owner ids and event names only, never a
//! `NodeId`, a control value or typed text.
//!
//! [`DioxusDocument::record_reached`]: dioxus_native_dom::DioxusDocument::record_reached

use crate::error::SessionError;
use crate::recording::Check;
use crate::session::Session;

/// One handler of a [`Coverage`]: the graph node carrying it and its event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandlerRef {
    /// The stable id of the graph node carrying the handler.
    pub id: String,
    /// The stable id of the element whose handler serves the node, where that is another
    /// element: a select for its options and groups.
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

/// Adds every handler the session's screen shows now, `mounted` left out, to `seen`, once each.
fn see(session: &Session, seen: &mut Vec<HandlerRef>) {
    for node in session.harness().doc.graph().nodes {
        for handler in node.handlers {
            if handler.event == "mounted"
                || seen
                    .iter()
                    .any(|known| known.id == node.id && known.event == handler.event)
            {
                continue;
            }
            seen.push(HandlerRef {
                id: node.id.clone(),
                owner: node.owner.clone(),
                event: handler.event,
            });
        }
    }
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
        reached.extend(session.harness().doc.reached_handlers());
    }
    let unexercised = seen
        .into_iter()
        .filter(|handler| {
            !reached
                .iter()
                .any(|hit| hit.id == handler.id && hit.event == handler.event)
        })
        .collect();
    Ok(Coverage {
        unexercised,
        checks: checks.len(),
        held,
    })
}
