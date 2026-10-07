//! Settling: run every piece of work that is due now, then say whether anything is still
//! outstanding.

use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use blitz_dom::Document;
use blitz_traits::net::{Bytes, NetHandler, NetProvider, Request};

use crate::Harness;

/// The most passes one [`Harness::settle`] call makes before it reports the instance busy.
pub const SETTLE_PASS_LIMIT: u32 = 64;

/// A class of work that can hold an instance unsettled.
///
/// It names a class only: no node, element, name, URL or value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Busy {
    /// The document's own pending work: it was still rendering at the bound.
    Render,
    /// Style and layout: the hovered node was still moving with the layout at the bound.
    Layout,
    /// A resource was requested and is not applied yet.
    Loads,
}

/// What [`Harness::settle`] returns for an instance with no work due.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settled {
    /// How many passes ran. An idle instance takes one.
    pub passes: u32,
    /// Whether the document read as animating
    /// ([`BaseDocument::is_animating`](blitz_dom::BaseDocument::is_animating)) when settle
    /// returned. An animation does not hold settle open.
    pub animating: bool,
}

/// What [`Harness::settle`] returns for an instance that did not go quiet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotSettled {
    /// The class of work that was still outstanding.
    pub busy: Busy,
}

impl fmt::Display for NotSettled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.busy {
            Busy::Render => "the instance did not go quiet: it was still rendering",
            Busy::Layout => "the instance did not go quiet: its layout was still moving",
            Busy::Loads => "the instance did not go quiet: a load was still outstanding",
        })
    }
}

impl Error for NotSettled {}

#[derive(Default)]
struct LoadCounts {
    in_flight: AtomicUsize,
    finished: AtomicU64,
}

/// A [`NetProvider`] that counts the requests crossing it and stores nothing else.
///
/// A request is in flight from `fetch` until its handler is consumed by `bytes` or dropped
/// unanswered. An answered handler has put its message on the document's channel by then, so
/// the next resolve applies it.
pub(crate) struct LoadCounter {
    inner: Arc<dyn NetProvider>,
    counts: Arc<LoadCounts>,
}

impl LoadCounter {
    pub(crate) fn new(inner: Arc<dyn NetProvider>) -> Self {
        Self {
            inner,
            counts: Arc::default(),
        }
    }

    fn in_flight(&self) -> usize {
        self.counts.in_flight.load(Ordering::SeqCst)
    }

    /// How many handlers have been consumed or dropped. It only grows.
    fn finished(&self) -> u64 {
        self.counts.finished.load(Ordering::SeqCst)
    }
}

impl NetProvider for LoadCounter {
    fn fetch(&self, doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        self.counts.in_flight.fetch_add(1, Ordering::SeqCst);
        let handler = CountedHandler {
            inner: Some(handler),
            counts: Arc::clone(&self.counts),
        };
        self.inner.fetch(doc_id, request, Box::new(handler));
    }

    fn is_noop(&self) -> bool {
        self.inner.is_noop()
    }
}

struct CountedHandler {
    inner: Option<Box<dyn NetHandler>>,
    counts: Arc<LoadCounts>,
}

impl NetHandler for CountedHandler {
    fn bytes(mut self: Box<Self>, resolved_url: String, bytes: Bytes) {
        if let Some(inner) = self.inner.take() {
            inner.bytes(resolved_url, bytes);
        }
    }
}

impl Drop for CountedHandler {
    fn drop(&mut self) {
        self.counts.in_flight.fetch_sub(1, Ordering::SeqCst);
        self.counts.finished.fetch_add(1, Ordering::SeqCst);
    }
}

impl<D: Document> Harness<D> {
    /// Run every piece of work that is due now, then say whether anything is still outstanding.
    ///
    /// One pass is what [`pump`](Self::pump) does: a poll, then a resolve at the harness's
    /// clock. Settle repeats the pass while a pass still changed something — the poll did
    /// work, a load was answered, or the hovered node moved with the layout — up to
    /// [`SETTLE_PASS_LIMIT`] passes.
    ///
    /// "Settled" means no work is due:
    ///
    /// - render and layout are driven to quiet; an instance still busy at the bound returns
    ///   [`Busy::Render`] or [`Busy::Layout`];
    /// - a load in flight is reported as [`Busy::Loads`] at once and never waited on — settle
    ///   reads no clock, so it cannot wait for the outside world. Loads are counted on the
    ///   net provider the harness was constructed with; a document passed to
    ///   [`wrap`](Self::wrap) is read through its render-blocking resources alone;
    /// - a source only time moves — a timer not yet due, a running animation — does not hold
    ///   settle open and is not advanced by it. It reads animation time only through the
    ///   harness's clock, which moves when the caller calls [`tick`](Self::tick), and reports
    ///   a running animation as [`Settled::animating`].
    ///
    /// Settle neither reads nor drains the document's changed set.
    pub fn settle(&mut self) -> Result<Settled, NotSettled> {
        let time = self.time();
        let mut busy = Busy::Layout;
        for passes in 1..=SETTLE_PASS_LIMIT {
            let hover = self.doc.inner().get_hover_node_id();
            let finished = self.loads_finished();

            let rendered = self.doc.poll(None);
            self.doc.inner_mut().resolve(time);

            let loaded = self.loads_finished() != finished;
            let hover_moved = self.doc.inner().get_hover_node_id() != hover;
            if !(rendered || loaded || hover_moved) {
                let in_flight = self
                    .loads
                    .as_ref()
                    .is_some_and(|loads| loads.in_flight() > 0);
                let doc = self.doc.inner();
                if in_flight || doc.has_pending_critical_resources() {
                    return Err(NotSettled { busy: Busy::Loads });
                }
                return Ok(Settled {
                    passes,
                    animating: doc.is_animating(),
                });
            }
            busy = if rendered {
                Busy::Render
            } else if loaded {
                Busy::Loads
            } else {
                Busy::Layout
            };
        }
        Err(NotSettled { busy })
    }

