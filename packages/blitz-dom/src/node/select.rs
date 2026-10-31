//! A `select`'s list of options, through its `optgroup`s, and the rules that choose among them:
//! a single-choice select's chosen option, the selectedness setting algorithm, a key's step to
//! another option and a typed prefix's match; and a `multiple` select's set, by a gesture on one
//! row.
//!
//! An option's selectedness is held as its `CHECKED` element state, which `:checked` reads.
//!
//! <https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element>

use blitz_traits::node_id::NodeId;
use markup5ever::local_name;
use web_time::Duration;

use super::{ElementData, Node};
use crate::BaseDocument;
use style_dom::ElementState;

/// What the rules read of one option in a select's list of options.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OptionReading {
    /// Whether the option carries `disabled`.
    pub(crate) disabled: bool,
    /// Whether the option is selected.
    pub(crate) chosen: bool,
    /// The option's label.
    pub(crate) label: String,
}

/// A move of a select's choice by one of its keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum SelectStep {
    /// To the next option that is not disabled.
    Next,
    /// To the previous option that is not disabled.
    Previous,
    /// To the first option that is not disabled.
    First,
    /// To the last option that is not disabled.
    Last,
}

/// The text typed into a focused select so far, and when its last character came.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Typeahead {
    /// The select the text was typed into.
    pub(crate) select: NodeId,
    /// The text typed so far.
    pub(crate) text: String,
    /// The document clock's reading at the last character.
    pub(crate) at: Duration,
}

/// How long after its last character typed text is kept for the next one.
pub(crate) const TYPEAHEAD_TIMEOUT: Duration = Duration::from_secs(1);

/// Where a focused `multiple` select's keys and clicks stand.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SelectCursor {
    /// The select the rows belong to.
    pub(crate) select: NodeId,
    /// The row a Shift gesture spans from.
    pub(crate) anchor: NodeId,
    /// The row the keys step from and Space toggles.
    pub(crate) current: NodeId,
}

/// A gesture on one row of a `multiple` select, the row an index into its list of options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum RowGesture {
    /// Chooses the row alone.
    Alone(usize),
    /// Chooses the enabled rows from the anchor to the row, and no other.
    Span(usize),
    /// Flips the row, the rest of the set standing.
    Toggle(usize),
    /// Moves the current row, the set standing.
    Move(usize),
}

/// What a [`RowGesture`] leaves.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Gestured {
    /// The chosen rows it leaves; `None` where it leaves the set as it stands.
    pub(crate) set: Option<Vec<usize>>,
    /// The row a Shift gesture spans from next.
    pub(crate) anchor: usize,
    /// The current row.
    pub(crate) current: usize,
}

/// Whether `element` is a `select` without `multiple`: one whose choice is a single option.
pub(crate) fn is_single_select(element: &ElementData) -> bool {
    element.name.local == local_name!("select") && !element.has_attr(local_name!("multiple"))
}

/// Whether `element` is a `select` with `multiple`: one whose choice is a set of options.
pub(crate) fn is_multiple_select(element: &ElementData) -> bool {
    element.name.local == local_name!("select") && element.has_attr(local_name!("multiple"))
}

/// The select an `option` or an `optgroup` belongs to: its parent, or an option's group's parent.
fn select_holding(node: &Node) -> Option<NodeId> {
    let is_option = node.data.is_element_with_tag_name(&local_name!("option"));
    if !is_option && !node.data.is_element_with_tag_name(&local_name!("optgroup")) {
        return None;
    }
    let parent = node.with(node.parent?);
    if parent.data.is_element_with_tag_name(&local_name!("select")) {
        return Some(parent.id);
    }
    if !is_option
        || !parent
            .data
            .is_element_with_tag_name(&local_name!("optgroup"))
    {
        return None;
    }
    let group_parent = node.with(parent.parent?);
    group_parent
        .data
        .is_element_with_tag_name(&local_name!("select"))
        .then_some(group_parent.id)
}

/// The select the option or the group `node` belongs to; `None` for an id that does not resolve
/// and for any other node.
pub(crate) fn owner_select(doc: &BaseDocument, node: NodeId) -> Option<NodeId> {
    select_holding(doc.get_node(node)?)
}

