# security · cascade read · group 3 of 6

## Carried
- (.andromeda/security-plan-detail/input-validation.md:173) `8b1a0a3ce` moves layout damage for `<col span>` and SVG attributes, so under any take carrying it these numeric attributes must keep their closed parses, fallbacks and floor.
  > `.parse().ok()` with default fallbacks: textarea `rows` (default 2) and `cols` (packages/blitz-dom/src/layout/mod.rs:280-287), replaced-element `width`/`height` (packages/blitz-dom/src/layout/mod.rs:384-391), `<col span>` floored at 1 (packages/blitz-dom/src/layout/table.rs:469-473), `colspan` default 1 (packages/blitz-dom/src/layout/table.rs:626-629)
  (.andromeda/security-plan.md:131)
  > Legacy attributes — `bgcolor`, dimensions, SVG `width` / `height`, `border` and body margin pass closed parsers; anything else is ignored or rejected.
- (.andromeda/security-plan-detail/input-validation.md:179) A conflict resolution in `stylo.rs` or `default.css` must keep the `size` hint's parse, cap and scroll rule as ratified, widening nothing past it, with its checks green in both layout modes.
  > In every document, remote pages included: `size_rows` parses it through `parse_unsigned_integer`, capped at 65534, `0` or a non-numeric value giving no hint, and a valid value hints a `height` of `size` × 1.2em, carrying no text out; `default.css` gives such a select `overflow-y: auto` and its rows `line-height: 1.2`, and `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size`; a single select's `size` is unread; pinned by the unit `size_reads_a_whole_number_above_zero_capped` and, both layout modes, `hostile_markup_keeps_every_box_finite_and_panics_nowhere` and `size_makes_a_multiple_select_that_many_rows_tall_and_a_key_brings_the_next_in`
  (.andromeda/security-plan-detail/input-validation.md:179)
  > an engine change for every document and a boundary widening, ratified
  (.andromeda/security-plan.md:133)
  > Markup numeric attributes — parsed with default fallbacks; `rowspan` clamped to 1..=65534, `size` on a `multiple` select capped at 65534 with `0` or a non-numeric value no hint, `<ol start>` parsed as `usize`; the `size` read is an engine change for every document and a boundary widening, ratified.
- (.andromeda/security-plan-detail/input-validation.md:182) A resolution of `layout/construct.rs` must keep this reader's parse, and any change to its subtraction is named as an edit of the conflict.
  > Parsed as `usize` and has 1 subtracted (packages/blitz-dom/src/layout/construct.rs:475-481)
- (.andromeda/security-plan-detail/input-validation.md:185) The focus order the chunk re-proves must still read a malformed `tabindex` as no value and `disabled` by presence, not through this reader.
  > Returns `None` on parse failure, as for `tabindex` (packages/blitz-dom/src/node/element.rs:502-505); `disabled` no longer goes through it since 2026-10-13-stand-keyboard-harness, read by presence (packages/blitz-dom/src/node/element.rs:742-744; the `disabled` row)
- (.andromeda/security-plan-detail/input-validation.md:188) `stand_keyboard` and the snapshot checks re-run after the take must still read `disabled` by presence on a form control alone, across focus, activation and the snapshot's state.
  > Read by its presence — bare, `"false"` or `"true"` — on a form control alone (ratified by the founder, 2026-10-18): `ElementData::is_disabled` is `can_be_disabled()` (`button` · `input` · `select` · `textarea`) and the attribute present, and focusability (`ElementData::flush_is_focussable`, so the Tab walk, autofocus and `DioxusDocument::unkeyed_actionable`), the pointer-move gate, `ClickTarget::Disabled` and `handle_click`'s walk gate read it, so a `<div disabled tabindex=0>` takes focus and a disabled ancestor `div` does not stop the walk
  (.andromeda/security-plan-detail/input-validation.md:189)
  > the Enter and Space activation arms act only on what `ElementData::is_disabled` does not read disabled
  (.andromeda/security-plan-detail/input-validation.md:189)
  > the snapshot's `NodeState::enabled` follows the same presence reading as focusability, so a `disabled="false"` and a bare `disabled` both read `Some(false)` and neither is focusable
