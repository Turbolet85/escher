use blitz_traits::navigation::NavigationOptions;
use blitz_traits::net::NetWaker;
use futures_util::task::ArcWake;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::{any::Any, sync::Arc};
use winit::{event_loop::EventLoopProxy, window::WindowId};

#[cfg(feature = "accessibility")]
use accesskit_xplat::WindowEvent as AccessKitEvent;

#[derive(Debug, Clone)]
pub enum BlitzShellEvent {
    Poll {
        window_id: WindowId,
    },

    /// The renderer for this window has finished its async initialization. The
    /// embedder should call `View::complete_resume` to transition the view into
    /// an active state.
    ResumeReady {
        window_id: WindowId,
    },

    RequestRedraw {
        doc_id: usize,
    },

    /// Close a window programmatically (e.g. a custom titlebar close button).
    /// Handled identically to `WindowEvent::CloseRequested`.
    CloseWindow {
        window_id: WindowId,
    },

    /// An accessibility event from `accesskit`.
    #[cfg(feature = "accessibility")]
    Accessibility {
        window_id: WindowId,
        data: Arc<AccessKitEvent>,
    },

    /// An arbitary event from the Blitz embedder
    Embedder(Arc<dyn Any + Send + Sync>),

    /// Navigate to another URL (triggered by e.g. clicking a link)
    Navigate(Box<NavigationOptions>),

    /// Navigate to another URL (triggered by e.g. clicking a link)
    NavigationLoad {
        url: String,
        contents: String,
        retain_scroll_position: bool,
        is_md: bool,
    },

    /// Delivered after the WASM resize-debounce window expires. Route to
    /// `View::apply_pending_resize_if_settled`, which applies the pending
    /// size iff motion has actually settled.
    #[cfg(target_arch = "wasm32")]
    ResizeSettleCheck {
        window_id: WindowId,
    },
}
impl BlitzShellEvent {
    pub fn embedder_event<T: Any + Send + Sync>(value: T) -> Self {
        let boxed = Arc::new(value) as Arc<dyn Any + Send + Sync>;
        Self::Embedder(boxed)
    }
}

#[derive(Clone)]
pub struct BlitzShellProxy(Arc<BlitzShellProxyInner>);
pub struct BlitzShellProxyInner {
    winit_proxy: EventLoopProxy,
    sender: Sender<BlitzShellEvent>,
}

impl BlitzShellProxy {
    pub fn new(winit_proxy: EventLoopProxy) -> (Self, Receiver<BlitzShellEvent>) {
        let (sender, receiver) = channel();
        let proxy = Self(Arc::new(BlitzShellProxyInner {
            winit_proxy,
            sender,
        }));
        (proxy, receiver)
    }

    pub fn wake_up(&self) {
        self.0.winit_proxy.wake_up();
    }
    pub fn send_event(&self, event: impl Into<BlitzShellEvent>) {
        self.send_event_impl(event.into());
    }
    fn send_event_impl(&self, event: BlitzShellEvent) {
        let _ = self.0.sender.send(event);
        self.wake_up();
    }
}

impl NetWaker for BlitzShellProxy {
    fn wake(&self, client_id: usize) {
        self.send_event_impl(BlitzShellEvent::RequestRedraw { doc_id: client_id })
    }
}

/// Create a waker that will send a poll event to the event loop.
///
/// This lets the VirtualDom "come up for air" and process events while the main thread is blocked by the WebView.
///
/// All other IO lives in the Tokio runtime,
pub fn create_waker(proxy: &BlitzShellProxy, id: WindowId) -> std::task::Waker {
    struct DomHandle {
        proxy: BlitzShellProxy,
        id: WindowId,
    }
    impl ArcWake for DomHandle {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            let event = BlitzShellEvent::Poll {
                window_id: arc_self.id,
            };
            arc_self.proxy.send_event(event)
        }
    }

    let proxy = proxy.clone();
    futures_util::task::waker(Arc::new(DomHandle { id, proxy }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use winit::event_loop::EventLoopProxyProvider;

    /// Stands in for the event loop a real proxy wakes, counting its wake-ups.
    #[derive(Debug, Default)]
    struct CountedWakes(AtomicUsize);

    impl EventLoopProxyProvider for CountedWakes {
        fn wake_up(&self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    /// The redraw route's first hop, with no window: a redraw request goes through the proxy to
    /// its receiver, and wakes the event loop once. What comes after it needs a real window and
    /// stays with the windowed boot smoke: `BlitzShellProvider::request_redraw` holds one, and the
    /// application's handling of `RequestRedraw` asks one for its frame.
    #[test]
    fn a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once() {
        let wakes = Arc::new(CountedWakes::default());
        let (proxy, receiver) = BlitzShellProxy::new(EventLoopProxy::new(wakes.clone()));
        assert!(
            receiver.try_recv().is_err(),
            "the fixture's receiver starts empty"
        );

        proxy.send_event(BlitzShellEvent::RequestRedraw { doc_id: 7 });

        assert!(
            matches!(
                receiver.try_recv(),
                Ok(BlitzShellEvent::RequestRedraw { doc_id: 7 })
            ),
            "the request reaches the receiver"
        );
        assert!(
            receiver.try_recv().is_err(),
            "the one request is the one event"
        );
        assert_eq!(wakes.0.load(Ordering::SeqCst), 1, "the loop wakes once");
    }
}
