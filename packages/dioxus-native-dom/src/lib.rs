#![cfg_attr(docsrs, feature(doc_cfg))]

//! Core headless native renderer for Dioxus.
//!
//! ## Feature flags
//!  - `default`: Enables the features listed below.
//!  - `accessibility`: Enables [`accesskit`](https://docs.rs/accesskit/latest/accesskit/) accessibility support,
//!    and the snapshot model (`DioxusDocument::snapshot`), its text form (`Snapshot::to_text`)
//!    and its parts (`Snapshot::text_parts`), its diff (`Snapshot::diff`) and the actionable-key check
//!    (`DioxusDocument::unkeyed_actionable`) and the actionable nodes (`DioxusDocument::actionable_nodes`)
//!    built on it, and the UI graph (`DioxusDocument::graph`),
//!    which reads the states an app registered through `use_graph_state` (a hook every build
//!    holds, so an app registers alike with the feature off).
//!  - `hot-reload`: Enables hot-reloading of Dioxus RSX.
//!  - `menu`: Enables the [`muda`](https://docs.rs/muda/latest/muda/) menubar.
//!  - `tracing`: Enables tracing support.

#[cfg(feature = "accessibility")]
mod actionable;
mod dioxus_document;
mod element_id;
mod events;
#[cfg(feature = "accessibility")]
mod graph;
mod keyboard_compat;
mod mutation_buffer;
mod mutation_writer;
mod reached;
mod signal_registry;
#[cfg(feature = "accessibility")]
mod snapshot;
#[cfg(feature = "accessibility")]
mod snapshot_diff;
#[cfg(feature = "accessibility")]
mod snapshot_text;
mod write_once_attr;
#[cfg(feature = "accessibility")]
pub use actionable::{ActionableNode, UnkeyedActionable};
pub use blitz_dom::DocumentConfig;
pub use dioxus_document::DioxusDocument;
pub use element_id::{VIRTUAL_KEY_SEPARATOR, virtual_node_id};
pub use events::{NodeHandle, synthetic_click_event, synthetic_form_event};
#[cfg(feature = "accessibility")]
pub use graph::{GraphHandler, GraphNode, GraphState, GraphWrite, GraphWriter, UiGraph, WriteFlag};
pub use reached::ReachedHandler;
pub use signal_registry::{DeclaredWrite, SignalRegistry, use_graph_state};
#[cfg(feature = "accessibility")]
pub use snapshot::{
    MASKED_VALUE, NodeState, Snapshot, SnapshotNode, closed_select_of, is_masked_input,
    is_multiple_select, is_select,
};
#[cfg(feature = "accessibility")]
pub use snapshot_diff::{DiffNode, Reading, SnapshotDiff};
#[cfg(feature = "accessibility")]
pub use snapshot_text::{SNAPSHOT_TEXT_BUDGET, TextPart};
pub use write_once_attr::{CustomWidgetAttr, SubDocumentAttr};

pub use blitz_dom::NodeId;
use blitz_dom::{LocalName, Namespace, QualName, ns};

pub(crate) fn qual_name(local_name: &str, namespace: Option<&str>) -> QualName {
    QualName {
        prefix: None,
        ns: namespace.map(Namespace::from).unwrap_or(ns!(html)),
        local: LocalName::from(local_name),
    }
}

// Syntax sugar to make tracing calls less noisy in function below
macro_rules! trace {
    ($pattern:literal) => {{
        #[cfg(feature = "tracing")]
        tracing::debug!($pattern);
    }};
    ($pattern:literal, $item1:expr) => {{
        #[cfg(feature = "tracing")]
        tracing::debug!($pattern, $item1);
    }};
    ($pattern:literal, $item1:expr, $item2:expr) => {{
        #[cfg(feature = "tracing")]
        tracing::debug!($pattern, $item1, $item2);
    }};
    ($pattern:literal, $item1:expr, $item2:expr, $item3:expr) => {{
        #[cfg(feature = "tracing")]
        tracing::debug!($pattern, $item1, $item2);
    }};
    ($pattern:literal, $item1:expr, $item2:expr, $item3:expr, $item4:expr) => {{
        #[cfg(feature = "tracing")]
        tracing::debug!($pattern, $item1, $item2, $item3, $item4);
    }};
}
pub(crate) use trace;