    fn loads_finished(&self) -> u64 {
        self.loads.as_ref().map_or(0, |loads| loads.finished())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Mutex;
    use std::task::Context;

    use blitz_dom::{BaseDocument, DocGuard, DocGuardMut, DocumentConfig};
    use blitz_html::HtmlDocument;
    use blitz_traits::net::Url;

    use super::*;

    const URL: &str = "http://settle.test/sheet.css";

    /// A provider that keeps every handler it is given.
    #[derive(Default)]
    struct Held {
        handlers: Mutex<Vec<Box<dyn NetHandler>>>,
        noop: bool,
    }

    impl Held {
        fn take(&self) -> Box<dyn NetHandler> {
            self.handlers
                .lock()
                .unwrap()
                .pop()
                .expect("a handler is held")
        }
    }

    impl NetProvider for Held {
        fn fetch(&self, _doc_id: usize, _request: Request, handler: Box<dyn NetHandler>) {
            self.handlers.lock().unwrap().push(handler);
        }

        fn is_noop(&self) -> bool {
            self.noop
        }
    }

    /// A handler that keeps what it is answered with.
    #[derive(Default, Clone)]
    struct Answers(Arc<Mutex<Vec<(String, Bytes)>>>);

    impl NetHandler for Answers {
        fn bytes(self: Box<Self>, resolved_url: String, bytes: Bytes) {
            self.0.lock().unwrap().push((resolved_url, bytes));
        }
    }

    fn counted() -> (Arc<Held>, Arc<LoadCounter>) {
        let held = Arc::new(Held::default());
        let counter = Arc::new(LoadCounter::new(Arc::clone(&held) as _));
        (held, counter)
    }

    fn fetch(counter: &LoadCounter, answers: &Answers) {
        let request = Request::get(Url::parse(URL).unwrap());
        counter.fetch(0, request, Box::new(answers.clone()));
    }

    #[test]
    fn an_answered_request_reaches_the_inner_handler_and_leaves_none_in_flight() {
        let (held, counter) = counted();
        let answers = Answers::default();
        assert_eq!(counter.in_flight(), 0);

        fetch(&counter, &answers);
        assert_eq!(counter.in_flight(), 1);
        assert!(answers.0.lock().unwrap().is_empty());

        held.take()
            .bytes(URL.to_string(), Bytes::from_static(b"a { }"));
        assert_eq!(
            *answers.0.lock().unwrap(),
            [(URL.to_string(), Bytes::from_static(b"a { }"))]
        );
        assert_eq!(counter.in_flight(), 0);
    }

    #[test]
    fn a_handler_dropped_unanswered_leaves_none_in_flight() {
        let (held, counter) = counted();
        let answers = Answers::default();
        fetch(&counter, &answers);
        assert_eq!(counter.in_flight(), 1);

        drop(held.take());
        assert_eq!(counter.in_flight(), 0);
        assert!(answers.0.lock().unwrap().is_empty());
    }

    #[test]
    fn the_finished_count_grows_by_one_per_handler_and_never_falls() {
        let (held, counter) = counted();
        let answers = Answers::default();
        assert_eq!(counter.finished(), 0);

        fetch(&counter, &answers);
        fetch(&counter, &answers);
        assert_eq!((counter.in_flight(), counter.finished()), (2, 0));

        held.take().bytes(URL.to_string(), Bytes::new());
        assert_eq!((counter.in_flight(), counter.finished()), (1, 1));

        drop(held.take());
        assert_eq!((counter.in_flight(), counter.finished()), (0, 2));

        fetch(&counter, &answers);
        assert_eq!((counter.in_flight(), counter.finished()), (1, 2));
    }

    #[test]
    fn is_noop_is_the_inner_providers() {
        for noop in [false, true] {
            let held = Held {
                noop,
                ..Held::default()
            };
            assert_eq!(LoadCounter::new(Arc::new(held)).is_noop(), noop);
        }
    }

    /// What one scripted poll does.
    #[derive(Clone, Copy)]
    enum Poll {
        /// Answers `true`.
        Work,
        /// Answers `false`.
        Idle,
        /// Answers a held handler, then answers `false`.
        Answer,
    }

    /// An empty document whose `poll` answers from a script, then from `rest`, and counts its
    /// calls.
    struct Scripted {
        base: BaseDocument,
        script: VecDeque<Poll>,
        rest: Poll,
        polls: u32,
        held: Arc<Held>,
    }

    impl Document for Scripted {
        fn inner(&self) -> DocGuard<'_> {
            DocGuard::Ref(&self.base)
        }

        fn inner_mut(&mut self) -> DocGuardMut<'_> {
            DocGuardMut::Ref(&mut self.base)
        }

