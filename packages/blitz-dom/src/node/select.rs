//! A single-choice `select`'s list of options and the rules that choose one of them: its chosen
//! option, its value, the selectedness setting algorithm, a key's step to another option and a
//! typed prefix's match.
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

/// Whether `element` is a `select` without `multiple`: one whose choice is a single option.
pub(crate) fn is_single_select(element: &ElementData) -> bool {
    element.name.local == local_name!("select") && !element.has_attr(local_name!("multiple"))
}

/// Whether `node` is an `option` whose parent is a `select`.
pub(crate) fn is_select_option(node: &Node) -> bool {
    node.data.is_element_with_tag_name(&local_name!("option"))
        && node.parent.is_some_and(|parent| {
            node.with(parent)
                .data
                .is_element_with_tag_name(&local_name!("select"))
        })
}

/// The select's list of options: its `option` children in tree order. The options of an
/// `optgroup` are not listed.
pub(crate) fn options(doc: &BaseDocument, select: NodeId) -> Vec<NodeId> {
    doc.get_node(select)
        .map(|node| {
            node.children
                .iter()
                .copied()
                .filter(|child| {
                    doc.get_node(*child).is_some_and(|child| {
                        child.data.is_element_with_tag_name(&local_name!("option"))
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Whether `option` is selected.
pub(crate) fn is_chosen(node: &Node) -> bool {
    node.element_data()
        .is_some_and(|element| element.element_state.contains(ElementState::CHECKED))
}

/// The select's chosen option: the last of its options that is selected.
pub(crate) fn chosen_option(doc: &BaseDocument, select: NodeId) -> Option<NodeId> {
    options(doc, select)
        .into_iter()
        .rev()
        .find(|option| doc.get_node(*option).is_some_and(is_chosen))
}

/// What the rules read of each of `options`.
pub(crate) fn readings(doc: &BaseDocument, options: &[NodeId]) -> Vec<OptionReading> {
    options
        .iter()
        .filter_map(|option| doc.get_node(*option))
        .map(|node| OptionReading {
            disabled: node
                .element_data()
                .is_some_and(|element| element.has_attr(local_name!("disabled"))),
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

/// The index of the option `step` moves the choice to from the chosen option, skipping disabled
/// options; `None` where there is no such option. With none chosen, `Next` reads as `First` and
/// `Previous` as `Last`.
pub(crate) fn stepped(options: &[OptionReading], step: SelectStep) -> Option<usize> {
    let enabled = |(_, option): &(usize, &OptionReading)| !option.disabled;
    let current = options.iter().rposition(|option| option.chosen);
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
        for (index, (step, expected)) in rows.iter().enumerate() {
            assert_eq!(stepped(&list, *step), *expected, "row {index}");
        }
        let at_last = options(&[("a", false, false), ("b", false, true)]);
        assert_eq!(stepped(&at_last, SelectStep::Next), None);
        let at_first = options(&[("a", false, true), ("b", false, false)]);
        assert_eq!(stepped(&at_first, SelectStep::Previous), None);
        let none_chosen = options(&[("a", false, false), ("b", false, false)]);
        assert_eq!(stepped(&none_chosen, SelectStep::Next), Some(0));
        assert_eq!(stepped(&none_chosen, SelectStep::Previous), Some(1));
        assert_eq!(stepped(&[], SelectStep::First), None);
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
