
## 2026-10-13-range-input-interaction — the range input's types, its snapshot value and its checks
**Section:** §Standard Contracts → Nodes and widgets · Dioxus DOM bridge · §Existing Scopes → blitz-dom: node · blitz-tests
**Change:**
- Nodes and widgets:
  - `SpecialElementData` lists `RangeInput`, a source break for an external exhaustive match (the enum is not `#[non_exhaustive]`; none in the workspace outside blitz-dom), beside `SpecialElementType::RangeInput` (`#[non_exhaustive]`, no reader).
  - The public `RangeInputData` (`value() -> f64`) is exported from `blitz_dom::node`, not re-exported from the crate root.
  - The public `ElementData` accessors are `range_input_data()`, `range_input_data_mut()` and `range_input_value() -> Option<String>`.
  - `RangeRules`, `RangeMove` and four range helpers stay crate-internal.
- Dioxus DOM bridge: the snapshot's `value` reads the editor text, else a range input's current value `range_input_value()`, else the `value` attribute. Was: the editor text, else the `value` attribute.
- §Existing Scopes, blitz-dom node: the module list adds range input data, the private module `range`.
- §Existing Scopes, blitz-tests:
  - `range_input_interaction` (6 tests, both layout modes) is registered.
  - `stand_act_range` now asserts that the driver moves the slider by click and key, its changes named by the diff on the slider (`value`) and its display (`name`). Was: "the range input's measured reading", a pinned no-op.
**Why:** the chunk added these public symbols, the module and the check, and moved the snapshot's range reading. The `stand_act_range` restatement rides the answer-card group the founder confirmed (2026-10-13, «Подтвердить (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label).
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/
