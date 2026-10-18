proposals:
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Unsupported features]
    change: 'Drop the clause «`inert` is not implemented — a truthy `inert` takes no focusability away … `stand_keyboard::the_engine_honours_no_inert` staying its pin»: since 2026-10-18-engine-debts `inert` is honoured (stated under [DOM semantics]), its residuals line discharged, the pin renamed `stand_keyboard::an_inert_subtree_is_skipped_by_tab`.'
    sidecar: '[Unsupported features]: `inert` leaves the list — honoured since 2026-10-18-engine-debts (F1, PROVISIONAL on the delegate''s word).'
    rationale: 'Report Changes → Symbols / APIs, F1 «`inert` honoured» (node.rs:558-575, :1401-1410; accessibility.rs:243, :245-256) contradicts the locked [Unsupported features] clause; Counts → renamed tests lists `the_engine_honours_no_inert` → `an_inert_subtree_is_skipped_by_tab`; the sweep marks architecture.md:85 `inert` stale.'
    basis: .andromeda/architecture.md:85
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [DOM semantics]
    change: 'Add: since 2026-10-18-engine-debts `inert` is honoured by presence on the element or an ancestor element — `Node::is_focussable` also reads false through the new `pub(crate) Node::is_inert` (an ancestor walk; the cached `ElementData::is_focussable` stays per element, no mutator hook), so the Tab walk `focus_next_node` / `focus_prev_node`, autofocus and `DioxusDocument::unkeyed_actionable` skip it; `hit_inner` returns `None` at an element carrying `inert` before any other test; `is_hidden_from_accessibility_tree` reads true for it, so the subtree leaves the accessibility tree and the snapshot (packages/blitz-dom/src/node/node.rs:558-575; packages/blitz-dom/src/node/node.rs:1401-1410; packages/blitz-dom/src/accessibility.rs:243; packages/blitz-dom/src/accessibility.rs:245-256); a known limit: HTML''s focus fixup is not built — a node that holds focus when it becomes inert keeps it until focus moves (pinned by `inert_subtree::removing_inert_restores_focus_hit_and_tree`, owned by a new `.andromeda/residuals.md` line, target next); a direct engine change for every document, remote pages included, PROVISIONAL on the operator''s word as the founder''s delegate (inputs#I2, F1 A), the founder''s own word pending; the bridge `mutation_writer.rs` untouched, byte-identical to upstream at `23b9b26a9` — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md.'
    sidecar: '[DOM semantics]: `inert` honoured (focus, hit, accessibility tree) since 2026-10-18-engine-debts, PROVISIONAL; the focus-fixup limit stated.'
    rationale: 'Report Changes → Symbols / APIs, F1, and Decisions & corrections (card 2 row 4, the focus-fixup residuals line); Expected amendments name [DOM semantics] for the seven PROVISIONAL engine changes; the report notes the focus-fixup limit has 0 master rows.'
    basis: packages/blitz-dom/src/node/node.rs:558-575
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [DOM semantics]
    change: 'Restate the `disabled` read: since 2026-10-18-engine-debts the new `pub fn ElementData::is_disabled` — `can_be_disabled()` (`button` · `input` · `select` · `textarea`) and a `disabled` attribute by presence (packages/blitz-dom/src/node/element.rs:490-494) — is what `flush_is_focussable`, the pointer-move gate, `ClickTarget::Disabled`, `handle_click`''s walk gate and the Enter / Space activation gates read (packages/blitz-dom/src/node/element.rs:709; packages/blitz-dom/src/events/pointer.rs:330; packages/blitz-dom/src/events/pointer.rs:463; packages/blitz-dom/src/events/pointer.rs:764; packages/blitz-dom/src/events/keyboard.rs:163; packages/blitz-dom/src/events/keyboard.rs:180), so «a bare, a `"false"` and a `"true"` `disabled` on any element remove focusability» narrows to the four form controls, a `<div disabled tabindex=0>` takes focus, and the «known limit … its narrowing … owned by a `.andromeda/residuals.md` line … on the founder''s C15» clause is retired, `stand_keyboard`''s pin now `a_disabled_attribute_skips_a_form_control_and_not_a_div`; PROVISIONAL on the operator''s word as the founder''s delegate (inputs#I2, F7 A).'
    sidecar: '[DOM semantics]: `disabled` read on form controls only through `ElementData::is_disabled` since 2026-10-18-engine-debts (F7, PROVISIONAL); the `<div disabled tabindex=0>` limit retired.'
    rationale: 'Report Changes → Symbols / APIs, F7 (element.rs:490-494, :709; pointer.rs:330, :463, :764; keyboard.rs:163, :180, with its consequences) contradicts the locked [DOM semantics] «on any element» read; Counts → renamed tests; the sweep marks architecture.md:90 `disabled-any` stale (@c334/@c1506).'
    basis: .andromeda/architecture.md:90
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Scrolling and input]
    change: 'In the keyboard-activation and button-focus entry (the sixth to the eighth engine changes), the gates «carries no `disabled` attribute (presence)», «with no `disabled` attribute», «a `disabled` attribute still stops the walk first» and «focusability reads `disabled` by presence ([DOM semantics])» now read `ElementData::is_disabled` — form controls only, since 2026-10-18-engine-debts, PROVISIONAL — so a `<summary disabled>` or an `<a href disabled>` takes Enter / Space activation and a click, and a disabled ancestor `div` no longer stops `handle_click`''s walk; the range input''s key gate stays on its `input` by presence.'
    sidecar: '[Scrolling and input]: the activation and click gates read `ElementData::is_disabled` (F7) since 2026-10-18-engine-debts.'
    rationale: 'Report Changes → Symbols / APIs, F7: readers `activates_on_enter` / `activates_on_space` (keyboard.rs:163, :180) and `handle_click`''s walk gate (pointer.rs:764), «the range-input key gate stays on its `input` as before», and the three consequences; the sweep marks architecture.md:86 `disabled-any` stale (@c9148).'
    basis: .andromeda/architecture.md:86
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Scrolling and input]
    change: 'In the bounds-and-hit entry replace «and not at a `contain: paint` box whose `overflow` is `visible`» with: and, since 2026-10-18-engine-debts, at a box whose computed `contain` includes paint, on a box paint applies it to (not an inline flow box, not an internal table box other than a cell — blitz-paint''s own rule), the root still excepted, the flag renamed `clips_to_padding_box`, so hit and paint agree (packages/blitz-dom/src/node/node.rs:1501-1516) — a direct engine change for every document, PROVISIONAL on the operator''s word as the founder''s delegate (inputs#I2, F5 A); the «stop at paint''s overflow half» reach of the 2026-10-10 ruling widens with it.'
    sidecar: '[Scrolling and input]: the hit stops at a `contain: paint` box''s padding box since 2026-10-18-engine-debts (F5, PROVISIONAL).'
    rationale: 'Report Changes → Symbols / APIs, F5 «`hit_inner`''s padding-box stop … also holds for a box whose computed `contain` includes `Contain::PAINT`»; the sweep marks architecture.md:86 `contain-paint` stale (@c1607).'
    basis: .andromeda/architecture.md:86
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Scrolling and input]
    change: 'In the bounds-and-hit entry replace «the second reader `Node::absolute_position` measured and left as it reads» with: since 2026-10-18-engine-debts `Node::absolute_position` adds the node''s own location and subtracts only its containing blocks'' scroll, through the new private `position_in_containing_block`, and `scroll_into_view` passes `(0.0, 0.0)` in place of its add-back (packages/blitz-dom/src/node/node.rs:1717-1738; packages/blitz-dom/src/scrolling.rs:719), every caller moving with it (`handle_dom_event`, `Node::focus`''s IME area, `synthetic_click_event_data`, the harness''s `layout_rect_of` / `write_node`) — a direct engine change for every document, PROVISIONAL on the operator''s word as the founder''s delegate (inputs#I2, F4 A).'
    sidecar: '[Scrolling and input]: `Node::absolute_position` reads a self-scrolled box where it stands since 2026-10-18-engine-debts (F4, PROVISIONAL).'
    rationale: 'Report Changes → Symbols / APIs, F4 and Outcome «(arch · F4) met — (40, 50) at every scroll offset»; the sweep marks architecture.md:86 `abs-position` stale (@c2120).'
    basis: .andromeda/architecture.md:86
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Scrolling and input]
    change: 'In the third direct engine change (the builder''s state reads «from `aria-selected`, `aria-pressed` and `aria-invalid`, read by value»), add: since 2026-10-18-engine-debts `toggled` has one source per element — a native `input` whose `type` is exactly `checkbox` or `radio` (private `is_native_checkable`) reads no author toggle at all, a role in the private `is_checkable` set (`CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio`) writes it from `aria-checked` (`true` · `false` · `mixed`, trimmed, exact), every other role from `aria-pressed` as before, and `role_from_name` maps `"switch"` to `Role::Switch` (packages/blitz-dom/src/accessibility.rs:171-184; packages/blitz-dom/src/accessibility.rs:330; packages/blitz-dom/src/accessibility.rs:378-398) — PROVISIONAL on the operator''s word as the founder''s delegate (inputs#I2 F2 A; the `switch` mapping inputs#I3; the native exclusion the operator''s two corrections, inputs#I5, inputs#I6, after HTML-AAM / ARIA in HTML).'
    sidecar: '[Scrolling and input]: the builder''s toggle read per role (`aria-checked` on five checkable roles, none on a native checkbox or radio, `role="switch"` mapped) since 2026-10-18-engine-debts (F2, PROVISIONAL).'
    rationale: 'Report Changes → Symbols / APIs, F2; the decision states the builder''s three reads, which F2 changes; the sweep marks the `aria-checked` fact stale across architecture (architecture.md:121, :262) and the report states the `switch` mapping and the native exclusion share that pattern.'
    basis: packages/blitz-dom/src/accessibility.rs:171-184
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Scrolling and input]
    change: 'Add after the sixth-to-eighth entry: seven more direct engine changes for every document since 2026-10-18-engine-debts — `inert` honoured and `disabled` on form controls ([DOM semantics]), the toggle read, the hit at `contain: paint` and `Node::absolute_position` (above), and two here: a click in a password `input` places the caret by its bullets — `ClickTarget::TextInput` gains a `password` flag and the private `click_password_input` lays out one U+2022 per character through `BaseDocument::layout_as_text_input`, scrolls that layout as paint scrolls it and moves the real editor to the byte offset of the character under the point (single click `move_to_byte`, shift-click `extend_selection_to_byte`, two or more clicks `select_all`), every other text input keeping `move_to_point` (packages/blitz-dom/src/events/pointer.rs:586-652); and `get_client_bounding_rect` / `node_client_rects` read an element where its transforms move it (§Standard Contracts → Scrolling, selection, tree) — each PROVISIONAL on the operator''s word as the founder''s delegate (inputs#I2, F6 A · F8 A), the batch listed for the founder''s own word; F3 B (`aria-activedescendant` unread, pinned) and F9 A (`covered` on a box with no area, stated) change no engine; flagged upstreamable against `23b9b26a9` with no upstream pull request, the seven merge surfaces in escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md, owned by a CARRY on «Upstream sync ahead of determinism and traces».'
    sidecar: '[Scrolling and input]: seven PROVISIONAL engine changes of 2026-10-18-engine-debts counted, the password click caret and the transformed client rect stated, their merge surface owned by a CARRY.'
    rationale: 'Report Changes → Symbols / APIs F6 and F8, F3 B / F9 A «no engine change»; Decisions & corrections (the nine delegate answers, PROVISIONAL; card 2 row 1, the merge-surface CARRY); Counts → merge-surface «the seven new surfaces are additions»; Expected amendments name [Scrolling and input] for the seven changes.'
    basis: packages/blitz-dom/src/events/pointer.rs:586-652
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Mutation, query and CSSOM
    change: 'In the builder''s state reads replace «whatever its role and with no role consulted» (for `aria-pressed`) and «`aria-checked` is not read and nothing else in the file writes `toggled`» with the per-element toggle source of 2026-10-18-engine-debts — a native `checkbox` / `radio` `input` none, the five checkable roles `aria-checked` (`true` · `false` · `mixed`), every other role `aria-pressed`, `role="switch"` mapping to `Role::Switch` (packages/blitz-dom/src/accessibility.rs:171-184; packages/blitz-dom/src/accessibility.rs:330; packages/blitz-dom/src/accessibility.rs:378-398); add the measured absence that the builder writes no `toggled` from a native input''s own checked state (no `checkbox_input_checked()` read), so the adapter''s tree never states a native checkbox checked while the snapshot''s `checked` does — owned by a new `.andromeda/residuals.md` line (target: next); re-point the check to `accessibility_states`, 5 tests, its fourth rewritten (tests/blitz-tests/tests/accessibility_states.rs:245-299).'
    sidecar: 'Standard Contracts → Mutation, query and CSSOM: the toggle read per role, `role="switch"`, native checkedness unread by the builder (a measured absence).'
    rationale: 'Report Changes → Symbols / APIs F2; Decisions & corrections «For P5, on the operator''s word (inputs#I5)» (native checkedness unread, measured from the builder); Counts → acc-states and `aria-checked` stale at architecture.md:121.'
    basis: .andromeda/architecture.md:121
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Scrolling, selection, tree
    change: 'Replace «the walk does not stop at a `contain: paint` box whose `overflow` is `visible` — measured, still hit through» with: since 2026-10-18-engine-debts it stops at such a box''s padding box, on a box paint applies `contain: paint` to, as paint clips it (packages/blitz-dom/src/node/node.rs:1501-1516), pinned by `hit_clipped_at_scrolling_box::a_contain_paint_box_stops_the_hit_at_its_padding_box`; and drop from «of these readings two stand as measured, each owned by a `.andromeda/residuals.md` line» the `contain: paint` widening on the founder''s C4, now made (PROVISIONAL, F5 A).'
    sidecar: 'Standard Contracts → Scrolling, selection, tree: the hit stops at a `contain: paint` box; its residuals ownership retired.'
    rationale: 'Report Changes → Symbols / APIs F5; Counts → renamed tests; the sweep marks architecture.md:123 `contain-paint` stale (@c4430/@c7404).'
    basis: .andromeda/architecture.md:123
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Scrolling, selection, tree
    change: 'Replace «`Node::absolute_position`, the second position reader, is not edited … reads (−30, −70) … left as it reads by the operator''s answer of 2026-10-10» with: since 2026-10-18-engine-debts it subtracts only its containing blocks'' scroll, so a box at (40, 50) reads (40, 50) at every scroll offset, in both layout modes, and the harness''s `layout_rect`, `layout_rect_of` and `center_of` read such a box where it stands (packages/blitz-dom/src/node/node.rs:1717-1738; packages/blitz-dom/src/scrolling.rs:719), pinned by `scrolled_box_absolute_position::a_scrolled_box_reads_its_absolute_position_where_it_stands`; and drop the C14 residuals ownership from «of these readings two stand as measured».'
    sidecar: 'Standard Contracts → Scrolling, selection, tree: `Node::absolute_position` reads a self-scrolled box where it stands; its residuals ownership retired.'
    rationale: 'Report Changes → Symbols / APIs F4; Outcome «(arch · F4) met — (40, 50) at every scroll offset»; Counts → `scrolled_box_absolute_position` test renamed; the sweep marks architecture.md:123 `abs-position` stale (@c6731-7674).'
    basis: .andromeda/architecture.md:123
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Scrolling, selection, tree
    change: 'Replace «`BaseDocument::get_client_bounding_rect` reads that box at its layout position (0, 40, 300, 140), not where its transform moves it — a divergence from CSSOM View … owned by a `.andromeda/residuals.md` line» with: since 2026-10-18-engine-debts `get_client_bounding_rect` and `node_client_rects` map the border box''s corners through the node''s own transform and each containing block''s, in device pixels as `hit_inner` and the paint tree map them, and read the axis-aligned bounding box, through the private `transformed_bounds`, gated by `is_transformed_chain`, a non-atomic inline element mapping its fragments through `transformed_fragment_rects` (packages/blitz-dom/src/document.rs:2376-2378; packages/blitz-dom/src/document.rs:2387-2396; packages/blitz-dom/src/document.rs:2405-2494; packages/blitz-dom/src/document.rs:2565-2567); with no transform on the chain the reading is unchanged, and `offset_rect`, `inline_fragment_rects`, `physical_unrounded_geometry` and `visible_region` are not edited; add the measured limit: `visible_region` reads a scrolling box under a transform where it stands untransformed, so a target inside a translated scrolling box reads outside its visible region (pinned by `transformed_client_rect::a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region`, stated in `off-screen`''s meaning, owned by a new `.andromeda/residuals.md` line, target next); PROVISIONAL (F8 A).'
    sidecar: 'Standard Contracts → Scrolling, selection, tree: the client rect under transforms; the `visible_region` limit under a transformed scrolling box stated.'
    rationale: 'Report Changes → Symbols / APIs F8 and «The driver texts» (off-screen); Deviations 7 (the forecast measured); Decisions & corrections card 2 row 3 (its condition MET, a new residuals line); the sweep marks architecture.md:123 `client-rect-tf` stale (@c2722/@c6241-6338).'
    basis: .andromeda/architecture.md:123
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Paint and shell
    change: 'Replace «a click inside a field holding a value still places the caret by the real characters'' widths (blitz-dom''s pointer path, unchanged) … a known 0.1.0 limit whose fix … is owned by a `.andromeda/residuals.md` line (target: next) on the founder''s C11» with: since 2026-10-18-engine-debts a click places the caret by the bullets — blitz-dom''s private `click_password_input` lays the bullets out through `layout_as_text_input`, scrolled as paint scrolls them, so no width of the real characters reaches the caret, a double click selecting the whole value (packages/blitz-dom/src/events/pointer.rs:586-652); `password_caret_click`, 2 tests, asserts one index across the narrow and the wide filling and prints none; PROVISIONAL (F6 A).'
    sidecar: 'Standard Contracts → Paint and shell: the password click caret placed by the bullets; the C11 residuals limit retired.'
    rationale: 'Report Changes → Symbols / APIs F6; Deviations 1 and 11; Counts → `password_caret_click` 1 → 2 tests, renamed; the sweep marks architecture.md:128 `pw-caret` stale.'
    basis: .andromeda/architecture.md:128
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Document core (blitz-dom)
    change: 'In `BaseDocument::layout_as_text_input`, replace «its one caller is blitz-paint''s password mask» with two callers: blitz-paint''s password mask and, since 2026-10-18-engine-debts, blitz-dom''s password click caret (`click_password_input`, packages/blitz-dom/src/events/pointer.rs:586-652).'
    sidecar: 'Standard Contracts → Document core: `layout_as_text_input` gains its second caller, the password click caret.'
    rationale: 'Report Changes → Symbols / APIs F6 «`layout_as_text_input` gains a second caller (it had the paint mask alone)».'
    basis: packages/blitz-dom/src/events/pointer.rs:586-652
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Nodes and widgets
    change: 'Register the new public accessor `ElementData::is_disabled(&self) -> bool` (since 2026-10-18-engine-debts): `can_be_disabled()` — `button` · `input` · `select` · `textarea` — and a `disabled` attribute by presence, the one reading focusability, the pointer gates and the activation keys share (packages/blitz-dom/src/node/element.rs:490-494).'
    sidecar: 'Standard Contracts → Nodes and widgets: `ElementData::is_disabled` registered.'
    rationale: 'Report Changes → Symbols / APIs F7 names «the new `pub fn ElementData::is_disabled`»; the contract lists `ElementData`''s public accessors and holds no `is_disabled` — an unregistered public API.'
    basis: packages/blitz-dom/src/node/element.rs:490-494
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'Restate `covered`''s meaning: drop «a hit reaches content clipped by `contain: paint`, so an element lying where such content extends can read `covered` though nothing shows over it» and add «a box with no width or no height is hit at its centre on the element that holds it, so it can read `covered` though nothing covers it» (F9 A, since 2026-10-18-engine-debts), the one-axis sentence standing; re-point to packages/escher-driver/src/refusal.rs:79-83 and the refusal-line test packages/escher-driver/src/json.rs:678-681; the cause set nine, no remedy moved.'
    sidecar: 'Standard Contracts → Driver session: `covered`''s meaning drops the `contain: paint` sentence and gains the no-area one.'
    rationale: 'Report Changes → Symbols / APIs «The driver texts» and Schema / config (`covered`''s meaning); the sweep marks architecture.md:132 `contain-paint` and `covered-limits` stale (@c18533-22088).'
    basis: .andromeda/architecture.md:132
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'Add to `off-screen`''s meaning, beside its 2026-10-07 text: since 2026-10-18-engine-debts it also reads «a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows» (F8''s measured branch, `transformed_client_rect`; packages/escher-driver/src/refusal.rs:20-24; packages/escher-driver/src/refusal.rs:269-273), carried in every `off-screen` refusal''s line and `describe()`''s causes.'
    sidecar: 'Standard Contracts → Driver session: `off-screen`''s meaning gains the transformed-scrolling-box sentence.'
    rationale: 'Report Changes → Symbols / APIs «The driver texts» (`Cause::OffScreen`''s doc and `meaning()` gain the sentence) and Counts → offscreen «The limit sentence is an addition — its homes are where `covered`''s limits are stated (architecture.md:132)».'
    basis: .andromeda/architecture.md:132
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'Restate the crate docs'' paragraph «two readings as off — the `contain: paint` one and … a box clipped on one axis» as «Three readings are known to be off»: a box clipped on one axis, a box with no area read `covered`, and an element inside a transformed scrolling box read `off-screen` (packages/escher-driver/src/lib.rs:68-69; packages/escher-driver/src/lib.rs:71-74).'
    sidecar: 'Standard Contracts → Driver session: the crate docs state three known-off readings.'
    rationale: 'Report Changes → Symbols / APIs «The driver texts»: «the crate docs'' paragraph, now «Three readings are known to be off» (`lib.rs:68-69`, `:71-74`)»; Deviations 7.'
    basis: .andromeda/architecture.md:132
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'In «The `covered` reading as it now stands», replace «two readings are still off — content clipped by `contain: paint` beside a `visible` `overflow`, which a hit still reaches, and … a target lying beside a box clipped on one axis» with: since 2026-10-18-engine-debts a hit stops at a `contain: paint` box too; the one-axis reading stands, and a box with no area (`task-header-spacer`, 0 high) reads `covered`, stated in `covered`''s meaning and pinned by `stand_act_ids::refused_at_boot`.'
    sidecar: 'Standard Contracts → Driver session: the `covered` reading as it stands no longer counts `contain: paint`.'
    rationale: 'Report Changes → Symbols / APIs F5 and «F3 B and F9 A — no engine change» (`covered` on a box with no area stays, pinned by `stand_act_ids::refused_at_boot`); the sweep marks architecture.md:132 `covered-limits` stale.'
    basis: .andromeda/architecture.md:132
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'In the `NODE_FIELDS` help, replace «`selected` · `pressed` · `invalid` read by `aria-selected` · `aria-pressed` · `aria-invalid`» with the 2026-10-18-engine-debts help: `checked` states its new source (a native checkbox''s or radio''s own state, else a checkable role''s toggle) and that a mixed one reads false; `pressed` reads the toggle only off «the five roles whose toggle reads as `checked`», not spelling `aria-checked` (packages/escher-driver/src/schema.rs:74-77; packages/escher-driver/src/schema.rs:93-94).'
    sidecar: 'Standard Contracts → Driver session: the `checked` and `pressed` node-key help restated.'
    rationale: 'Report Changes → Symbols / APIs F2 (the escher-driver node-key help for `checked` and `pressed`) and Schema / config; Deviations 4.'
    basis: .andromeda/architecture.md:132
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Dioxus DOM bridge
    change: 'Replace «and the engine honours no `inert`, so neither a truthy nor a falsy one makes content inert (§Established Decisions → Unsupported features)» with: a truthy `inert` makes its subtree inert since 2026-10-18-engine-debts — not focusable, not hit, out of the accessibility tree and the snapshot — while a framework-written `inert: false` is cleared and leaves content as it was (§Established Decisions → DOM semantics).'
    sidecar: 'Standard Contracts → Dioxus DOM bridge: a truthy `inert` now honoured; the bridge itself unchanged.'
    rationale: 'Report Changes → Symbols / APIs F1 (the bridge untouched, byte-identical to upstream); the sweep marks architecture.md:135 `inert` stale (@c1713-1825).'
    basis: .andromeda/architecture.md:135
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Dioxus DOM bridge
    change: 'Restate the snapshot''s `checked` and `pressed` sources: `checked` takes `checkbox_input_checked()` for a native input, else, for a node whose tree role is one of the five checkable roles (its own private `is_checkable`, a copy of blitz-dom''s), `toggled == Toggled::True` — `None` where the role states nothing, a mixed one `Some(false)`; `pressed` reads `toggled` only off those five roles; «the three are what the engine''s builder writes from `aria-selected`, `aria-pressed` and `aria-invalid`» gains `aria-checked` on a checkable role (packages/dioxus-native-dom/src/snapshot.rs:163-175; packages/dioxus-native-dom/src/snapshot.rs:213-225); PROVISIONAL (F2 A).'
    sidecar: 'Standard Contracts → Dioxus DOM bridge: the snapshot''s `checked` and `pressed` sources per role.'
    rationale: 'Report Changes → Symbols / APIs F2 (snapshot.rs:163-175, :213-225) and Deviations 2-3; Expected amendments name «the snapshot''s `checked` / `pressed` sources» under Dioxus DOM bridge.'
    basis: .andromeda/architecture.md:135
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Dioxus DOM bridge
    change: 'To `bounds` `BaseDocument::get_client_bounding_rect` add: since 2026-10-18-engine-debts the reader maps an element through the transforms on its containing-block chain (§Standard Contracts → Scrolling, selection, tree), so a translated element''s `bounds` read where it shows and a driver `click` on it is accepted and lands (`stand_act_transformed`, 2 tests, both layout modes).'
    sidecar: 'Standard Contracts → Dioxus DOM bridge: the snapshot''s `bounds` follow transforms.'
    rationale: 'Report Changes → Symbols / APIs F8 «The callers of `get_client_bounding_rect` move with it: the snapshot''s `bounds` … dioxus-native-dom''s `get_client_rect`»; Outcome «(arch · F8) met — `transformed_client_rect` 4 and `stand_act_transformed` 2».'
    basis: .andromeda/architecture.md:135
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'Restate the `accessibility_states` entry «`aria-checked` setting none of the three, 4 tests» as: `aria-checked` read on a checkable role alone and no author toggle on a native checkbox or radio, 5 tests, its fourth rewritten (`aria_checked_reads_its_three_values_on_a_checkable_role_alone`), the fifth `aria_pressed_on_a_checkable_role_reads_nothing_and_a_long_aria_checked_is_nowhere` (tests/blitz-tests/tests/accessibility_states.rs:245-316).'
    sidecar: 'Existing Scopes → blitz-tests: `accessibility_states` 5 tests, the `aria-checked` read.'
    rationale: 'Report Counts → `accessibility_states` 4 → 5 tests, its fourth rewritten; the sweep marks architecture.md:262 `acc-states` and `aria-checked` stale.'
    basis: .andromeda/architecture.md:262
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'Restate «a `contain: paint` box still hit through» in `hit_clipped_at_scrolling_box` (13 tests) as a `contain: paint` box stopping the hit at its padding box, and «`scrolled_box_absolute_position`, 1 test (`Node::absolute_position` pinned as it reads)» as 1 test reading a scrolled box''s absolute position where it stands; add `transformed_client_rect`, 4 tests in both layout modes (a translated, a scaled and a translated parent''s child box read moved, the offset rect not; a target inside a translated scrolling box outside its visible region).'
    sidecar: 'Existing Scopes → blitz-tests: the hit and absolute-position checks restated, `transformed_client_rect` added.'
    rationale: 'Report Counts → renamed tests (`a_contain_paint_box_stops_the_hit_at_its_padding_box`, `a_scrolled_box_reads_its_absolute_position_where_it_stands`) and Files → new checks; the sweep marks architecture.md:262 `contain-paint` and `abs-position` stale.'
    basis: .andromeda/architecture.md:262
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'In the `stand_keyboard` entry replace «a `disabled` attribute of any value skipped by Tab, the `<div disabled tabindex=0>` limit pinned» with a `disabled` attribute skipping a form control and not a `div`, and «the engine honouring no `inert`» with an inert subtree skipped by Tab (14 tests unchanged); add `inert_subtree`, 4 tests in both layout modes — Tab and Shift+Tab skip it, a hit passes through it, the tree holds no node for it, and removing `inert` restores focus, hit and tree, the focus-fixup limit pinned.'
    sidecar: 'Existing Scopes → blitz-tests: `stand_keyboard`''s inert and disabled readings restated, `inert_subtree` added.'
    rationale: 'Report Counts → renamed tests (`an_inert_subtree_is_skipped_by_tab`, `a_disabled_attribute_skips_a_form_control_and_not_a_div`), `stand_keyboard` 14 unchanged; Files → new checks; the sweep marks architecture.md:262 `inert` (@c7929) and `disabled-any` (@c7763) stale.'
    basis: .andromeda/architecture.md:262
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'In the Dioxus falsy boolean-attribute entry replace «`stand_keyboard` pins that the engine honours neither a truthy nor a falsy `inert`» with: since 2026-10-18-engine-debts a truthy `inert` is honoured (`inert_subtree`, `stand_keyboard`), a falsy one cleared by the bridge.'
    sidecar: 'Existing Scopes → blitz-tests: the falsy-`inert` clause restated.'
    rationale: 'Report Changes → Symbols / APIs F1; the sweep marks architecture.md:262 `inert` stale; the clause restates the retired [Unsupported features] claim.'
    basis: .andromeda/architecture.md:262
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'Replace «`password_caret_click`, 1 test in both layout modes, a password''s click-placed caret pinned at index 5 after eight narrow characters and 2 after eight wide ones» with `password_caret_click`, 2 tests in both layout modes: a click at one offset places the caret by the bullets, one index across the narrow and the wide filling (no index printed), and a double click selects the whole value (tests/blitz-tests/tests/password_caret_click.rs:106-121).'
    sidecar: 'Existing Scopes → blitz-tests: `password_caret_click` 2 tests, the caret by the bullets.'
    rationale: 'Report Counts → `password_caret_click` 1 → 2 tests and renamed tests; Deviations 11; the sweep marks architecture.md:262 `pw-caret` stale.'
    basis: .andromeda/architecture.md:262
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'Add the fourteenth driver-action file `stand_act_transformed`, 2 tests in both layout modes — a click on a translated button accepted and landing, its snapshot `bounds` where it shows — a `session_common` reader, so «the thirteen `stand_act_*`» reads fourteen and the readers twenty-two (twenty-one until 2026-10-18-engine-debts).'
    sidecar: 'Existing Scopes → blitz-tests: `stand_act_transformed` added; driver-action files 14, `session_common` readers 22.'
    rationale: 'Report Files → new checks and Harness / gate surface («one new `stand_*` file (`stand_act_transformed`, a `session_common` reader)»); Counts → stand-act-13 stale at architecture.md:262 and `session_common` readers 21 → 22 — the «twenty-one readers» wording at architecture.md:262 is one the `sc-readers` pattern did not reach.'
    basis: .andromeda/architecture.md:262
    dependent-of: D-arch-decisions
note: 'No new port, socket, listener, env var, carrier, crate or dependency (report Changes; audit green, deny.toml unchanged), so §Occupied Resources and §Stack need nothing; the cascade''s count row architecture.md:114 («the thirteen driver-action checks `stand_*`» → fourteen) and the two CARRY checks `accessibility_merged_states` (3) and `accessibility_active_descendant_unread` (1) for the blitz-tests scope row are left to the cascade / test-plan owner; every engine proposal carries PROVISIONAL per inputs#I2 and the operator''s gated-arm directive (inputs#I7).'
