# Merge — `23b9b26a9` into `build/escher-0.2.0`, no commit

`git merge --no-ff --no-commit 23b9b26a9b6af5947b8f003d2c2cc768bff10b8a` at HEAD `143b6013` (2026-10-17T14:36Z):
exit 1, `CONFLICT (content)` in `Cargo.lock` and `packages/blitz-dom/src/accessibility.rs` — the two the trial tree
`3f625f43` predicted (inputs#I1). Every other file auto-merged: `Cargo.toml`, `packages/blitz-dom/src/events/keyboard.rs`
and `packages/dioxus-native-dom/src/lib.rs` (both-sides files), the rest upstream's alone. git printed `Recorded
preimage` for both conflicts (the host has `rerere` on); no recorded resolution was replayed.

Staged by the merge: `Cargo.toml` · `justfile` · `apps/bump/{Cargo.toml,src/main.rs}` (deleted) ·
`packages/blitz-dom/src/{events/keyboard.rs,layout/inline.rs,node/text.rs,util.rs}` ·
`packages/blitz-shell/src/convert_events.rs` · `packages/blitz-vibey-script/{src/dom/element.rs,src/runtime.rs,tests/preact.rs}` ·
`packages/dioxus-native-dom/src/{events.rs,keyboard_compat.rs (new),lib.rs}` ·
`tests/blitz-tests/tests/{accessibility_description.rs (new),accessibility_label.rs (new),all.rs}`.

## `packages/blitz-dom/src/accessibility.rs` — the five regions (cited by symbol; the trial's line numbers move)

| Region | Ours (HEAD) | Upstream (`23b9b26a9`) | Resolution |
|---|---|---|---|
| 1 — the `use` lines | `crate::{BaseDocument, ElementData, Node as BlitzDomNode, local_name}`; `accesskit::{Invalid, …, Toggled, …}` | `crate::{…, NodeData, NodeId as BlitzDomNodeId, local_name}`; `accesskit::{…}` without `Invalid`/`Toggled` | union: upstream's `crate` line, our `accesskit` line |
| 2 — `build_accessibility_tree`'s collections | `let mut labels` | `let mut labelled_by_nodes`, `let mut described_by_nodes` | all three, ours first |
| 3 — the visit | `label` element → `labels.push` | `aria-labelledby` holder → `labelled_by_nodes.push`; `aria-describedby` holder → `described_by_nodes.push` | all three `if`s, ours first |
| 4 — after the visit | the label → input `push_labelled_by` loop | the `aria-labelledby` loop (`clear_labelled_by`, push targets, `write_text_alternative`, computed `set_label`); the `aria-describedby` loop (`push_described_by`) | ours, then upstream's two loops as written — so `aria-labelledby` (accname 2B) outranks a native `<label>` (2D) |
| 5 — `build_accessibility_node` | the closed-set trimmed `aria-selected` · `aria-pressed` · `aria-invalid` reads | `aria-expanded` · `aria-disabled` · `aria-selected` (`parse::<bool>`) · `aria-label` (unfiltered) · `aria-description` · `aria-level` with tag/role defaults · `set_html_tag` | ours kept; upstream's `aria-expanded`, `aria-disabled`, `aria-description`, `aria-level` (+ defaults) and `set_html_tag` (last) kept as written; **dropped:** upstream's `aria-selected` read (`if let Some(aria_selected) … parse() … set_selected`) and its `aria-label` read (`if let Some(aria_label) … set_label(aria_label)`) |

Outside the regions, and unchanged: our trimmed non-empty `aria-label` read (accname 2C) in `build_accessibility_node`;
upstream's helpers `referenced_dom_ids_to_node_ids`, `BlitzDomNode::write_text_alternative`,
`BlitzDomNode::write_flat_text_content`, `FlatText`, `default_level_for_tag`, `default_level_for_role`, as upstream
wrote them. `label_bound_input_element` is still called from the builder's label loop.

The two drops change no outcome (research.md §Patterns detected): upstream's strict `parse::<bool>` decides a subset
of ours with the same value, and its unfiltered `aria-label` differs from ours only on a blank label, which our
ratified contract names nothing (a11y-plan §7) and no upstream test asserts.

Result: 0 conflict markers (`command grep -n -E '^(<<<<<<<|=======|>>>>>>>)'`: no hit), `rustfmt --edition 2024
--check` exit 0.