/// Whether `node` is an `option` in a `select`'s list of options.
pub(crate) fn is_select_option(node: &Node) -> bool {
    node.data.is_element_with_tag_name(&local_name!("option")) && select_holding(node).is_some()
}

/// https://html.spec.whatwg.org/multipage/form-elements.html#concept-select-option-list: the
/// select's list of options, its `option` children and the `option` children of its `optgroup`
/// children, in tree order.
pub(crate) fn options(doc: &BaseDocument, select: NodeId) -> Vec<NodeId> {
    let Some(node) = doc.get_node(select) else {
        return Vec::new();
    };
    let mut list = Vec::new();
    for child in node.children.iter().copied() {
        let Some(child_node) = doc.get_node(child) else {
            continue;
        };
        if child_node
            .data
            .is_element_with_tag_name(&local_name!("option"))
        {
            list.push(child);
        } else if child_node
            .data
            .is_element_with_tag_name(&local_name!("optgroup"))
        {
            list.extend(child_node.children.iter().copied().filter(|grandchild| {
                doc.get_node(*grandchild).is_some_and(|grandchild| {
                    grandchild
                        .data
                        .is_element_with_tag_name(&local_name!("option"))
                })
            }));
        }
    }
    list
}

/// Whether `option` is selected.
pub(crate) fn is_chosen(node: &Node) -> bool {
    node.element_data()
        .is_some_and(|element| element.element_state.contains(ElementState::CHECKED))
}

/// https://html.spec.whatwg.org/multipage/form-elements.html#concept-option-disabled: whether the
/// option carries `disabled` or its `optgroup` does.
pub(crate) fn option_disabled(node: &Node) -> bool {
    let carries = |node: &Node| {
        node.element_data()
            .is_some_and(|element| element.has_attr(local_name!("disabled")))
    };
    carries(node)
        || node.parent.is_some_and(|parent| {
            let parent = node.with(parent);
            parent
                .data
                .is_element_with_tag_name(&local_name!("optgroup"))
                && carries(parent)
        })
}

/// The select's chosen option: the last of its options that is selected.
pub(crate) fn chosen_option(doc: &BaseDocument, select: NodeId) -> Option<NodeId> {
    options(doc, select)
        .into_iter()
        .rev()
        .find(|option| doc.get_node(*option).is_some_and(is_chosen))
}

/// The first of the select's options that is selected.
pub(crate) fn first_chosen_option(doc: &BaseDocument, select: NodeId) -> Option<NodeId> {
    options(doc, select)
        .into_iter()
        .find(|option| doc.get_node(*option).is_some_and(is_chosen))
}

/// What the rules read of each of `options`.
pub(crate) fn readings(doc: &BaseDocument, options: &[NodeId]) -> Vec<OptionReading> {
    options
        .iter()
        .filter_map(|option| doc.get_node(*option))
        .map(|node| OptionReading {
            disabled: option_disabled(node),
            chosen: is_chosen(node),
            label: option_label(node),
        })
        .collect()
}

/// https://html.spec.whatwg.org/multipage/form-elements.html#concept-option-label: the option's
/// `label` attribute where it is not empty, else its text.
pub(crate) fn option_label(node: &Node) -> String {
    match node
        .element_data()
        .and_then(|element| element.attr(local_name!("label")))
    {
        Some(label) if !label.is_empty() => label.to_string(),
        _ => option_text(node),
    }
}

/// https://html.spec.whatwg.org/multipage/form-elements.html#concept-option-value: the option's
/// `value` attribute where it has one, else its text.
pub(crate) fn option_value(node: &Node) -> String {
    match node
        .element_data()
        .and_then(|element| element.attr(local_name!("value")))
    {
        Some(value) => value.to_string(),
        None => option_text(node),
    }
}

/// https://html.spec.whatwg.org/multipage/form-elements.html#dom-option-text: the option's text
/// with ASCII whitespace stripped and collapsed.
fn option_text(node: &Node) -> String {
    strip_and_collapse(&node.text_content())
}

