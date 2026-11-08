use dioxus_native::prelude::*;
use dioxus_native::{DeclaredWrite, use_graph_state};

/// A `use_signal` the UI graph also reads: registered under `name`, `writers` the handlers the
/// task's code writes it from during their own dispatch. A write from no handler — a tick of
/// the time step, a task an async handler spawned, a `mounted` handler — is declared by none.
pub fn use_graph_signal<T: 'static>(
    name: &'static str,
    writers: &'static [DeclaredWrite],
    init: impl FnOnce() -> T,
) -> Signal<T> {
    let signal = use_signal(init);
    use_graph_state(name, writers, signal.subscribers());
    signal
}

/// The declaration of a handler's write: the element `id` carries it, on `event`.
pub(crate) const fn writes(id: &'static str, event: &'static str) -> DeclaredWrite {
    DeclaredWrite::new(id, event)
}

pub mod cells;
pub mod circle_drawer;
pub mod counter;
pub mod crud;
pub mod flight_booker;
pub mod temp_converter;
pub mod timer;
