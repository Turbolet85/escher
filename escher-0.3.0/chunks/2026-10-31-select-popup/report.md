# Report — 2026-10-31-select-popup

**Chunk:** Select popup — a closed select opens a painted list of its options, chosen by pointer and key
**Date:** 2026-10-31T20:21:47Z
**Commits:** since `last_wrap` 2026-10-31T13:53:30Z: `5c2ef4043 chore(records-door): the bookkeeping at Engine debts' close, so the ledger at HEAD is the ledger on disk` · `3af5a9bcf chore(records-door): commit B, the spine family moves into the store` (both before the chunk's base, neither this chunk's) · `8cd9c1286 chore(2026-10-31-select-popup): operator pre-CI commit, for the run this chunk's verdict reads` · `715798ce2 fix(2026-10-31-select-popup): a group header heavier than a row on an emboldening build`. Uncommitted, riding this wrap on the operator's word (inputs#I8): `evidence/ci.md`, `outcome-2026-10-31-select-popup.md`, the handoff's end stamp.

## Changes (structured — detectors read this)
- **Files:** 41 changed against base `3af5a9bcf`, 5 new (`gate.py scope`: `clean — changed 41 · listed 40 · recorded 1`; `cites.py added`: 3891 lines added). New: `packages/blitz-dom/src/node/select_list.rs` · `packages/blitz-paint/src/render/select_list.rs` · `tests/blitz-tests/tests/{select_popup,select_popup_painted,stand_act_popup}.rs`. Engine: blitz-dom `src/{document,resolve,accessibility,scrolling,stylo,lib}.rs`, `src/events/{keyboard,pointer}.rs`, `src/node/mod.rs`, `assets/default.css`; blitz-paint `src/{render,text}.rs`; dioxus-native-dom `src/{snapshot,lib}.rs`. Driver: `packages/escher-driver/src/{execute,lint,refusal,schema,schema_tests,json,lib}.rs`. Checks: `tests/blitz-tests/tests/{select_interaction,stand_act_select,stand_contrast,stand_id_persistence,stand_keyboard,stand_lint,stand_snapshot_state}.rs`; `examples/seven_guis/tests/{cli_commands,host_log,mcp_log,common/mod}.rs`. Other: `docs/capabilities.md`, `.gitignore`, `.github/scripts/{ci-leg.sh,test_ci_workflows.py}`. Evidence: `chunks/2026-10-31-select-popup/evidence/{ci,counts,merge-surface}.md`.
- **Symbols / APIs:**
  - **The list's state, blitz-dom `node/select_list.rs`, crate-private:** `SelectList` (`select`, `current`, `scroll_y`, `row_height`, `width`, `anchor`, `fresh`; lines 28-46), `Row`, `Placement`, `Geometry`, `size_rows` (125-130), `rows` (147-192), `group_label`, `row_label`, `option_rows`, `place` (218-263), `geometry` (265-293), `row_disabled`, `MAX_ROWS` = 20, `BORDER_WIDTH`, `ROW_PADDING_X`, `ROW_PADDING_Y`, `GROUP_INDENT`. Public read types `SelectListView` (78-91) and `SelectListRow` (93-108), re-exported from the crate root (`lib.rs:108-111`) as `ScrollbarRef` is; `SelectListView`'s associated constants `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999`, `BORDER_WIDTH`.
  - **`BaseDocument`, `document.rs:761-1089`:** one field `select_list: Option<SelectList>` (`:343-344`), `None` at construction. Public read-only: `open_select` (990-993), `select_list_bounds` (995-1012), `select_list_view` (1022-1052), `select_row_layout` (1054-1064); public writer `close_select_list` (881-889), its callers keyboard, pointer, reconcile and the driver's `focus_select`. Crate-private: `live_select_list` (797-804, the one reader of the field), `select_list_for`, `open_select_list`, `reconcile_select_list`, `select_list_options`, `move_select_list_current`, `scroll_select_list_to`, `scroll_select_list_by`, `select_list_hit` (962-988), `select_list_region`. Private: `select_list_allowed`, `measure_select_list`, `text_row_layout`.
  - **The hit:** `hit_with_scrollbar` answers `select_list_hit` first (`document.rs:2404-2406`) and no scrollbar with it, so `hit` (the driver's `covered` through it), `set_hover_to` and `handle_pointerdown` all read the list. `hit_inner` is not edited.
  - **The resolve:** `resolve` calls `reconcile_select_list` after layout and before `refresh_hover` (`resolve.rs:133-135`). `clear_interaction_state_for_removed_node` clears the field for its select (`document.rs:1408-1410`).
  - **Pointer, `events/pointer.rs`:** `handle_pointerdown` returns on a press in the list and else `close_list_outside` (415-420); `handle_click`'s `select` arm opens a single select's list or closes one standing; its `option` arm gains `click_list_row` behind `click_lands_on_row`; new `optgroup` arm; the `label` arm focuses a bound single select and opens nothing; `handle_pointermove` calls `hover_list_row`, and a press in the list starts no text drag (`pressed_in_list`); `handle_wheel` scrolls the list over it, else `close_select_list`, then scrolls. A new private `focus_select` there holds what the `select` arm did.
  - **Keys, `events/keyboard.rs`:** Tab runs `choose_current_and_close` before moving focus, either direction (32-34); `press_select_key` opens on Enter, and on a Space with no typed text standing; `press_open_list_key` (215-250) is the open list's closed key match; `choose_current_and_close` (252-267); `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size` (423-430, 452-459). `press_multiple_select_key` and `click_select_row` keep their semantics.
  - **Tree, `accessibility.rs:238-242`:** a single select writes `set_expanded(open)` and reads no `aria-expanded`; every other element keeps that read. `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing.
  - **Seen through the list, `scrolling.rs`:** `visible_region` answers the list's scrollport for a row or a group of an open list (754-756); `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, no chain or viewport moving (669-672).
  - **Paint:** `paint_scene` calls `paint_select_list` after the root's render and before the devtools overlays (`render.rs:246-247`); new `render/select_list.rs` `paint_select_list`, `device_rect`, `paint_list_box`, `paint_list_row`, `paint_list_thumb` (the last under `feature = "scrollbars"`), a non-finite box unpainted, only rows in the scrollport laid out. `text.rs`: `stroke_text` delegates to new `pub(crate) stroke_text_as`, with `RunPaint { color, synthesize_bold }` (586-593); every other caller passes `RunPaint::default()`. A synthesized bold adds one embolden weight to a build's own (683-689). The `hint` argument stays `!FONT_EMBOLDEN_ENABLED` (`text.rs:695`, read at this wrap), so a synthesized header on a plain build is emboldened with hinting on.
  - **Bridge, dioxus-native-dom:** `closed_select_of` answers `None` for a select whose list is open; the snapshot's builder reads `select_list_bounds(node)` before the client rect (`snapshot.rs:152-155`); new public `is_select`, re-exported. `NodeState` gains no field, `NODE_FIELDS` stays thirteen.
  - **Driver:** `Target.multiple` → `Target.select`; `focus_multiple_select` → `focus_select`, which also closes any list; `type` on any select focuses with no click, so it fires no DOM focus events on a single select either, and a text opening on a space types nothing there. Remaining callers: `is_select` — `execute.rs` `locate` alone; `is_multiple_select` (dioxus-native-dom) — `closed_select_of` alone within the workspace, still re-exported (`git grep -n` at HEAD); `clear_select_typeahead` — one non-test caller, `focus_select`. `lint.rs`: `Entry.in_closed_select` → `in_select_box`, true also for an open list's row or group; `clip_region` takes an open row's region from `visible_region` alone, no `contain: paint` cut. `refusal.rs`: `covered`'s meaning gains «an option of a closed select lies under it: click the select first, then the option»; name, remedy and order unchanged. `schema.rs`: `click`'s, `type`'s and `press`'s fixed help restated. `VERBS` 12, causes 8, `describe`'s count row `[12, 13, 12, 3, 8, 4, 15]` unmoved.
  - **`size`:** `stylo.rs:1124-1139` hints `height` of `size × 1.2em` on a `multiple` select through `size_rows` (`parse_unsigned_integer`, capped at 65534, `0` or a bad value no hint); `default.css:183-193` `select[multiple][size] { overflow-y: auto; }` and `line-height: 1.2` on its rows. A single select's `size` stays unread.
  - **Test-only state directories:** `cc-popup` (`cli_commands.rs:442`) and `hl-popup` (`host_log.rs:296`) under `CARGO_TARGET_TMPDIR`. No `ml-popup`.
  - **Test renames:** `select_interaction::enter_and_space_open_nothing_on_a_single_select` → `enter_and_space_open_the_list_and_a_released_space_does_nothing_more` (`git diff`); no other test renamed in `select_interaction` or `stand_act_select`. Its old name stands at `.andromeda/test-plan-detail/test-scope-summary.md:24` (`git grep -n` at HEAD).
- **Crates / modules:** two new modules, blitz-dom `node::select_list` and blitz-paint `render::select_list`. No crate added or removed.
- **Dependencies:** none (preservation guard green: `Cargo.toml`, `Cargo.lock`, `deny.toml`, every manifest unmoved).
- **Schema / config:** the driver's schema unmoved in shape (`describe`'s row above); three fixed help texts and `covered`'s meaning restated, the program's own `'static` text. No key name, node key, field, cause, session error, exit, carrier, listener, port, env var, log field or span. The list crosses the ratified carriers as: a row's own `bounds` in the snapshot and the diff (`differs` names `bounds`, a closed set inside the ratified crossing) and pixels in a `screenshot`. New pixel content in every shot of an open list: an option's label and a group's `label`, painted. New attribute read: `size` on a `multiple` select, a parse into a capped whole number, no text carried out. No redaction shape moved.
- **Spec-master edits:** none (this wrap's P2 makes them).
- **Retired:** the limits «no popup is painted», «Enter and Space open nothing on a single select», «a group's label is unpainted», «a `multiple` select's `size` is unread», `Entry.in_closed_select`, `Target.multiple` and `focus_multiple_select`. No census row: none stands for them.
- **Counts / qualifiers moved:** each fact names its pattern id; the rows are in this run's `cascade-2026-10-31-select-popup-pre-pass.json`.
  - `repeat-names` — `stand-repeat`'s named `--test` words fifteen → seventeen (`select_popup`, `select_popup_painted`; `ci-leg.sh:59-60`): 29 hits · 9 changed (standard-contracts:312, test-plan:377, :413, test-plan ci-integration:9, :10, :16, quality-gates:22, unit-test-strategy:46, a11y ci-integration:9) · 9 handled under `act-files` · 11 no-change (session errors, modules, ordinals, history).
  - `act-files` — `stand_act_*` files fifteen → sixteen (`git ls-tree`: 15 → 16): 11 hits · 10 changed (conventions:54, existing-scopes:116, test-plan:146, :149, integration-test-strategy:31, test-scope-summary:94, test-strategy:53, unit-test-strategy:26, session-lifecycle:11, obs-plan:76, whose «fourteen» was already behind at the base) · 1 no-change (test-plan:150, an ordinal).
  - `stand-files` — `stand_*.rs` 50 → 51 and `run stand` 222 · 0 · 7 → 237 · 0 · 7 (`evidence/counts.md`): 13 hits · 4 changed (test-plan:176, :190, test-plan ci-integration:8, quality-gates:22) · 9 no-change (the glob stated, history).
  - `a11y-tally` — the a11y leg 14 · 112 · 0 → 14 · 114 · 0: 3 hits · 3 changed (test-plan ci-integration:75, :99, a11y ci-integration:11).
  - `kbd-count` — `stand_keyboard` 21 → 23: 6 hits · 6 changed (existing-scopes:106, test-plan ci-integration:99, test-scope-summary:151, a11y ci-integration:10, :11, keyboard-navigation:120).
  - `state-rows` — `STATE_ROWS` 18 → 20, tally (19, 0, 1, 0): 7 hits · 4 changed (existing-scopes:113, test-scope-summary:151, a11y-plan:64, visual-design-verification:12) · 3 no-change (no count stated).
  - `cli-count` — `cli_commands` 14 → 15: 6 hits · 3 changed (e2e-test-strategy:8, unit-test-strategy:26, session-lifecycle:11) · 3 no-change (standard-contracts:185, input-validation:65, test-scope-summary:16: a token shared, no count).
  - `ci-tally` — the per-OS tally, CI#41192433140 → CI#41235503227: linux 217 · 1236 · 0 · 12 · macos and coverage 198 · 1233 · 0 · 12 · windows 198 · 1185 · 0 · 9 · a11y 14 · 114 · 0 · 0 · stand repeat 228 · 1155 · 0 · 21 (`evidence/ci.md`, each move explained by name there): 7 hits · 4 changed (test-plan ci-integration:75, quality-gates:16, :22, a11y ci-integration:11) · 3 no-change (occupied-resources:98, dependency-security:51, test-plan ci-integration:40: the free-disk step's record of that run).
  - `cover-read` — the coverage reading: escher-driver 4959/5058 = 98.04 % · dioxus-native-dom 2791/3240 = 86.14 % · blitz-test-harness 968/1008 = 96.03 %, floors 3 of 3 (`evidence/ci.md`): 1 hit · 1 changed (quality-gates:16).
  - `mod-decl` — checks declaring `mod session_common;` 31 → 32, `mod fixtures;` 28 → 31 (`git grep -l -E '^mod {name};'` at both revs; unanchored it reads 32 and 29 at the base, one more each, the module file's own doc comment): 12 hits · 3 changed (test-plan:151, test-strategy:52, :53) · 9 no-change (per-target rows; test-plan:149 under `act-files`).
  - `run-all` — `run all` → 727 · 0 · 12 (`evidence/counts.md`): 13 hits · 2 changed (test-plan:176, test-plan ci-integration:75) · 11 no-change (dated history rows).
  - `target-files` — blitz-tests `.rs` targets 153 → 156 (`git ls-tree`): 2 hits · 0 changed · 2 no-change (a citation's digits, `stand_actionable_keys.rs:153-155`). No master states the count in this wording.
  - `no-popup` — the limit retired: 8 hits · 8 changed (architecture:62, established-decisions:78, input-validation:249, surface:13, test-scope-summary:24, a11y-plan:123, a11y-scope-summary:23, keyboard-navigation:41).
  - `enter-space` — Enter and Space open a single select's list: 10 hits · 10 changed (architecture:62, established-decisions:78, existing-scopes:112, input-validation:248, :249, surface:13, test-scope-summary:24, a11y-scope-summary:23, keyboard-navigation:41, :119).
  - `closed-option` — `closed_select_of`'s open answer, `in_select_box`, an option of a closed select still `covered` (F9): 26 hits · 14 changed (architecture:183, existing-scopes:76, standard-contracts:18, :212, :213, :220, :279, input-validation:264, :265, layout-templates:61, test-scope-summary:237, keyboard-navigation:26, screen-reader-support:11, visual-design-verification:16) · 12 no-change, true as they stand on his F9, or moved by another fact on the same line.
  - `size-unread` — `size` read on a `multiple` select, a single select's still unread: 6 hits · 6 changed (architecture:62, established-decisions:78, input-validation:249, surface:13, test-scope-summary:24, a11y-scope-summary:23).
  - `group-label` — a group's `label` painted as a header: 8 hits · 6 changed (established-decisions:121, standard-contracts:26, input-validation:256, surface:13, a11y-plan:202, aria-patterns-roles:21) · 2 no-change (test-scope-summary:24 under `size-unread`, unit-test-strategy:20 a screenshot's unpainted pixels).
  - `focus-select` — the renames and new reader: 9 hits · 8 changed (architecture:135, :183, existing-scopes:76, standard-contracts:18, :190, input-validation:262, :264, :265) · 1 no-change (log-coverage:75: names the readers, its silence holds; add `is_select`).
  - `type-focus` — `type` on any select focuses with no click: 12 hits · 7 changed (standard-contracts:189, :190, input-validation:249, :261, :262, keyboard-navigation:48, session-lifecycle:5) · 5 no-change (project-intent:33, standard-contracts:188, input-validation:56, integration-test-strategy:29, :31: `type` on a target that cannot take it, or `type` replacing on other controls).
  - `expanded` — a single select's native `expanded`: 12 hits · 3 changed (standard-contracts:30, a11y-plan:55, input-validation:219) · 9 no-change (the attribute read still true on every other element).
  - `changed-set` — open and close mark the select, a new kind of site: 34 hits · 4 changed (architecture:275, standard-contracts:9, a11y-plan:29, a11y-strategy:10) · 30 no-change. See Spec claims disproved for architecture:275's «no scroll or layout».
  - `hit-first` — the list answers first in `hit_with_scrollbar`: 5 hits · 3 changed (standard-contracts:49, :61, established-decisions:88's neighbour clause, a row never a thumb drag) · 2 no-change (motion:15, visual-design-verification:85: the thumb hazard stands).
  - `paint-order` — the list painted last in `paint_scene`, `SelectListView`: 6 hits · 2 changed (architecture:157, standard-contracts:105) · 4 no-change.
  - `stroke-text` — `stroke_text_as`, a synthesized bold: 3 hits · 1 changed (typography:51) · 2 no-change (feature names at established-decisions:32, typography:31).
  - `visible-reg` — a row reads the list's scrollport: 7 hits · 3 changed (architecture:145, standard-contracts:51, depth-strategy:28) · 4 no-change.
  - `covered-text` — `covered`'s meaning gains a clause: 5 hits · 2 changed (standard-contracts:178, test-plan integration-test-strategy:30) · 3 no-change (the two standing readings restated with no clause lost).
  - `click-help` — a click on a single select opens: 5 hits · 1 changed (a11y-plan:113 through this fact's neighbour clause) · 4 no-change (a click on a button or range).
  - `label-click` — a label's click on a single select focuses and opens nothing, asserted by `select_popup`'s `a_labels_click_focuses_the_select_and_opens_nothing` (428-441): 3 hits · 3 changed (input-validation:210, test-scope-summary:24 «asserted by no check», keyboard-navigation:55).
  - `select-colours` — the list's colours, new rows: 6 hits · 2 changed (color-palette:29's neighbour, surface:9's neighbour) · 4 no-change (the `multiple` row's pair and the sheet's border stand).
  - `lint-exclude` — the `overlap` exclusion restated: 5 hits · 5 changed (standard-contracts:213, :220, layout-templates:61, test-scope-summary:237, visual-design-verification:16).
  - `free-disk` — the WATCH's count 0/3 → 2/3 (route, P5): 8 hits · 0 changed · 8 no-change (the step's record stands; test-plan ci-integration:40 reads «unproven», still so).
  - `panic-pos` — see Spec claims disproved: 5 hits · 0 changed · 5 no-change (standard-contracts:51 describes the reader; the rest anonymous blocks generally). The owed text is a new row in security-plan §Error Handling.
  - `os-sensitive` — see Spec claims disproved: 3 hits · 0 changed · 3 no-change (feature statements true).
  - `select-model` — a control, nothing moved: `select_interaction` 18, `stand_act_select` 10, the one path `apply_select_choice`: 61 hits · 0 changed by this fact · 61 no-change (citation digits are P2's sweep's).
  - `region-scroll` — no master site, the new public readers need new text: `control: region-scroll, incubator, ua-pair never fired over the seven masters' pre-pass text, bodies and sets (715798ce)`.
  - `incubator` — no master site, same refusal line; the `.gitignore` line and the deferred file are route and incubator matters.
  - `ua-pair` — capabilities' «A closed select's contrast pair is unmeasured» removed (`docs/capabilities.md`), the open list's plain pair measured by `select_popup_painted`: zero master sites in this wording, same refusal line. The closed select's own pair stays unread; a detector's own sweep is the control.
  - Lines over 2 000 chars, per (line, fact): existing-scopes:112 — `enter-space` changed · `closed-option` true · `repeat-names` true · `select-model` true. standard-contracts:311 — `select-model` true. input-validation:200 — `changed-set` true. input-validation:249 — `no-popup` changed · `enter-space` changed · `closed-option` true · `size-unread` changed · `type-focus` changed · `changed-set` true · `select-model` true. test-plan:176 — `stand-files` changed · `run-all` changed. test-plan ci-integration:8 — `select-model` true · `stand-files` changed. test-plan ci-integration:72 — `run-all` history · `select-model` history. keyboard-navigation:26 — `closed-option` changed · `click-help` true · `select-model` true. keyboard-navigation:119 — `enter-space` changed · `closed-option` true · `changed-set` true · `select-model` true. session-lifecycle:5 — `type-focus` changed · `changed-set` true. session-lifecycle:11 — `cli-count` changed · `repeat-names` ordinal · `act-files` changed · `select-model` true.
  - Sweep reach: 36 patterns · 37 moved facts, 37 with a pattern (this report's own count) · owner pattern 2 standing · by line 11 lines · 37 offsets. Owner rows: `owner-title` established-decisions:78, surface:13 ×2, each dropping the owner phrase; `owner-generic` input-validation:203, test-scope-summary:228, neither this entry's. A listing reaches its patterns' wording and no further.
- **Dev-tool versions:** none — no host tool installed, upgraded or read changed.
- **Harness / gate surface:** `ci-leg.sh` `stand-repeat`'s list gains two words (`:59-60`); `test_ci_workflows.py:367` asserts them. `stand_act_popup` joins `run stand` and `stand-repeat` by its prefix. The `a11y` leg stays fourteen files. No verdict shape moved.
- **Cross-project / external claims:**
  - **CI, the run this verdict reads:** CI#41235503227 at `715798ce2e5c7bb378b6474c6d2e2fe5b35fcc00`: `verdict: green · checks 17/17 · wall 1381 s` (inputs#I8; `evidence/ci.md`, entry 30 verbatim). The sha is the record; this wrap's commit adds to that tree.
  - **CI, the first read:** CI#41233234207 at `8cd9c1286`: `completed · failure`, `Test (macos)` alone, sixteen success (`evidence/ci.md`).
  - **Stand repeat's free-disk step:** `Free Disk Space (Ubuntu)` `success` on both runs (`gh run view {id} -R Turbolet85/escher --json jobs`, read at this wrap).
  - **Take-up read:** `da536ea7a714` green 17/17, CI#41208321538 (`scope.md`).
  - **inputs.py verify,** fired before this report: `inputs: 8 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 6 · uncited 2 · unparsed 0`. No DRIFT. Its two uncited, I7 and I8, are cited here.
    - inputs#I1 · copy no-repo `../additional/escher-overseer/relays/select-popup-go-founder-2026-10-31.md` · unchanged — the founder lifts the pause for this one chunk, «Запускать Select popup (Рекомендую)»; the wrap's run terms: sampler forced, classifiers at effort high, 2 draws on the wrap, a dead draw one re-spawn, a second death stops and comes to the overseer.
    - inputs#I2 · copy message · n/a — run terms at the phase.
    - inputs#I3 · copy message · n/a — correction: classify in parts is the designed path.
    - inputs#I4 · copy no-repo `../additional/escher-overseer/relays/select-popup-founder-2026-10-31.md` · unchanged — his ten forks, all A, verbatim; the operator's call of no `expanded` node key.
    - inputs#I5 · copy message · n/a — the ten forks as put to him.
    - inputs#I6 · copy message · n/a — the operator's word: no `expanded` key, the other leans stand.
    - inputs#I7 · copy message · n/a — the operator's word on the P5 card `.andromeda/runs/2026-10-31T16-59-55-phase/p5-card-1.md`, relayed verbatim by the operator, 2026-10-31. Its `## Beyond the plan`, verbatim:
      | # | Proposal | Where it lands | By whom |
      |---|---|---|---|
      | 1 | **Every limit this plan states and leaves for later gets a residual line and a deferred line:** the thumb not draggable · a highlighted row's contrast unread by the lint · a larger authored font clipping · the highlight absent from the snapshot and the tree · a single select's `size` unread. The existing `press` line, Ctrl on `press`, also owes its deferred line | `.andromeda/residuals.md`, one line each for the five; `escher-0.4.0-incubator/deferred-from-0.3.0.md`, one line each for the six | this chunk's wrap, at its P5 |
    - inputs#I8 · copy message · n/a — snapped at this wrap: stop after P1; the sampler forced (`detail.py sample --side wrap --force`); the run green on CI#41235503227 at `715798ce2`; the uncommitted `evidence/ci.md` and outcome ride this wrap.
- **Reverted / negative API facts:**
  - `mcp_log`'s popup test records nothing and no `ml-popup` directory exists: recording there would have needed a `cfg!(unix)` site.
  - `8cd9c1286`'s embolden `FONT_EMBOLDEN_ENABLED || (paint.synthesize_bold && synthesis.embolden())` was replaced at `715798ce2`: one vector for both arms read a header no heavier than a row on an emboldening build.
  - A throwaway frame-hash test target was removed before the fix commit.
  - The plan's rejected approaches stand unbuilt: `:open` plus rows as boxes, a second OS window, an overlay hit in `hit_inner`, colours in `default.css`.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `panic-pos` — a key press panics at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`) when the focused control's anonymous block was dropped because an ancestor gained `hidden`. Reproduced at implement on a `select` and a plain `button`, with no list opened, the stack through `handle_dom_event`'s `absolute_position` (outcome §What needs a decision). A remote page can reach it. security-plan §Error Handling's panic-path list holds no row for it (`grep -n -i panic .andromeda/security-plan-detail/error-handling.md`, read at this wrap: rows 18, 24, 30, 36). `node.rs` and `events/mod.rs` are unchanged against the base (`git diff --quiet`, exit 0); no controlled run at the base was taken, so «predates this chunk» rests on that and on the stack. `select_popup` skips the key press in that one row, its comment saying why. Owner: P5, below.
  - `changed-set` — architecture:275 «the changed set is written … by no node creation, hover, scroll, resize, load, animation or layout» is false where a list stands: a page scroll or a move that closes a list marks its select through `reconcile_select_list` at the next resolve, and a wheel elsewhere marks it through `close_select_list`. With no list open it holds. A highlight or a wheel move inside the list marks nothing, as stated in the plan.
  - `stroke-text` — typography:51 «with hinting off when emboldening» no longer holds for a synthesized header on a plain build: emboldened, `hint` still on (`text.rs:695`). On an emboldening build a synthesized header takes twice the strength, not the stated one.
  - `os-sensitive` — the plan's `Test Commands` «OS-sensitive cases: none is added. No `cfg(unix)`, `cfg(windows)` or `cfg!(…)` site lands» and the CI entry's note: false twice. `apple-font-embolden` made CI#41233234207 red on macOS alone, no local gate building that feature; `host_log`'s new test carries `#[cfg(unix)]`, as every host-driving test there does. A plan claim, no master.
  - `label-click` — test-scope-summary:24 «a click on a select's label re-dispatched to the select is asserted by no check»: the single-select half is now asserted, and no longer re-dispatches.
- **Expected amendments (from plan):**
  - Playbook *Boundary widening*, and *An engine behaviour change for every document*: carried — **Schema / config** (pixels, `size`) and **Symbols / APIs**. No master site for the rule itself — a playbook rule P2 applies; `owner-generic` base rows playbook:58, :66 no-change. Halts once at P2 as his answer card, on his recorded word inputs#I4 (CARRY `cry-musoudbr`). The group's `label` as a name is settled by his F7 (inputs#I4), so input-validation:256's «not as to the `label` as a name» ends.
  - `architecture` §Established Decisions → Scrolling and input; §Standard Contracts → Document core, Scrolling, Mutation, Paint and shell, Dioxus DOM bridge, Driver session, CI contracts; §Cross-cutting Patterns → Invalidation: carried — **Symbols / APIs**, **Harness / gate surface**. Search: `no-popup` 2, `enter-space` 3, `closed-option` 10, `size-unread` 2, `focus-select` 5, `hit-first` 3, `paint-order` 5, `changed-set` 13, `repeat-names` 7, `visible-reg` 3, `covered-text` 3, `expanded` 3, `stroke-text` 1 in architecture and its set.
  - `security-plan` §Input Validation; §Threat Model Summary: carried — **Schema / config**, **Coverage**. Search: `no-popup` 1, `enter-space` 2, `closed-option` 3, `size-unread` 1, `focus-select` 3, `type-focus` 4, `expanded` 2, `group-label` 1, `label-click` 1, `covered-text` 1 in security-plan and its set. New rows for the list's keys, `size` and the pixels; §Error Handling's panic row is beyond the plan's list.
  - `design-system` §Color Palette; §Surface: desktop-native; §Spacing, §Depth Strategy; §Motion: carried — **Symbols / APIs** (constants, placement), **Coverage** (`tokens`). Search: `no-popup` 1, `group-label` 1, `select-colours` 2, `stroke-text` 2, `hit-first` 1, `visible-reg` 1. §Spacing's and §Motion's rows are new text, no pattern stands on them: `no master site — no statement of the list exists to find`.
  - `layout-templates` §Surface: desktop-native: carried — **Symbols / APIs**. Search: `closed-option` 1 (×3 on layout-templates:61), `lint-exclude` 1, `visible-reg` 1.
  - `test-plan` §1, §4, §5, §9, §10: carried — **Counts / qualifiers moved**. Search: `repeat-names` 19, `act-files` 8, `stand-files` 10, `kbd-count` 2, `state-rows` 1, `cli-count` 4, `ci-tally` 4, `cover-read` 1, `mod-decl` 11, `run-all` 13, `no-popup` 1, `enter-space` 1. Registry `session-lifecycle`'s rows counted here: `repeat-names` :6, :11, `act-files` :11, `cli-count` :11.
  - `obs-plan` §3, §6, §8: carried — **Coverage**, **Counts / qualifiers moved**. Search: `act-files` 1 (×3 obs-plan:76), `closed-option` 1, `focus-select` 1 (log-coverage:75), `select-model` 3 (the silence rows). New silence rows for three checks; a group's label painted.
  - `a11y-plan` §2, §5, §6, §7, §9: carried — **Counts / qualifiers moved**, **Symbols / APIs**. Search: `no-popup` 3, `enter-space` 3, `closed-option` 6, `expanded` 6, `group-label` 2, `kbd-count` 3, `state-rows` 4, `a11y-tally` 1, `label-click` 1, `changed-set` 9.
- **Coverage of new surfaces:**
  - `select list, engine overlay (every document, remote pages included)` → validation closed key and element-name matches, one reader of the state, every node through `get_node`, every edge finite✓ · instrumentation none added, none required (silence census, gate entries 2, 4, 5)✓ · PII no log site; a label leaves only as a painted pixel and the snapshot's existing `name`✓ · tests unit (`select_list` 6) · integ (`select_popup` 25, `select_popup_painted` 6, `stand_act_popup` 12, `stand_keyboard` +2, `stand_lint` +1) · process (`cli_commands`, `host_log`, `mcp_log` +1 each) · a11y SC 2.1.1, 2.4.3, 1.4.3 asserted; an assistive technology's reading `unrunnable-here`; the windowed list read and not run · tokens hardcoded✗ — engine constants on his F6, the engine holding no tokens; new rows owed in design-system §Color Palette.
  - `the list's pixels on the screenshot carriers` → validation n/a, no new input · instrumentation shot's fields unmoved✓ · PII a page's option and group labels in pixels: **boundary widening, P2's halt** · tests `stand_act_popup` crop, `cli_commands` hosted shot equal to in-process · a11y n/a · tokens n/a.
  - `size on a multiple select` → validation `parse_unsigned_integer`, capped at 65534, `0` and non-numeric read none✓ · instrumentation n/a · PII n/a · tests `select_popup` (`size_rows` unit; hostile `size`; `size=3` shows three rows) · a11y the row brought into view by key✓ · tokens n/a.
  - `tree expanded on a single select` → validation a closed bool from engine state, no author attribute✓ · instrumentation n/a · PII n/a · tests `select_popup` counted table · a11y `ComboBox`, `expanded`, chosen option `selected`; the highlight unread by the tree, stated · tokens n/a.
  - `BaseDocument public readers` (`open_select`, `select_list_bounds`, `select_list_view`, `select_row_layout`, `close_select_list`) → validation through `get_node`, a row with no styles `None`, no `layout_style()`✓ · instrumentation n/a · PII `SelectListRow.label` page text in a read type, never logged · tests unit and integ above · a11y n/a · tokens n/a.
  - `driver help and covered text` → validation fixed `'static` text · instrumentation span fields eight, unmoved (`stand_act_spans`)✓ · PII no call text · tests `schema_tests`, `refusal`, `json` pins · a11y n/a · tokens n/a.

## Deviations from intent
- `packages/blitz-paint/src/text.rs` outside the lists: step 4's «brush its row state gives» cannot carry a colour, `TextBrush` holding a node id alone. Justified, recorded, and it later held this chunk's one CI defect.
- `SelectList` has a seventh field, `fresh`: a click focuses and opens in one step, before a `:focus` rule's restyle, so a box-moving rule would read as moved and close the list at once. Pinned by `a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened`.
- Two public read types, where the plan says one: paint needs a row's fields. The colours are associated constants, no further type.
- `live_select_list` is `pub(crate)`, the plan saying private, though every caller stands in `document.rs` (`git grep -n` at HEAD). The field is read past it in two places the plan's «no reader takes the field any other way» does not name: `reconcile_select_list` (`document.rs:894`), which must see a list the reader no longer answers, and the removal arm (`:1408`). `hover_list_row` writes `current` directly (`pointer.rs:604`).
- `select_interaction`'s `KEYS` table moved one row, Enter → `PageDown`, its count still 14: step 14's «read as they stand» did not hold for a row that now opens.
- `#[cfg(unix)]` on the new `host_log` test, and no `ml-popup`: see Spec claims disproved and Reverted.
- Details the plan left open, each chosen at implement: a select with no row to show opens nothing; a group with no label reads the whole list's box; a press on a row starts no text drag, and the release chooses the row under it; a close clears the typed text; the row height is the select's font on one line.
- `gate.py scope` at P1: `scope: clean — changed 41 · listed 40 · recorded 1 (companion 0 · mechanical 0 · in-intent 1 · widening 0) · absorbed 0 · excluded 720`.
- Scope record, by class — **in-intent:** `packages/blitz-paint/src/text.rs` · serves step 10 · self. **Companion · mechanical · widening:** none.
- A direct operator direction past implement: «your fix, a header heavier than a row on every build, the test as written; a run with no synthesized bold paints as before» (`evidence/ci.md`), the fix at `715798ce2`, `select_popup_painted.rs` unchanged by it.
- Plan entry 30's bound: first pass `--wait 1800` on the operator's word, the plan writing 2400; second pass as written.

## Decisions & corrections
- A local gate block read green over a defect only macOS showed: no local gate builds `blitz-paint/font-embolden`. `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` reproduced macOS's two numbers on linux (`the header reads 70972, the row 70972`). A paint change under an OS-gated feature has a local witness in that flag (`evidence/ci.md`).
- `cargo` stops at the first failing target, so a red job's later targets go unread on that run (`evidence/ci.md`).
- His ten forks, all A, in his own words, inputs#I4. The operator: no `expanded` node key, the other leans stand, inputs#I6; the page's scroll closing the list by the anchor rule and a label opening nothing, both the operator's leans, plan steps 3 and 6.
- A vacuous check found by its own control: «a click anywhere reads as landing on a row» survived its first firing, since choosing the chosen option and closing read alike; a disabled chosen option then read it red (`evidence/counts.md`; friction `contract.vacuous-check-found`).
- A test held the wrong value: a placement row expected a list above a select whose rows fit below it, reddening entries 9 and 15 on the first full block, because the table had been re-run by a filter before an earlier row's fix (friction `contract.test-expectation`).
- `recurrence-despite-learning`: the Bash guard refused a `cat` heredoc with a file target at implement; the script went to a scratchpad file (`.claude/rules/host-linux.md`, Transports).
- `recurrence-despite-learning`: a controlled-select fixture was first written uncontrolled and its choice reverted on re-render, the limit `docs/capabilities.md` states; the fixture now writes `selected` back.
- `harness.center_of` on an inline element reads no box, the deferred learning of the last handoff, met again: `select_popup` reads client rects through its own `client_point` (157-164).
- A correction of implement's own: it took the row-versus-box point test for a general hazard; the surviving control showed it decides only for a disabled chosen option (outcome).
- A sweep hazard: `git grep -l -F 'mod fixtures;'` over `tests/blitz-tests/tests/*.rs` reads one more than the checks, each shared module's doc comment naming the line; anchor it, `-E '^mod fixtures;'`. Measured at this wrap: 29 and 32 unanchored against 28 and 31.
- A sweep hazard: `\b153\b`-shaped count patterns over the masters read citation digits (`stand_actionable_keys.rs:153-155`), the tool's warning class.
- `stand_keyboard`'s closed-arrow test passes untouched, yet its Enter and Space rows now open and then choose: it reads true and no longer reads what its row order suggests (outcome).
- This wrap: the Bash guard refused one call piping `rec.py query` into `head`, re-issued bare and run (`guard-refused`).
- The `Focussed node` count is not settled: 0 lines through the sink on the workspace build, no base reading, so neither moved nor unmoved; `stand_keyboard`'s `holder` holds the forecast's premise instead (`evidence/counts.md`).

## Outcome
**Acceptance, each against the diff:**
- `v030-17` — met: `select_popup`, `select_popup_painted`, `stand_act_popup` green in both layout modes, CI green. Ledger `implemented`; P7.3 flips it.
- (arch) the one write path — met: `click_list_row` and `choose_current_and_close` call `apply_select_choice`; no other writer in the diff.
- (arch) verbs 12, causes 8, `covered` restated once, the count row unmoved — met (`cargo test -p escher-driver`, `schema_tests`).
- (arch) through `paint_scene` alone, no shell or renderer moved — met (guard entry; `packages/blitz-shell`, `packages/dioxus-native` absent from the 41). Windowed list read and not run, stated.
- (arch) two `click`s open then close, each focusing — met (`two_clicks_on_one_point_open_and_close_each_focusing`).
- (security) disabled, inert, disabled option, non-invertible transform; removed, disabled, hidden, made inert closes with no panic — **met to its letter, with a defect beside it:** the list closes with an ancestor hidden too, and a key pressed after that panics, for any control, with no list opened. `select_popup` skips the key press in that one row. Routed to P5 below; a P2 row in security-plan §Error Handling.
- (security) hostile `size`, 2,000 options, a 5,500-character label — met (`hostile_markup_keeps_every_box_finite_and_panics_nowhere`).
- (security) `host_log`, `mcp_log` 0 needles at `trace`, `stand_act_spans` eight fields — met.
- (design) clips and `opacity` — met (`a_box_that_clips_the_select_clips_no_row_and_a_point_outside_reads_what_shows`, `a_list_paints_whole_past_an_opacity_ancestor_and_a_box_that_clips_its_select`).
- (design) no motion, no `advance` needed — met (`a_screen_read_after_a_step_reads_the_new_state_with_no_advance`; entry `prefers-reduced-motion|…` green).
- (design) a shot of an open list, `fonts-unavailable` standing — met (`a_shot_of_a_row_crops_to_it_and_leaves_the_list_open`, the refusal unmoved).
- (layouts) after open and close the same screen, the Tab walk holds — met (`escape_closes_with_the_value_unchanged_and_the_rows_back_at_the_selects_box`, `the_flight_type_walk_holds_with_its_list_open_and_after_each_close`).
- (layouts) a pointer at a row's centre, scrolled or not — met (`a_pointer_at_a_rows_centre_lands_on_it_with_the_page_scrolled_or_not`).
- (tests) `run stand`, `run all`, the four legs, CI green, stand repeat, floors 3 of 3 — met (below).
- (obs) 2 `info_span!`, silence census 0, added lines clean — met at the pre-CI tree; `text.rs`'s fix adds no such token (`git diff 3af5a9bcf -- packages/blitz-paint/src/text.rs`, read at this wrap).
- (a11y) SC 2.1.1 · 2.4.3 · 1.4.3 and the tree — met in the harness; what an assistive technology reads stays unmeasured, stated.
- (lint) `at_boot` `[0, 0, 0, 0]`, a row no `overlap`, the planted pair reading — met (`an_open_lists_rows_read_no_overlap_on_the_stand`, `the_lint_reads_no_overlap_on_a_row_and_the_planted_pair_still_reads`).

**Gates,** the last full block at 2026-10-31T19:05:23Z on the pre-CI tree (`entries 31 · green 27 · red 0 · recorded 0 · timeout 0 · not-run 4`; trail `.andromeda/runs/2026-10-31T18-16-50-implement/gate-2026-10-31-select-popup.json`), each with the exit its `expect` held:
- `git diff --quiet 3af5a9bcf… -- Cargo.toml Cargo.lock deny.toml …` green · exit 0
- `grep -lE 'println!|…' tests/blitz-tests/tests/{select_popup,…}.rs … | wc -l` green · exit 1, last line 0
- `grep -rh 'info_span!(' packages/escher-driver/src | wc -l` green · last line 2
- `git diff … 'packages/*.rs' | grep -c -E '^\+[^+].*(tracing::|…)'` green · exit 1, last line 0
- `grep -cE 'tracing::|…' packages/blitz-dom/src/node/select_list.rs packages/blitz-paint/src/render/select_list.rs` green · exit 1
- `grep -rlE 'Instant::now\(\)|…' packages/blitz-dom/src | grep -vc 'clock\.rs$'` green · exit 1, last line 0
- `grep -c -E 'DioxusLabs/taffy|…' Cargo.toml` green · exit 1, last line 0
- `grep -c -E 'prefers-reduced-motion|…' …` green · exit 1
- `cargo test -p blitz-dom --locked` · `-p dioxus-native-dom` · `-p escher-driver` green · exit 0, lacks FAILED
- `cargo test -p blitz-tests --locked --test select_popup --test select_popup_painted --test stand_act_popup` green
- `cargo test -p blitz-tests --locked --test select_interaction … --test stand_screenshot_repeat` green
- `cargo test -p blitz-tests --locked --test inert_subtree … --test incremental_oracle` green
- `bash .github/scripts/ci-leg.sh fast` · `doc` · `audit` (`advisories ok`, `licenses ok`) · `a11y` green
- `grep -E '^test result:' target/ci-logs/a11y.log | awk …` green · `lines 14 passed 114 failed 0`
- `bash scripts/agent-run.sh boot` · `run stand` (237 · 0 · 7) green
- `ls tests/blitz-tests/tests/stand_*.rs | wc -l` green · last line 51
- `bash scripts/agent-run.sh run all` (727 · 0 · 12) · `cargo test -p seven_guis --locked` green
- `cargo tree … | grep -c -E '^accesskit_(xplat|winit) '` green · exit 1, last line 0
- `cargo build -p seven_guis --bin seven_guis_native --locked` green
- `RUST_LOG=info timeout 10 target/debug/seven_guis_native; test $? -eq 124` smoke green · `service.name=seven_guis`
- `gate.py hygiene` (`leg = operator`) — `hygiene: clean`, recorded in `evidence/ci.md`
- `ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push …` (`leg = operator`) — exit 0 both passes, `evidence/ci.md`
- `ci.py conclusion --sha HEAD --wait 2400` (`leg = operator`) — final HEAD `verdict: green`, 17/17, CI#41235503227; the first pass red on `Test (macos)`, this chunk's, fixed
- `gh run view <id> … --log | …` (`leg = operator`, `expect = []`) — recorded: linux 217 · 1236 · 0 · 12 · macos and coverage 198 · 1233 · 0 · 12 · windows 198 · 1185 · 0 · 9 · a11y 14 · 114 · 0 · 0 · stand repeat 228 · 1155 · 0 · 21, CI#41235503227, all green, each move named in `evidence/ci.md`.

The `text.rs` fix postdates that block; `select_popup`, `select_popup_painted` and `stand_act_popup` were read after it on both builds (25 · 0, 6 · 0, 12 · 0 each), `ci-leg.sh fast` and CI ran on it, and five frame hashes moved only where a synthesized header stands on the emboldening build (`evidence/ci.md`). No `defer` entry. No `red — not this chunk's`.

**Watches:** the free-disk step in `Stand checks repeated`, not shown to be the cure of CI#41190771843's out-of-disk red · 2 green runs [CI#41233234207, CI#41235503227], the step and the job `success` on each, 2/3 toward its retirement. The overall red of the first run is `Test (macos)`'s, no disk fault.

**Outcome basis:** the operator pass ran. Commits from the parent `3af5a9bcf`: `8cd9c1286` and `715798ce2`. The verdicts above rest on the final HEAD's run in `chunks/2026-10-31-select-popup/evidence/ci.md`. Implement's report is `.andromeda/runs/2026-10-31T18-16-50-implement/outcome-2026-10-31-select-popup.md`, read whole, no later run superseding it, and this window holds no implement run. Implement's own friction records (7, `chunk` filtered) carry the same deviations. `evidence/ci.md` and that outcome are uncommitted and ride this wrap.

**For the later phases of this wrap, which reads this report alone:**
- **P2:** the sampler is forced (`detail.py sample --side wrap --force`, inputs#I8); classifiers at effort high, 2 draws, a dead draw one re-spawn, a second death stops and comes to the overseer (inputs#I1). Validate halts once for his answer card on the boundary widening, citing inputs#I4, per the playbook and CARRY `cry-musoudbr`; his F7 settles the group's `label` as a name.
- **P3:** the *Decisions & corrections* above; the last handoff's two deferred learnings still stand.
- **P5:**
  - The operator's row on the card, inputs#I7: five lines in `.andromeda/residuals.md` (the thumb not draggable · a highlighted row's contrast unread by the lint · a larger authored font clipping · the highlight absent from the snapshot and the tree · a single select's `size` unread) and six in `escher-0.4.0-incubator/deferred-from-0.3.0.md` (those five and Ctrl on `press`). That file is absent today (`ls escher-0.4.0-incubator`: `adapt-0.3.0-for-canvas.md`, `canvas-and-node-graph.md`, `README.md`).
  - **The `position_in_containing_block` panic needs an owner.** It is remote-reachable and no list's; a markerless-entry CARRY or a `residuals.md` line, the operator's to place if a trajectory question.
  - The WATCH's count reads 2 of 3 (`cry-rxreh5hz`).
  - CARRYs discharged: `cry-hzymz44x` (F9, `covered`, the two limits), `cry-nsqejhza` (`.gitignore:40-41`). `cry-musoudbr` discharges at P2's halt.
  - Merge surface: `evidence/merge-surface.md`, sixteen files, its line owned by Epoch 3's «Upstream sync ahead of the UI graph».
  - The pause stands again at this seam (inputs#I1), and only the founder lifts it.
  - `v030-15` owns the cold-agent run: the new `covered` clause and the list can move `wrong_calls`, unread here.
- **P7:** this chunk completes Epoch 2 — Engine debts (`rec.py` `epoch`: markerless 0, pending 1), so step 3b's consolidation fires and the evolve nudge is due. Coverage before the flip reads `verified 5/17 · deferred 0 · planned 11 · implemented 1 · unclaimed 11`. The light gate's `leg = operator` entries re-verify by `evidence/ci.md`; the smoke needs `WAYLAND_DISPLAY`.
- **P6:** no failed command open.

**Process hygiene:** implement's census read `census: 1 running`, a `cargo metadata` (pid 94460) not this run's tree, left running; `escher-session` and `seven_guis_native` terminated, session state directories ended by `stop`. Re-measured at this wrap: `gate.py census --comm escher-session --comm seven_guis_nati` → `census: none running`.
## Addendum — resumed 2026-10-31
- inputs#I9 · message: the operator, relaying as the overseer, 2026-10-31 21:26 local · copy message · n/a — the resume's terms: the sampler stays forced; classifiers read effort high and 2 draws, on max or fewer, or a Workflow size fault, the operator is told; one re-spawn for a dead draw, a second death comes to the operator; the widening card goes to the operator for the founder; P5 carries the P5 card's `## Beyond the plan` row 1, the `press` residual standing at `.andromeda/residuals.md:43`, the panic at `node.rs:1775` and no local gate building `font-embolden`, each with a proposed owner on the card, the panic's fix being his fork, and the free-disk WATCH's count.
  - bears on **P2:**: adds to it
  - bears on **P5:**: adds to it
- inputs.py verify, re-fired: inputs: 9 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 7 · uncited 1 · unparsed 0; moved since the report: uncited 1 → 0 by this block
## New text, by line
Generated by `cites.py added` (cites v1.8); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 3af5a9bc (the parent of the oldest pre-CI commit 8cd9c128) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/ci-leg.sh — added 2 line(s) in 1 range(s)
added: 59-60
### .github/scripts/test_ci_workflows.py — added 1 line(s) in 1 range(s)
added: 367
### .gitignore — added 2 line(s) in 1 range(s)
added: 40-41
### docs/capabilities.md — added 28 line(s) in 7 range(s)
added: 81-83 · 88 · 135-141 · 159-160 · 205-213 · 223-226 · 297-298
- 135-140 «- **a `select` list**: a click, Enter and Space open a list the engine paints over the page, past any box t…»
- 205-207 «- **The `select` model's limits.** An option of a closed select is not clicked: a driver click on one is re…»
- 208-210 «- **The highlight is unread.** The highlighted row is in no snapshot and no tree, so an agent pressing keys…»
- 211-213 «- **The list's thumb and one row height.** The thumb is painted and not draggable; keys, the wheel and `scr…»
- 223-226 «- **The `select` model's other limits.** A `multiple` select with no `size` lists every row; Ctrl+A is not …»
- 297-298 «- `aria-activedescendant` is unread (SC 4.1.2, outside tier 0), pinned by `accessibility_active_descendant_…»
### examples/seven_guis/tests/cli_commands.rs — added 47 line(s) in 1 range(s)
added: 419-465
- 419-464 @420 «fn a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses() {»
  - 421-424 @422 «let mut held = escher_driver::Session::start("flight-booker", || {»
  - 426-432 «let opening = Call {»
  - 434-437 «let shot = Call {»
  - 438-440 «let Ok(Outcome::Shot { png, .. }) = held.run(&shot) else {»
  - 444-447 «assert!(»
  - 449-451 «let hosted = string_at(&answer, "png")»
  - 452-455 «assert!(»
  - 457-462 «assert!(»
### examples/seven_guis/tests/common/mod.rs — added 30 line(s) in 1 range(s)
added: 425-454
- 425-453 @428 «pub fn option_needles() -> Vec<String> {»
  - 433-435 «let select = snapshot»
  - 436-440 «assert_eq!(»
  - 442-446 «for option in &select.children {»
  - 448-451 «assert!(»
### examples/seven_guis/tests/host_log.rs — added 117 line(s) in 2 range(s)
added: 17-20 · 282-394
- 282-393 @284 «fn a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace() {»
  - 298-308 «let mut host = Host(»
  - 313-321 «while attach(&dir).is_err() {»
  - 325-331 «let calls = [»
  - 332-344 «let answers: Vec<String> = calls»
  - 345-348 «assert!(»
  - 352-358 «let status = loop {»
  - 359-362 «let stdout = stdout»
  - 363-366 «let stderr = stderr»
  - 371-375 «assert!(»
  - 376-379 «let command_spans = stderr»
  - 380-385 «assert_eq!(»
  - 388-392 «assert!(»
### examples/seven_guis/tests/mcp_log.rs — added 51 line(s) in 2 range(s)
added: 12-15 · 68-114
- 68-113 @69 «fn a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace() {»
  - 75-77 «speaker.send(»
  - 79-87 «let calls = [»
  - 90-98 «for (index, (name, arguments)) in calls.iter().enumerate() {»
  - 99-102 «assert!(»
  - 108-112 «assert!(»
### packages/blitz-dom/assets/default.css — added 11 line(s) in 1 range(s)
added: 183-193
- 183-187 @185 «select[multiple][size] {»
- 190-192 «select[multiple][size] > optgroup > option {»
### packages/blitz-dom/src/accessibility.rs — added 5 line(s) in 1 range(s)
added: 238-242
### packages/blitz-dom/src/document.rs — added 340 line(s) in 6 range(s)
added: 10-11 · 343-344 · 562 · 761-1089 · 1408-1410 · 2404-2406
  - 761-795 @764 «fn select_list_allowed(&self, select: NodeId) -> bool {»
  - 797-804 @799 «pub(crate) fn live_select_list(&self) -> Option<&select_list::SelectList> {»
  - 806-809 @807 «pub(crate) fn select_list_for(&self, select: NodeId) -> Option<&select_list::SelectList> {»
  - 811-844 @813 «pub(crate) fn open_select_list(&mut self, select: NodeId) {»
  - 846-879 @848 «fn measure_select_list(&self, select: NodeId, rows: &[select_list::Row]) -> Option<(f64, f64)> {»
  - 881-889 @883 «pub fn close_select_list(&mut self) {»
  - 891-907 @893 «pub(crate) fn reconcile_select_list(&mut self) {»
  - 909-919 @910 «pub(crate) fn select_list_options(&self, select: NodeId) -> (Vec<NodeId>, Option<usize>) {»
  - 921-928 @922 «pub(crate) fn move_select_list_current(&mut self, option: NodeId) {»
  - 930-945 @932 «pub(crate) fn scroll_select_list_to(&mut self, node: NodeId) -> bool {»
  - 947-960 @948 «pub(crate) fn scroll_select_list_by(&mut self, delta: f64) {»
  - 962-988 @965 «pub(crate) fn select_list_hit(&self, x: f32, y: f32) -> Option<HitResult> {»
  - 990-993 @991 «pub fn open_select(&self) -> Option<NodeId> {»
  - 995-1012 @998 «pub fn select_list_bounds(&self, node: NodeId) -> Option<BoundingRect> {»
  - 1014-1020 @1016 «pub(crate) fn select_list_region(&self, node: NodeId) -> Option<BoundingRect> {»
  - 1022-1052 @1024 «pub fn select_list_view(&self) -> Option<SelectListView> {»
  - 1054-1064 @1057 «pub fn select_row_layout(&self, row: NodeId, text: &str) -> Option<parley::Layout<TextBrush>> {»
  - 1066-1088 «fn text_row_layout(»
### packages/blitz-dom/src/events/keyboard.rs — added 96 line(s) in 10 range(s)
added: 3 · 7 · 32-34 · 175-180 · 187-190 · 195-198 · 200-206 · 215-268 · 423-430 · 452-459
  - 187-190 «if doc.select_list_for(node_id).is_some() {»
- 215-250 @218 «fn press_open_list_key(»
  - 226-246 «let target = match (&event.key, select_step(&event.key)) {»
  - 247-249 «if let Some(option) = target.and_then(|index| options.get(index).copied()) {»
- 252-267 @253 «fn choose_current_and_close(»
  - 260-266 «if let Some(option) = current»
  - 423-429 @424 «fn carries_valid_size(&self, select: NodeId) -> bool {»
### packages/blitz-dom/src/events/pointer.rs — added 131 line(s) in 11 range(s)
added: 220 · 223-226 · 415-420 · 595-656 · 947-949 · 951-958 · 961-963 · 972-983 · 990-999 · 1108-1120 · 1194-1202
  - 415-418 @416 «if doc.select_list_hit(x, y).is_some() {»
- 595-611 @597 «fn hover_list_row(doc: &mut BaseDocument, x: f32, y: f32) -> bool {»
  - 598-600 «let Some(hit) = doc.select_list_hit(x, y) else {»
  - 601-603 «let enabled_option = doc»
  - 604-610 «match doc.select_list.as_mut() {»
- 613-617 @614 «fn pressed_in_list(doc: &BaseDocument) -> bool {»
  - 615-616 «doc.select_list_hit(doc.mousedown_position.x, doc.mousedown_position.y)»
- 619-633 @621 «fn close_list_outside(doc: &mut BaseDocument, x: f32, y: f32) {»
  - 622-624 «let Some(select) = doc.open_select() else {»
  - 626-631 «while let Some(node) = landed {»
- 635-640 @637 «fn click_lands_on_row(doc: &BaseDocument, row: NodeId, event: &BlitzPointerEvent) -> bool {»
  - 638-639 «doc.select_list_hit(event.page_x(), event.page_y())»
- 642-655 @644 «fn click_list_row(»
  - 650-652 «if doc.get_node(row).is_some_and(select::option_disabled) {»
- 1108-1119 @1109 «fn focus_select(doc: &mut BaseDocument, select: NodeId, dispatch_event: &mut dyn FnMut(DomEvent)) {»
  - 1112-1118 «generate_focus_events(»
  - 1194-1200 «if doc»
### packages/blitz-dom/src/lib.rs — added 4 line(s) in 1 range(s)
added: 108-111
- 108-111 «pub use node::{»
### packages/blitz-dom/src/node/mod.rs — added 2 line(s) in 2 range(s)
added: 11 · 33
### packages/blitz-dom/src/node/select_list.rs — new file · 539 line(s)
- 28-46 @30 «pub(crate) struct SelectList {»
- 48-57 @50 «pub(crate) struct Row {»
- 59-66 @61 «pub(crate) struct Placement {»
- 68-76 @70 «pub(crate) struct Geometry {»
- 78-91 @80 «pub struct SelectListView {»
- 93-108 @95 «pub struct SelectListRow {»
- 110-123 «impl SelectListView {»
- 125-130 @127 «pub(crate) fn size_rows(value: &str) -> Option<u32> {»
- 132-145 @134 «fn shows(node: &Node) -> bool {»
  - 135-137 «let carries_hidden = node»
  - 138-144 «!carries_hidden»
- 147-192 @149 «pub(crate) fn rows(doc: &BaseDocument, select: NodeId) -> Vec<Row> {»
  - 150-152 «let Some(holder) = doc.get_node(select) else {»
  - 154-190 «for child in holder.children.iter().copied() {»
- 194-199 @195 «pub(crate) fn group_label(node: &Node) -> Option<&str> {»
  - 196-198 «node.element_data()?»
- 201-208 @202 «pub(crate) fn row_label(node: &Node, row: &Row) -> String {»
  - 203-207 «if row.header {»
- 210-216 @211 «pub(crate) fn option_rows(rows: &[Row]) -> Vec<NodeId> {»
  - 212-215 «rows.iter()»
- 218-263 @223 «pub(crate) fn place(»
  - 231-232 «let (viewport_width, viewport_height) =»
  - 235-239 «let row_height = if row_height.is_finite() && row_height > 0.0 {»
  - 248-252 «let y = if opens_below {»
  - 254-262 «Placement {»
- 265-293 @266 «pub(crate) fn geometry(doc: &BaseDocument, list: &SelectList) -> Geometry {»
  - 270-276 «let placement = place(»
  - 278-283 «let scrollport = BoundingRect {»
  - 284-290 «let mut geometry = Geometry {»
- 295-375 «impl Geometry {»
  - 296-304 @297 «pub(crate) fn clamped(&self, scroll_y: f64) -> f64 {»
  - 306-314 @307 «pub(crate) fn row_rect(&self, index: usize) -> BoundingRect {»
  - 316-319 @317 «pub(crate) fn holds(&self, x: f64, y: f64) -> bool {»
  - 321-328 @322 «pub(crate) fn row_at(&self, x: f64, y: f64) -> Option<&Row> {»
  - 330-343 @332 «pub(crate) fn scroll_showing(&self, index: usize) -> f64 {»
  - 345-353 @346 «pub(crate) fn rows_in_view(&self) -> std::ops::Range<usize> {»
  - 355-374 @356 «pub(crate) fn thumb(&self) -> Option<BoundingRect> {»
- 377-379 «fn within(rect: BoundingRect, x: f64, y: f64) -> bool {»
- 381-384 @382 «pub(crate) fn row_disabled(node: &Node) -> bool {»
- 386-539 @387 «mod tests {»
  - 390-397 «fn rect(x: f64, y: f64, width: f64, height: f64) -> BoundingRect {»
  - 399-418 @400 «fn a_list_opens_where_its_rows_fit_and_else_on_the_side_with_more_room() {»
  - 420-437 @421 «fn a_list_is_at_least_its_selects_width_at_most_the_viewports_and_stays_inside_it() {»
  - 439-457 @440 «fn every_edge_of_a_placed_list_is_finite() {»
  - 459-475 @460 «fn size_reads_a_whole_number_above_zero_capped() {»
  - 477-494 «fn geometry_of(count: usize, scroll_y: f64) -> Geometry {»
  - 496-519 @497 «fn a_scroll_stays_within_the_rows_and_brings_a_row_in_by_the_least_movement() {»
  - 521-538 @522 «fn a_point_reads_the_row_it_lies_on_where_the_rows_are_scrolled() {»
### packages/blitz-dom/src/resolve.rs — added 3 line(s) in 1 range(s)
added: 133-135
### packages/blitz-dom/src/scrolling.rs — added 9 line(s) in 3 range(s)
added: 669-672 · 746-747 · 754-756
### packages/blitz-dom/src/stylo.rs — added 16 line(s) in 1 range(s)
added: 1124-1139
### packages/blitz-paint/src/render.rs — added 3 line(s) in 2 range(s)
added: 7 · 246-247
### packages/blitz-paint/src/render/select_list.rs — new file · 155 line(s)
- 14-18 @15 «fn fixed(rgb: [u8; 3]) -> Color {»
- 20-155 «impl BlitzDomPainter<'_, '_> {»
  - 21-40 @22 «pub(super) fn paint_select_list(&self, scene: &mut impl PaintScene) {»
  - 42-52 @44 «fn device_rect(&self, rect: BoundingRect) -> Option<Rect> {»
  - 54-83 @56 «fn paint_list_box(&self, scene: &mut impl PaintScene, view: &SelectListView, bounds: Rect) {»
  - 85-128 @87 «fn paint_list_row(&self, scene: &mut impl PaintScene, row: &SelectListRow) {»
  - 130-154 @133 «fn paint_list_thumb(&self, scene: &mut impl PaintScene, view: &SelectListView) {»
### packages/blitz-paint/src/text.rs — added 47 line(s) in 5 range(s)
added: 573-605 · 671-674 · 676-677 · 679 · 683-689
  - 574-583 «stroke_text_as(»
- 586-593 @588 «pub(crate) struct RunPaint {»
### packages/dioxus-native-dom/src/lib.rs — added 1 line(s) in 1 range(s)
added: 37
### packages/dioxus-native-dom/src/snapshot.rs — added 13 line(s) in 4 range(s)
added: 147 · 152-155 · 245-246 · 250-255
### packages/escher-driver/src/execute.rs — added 19 line(s) in 10 range(s)
added: 10 · 145-147 · 223 · 225-226 · 236-238 · 371-373 · 406 · 413 · 558-560 · 562
### packages/escher-driver/src/json.rs — added 2 line(s) in 1 range(s)
added: 868-869
### packages/escher-driver/src/lib.rs — added 2 line(s) in 1 range(s)
added: 64-65
### packages/escher-driver/src/lint.rs — added 16 line(s) in 7 range(s)
added: 203-204 · 247-249 · 339-342 · 351 · 368 · 372 · 405-408
  - 339-342 «let in_select_box = element.is_some_and(|element| {»
  - 405-408 @406 «if doc.select_list_bounds(element).is_some() {»
### packages/escher-driver/src/refusal.rs — added 14 line(s) in 3 range(s)
added: 20-21 · 76-81 · 261-266
### packages/escher-driver/src/schema.rs — added 15 line(s) in 3 range(s)
added: 222-226 · 240-243 · 264-269
  - 222-226 «help: "clicks the element an id names, then settles; a click on a select opens its list or \»
  - 264-269 «help: "presses one key on the focused element, then settles; on a closed select `enter` and \»
### packages/escher-driver/src/schema_tests.rs — added 19 line(s) in 3 range(s)
added: 98-110 · 119-123 · 136
### tests/blitz-tests/tests/select_interaction.rs — added 26 line(s) in 11 range(s)
added: 10-11 · 111 · 115 · 195 · 285 · 359 · 362 · 368-381 · 389 · 392-393 · 812
### tests/blitz-tests/tests/select_popup.rs — new file · 1186 line(s)
- 32-107 @36 «fn page() -> Element {»
  - 42-106 «rsx! {»
- 109-115 «fn number(harness: &Page, selector: &str) -> u32 {»
  - 110-114 «harness»
- 117-122 «fn counted(harness: &Page) -> (u32, u32) {»
  - 118-121 «(»
- 124-126 «fn value(harness: &Page, selector: &str) -> Option<String> {»
- 128-130 «fn open(harness: &Page) -> Option<NodeId> {»
- 132-139 @133 «fn highlighted(harness: &Page) -> Option<NodeId> {»
  - 135-138 «view.rows»
- 141-148 @142 «fn point_of(harness: &Page, rect: BoundingRect) -> (f32, f32) {»
  - 144-147 «(»
- 150-155 @151 «fn row_point(harness: &Page, selector: &str, at: &str) -> (f32, f32) {»
- 157-164 @158 «fn client_point(harness: &Page, selector: &str, at: &str) -> (f32, f32) {»
  - 159-161 «let rect = harness»
- 166-169 «fn click(harness: &mut Page, selector: &str, at: &str) {»
- 171-174 «fn click_row(harness: &mut Page, selector: &str, at: &str) {»
- 176-178 «fn named(key: NamedKey) -> Key {»
- 180-182 «fn space() -> Key {»
- 187-200 «fn open_by(harness: &mut Page, opener: usize, at: &str) {»
  - 188-199 «match opener {»
- 202-230 @203 «fn a_click_enter_and_space_open_the_list_on_its_chosen_option_and_fire_nothing() {»
  - 204-229 «for incremental in [false, true] {»
- 232-247 @233 «fn moves() -> [(Key, &'static str); 11] {»
  - 234-246 «[»
- 249-273 @250 «fn the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing() {»
  - 252-272 «for incremental in [false, true] {»
- 278-292 «fn choose_by(harness: &mut Page, chooser: usize, at: &str) {»
  - 279-291 «match chooser {»
- 294-327 @295 «fn enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change() {»
  - 296-326 «for incremental in [false, true] {»
- 329-361 @330 «fn tab_chooses_and_closes_before_focus_moves_either_way() {»
  - 331-360 «for incremental in [false, true] {»
- 363-375 @364 «fn a_choice_of_the_chosen_option_fires_nothing() {»
  - 365-374 «for incremental in [false, true] {»
- 377-388 @378 «fn a_disabled_row_changes_nothing_and_leaves_the_list_open() {»
  - 379-387 «for incremental in [false, true] {»
- 390-399 @391 «fn close_by(harness: &mut Page, closer: usize, at: &str) {»
  - 392-398 «match closer {»
- 401-426 @402 «fn escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event() {»
  - 406-425 «for incremental in [false, true] {»
- 428-441 @429 «fn a_labels_click_focuses_the_select_and_opens_nothing() {»
  - 430-440 «for incremental in [false, true] {»
- 443-472 @444 «fn a_disabled_inert_hidden_singular_or_empty_select_opens_nothing() {»
  - 445-451 «let rows = [»
  - 453-471 «for incremental in [false, true] {»
- 474-476 «fn attribute(name: &str) -> QualName {»
- 478-520 @479 «fn a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes() {»
  - 481-519 «for incremental in [false, true] {»
- 522-552 @523 «fn an_option_dropped_with_the_list_open_is_no_row() {»
  - 524-551 «for incremental in [false, true] {»
- 554-563 @555 «fn tree_node(harness: &Page, selector: &str, at: &str) -> TreeNode {»
  - 558-562 «tree.nodes»
- 565-614 @566 «fn the_tree_reads_the_list_open_on_the_select_its_focus_staying_there() {»
  - 570-613 «for incremental in [false, true] {»
- 616-626 @617 «fn stated_page() -> Element {»
  - 618-625 «rsx! {»
- 628-651 @629 «fn an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match() {»
  - 630-650 «for incremental in [false, true] {»
- 653-675 @655 «fn clipped_page() -> Element {»
  - 656-674 «rsx! {»
- 677-716 @678 «fn a_box_that_clips_the_select_clips_no_row_and_a_point_outside_reads_what_shows() {»
  - 679-682 «let rows = [»
  - 684-715 «for incremental in [false, true] {»
- 718-749 @719 «fn a_pointer_at_a_rows_centre_lands_on_it_with_the_page_scrolled_or_not() {»
  - 722-748 «for incremental in [false, true] {»
- 751-781 @752 «fn the_pointer_moves_the_highlight_to_an_enabled_row_and_a_later_key_stands() {»
  - 753-780 «for incremental in [false, true] {»
- 783-803 @784 «fn a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends() {»
  - 785-802 «for incremental in [false, true] {»
- 805-855 @806 «fn groups_read_as_headers_their_options_under_them_and_a_header_chooses_nothing() {»
  - 807-854 «for incremental in [false, true] {»
- 857-868 @858 «fn long_page() -> Element {»
  - 859-867 «rsx! {»
- 870-911 @871 «fn a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel() {»
  - 872-910 «for incremental in [false, true] {»
- 913-930 @914 «fn hostile_html() -> String {»
  - 915-917 «let options: String = (0..2000)»
  - 919-929 «format!(»
- 932-936 «fn finite(rect: BoundingRect) -> bool {»
  - 933-935 «[rect.x, rect.y, rect.width, rect.height]»
- 938-975 @939 «fn hostile_markup_keeps_every_box_finite_and_panics_nowhere() {»
  - 944-974 «for incremental in [false, true] {»
- 977-993 @978 «fn sized_page() -> Element {»
  - 979-992 «rsx! {»
- 995-1032 @996 «fn size_makes_a_multiple_select_that_many_rows_tall_and_a_key_brings_the_next_in() {»
  - 997-1031 «for incremental in [false, true] {»
- 1034-1037 @1035 «struct CountingShell {»
- 1039-1043 «impl ShellProvider for CountingShell {»
  - 1040-1042 «fn request_redraw(&self) {»
- 1045-1090 @1046 «fn opening_moving_scrolling_and_closing_each_ask_a_redraw() {»
  - 1047-1049 «let html = r#"<html><body><select id="pick">»
  - 1050-1089 «for incremental in [false, true] {»
- 1092-1103 @1093 «fn focus_moved_page() -> Element {»
  - 1094-1102 «rsx! {»
- 1105-1122 @1106 «fn a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened() {»
  - 1107-1121 «for incremental in [false, true] {»
- 1124-1134 @1125 «fn chosen_disabled_page() -> Element {»
  - 1126-1133 «rsx! {»
- 1136-1163 @1137 «fn a_click_in_the_selects_box_on_a_disabled_chosen_option_closes_as_on_any_other() {»
  - 1138-1162 «for incremental in [false, true] {»
- 1165-1186 @1166 «fn a_click_on_the_chosen_text_in_the_selects_own_box_closes_and_chooses_nothing() {»
  - 1167-1185 «for incremental in [false, true] {»
### tests/blitz-tests/tests/select_popup_painted.rs — new file · 351 line(s)
- 22-34 @24 «fn page() -> Element {»
  - 25-33 «rsx! {»
- 36-38 «fn enter() -> Key {»
- 40-46 @41 «fn open_by_key(harness: &mut Page, selector: &str) {»
- 48-52 «fn frame(harness: &mut Page, at: &str) -> Raster {»
  - 49-51 «harness»
- 54-61 «fn pixel(raster: &Raster, x: f64, y: f64) -> [u8; 3] {»
  - 56-60 «[»
- 63-68 «fn row_rect(harness: &Page, selector: &str, at: &str) -> BoundingRect {»
  - 64-67 «harness»
- 70-93 @72 «fn furthest_from(raster: &Raster, rect: BoundingRect, from: [u8; 3]) -> [u8; 3] {»
  - 76-79 «let (x1, y1) = (»
  - 80-91 «for y in y0..y1.min(raster.height) {»
- 95-103 @96 «fn assert_within_one(read: [u8; 3], expected: [u8; 3], at: &str) {»
  - 97-102 «for channel in 0..3 {»
- 105-117 @106 «fn ratio_of(text: [u8; 3], background: [u8; 3]) -> f32 {»
  - 108-112 «let html = format!(»
  - 114-116 «harness»
- 119-159 @120 «fn the_highlight_and_a_disabled_row_read_as_painted_against_their_constants() {»
  - 121-158 «for incremental in [false, true] {»
- 161-187 @162 «fn the_plain_pair_is_measured_at_the_default_sheet_and_at_the_stand() {»
  - 163-186 «for incremental in [false, true] {»
- 189-209 @191 «fn clipped_page() -> Element {»
  - 192-208 «rsx! {»
- 211-235 @212 «fn a_list_paints_whole_past_an_opacity_ancestor_and_a_box_that_clips_its_select() {»
  - 215-234 «for incremental in [false, true] {»
- 237-253 @238 «fn a_closed_selects_frame_is_byte_equal_before_opening_and_after_a_close_with_no_choice() {»
  - 239-252 «for incremental in [false, true] {»
- 255-267 @256 «fn grouped_page() -> Element {»
  - 257-266 «rsx! {»
- 269-280 @270 «fn ink(raster: &Raster, rect: BoundingRect) -> u64 {»
  - 274-278 «for y in y0..y1 {»
- 282-297 @283 «fn a_groups_header_is_darker_than_a_row_by_weight() {»
  - 284-296 «for incremental in [false, true] {»
- 299-310 @300 «fn long_page() -> Element {»
  - 301-309 «rsx! {»
- 312-351 @313 «fn a_scrolled_list_paints_its_thumb_which_takes_no_hit() {»
  - 314-350 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_popup.rs — new file · 423 line(s)
- 20-23 «use session_common::{»
- 25-27 «fn flight(incremental: bool) -> Session {»
- 29-36 «fn value(session: &Session, id: &str) -> Option<String> {»
  - 30-35 «session»
- 38-43 «fn bounds(session: &Session, id: &str) -> Option<(f64, f64, f64, f64)> {»
- 45-53 «fn differs(acted: &Acted, id: &str) -> Vec<Reading> {»
  - 46-52 «acted»
- 58-101 @59 «fn a_click_opens_the_list_and_a_click_on_a_row_chooses_it() {»
  - 60-100 «for incremental in [false, true] {»
- 103-125 @104 «fn the_keys_alone_open_move_and_choose() {»
  - 107-124 «for incremental in [false, true] {»
- 127-155 @128 «fn escape_closes_with_the_value_unchanged_and_the_rows_back_at_the_selects_box() {»
  - 129-154 «for incremental in [false, true] {»
- 157-166 @158 «fn a_closed_option_stays_refused_covered_and_changes_nothing() {»
  - 159-165 «for incremental in [false, true] {»
- 168-192 @169 «fn long_page() -> Element {»
  - 170-191 «rsx! {»
- 194-196 «fn hold_long(incremental: bool) -> Session {»
- 198-226 @199 «fn a_row_past_the_scrollport_is_off_screen_until_scroll_brings_it_in() {»
  - 200-225 «for incremental in [false, true] {»
- 228-245 @229 «fn a_button_under_the_list_is_refused_covered() {»
  - 230-244 «for incremental in [false, true] {»
- 247-285 @248 «fn type_leaves_no_list_open_and_chooses_by_its_text() {»
  - 249-284 «for incremental in [false, true] {»
- 287-316 @288 «fn a_shot_of_a_row_crops_to_it_and_leaves_the_list_open() {»
  - 289-315 «for incremental in [false, true] {»
- 318-350 @319 «fn the_lint_reads_no_overlap_on_a_row_and_the_planted_pair_still_reads() {»
  - 320-349 «for incremental in [false, true] {»
- 352-377 @353 «fn the_open_screen_stays_under_its_budget_and_every_actionable_element_keyed() {»
  - 354-376 «for incremental in [false, true] {»
- 379-406 @380 «fn two_clicks_on_one_point_open_and_close_each_focusing() {»
  - 381-405 «for incremental in [false, true] {»
- 408-423 @409 «fn a_screen_read_after_a_step_reads_the_new_state_with_no_advance() {»
  - 410-422 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_select.rs — added 24 line(s) in 6 range(s)
added: 6-7 · 25 · 175-176 · 185 · 187-203 · 227
### tests/blitz-tests/tests/stand_contrast.rs — added 28 line(s) in 8 range(s)
added: 12-13 · 286 · 288-289 · 304-316 · 436 · 448-454 · 731 · 748
  - 305-310 «(»
  - 311-316 «(»
### tests/blitz-tests/tests/stand_id_persistence.rs — added 1 line(s) in 1 range(s)
added: 98
### tests/blitz-tests/tests/stand_keyboard.rs — added 80 line(s) in 1 range(s)
added: 1055-1134
- 1060-1080 @1062 «fn list_walk() -> [ListStep; 10] {»
  - 1063-1067 «let (enter, escape, tab) = (»
  - 1068-1079 «[»
- 1082-1111 @1083 «fn the_flight_type_walk_holds_with_its_list_open_and_after_each_close() {»
  - 1085-1110 «for incremental in [false, true] {»
- 1113-1134 @1114 «fn a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it() {»
  - 1115-1133 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_lint.rs — added 28 line(s) in 1 range(s)
added: 492-519
- 493-519 @494 «fn an_open_lists_rows_read_no_overlap_on_the_stand() {»
  - 496-518 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_snapshot_state.rs — added 4 line(s) in 2 range(s)
added: 121-123 · 131