- (.andromeda/security-plan-detail/input-validation.md:192) Resolving `mutator.rs` and `stylo.rs` must keep the `open` reader on `details` and `dialog` alone, its arm independent of the range arm, and its path free of text, print and log.
  > Read by its presence on a `details` or a `dialog`, in every document, remote pages included: `ElementData::flush_open_state` sets `ElementState::OPEN` from it
  (.andromeda/security-plan-detail/input-validation.md:192)
  > in `set_attribute` the `open` arm stands after the range re-sanitize arm as a second independent `if`; no attribute text is carried out and no print or log site stands on the path;
- (.andromeda/security-plan-detail/input-validation.md:195) A resolution of `mutator.rs` must keep the `autofocus` reading exactly as stated.
  > On a focusable element, under blitz-dom's `autofocus` feature, enables autofocus when present with any value but `"false"`: a bare, an empty and a `"true"` attribute each focus the element, and `"false"` is ignored
- (.andromeda/security-plan-detail/input-validation.md:201) A resolution of `mutator.rs` must keep the canvas `src` parse as the only way in.
  > Accepted only if it parses as `u64` (packages/blitz-dom/src/mutator.rs:1451-1462)
- (.andromeda/security-plan-detail/input-validation.md:204) A resolution of `dioxus_document.rs`, or a whole merge's stable-id port, must keep the id's admission, its no-panic reads and its absence from logs, with `stand_element_ids`, `stand_id_persistence`, `stand_id_edits`, the snapshot checks and the 0-needle captures deciding.
  > a Dioxus document's `element_id` / `element_ids` read the HTML `id` as an element's author key only when it is non-empty, contains no `/` and is the first element in document pre-order to claim it; an empty, `/`-bearing or later-duplicate value gives no key and anchors nothing;
  (.andromeda/security-plan-detail/input-validation.md:204)
  > a non-element, stale or detached node reads `None`, and no path panics;
  (.andromeda/security-plan-detail/input-validation.md:204)
  > the id is computed on demand and escher's own code writes it to no log, DOM or vdom — an author-key id is the element's HTML `id`, though, which Stylo's own records carry (`style::*`, `selectors::matching`): those targets are outside the sink's allowlist, so since 2026-10-07-sink-target-allowlist a host that installs escher's sink drops them whole and prints that value at no `RUST_LOG` level
  (.andromeda/security-plan-detail/input-validation.md:205)
  > under the `accessibility` feature `DioxusDocument::accessibility_tree` carries it as each element node's AccessKit `author_id` — never on a `TextRun`, the document root or the synthetic `Window` node, and never by indexing the slab with a tree id
  (.andromeda/security-plan-detail/input-validation.md:208)
  > neither the id nor the snapshot carries a `NodeId`, `ElementId`, `ScopeId` or pointer;
  (.andromeda/security-plan-detail/input-validation.md:211)
  > a keyed list row's segment carries its Dioxus key (`{tag}[{key}]`), and a Dioxus key that is empty or holds `/` is refused — the row reads the positional `{tag}:{n}` segment instead (`/ul:0/li:1` beside `/ul:0/li[ok]`), each of the two cases witnessed by a unit test;
  (.andromeda/security-plan-detail/input-validation.md:211)
  > so a row reads the same id across a re-render, a remount and a second process, and a dropped pre-remount `NodeId` reads `None` with no panic
  (.andromeda/security-plan.md:137)
  > Markup attributes — the stable element id: the HTML `id` as an author key when non-empty, `/`-free and first in document order, else an anchored, component or document path, so no `id` can make two ids equal; computed on demand and written by escher's code to no log, it leaves by the platform adapter, the driver's answers, MCP results and kept checks, and no `NodeId` crosses.
- (.andromeda/security-plan-detail/input-validation.md:216) The `accessibility_*` checks and the capture checks re-run after the take must still hold the name's reading, the label click on a select, and 0 name needles in a log.
  > an `aria-label` non-empty after trimming becomes the node's `label` (accname-1.2 §2C) and a whitespace-only one names nothing;
  (.andromeda/security-plan-detail/input-validation.md:216)
  > and a click on a label re-dispatches to its select, but for a bound single select, whose label's click focuses it and opens nothing, asserted by `select_popup`'s `a_labels_click_focuses_the_select_and_opens_nothing`
  (.andromeda/security-plan-detail/input-validation.md:217)
  > pinned by `accessibility_label`'s 13 tests
  (.andromeda/security-plan-detail/input-validation.md:217)
  > escher's own code logs no name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level
  (.andromeda/security-plan-detail/input-validation.md:219)
  > the `mcp` role's stderr at `RUST_LOG=trace` holds 0 typed-sentinel occurrences, 0 id needles and 0 name needles under both builds
