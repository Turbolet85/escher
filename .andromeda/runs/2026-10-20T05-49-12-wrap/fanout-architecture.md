proposals:
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Scrolling and input]
    change: "Add, beside the range interaction model, since 2026-10-20-select-by-id the select interaction model for every document: a focused single-choice `select` (`select::is_single_select`) with no disabled form control (`ElementData::is_disabled`) takes a pressed `ArrowDown` / `ArrowUp` to the next / previous enabled option and `Home` / `End` to the first / last (none chosen reading `Next` as `First`, `Previous` as `Last`), a `Key::Character` with no Ctrl, Alt or Meta as typeahead — the first enabled option whose label starts with the buffered text, lowercase-compared, the buffer (`select_typeahead`) starting afresh for another select or once the document's clock has moved 1 s (`TYPEAHEAD_TIMEOUT`) — and every other key nothing (Enter and Space open nothing, a released Space activates no select) (packages/blitz-dom/src/events/keyboard.rs:108-113; packages/blitz-dom/src/events/keyboard.rs:165-200; packages/blitz-dom/src/events/keyboard.rs:248-269); a click on a select clears the typeahead and the document's text selection and focuses it through `generate_focus_events` / `set_focus_to`, a disabled select stopped by the walk's gate first (packages/blitz-dom/src/events/pointer.rs:857-870); each choice that changes the chosen option goes through the one write · mark · events · redraw path `apply_select_choice`, dispatching `input` then `change`, each carrying `select_value`, and a choice of the chosen option fires nothing (packages/blitz-dom/src/events/keyboard.rs:271-297); no popup is painted, an option is not clicked (a driver `click` on one refused `covered`) and a `multiple` select or an `optgroup`'s options are not modelled — ratified by the founder (2026-10-20, inputs#I3) «Утвердить все четыре (Рекомендую)» (F1 «Keys + typeahead»), by question dialog in the overseer session, relayed verbatim by the overseer — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md."
    sidecar: "[Scrolling and input] gains the select interaction model — arrows, Home/End, 1 s typeahead on the document's clock, a click focusing, input then change per change (2026-10-20-select-by-id)."
    rationale: "The report's Symbols / APIs (Keyboard, Pointer) land a select key arm, typeahead and a click arm, and its Expected amendments name §Established Decisions → [Scrolling and input] for the select model; the `select` pattern finds no architecture site stating it, so it is an addition. The founder ratified F1–F4 in the Addendum (inputs#I3), superseding the delegate's PROVISIONAL answers."
    basis: "packages/blitz-dom/src/events/keyboard.rs:165-200"
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Scrolling and input]
    change: "The click-focus clause (report offsets @c9361,9394) that ends «and a click on any other element still clears focus» now names the select: a click focuses a button (the button arm) and, since 2026-10-20-select-by-id, a `select` (its own arm in `handle_click`'s walk, packages/blitz-dom/src/events/pointer.rs:857-870), a disabled form control still stopping the walk first, and a click on any other element still clears focus."
    sidecar: "[Scrolling and input]'s click-focus clause names a select among the elements a click focuses (2026-10-20-select-by-id)."
    rationale: "Report Counts / qualifiers moved, pattern `click-focus`: architecture.md:86 @c9361,9394 stale — `select` added to the elements a click focuses (Symbols / APIs → Pointer)."
    basis: "packages/blitz-dom/src/events/pointer.rs:857-870"
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [DOM semantics]
    change: "Add since 2026-10-20-select-by-id the select's choice model for every document, its rules in the crate-private `node/select.rs` cited to the WHATWG `select` element: a choice is held as an option's `CHECKED` element state (so `:checked`), an option created with a `selected` attribute seeded `CHECKED` by `ElementData::new` (packages/blitz-dom/src/node/element.rs:455-461); a single-choice select is settled by HTML's selectedness setting algorithm — the last chosen option, else the first option not disabled — its options the `option` children in tree order (an `optgroup`'s not listed), an option's label its non-empty `label` attribute else its stripped and collapsed text, its value its `value` attribute when present else the text (packages/blitz-dom/src/node/select.rs:70-86; packages/blitz-dom/src/node/select.rs:117-139; packages/blitz-dom/src/node/select.rs:155-164); the mutator records every select whose children an entry point touches (`maybe_record_node`) and every select entering the document (`process_added_subtree`, since a Dioxus static template is deep-cloned and its options never pass an append), a `selected` write sets that option chosen and its select's other options not (exclusive) and a clear sets it not chosen, and `flush` settles every recorded select, firing no event (packages/blitz-dom/src/mutator.rs:1017-1020; packages/blitz-dom/src/mutator.rs:1097-1101; packages/blitz-dom/src/mutator.rs:1211-1215; packages/blitz-dom/src/mutator.rs:1220-1241); Dioxus re-writes a dynamic `selected` on each re-render, so a controlled app writes `selected` from its `input` event's value or the choice reverts; form submission sends every chosen option that is not disabled, its value (packages/blitz-dom/src/form.rs:276-290); the user-agent stylesheet gives `select` the `input, textarea` border, padding and background and `display: inline-block`, and shows its chosen option by `select:not([multiple]) > option:checked { display: inline; }` beside `option { display: none; }`, so a select lays out as a box holding its chosen option (packages/blitz-dom/assets/default.css:77-78; packages/blitz-dom/assets/default.css:84-86; packages/blitz-dom/assets/default.css:155-159) — ratified by the founder (2026-10-20, inputs#I3) «Утвердить все четыре (Рекомендую)» (F2 «Engine shows its label»), relayed verbatim by the overseer."
    sidecar: "[DOM semantics] gains the select's choice model — CHECKED as the choice, selectedness settle, mutator re-seed, form entries, the UA select box (2026-10-20-select-by-id)."
    rationale: "Report Symbols / APIs (select rules, Mutator, Seeding, Form), Schema / config (default.css) and Deviations 1, 2, 4, 6; Expected amendments name §Established Decisions → [DOM semantics] for the select model; pattern `select` holds no architecture site stating it (architecture.md:98 no-change), so it is an addition."
    basis: "packages/blitz-dom/src/node/select.rs:155-164"
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Scrolling and input] (detail dec-scrolling-and-input)
    change: "Add a bullet beside the seven direct engine changes and the document's clock (architecture.md:90): the select model — its keys and typeahead, its click focus, its choice held as `CHECKED` and shown as the select's content, its options in the accessibility tree, the `change` event and a `<label>` binding every labelable element — is a direct engine change for every document, remote pages included, ratified by the founder (2026-10-20, inputs#I3) «Утвердить все четыре (Рекомендую)», flagged upstreamable with no upstream pull request open; its merge surface, by file — blitz-dom `node/select.rs`, `document.rs`, `events/{keyboard,pointer,mod}.rs`, `mutator.rs`, `accessibility.rs`, `form.rs`, `node/element.rs`, `assets/default.css`; blitz-traits `events.rs`; blitz-vibey-script `runtime.rs` — owned by a CARRY on the Epoch 5 upstream-sync entry."
    sidecar: "The engine-change list gains the select model, founder-ratified 2026-10-20, its merge surface by file owned by the Epoch 5 upstream-sync CARRY (2026-10-20-select-by-id)."
    rationale: "Report Expected amendments: «The merge surface (upstreamable) — the wrap pins a CARRY: on the Epoch 5 upstream-sync entry naming them by file … (precedent: pattern `owner`, architecture.md:90)», and the Addendum: the pattern «An engine behaviour change for every document» is answered by the founder (inputs#I3), so the amendment writes it ratified, not PROVISIONAL."
    basis: "escher-0.2.0/chunks/2026-10-20-select-by-id/report.md"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Mutation, query and CSSOM
    change: "The `<label>` source now reads «a `<label>` bound through `label_bound_input_element` (its `for`-target by id, else its first nested element, each a labelable element as HTML lists them — `button`, an `input` whose `type` is not `hidden` (ASCII case-insensitive), `meter`, `output`, `progress`, `select`, `textarea` — through the private `is_labelable`, since 2026-10-20-select-by-id; until then an `<input>` alone)», pushed onto the bound element's `labelled_by`; the same binding moves `handle_click`'s label arm, so a click on a select's label re-dispatches to it and focuses it (packages/blitz-dom/src/document.rs:3126-3140) — ratified by the founder (2026-10-20, inputs#I3, F4 «`<label>` names it»)."
    sidecar: "Mutation, query and CSSOM: a `<label>` binds every labelable element, not an `<input>` alone (2026-10-20-select-by-id)."
    rationale: "Report Counts / qualifiers moved, pattern `label`: architecture.md:159 stale (describes `label_bound_input_element` as its for-target `<input>`, else first nested `<input>`); Symbols / APIs → BaseDocument: both arms now match HTML's labelable elements."
    basis: "packages/blitz-dom/src/document.rs:3126-3140"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Mutation, query and CSSOM
    change: "Where the builder «reads state attributes on every element … `aria-selected`: `true` / `false` sets AccessKit `selected`», add the exception since 2026-10-20-select-by-id: an `option` whose parent is a `select` writes `selected` from its chosen state (`is_chosen`), true or false, and reads no `aria-selected`; such an option stays in the tree though it computes `display: none` (`hidden`, `inert` and `visibility: hidden` still exclude it), and its non-empty `label` attribute is its `label` when it has no `aria-label`; role mapping unchanged (`ComboBox`, `ListBox` for `multiple`, `ListBoxOption`) (packages/blitz-dom/src/accessibility.rs:157-162; packages/blitz-dom/src/accessibility.rs:171-181; packages/blitz-dom/src/accessibility.rs:257-258; packages/blitz-dom/src/accessibility.rs:260-273) — ratified by the founder (2026-10-20, inputs#I3, F3 «Options as children»)."
    sidecar: "Mutation, query and CSSOM: a select's options stay in the accessibility tree under display: none, selected from their chosen state, named by their label attribute (2026-10-20-select-by-id)."
    rationale: "Report Symbols / APIs → Accessibility: a select's option is kept though `display: none`, writes `selected` from `is_chosen` and reads no `aria-selected`, its `label` attribute its label; the builder's «every element» `aria-selected` read at architecture.md:159 no longer holds for that option."
    basis: "packages/blitz-dom/src/accessibility.rs:171-181"
  - detector: D-arch-resources
    severity: warning
    section: §Established Decisions → [Scrolling and input]
    change: "Where the accessibility builder is said to write AccessKit's `selected` «from `aria-selected` … read by value», add «— a select's option excepted, whose `selected` is its chosen state (§Standard Contracts → Mutation, query and CSSOM)»."
    sidecar: "[Scrolling and input]'s builder clause notes a select's option reads `selected` from its chosen state (2026-10-20-select-by-id)."
    rationale: "Second occurrence of the builder's `aria-selected`-only claim in architecture.md:86; the report's Symbols / APIs → Accessibility says a select's option reads no `aria-selected`."
    basis: "packages/blitz-dom/src/accessibility.rs:171-181"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Dioxus DOM bridge
    change: "Where the snapshot's `selected` · `pressed` · `invalid` are «what the engine's builder writes from `aria-selected`, `aria-pressed` (or `aria-checked` on a checkable role) and `aria-invalid`», add that a select's option's `selected` is its chosen state, so a choice names the old and the new option with `selected` in `differs`."
    sidecar: "Dioxus DOM bridge: a select's option reads `selected` from its chosen state, not `aria-selected` (2026-10-20-select-by-id)."
    rationale: "Third occurrence of the builder's `aria-selected`-only claim (architecture.md:205); report Outcome: `type --id fx-pick --text Cherry` diffs `fx-banana` / `fx-cherry` with `selected`."
    basis: "tests/blitz-tests/tests/stand_act_select.rs:134-167"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Events
    change: "Add since 2026-10-20-select-by-id `DomEventKind::Change` (after `Input`, so every later kind's discriminant moves up by one, the Dioxus bridge's 64-slot `event_handler_counts` kept aligned), `DomEventKind::from_str` mapping `change`, and `DomEventData::Change(BlitzInputEvent)` — name `change`, not cancelable, bubbling (packages/blitz-traits/src/events.rs:150; packages/blitz-traits/src/events.rs:202; packages/blitz-traits/src/events.rs:250-251; packages/blitz-traits/src/events.rs:307; packages/blitz-traits/src/events.rs:354; packages/blitz-traits/src/events.rs:402; packages/blitz-traits/src/events.rs:449) — with no UI event mapped and no default action (packages/blitz-dom/src/events/mod.rs:93; packages/blitz-dom/src/events/mod.rs:246-248); the engine dispatches it only from a select's choice, after `input`, through the crate-internal `BaseDocument::apply_select_choice` (packages/blitz-dom/src/events/keyboard.rs:271-297), a checkbox, radio or text input firing no engine `change` (vibey-script's own synthesis unchanged) — ratified by the founder (2026-10-20, inputs#I3, F4 «`change` event»)."
    sidecar: "Events: a new DomEventKind::Change / DomEventData::Change, dispatched after input on a select's choice (2026-10-20-select-by-id)."
    rationale: "Report Symbols / APIs → Events and Keyboard; pattern `change-event`: architecture.md:160 re-derived (`DomEventKind::from_str`'s list gains `change`); Expected amendments name §Standard Contracts → Events (`Change`). A new event kind is a resource D-arch-resources requires registered."
    basis: "packages/blitz-traits/src/events.rs:250-251"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Document core
    change: "Add the public read-only `BaseDocument::select_value(&self, select: NodeId) -> Option<String>` since 2026-10-20-select-by-id — HTML's `select.value`, the chosen option's value, `None` with none chosen or for a node that is no select; its non-test callers dioxus-native-dom's snapshot `value` and `apply_select_choice` (packages/blitz-dom/src/document.rs:717-728); and the crate-private `settle_select_choice`, `set_select_choice` and `set_option_chosen` (the `CHECKED` write through `snapshot_node_and` for an in-document option) beside the field `select_typeahead: Option<select::Typeahead>`, `None` at construction (packages/blitz-dom/src/document.rs:338-339; packages/blitz-dom/src/document.rs:730-775)."
    sidecar: "Document core registers BaseDocument::select_value, a new public reader (2026-10-20-select-by-id)."
    rationale: "Report Symbols / APIs → BaseDocument: new public `select_value`; a new public API is a contract D-arch-resources requires registered in §Standard Contracts."
    basis: "packages/blitz-dom/src/document.rs:717-728"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Nodes and widgets
    change: "Add the crate-private module `node::select` (`pub(crate) mod select`, packages/blitz-dom/src/node/mod.rs:10) since 2026-10-20-select-by-id — `OptionReading`, `SelectStep` (`Next` · `Previous` · `First` · `Last`), `Typeahead` and `TYPEAHEAD_TIMEOUT` (1 s), with the readers and rules `is_single_select`, `is_select_option`, `options`, `is_chosen`, `chosen_option`, `readings`, `option_label`, `option_value`, `settled`, `stepped`, `matched` (packages/blitz-dom/src/node/select.rs:17-193) — no public type, `SpecialElementData` unchanged (a select holds no special data)."
    sidecar: "Nodes and widgets registers the crate-private node::select module (2026-10-20-select-by-id)."
    rationale: "Report Crates / modules: module added `blitz-dom::node::select` (`pub(crate)`); Expected amendments name §Standard Contracts → Nodes and widgets."
    basis: "packages/blitz-dom/src/node/select.rs:17-193"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Dioxus DOM bridge
    change: "Add since 2026-10-20-select-by-id: the bridge hands `Input | Change` to the app as `NativeFormData { value }`, so an `onchange` handler fires (packages/dioxus-native-dom/src/dioxus_document.rs:377-382); the snapshot's `value` reader takes `(doc, node_id, element)` and reads a `select` through `BaseDocument::select_value` (packages/dioxus-native-dom/src/snapshot.rs:253-259); and an option whose parent is a `select` reads its select's client rect as its `bounds` (the private `select_parent`, packages/dioxus-native-dom/src/snapshot.rs:144-152; packages/dioxus-native-dom/src/snapshot.rs:241-250), so a select's options are its children in the snapshot, named by their labels — no role or name mapped there."
    sidecar: "Dioxus DOM bridge: onchange fires, a select's snapshot value is its chosen option's value, an option reads its select's bounds (2026-10-20-select-by-id)."
    rationale: "Report Symbols / APIs → Dioxus bridge and Snapshot; Expected amendments name §Standard Contracts → Dioxus DOM bridge (a select's value, an option's bounds). The bridge's value reader at architecture.md:205 lists editor text, a range input's current value and a textarea or text-entry input's attribute, not a select."
    basis: "packages/dioxus-native-dom/src/snapshot.rs:253-259"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session
    change: "«`type`'s help still says it replaces what the element holds and that an empty text clears it» (report offset @c21636) gains «and, since 2026-10-20-select-by-id, that on a select the text chooses the first option whose label starts with it» (packages/escher-driver/src/schema.rs:208-209; its pin packages/escher-driver/src/schema_tests.rs:77-78) — no verb, argument, key name, result field, refusal cause, session error or exit added, `describe`'s count row unmoved."
    sidecar: "Driver session: `type`'s help gains its select sentence (2026-10-20-select-by-id)."
    rationale: "Report Symbols / APIs → Driver and pattern `type-help`: architecture.md:192 @c21636 re-derived (the «`type`'s help still says …» clause gains the sentence)."
    basis: "packages/escher-driver/src/schema.rs:208-209"
  - detector: D-arch-resources
    severity: warning
    section: §Cross-cutting Patterns → Invalidation and state integrity
    change: "The list of what marks the changed set gains, since 2026-10-20-select-by-id, `BaseDocument::apply_select_choice`, which marks a select where a key or typeahead changed its chosen option and nothing for a choice of the chosen option (packages/blitz-dom/src/events/keyboard.rs:271-297), the option's `CHECKED` write going through `snapshot_node_and` (packages/blitz-dom/src/document.rs:755-775); a mutator `selected` write settles at `flush` and fires no event."
    sidecar: "Invalidation and state integrity: a select's changed choice marks the changed set (2026-10-20-select-by-id)."
    rationale: "Report Symbols / APIs → Keyboard: where a choice changed, the select joins `changed_nodes`; Outcome (arch): a choice marks the changed set and a no-change key marks nothing. The section's mark list names `apply_range_move` as the last marker and not this path."
    basis: "packages/blitz-dom/src/events/keyboard.rs:271-297"
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → Tests
    change: "«the fourteen driver-action checks `stand_act_*`» becomes «the fifteen driver-action checks `stand_act_*`» (`stand_act_select` the fifteenth, since 2026-10-20-select-by-id)."
    sidecar: "Conventions → Tests: the driver-action checks fourteen → fifteen (2026-10-20-select-by-id)."
    rationale: "Report Counts / qualifiers moved, pattern `stand-act`: architecture.md:137 stale (fourteen → fifteen, `stand_act_select` the fifteenth)."
    basis: "tests/blitz-tests/tests/stand_act_select.rs:79-132"
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes
    change: "In the blitz-tests row, after «a fourteenth, `stand_act_transformed`», add «and, since 2026-10-20-select-by-id, a fifteenth, `stand_act_select` — a select driven by id: its snapshot at boot (`ComboBox`, name, value, its options as children), `type` with a label choosing that option, a click then a key, the Tab walk skipping a disabled select, a disabled select and an unchosen option refused, 5 tests in both layout modes (tests/blitz-tests/tests/stand_act_select.rs:79-256)» and the non-`stand_*` `select_interaction` (the select's box, seeding, keys, typeahead kept for 1 s of the document's clock, a disabled select, re-seeding, 7 tests in both modes, tests/blitz-tests/tests/select_interaction.rs:113-343); «by the fourteen `stand_act_*`» in the `session_common` reader list becomes «by the fifteen `stand_act_*`»."
    sidecar: "Existing Scopes: blitz-tests gains stand_act_select and select_interaction; the stand_act_* readers fourteen → fifteen (2026-10-20-select-by-id)."
    rationale: "Report Counts / qualifiers moved, pattern `stand-act`: architecture.md:354 stale; the two new test targets (report Files, Harness / gate surface) are not in the row."
    basis: "tests/blitz-tests/tests/stand_act_select.rs:79-256"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes
    change: "«`tests/session_common/mod.rs`, by 27» becomes «by 28» and «`tests/fixtures/mod.rs` … by 21 (20 `stand_*` files and `font_face_sniffed_source`)» becomes «by 23 (21 `stand_*` files, `font_face_sniffed_source` and `select_interaction`)», measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md; «`mod common;` by 21 checks» unchanged."
    sidecar: "Existing Scopes: session_common readers 27 → 28, fixtures readers 21 → 23 (2026-10-20-select-by-id)."
    rationale: "Report Counts / qualifiers moved, patterns `session-common` and `fixtures`: architecture.md:363 stale («by 27», «by 21 (20 `stand_*` files and `font_face_sniffed_source`)»); the `tests/common/mod.rs` count is unmoved — neither new check declares `mod common;`."
    basis: "tests/blitz-tests/tests/stand_act_select.rs:48-50"
note: "D-arch-decisions found no new library, runtime or contradicted locked decision (Dependencies: none); its proposals are the select model the report's Expected amendments route to [Scrolling and input] and [DOM semantics]. Count proposals ride D-arch-resources for want of a count detector. Pre-existing, not this chunk's: architecture.md:354's session_common reader list ends «twenty-five readers» while :363 reads 27 (now 28) — its enumeration omits readers; reconcile at apply rather than adding one to twenty-five. Citation shifts into keyboard.rs, runtime.rs, default.css and accessibility.rs past the added ranges are the P2 citation sweep's."
