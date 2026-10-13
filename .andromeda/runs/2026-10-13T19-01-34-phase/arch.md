# arch extract

## Relevance
relevant — this chunk changes engine behaviour for every document with a range input: blitz-dom event handling, possibly layout construction, and the Dioxus bridge's `input` value. The driver is reached only through its existing verbs.

## Constraints
- Workspace placement: the interaction model belongs in blitz-dom's events module (submodules driver, focus, ime, keyboard, pointer), and in its layout `construct` only if range state is held there. The value reaches the app through dioxus-native-dom's event converter. No new workspace crate. Per architecture §Existing Scopes (rows "blitz-dom: events", "blitz-dom: layout", "dioxus-native-dom").
- A behaviour change for every document must be recorded as a direct engine change, like the four already listed: the founder's ratification, an upstreamable flag against the upstream ref, the merge-surface files, and the measured report. The founder's ruling «Пусть будет а» is quoted verbatim. Architecture §Established Decisions → [Scrolling and input] requires this record. The wrap writes it, and the scope names this section as the one the wrap amends.
- Construction special-cases only text inputs (a Parley editor) and checkbox/radio (checkbox data); `range` is not in the list. Holding range state in layout means a new `SpecialElementData` variant, which changes a public contract. Choosing between that and reading/writing the `value` attribute is a P4 fork. Per architecture §Established Decisions → [Layout] and §Standard Contracts → Nodes and widgets. Whether the code already handles range anywhere is research's question.
- Keyboard events target the focused node, and pointer events target the hovered node (falling back to the root). A default action runs after handlers unless cancelled, and events are queued in order. The step-by-key and set-by-click must fit this dispatch model. Per architecture §Standard Contracts → Events and §Cross-cutting Patterns → Event processing.
- The driver surface stays closed. No new verb: `VERBS` stays eleven. No new key name: `KEY_NAMES` already holds `arrow-left` · `arrow-right` · `arrow-up` · `arrow-down` · `home` · `end`. No new refusal cause and no new `SessionError`. A driver `click` lands at the centre of the snapshot's `bounds`, and `press` acts on what holds focus. Per architecture §Standard Contracts → Driver session.
- `disabled` is read two ways: focusability parses it as a boolean, while the pointer click target and `:disabled` key on the attribute's presence. A range interaction must not add a third reading. Per architecture §Established Decisions → [DOM semantics].
- Incremental and non-incremental layout must stay identical after any sequence of mutations, so a range value change must hold in both modes. Per architecture §Design Philosophy ("One incremental pipeline, safe identities").

## Patterns to follow
- Use the existing checkbox/radio path as the model: per-type data at construction, a pointer default action that toggles, and the `CHECKED` state marking the changed set. A value write through the mutator's `set_attribute` on an in-document node also marks the changed set, which the snapshot diff relies on. Per architecture §Established Decisions → [Layout] and §Cross-cutting Patterns → Invalidation and state integrity.
- Event-producing functions take a `dispatch_event` callback, as in `events/keyboard.rs` and `events/focus.rs`. A synthesized `input` event should be built the same way. Per architecture §Conventions → DOM API patterns.
- Follow "Behave like a browser, and cite the spec": write the step, clamp and snap rules against the HTML range-state value sanitization, with the spec URL next to the implementation. Per architecture §Design Philosophy and §Conventions → Documentation.
- Tests: one integration file per behaviour, opening with a `//!` doc. Run `for incremental in [false, true]`, and first assert that the fixture produces the condition under test. Stand checks that hold a `Session` share `tests/session_common/mod.rs`. `stand_act_range` is rewritten, not deleted. Per architecture §Conventions → Tests.
- Engine tracing is gated per call site with `#[cfg(feature = "tracing")]` and a no-op fallback. Per architecture §Conventions → Feature gating.

## Anti-patterns to avoid
- Reaching through `DocumentMutator::doc` instead of extending the mutator for the value write. Per architecture §Conventions → DOM API patterns.
- Indexing a `NodeId` that may be stale; a stale index panics. Use `get` / `contains_key` for the focused or hovered node. Per architecture §Standard Contracts → Scrolling, selection, tree.
- Logging a range value or any element content from an engine path, or an ungated `println!`. Per architecture §Conventions → Feature gating and §Cross-cutting Patterns → Logging and timing.

## Contract bindings
- arch §Established Decisions → [Scrolling and input] ↔ security-plan §Input Validation: this is a boundary widening for every document, the browser's remote pages included. Security states it, and arch records it as a direct engine change at the wrap.
- arch §Standard Contracts → Dioxus DOM bridge (event converter for form and keyboard events) ↔ the stand Timer's `oninput`, which parses `evt.value()` (§Standard Contracts → Headless stand). Whether the converter carries a range element's new value into the form data is research's question.
- arch §Standard Contracts → Dioxus DOM bridge (snapshot `value`: "the editor text else the `value` attribute of a `textarea` or a text-entry `input`") ↔ tests and the driver's diff. Whether a range input's `value` is read by that clause or elsewhere is research's question; the measured starting point read `15`.
- arch §Standard Contracts → Mutation, query and CSSOM (accessibility builder, role Slider) ↔ a11y: the accessibility node's numeric value, and AccessKit Increment/Decrement/SetValue actions, which stay out unless P3 finds them needed.
- arch §Established Decisions → [Driver session] ↔ the driver schema: if a verb's help text changes to say what a range input does, it stays fixed `'static` table text riding the ratified carriers, with no new carrier, field or closed set.

## Acceptance criteria contributions
- (arch) Arrow keys step a focused range input, a click sets it by position, and an `input` event reaches the app's handler. All three read the same in incremental and non-incremental layout. Per architecture §Design Philosophy ("One incremental pipeline, safe identities").
- (arch) The driver schema is unchanged: `VERBS` 11, `KEY_NAMES` 12, `CAUSES` 9, `SessionError` 13. Per architecture §Standard Contracts → Driver session.
- (arch) No new workspace crate, listener, port or env var, and no new package in `Cargo.lock`. Per architecture §Occupied Resources (Names, Network ports and listeners, Environment variables).
- (arch) The wrap records the range interaction as a direct engine change for every document. The record carries the founder's ratification, the upstreamable flag against the upstream ref, the merge-surface files and the measured report. Per architecture §Established Decisions → [Scrolling and input].