- (.andromeda/security-plan-detail/input-validation.md:228) The select model's row-by-row re-read and the `stylo.rs` resolution must leave an author's `open` and `aria-expanded` steering nothing on a select.
  > In every document, remote pages included: an author's `open` on a select and `aria-expanded` on a single select steer nothing: a single select's `expanded` is the closed bool of the engine's own state, and `flush_open_state` reads `open` on a `details` or a `dialog` alone, so `open` on a select does not match `:open`; no attribute text is carried out; pinned by `an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match`, both layout modes
- (.andromeda/security-plan-detail/input-validation.md:239) Resolving `node/node.rs` and `mutator.rs`, `hit_inner`'s hunks among them, must keep `inert`'s ancestor reading, its hit stop first and its focus clear, with no panic path or log site added.
  > Read by its presence from parsed content — remote pages included — on the element or an ancestor element, through the crate-private `Node::is_inert`, an ancestor walk (ratified by the founder, 2026-10-18): an inert node is not focusable, so the Tab walk, autofocus (`process_added_subtree`) and `DioxusDocument::unkeyed_actionable` skip it; `hit_inner` answers no hit at an element carrying `inert`, before any other test;
  (.andromeda/security-plan-detail/input-validation.md:239)
  > a presence check only — no value read, no text carried out, no print, log site, env read or panic path, and no carrier, verb, key name, field, log field or span added;
  (.andromeda/security-plan-detail/input-validation.md:239)
  > a node that holds focus when it comes under `inert` gives it up: `DocumentMutator::flush`, where `focus_node_id` resolves to a node reading `is_inert()`, calls `BaseDocument::clear_focus`, no `inert` arm joining the attribute writes, so a subtree moved under an inert box is covered too, `disabled` and `hidden` unmoved — an engine change for every document, ratified
- (.andromeda/security-plan-detail/input-validation.md:242) The select list's row-by-row re-read must keep this check set whole, its non-invertible hold included where `0980cce5f`'s `hit_inner` hunk conflicts with the non-invertible answer.
  > In every document, remote pages included: a disabled, inert or hidden select and a select with no row to show open no list, and a non-invertible transform is held by the same check set, `select_list_allowed`; a disabled row changes nothing and leaves the list open; a group header chooses nothing; pinned by `a_disabled_inert_hidden_singular_or_empty_select_opens_nothing`, `a_disabled_row_changes_nothing_and_leaves_the_list_open` and `groups_read_as_headers_their_options_under_them_and_a_header_chooses_nothing`, both layout modes
- (.andromeda/security-plan-detail/input-validation.md:245) Resolving `mutator.rs` must keep the inert clear silent and the tree's `focus` naming only built nodes, with `inert_subtree` re-proving both.
  > In every document, remote pages included: `build_accessibility_tree` writes the tree's `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build; the inert focus clear fires no DOM `blur` or `focusout`; closed values, the added lines holding no log site; pinned by `inert_subtree` 6, what an assistive technology reads unmeasured — an engine change for every document, ratified
- (.andromeda/security-plan-detail/input-validation.md:248) Resolving `mutator.rs`'s range arms must keep every value finite, bounded and on its grid, and its current value inside the ratified carriers.
  > every seed, attribute write and move is sanitized — clamped to `[min, max]`, moved to the nearest grid value, the greater on a tie, the grid index computed `value/step − base/step` so the difference of two finite bounds cannot overflow, then rounded to the decimals of the step and the base and written in its shortest round-trip form
  (.andromeda/security-plan-detail/input-validation.md:249)
  > a `value` write re-seeds the current value and a `min`, `max` or `step` write or removal re-sanitizes it
  (.andromeda/security-plan-detail/input-validation.md:249)
  > the rules module holds no print, log site, `unwrap`, `expect`, panic or env read outside its tests
  (.andromeda/security-plan-detail/input-validation.md:250)
  > the current value leaves only as form submission's entry (packages/blitz-dom/src/form.rs:330-331), the `input` event's value and the snapshot's `NodeState.value` (packages/dioxus-native-dom/src/snapshot.rs:263-265) — a control value, crossing the driver's answers and MCP results inside the ratified carriers, with no carrier, verb, field, log field or span added
