# Report — 2026-10-13-stand-keyboard-harness

**Chunk:** Stand keyboard harness — Tab, Shift+Tab and activation keys dispatched headlessly on the stand, the focused node read back per step
**Date:** 2026-10-13
**Commits:** since `last_wrap` 2026-10-13T21:44:39Z — `71af33d06 chore(2026-10-13-stand-keyboard-harness): operator pre-CI commit, for the run this chunk verdict reads` (`git log --format='%h %s' e40d9437..HEAD`)

## Changes (structured — detectors read this)
- **Files:** 18 changed from `e40d9437` (the parent of the pre-CI commit `71af33d0`), every one listed by research or
  absorbed by its derived `tests/blitz-tests/tests/stand_*.rs` row (`gate.py scope`, below):
  - engine: `packages/blitz-dom/src/node/element.rs`, `packages/blitz-dom/src/events/keyboard.rs`,
    `packages/blitz-dom/src/events/mod.rs`, `packages/blitz-dom/src/events/pointer.rs`;
  - bridge: `packages/dioxus-native-dom/src/snapshot.rs` (unit test only);
  - stand: `examples/seven_guis/src/tasks/crud.rs`;
  - checks: new `tests/blitz-tests/tests/stand_keyboard.rs` (618 lines);
  - modified checks: `tests/blitz-tests/tests/{common/mod.rs, stand_accessibility_ids.rs, stand_snapshot_state.rs,
    focusability_updates.rs, dioxus_falsy_boolean_attrs.rs, stand_act_keys.rs, stand_act_obstructed.rs,
    stand_act_diff.rs, stand_act_ids.rs, stand_act_spans.rs, stand_diff.rs}`.