        fn poll(&mut self, _task_context: Option<Context>) -> bool {
            self.polls += 1;
            match self.script.pop_front().unwrap_or(self.rest) {
                Poll::Work => true,
                Poll::Idle => false,
                Poll::Answer => {
                    self.held.take().bytes(URL.to_string(), Bytes::new());
                    false
                }
            }
        }
    }

    /// A harness over a scripted document, with a counter attached, and the counter.
    fn scripted(script: &[Poll], rest: Poll) -> (Harness<Scripted>, Arc<LoadCounter>) {
        let (held, counter) = counted();
        let doc = Scripted {
            base: BaseDocument::new(DocumentConfig::default()),
            script: script.iter().copied().collect(),
            rest,
            polls: 0,
            held,
        };
        let mut harness = Harness::wrap(doc);
        harness.loads = Some(Arc::clone(&counter));
        (harness, counter)
    }

    #[test]
    fn an_idle_document_settles_in_one_pass() {
        let (mut harness, _counter) = scripted(&[], Poll::Idle);
        assert_eq!(
            harness.settle(),
            Ok(Settled {
                passes: 1,
                animating: false
            })
        );
        assert_eq!(harness.doc.polls, 1);
    }

    #[test]
    fn work_for_k_polls_settles_in_k_plus_one_passes() {
        for k in [1, 2, 7, SETTLE_PASS_LIMIT - 1] {
            let script = vec![Poll::Work; k as usize];
            let (mut harness, _counter) = scripted(&script, Poll::Idle);
            assert_eq!(
                harness.settle().map(|settled| settled.passes),
                Ok(k + 1),
                "{k}"
            );
            assert_eq!(harness.doc.polls, k + 1, "{k}");
        }
    }

    #[test]
    fn work_for_ever_ends_at_the_bound_as_render() {
        let (mut harness, _counter) = scripted(&[], Poll::Work);
        assert_eq!(harness.settle(), Err(NotSettled { busy: Busy::Render }));
        assert_eq!(harness.doc.polls, SETTLE_PASS_LIMIT);
    }

    #[test]
    fn a_request_in_flight_after_a_quiet_pass_reads_loads_after_one_poll() {
        let (mut harness, counter) = scripted(&[], Poll::Idle);
        fetch(&counter, &Answers::default());
        assert_eq!(harness.settle(), Err(NotSettled { busy: Busy::Loads }));
        assert_eq!(harness.doc.polls, 1);
    }

    #[test]
    fn a_handler_that_finishes_inside_a_pass_costs_one_more_pass() {
        let (mut harness, counter) = scripted(&[Poll::Answer], Poll::Idle);
        fetch(&counter, &Answers::default());
        assert_eq!(
            harness.settle().map(|settled| settled.passes),
            Ok(2),
            "the pass that answered the handler is not the last"
        );
        assert_eq!(harness.doc.polls, 2);
        assert_eq!(counter.in_flight(), 0);
    }

    #[test]
    fn a_wrapped_document_is_read_through_its_render_blocking_resources() {
        let held = Arc::new(Held::default());
        let doc = HtmlDocument::from_html(
            r#"<html><head><link rel="stylesheet" href="sheet.css"></head><body></body></html>"#,
            DocumentConfig {
                base_url: Some("http://settle.test/".to_string()),
                net_provider: Some(Arc::clone(&held) as _),
                ..Default::default()
            },
        );
        let mut harness = Harness::wrap(doc);
        assert!(harness.loads.is_none());
        assert!(harness.base().has_pending_critical_resources());
        assert_eq!(harness.settle(), Err(NotSettled { busy: Busy::Loads }));

        held.take().bytes(URL.to_string(), Bytes::new());
        assert_eq!(harness.settle().map(|settled| settled.passes), Ok(1));
    }

    #[test]
    fn every_message_is_non_empty_distinct_and_holds_no_path() {
        let messages =
            [Busy::Render, Busy::Layout, Busy::Loads].map(|busy| NotSettled { busy }.to_string());
        for message in &messages {
            assert!(!message.is_empty());
            assert!(!message.contains('/'), "{message}");
        }
        assert_ne!(messages[0], messages[1]);
        assert_ne!(messages[0], messages[2]);
        assert_ne!(messages[1], messages[2]);
    }
}