- (.andromeda/security-plan-detail/input-validation.md:257) A resolution of the script runtime's `runtime/events.rs` and `bootstrap.rs` must not bring back a synthesized `change` and must keep this runtime's arguments and fetch narrowing, `91ef65e21`'s `URLSearchParams` under the same argument rule.
  > `handle_click`'s checkbox and radio arms send, after `input`, a `DomEventData::Change` with the same value, the value class `input` carries — a checkbox always, a radio only where it was not checked before the click; `blitz-vibey-script`'s runtime synthesizes no `change`, so a page's script hears one `change`, from the engine;
  (.andromeda/security-plan-detail/input-validation.md:257)
  > pinned by `checkable_change_event` 3 (a click and a Space on a checkbox each fire `input` then `change`), `text_range_no_change_event` 2 and `blitz-vibey-script`'s `radio_click_fires_change_only_where_it_was_not_checked`
  (.andromeda/security-plan.md:153)
  > JS API arguments — the script runtime's native APIs check or coerce their arguments, throwing a RangeError, TypeError, SyntaxError or JS error on a bad call or falling back to a default; `fetch()` takes GET or HEAD only.
  (.andromeda/security-plan.md:101)
  > Script fetch — invalid `data:` URLs or non-UTF-8 script bytes are `FetchError::InvalidData`; a `file:` URL loads only for a `file:` document, `fetch()`, a module `import` and `<script src>` each refused otherwise with a `FetchError` — a narrowing: the default fetcher takes only `file:` and `data:`, and same-origin and CORS checks are absent.
- (.andromeda/security-plan-detail/input-validation.md:260) Resolving `mutator.rs` and `dioxus_document.rs` must keep the single select's choice, one write path, typeahead on the document's clock and masking boundary unchanged, with `select_interaction` and `stand_act_select` deciding.
  > In every document, remote pages included: a `select` without `multiple` holds its choice as its options' `CHECKED` state, seeded from a `selected` attribute at creation, a `selected` write exclusive and a cleared one unchosen, every recorded select settled at the mutator's flush and on entering the document by HTML's selectedness setting algorithm — the last chosen, else the first option not disabled; a `multiple` write or removal records the select, so removal settles it to one, and `set_option_selectedness` leaves a `multiple` select's other options alone
  (.andromeda/security-plan-detail/input-validation.md:260)
  > typeahead keeps the typed text in the document's private `select_typeahead` buffer, starting afresh for another select or after 1 s of the document's clock, and chooses the first enabled option whose label starts with it, compared in lower case, an empty text matching nothing — compared, never kept past 1 s and never carried out
  (.andromeda/security-plan-detail/input-validation.md:261)
  > every choice goes through one path that marks the changed set and dispatches `input` then the engine-emitted `change` (not cancelable, bubbling), each carrying `BaseDocument::select_value`, a choice of the chosen option firing nothing
  (.andromeda/security-plan-detail/input-validation.md:261)
  > no print, log site, env read or panic path in the rules module, and no carrier, verb, argument, key name, result field, cause, session error, exit, log field or span added
  (.andromeda/security-plan-detail/input-validation.md:261)
  > pinned by the rules module's 8 unit tests, `select_interaction` 18 and `stand_act_select` 10, both layout modes
  (.andromeda/security-plan.md:139)
  > Markup attributes — a password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path; a password input paints a bullet per character, and a recorded masked `type` is kept as `MASKED_STAND_IN`; the DOM and form submission keep the value.
- (.andromeda/security-plan-detail/input-validation.md:265) The row-by-row re-read of the select list's surface must keep one state field and this one reader, its named exceptions alone standing.
  > In every document, remote pages included: the list is one `BaseDocument` field, `select_list`, `None` at construction, read through `live_select_list` but for `reconcile_select_list`, which must see a list that reader no longer answers, and the removal arm, each reading the field past it, and `hover_list_row`, which writes `current` directly
- (.andromeda/security-plan-detail/input-validation.md:268) The list's keys must stay a closed match with one write path through the take, and a panic fix or the lift of `select_popup`'s skipped key-press row must change neither.
  > In every document, remote pages included: an open list's keys pass one closed match, `press_open_list_key`; Tab runs `choose_current_and_close` before focus moves, in either direction; `click_list_row` and `choose_current_and_close` write only through `apply_select_choice`, the one write path; pinned by `the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing`, `enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change` and `tab_chooses_and_closes_before_focus_moves_either_way`, both layout modes
  (.andromeda/security-plan.md:135)
  > Markup attributes — read from parsed content, remote pages included, by in-process readers with no log site or panic path: `disabled` and `inert` by presence, ARIA states and closed values through closed parses carrying no attribute text, the range and select controls held within their rules, Enter activating buttons and links and Space buttons as a click, Enter and Space opening a single select's engine-painted list, Space only with no typed text standing, names and descriptions as page text, an option's and a group's label also painted in an open list; they leave escher only by the ratified carriers, the page's own form submission and events aside; the list's keys and painted labels are an engine change for every document and a boundary widening, ratified.
