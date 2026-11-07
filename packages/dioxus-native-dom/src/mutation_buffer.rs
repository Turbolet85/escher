//! A recording of the mutations a Dioxus render writes, replayed into the document afterwards.
//!
//! The vdom runs user code while it renders (component bodies, and the tasks it polls), and that
//! code may reach the document through a `MountedData` call. So a render writes into this buffer
//! with the document unborrowed, and the buffer is then replayed, in recorded order, into the
//! document's writer under one borrow. This is upstream's buffered render (`286874cc1`,
//! <https://github.com/DioxusLabs/blitz/pull/1260>), which records Dioxus 0.8's stack-machine
//! calls, ported onto the id-addressed calls of Dioxus 0.7's [`WriteMutations`].
use dioxus_core::{AttributeValue, ElementId, Template, WriteMutations};
use std::ops::Range;

/// One recorded [`WriteMutations`] call, its text arguments held as spans of
/// [`MutationBuffer::strings`].
enum BufferedMutation {
    AppendChildren {
        id: ElementId,
        m: usize,
    },
    AssignNodeId {
        path: &'static [u8],
        id: ElementId,
    },
    CreatePlaceholder(ElementId),
    CreateTextNode {
        value: Range<usize>,
        id: ElementId,
    },
    LoadTemplate {
        template: Template,
        index: usize,
        id: ElementId,
    },
    ReplaceNodeWith {
        id: ElementId,
        m: usize,
    },
    ReplacePlaceholderWithNodes {
        path: &'static [u8],
        m: usize,
    },
    InsertNodesAfter {
        id: ElementId,
        m: usize,
    },
    InsertNodesBefore {
        id: ElementId,
        m: usize,
    },
    SetAttribute {
        name: &'static str,
        ns: Option<&'static str>,
        value: AttributeValue,
        id: ElementId,
    },
    SetNodeText {
        value: Range<usize>,
        id: ElementId,
    },
    CreateEventListener {
        name: &'static str,
        id: ElementId,
    },
    RemoveEventListener {
        name: &'static str,
        id: ElementId,
    },
    RemoveNode(ElementId),
    PushRoot(ElementId),
}

/// Records the mutations a render writes so that they can be applied to the document once the
/// vdom, and any user code it runs, has finished.
///
/// The buffer keeps its allocations when it is replayed, so later renders reuse them.
#[derive(Default)]
pub(crate) struct MutationBuffer {
    mutations: Vec<BufferedMutation>,
    /// The concatenated text arguments of `mutations`.
    strings: String,
}

impl std::fmt::Debug for MutationBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutationBuffer")
            .field("len", &self.mutations.len())
            .finish_non_exhaustive()
    }
}

impl MutationBuffer {
    fn push_str(&mut self, value: &str) -> Range<usize> {
        let start = self.strings.len();
        self.strings.push_str(value);
        start..self.strings.len()
    }

    /// Applies the recorded calls to `to` in the order they were recorded, and empties the buffer.
    pub(crate) fn replay(&mut self, to: &mut impl WriteMutations) {
        let strings = self.strings.as_str();
        let text = |span: Range<usize>| strings.get(span).unwrap_or_default();

        for mutation in self.mutations.drain(..) {
            match mutation {
                BufferedMutation::AppendChildren { id, m } => to.append_children(id, m),
                BufferedMutation::AssignNodeId { path, id } => to.assign_node_id(path, id),
                BufferedMutation::CreatePlaceholder(id) => to.create_placeholder(id),
                BufferedMutation::CreateTextNode { value, id } => {
                    to.create_text_node(text(value), id)
                }
                BufferedMutation::LoadTemplate {
                    template,
                    index,
                    id,
                } => to.load_template(template, index, id),
                BufferedMutation::ReplaceNodeWith { id, m } => to.replace_node_with(id, m),
                BufferedMutation::ReplacePlaceholderWithNodes { path, m } => {
                    to.replace_placeholder_with_nodes(path, m)
                }
                BufferedMutation::InsertNodesAfter { id, m } => to.insert_nodes_after(id, m),
                BufferedMutation::InsertNodesBefore { id, m } => to.insert_nodes_before(id, m),
                BufferedMutation::SetAttribute {
                    name,
                    ns,
                    value,
                    id,
                } => to.set_attribute(name, ns, &value, id),
                BufferedMutation::SetNodeText { value, id } => to.set_node_text(text(value), id),
                BufferedMutation::CreateEventListener { name, id } => {
                    to.create_event_listener(name, id)
                }
                BufferedMutation::RemoveEventListener { name, id } => {
                    to.remove_event_listener(name, id)
                }
                BufferedMutation::RemoveNode(id) => to.remove_node(id),
                BufferedMutation::PushRoot(id) => to.push_root(id),
            }
        }

        self.strings.clear();
    }
}

impl WriteMutations for MutationBuffer {
    fn append_children(&mut self, id: ElementId, m: usize) {
        self.mutations
            .push(BufferedMutation::AppendChildren { id, m });
    }

    fn assign_node_id(&mut self, path: &'static [u8], id: ElementId) {
        self.mutations
            .push(BufferedMutation::AssignNodeId { path, id });
    }

    fn create_placeholder(&mut self, id: ElementId) {
        self.mutations.push(BufferedMutation::CreatePlaceholder(id));
    }

    fn create_text_node(&mut self, value: &str, id: ElementId) {
        let value = self.push_str(value);
        self.mutations
            .push(BufferedMutation::CreateTextNode { value, id });
    }

    fn load_template(&mut self, template: Template, index: usize, id: ElementId) {
        self.mutations.push(BufferedMutation::LoadTemplate {
            template,
            index,
            id,
        });
    }

    fn replace_node_with(&mut self, id: ElementId, m: usize) {
        self.mutations
            .push(BufferedMutation::ReplaceNodeWith { id, m });
    }

    fn replace_placeholder_with_nodes(&mut self, path: &'static [u8], m: usize) {
        self.mutations
            .push(BufferedMutation::ReplacePlaceholderWithNodes { path, m });
    }

    fn insert_nodes_after(&mut self, id: ElementId, m: usize) {
        self.mutations
            .push(BufferedMutation::InsertNodesAfter { id, m });
    }

    fn insert_nodes_before(&mut self, id: ElementId, m: usize) {
        self.mutations
            .push(BufferedMutation::InsertNodesBefore { id, m });
    }

    fn set_attribute(
        &mut self,
        name: &'static str,
        ns: Option<&'static str>,
        value: &AttributeValue,
        id: ElementId,
    ) {
        self.mutations.push(BufferedMutation::SetAttribute {
            name,
            ns,
            value: value.clone(),
            id,
        });
    }

    fn set_node_text(&mut self, value: &str, id: ElementId) {
        let value = self.push_str(value);
        self.mutations
            .push(BufferedMutation::SetNodeText { value, id });
    }

    fn create_event_listener(&mut self, name: &'static str, id: ElementId) {
        self.mutations
            .push(BufferedMutation::CreateEventListener { name, id });
    }

    fn remove_event_listener(&mut self, name: &'static str, id: ElementId) {
        self.mutations
            .push(BufferedMutation::RemoveEventListener { name, id });
    }

    fn remove_node(&mut self, id: ElementId) {
        self.mutations.push(BufferedMutation::RemoveNode(id));
    }

    fn push_root(&mut self, id: ElementId) {
        self.mutations.push(BufferedMutation::PushRoot(id));
    }
}