/// https://infra.spec.whatwg.org/#strip-and-collapse-ascii-whitespace
fn strip_and_collapse(text: &str) -> String {
    text.split(|c: char| c.is_ascii_whitespace())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// https://html.spec.whatwg.org/multipage/form-elements.html#selectedness-setting-algorithm, for
/// a select without `multiple`: the index of the option to be its only chosen one — the last
/// option that is selected, else the first option that is not disabled; `None` where there is
/// neither.
pub(crate) fn settled(options: &[OptionReading]) -> Option<usize> {
    options
        .iter()
        .rposition(|option| option.chosen)
        .or_else(|| options.iter().position(|option| !option.disabled))
}

/// The last of `options` that is chosen: the row a key steps from where nothing else says.
pub(crate) fn last_chosen(options: &[OptionReading]) -> Option<usize> {
    options.iter().rposition(|option| option.chosen)
}

/// The index of the option `step` moves to from the row `current`, skipping disabled options;
/// `None` where there is no such option. With no row, `Next` reads as `First` and `Previous` as
/// `Last`.
pub(crate) fn stepped(
    options: &[OptionReading],
    step: SelectStep,
    current: Option<usize>,
) -> Option<usize> {
    let enabled = |(_, option): &(usize, &OptionReading)| !option.disabled;
    let mut forward = options.iter().enumerate().filter(enabled);
    let mut backward = options.iter().enumerate().rev().filter(enabled);
    let found = match (step, current) {
        (SelectStep::Next, Some(current)) => forward.find(|(index, _)| *index > current),
        (SelectStep::Previous, Some(current)) => backward.find(|(index, _)| *index < current),
        (SelectStep::First, _) | (SelectStep::Next, None) => forward.next(),
        (SelectStep::Last, _) | (SelectStep::Previous, None) => backward.next(),
    };
    found.map(|(index, _)| index)
}

/// The index of the first option that is not disabled whose label starts with `text`, compared
/// case-insensitively by lowercase; `None` for an empty text or where no label matches.
pub(crate) fn matched(options: &[OptionReading], text: &str) -> Option<usize> {
    if text.is_empty() {
        return None;
    }
    let text = text.to_lowercase();
    options
        .iter()
        .position(|option| !option.disabled && option.label.to_lowercase().starts_with(&text))
}

/// The enabled rows from `from` to `to`, either way round, in list order.
pub(crate) fn spanned(options: &[OptionReading], from: usize, to: usize) -> Vec<usize> {
    let (low, high) = (from.min(to), from.max(to));
    options
        .iter()
        .enumerate()
        .filter(|(index, option)| (low..=high).contains(index) && !option.disabled)
        .map(|(index, _)| index)
        .collect()
}

/// What `gesture` leaves on a `multiple` select's rows, a Shift gesture spanning from `anchor`
/// (the row itself where there is none). A disabled row is never chosen, and a gesture on one
/// leaves the set as it stands.
pub(crate) fn gestured(
    options: &[OptionReading],
    anchor: Option<usize>,
    gesture: RowGesture,
) -> Gestured {
    let disabled = |row: usize| options.get(row).is_none_or(|option| option.disabled);
    match gesture {
        RowGesture::Alone(row) => Gestured {
            set: (!disabled(row)).then(|| vec![row]),
            anchor: row,
            current: row,
        },
        RowGesture::Span(row) => {
            let anchor = anchor.unwrap_or(row);
            Gestured {
                set: Some(spanned(options, anchor, row)),
                anchor,
                current: row,
            }
        }
        RowGesture::Toggle(row) => Gestured {
            set: (!disabled(row)).then(|| {
                options
                    .iter()
                    .enumerate()
                    .filter(|(index, option)| option.chosen != (*index == row))
                    .map(|(index, _)| index)
                    .collect()
            }),
            anchor: row,
            current: row,
        },
        RowGesture::Move(row) => Gestured {
            set: None,
            anchor: anchor.unwrap_or(row),
            current: row,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One option's `(label, disabled, chosen)`.
    type Row = (&'static str, bool, bool);

    /// Options from rows of `(label, disabled, chosen)`.
    fn options(rows: &[Row]) -> Vec<OptionReading> {
        rows.iter()
            .map(|(label, disabled, chosen)| OptionReading {
                disabled: *disabled,
                chosen: *chosen,
                label: label.to_string(),
            })
            .collect()
    }

    #[test]
    fn the_selectedness_setting_algorithm_keeps_one_chosen_option() {
        // (options, the index left chosen)
        let rows: [(&[Row], Option<usize>); 7] = [
            (
                &[("a", false, false), ("b", false, true), ("c", false, false)],
                Some(1),
            ),
            (&[("a", false, false), ("b", false, false)], Some(0)),
            (&[("a", true, false), ("b", false, false)], Some(1)),
            (
                &[("a", false, true), ("b", false, true), ("c", false, false)],
                Some(1),
            ),
            (&[("a", true, true), ("b", false, false)], Some(0)),
            (&[("a", true, false), ("b", true, false)], None),
            (&[], None),
        ];
        for (index, (rows, expected)) in rows.iter().enumerate() {
            assert_eq!(settled(&options(rows)), *expected, "row {index}");
        }
    }

    #[test]
    fn a_step_skips_disabled_options_and_stops_at_the_ends() {
        let list = options(&[
            ("a", true, false),
            ("b", false, false),
            ("c", false, true),
            ("d", true, false),
            ("e", false, false),
            ("f", true, false),
        ]);
        let rows = [
            (SelectStep::Next, Some(4)),
            (SelectStep::Previous, Some(1)),
            (SelectStep::First, Some(1)),
            (SelectStep::Last, Some(4)),
        ];
        let current = last_chosen(&list);
        for (index, (step, expected)) in rows.iter().enumerate() {
            assert_eq!(stepped(&list, *step, current), *expected, "row {index}");
        }
        let at_last = options(&[("a", false, false), ("b", false, true)]);
        assert_eq!(
            stepped(&at_last, SelectStep::Next, last_chosen(&at_last)),
            None
        );
        let at_first = options(&[("a", false, true), ("b", false, false)]);
        assert_eq!(
            stepped(&at_first, SelectStep::Previous, last_chosen(&at_first)),
            None
        );
        let none_chosen = options(&[("a", false, false), ("b", false, false)]);
        assert_eq!(last_chosen(&none_chosen), None);
        assert_eq!(stepped(&none_chosen, SelectStep::Next, None), Some(0));
        assert_eq!(stepped(&none_chosen, SelectStep::Previous, None), Some(1));
        assert_eq!(stepped(&[], SelectStep::First, None), None);
    }

    #[test]
    fn a_step_moves_from_the_row_it_is_given() {
        let list = options(&[
            ("a", false, true),
            ("b", true, false),
            ("c", false, false),
            ("d", false, true),
        ]);
        let rows = [
            (SelectStep::Next, Some(0), Some(2)),
            (SelectStep::Previous, Some(2), Some(0)),
            (SelectStep::Next, Some(3), None),
            (SelectStep::First, Some(3), Some(0)),
            (SelectStep::Last, Some(0), Some(3)),
        ];
        assert_eq!(rows.len(), 5);
        for (index, (step, from, expected)) in rows.iter().enumerate() {
            assert_eq!(stepped(&list, *step, *from), *expected, "row {index}");
        }
    }

    #[test]
    fn a_span_lists_the_enabled_rows_between_two_either_way_round() {
        let list = options(&[
            ("a", false, false),
            ("b", true, false),
            ("c", false, false),
            ("d", false, false),
        ]);
        let rows: [(usize, usize, &[usize]); 4] = [
            (0, 3, &[0, 2, 3]),
            (3, 0, &[0, 2, 3]),
            (2, 2, &[2]),
            (1, 1, &[]),
        ];
        assert_eq!(rows.len(), 4);
        for (index, (from, to, expected)) in rows.iter().enumerate() {
            assert_eq!(spanned(&list, *from, *to), *expected, "row {index}");
        }
    }

    #[test]
    fn a_gesture_on_a_row_leaves_the_set_it_states() {
        let list = options(&[
            ("a", false, true),
            ("b", true, false),
            ("c", false, false),
            ("d", false, true),
        ]);
        let moved = |set: Option<&[usize]>, anchor, current| Gestured {
            set: set.map(<[usize]>::to_vec),
            anchor,
            current,
        };
        // (anchor, gesture, what it leaves)
        let rows = [
            (Some(0), RowGesture::Alone(2), moved(Some(&[2]), 2, 2)),
            (Some(0), RowGesture::Alone(1), moved(None, 1, 1)),
            (Some(0), RowGesture::Span(3), moved(Some(&[0, 2, 3]), 0, 3)),
            (Some(3), RowGesture::Span(2), moved(Some(&[2, 3]), 3, 2)),
            (None, RowGesture::Span(2), moved(Some(&[2]), 2, 2)),
            (
                Some(0),
                RowGesture::Toggle(2),
                moved(Some(&[0, 2, 3]), 2, 2),
            ),
            (Some(0), RowGesture::Toggle(0), moved(Some(&[3]), 0, 0)),
            (Some(0), RowGesture::Toggle(1), moved(None, 1, 1)),
            (Some(0), RowGesture::Move(2), moved(None, 0, 2)),
            (None, RowGesture::Move(2), moved(None, 2, 2)),
        ];
        assert_eq!(rows.len(), 10);
        for (index, (anchor, gesture, expected)) in rows.iter().enumerate() {
            assert_eq!(gestured(&list, *anchor, *gesture), *expected, "row {index}");
        }
    }

    #[test]
    fn typed_text_matches_the_first_enabled_label_it_starts() {
        let list = options(&[
            ("Apple", false, false),
            ("Apricot", false, true),
            ("banana", false, false),
            ("Cherry", true, false),
            ("Éclair", false, false),
        ]);
        let rows = [
            ("a", Some(0)),
            ("apr", Some(1)),
            ("B", Some(2)),
            ("BAN", Some(2)),
            ("c", None),
            ("x", None),
            ("é", Some(4)),
            ("", None),
        ];
        for (index, (text, expected)) in rows.iter().enumerate() {
            assert_eq!(matched(&list, text), *expected, "row {index}");
        }
    }

    /// A document whose one select holds `rows` of `(tag, parent index)`, a parent index naming an
    /// earlier row, and the select and the rows' ids.
    fn select_of_rows(rows: &[(&str, Option<usize>)]) -> (BaseDocument, NodeId, Vec<NodeId>) {
        use crate::{DocumentConfig, QualName, qual_name};
        let mut doc = BaseDocument::new(DocumentConfig::default());
        let root_id = doc.root_node().id;
        let mut mutator = doc.mutate();
        let select = mutator.create_element(qual_name!("select", html), vec![]);
        mutator.append_children(root_id, &[select]);
        let mut ids = Vec::new();
        for (tag, parent) in rows.iter().copied() {
            let name = QualName::new(None, markup5ever::ns!(html), tag.into());
            let id = mutator.create_element(name, vec![]);
            let parent = parent.map_or(select, |parent| ids[parent]);
            mutator.append_children(parent, &[id]);
            ids.push(id);
        }
        drop(mutator);
        (doc, select, ids)
    }

    #[test]
    fn the_list_of_options_runs_through_a_group_in_tree_order() {
        let (doc, select, ids) = select_of_rows(&[
            ("option", None),
            ("optgroup", None),
            ("option", Some(1)),
            ("div", None),
            ("option", Some(3)),
            ("optgroup", Some(1)),
            ("option", Some(5)),
            ("option", None),
        ]);
        assert_eq!(super::options(&doc, select), [ids[0], ids[2], ids[7]]);
        let rows = [
            (ids[0], Some(select)),
            (ids[1], Some(select)),
            (ids[2], Some(select)),
            (ids[3], None),
            (ids[4], None),
            (ids[5], None),
            (ids[6], None),
            (select, None),
        ];
        assert_eq!(rows.len(), 8);
        for (index, (node, expected)) in rows.into_iter().enumerate() {
            assert_eq!(owner_select(&doc, node), expected, "row {index}");
        }
        let in_list = |node: NodeId| doc.get_node(node).is_some_and(is_select_option);
        assert!(in_list(ids[2]) && !in_list(ids[4]) && !in_list(ids[6]));
    }

    #[test]
    fn text_is_stripped_and_collapsed() {
        let rows = [
            ("  Banana  ", "Banana"),
            ("Big\n\t Banana", "Big Banana"),
            ("", ""),
            (" \u{a0}x ", "\u{a0}x"),
        ];
        for (index, (text, expected)) in rows.iter().enumerate() {
            assert_eq!(strip_and_collapse(text), *expected, "row {index}");
        }
    }
}
