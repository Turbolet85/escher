//! The clock a document reads its time from.
//!
//! The engine's own time reads — the overlay scrollbar fade, double-click
//! detection, smooth scrolling and fling — take the document's [`Clock`]. By
//! default that is a [`WallClock`]; an embedder that drives time itself (a
//! headless test harness) supplies a [`VirtualClock`] through
//! [`DocumentConfig::clock`](crate::DocumentConfig::clock).

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use web_time::{Duration, Instant};

/// A monotonic source of time for a document.
pub trait Clock: Send + Sync {
    /// The time elapsed since this clock's own origin. Never less than an
    /// earlier reading.
    fn now(&self) -> Duration;
}

/// Wall time, measured from when the clock was made.
#[derive(Debug, Clone, Copy)]
pub struct WallClock {
    origin: Instant,
}

impl WallClock {
    /// A wall clock whose origin is now.
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }
}

impl Default for WallClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for WallClock {
    fn now(&self) -> Duration {
        self.origin.elapsed()
    }
}

/// A clock that moves only when [`advance`](VirtualClock::advance) is called.
///
/// Clones are handles onto one shared reading: advancing any of them moves
/// every one.
#[derive(Debug, Clone, Default)]
pub struct VirtualClock {
    nanos: Arc<AtomicU64>,
}

impl VirtualClock {
    /// A virtual clock reading zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Move the clock forward by `by`, saturating at the largest reading it holds.
    pub fn advance(&self, by: Duration) {
        let by = u64::try_from(by.as_nanos()).unwrap_or(u64::MAX);
        let mut now = self.nanos.load(Ordering::SeqCst);
        while let Err(read) = self.nanos.compare_exchange_weak(
            now,
            now.saturating_add(by),
            Ordering::SeqCst,
            Ordering::SeqCst,
        ) {
            now = read;
        }
    }
}

impl Clock for VirtualClock {
    fn now(&self) -> Duration {
        Duration::from_nanos(self.nanos.load(Ordering::SeqCst))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_virtual_clock_reads_the_sum_of_its_advances_on_every_clone() {
        let clock = VirtualClock::new();
        let other = clock.clone();
        assert_eq!(clock.now(), Duration::ZERO);
        clock.advance(Duration::from_millis(250));
        other.advance(Duration::from_millis(99));
        assert_eq!(clock.now(), Duration::from_millis(349));
        assert_eq!(other.now(), Duration::from_millis(349));
    }

    #[test]
    fn a_virtual_clock_saturates_rather_than_wrapping() {
        let clock = VirtualClock::new();
        clock.advance(Duration::MAX);
        clock.advance(Duration::from_millis(1));
        assert_eq!(clock.now(), Duration::from_nanos(u64::MAX));
    }

    #[test]
    fn a_wall_clock_never_reads_less_on_a_later_read() {
        let clock = WallClock::new();
        let first = clock.now();
        let second = clock.now();
        assert!(second >= first);
    }
}
