//! A named fixture state of an app, and a held session's reach of it from boot.
//!
//! A fixture state is static data its app owns: a name and the driver calls that reach it from
//! the app's boot. [`Session::reach`] runs each call through [`Session::run`], the one validated
//! run path, so a session started at boot and brought to a state reads as one driven there by
//! hand, and a recording attached before the reach keeps its steps as ordinary steps.

use std::fmt;

use blitz_test_harness::Busy;

use crate::command::{ArgValue, Call};
use crate::execute::{Outcome, busy_word};
use crate::refusal::Cause;
use crate::session::Session;

/// One driver call a fixture state is reached by: one of the four acting verbs, over static
/// data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureStep {
    /// `click` on the element the id names.
    Click(&'static str),
    /// `type` the text into the element the id names.
    Type(&'static str, &'static str),
    /// `press` the key the name names, on what holds focus.
    Press(&'static str),
    /// `advance` the session's clock by the milliseconds.
    Advance(u32),
}

impl FixtureStep {
    /// The call the verb table spells for this step: the verb's name and its arguments by their
    /// schema names, in schema order.
    pub(crate) fn call(self) -> Call {
        let text = |value: &str| ArgValue::Text(value.to_string());
        let (verb, args) = match self {
            FixtureStep::Click(id) => ("click", vec![("id", text(id))]),
            FixtureStep::Type(id, typed) => ("type", vec![("id", text(id)), ("text", text(typed))]),
            FixtureStep::Press(key) => ("press", vec![("key", text(key))]),
            FixtureStep::Advance(ms) => ("advance", vec![("ms", ArgValue::Number(i64::from(ms)))]),
        };
        Call {
            verb: verb.to_string(),
            args: args
                .into_iter()
                .map(|(name, value)| (name.to_string(), value))
                .collect(),
        }
    }
}

/// A named fixture state: its name and the steps that reach it from the app's boot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fixture {
    name: &'static str,
    steps: &'static [FixtureStep],
}

impl Fixture {
    /// The fixture state `name`, reached from boot by `steps`.
    pub const fn new(name: &'static str, steps: &'static [FixtureStep]) -> Fixture {
        Fixture { name, steps }
    }

    /// The fixture's name.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// The steps that reach the fixture from boot.
    pub const fn steps(&self) -> &'static [FixtureStep] {
        self.steps
    }

    /// The fixture of `fixtures` named `name`, compared whole and byte for byte; any other name
    /// is refused [`ReachError::UnknownState`].
    pub fn named(fixtures: &'static [Fixture], name: &str) -> Result<&'static Fixture, ReachError> {
        fixtures
            .iter()
            .find(|fixture| fixture.name == name)
            .ok_or(ReachError::UnknownState)
    }
}

/// Why a fixture state was not reached. It holds nothing a caller supplied: a step's index and
/// closed words of the schema only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReachError {
    /// No fixture of the app's table carries the name.
    UnknownState,
    /// A step was refused: its index, counted from 0, and its refusal's cause.
    Refused {
        /// The refused step's index.
        step: usize,
        /// Why it was refused.
        cause: Cause,
    },
    /// A step ran and its instance did not go quiet: its index, counted from 0, and the class of
    /// work still outstanding.
    NotSettled {
        /// The unsettled step's index.
        step: usize,
        /// The class of work still outstanding.
        busy: Busy,
    },
}

impl fmt::Display for ReachError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReachError::UnknownState => {
                f.write_str("unknown-state: no fixture state of the app carries the name")
            }
            ReachError::Refused { step, cause } => {
                write!(
                    f,
                    "refused: step {step} of the fixture state was refused {cause}"
                )
            }
            ReachError::NotSettled { step, busy } => write!(
                f,
                "not-settled: step {step} of the fixture state left {} outstanding",
                busy_word(*busy)
            ),
        }
    }
}

impl std::error::Error for ReachError {}

impl Session {
    /// Brings the held instance to `fixture`'s state: runs each of its steps' calls through
    /// [`Session::run`], in order, and stops at the first one refused or left unsettled. The
    /// steps before it stand: an acting call is not rolled back.
    pub fn reach(&mut self, fixture: &Fixture) -> Result<(), ReachError> {
        for (step, fixture_step) in fixture.steps.iter().enumerate() {
            let outcome =
                self.run(&fixture_step.call())
                    .map_err(|refusal| ReachError::Refused {
                        step,
                        cause: refusal.cause(),
                    })?;
            if let Some(busy) = unsettled(&outcome) {
                return Err(ReachError::NotSettled { step, busy });
            }
        }
        Ok(())
    }
}

/// The class of work an acting step's outcome reads outstanding; `None` once it went quiet.
fn unsettled(outcome: &Outcome) -> Option<Busy> {
    match outcome {
        Outcome::Acted {
            settled: false,
            busy,
            ..
        }
        | Outcome::Advanced {
            settled: false,
            busy,
            ..
        }
        | Outcome::Scrolled {
            settled: false,
            busy,
            ..
        } => *busy,
        _ => None,
    }
}

#[cfg(test)]
#[path = "fixture_tests.rs"]
mod tests;