- (.andromeda/security-plan-detail/input-validation.md:271) Resolving `scrolling.rs` and taking `e42ee1f25` or `ae774d7f4` must keep what closes the list, its `fresh` field, and the press and release rules, with scroll targets still clamped.
  > In every document, remote pages included: in the list a press starts no text drag and the release chooses the row under it; a press elsewhere closes it through `close_list_outside`; a page scroll or a box move closes it through `reconcile_select_list` at the next resolve, the list's `fresh` field keeping a list a click just opened through a `:focus` rule's restyle; pinned by `a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends`, `escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event` and `a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened`, both layout modes
  (.andromeda/security-plan.md:145)
  > Scrolling — targets are clamped to the scroll range; a programmatic scroll to a missing node does nothing.
- (.andromeda/security-plan-detail/input-validation.md:274) After the take, hostile markup must keep every box finite and panic nowhere under the list's bound, and the page CSS, stylesheet and paint changes the delta brings answer the same hostile page by the same value rules.
  > In every document, remote pages included: a hostile `size`, 2,000 options and a 5,500-character label keep every box finite and panic nowhere, asserted by `hostile_markup_keeps_every_box_finite_and_panics_nowhere`; the list shows at most `MAX_ROWS` 20 rows, those past that scrolled, pinned by `a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel`, both layout modes
  (.andromeda/security-plan.md:125)
  > CSS declarations — validated by parsing: invalid declarations are ignored, and unsupported property names or invalid values refused.
  (.andromeda/security-plan.md:107)
  > Fetched stylesheets — must be valid UTF-8, else the load errs.
  (.andromeda/security-plan.md:141)
  > Layout values — degenerate aspect ratios are discarded, inline-box heights kept finite, and `text_extent` is a read-only reader returning geometry only.
  (.andromeda/security-plan.md:143)
  > Paint values — non-finite geometry is culled, gradient overflow clamped, oversized radii scaled down, and hidden inputs not painted.

## Bindings
- **input-validation's markup readers ↔ error handling's panic paths**: a page-reached reader carries no panic path while `layout_style()` and the other node accessors panic by record, so a `hit_inner` resolution, `ebacd68bd` and any fix of the key-press panic must add no panic a page can reach, (.andromeda/security-plan-detail/input-validation.md:239) with (.andromeda/security-plan.md:360)
- **the stable id and accessible names ↔ logging's sink**: ids and names reach no log only because the sink drops other targets and allowlists engine fields, so each new call site and target the delta brings is read against it and the captures stay at 0 needles, (.andromeda/security-plan-detail/input-validation.md:204) and (.andromeda/security-plan-detail/input-validation.md:219) with (.andromeda/security-plan.md:396)
- **the stable id ↔ dependency pinning**: Stylo's records carry an author-key id, and a whole merge moves Stylo to a git dependency against the recorded pin, so that shape must re-read both the targets it brings and the rule it reverses, the record moving only on the founder's word, (.andromeda/security-plan-detail/input-validation.md:204) with (.andromeda/security-plan.md:232)
- **the id and name carriers ↔ data protection's committed checks**: a moved digest or kept answer re-recorded in a committed check keeps that check free of typed text, host path, name or secret, (.andromeda/security-plan-detail/input-validation.md:208) with (.andromeda/security-plan.md:187)
- **the select's unmasked value ↔ masking in data protection**: a resolution on the snapshot and text paths keeps a select's chosen value as the recorded unmasked crossing while a password's or file input's value stays masked in every carrier, (.andromeda/security-plan-detail/input-validation.md:261) with (.andromeda/security-plan.md:195)
- **the script runtime's event rule ↔ API security's outbound methods**: the conflicted runtime files and `91ef65e21` add no outbound method past GET and HEAD for JS-facing fetch, (.andromeda/security-plan-detail/input-validation.md:257) with (.andromeda/security-plan.md:215)
- **the settle's load rule ↔ the failed stylesheet path**: `f6edb9701`'s new path keeps the fetched-stylesheet rule and a settle that never waits on a load in flight, (.andromeda/security-plan.md:107) with (.andromeda/security-plan.md:213)
