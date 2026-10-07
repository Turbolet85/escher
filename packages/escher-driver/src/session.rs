//! The held instance.

use blitz_test_harness::{Harness, Settled};
use dioxus_native_dom::DioxusDocument;

use crate::SessionError;

const MAX_LABEL_BYTES: usize = 32;

/// Whether `label` is 1 to 32 bytes of `a-z`, `0-9` and `-`.
pub(crate) fn valid_label(label: &str) -> bool {
    (1..=MAX_LABEL_BYTES).contains(&label.len())
        && label
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

/// A caller's way to move its app's time: handed the held instance and the milliseconds asked,
/// it returns the milliseconds the app actually moved.
pub(crate) type TimeStep = Box<dyn FnMut(&mut Harness<DioxusDocument>, u32) -> u32>;

/// One headless app instance, held for as long as the session lives.
///
/// The caller boots the instance; the session owns it and hands it out between commands, so
/// what one command leaves is what the next one reads. Dropping the session ends the instance.
/// Anything else the app needs alive beside its instance stays the caller's to hold.
pub struct Session {
    label: String,
    pub(crate) harness: Harness<DioxusDocument>,
    pub(crate) time: Option<TimeStep>,
}

impl Session {
    /// Starts a session over the instance `boot` returns.
    ///
    /// `label` is the caller's short name for what it boots: 1 to 32 bytes of `a-z`, `0-9` and
    /// `-`. It is checked first, and a label outside that set returns
    /// [`SessionError::InvalidLabel`] without `boot` being called. `boot` then runs once, on
    /// the calling thread.
    pub fn start(
        label: &str,
        boot: impl FnOnce() -> Harness<DioxusDocument>,
    ) -> Result<Session, SessionError> {
        if !valid_label(label) {
            return Err(SessionError::InvalidLabel);
        }
        Ok(Session {
            label: label.to_string(),
            harness: boot(),
            time: None,
        })
    }

    /// The session, carrying `step` as its way to move the app's time.
    ///
    /// An app moves time in its own units, so the step is the caller's: it is handed the held
    /// instance and the milliseconds an `advance` asks for, moves the app by as much of that
    /// as its units hold, and returns the milliseconds it actually moved. A session started
    /// without one has no way to move time, and refuses `advance`.
    pub fn with_time(
        mut self,
        step: impl FnMut(&mut Harness<DioxusDocument>, u32) -> u32 + 'static,
    ) -> Session {
        self.time = Some(Box::new(step));
        self
    }

    /// The label the session was started with.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The held instance.
    pub fn harness(&self) -> &Harness<DioxusDocument> {
        &self.harness
    }

    /// The held instance, to drive.
    pub fn harness_mut(&mut self) -> &mut Harness<DioxusDocument> {
        &mut self.harness
    }

    /// Runs `step` on the held instance, then settles it ([`Harness::settle`]): when this
    /// returns `Ok`, everything the step made due is on the screen.
    ///
    /// An instance that does not go quiet returns [`SessionError::NotSettled`] with the class
    /// of work still outstanding. The step is not rolled back: the instance keeps what it did.
    /// Settling moves no time — a timer or an animation advances only when the caller
    /// advances it.
    pub fn act(
        &mut self,
        step: impl FnOnce(&mut Harness<DioxusDocument>),
    ) -> Result<Settled, SessionError> {
        step(&mut self.harness);
        self.harness
            .settle()
            .map_err(|not_settled| SessionError::NotSettled(not_settled.busy))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_label_of_the_closed_set_is_valid() {
        for label in ["counter", "flight-booker", "a", "7", "-", &"x".repeat(32)] {
            assert!(valid_label(label), "{label:?}");
        }
    }

    #[test]
    fn a_label_outside_the_closed_set_is_refused_before_the_boot_runs() {
        let over_long = "x".repeat(33);
        for label in ["", &over_long, "Counter", "flight booker", "a/b", "é"] {
            let started = Session::start(label, || panic!("the boot ran for a refused label"));
            assert!(
                matches!(started, Err(SessionError::InvalidLabel)),
                "{label:?}"
            );
        }
    }
}
