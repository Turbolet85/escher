//! Exploration: every action the held app offers, driven breadth-first within a step budget.
//!
//! [`explore`] boots the app once to read its first screen, then takes paths of calls off a
//! frontier ordered by length. Each path runs on a fresh session the caller's boot starts: its
//! earlier calls re-reach the screen it was found on, each through [`Session::run`], and its last
//! call is the step it explores, run inside a panic catch. A path that ends accepted on a screen
//! no earlier path reached adds one child per actionable node and explored verb of that screen.
//!
//! The verbs explored are the instance verbs the `actions` reader judges a node by, less any verb
//! whose result holds an image, read from the verb table; the call driven for a node is the call
//! `actions` judged. Nothing here names a verb.
//!
//! Exploration is in process only: no verb, role, argument or carrier reaches it. It follows no
//! navigation of its own — a session navigates only as its boot configured it — and reads the
//! seed its boot gives. It leaves no span of its own (each driven call leaves its `command`
//! span), no log line, no print, and reads no environment. A panic's payload is never read; the
//! process's panic hook still runs for a caught panic.

use std::collections::{HashSet, VecDeque};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;

use crate::actions::{call_naming, target_verbs};
use crate::command::Call;
use crate::error::SessionError;
use crate::execute::Outcome;
use crate::recording::{fnv1a, prepare_dir};
use crate::refusal::{Cause, Refusal};
use crate::schema::{FieldKind, VerbSpec};
use crate::session::Session;

/// The most acting calls one exploration may run.
pub const MAX_EXPLORE_STEPS: u32 = 10_000;

/// How many acting calls an exploration may run, every call that re-reaches a path counted: a
/// whole number from 1 to [`MAX_EXPLORE_STEPS`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepBudget(u32);

impl StepBudget {
    /// The budget of `steps` calls, or `None` for 0 or a number past [`MAX_EXPLORE_STEPS`].
    pub fn new(steps: u32) -> Option<StepBudget> {
        (1..=MAX_EXPLORE_STEPS)
            .contains(&steps)
            .then_some(StepBudget(steps))
    }

    /// The number of calls the budget allows.
    pub fn get(self) -> u32 {
        self.0
    }
}

/// What an [`explore`] read: ids, the table's verb words, closed endings, indices and counts,
/// never a screen's text, a control value, a panic's payload or a `NodeId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exploration {
    /// Every path run, in the order it was run.
    pub paths: Vec<ExploredPath>,
    /// The acting calls run, every call that re-reached a path counted.
    pub steps: u32,
    /// Whether every path found was run: false when the budget stopped the run first.
    pub exhausted: bool,
}

/// One explored path: its calls from boot and how its run ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExploredPath {
    /// The calls from boot, the explored step last.
    pub calls: Vec<Call>,
    /// How the run ended.
    pub ending: PathEnding,
}

/// How a path's run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathEnding {
    /// The last call was accepted; `settled` is whether the app went quiet after it.
    Accepted {
        /// Whether the app went quiet after the step.
        settled: bool,
    },
    /// The last call was refused with this cause. A refused path is not explored further.
    Refused(Cause),
    /// The last call panicked. Its payload is not read, and the path is not explored further.
    Panicked,
    /// The call at this index, before the last, was not accepted again or panicked on the fresh
    /// session, though it was when the path was found.
    Diverged {
        /// The index of the call that diverged.
        step: usize,
    },
}