- **Symbols / APIs:** (coordinates from `## New text, by line`)
  - **F3 — focusability by presence, in every document** (founder, «По наличию (Рекомендую)», inputs#I3):
    - `ElementData::flush_is_focussable` reads `disabled` by presence
      (`self.has_attr(local_name!("disabled"))`, packages/blitz-dom/src/node/element.rs:703), where it parsed a
      boolean through `attr_parsed`. Any `disabled` attribute — bare, `"false"`, `"true"` — on ANY element now removes
      focusability.
    - Known limit, pinned (operator's condition, inputs#I5): a `<div disabled tabindex=0>` is no longer focusable,
      where browsers apply `disabled` to form controls only and focus it.
    - Readers that move with it (research §Graph impact): the Tab walk `focus_next_node` / `focus_prev_node`;
      autofocus `mutator.rs` (a disabled-attributed autofocus candidate is now skipped);
      `DioxusDocument::unkeyed_actionable` (`actionable.rs`).
    - The recompute on a `disabled` write is unchanged.
  - **F1 — keyboard activation, in every document** (founder, «Клавиши нажимают (Рекомендую)», inputs#I3):
    - In `handle_key_or_input_event` (packages/blitz-dom/src/events/keyboard.rs:118-123): a pressed `Key::Enter`
      whose target is the focused node, after the text-input arm and the range arm, dispatches
      `DomEvent::new(node, DomEventData::Click(..))` when `activates_on_enter` holds. It holds for an element with no
      `disabled` attribute (presence) that is a `button`, a `summary`, an `input` of type `submit`, `reset` or
      `button`, or an `a` with `href` (keyboard.rs:150-165).
    - New `pub(crate) fn handle_key_up` (keyboard.rs:128-148): a released `Key::Character(" ")` targeting the
      focused node dispatches the same Click when `activates_on_space` holds (keyboard.rs:167-181). It holds for an
      element with no `disabled` attribute that is a `button`, a `summary`, or an `input` of type `submit`, `reset`,
      `button`, `checkbox` or `radio`. A link takes no Space; a text input and a range input take neither key here.
    - `dispatch_activation_click` (keyboard.rs:183-193) builds the event data from
      `Node::synthetic_click_event_data(modifiers)`. The Click is queued by the event driver, so the app's handlers
      (a Dioxus `onclick`) run, then its default action `handle_click`.
    - `events/mod.rs` routes `DomEventData::KeyUp` to `handle_key_up` (packages/blitz-dom/src/events/mod.rs:12,
      :228-229). KeyUp had no default action before.
    - `synthetic_click_event_data` keeps its other callers (`handle_click`'s label arm, `Node::synthetic_click_event`);
      `absolute_position` is untouched (F4).
  - **F2 — a click focuses a button, in every document** (founder, «Клик даёт фокус (Рекомендую)», inputs#I3):
    - `handle_click`'s submit arm (packages/blitz-dom/src/events/pointer.rs:800-821) became a button arm. It matches
      a `button` (any type) or an `input` of type `submit`, `reset` or `button`.
    - It focuses the control through `generate_focus_events` / `set_focus_to`, then submits the form owner when
      `is_submit_button()` or `type == "submit"`, and ends the walk matched.
    - Before: a submit button submitted without breaking the walk, and every other button fell through to the
      unmatched clear of focus.
    - A `disabled` attribute still stops the walk first. A click on any other element is unchanged: a CRUD row or the
      list still clears focus.
  - **F5 — the CRUD list takes focus** (operator, "List takes focus", inputs#I4), stand markup only
    (examples/seven_guis/src/tasks/crud.rs:59-86, :91, :172-178):
    - `crud-list` gains `tabindex: "0"` and an `onkeydown`. ArrowDown selects the next visible row (the first when none
      is selected); ArrowUp the previous; Home the first; End the last. A key at the end of the list changes nothing,
      and so does ArrowUp with no row selected.
    - Each selection sets `selected`, `first_field` and `last_field` as a row's `onclick` does.
    - The filter predicate moved into a private `fn shown(person, filter_lower)` read by the render and the handler.
    - No class, style, element, wrapper or row change.
  - **Tests' shared module** (tests/blitz-tests/tests/common/mod.rs:48-70, :72-92):
    - `pub fn tab_order(task)` moved from `stand_accessibility_ids.rs`. CRUD restated to `back-btn`, `crud-filter`,
      `crud-list`, `crud-name`, `crud-surname`, `crud-create` (6 stops, was 5).
    - `pub fn focused(harness)` moved from `stand_snapshot_state.rs`.
    - The module now imports `blitz_dom::Document`.
- **Crates / modules:** none added or removed. Changed: blitz-dom (`node/element.rs`, `events/{keyboard,mod,pointer}.rs`),
  dioxus-native-dom (a unit test in `snapshot.rs`), seven_guis (`tasks/crud.rs`), blitz-tests (one new target).
- **Dependencies:** none — gate `git diff e40d9437… -- Cargo.lock | grep -c '^+\[\[package\]\]'` reads 0.
- **Schema / config:** none. No driver schema, no `KEY_NAMES` change: `enter`, `space`, `arrow-*`, `home` and `end` were
  already in it (packages/escher-driver/src/schema.rs:14-27, unchanged — gate `git diff --quiet … packages/escher-driver …`).
- **Spec-master edits:** none (the masters are read-only to implement; this wrap's P2 owns them).
- **Counts / qualifiers moved:**
  - Workspace tests: 181 result lines · 915 passed · 0 failed · 12 ignored → 182 · 928 · 0 · 12. The +13 passed are
    `stand_keyboard` 11, `stand_act_keys` +1 and `focusability_updates` +1. Basis: `target/ci-logs/test.log` of
    implement's `ci-leg.sh fast` run, parsed for `test result:` lines. Master site: ws-counts pattern, standing 2 —
    test-plan.md:330 (the 181 · 915 figure), stale.
  - `run stand`: run.end passed 141 → 153 · failed 0 · ignored 7 (`target/agent-run/events.jsonl` run.end of
    2026-10-13T23:02:26Z). 37 → 38 `stand_*` files (`ls tests/blitz-tests/tests/stand_*.rs`).
    Master site: test-plan.md:116 (ws-counts), stale.
  - blitz-tests: 120 `.rs` files, 119 targets → 121 · 120 (`ls tests/blitz-tests/tests/*.rs`; `all.rs` unbuilt).
    Master site: bt-files pattern, standing 1 — test-plan.md:66, stale.
  - `common` readers: 18 → 19, `stand_keyboard` the nineteenth. Basis: `mod common;` counted over
    `git ls-tree HEAD~` vs the tree. Master site: obs-plan.md:70 "18 of which read the shared module …
    `stand_contrast` the eighteenth", stale. Also test-plan.md:100's reader list and table list (`tab_order` and
    `focused` now in `common`); its reader enumeration and helper list are stale.
  - CRUD Tab sequence 5 → 6 stops (`crud-list` third). Master sites: tab-order pattern, standing 5 —
    test-plan.md:26 @c3973 ("the Tab order unchanged", still true of `stand_accessibility_ids` for the three other
    tasks, restated for CRUD), test-plan.md:100 (`tab_order` named as `stand_accessibility_ids`' own), a11y-plan.md:212,
    :278, :324 (true claims about other checks; :324 names the forward-sequence assertion — restated CRUD).
  - `dioxus_falsy_boolean_attrs` names 27 → 28 (`inert` added; `BOOLEAN_ATTRS: [&str; 28]`). Master sites: inert
    pattern, standing 8 — architecture.md:263 and test-plan.md:26 @c3032 say "whether a check reads a falsy `inert` is
    not measured", stale; architecture.md:136 @c1179 and security-plan.md:143 describe the bridge's 28 names, true;
    test-plan.md:24, test-plan.md:183 and obs-plan.md:70 @c8614 use "inert" in another sense (no-change).
  - `stand_act_ids`: the controls changing the screen at boot go 9 → 15 control rows, every accepted control, since a
    click focuses a button (tests/blitz-tests/tests/stand_act_ids.rs:243-244).
  - `stand_diff`: Book's 8 changed nodes now read 7 bounds-only plus `flight-book` `[Focused, Bounds]`.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none — no agent-run script, CI step or status shape changed. `stand_keyboard` joins `run
  stand` by its `stand_` prefix.
- **Cross-project / external claims:**
  - CI on the pushed sha `71af33d06f0a782469c7589c5ff88245af780bbe`: CI#38482525151 push, completed/success, 16/16
    checks, wall 754 s. `Test (windows)`, `Test (macos)`, `MSRV Build [Rust 1.91]`, `Test (ios)` and
    `Test (android)` are green (escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/evidence/ci.md).
  - `inputs.py verify` at this wrap (inputs: 6 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · n/a 5 ·
    uncited 1 · unparsed 0):
    - `I1 · message: the operator, /andromeda-phase invocation arguments, 2026-10-13T22:07Z · copy message · n/a`
      (cited by scope.md, research.md, plan.md)
    - `I2 · message: the operator relaying the overseer, P4 question dialog answer, 2026-10-13T22:33Z · copy message ·
      n/a` (cited plan.md:8)
    - `I3 · ../additional/escher-overseer/relays/keyboard-harness-founder-answers.md · copy no-repo · unchanged`
      (cited by research.md and plan.md; the founder's F1–F4 labels)
    - `I4 · message: the operator, P4 question dialog answer to fork F5, 2026-10-13T22:36Z · copy message · n/a`
    - `I5 · message: the operator, P5 review of .andromeda/runs/2026-10-13T22-07-35-phase/p5-card-1.md,
      2026-10-13T22:46Z · copy message · n/a`. It names card 1, which is **superseded** by I6's card 2: card 2 folds
      I5 and carries its rows 1–4.
    - `I6 · message: the operator, P5 word on .andromeda/runs/2026-10-13T22-07-35-phase/p5-card-2.md,
      2026-10-13T22:47Z · copy message · n/a`, cited here as inputs#I6 (`verify` printed it UNCITED before this report
      existed). The word: "yes — apply. The div pin row is kept, and row 5 is approved." Authority: the operator,
      2026-10-13, given here. Card 2's `## Beyond the plan` rows, verbatim (rows 1–4 approved on card 1, inputs#I5;
      row 5 approved by inputs#I6):

      | # | proposal | where |
      |---|---|---|
      | 1 | New CARRY: the CRUD list takes focus but the tree's focus is the list, not the row. The accessibility builder reads no `aria-activedescendant`, and a click on a row or on the list still leaves nothing focused (F2 covers buttons only). This is your F5 condition, inputs#I4. | "Stand a11y assertions" (escher-0.1.0/working-route.md:94), beside the listbox-name CARRY |
      | 2 | The CARRY "the second position reader reads a scrolled box shifted" moves off this entry, by the founder's F4 word «Отложить (Рекомендую)». Its hypothesis is restated: an activation key reaches the self-scrolled node, and only the click event's coordinates are off. | "Stand requirement sweep" (escher-0.1.0/working-route.md:96) |
      | 3 | New CARRY: the engine honours no `inert`. A truthy `inert` neither removes focusability nor stops a hit. Pinned by `stand_keyboard`; a fix is an engine change for every document. | "Stand a11y assertions" (escher-0.1.0/working-route.md:94) |
      | 4 | New CARRY: the merge surface of the three ratified engine changes, against 434e0661 (`events/keyboard.rs`, `events/mod.rs`, `events/pointer.rs`, `node/element.rs`). This follows the range model's precedent. | "Quality gates" (escher-0.1.0/working-route.md:98) |
      | 5 | New CARRY: narrow F3's presence reading to the form controls `disabled` applies to (`button`, `input`, `select`, `textarea`). Until then a `<div disabled tabindex=0>` is unfocusable, unlike in browsers, which is the known limit pinned by `stand_keyboard`. This is your condition, inputs#I5. | "Stand requirement sweep" (escher-0.1.0/working-route.md:96) |

      P5 takes these five as recorded directions for its route edits.
  - The upstream base `434e0661` (DioxusLabs/blitz) is named as the merge surface of F1–F3. Not read this chunk.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):**
  - F5 makes the CRUD list keyboard-operable, yet the tree's focus is the list, never the option. The engine's
    accessibility builder reads no `aria-activedescendant`, so an assistive technology learns the current row only
    from its `selected` reading.
  - Owner: card 2 row 1, a CARRY on "Stand a11y assertions".
- **Spec claims disproved by measurement:**
  - **"Focusability parses `disabled` as a boolean; `disabled="false"` and a bare `disabled=""` stay focusable."**
    Measured false after F3: `stand_keyboard::a_disabled_attribute_of_any_value_is_skipped`, the restated
    `stand_snapshot_state` parsed-markup pin, `snapshot.rs` unit pin
    `a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value` and
    `focusability_updates::a_bare_disabled_set_after_creation_takes_the_focusability_away`, all green.
    Sites (disabled-focus pattern, standing 9), each stale:
    - architecture.md:91 @c377/@c684 ([DOM semantics] "`disabled` is read two ways — focusability parses it as a
      boolean …")
    - architecture.md:136 @c14297 (snapshot `enabled` "… while the engine's focusability reader still takes both as
      focusable")
    - security-plan.md:118 (the `attr_parsed` row "as for `tabindex` and `disabled`" — `disabled` no longer;
      `tabindex` still)
    - security-plan.md:119 (the `disabled` row "Parsed as a boolean value for focusability")
    - a11y-plan.md:144 (the focusability definition "does not parse as `true`")
    - a11y-plan.md:148 ("focusability on its value PARSED as a bool …")
    - a11y-plan.md:279 @c2370 ("a `disabled="false"` and a bare `disabled` read not enabled while both stay
      focusable")
    - test-plan.md:26 @c4516 and test-plan.md:99 describe the pins' fixtures and stay true as fixture statements;
      their "focusable" reading, where stated, is stale.
  - **"A pointer click on a plain `button` leaves no element focused"** — a11y-plan.md:158 (click-focus pattern,
    standing 2; the other hit, :155, is its heading). Measured false after F2:
    `stand_keyboard::a_click_focuses_a_stand_button_and_tab_continues_from_it`, and restated
    `stand_act_obstructed` (after an accepted click `fx-before` reads focused and is the tree's focus).
  - **"No key activates a stand button" / SC 2.1.1 unmet on stand buttons** (scope.md, P3 probe
    `.andromeda/runs/2026-10-13T22-07-35-phase/probe-counter.txt`): measured false after F1 —
    `stand_keyboard::enter_and_space_on_a_focused_button_read_as_its_click` (20 rows) and
    `stand_act_keys::enter_and_space_on_a_focused_button_count_as_its_click_does`. Master sites: activation pattern
    — control UNFIRED over the seven masters (`.andromeda/runs/2026-10-13T23-23-45-wrap/sweep-refusal.txt`). No
    master states it in those words; the a11y-plan §5 Keyboard handling list (a11y-plan.md:163-174) states no button
    activation (an absence to fill).
  - **"A framework-written `inert: false` is now cleared, so it no longer makes content inert"** — inert-claim
    pattern, standing 2: architecture.md:136 @c1749 and a11y-plan.md:185. Measured: the engine implements no `inert`,
    so a truthy `inert` makes nothing inert either — `stand_keyboard::the_engine_honours_no_inert` (Tab reaches the
    buttons under `inert: true` and `inert: false`). The attribute half of the claim holds
    (`dioxus_falsy_boolean_attrs`, 28 names). The "no longer makes content inert" half is false: it never did.
- **Expected amendments (from plan):** (patterns in `.andromeda/runs/2026-10-13T23-23-45-wrap/report-patterns.toml`;
  counts from `sweep-pre-pass*.txt` there)
  - **architecture §Established Decisions → [Scrolling and input] + §Standard Contracts → Events** (three direct engine
    changes; KeyUp gains a default action) — carried: Symbols / APIs F1, F2, F3. Search:
    - scroll-input standing 4 in architecture (architecture.md:87 the decision, :89, :95, :123);
    - engine-change standing 5 (architecture.md:87 ×2, :95; security-plan.md:126, :127);
    - keyup standing 2, both a11y-plan (a11y-plan.md:172, :180), 0 in architecture — KeyUp's default action is new
      text in architecture;
    - disabled-focus in architecture: :91, :136 (stale, above).
  - **security-plan §Input Validation** (a keyboard-activation row for every document, remote pages included, gated on
    `disabled` presence; the `disabled` row's split closed by F3) — carried: Symbols / APIs F1, F3, Spec claims
    disproved. Search:
    - disabled-focus standing 2 in security-plan (security-plan.md:118, :119);
    - engine-change security-plan.md:126, :127 (the range rows, precedent for the new row);
    - activation 0 (unfired control).
    - The founder answered F1 himself («Клавиши нажимают (Рекомендую)», inputs#I3); the plan names the Boundary
      widening pattern (playbook :36), so Validate stops once on the answer card.
  - **a11y-plan §5 Focus order / Focus restoration / Keyboard handling / Per-surface keyboard shortcuts / Script and
    framework exposure / Test harness pattern; §8 the restated CRUD sequence** — carried: F1–F3, F5, Spec claims
    disproved, Counts. Search:
    - disabled-focus a11y-plan.md:144, :148, :279;
    - click-focus :155, :158;
    - keyup :172, :180 (true, no-change);
    - inert-claim :185;
    - tab-order :212, :278, :324;
    - the `<div disabled tabindex=0>` limit text per inputs#I5 — tabindex pattern 15 standing in a11y-plan and
      security-plan, design-system:416 (other sense).
    - No hit names the CRUD list's keys (crud-tab in a11y-plan: :227, the contrast record, no-change).
  - **test-plan §1, §3, §4** (the new target, the restated pins, the file/target/`common` reader/`run stand` counts) —
    carried: Counts / qualifiers moved, Files. Search:
    - ws-counts test-plan.md:116, :330;
    - bt-files :66;
    - common-readers test-plan standing (test-plan.md:10, :26, :49, :50, :66, :100, :101, :103, :330);
    - tab-order :26, :100;
    - kbd-harness 0 (unfired control).
  - **obs-plan §3** (`stand_keyboard` in the silent census, the `common` reader count) — carried: Counts (18 → 19
    readers), Coverage (`stand_keyboard` silent). Search:
    - common-readers obs-plan.md:70 (the "18 of which … `stand_contrast` the eighteenth" site);
    - obs-silent obs-plan.md:69, :70, :106;
    - silent-census 0 (unfired control).
  - **layout-templates §Surface: desktop-native → Primary screens** (`crud-list` takes a `tabindex`; the CRUD Tab order
    moves) — carried: F5. Search:
    - crud-tab layout-templates.md:10 ×4 (the id list; @c3323 "ids add no class, style, wrapper or order";
      @c4038 "`crud-list` takes `role="listbox"` … adding no element, class, style, key, id or order"), :45 (the
      scrolling-box geometry, no-change);
    - tabindex 0 in layout-templates.
- **Coverage of new surfaces:**
  - `Enter on a focused control → Click` (keyboard.rs) → validation n/a (closed element-name and type match,
    `disabled` presence gate) · instrumentation n/a (no log site, field or span — census probe 0) · PII n/a · tests
    integ (`stand_keyboard` activation and fixture rows, `stand_act_keys`) · a11y kbd✓ (SC 2.1.1) · tokens n/a
  - `Space keyup on a focused control → Click` (`handle_key_up`) → validation n/a (same) · instrumentation n/a · PII
    n/a · tests integ (same) · a11y kbd✓ · tokens n/a
  - `click on a button focuses it` (pointer.rs) → validation n/a · instrumentation n/a · PII n/a · tests integ
    (`stand_keyboard` click-focus rows; restated `stand_act_obstructed`, `stand_act_diff`, `stand_diff`,
    `stand_act_ids`, `stand_act_spans`) · a11y focus✓ (SC 2.4.3) · tokens n/a
  - `disabled by presence → not focusable` (element.rs) → validation n/a · instrumentation n/a · PII n/a · tests
    unit+integ (snapshot.rs pin, `focusability_updates`, `stand_keyboard`, `stand_snapshot_state`) · a11y focus✓ ·
    tokens n/a
  - `crud-list` keyboard (`tabindex` + `onkeydown`, a UI element) → validation n/a · instrumentation n/a · PII n/a ·
    tests integ (`stand_keyboard::the_crud_list_takes_focus_and_its_keys_move_the_selection`;
    `stand_actionable_keys` green — the list reads its author key) · a11y kbd✓ / focus✓, AT reading ✗ (no
    `aria-activedescendant`, Insufficient fixes) · tokens n/a (no style change)
  - `stand_keyboard` (a check) → silent: the silence probe reads 0 `println!|eprintln!|print!|dbg!|std::env|env::var|
    escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`.

## Deviations from intent
- `stand_keyboard`'s three-way focus reading drops the root element from the engine's reading.
  - Justification: `BaseDocument::get_focussed_node_id` (packages/blitz-dom/src/document.rs:647-650) falls back to
    the root element when nothing is focused, so `Harness::focused()` never reads `None`. The check's first run read
    that fallback as a focused node and went red on "nothing is focused at boot".
- The disabled-after-focus fixture row disables its button by a direct `DocumentMutator::set_attribute(…, "")`, not
  through app state.
  - Justification: the plan names no mechanism. The row asserts first that the button carries the attribute and still
    holds focus.
- Step 6 left ArrowUp with no row selected unstated; it changes nothing, beside the stated end-of-list rule.
- Step 11's restatements went where the run turned red: `stand_act_diff`, `stand_act_ids`, `stand_act_obstructed`,
  `stand_act_spans` and `stand_diff`. The seven_guis checks and both flows stayed green, so none was restated there.
  The plan predicted "only those that pin a focus reading or an exact `changed` list after a button click", and that
  held.
- Scope record: none — `gate.py scope` clean, 0 recorded ("scope: clean — changed 18 · listed 18 · recorded 0 (companion
  0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 3 · excluded 92"; the three absorbed are `stand_act_ids.rs`,
  `stand_act_spans.rs` and `stand_diff.rs`, by research's derived `tests/blitz-tests/tests/stand_*.rs` row).

## Decisions & corrections
- **Guard refusal, implement P1.** `bash-guard.sh` refused a `cat`/`tee` heredoc with a file target (a scratch python
  script for the CRUD edit). It was re-issued once as Edit-tool calls. Evidence: the guard's message "Blocked: a
  cat/tee heredoc with a file target …".
- **Measured engine fact.** With nothing focused, `BaseDocument::get_focussed_node_id` / `Harness::focused()` returns
  the root element, never `None` (document.rs:647-650). A check comparing the engine's focus with the snapshot or the
  tree must drop the root. Evidence: `stand_keyboard`'s first run, 2 tests red on "the engine's focused node is the
  snapshot's focused node", green after the root was dropped.
- **Sweep hazard, the driver's key names are hyphenated.** `arrow-down`, not `arrowdown`
  (packages/escher-driver/src/schema.rs:14-27). The smoke's `press --key arrowdown` was refused `malformed` ("the
  value of `key` is outside its bound", exit 1) and re-run as `arrow-down`.
- **Recurrence of a standing learning** (host-linux.md Session Additions 2026-10-13: run every pipeline tool bare or
  redirected to a file read whole, never through `grep`/`head`/`tail`).
  - At this wrap's P1 the `cascade.py sweep --pre-pass` output was redirected to a file and then read through
    `command grep` twice before being read whole.
  - Corrected in the same step; the whole outputs are kept at `.andromeda/runs/2026-10-13T23-23-45-wrap/sweep-refusal.txt`
    and `sweep-pre-pass.txt`.
- **A miscount caught by its own gate.** In `stand_act_ids` the changing-at-boot tally counts control ROWS over
  `controls(task)` (18 rows, `back-btn` ×4), not distinct controls. The first restatement wrote 12, the run read 15,
  and the restatement says 15 "control rows". Evidence: `stand_act_ids.rs:243-244`.
- **The derived scope row worked as designed.** Research listed `derived tests/blitz-tests/tests/stand_*.rs by cargo
  test -p blitz-tests --locked --no-fail-fast`. The five F2 restatements needed no scope-record line, and the three
  files no list named are `absorbed 3` in the scope read. This is the standing testing.md learning (2026-10-10, search
  for pins before calling a check unaffected) met by construction.
- **Operator directive at the operator pass** (the operator, 2026-10-13, in this session): "Run the operator pass and
  stop before the wrap. Fire entry 15 (hygiene) AS WRITTEN, by its own command line, never through `gate.py run
  --operator`. Commit as `chore(2026-10-13-stand-keyboard-harness): operator pre-CI commit, for the run this chunk
  verdict reads`. Then fire 16 (fast, clean-tree, push) and 17 (CI read), and write evidence/ci.md." Followed:
  - entry 15 fired as written;
  - entry 16 through `gate.py run --operator 16` with no run dir;
  - entry 17 as written.
- **Operator directive at this wrap** (the operator, 2026-10-13): "run P1 only and stop" — the two-window form; the
  resume continues at P2 in `.andromeda/runs/2026-10-13T23-23-45-wrap/`.
- **Phase's snapped words, carried for P3:**
  - inputs#I1 — "Keep pipeline tools bare, or redirect them to a file you read whole. The founder's word stands: no
    mutation runs. A P4 fork that changes engine behaviour for every document goes to the founder (playbook :64), so
    raise it as a dialog";
  - inputs#I4 — F5's condition that the plan state the gap plainly;
  - inputs#I5 — F3 keeps today's reach on condition the `<div disabled tabindex=0>` divergence is named as a known
    limit in the plan and in the wrap's a11y-plan §5 text;
  - inputs#I6 — card 2 approved.
- **For P5:** card 2 rows 1–5 are the route directions above (owners named in their `where` column).
  - No PREREQ is owed: no gate was deferred.
  - No new requirement was requested.
  - The F4 CARRY moves off this entry (row 2).
- **Failed commands for P6:** none of the chunk's gates failed. The two failed calls were `escher-session press --key
  arrowdown` (exit 1, refused `malformed`, re-run as `arrow-down`) and the first `cascade.py sweep --pre-pass` (exit 3,
  three unfired controls, re-fired without them) — each re-run as its own remedy.

## Outcome
- **Acceptance criteria, re-asserted against the diff:**
  - (a11y) The Tab walk forward and back with wrap, the three focus readings agreeing at every step, all four lean
    tasks, both modes — MET: `stand_keyboard::tab_and_shift_tab_walk_the_stated_sequence_and_wrap` green (gate
    `cargo test -p blitz-tests --locked --test stand_keyboard`).
  - (a11y) Enter and Space equal a click, 20 rows stated and counted, ≥1 changing; the 10 non-activating rows hold —
    MET: `enter_and_space_on_a_focused_button_read_as_its_click` (changing rows `counter-increment`, `flight-return`,
    `flight-book`, `crud-create`, as predicted) and `keys_that_do_not_activate_type_or_change_nothing`.
  - (a11y) The fixture rows — MET: `the_fixture_controls_take_the_keys_html_gives_them`,
    `a_button_disabled_after_it_took_focus_takes_no_key`,
    `a_button_that_is_itself_scrolled_takes_enter_as_its_own_click` (the box's scroll offset asserted non-zero first).
  - (a11y) A click focuses each stand button and Tab continues; restated `stand_act_obstructed` / `stand_act_diff` —
    MET: `a_click_focuses_a_stand_button_and_tab_continues_from_it` and the restated-pins gate.
  - (a11y) Tab skips `disabled` bare / `"false"` / `"true"` (and the `div disabled tabindex=0` limit), the three F3
    pins restated — MET: `a_disabled_attribute_of_any_value_is_skipped`, `cargo test -p dioxus-native-dom --locked`,
    the restated-pins gate.
  - (a11y) On CRUD the third Tab is `crud-list`; ArrowDown/ArrowUp/Home/End move `selected` and fill the fields as a
    click does; `focused()` reads `crud-list`; `stand_accessibility_ids` reads the restated sequence — MET:
    `the_crud_list_takes_focus_and_its_keys_move_the_selection`, `stand_accessibility_ids::tab_order_is_unchanged`
    green. The stated gap (tree focus on the list) is pinned, not met — owned by card 2 row 1.
  - (tests) `apple_keybinding("deleteBackward:")` deletes on the linux leg, and windows and macos read green on the
    pushed sha — MET: `the_apple_delete_binding_deletes_on_every_os` green locally; CI#38482525151 `Test (windows)`,
    `Test (macos)` green (evidence/ci.md).
  - (tests) Through the driver, `press enter` and `press space` on a Tab-focused `counter-increment` each name
    `counter-value` with `differs: ["name"]` — MET: `stand_act_keys::enter_and_space_on_a_focused_button_count_as_its_click_does`.
  - (tests) `dioxus_falsy_boolean_attrs` reads `inert` as its 28th name on both paths; `stand_keyboard` pins Tab
    reaching content under either — MET.
  - (tests) Every Test Commands entry reads its `expect`, `run stand` included; `ci-leg.sh fast` and `doc` pass — MET
    (gates below).
  - (security) No package enters `Cargo.lock`; no added engine, bridge or stand line prints, logs, reads env or adds an
    unwrap, expect or panic — MET: both census probes read 0.
  - (obs) `stand_keyboard` installs no subscriber, reads no env var and prints nothing — MET: the silence probe reads 0.
- **Gates** (implement's run `.andromeda/runs/2026-10-13T22-48-57-implement`, one firing, "entries 17 · green 14 · red 0
  · recorded 0 · timeout 0 · not-run 3"):
  - `cargo test -p blitz-dom --locked` — green · exit 0
  - `cargo test -p dioxus-native-dom --locked` — green · exit 0
  - `cargo test -p blitz-tests --locked --test stand_keyboard` — green · exit 0
  - `cargo test -p blitz-tests --locked --test stand_accessibility_ids … --test stand_actionable_keys` — green · exit 0
  - `cargo test --workspace --locked --test stand_keyboard … --test stand_act_diff` — green · exit 0
  - `cargo test -p seven_guis --locked` — green · exit 0
  - `git diff e40d9437… -- Cargo.lock | grep -c '^+\[\[package\]\]'` — green · exit 1, last line 0
  - `git diff e40d9437… -- packages/blitz-dom packages/dioxus-native-dom examples/seven_guis/src | grep -c -E …` —
    green · exit 1, last line 0
  - `grep -c -E 'println!|…|bind\(' tests/blitz-tests/tests/stand_keyboard.rs` — green · exit 1, last line 0
  - `git diff --quiet e40d9437… -- packages/escher-driver … Cargo.toml` — green · exit 0
  - `bash scripts/agent-run.sh boot` — green · exit 0
  - `bash scripts/agent-run.sh run stand` — green · exit 0 (run.end passed 153 · failed 0 · ignored 7)
  - `bash .github/scripts/ci-leg.sh fast` — green · exit 0
  - `bash .github/scripts/ci-leg.sh doc` — green · exit 0
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — leg operator, fired as written before
    the pre-CI commit: "hygiene: clean — read 83 …", exit 0 (evidence/hygiene.md)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
    build/escher-0.1.0` — leg operator, `gate.py run --operator 16`: "green · exit 0 · 126.53s … history moved:
    refs/remotes/origin/build/escher-0.1.0 e40d9437→71af33d0" — the intended push (evidence/ci.md)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — leg operator,
    fired as written: "71af33d06f0a verdict: green · checks 16/16 · wall 754 s · runs CI#38482525151
    completed/success", exit 0 (evidence/ci.md)
  - No `defer`, no skip, no red.
  - Smoke (by hand, the boot-path through `escher-session`): crud — three Tabs then `arrow-down` select
    `crud-person-0` and fill `Hans`/`Emil`, and `end` selects `crud-person-2`; counter — two Tabs then `enter` and
    `space` read `counter-value` 1, then 2.
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran.
  - Setup 4's commit list: `71af33d06 chore(2026-10-13-stand-keyboard-harness): operator pre-CI commit, for the run
    this chunk verdict reads` alone.
  - The final HEAD `71af33d0`'s CI run CI#38482525151, green, is recorded in evidence/ci.md.
  - Implement's P4 report is held whole in this window and persisted at
    `.andromeda/runs/2026-10-13T22-48-57-implement/outcome-2026-10-13-stand-keyboard-harness.md` (the one
    `*-implement` dir for this marker); it stays the basis for the step-12 counts.
- **Process hygiene:** every process this chunk's runs started was terminated:
  - three `escher-session` hosts of the smoke (pids 2154030, 2155432, 2155454), each answered `stop` with
    `{"stopped":true}`;
  - the gate entries' cargo and test binaries exited with their entries;
  - the wrap's background `code-graph.py refresh` exited 0.
  - Re-measured at this wrap by `ps -eo pid,comm,args`: no `escher-session` or `seven_guis` process. The two cargo
    processes listed (`cargo-llvm-cov`, `cargo-nextest`) run under `~/dev/projects/viola` — another project's, not
    started by this chunk.
## Addendum — resumed 2026-10-14
- inputs#I7 · ../additional/escher-overseer/relays/keyboard-harness-founder-answers.md · copy · unchanged — the founder's answer to the wrap's answer card (`.andromeda/runs/2026-10-13T23-23-45-wrap/answer-card.md`): «Подтвердить (Рекомендую)» = Confirm, all six proposals (architecture 1 · 7, security-plan 1–3, a11y-plan 6) apply as his ratification, 2026-10-14, relayed verbatim by the overseer as his chosen option label
  - bears on **Expected amendments (from plan)**: adds to it — the answer card's one stop is answered by the founder himself
- I3 · unchanged → drifted — DRIFT: the overseer appended the section "The wrap's answer card (answered 2026-10-14)" to the relay; the F1–F4 table above it is unchanged, and the appended section is snapped whole as I7
- inputs.py verify, re-fired: inputs: 7 entries — unchanged 1 · drifted 1 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 5 · uncited 1 · unparsed 0; moved since the report: I3 unchanged → drifted; uncited I7 1 → 0 by this addendum

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: e40d9437 (the parent of the oldest pre-CI commit 71af33d0) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### examples/seven_guis/src/tasks/crud.rs — added 36 line(s) in 3 range(s)
added: 59-86 · 91 · 172-178
- 172-177 @173 «fn shown(person: &Person, filter_lower: &str) -> bool {»
  - 174-176 «format!("{}, {}", person.last, person.first)»
### packages/blitz-dom/src/events/keyboard.rs — added 74 line(s) in 3 range(s)
added: 3 · 118-123 · 128-194
- 128-148 @129 «pub(crate) fn handle_key_up<F: FnMut(DomEvent)>(»
  - 135-137 «if doc.focus_node_id != Some(target) || event.state.is_pressed() {»
  - 138-140 «if !matches!(&event.key, Key::Character(c) if c == " ") {»
  - 141-147 «if doc»
- 150-165 @152 «fn activates_on_enter(element: &ElementData) -> bool {»
  - 153-155 «if element.has_attr(local_name!("disabled")) {»
  - 156-164 «match element.name.local {»
- 167-181 @169 «fn activates_on_space(element: &ElementData) -> bool {»
  - 170-172 «if element.has_attr(local_name!("disabled")) {»
  - 173-180 «match element.name.local {»
- 183-193 @185 «fn dispatch_activation_click(»
### packages/blitz-dom/src/events/mod.rs — added 3 line(s) in 2 range(s)
added: 12 · 228-229
### packages/blitz-dom/src/events/pointer.rs — added 18 line(s) in 4 range(s)
added: 800-801 · 803-807 · 809-818 · 821
### packages/blitz-dom/src/node/element.rs — added 1 line(s) in 1 range(s)
added: 703
### packages/dioxus-native-dom/src/snapshot.rs — added 3 line(s) in 2 range(s)
added: 530 · 563-564
### tests/blitz-tests/tests/common/mod.rs — added 49 line(s) in 3 range(s)
added: 2-3 · 11 · 48-93
- 48-70 @50 «pub fn tab_order(task: LeanTask) -> &'static [&'static str] {»
  - 51-69 «match task {»
- 72-92 @75 «pub fn focused(harness: &Harness<DioxusDocument>) -> Vec<String> {»
  - 77-81 «let focused: Vec<String> = snapshot»
  - 83-89 «let tree_focus: Vec<String> = tree»
### tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs — added 14 line(s) in 7 range(s)
added: 8-9 · 87 · 112 · 159 · 166 · 175-177 · 191-195
  - 175-177 «for listed in ["readonly", "inert"] {»
### tests/blitz-tests/tests/focusability_updates.rs — added 22 line(s) in 2 range(s)
added: 61-62 · 77-96
- 78-96 @79 «fn a_bare_disabled_set_after_creation_takes_the_focusability_away() {»
  - 84-88 «doc.mutate().set_attribute(»
  - 90-94 «assert_eq!(»
### tests/blitz-tests/tests/stand_accessibility_ids.rs — added 1 line(s) in 1 range(s)
added: 15
### tests/blitz-tests/tests/stand_act_diff.rs — added 15 line(s) in 4 range(s)
added: 4-8 · 104-107 · 207 · 220-224
### tests/blitz-tests/tests/stand_act_ids.rs — added 26 line(s) in 4 range(s)
added: 9-12 · 34-37 · 40-55 · 243-244
### tests/blitz-tests/tests/stand_act_keys.rs — added 40 line(s) in 3 range(s)
added: 8-10 · 12 · 130-165
- 131-165 @132 «fn enter_and_space_on_a_focused_button_count_as_its_click_does() {»
  - 133-164 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_obstructed.rs — added 16 line(s) in 7 range(s)
added: 276-278 · 288-289 · 353-355 · 389-391 · 421-422 · 434 · 436-437
### tests/blitz-tests/tests/stand_act_spans.rs — added 3 line(s) in 1 range(s)
added: 653-655
### tests/blitz-tests/tests/stand_diff.rs — added 14 line(s) in 7 range(s)
added: 3 · 10-11 · 317-319 · 491 · 521 · 524-527 · 789-790
### tests/blitz-tests/tests/stand_keyboard.rs — new file · 618 line(s)
- 34-47 @36 «const BUTTONS: [(LeanTask, &str, bool); 10] = [»
- 49-55 @50 «const TEXT_INPUTS: [(LeanTask, &str); 4] = [»
- 57-59 «fn space() -> Key {»
- 61-64 @62 «fn activation_keys() -> [(&'static str, Key); 2] {»
- 66-84 @70 «fn holder(harness: &Harness<DioxusDocument>) -> Vec<String> {»
  - 73-78 «let by_engine: Vec<String> = harness»
  - 79-82 «assert!(»
- 86-100 @88 «fn tab_to(harness: &mut Harness<DioxusDocument>, task: LeanTask, id: &str, mode: &str) {»
  - 89-92 «let position = tab_order(task)»
  - 93-95 «for _ in 0..=position {»
  - 96-99 «assert!(»
- 102-112 @104 «fn tab_until(harness: &mut Harness<DioxusDocument>, id: &str, bound: usize, what: &str) {»
  - 105-110 «for _ in 0..bound {»
- 114-116 «fn text(harness: &Harness<DioxusDocument>) -> String {»
- 118-146 @119 «fn tab_and_shift_tab_walk_the_stated_sequence_and_wrap() {»
  - 120-145 «for incremental in [false, true] {»
- 148-204 @149 «fn enter_and_space_on_a_focused_button_read_as_its_click() {»
  - 150-162 «let derived: Vec<(LeanTask, &str)> = LeanTask::ALL»
  - 164-167 «assert!(»
  - 169-203 «for incremental in [false, true] {»
- 206-248 @207 «fn keys_that_do_not_activate_type_or_change_nothing() {»
  - 208-247 «for incremental in [false, true] {»
- 250-284 @251 «fn activation_fixture() -> Element {»
  - 256-283 «rsx! {»
- 286-288 «fn boot_fixture(app: fn() -> Element, incremental: bool) -> Harness<DioxusDocument> {»
- 290-369 @291 «fn the_fixture_controls_take_the_keys_html_gives_them() {»
  - 292-368 «for incremental in [false, true] {»
- 371-396 @372 «fn a_button_disabled_after_it_took_focus_takes_no_key() {»
  - 373-395 «for incremental in [false, true] {»
- 398-428 @399 «fn a_button_that_is_itself_scrolled_takes_enter_as_its_own_click() {»
  - 400-427 «for incremental in [false, true] {»
- 430-457 @431 «fn a_click_focuses_a_stand_button_and_tab_continues_from_it() {»
  - 432-456 «for incremental in [false, true] {»
- 459-467 @460 «fn disabled_markup() -> Element {»
  - 461-466 «rsx! {»
- 469-500 @470 «fn a_disabled_attribute_of_any_value_is_skipped() {»
  - 471-499 «for incremental in [false, true] {»
- 502-562 @503 «fn the_crud_list_takes_focus_and_its_keys_move_the_selection() {»
  - 505-508 «let fields = |harness: &Harness<DioxusDocument>| {»
  - 509-512 «let selected = |harness: &Harness<DioxusDocument>| {»
  - 513-561 «for incremental in [false, true] {»
- 564-581 @565 «fn the_apple_delete_binding_deletes_on_every_os() {»
  - 566-580 «for incremental in [false, true] {»
- 583-595 @584 «fn inert_fixture() -> Element {»
  - 587-594 «rsx! {»
- 597-618 @598 «fn the_engine_honours_no_inert() {»
  - 599-617 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_snapshot_state.rs — added 3 line(s) in 2 range(s)
added: 21 · 218-219
