# Report — 2026-10-18-engine-debts

**Chunk:** Engine debts — the nine recorded engine deferrals each fixed, pinned or kept on the founder's fork
**Date:** 2026-10-18
**Commits:** `48680dfb9 chore(2026-10-18-engine-debts): operator pre-CI commit, for the run this chunk verdict reads` (the
chunk's code, evidence and the phase run, pushed; CI#39096326308 green on it). `34bcf71c6`, the one other commit since
`state.yaml` `last_wrap` (2026-10-18T02:01:00Z), is the previous chunk's own wrap commit and none of this chunk's.
**Wrap mode:** two-window, on the operator's word at invocation «run P1 only and stop» (2026-10-18): this window wrote P1
alone; P2–P7 resume in run dir `.andromeda/runs/2026-10-18T04-30-47-wrap` (Setup 2a).

## Changes (structured — detectors read this)
- **Files:** 22 tracked source and check files changed against `34bcf71c` (the parent of the pre-CI commit `48680dfb`;
  `gate.py scope`: `changed 22 · listed 22 · recorded 0`), every one on research's lists:
  - blitz-dom (7, the upstream merge surface): `packages/blitz-dom/src/node/node.rs` · `node/element.rs` · `scrolling.rs` ·
    `accessibility.rs` · `document.rs` · `events/pointer.rs` · `events/keyboard.rs`;
  - escher's own (5): `packages/dioxus-native-dom/src/snapshot.rs`; `packages/escher-driver/src/{schema,refusal,json,lib}.rs`;
  - new checks (5): `tests/blitz-tests/tests/{accessibility_merged_states,inert_subtree,accessibility_active_descendant_unread,transformed_client_rect,stand_act_transformed}.rs`;
  - rewritten checks (5): `tests/blitz-tests/tests/{stand_keyboard,accessibility_states,scrolled_box_absolute_position,hit_clipped_at_scrolling_box,password_caret_click}.rs`.
  Beside them: the chunk folder (`scope.md`, `research.md`, `plan.md`, `inputs/` I1–I6, `evidence/{controls,merge-surface,readings,operator-pass}.md`),
  `escher-0.2.0/verification-matrix.json` (v020-03 claimed gated at phase P5), `escher-0.2.0/working-route.md` and
  `.andromeda/master-route.md` (the phase's promotion), `.andromeda/friction-log.ndjson`, `.claude/session-handoff.md`
  (the previous wrap's), and the run dirs `.andromeda/runs/2026-10-18T02-38-20-phase/`, `…/2026-10-18T03-17-10-implement/`,
  `…/2026-10-18T04-30-47-wrap/`. No stand file, script, workflow, manifest, lockfile or `deny.toml` moved (gate entries
  `git diff --quiet 34bcf71c… -- Cargo.toml Cargo.lock deny.toml …` and the source-list probe, both green).
- **Symbols / APIs:** every engine change below is a behaviour change for every document — remote pages included —
  PROVISIONAL on the operator's word as the founder's delegate (inputs#I2), the `switch` mapping with it (inputs#I3):
  - **F1 `inert` honoured** (by presence, on the element or an ancestor element): `Node::is_focussable` also reads
    false for an inert node, through the new `pub(crate) fn Node::is_inert` (an ancestor walk;
    `packages/blitz-dom/src/node/node.rs:558-575`), so the cached `ElementData::is_focussable` stays per element and no
    mutator hook is added; its callers move with it — the Tab walk `focus_next_node` / `focus_prev_node`, autofocus
    (`process_added_subtree`) and `DioxusDocument::unkeyed_actionable`. `hit_inner` returns `None` at an element carrying
    `inert`, before any other test (`node.rs:1401-1410`). The accessibility builder's `is_hidden_from_accessibility_tree`
    reads true for an element carrying `inert` (`packages/blitz-dom/src/accessibility.rs:243`, `:245-256`), so the
    subtree leaves the tree, and the snapshot with it. Known limit: HTML's focus fixup is not built — a node that holds
    focus when it becomes inert keeps it until focus moves (pinned by `inert_subtree::removing_inert_restores_focus_hit_and_tree`).
    The bridge (`mutation_writer.rs`) is untouched, byte-identical to upstream at `23b9b26a9` (gate entry green).
  - **F2 the toggle read** (`build_accessibility_node`, `accessibility.rs:171-184`): a native `input` whose `type` is
    exactly `checkbox` or `radio` (the new private `is_native_checkable`, `:390-398`) reads no author toggle at all;
    else a role in the new private `is_checkable` set (`:378-388`: `CheckBox` · `Switch` · `RadioButton` ·
    `MenuItemCheckBox` · `MenuItemRadio`) writes `toggled` from `aria-checked` (`true` · `false` · `mixed`, trimmed,
    exact); every other role reads `aria-pressed` as before — one read per attribute, nothing else writes `toggled`.
    `role_from_name` gains `"switch"` → `Role::Switch` (`:330`); `role="switch"` mapped to no role before. The native
    exclusion is the operator's two corrections inside F2 A (inputs#I5, inputs#I6; HTML-AAM / ARIA in HTML).
    The snapshot (`packages/dioxus-native-dom/src/snapshot.rs:163-175`): `NodeState::checked` takes the native reader,
    else, for a node whose tree role is one of the five (its own private `is_checkable`, `:213-225` — a copy, as
    exporting blitz-dom's would move blitz-dom `lib.rs`, outside the twelve-file source gate), `toggled ==
    Toggled::True` — `None` where the role states nothing, a mixed one `Some(false)`; `NodeState::pressed` reads
    `toggled` only off those five roles. The escher-driver node-key help for `checked` and `pressed`
    (`packages/escher-driver/src/schema.rs:74-77`, `:93-94`) states the new source and `checked`'s mixed-reads-false;
    `pressed`'s help names "the five roles whose toggle reads as `checked`" and does not spell `aria-checked`, which
    the standing `schema_tests.rs` assertion (`!help("pressed").contains("aria-checked")`) forbids.
  - **F4 `Node::absolute_position`** (`node.rs:1717-1738`) adds the node's own location and hands the sum to the new
    private `position_in_containing_block`, which subtracts its containing block's scroll offset and recurses — only
    ancestors' scroll is subtracted. `BaseDocument::scroll_into_view` passes `(0.0, 0.0)` in place of the add-back
    (`packages/blitz-dom/src/scrolling.rs:719`). Every caller moves with it: `handle_dom_event`, `Node::focus`'s IME area,
    `synthetic_click_event_data`, `scroll_into_view`, and the harness's `layout_rect_of` / `write_node` (unedited).
  - **F5 the hit at `contain: paint`**: `hit_inner`'s padding-box stop (`node.rs:1501-1516`, the flag renamed
    `clips_to_padding_box`) also holds for a box whose computed `contain` includes `Contain::PAINT`, on a box paint
    applies it to (not an inline flow box, not an internal table box other than a cell — blitz-paint `render.rs`'s own
    rule), the root still excepted. Hit and paint agree.
  - **F6 the password click caret** (`packages/blitz-dom/src/events/pointer.rs`): `ClickTarget::TextInput` gains a
    `password` flag (an `input` whose `type` reads `password`, ASCII case-insensitive); for it the new private
    `click_password_input` (`pointer.rs:586-652`) lays out one U+2022 per character through
    `BaseDocument::layout_as_text_input`, scrolls that layout as paint scrolls it (`caret.x1 − content width`), finds
    the bullet under the point with parley's `Cursor::from_point`, and moves the real editor to that character's byte
    offset — a single click `move_to_byte`, a shift-click `extend_selection_to_byte`, two or more clicks `select_all`.
    Every other text input keeps `move_to_point` on the editor's own layout. `layout_as_text_input` gains a second
    caller (it had the paint mask alone).
  - **F7 `disabled` on form controls only**: the new `pub fn ElementData::is_disabled` (`packages/blitz-dom/src/node/element.rs:490-494`)
    — `can_be_disabled()` (`button` · `input` · `select` · `textarea`) and a `disabled` attribute by presence. Its
    readers: `flush_is_focussable` (`:709`), the pointer-move gate, `ClickTarget::Disabled`, `handle_click`'s walk gate
    (`pointer.rs:330`, `:463`, `:764`), `activates_on_enter` and `activates_on_space` (`packages/blitz-dom/src/events/keyboard.rs:163`,
    `:180`). The range-input key gate stays on its `input` as before. Consequences: a `<div disabled tabindex=0>` takes
    focus; a `<summary disabled>` or `<a href disabled>` takes Enter / Space activation and a click; a disabled
    ancestor `div` no longer stops `handle_click`'s walk.
  - **F8 the client rect under transforms** (`packages/blitz-dom/src/document.rs`): `get_client_bounding_rect`
    (`:2376-2378`, `:2387-2396`) and `node_client_rects` (`:2565-2567`) map through the new private
    `transformed_bounds` (`:2405-2452`: the border box's corners through the node's own transform and each containing
    block's, in device pixels as `hit_inner` and the paint tree map them, the offsets from `physical_unrounded_geometry`
    differences so both bodies are served, the axis-aligned bounding box), gated by `is_transformed_chain`
    (`:2454-2458`); a non-atomic inline element maps its fragments through `transformed_fragment_rects` (`:2460-2494`).
    With no transform on the chain the reading is today's in both bodies. `physical_unrounded_geometry`, `offset_rect`,
    `inline_fragment_rects` and `visible_region` are not edited — `offset_rect` (CSSOM `offsetTop`) stays
    untransformed. The callers of `get_client_bounding_rect` move with it: the snapshot's `bounds`, `node_client_rects`,
    `adjust_element_coords` (events driver), the script `getBoundingClientRect`, dioxus-native-dom's `get_client_rect`.
  - **The driver texts** (`packages/escher-driver/src/refusal.rs`, `json.rs`, `lib.rs`): `Cause::Covered`'s doc and
    `meaning()` drop the `contain: paint` sentence and gain «a box with no width or no height is hit at its centre on
    the element that holds it, so it can read `covered` though nothing covers it» (F9 A); `Cause::OffScreen`'s doc and
    `meaning()` gain «a scrolling box under a transform is read where it stands untransformed, so an element inside it
    can read `off-screen` though it shows» (F8's measured branch, `transformed_client_rect`); the unit table
    (`refusal.rs:269-273`, `:279-281`), the refusal-line test (`json.rs:678-681`) and the crate docs' paragraph, now
    «Three readings are known to be off» (`lib.rs:68-69`, `:71-74`), carry the same words. The cause set stays the nine of
    `CAUSES`; no remedy moved.
  - **F3 B and F9 A — no engine change**: `aria-activedescendant` stays unread, pinned by
    `accessibility_active_descendant_unread`; `covered` on a box with no area stays, stated in `covered`'s meaning and
    pinned by `stand_act_ids::refused_at_boot`.
  - No new port, socket, listener, env read, carrier, verb, key name, field, log field or span (the census and silence
    probes, both reading 0).
- **Crates / modules:** none added or removed; blitz-dom, dioxus-native-dom and escher-driver changed (above).
- **Dependencies:** none added or bumped (the manifest probe green; `bash .github/scripts/ci-leg.sh audit` green,
  `deny.toml` unchanged).
- **Schema / config:** the driver's served fixed texts: `checked`'s and `pressed`'s node-key help, `covered`'s and
  `off-screen`'s meanings (above) — the program's own `'static` table text inside the ratified carriers; no config key,
  no scrub set, no violation schema.
- **Spec-master edits:** none — no master was edited by implement; the masters' restatements are this wrap's P2.
- **Counts / qualifiers moved:** each with its pattern id, the sweep `cascade.py sweep --pre-pass` over
  `{run_dir}/report-patterns.toml` (trail `.andromeda/runs/2026-10-18T04-30-47-wrap/cascade-2026-10-18-engine-debts-pre-pass.json`),
  baseline `48680dfb` = HEAD; dispositions per row below (`stale` = states the reading this chunk changed · `true` = a
  true claim sharing the token · `historical` = a past chunk's dated tally).
  - blitz-tests files 127 → 132 and targets 126 → 131 (five new files; `ls tests/blitz-tests/tests/*.rs | wc -l` → 132,
    `all.rs` unbuilt) — `files-127`: 1 standing · 1 stale (test-plan.md:66).
  - `stand_*.rs` files (`run stand`, `stand-repeat`) 40 → 41 (`stand_act_transformed`; `ls …/stand_*.rs | wc -l` → 41) —
    `stand-40`: 11 standing · 2 stale (test-plan.md:315 «40 files», :344 «the 40 `stand_*.rs` files») · 1 historical
    (test-plan.md:116, dated per-chunk `run stand` tallies, «over 40 files» @c8516, @c9045) · 8 true (`run stand`
    named: test-plan.md:18, :26, :107, :110, :115, :122, :147; obs-plan.md:87).
  - `session_common` readers 21 → 22 (`grep -l '^mod session_common;'` → 22) — `sc-readers`: 1 standing · 1 stale
    (test-plan.md:101).
  - `stand_act_*` files 13 → 14 («thirteen driver-action checks»; `ls …/stand_act_*.rs | wc -l` → 14) — `stand-act-13`:
    13 standing · 10 stale (architecture.md:114, :262; test-plan.md:21, :66, :100, :101, :143, :181; obs-plan.md:70;
    registries/contracts/test-plan/session-lifecycle.md:11 — the key file of test-plan) · 3 historical (test-plan.md:26
    @c43412, :116, :331).
  - dioxus-native-dom lib tests 69 → 72 (gate entry `cargo test -p dioxus-native-dom --locked --lib`: `72 passed`) —
    `dnd-69`: 1 standing · 1 stale (test-plan.md:24).
  - `accessibility_states` 4 → 5 tests, its fourth test rewritten (`aria_checked_sets_none_of_the_three` →
    `aria_checked_reads_its_three_values_on_a_checkable_role_alone`) and a fifth added — `acc-states`: 11 standing ·
    7 stale (architecture.md:121, :262; security-plan.md:127; test-plan.md:26 @c11104; a11y-plan.md:46, :69, :104 —
    the «fourth test standing as its pin» and its `:128-220` coordinates) · 1 true (test-plan.md:66, the file list) ·
    3 historical (test-plan.md:116, :331; obs-plan.md:70 the census list is true).
  - `password_caret_click` 1 → 2 tests — `pw-caret`: 11 standing · 5 stale (architecture.md:128, :262;
    security-plan.md:130; test-plan.md:18 «`password_caret_click` 1»; a11y-plan.md:283) · 6 true or historical
    (test-plan.md:66 file list, :100, :101 declares no shared module, :116, :331 historical; obs-plan.md:70 census list).
  - Unchanged and re-read: `stand_keyboard` 14, `hit_clipped_at_scrolling_box` 13, escher-driver lib 74,
    `scroll_into_view_nested` 7, `stand_act_obstructed` 6, `scrolled_box_absolute_position` 1 (its test renamed
    `a_scrolled_box_reads_its_absolute_position_where_it_stands`), the cause set nine.
  - Renamed tests (a name a master cites): `stand_keyboard::the_engine_honours_no_inert` → `an_inert_subtree_is_skipped_by_tab`;
    `stand_keyboard::a_disabled_attribute_of_any_value_is_skipped` → `a_disabled_attribute_skips_a_form_control_and_not_a_div`;
    `hit_clipped_at_scrolling_box::a_contain_paint_box_with_visible_overflow_is_still_hit_through` →
    `a_contain_paint_box_stops_the_hit_at_its_padding_box`; `password_caret_click::a_click_at_one_offset_places_the_caret_by_the_real_characters`
    → `a_click_at_one_offset_places_the_caret_by_the_bullets`, plus `a_double_click_selects_the_whole_value`.
  - The behaviour qualifiers, each its own fact:
    - F1 «the engine honours no `inert`» — `inert`: 10 standing · 6 stale (architecture.md:85 [Unsupported
      features], :135 @c1713-1825, :262 @c7929; test-plan.md:26 @c3218, @c39390; a11y-plan.md:188 @c1163-1597, :215) ·
      4 true (security-plan.md:146 and architecture.md:135/:262's 28-name boolean list sharing the token;
      test-plan.md:24 «a keyed or inert element not reported» (`unkeyed_actionable`); test-plan.md:183 and
      obs-plan.md:70 «inert» meaning harmless).
    - F2 «`aria-checked` is not read» — `aria-checked`: 6 standing · 6 stale (architecture.md:121 @c4904, :262 @c397;
      security-plan.md:127; test-plan.md:26 @c11412; a11y-plan.md:46, :104). The `switch` mapping and the native
      exclusion share this pattern (`Role::Switch`, `checkable`, `native checkedness`): no further master row.
    - F3 the unread `aria-activedescendant`, now pinned by a check — `active-desc`: 3 standing · 2 true, the check to
      name (layout-templates.md:10 @c5280; a11y-plan.md:198) · 1 stale owner (a11y-plan.md:285 «owned by a
      `.andromeda/residuals.md` line» → pinned by `accessibility_active_descendant_unread`, F3 B).
    - F4 `Node::absolute_position` less its own scroll, (−30, −70) — `abs-position`: 9 standing · 5 stale
      (architecture.md:86 @c2120, :123 @c6731-7674, :262 @c3803-4041; test-plan.md:26 @c31963-32661, :92) · 4 true
      (test-plan.md:66 file list; :324 and :331 «`scrolled_box_absolute_position` (1)» still 1; obs-plan.md:70 census list).
    - F5 the hit through `contain: paint` — `contain-paint`: 7 standing · 7 stale (architecture.md:86 @c1607, :123
      @c4430/@c7404, :132 @c18652/@c20536/@c21928, :262 @c3558; security-plan.md:355 @c1742; test-plan.md:26
      @c31866/@c32442, :181 @c4046).
    - `covered`'s limits («two readings», the one-axis one standing) — `covered-limits`: 7 standing · 5 stale (the
      «two readings» / `contain: paint` clause: architecture.md:86 @c1575, :123 @c5573, :132 @c18533-22088;
      security-plan.md:355 @c1654/@c1822; test-plan.md:181 @c3933/@c4200) · 2 true (the one-axis claim alone:
      architecture.md:262 @c11694; test-plan.md:26 @c22350/@c31687).
    - F9 `covered` on a box with no area (`task-header-spacer`) — `covered-noarea`: 3 standing · 2 true, now stated in
      `covered`'s meaning (architecture.md:132 @c37484-37538; test-plan.md:26 @c18349) · 1 true by token alone
      (layout-templates.md:10 «600 high» matched `0 high`).
    - F6 the caret by the real characters — `pw-caret` (above): the 5 stale rows carry it.
    - F7 `disabled` by presence on any element, the `<div disabled tabindex=0>` limit — `disabled-any`: 12 standing ·
      8 stale (architecture.md:86 @c9148, :90 @c334/@c1506, :262 @c7763; security-plan.md:119 @c40/@c638/@c2642;
      test-plan.md:24 @c3192 «focusability read by presence in every document», :26 @c39076; a11y-plan.md:146, :215
      @c957) · 4 true (architecture.md:135 @c14978 and security-plan.md:126 — the snapshot's `enabled` on a
      `can_be_disabled()` element; security-plan.md:118 «read by presence», true on the four controls, its `disabled`
      row carrying the narrowing; a11y-plan.md:167 the range input's gate).
    - F8 the client rect at the layout position — `client-rect-tf`: 4 standing · 2 stale (architecture.md:123
      @c2722/@c6241-6338 «(0, 40, 300, 140), not where its transform moves it»; test-plan.md:26 @c31023-31133) · 2
      true (architecture.md:135 @c13169 — `bounds` is `get_client_bounding_rect`; test-plan.md:92 — a scrolled box's
      client rect where it stands).
    - F8's measured limit, `off-screen` under a transformed scrolling box — `offscreen`: 13 standing · 0 stale · 13
      true (the cause's name or `visible_region` named: architecture.md:123 @c2362/@c5184, :132 @c16819-35876;
      security-plan.md:81, :355; layout-templates.md:45, :73; test-plan.md:26, :143, :181, :200, :324;
      obs-plan.md:106; registries/contracts/test-plan/session-lifecycle.md:5). The limit sentence is an addition —
      its homes are where `covered`'s limits are stated (architecture.md:132; security-plan.md:355), P2's to place.
    - The CARRY: the merged `aria-expanded` · `aria-disabled` · `aria-level` reads, «observed absent» → present
      (`accessibility_merged_states`, 3 tests) — `merged-aria`: 79 standing · 1 stale (test-plan.md:26 @c13325-13528
      «observed absent: a test naming the builder's merged …») · 7 true (architecture.md:121 @c3305-7871, :135
      @c13002; security-plan.md:126; a11y-plan.md:47 @c705-1185, :124, :127, :283 @c2489-2640 — the reads and
      «`aria-disabled` moves no snapshot reading») · 71 true by token alone (the pattern's «observed absent» clause on
      every other master's unrelated «observed absent» lines: architecture.md ×4, security-plan.md ×14,
      design-system.md ×2, test-plan.md ×7, obs-plan.md ×44).
    - The merge surface of the seven fixes, owned by a CARRY on «Upstream sync ahead of determinism and traces» —
      `merge-surface`: 9 standing · 9 true (the `23b9b26a9` merge base and earlier fixes' surfaces: architecture.md:75,
      :86 @c6600/@c9675, :94, :121, :205; security-plan.md:244; test-plan.md:10, :26; a11y-plan.md:47) · the seven new
      surfaces are additions (`evidence/merge-surface.md`), P2's and P5's.
    - The two new limits this chunk measured, with no master row yet: F1's focus-fixup limit (`inert` pattern's
      «focus fixup» clause: 0 master rows) and native checkedness unread by the tree (`aria-checked` pattern's «native
      checkedness» clause: 0 master rows).
  - Lines over 2 000 chars, one word per (line, fact) pair (the `by line` block's 26 heads; offsets as above):
    architecture.md:85 inert stale · :86 abs-position stale, contain-paint stale, disabled-any stale, covered-limits
    stale, merge-surface true · :90 disabled-any stale · :114 stand-act-13 stale · :121 aria-checked stale, merged-aria
    true, acc-states stale, merge-surface true · :123 abs-position stale, contain-paint stale, client-rect-tf stale,
    covered-limits stale, offscreen true · :128 pw-caret stale · :132 contain-paint stale, covered-limits stale,
    offscreen true, covered-noarea true · :135 inert stale, disabled-any true, client-rect-tf true, merged-aria true ·
    :262 inert stale, aria-checked stale, abs-position stale, contain-paint stale, pw-caret stale, disabled-any stale,
    covered-limits true, stand-act-13 stale, acc-states stale; security-plan.md:81 offscreen true · :119 disabled-any
    stale · :130 pw-caret stale · :355 contain-paint stale, covered-limits stale, offscreen true; test-plan.md:26
    inert stale, aria-checked stale, abs-position stale, contain-paint stale, disabled-any stale, client-rect-tf stale,
    covered-noarea true, covered-limits true, offscreen true, merged-aria stale, stand-40 true, stand-act-13
    historical, acc-states stale, merge-surface true · :66 abs-position true, pw-caret true, files-127 stale,
    stand-act-13 stale, acc-states true · :101 pw-caret true, sc-readers stale, stand-act-13 stale · :116 pw-caret
    true, stand-40 historical, stand-act-13 historical, acc-states historical · :181 contain-paint stale,
    covered-limits stale, offscreen true, stand-act-13 stale · :331 abs-position true, pw-caret historical,
    stand-act-13 historical, acc-states historical; obs-plan.md:70 inert true, abs-position true, pw-caret true,
    stand-act-13 stale, acc-states true · :106 offscreen true, merged-aria true; a11y-plan.md:47 merged-aria true,
    merge-surface true · :188 inert stale · :283 merged-aria true, pw-caret stale;
    registries/contracts/test-plan/session-lifecycle.md:5 offscreen true.
  - Sweep reach: 20 patterns · 24 moved facts, 24 with a pattern (the report's own count: F1–F9, the `switch` mapping,
    the native exclusion, the off-screen limit, the covered sentence dropped, the CARRY, the seven counts, the merge
    surface, the two new limits) · owner pattern control unfired (`title · control UNFIRED over the seven masters at
    48680dfb` — the chunk title «Engine debts» stands in no master) · by line 26 lines · 146 offsets. `provisional`
    (`PROVISIONAL|Provisional discharge|delegate operator`) was dropped the same way: `control UNFIRED` — no master
    states a PROVISIONAL mark today. A listing reaches its patterns' wording alone; a site stating a fact in other words
    is the detectors' to find.
- **Dev-tool versions:** none — no host tool installed, upgraded or read changed.
- **Harness / gate surface:** none changed (no CI step, script, leg or agent-run verb); five new blitz-tests targets
  and one new `stand_*` file (`stand_act_transformed`, a `session_common` reader), picked up by `run stand`,
  `stand-repeat` and the workspace leg by their names.
- **Cross-project / external claims:**
  - CI on the pushed pre-CI commit: `48680dfb98f3 verdict: green · checks 17/17 · wall 928 s · runs CI#39096326308
    completed/success` (repo Turbolet85/escher; `evidence/operator-pass.md`). This wrap's own commit adds to that tree.
  - Inputs (`inputs.py verify`, wrap P1; trail `.andromeda/runs/2026-10-18T04-30-47-wrap/inputs-2026-10-18-engine-debts.json`),
    summary `inputs: 6 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 6 ·
    uncited 3 · unparsed 0` (read before this report existed; I4, I5, I6 are cited here):
    - I1 · message: the operator, at the /andromeda-phase invocation, 2026-10-18 02:38Z · copy message · n/a — the
      founder's forks in one file, answered by the operator in his stead, PROVISIONAL (cited scope.md:12, research.md:7,
      plan.md:10, :517).
    - I2 · message: the operator, in the founder stead as delegate, 2026-10-18 02:53Z · copy message · n/a — F1 A ·
      F2 A · F3 B · F4 A · F5 A · F6 A · F7 A · F8 A · F9 A (cited scope.md:74, plan.md).
    - I3 · message: the operator, at the P5 review of p5-card-1.md, 2026-10-18 03:12Z · copy message · n/a — a P5
      review card: `.andromeda/runs/2026-10-18T02-38-20-phase/p5-card-1.md`, **superseded** by I4's card 2 (its rows
      1–4 carried unchanged onto card 2).
    - inputs#I4 · message: the operator, at the P5 review of p5-card-2.md, 2026-10-18 03:15Z · copy message · n/a — the
      governing P5 review card, `.andromeda/runs/2026-10-18T02-38-20-phase/p5-card-2.md`, its word the operator's own
      authority («yes»); its `## Beyond the plan` rows, verbatim (rows 1–4 accepted on card 1, inputs#I3; unchanged):
      | # | Proposal the word approves that neither plan.md nor the matrix claim holds |
      |---|---|
      | 1 | The merge surface of the seven engine fixes is owned by a CARRY on the working-route entry «Upstream sync ahead of determinism and traces» (escher-0.2.0/working-route.md:23), written by this chunk's wrap (route-resolve). |
      | 2 | The PROVISIONAL marks of the seven fixes and the `switch` mapping are listed for the founder at this chunk's wrap, the Epoch 2 boundary, under «Provisional discharge» (playbook:48): his own word ratifies or reworks each, and that word is v020-03's gated clearance. |
      | 3 | If step 8 measures the `visible_region` misreading, that pinned limit gets a new `open` line in `.andromeda/residuals.md` (target: next), appended by this chunk's wrap. |
      | 4 | The F1 focus-fixup limit (a node made inert keeps focus) gets a new `open` line in `.andromeda/residuals.md` (target: next), appended by this chunk's wrap. |
    - inputs#I5 · message: the operator, in the implementing session, 2026-10-18 about 03:52Z · copy message · n/a,
      snapped at this wrap (`.andromeda/runs/2026-10-18T04-30-47-wrap/relay-1.md`) — a native checkbox or radio takes no
      `aria-checked`; whether native checkedness reaches the tree, a new open residuals line if not.
    - inputs#I6 · message: the operator, in the implementing session, 2026-10-18 about 04:03Z · copy message · n/a,
      snapped at this wrap (`relay-2.md`) — a native checkbox or radio takes neither `aria-checked` nor `aria-pressed`;
      the operator pass entry by entry, the pre-CI commit subject given exactly.
  - HTML-AAM / ARIA in HTML (W3C) as the operator cited it (inputs#I5, inputs#I6): read by the operator, not snapped.
- **Reverted / negative API facts:** F8's first edit mapped transforms inside `inline_fragment_rects`, which
  `offset_rect` reads — gate `cargo test -p blitz-tests --locked --test transformed_client_rect` read red at the span's
  offset rect; reverted, `inline_fragment_rects` byte-identical to the base again, the transform moved to the private
  `transformed_fragment_rects`. The first F2 shape — a native checkbox writing its `aria-checked` as `toggled` — shipped
  in no commit: reworked twice on the operator's word (inputs#I5, inputs#I6) before the pre-CI commit.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none beyond the restatements the plan expected — every master claim this
  chunk falsified is an Expected amendment below, each with its pattern and stale rows (Counts / qualifiers moved). The
  native-checkedness absence is a measured absence, not a disproved claim: no master row states the tree carries a
  native input's checked state (`aria-checked` pattern's «native checkedness» clause: 0 master rows); a site stating it
  in other words is Validate's to raise.
- **Expected amendments (from plan):** (patterns and per-master hits from the sweep above; `.andromeda/registries/**`
  read with the masters)
  - architecture §Established Decisions → [Scrolling and input] / [DOM semantics] / [Unsupported features] — seven
    direct engine changes, PROVISIONAL, `inert` leaving [Unsupported features] — **carried** (Symbols / APIs). Patterns
    `inert` (architecture 3), `aria-checked` (2), `abs-position` (3), `contain-paint` (4), `pw-caret` (2),
    `disabled-any` (4), `client-rect-tf` (2); `provisional` 0 (control unfired — no master states a PROVISIONAL mark).
  - architecture §Standard Contracts → Scrolling, selection, tree · Dioxus DOM bridge · Driver session — the geometry
    readers, the snapshot's `checked` / `pressed` sources, `covered`'s two limits — **carried**. Patterns
    `abs-position`, `contain-paint`, `client-rect-tf`, `covered-limits` (architecture 4), `covered-noarea` (1),
    `offscreen` (2), `aria-checked`, `acc-states` (2), `stand-act-13` (2).
  - security-plan §Input Validation — the `aria-checked` read and the `switch` role, the `inert` read, the `disabled`
    row narrowed to `can_be_disabled()`, the password caret limit retired — **carried**. Patterns `aria-checked`
    (security-plan 1), `inert` (1, the boolean list — the `inert` read itself has no row yet), `disabled-any` (3),
    `pw-caret` (1), `contain-paint` (1), `covered-limits` (1), `offscreen` (2), `acc-states` (1).
  - a11y-plan §2, §4, §5, §7 — the `aria-checked` source, the fourth test rewritten, `inert` and `disabled` in focus
    order, the `aria-activedescendant` gap pinned by its check — **carried**. Patterns `aria-checked` (a11y-plan 2),
    `acc-states` (3), `inert` (2), `disabled-any` (3), `active-desc` (2), `pw-caret` (1), `merged-aria` (4).
  - test-plan §1, §2, §3, §9 — the five new files, the CARRY observed present, the counts, the per-OS tallies —
    **carried** (the counts and the CARRY above; per-OS tallies: CI#39096326308's 17/17 on the pushed sha, the per-OS
    test result lines not read by this report). Patterns `files-127` (test-plan 1), `stand-40` (10), `sc-readers` (1),
    `stand-act-13` (9 + its key file 1), `dnd-69` (1), `acc-states` (4), `merged-aria` (8), `pw-caret` (6),
    `abs-position` (5), `inert` (3), `disabled-any` (2), `client-rect-tf` (2), `contain-paint` (2), `offscreen` (5).
  - obs-plan §3 — the five new checks in the silence census — **carried** (the silence probe over the new and rewritten
    checks reads 0). Patterns `abs-position`, `pw-caret`, `acc-states`, `stand-act-13`, `inert` (obs-plan 1 each, all
    obs-plan.md:70, its census list).
  - design-system §Depth Strategy — the hit's stop at `contain: paint`; the transformed rect — **carried** (Symbols /
    APIs, F5 and F8), yet patterns `contain-paint` and `client-rect-tf` read **0 rows in design-system.md**: the sweep
    names no site there, so this report names design-system as owner of no row; the design-system detector's own
    sweep is the control for wording these patterns miss.
- **Coverage of new surfaces:**
  - `inert` attribute read (engine, every document) → validation presence-only, closed ✓ · instrumentation n/a (no log
    site, census 0) · PII n/a (nothing carried out) · tests integ `inert_subtree` (4) + `stand_keyboard` · a11y focus
    order and tree ✓ (SC 2.4.3) · tokens n/a
  - `aria-checked` read on the five checkable roles (engine) → validation closed set, trimmed, exact ✓ · instrumentation
    n/a · PII redacted✓ (a 5,500-character value nowhere in the tree's `Debug`) · tests integ `accessibility_states`
    (5) + snapshot unit (2) · a11y role derivation ✓ · tokens n/a
  - password click caret by the bullets (engine pointer path) → validation n/a · instrumentation n/a · PII masked✓ (no
    width of the real text reaches the caret) · tests integ `password_caret_click` (2) · a11y n/a · tokens n/a
  - transformed client rect (engine geometry; the snapshot's `bounds`) → validation n/a · instrumentation n/a · PII n/a ·
    tests integ `transformed_client_rect` (4) + driver `stand_act_transformed` (2) · a11y n/a · tokens n/a
  - the driver's reworded fixed texts (`covered`, `off-screen`, `checked`, `pressed`) → validation n/a · instrumentation
    n/a · PII n/a (program text) · tests unit escher-driver (74) · a11y n/a · tokens n/a

## Deviations from intent
Each from `.andromeda/runs/2026-10-18T03-17-10-implement/outcome-2026-10-18-engine-debts.md`, with its justification:
1. **F6, the click point (plan step 9).** The plan finds the bullet under `(tx, ty)`, whose `tx` carries the editor's
   scroll of the real text; `click_password_input` scrolls the bullets' layout as paint scrolls it instead, so no width
   of the real characters reaches the caret, even in an overflowing value. A triple click selects all, as a double does.
2. **F2, the checkable-role set** is copied privately into `snapshot.rs`: exporting blitz-dom's `is_checkable` would
   move blitz-dom `lib.rs`, outside the twelve-file source gate.
3. **F2, `checked` with no stated value** reads `None` on a checkable role, as `pressed` reads `None` where nothing is
   stated.
4. **F2, the `pressed` help** does not spell `aria-checked` — `schema_tests.rs` (not on the lists) asserts its absence.
5. **F2, native checkbox and radio** — two corrections on the operator's word inside F2 A: a native `input` of type
   `checkbox` or `radio` reads neither `aria-checked` (inputs#I5) nor `aria-pressed` (inputs#I6), its native
   checkedness winning (HTML-AAM / ARIA in HTML). `accessibility_states`' fourth test covers four unchecked natives
   (checkbox, radio, checkbox + `role="switch"`, radio + `role="button"`) each with both attributes; the count stays 5.
   The F2 mutation control and the native gate's own control were re-run on the final shape (`evidence/controls.md`,
   `readings.md`).
6. **F8, inline fragments** — the transform lives in the private `transformed_fragment_rects`, not in
   `inline_fragment_rects`, which `offset_rect` reads (Reverted / negative API facts).
7. **F8, the forecast measured (plan step 8).** `visible_region` misreads a target inside a translated scrolling box;
   the check pins it and `off-screen`'s meaning gains the planned sentence, `lib.rs` reading «Three readings».
8. **F1, where the focus-fixup limit is pinned** — the plan's «pinned by its fourth test» read as `inert_subtree`'s
   fourth test; the limit stated in both `inert_subtree`'s and `stand_keyboard`'s `//!` docs.
9. **F7, links and summaries** — a `<summary disabled>` or `<a href disabled>` now takes activation and a click, as HTML
   does: a consequence of the narrowing.
10. **The CARRY's tables** carry rows beyond the plan's minimum, each table's count asserted.
11. **F6, the caret index** is asserted equal across the two fillings and inside 1–7; the check prints no index.

scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 22 · listed 22 · recorded 0 (companion 0 ·
mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 63`, base 34bcf71c).

## Decisions & corrections
- The founder's nine forks answered by the operator as his delegate, PROVISIONAL: F1 A · F2 A · F3 B · F4 A · F5 A ·
  F6 A · F7 A · F8 A · F9 A (inputs#I2); the P5 review held v020-03 to the founder's own word, claimed gated, the `switch`
  mapping in the batch (inputs#I3, inputs#I4). **For P7:** the gated arm — v020-03 verifies only when the founder's own
  word discharges the batch; if he has not ruled by the coverage gate, P7 holds the cap and flips the master record
  `gated`; his word then runs plan step 13 in a gated `/andromeda-implement`.
- **For P5 (route-resolve), from card 2's rows (inputs#I4, the operator's own authority):** row 1 — a CARRY for the seven
  fixes' merge surface on «Upstream sync ahead of determinism and traces» (escher-0.2.0/working-route.md:23); row 2 — the
  PROVISIONAL batch listed for the founder at this wrap under «Provisional discharge» (playbook:48); row 3 — its
  condition MET (the `visible_region` misreading measured, `transformed_client_rect`): a new `open` line in
  `.andromeda/residuals.md` (target: next); row 4 — the F1 focus-fixup limit, a new `open` line (target: next).
- **For P5, on the operator's word (inputs#I5):** a new `open` residuals line (target: next), proposed: «native
  checkedness unread by the accessibility builder: an `<input type="checkbox">` or `type="radio"` writes no `toggled`
  from its own checked state, so the tree the shell hands the adapter never states it checked (HTML-AAM); the snapshot's
  `checked` reads the element and is right — found at 2026-10-18-engine-debts, after the operator's F2 A correction that
  a native input takes no `aria-checked`». Measured from the builder: `accessibility.rs` reads no
  `checkbox_input_checked()` and no `CHECKED` state; the base's own pin read a natively checked checkbox as no state.
  Its fix is an engine change for every document — the founder's or the operator's decision, not this wrap's.
- The operator's corrections inside F2 A, twice (inputs#I5 «must NOT take aria-checked …», inputs#I6 «takes neither
  aria-checked nor aria-pressed (HTML-AAM)»), each landed with its check and re-fired green before the operator pass.
- The operator pass on the operator's direction (inputs#I6): entry 29 hygiene, the pre-CI commit with the exact subject,
  entry 30 through `--operator 30` in the background, entry 31 as written — every entry green (`evidence/operator-pass.md`).
- The wrap's two-window form on the operator's word «run P1 only and stop» (2026-10-18, this invocation).
- Guard refusal, a recurrence of a standing learning: the PreToolUse bash guard refused a bare `cd` into the chunk's
  `inputs/` dir («moves the session cwd») at implement; re-issued with absolute paths — host-linux.md «Paths & the
  working directory», which the previous handoff's deferred learnings already name as recurring.
- Guard refusal: a `cat` heredoc with a file target (a scripted rewrite of `covered`'s text) refused at implement;
  re-issued through the Edit tool.
- Guard refusal: `xargs … command grep` refused (xargs cannot run the builtin); re-issued as a shell `for` loop.
- Sweep hazard (the census gate entry): `git diff -U0 … | grep … '\.unwrap\(\)|\.expect\(|panic!'` counts every added
  line of `snapshot.rs`, its `#[cfg(test)]` unit tests included — the new snapshot unit test was written without them.
- Sweep hazard (this report's site sweep): the CARRY pattern's clause `observed absent` matched 71 unrelated «observed
  absent» lines across five masters; `0 high` matched «600 high» (layout-templates.md:10). A clause shared with a
  master's stock wording widens a pattern far past its fact.
- Decision (implement): mutation controls run by a scratch script that saves bytes, applies an exact-match revert
  (asserted to match once), requires a FAILED test line (never a compile error) for red, restores, compares sha256 and
  reads green — eight rows, plus the two F2 re-runs after the corrections.

## Outcome
- Acceptance criteria, against the diff:
  - (v020-03) **UNMET as yet — gated**: the nine answers stand as the operator's, PROVISIONAL; the founder's own word
    has not been recorded (`evidence/founder-word.md` absent). The rest is met: the seven fixed debts each proven in both
    layout modes and red with its fix reverted (`evidence/controls.md`, 8 rows), each merge surface named
    (`evidence/merge-surface.md`, 7 rows), F3 pinned by `accessibility_active_descendant_unread` and F9 by `covered`'s
    meaning and `stand_act_ids::refused_at_boot`, none kept. Owner: P7's gated arm (P5 re-checks the block).
  - (a11y · F1) met — `inert_subtree` 4 passed, both layout modes.
  - (a11y · F7) met — `stand_keyboard` 14 passed; `bash .github/scripts/ci-leg.sh a11y` green.
  - (a11y · F2) met — the five roles' `aria-checked` read as `toggled` and the snapshot's `checked`; a toggle button
    still `pressed`; `accessibility_states` 5 passed; and, on the operator's word, a native checkbox or radio reads
    neither attribute.
  - (security · F2) met — the 5,500-character value nowhere in the tree's `Debug`; the census entry reads 0.
  - (security · F6) met — `password_caret_click` 2 passed, the two fillings one index.
  - (arch · F4) met — (40, 50) at every scroll offset; `scroll_into_view_nested` 7.
  - (design · F5) met — `hit_clipped_at_scrolling_box` 13 passed.
  - (arch · F8) met — `transformed_client_rect` 4 and `stand_act_transformed` 2, per package and in the workspace leg.
  - (arch · F5, F9) met — `covered`'s meaning holds the one-axis and no-area sentences, not the `contain: paint` one, in
    `refusal.rs`, `json.rs`'s test and `lib.rs`; the cause set nine; escher-driver 74 passed.
  - (tests · CARRY) met — `accessibility_merged_states` 3 passed; dioxus-native-dom lib 72 passed.
  - (obs) met — the silence probe reads 0.
  - (tests) met — `stand-repeat: 3 of 3 rounds green`; `coverage floor: 3 of 3 crates at or above their floors`.
  - (security) met — no manifest, lockfile or `deny.toml` moved; audit green.
  - (tests) met — `ci-leg.sh fast` and `ci-leg.sh doc` green; CI on the pushed HEAD `verdict: green`.
- Gates, by `run` (verdicts from implement's trail `.andromeda/runs/2026-10-18T03-17-10-implement/gate-2026-10-18-engine-debts.json`,
  each entry's last firing; after the second F2 correction the re-fired set was the check, the probes, a11y, coverage
  and fast):
  - `cargo test -p blitz-tests --locked --test accessibility_merged_states` — green · `contains test result: ok. 3 passed; 0 failed`
  - `… --test inert_subtree` — green · 4 passed
  - `… --test stand_keyboard` — green · 14 passed
  - `… --test accessibility_states` — green · 5 passed (re-fired after each correction)
  - `cargo test -p dioxus-native-dom --locked --lib` — green · 72 passed
  - `cargo test -p escher-driver --locked --lib` — green · 74 passed
  - `… --test accessibility_active_descendant_unread` — green · 1 passed
  - `… --test scrolled_box_absolute_position` — green · 1 passed
  - `… --test scroll_into_view_nested` — green · 7 passed
  - `… --test hit_clipped_at_scrolling_box` — green · 13 passed
  - `… --test transformed_client_rect` — green · 4 passed
  - `… --test stand_act_transformed` — green · 2 passed
  - `… --test stand_act_obstructed` — green · 6 passed
  - `… --test password_caret_click` — green · 2 passed
  - `grep -c -E '^\| [a-z_]+ \| [a-z_]+ \| red \| …' …/evidence/controls.md` — green · `last line 8`
  - `grep -c -E '^\| F[124-8] \|.*23b9b26a9.*no upstream PR.*…' …/evidence/merge-surface.md` — green · `last line 7`
  - `git diff -U0 34bcf71c… -- packages/blitz-dom/src packages/dioxus-native-dom/src/snapshot.rs | … grep -c …` (the census) — green · `exit 1`, `last line 0`
  - `grep -c -E 'println!|…' {the ten checks and common/mod.rs} | grep -c -v ':0$'` (the silence probe) — green · `exit 1`, `last line 0`
  - `git diff --quiet 23b9b26a9… -- packages/dioxus-native-dom/src/mutation_writer.rs` — green · exit 0
  - `git diff --quiet 34bcf71c… -- Cargo.toml Cargo.lock deny.toml …` — green · exit 0
  - `git diff --name-only 34bcf71c… -- packages examples apps wpt scripts .github | grep -c -v -x -E …` — green · `exit 1`, `last line 0`
  - `bash .github/scripts/ci-leg.sh a11y` — green · exit 0
  - `bash .github/scripts/ci-leg.sh stand-repeat` — green · `stand-repeat: 3 of 3 rounds green` (268.48 s, after the first correction; the second touched only the builder's toggle branch and its check, re-proven by the a11y, fast and coverage legs and by CI's «Stand checks repeated» on the pushed sha)
  - `bash .github/scripts/ci-leg.sh coverage` — green · `coverage floor: 3 of 3 crates at or above their floors` (escher-driver 98.19 %, dioxus-native-dom 84.57 %, blitz-test-harness 91.69 %)
  - `bash .github/scripts/ci-leg.sh audit` — green · exit 0
  - `grep -c -E '^\| F[1-9] \| [AB] \| ratified by the founder \| …' …/evidence/founder-word.md` — `not run — defer (key)`: `gated: the founder's own word discharging this chunk's PROVISIONAL batch at its wrap …` — no PREREQ; the gated record carries it
  - `bash .github/scripts/ci-leg.sh fast` — green · exit 0 (re-fired after each correction)
  - `bash .github/scripts/ci-leg.sh doc` — green · exit 0 (last fired after the first correction; the second changed a `//` comment and a check, no doc text; CI's «Documentation» green on the pushed sha)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg operator`: green · `hygiene: clean — read 54 …` (`evidence/operator-pass.md`)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.2.0` — `leg operator`: green · exit 0 · `history moved: refs/remotes/origin/build/escher-0.2.0 34bcf71c→48680dfb` (the intended push)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — `leg operator`: green · `48680dfb98f3 verdict: green · checks 17/17 · wall 928 s · runs CI#39096326308 completed/success`
  - smoke: skipped — no boot path changed (implement P3).
- Watches: none folded.
- Outcome basis: the operator pass ran — the gate verdicts rest on its final state: Setup 4's commit list (one pre-CI
  commit, `48680dfb`, no fix commit) and the final HEAD's CI run CI#39096326308 green 17/17 (`evidence/operator-pass.md`).
  Implement's report, `.andromeda/runs/2026-10-18T03-17-10-implement/outcome-2026-10-18-engine-debts.md` (the one
  implement run dir), as rewritten after both operator corrections; this window held that run whole.
- Process hygiene: implement P4's census — `census: none running` for cargo · rustc · escher-session · `blitz-tests`
  (`gate.py census`, implement's run dir), re-measured at this wrap's P1: `census: none running`. At the operator pass's
  end one `cargo check --workspace --message-format=json --keep-going --all-targets` stood, started by no run of this
  chunk — an editor's rust-analyzer by its command line — gone by this re-measure.
## Addendum — resumed 2026-10-18
- inputs#I7 · `.andromeda/runs/2026-10-18T04-30-47-wrap/relay-3.md` · copy message · n/a — the operator's directive at
  the resume: (a) the founder is away; at the Provisional discharge card the wrap does not wait for him, lists the batch
  (the nine delegate answers, the seven engine fixes, the `switch` mapping) as standing PROVISIONAL for his word, takes
  the gated arm for v020-03 (master record `gated`) and goes on to the commit; (b) the §Apply classification step is
  reported at the end.
  - bears on **Decisions & corrections**: adds to it — the «For P7» gated arm is the operator's direction, not a
    fallback; the founder's word is not waited on.
  - bears on **Outcome**: adds to it — (v020-03) stays UNMET as yet, gated, its owner P7's gated arm.
- inputs.py verify, re-fired: `inputs: 7 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 7 · uncited 1 · unparsed 0`; moved since the report: uncited 3 → 1 (I4 · I5 · I6 cited by the report, I7 new and cited here)

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 34bcf71c (the parent of the oldest pre-CI commit 48680dfb) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### packages/blitz-dom/src/accessibility.rs — added 49 line(s) in 6 range(s)
added: 171-175 · 177-184 · 243 · 245-256 · 330 · 378-399
- 378-388 @379 «fn is_checkable(role: Role) -> bool {»
  - 380-387 «matches!(»
- 390-398 @392 «fn is_native_checkable(element: &ElementData) -> bool {»
  - 393-397 «element.name.local == local_name!("input")»
### packages/blitz-dom/src/document.rs — added 107 line(s) in 4 range(s)
added: 2376-2378 · 2387-2396 · 2405-2495 · 2565-2567
  - 2405-2452 @2410 «fn transformed_bounds(&self, node_id: NodeId, rect: kurbo::Rect) -> Option<kurbo::Rect> {»
  - 2454-2458 @2455 «fn is_transformed_chain(&self, node_id: NodeId) -> bool {»
  - 2460-2494 @2465 «fn transformed_fragment_rects(&self, node_id: NodeId) -> Option<Vec<BoundingRect>> {»
### packages/blitz-dom/src/events/keyboard.rs — added 2 line(s) in 2 range(s)
added: 163 · 180
### packages/blitz-dom/src/events/pointer.rs — added 93 line(s) in 9 range(s)
added: 330 · 448-449 · 463 · 498-501 · 528 · 540-542 · 562-573 · 586-653 · 764
- 586-652 @590 «fn click_password_input(»
  - 600-603 «let Some(editor) = node»
  - 604-606 «else {»
  - 610-614 «let content_width = (frame.size.width»
  - 617-619 «let Some(layout) = doc.layout_as_text_input(node_id, &bullets) else {»
  - 621-623 «let caret =»
  - 625-629 «let under = parley::Cursor::from_point(»
  - 631-634 «let byte = text»
  - 636-638 «let Some(data) = doc.nodes[node_id]»
  - 639-641 «else {»
  - 642-645 «let mut font_ctx = doc»
  - 647-651 «match click_count {»
### packages/blitz-dom/src/node/element.rs — added 7 line(s) in 2 range(s)
added: 490-495 · 709
  - 490-494 @492 «pub fn is_disabled(&self) -> bool {»
### packages/blitz-dom/src/node/node.rs — added 60 line(s) in 9 range(s)
added: 558-575 · 1401-1410 · 1501-1505 · 1507-1511 · 1514 · 1516 · 1717-1719 · 1721-1724 · 1726-1738
### packages/blitz-dom/src/scrolling.rs — added 1 line(s) in 1 range(s)
added: 719
### packages/dioxus-native-dom/src/snapshot.rs — added 87 line(s) in 8 range(s)
added: 53-57 · 67-68 · 100 · 150 · 163-169 · 172-175 · 213-225 · 785-838
- 213-224 @215 «fn is_checkable(role: Role) -> bool {»
  - 216-223 «matches!(»
  - 785-809 @786 «fn an_aria_disabled_button_still_reads_enabled() {»
  - 811-823 @812 «fn a_switch_reads_its_aria_checked_as_checked_and_not_pressed() {»
  - 825-837 @826 «fn a_toggle_button_still_reads_its_aria_pressed_as_pressed() {»
### packages/escher-driver/src/json.rs — added 4 line(s) in 1 range(s)
added: 678-681
### packages/escher-driver/src/lib.rs — added 6 line(s) in 2 range(s)
added: 68-69 · 71-74
### packages/escher-driver/src/refusal.rs — added 23 line(s) in 6 range(s)
added: 20-24 · 27-28 · 79-83 · 87-89 · 269-273 · 279-281
### packages/escher-driver/src/schema.rs — added 6 line(s) in 2 range(s)
added: 74-77 · 93-94
### tests/blitz-tests/tests/accessibility_active_descendant_unread.rs — new file · 81 line(s)
- 18-23 «const LIST: &str = r#"<html><body>»
  - 19-22 «<div id="list" role="listbox" aria-label="People" tabindex="0" aria-activedescendant="second">»
- 25-37 «fn page(html: &str, incremental: bool) -> HtmlDocument {»
  - 26-33 «let mut doc = HtmlDocument::from_html(»
- 39-81 @40 «fn a_focused_listbox_reads_focus_and_no_active_descendant() {»
  - 41-80 «for incremental in [false, true] {»
### tests/blitz-tests/tests/accessibility_merged_states.rs — new file · 157 line(s)
- 22-34 «fn page(html: &str, incremental: bool) -> HtmlDocument {»
  - 23-30 «let mut doc = HtmlDocument::from_html(»
- 36-50 @38 «fn carriers(attribute: &str, open: &str, close: &str, values: &[Option<&str>]) -> String {»
  - 39-48 «let rows: String = values»
- 52-63 @54 «fn read<'t>(doc: &HtmlDocument, tree: &'t TreeUpdate, id: &str) -> &'t Node {»
  - 55-57 «let node_id = doc»
  - 58-62 «tree.nodes»
- 65-86 @68 «fn assert_rows<T: PartialEq + std::fmt::Debug>(»
  - 74-85 «for incremental in [false, true] {»
- 88-104 @89 «fn aria_expanded_reads_true_and_false_and_nothing_else() {»
  - 90-98 «let rows = [»
- 106-121 @107 «fn aria_disabled_sets_disabled_for_true_alone() {»
  - 108-115 «let rows = [»
- 123-157 @124 «fn aria_level_reads_its_number_less_one_and_the_heading_defaults() {»
  - 125-132 «let rows = [»
  - 139-147 «let defaults = [»
  - 149-153 «let html: String = defaults»
### tests/blitz-tests/tests/accessibility_states.rs — added 127 line(s) in 11 range(s)
added: 1-4 · 10 · 12-18 · 22 · 194-220 · 222-224 · 226 · 229 · 231-234 · 236-240 · 244-316
- 194-209 @195 «const CHECKABLE: [Supporting; 5] = [»
  - 199-203 «(»
  - 204-208 «(»
- 245-299 @246 «fn aria_checked_reads_its_three_values_on_a_checkable_role_alone() {»
  - 248-256 «let rows = [»
  - 261-264 «let unread: Vec<_> = rows»
  - 267-275 @270 «let natives = [»
  - 278-298 «for incremental in [false, true] {»
- 301-316 @302 «fn aria_pressed_on_a_checkable_role_reads_nothing_and_a_long_aria_checked_is_nowhere() {»
  - 306-310 «let rows = [»
### tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs — added 26 line(s) in 13 range(s)
added: 12-18 · 374-377 · 379 · 396-397 · 535-536 · 542 · 544-545 · 554 · 560 · 564 · 568 · 576-577 · 580
### tests/blitz-tests/tests/inert_subtree.rs — new file · 194 line(s)
- 17-26 @18 «const FIXTURE: &str = r#"<html><body style="margin:0">»
  - 21-24 «<div id="shield" inert style="position:absolute; left:0; top:30px; width:400px; height:100px">»
- 28-36 «fn boot(incremental: bool) -> Harness<HtmlDocument> {»
  - 29-35 «Harness::from_html_with(»
- 38-40 «fn inert() -> QualName {»
- 42-45 «fn tab(harness: &mut Harness<HtmlDocument>) -> Option<NodeId> {»
- 47-50 «fn shift_tab(harness: &mut Harness<HtmlDocument>) -> Option<NodeId> {»
- 52-60 @53 «fn in_tree(harness: &Harness<HtmlDocument>, node: NodeId) -> bool {»
  - 54-59 «harness»
- 62-66 @63 «fn hit_at_centre(harness: &Harness<HtmlDocument>, selector: &str) -> Option<NodeId> {»
- 68-93 @69 «fn tab_and_shift_tab_skip_an_inert_subtree() {»
  - 70-92 «for incremental in [false, true] {»
- 95-122 @96 «fn a_hit_passes_through_an_inert_subtree() {»
  - 97-121 «for incremental in [false, true] {»
- 124-142 @125 «fn the_tree_holds_no_node_for_an_inert_subtree() {»
  - 126-141 «for incremental in [false, true] {»
- 144-194 @145 «fn removing_inert_restores_focus_hit_and_tree() {»
  - 146-193 «for incremental in [false, true] {»
### tests/blitz-tests/tests/password_caret_click.rs — added 82 line(s) in 12 range(s)
added: 1-9 · 23-24 · 26-28 · 37-38 · 43-80 · 84 · 87 · 89-91 · 93-94 · 96 · 99-101 · 105-121
  - 43-47 «(»
  - 55-58 «assert!(»
  - 61-64 «assert!(»
  - 68-75 «let (left, layout) = {»
  - 76-79 «assert!(»
- 106-121 @107 «fn a_double_click_selects_the_whole_value() {»
  - 108-120 «for incremental in [false, true] {»
### tests/blitz-tests/tests/scrolled_box_absolute_position.rs — added 39 line(s) in 5 range(s)
added: 1-10 · 22 · 26-28 · 53 · 57-80
### tests/blitz-tests/tests/stand_act_transformed.rs — new file · 127 line(s)
- 22-40 @23 «fn transformed_fixture() -> Element {»
  - 25-39 «rsx! {»
- 42-50 «fn hold(incremental: bool) -> Session {»
  - 43-48 «Session::start("transformed", || {»
- 52-60 @53 «fn bounds(session: &Session, id: &str) -> (f64, f64, f64, f64) {»
  - 55-58 «let bounds = snapshot»
- 62-85 @65 «fn assert_the_button_is_moved(session: &Session, mode: &str) {»
  - 68-74 «assert!(»
  - 75-78 «assert!(»
  - 81-84 «assert!(»
- 87-110 @88 «fn a_click_on_a_translated_button_is_accepted_and_lands() {»
  - 89-109 «for incremental in [false, true] {»
- 112-127 @113 «fn the_snapshot_bounds_of_a_translated_button_read_where_it_shows() {»
  - 114-126 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_keyboard.rs — added 31 line(s) in 8 range(s)
added: 17-24 · 618 · 638-642 · 645-650 · 655 · 756 · 767-769 · 774-779
### tests/blitz-tests/tests/transformed_client_rect.rs — new file · 220 line(s)
- 20-22 «fn page(html: &str, incremental: bool) -> Harness<HtmlDocument> {»
- 24-32 @25 «fn one_box(style: &str) -> String {»
  - 26-27 «format!(»
- 34-45 @36 «fn parent_and_child(style: &str) -> String {»
  - 37-38 «format!(»
  - 39-42 «<div id="parent" style="width:300px; height:200px; {style}">»
- 47-59 «fn rect(harness: &Harness<HtmlDocument>, selector: &str) -> (f64, f64, f64, f64) {»
  - 49-57 «let BoundingRect {»
- 61-73 «fn offset_rect(harness: &Harness<HtmlDocument>, selector: &str) -> (f64, f64, f64, f64) {»
  - 63-71 «let BoundingRect {»
- 75-77 «fn moved((x, y, width, height): (f64, f64, f64, f64), by: (f64, f64)) -> (f64, f64, f64, f64) {»
- 79-94 @80 «fn a_translated_box_reads_its_rect_moved_by_the_translation() {»
  - 81-93 «for incremental in [false, true] {»
- 96-114 @97 «fn a_scaled_box_reads_its_scaled_box_about_its_centre() {»
  - 98-113 «for incremental in [false, true] {»
- 116-153 @117 «fn a_child_of_a_translated_parent_moves_with_it_and_its_offset_rect_does_not() {»
  - 118-152 «for incremental in [false, true] {»
- 155-166 @157 «fn scrolling_box(style: &str) -> String {»
  - 158-159 «format!(»
  - 160-163 «<div id="scroller" style="width:300px; height:100px; overflow:auto; {style}">»
- 168-170 «fn centre((x, y, width, height): (f64, f64, f64, f64)) -> (f64, f64) {»
- 172-177 «fn region(harness: &Harness<HtmlDocument>, selector: &str) -> Option<(f64, f64, f64, f64)> {»
  - 173-176 «harness»
- 179-181 «fn inside(point: (f64, f64), (x, y, width, height): (f64, f64, f64, f64)) -> bool {»
- 183-220 @186 «fn a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region() {»
  - 187-219 «for incremental in [false, true] {»
