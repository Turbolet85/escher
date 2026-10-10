//! The held instance.

use std::collections::HashMap;

use blitz_test_harness::{Harness, Settled};
use dioxus_native_dom::DioxusDocument;

use crate::SessionError;
use crate::schema::MAX_ID_BYTES;

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

/// The most ids a session remembers.
pub(crate) const MAX_SEEN_IDS: usize = 4096;

/// The ids the session's screens have read, as text: what tells an id that named an element
/// earlier from one that never did.
///
/// It holds at most its bound. Reading an id it already holds makes that id the most recently
/// read; past the bound the id read longest ago is forgotten, and reads as one never read. An
/// id longer than [`MAX_ID_BYTES`] is never held: no call can name it, so nothing asks whether
/// it was read.
pub(crate) struct SeenIds {
    bound: usize,
    reads: u64,
    /// Each id with the count of reads at which it was last read.
    last_read: HashMap<String, u64>,
}

impl SeenIds {
    pub(crate) fn with_bound(bound: usize) -> SeenIds {
        SeenIds {
            bound,
            reads: 0,
            last_read: HashMap::new(),
        }
    }

    pub(crate) fn record(&mut self, id: &str) {
        if id.len() > MAX_ID_BYTES {
            return;
        }
        self.reads += 1;
        if let Some(read) = self.last_read.get_mut(id) {
            *read = self.reads;
            return;
        }
        self.last_read.insert(id.to_string(), self.reads);
        if self.last_read.len() > self.bound {
            let oldest = self
                .last_read
                .iter()
                .min_by_key(|(_, read)| **read)
                .map(|(id, _)| id.clone());
            if let Some(oldest) = oldest {
                self.last_read.remove(&oldest);
            }
        }
    }

    pub(crate) fn holds(&self, id: &str) -> bool {
        self.last_read.contains_key(id)
    }
}

/// One headless app instance, held for as long as the session lives.
///
/// The caller boots the instance; the session owns it and hands it out between commands, so
/// what one command leaves is what the next one reads. Dropping the session ends the instance.
/// Anything else the app needs alive beside its instance stays the caller's to hold.
///
/// The session also remembers the ids of the screens it reads to answer a call, as text and up
/// to a bound, for as long as it lives: that is how [`Session::run`] tells an id that named an
/// element earlier from one that never did. Nothing reads that record out of the session.
pub struct Session {
    label: String,
    pub(crate) harness: Harness<DioxusDocument>,
    pub(crate) time: Option<TimeStep>,
    pub(crate) seen: SeenIds,
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
            seen: SeenIds::with_bound(MAX_SEEN_IDS),
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

    #[test]
    fn the_record_holds_the_ids_read_and_forgets_the_one_read_longest_ago() {
        // The ids read, in order, into a record bounded at three, and what it holds after.
        let rows: [(&[&str], &[&str], &[&str]); 6] = [
            (&[], &[], &["a"]),
            (&["a"], &["a"], &["b"]),
            (&["a", "b", "c"], &["a", "b", "c"], &["d"]),
            (&["a", "b", "c", "d"], &["b", "c", "d"], &["a"]),
            (&["a", "b", "c", "a", "d"], &["a", "c", "d"], &["b"]),
            (&["a", "a", "a", "b", "c", "d"], &["b", "c", "d"], &["a"]),
        ];
        assert_eq!(rows.len(), 6);
        for (row, (read, held, forgotten)) in rows.into_iter().enumerate() {
            let mut seen = SeenIds::with_bound(3);
            for id in read {
                seen.record(id);
            }
            assert!(held.iter().all(|id| seen.holds(id)), "row {row}");
            assert!(!forgotten.iter().any(|id| seen.holds(id)), "row {row}");
            assert!(seen.last_read.len() == held.len(), "row {row}");
        }
        assert_eq!(MAX_SEEN_IDS, 4096);
    }

    #[test]
    fn the_record_holds_no_id_longer_than_a_call_can_name() {
        let at_the_bound = "i".repeat(MAX_ID_BYTES);
        let over_long = "i".repeat(MAX_ID_BYTES + 1);
        let mut seen = SeenIds::with_bound(MAX_SEEN_IDS);
        seen.record(&over_long);
        assert!(!seen.holds(&over_long) && seen.last_read.is_empty());
        seen.record(&at_the_bound);
        seen.record(&over_long);
        assert!(seen.holds(&at_the_bound) && !seen.holds(&over_long));
        assert_eq!(seen.last_read.len(), 1);
        assert_eq!(MAX_ID_BYTES, 1024);
    }
}
