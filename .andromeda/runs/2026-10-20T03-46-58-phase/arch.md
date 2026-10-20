# arch extract

## Relevance
relevant — the chunk lands in blitz-dom (a possible `select` interaction model), dioxus-native-dom (the snapshot's `value` reading) and possibly escher-driver (how a choice is made), and is proven on a blitz-tests fixture.

## Constraints
- A `select` interaction model that changes engine behaviour for every document stands in the same class as the range interaction model and keyboard activation: arch §Established Decisions → [Scrolling and input] records each direct engine change for every document with the founder's ratification, flagged upstreamable against upstream `main` with its merge surface by file. A new one needs the same record, and the ratification comes before it lands. Whether the engine already gives a `select` any behaviour beyond its role and its `disabled` gate is research's question.
- `disabled` has one reader. Per arch §Established Decisions → [DOM semantics], `ElementData::is_disabled` (`button` · `input` · `select` · `textarea`, by the attribute's presence) is what focusability, the pointer gates, `ClickTarget::Disabled`, `handle_click`'s walk gate and the activation keys read. Any key, pointer or click gate a `select` gains must read that reader, not one of its own. The Tab walk (`focus_next_node` / `focus_prev_node`) moves with focusability.
- The driver's schema is closed. Arch §Standard Contracts → Driver session requires `VERBS` to hold eleven verbs in three levels, six `ArgKind`s with bounds, `KEY_NAMES` to hold twelve names, and every call to be validated before anything runs. Each past addition (`scroll`, the session verbs, `help`, `screenshot`) was recorded as a widening of the validated verb set and ratified by the founder. Choosing an option through a new verb, argument or key name is therefore a widening. Choosing through the existing `click` / `press` / `type` is not.
- The snapshot's `value` follows a fixed order. Arch §Standard Contracts → Dioxus DOM bridge requires it: the editor text, else a range input's `range_input_value()`, else the `value` attribute of a `textarea` or a text-entry `input`, with a password or file input masked as `MASKED_VALUE`. No field may hold a `NodeId`, `ElementId`, `ScopeId` or AccessKit tree id. Any reading of a `select`'s chosen value is a new branch in that order. Whether the builder reads a `select`'s value today, and whether `selected` moves on its options, is research's question.
- A chosen-option change must reach the changed set. Per arch §Cross-cutting Patterns → Invalidation and state integrity, the changed set is written only at its listed kinds of site: the mutator's flag sites, `snapshot_node_and` on `FOCUS` / `CHECKED`, the text-input `Input` arm, and `apply_range_move`. It is not written by node creation, hover or layout. A select's change has to mark through one of those sites or a new one recorded there, and mark nothing for a move that changes nothing, so that `has_changes` and the driver's diff stay truthful.
- Incremental and non-incremental layout must stay identical. Per arch §Design Philosophy → One incremental pipeline, safe identities, both modes run one pipeline. Per arch §Conventions → Tests, scenarios run `for incremental in [false, true]`, and the fixture proof runs in both.
- Every DOM write goes through `DocumentMutator`. Per arch §Conventions → DOM API patterns, it flushes on Drop, and the request is to extend the mutator rather than reach through `DocumentMutator::doc`. A select's option or value write from the app (a Dioxus re-render) follows the same path, like the range input's `value` re-seed in `mutator.rs`.

## Patterns to follow
- The range input is the template for an interaction model. Per arch §Standard Contracts → Nodes and widgets and arch §Established Decisions → [Scrolling and input], it has these parts:
  - a `SpecialElementData` variant holding the current value apart from the attribute (`RangeInputData`);
  - its rules in their own `node/*.rs` module, written against the WHATWG state;
  - key arms in `events/keyboard.rs`, gated on focus and not disabled;
  - one write · mark · `Input` · redraw path (`apply_range_move`);
  - a `value` write that re-seeds it;
  - an `input` event `DomEventData::Input(BlitzInputEvent { value })` that the Dioxus bridge carries to `oninput` unchanged.

  `SpecialElementData` is not `#[non_exhaustive]`, so a new variant is a source break for an external exhaustive match. Record it as `RangeInput` was recorded.
- The keyboard activation pattern applies if a key opens or commits a choice. Per arch §Standard Contracts → Events, a default action is dispatched through `dispatch_activation_click` / the event driver's queue, so the app's handlers run before the default action. `handle_click`, `handle_pointerdown` and `handle_key_or_input_event` keep their signatures, with arm bodies as private named functions.
- Behave like a browser, and cite the spec. Per arch §Design Philosophy, web behaviour is written against named specifications and engines. A `select`'s option-choice rules should cite the WHATWG `select` element, and its role mapping HTML-AAM, as the builder's role mapping does.
- Every actionable element carries an author key. Per arch §Standard Contracts → Dioxus DOM bridge, `ComboBox`, `ListBox` and `ListBoxOption` are already in `unkeyed_actionable`'s closed interactive-role list, so the fixture's `select` (and any options an agent targets) carry author keys.
- Fixture pages: per arch §Conventions → Tests, `tests/blitz-tests/tests/fixtures/mod.rs` holds the pages that more than one check reads, and a page with one reader stays in its check's file.

## Anti-patterns to avoid
- Reaching through `DocumentMutator::doc` instead of extending the mutator (per arch §Conventions → DOM API patterns).
- Letting a `NodeId` cross into a snapshot field, a driver answer or a recorded step. Per arch §Established Decisions → [Driver session], no `NodeId` crosses the session's carriers.
- Silently adding a crate, env var, listener or path. Per arch §Occupied Resources, each is registered there before it lands.

## Contract bindings
- arch ↔ security: a select model read from any document's markup (remote pages included), and any new verb, argument or key name, ties arch §Standard Contracts → Driver session to security-plan §Input Validation (closed sets, no text carried out, validated before anything runs).
- arch ↔ a11y: focusability and Tab order (arch §Established Decisions → [DOM semantics]) are the a11y plan's SC 2.1.1 / 2.4.3 surface. The `a11y` CI leg's `stand_keyboard` Tab tables must not move, since the stand gains no `select`.
- arch ↔ tests: the fixture module's contents are test-plan §3's (Crate-local test helpers), cited from arch §Conventions → Tests.
- arch ↔ obs: the `command` span's eight fields are fixed words and counts (arch §Standard Contracts → Driver session). A choice made through the driver adds no field.

## Acceptance criteria contributions
- On a fixture holding a single-choice `select` with an author key, a driver call addressed by the select's stable id changes the chosen option. The snapshot then reads the select with role `ComboBox`, its accessible name and the chosen value, and the step's diff names its `value`. All of this holds in both layout modes (per arch §Standard Contracts → Dioxus DOM bridge; arch §Conventions → Tests).
- `VERBS` still lists eleven verbs and `KEY_NAMES` twelve names, unless the founder's ratification of a widening is recorded (per arch §Standard Contracts → Driver session).
- A `select` carrying `disabled` refuses the choice through every gate that reads `ElementData::is_disabled`, and the Tab sequence of the fixture page is identical with and without the select's interaction model (per arch §Established Decisions → [DOM semantics]).
- A choice that changes the chosen option leaves `has_changes` true until drained. A choice that changes nothing marks nothing (per arch §Cross-cutting Patterns → Invalidation and state integrity).