/// Drives every action the app `boot` starts offers, breadth-first, until no path is left or the
/// next would pass `budget`, and reports each path's calls and ending.
///
/// `boot` starts a fresh session each time it is called: once for the first screen, which is
/// read and not recorded, then once per path. Each call of a path runs through
/// [`Session::run`]; a refused call and a panicking call end their path and the exploration goes
/// on. The first screen and every screen an accepted path reaches are told apart by their
/// snapshot's text, and a screen reached before is not explored again.
///
/// Given `record`, a directory, every path's session records into it
/// ([`Session::recording_into`]): one owner-only kept check per path, `check-{n}.json` with `n`
/// the path's index from 1 in a directory that held no check, holding the path's calls as they
/// were answered — a panicked call is never answered, so its check holds the calls before it. A
/// directory open to its group or to others returns [`SessionError::RecordDirNotPrivate`] before
/// anything boots. A boot that fails ends the exploration with its error.
///
/// A session whose step panicked is dropped under the same catch, so a drop that panics too
/// leaks what was left of it rather than ending the exploration.
pub fn explore(
    mut boot: impl FnMut() -> Result<Session, SessionError>,
    budget: StepBudget,
    record: Option<&Path>,
) -> Result<Exploration, SessionError> {
    if let Some(dir) = record {
        prepare_dir(dir)?;
    }
    let verbs = explored_verbs();
    let mut reached = HashSet::new();
    let mut frontier = VecDeque::new();
    let mut first = boot()?;
    reached.insert(screen_digest(&first));
    frontier.extend(children(&mut first, &[], &verbs));
    drop(first);
    let mut exploration = Exploration {
        paths: Vec::new(),
        steps: 0,
        exhausted: false,
    };
    while let Some(calls) = frontier.pop_front() {
        let cost = u32::try_from(calls.len()).unwrap_or(u32::MAX);
        if exploration.steps.saturating_add(cost) > budget.get() {
            frontier.push_front(calls);
            break;
        }
        let mut session = start(&mut boot, record)?;
        let (ending, ran) = walk(&mut session, &calls);
        exploration.steps = exploration.steps.saturating_add(ran);
        if matches!(ending, PathEnding::Accepted { .. }) && reached.insert(screen_digest(&session))
        {
            frontier.extend(children(&mut session, &calls, &verbs));
        }
        let _ = catch_unwind(AssertUnwindSafe(move || drop(session)));
        exploration.paths.push(ExploredPath { calls, ending });
    }
    exploration.exhausted = frontier.is_empty();
    Ok(exploration)
}

/// Whether exploration drives `verb`: none of its result fields holds an image.
fn explorable(verb: &VerbSpec) -> bool {
    !verb.fields.iter().any(|field| field.kind == FieldKind::Png)
}

/// The verbs exploration drives, in the table's order.
fn explored_verbs() -> Vec<&'static VerbSpec> {
    target_verbs().filter(|verb| explorable(verb)).collect()
}

/// A fresh session from `boot`, recording into `record` where one is named.
fn start(
    boot: &mut impl FnMut() -> Result<Session, SessionError>,
    record: Option<&Path>,
) -> Result<Session, SessionError> {
    let session = boot()?;
    match record {
        Some(dir) => session.recording_into(dir),
        None => Ok(session),
    }
}

/// The digest of the screen the session holds now.
fn screen_digest(session: &Session) -> u64 {
    fnv1a(session.harness().doc.snapshot().to_text().as_bytes())
}

/// `calls` extended by one call per actionable node of the screen, in snapshot pre-order, and
/// per verb of `verbs`, in the table's order.
fn children(session: &mut Session, calls: &[Call], verbs: &[&'static VerbSpec]) -> Vec<Vec<Call>> {
    session
        .actions()
        .into_iter()
        .flat_map(|node| {
            verbs.iter().map(move |verb| {
                let mut path = calls.to_vec();
                path.push(call_naming(verb, &node.id));
                path
            })
        })
        .collect()
}

/// Runs `call`, catching a panic: `None` when it panicked.
fn run_caught(session: &mut Session, call: &Call) -> Option<Result<Outcome, Refusal>> {
    catch_unwind(AssertUnwindSafe(|| session.run(call))).ok()
}

/// Runs a path's calls on its fresh session: how it ended and how many calls ran.
fn walk(session: &mut Session, calls: &[Call]) -> (PathEnding, u32) {
    let Some((last, before)) = calls.split_last() else {
        return (PathEnding::Accepted { settled: true }, 0);
    };
    for (step, call) in before.iter().enumerate() {
        if !matches!(run_caught(session, call), Some(Ok(_))) {
            return (PathEnding::Diverged { step }, count(step + 1));
        }
    }
    let ending = match run_caught(session, last) {
        None => PathEnding::Panicked,
        Some(Err(refusal)) => PathEnding::Refused(refusal.cause()),
        Some(Ok(outcome)) => PathEnding::Accepted {
            settled: settled(&outcome),
        },
    };
    (ending, count(calls.len()))
}

fn count(calls: usize) -> u32 {
    u32::try_from(calls).unwrap_or(u32::MAX)
}

/// Whether an accepted call's app went quiet; a call that runs no step reads quiet.
fn settled(outcome: &Outcome) -> bool {
    match outcome {
        Outcome::Acted { settled, .. }
        | Outcome::Advanced { settled, .. }
        | Outcome::Scrolled { settled, .. } => *settled,
        _ => true,
    }
}

#[cfg(test)]
#[path = "explore_tests.rs"]
mod tests;
