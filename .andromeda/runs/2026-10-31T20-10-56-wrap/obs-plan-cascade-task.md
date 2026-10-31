# Task — obs-plan · 2026-10-31-select-popup

## The session
Added an engine-painted list to a single select: a click, Enter or Space opens it, keys and the pointer choose in it, across every document including remote pages, through two new modules (blitz-dom `node::select_list`, blitz-paint `render::select_list`), with no log site, span, field, log field, env var, listener or port added.
Added three checks (`select_popup` 25 tests, `select_popup_painted` 6, `stand_act_popup` 12) and one popup test each in `cli_commands`, `host_log` and `mcp_log`, the last two reading 0 option needles at `trace`.
Measured at the close: escher-driver 2 `info_span!` sites, the silence census 0, added lines clean, CI#41235503227 green 17/17 at `715798ce2`; `stand_*` files 51, `stand_act_*` files 16, `mod session_common;` declarers 32, `mod fixtures;` declarers 31.
Retired the limits on no painted popup, Enter and Space opening nothing on a single select, an unpainted group label and an unread `size` on a `multiple` select, plus the names `Entry.in_closed_select`, `Target.multiple` and `focus_multiple_select`.
Widened one boundary: a page's option and group labels now leave as pixels in a shot of an open list, held for P2's halt.
The report's Changes name 41 changed files and this list caps at 40: `packages/blitz-dom/src/node/mod.rs` is left out, its change being two lines at 11 and 33 that declare the new module; the chunk's own evidence files are not listed.

## Reached files
- packages/blitz-dom/src/node/select_list.rs
- packages/blitz-paint/src/render/select_list.rs
- tests/blitz-tests/tests/select_popup.rs
- tests/blitz-tests/tests/select_popup_painted.rs
- tests/blitz-tests/tests/stand_act_popup.rs
- packages/blitz-dom/src/document.rs
- packages/blitz-dom/src/resolve.rs
- packages/blitz-dom/src/accessibility.rs
- packages/blitz-dom/src/scrolling.rs
- packages/blitz-dom/src/stylo.rs
- packages/blitz-dom/src/lib.rs
- packages/blitz-dom/src/events/keyboard.rs
- packages/blitz-dom/src/events/pointer.rs
- packages/blitz-dom/assets/default.css
- packages/blitz-paint/src/render.rs
- packages/blitz-paint/src/text.rs
- packages/dioxus-native-dom/src/snapshot.rs
- packages/dioxus-native-dom/src/lib.rs
- packages/escher-driver/src/execute.rs
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/refusal.rs
- packages/escher-driver/src/schema.rs
- packages/escher-driver/src/schema_tests.rs
- packages/escher-driver/src/json.rs
- packages/escher-driver/src/lib.rs
- tests/blitz-tests/tests/select_interaction.rs
- tests/blitz-tests/tests/stand_act_select.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_id_persistence.rs
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- examples/seven_guis/tests/cli_commands.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/mcp_log.rs
- examples/seven_guis/tests/common/mod.rs
- docs/capabilities.md
- .gitignore
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Touches
- §3 Logging stack, obs-plan:76, three mentions: the `stand_act_*` count moves to sixteen (15 → 16 files), `stand_act_popup` the sixteenth; the line's fourteen was already behind at the base.
- §3, obs-plan:76, the `session_common` reader count: `mod session_common;` declarers read 31 → 32 at the anchored pattern, so the line's twenty-nine is behind; `stand_act_popup` is a reader.
- §3 absence rows, obs-plan:92, the third shared module's 28 readers: `mod fixtures;` declarers 28 → 31 at the anchored pattern; an unanchored grep reads one more, the module's own doc comment.
- log-coverage:75, the row naming the select readers: its silence holds; `focus_multiple_select` is now `focus_select`, `Target.multiple` is now `Target.select`, `is_multiple_select` is read by `closed_select_of` alone within the workspace, `clear_select_typeahead` has one non-test caller, `focus_select`.
- The one standing obs-plan site on a closed select's answer: `closed_select_of` now answers `None` for a select whose list is open, and an option of a closed select is still refused as `covered`.
- §3 absence rows, obs-plan:95 and the two further select-model silence rows: re-read, unmoved in claim; `select_interaction` still 18 tests, `stand_act_select` 10, `apply_select_choice` still the one write path, the two files edited and the silence holding.
- obs-plan:95's description of the select model as the keyboard arm, its typeahead, the click arm and the mutator's settle: both arms now also open, move and close a list, so a row listing the model's parts may need the list beside them.
- Any obs-plan text that states a retired limit (no popup painted, a group's label unpainted, `size` unread on a `multiple` select, Enter and Space opening nothing): the expected amendments for obs-plan name a group's label painted.
- §3 absence rows, obs-plan:93, the visual lint: `Entry.in_closed_select` is now `in_select_box`, true also for an open list's row or group; the citation `lint.rs:176-232` moved (lines added at 203-204); `stand_lint` gains one test.
- Silence statements over the edited checks: `stand_contrast` (`STATE_ROWS` 18 → 20), `stand_keyboard` (21 → 23 tests), `stand_lint`, `stand_snapshot_state`, `stand_act_select`, `select_interaction` and `stand_id_persistence` (one line added at 98); the gate's census reads 0.
- §3 Logging stack, obs-plan:80, what the sink-reading checks drive: `host_log` no longer ends at its `cells` test, `mcp_log` and `cli_commands` each gain a test, each spawning the same sink-installing binary; the count of checks spawning it does not move.
- §3, obs-plan:81 and obs-plan:91, the seven_guis shared module: still shared by the same targets, now also holding `option_needles`; the citations `common/mod.rs:371-544`, `mcp_log.rs:53-315` and `host_log.rs:419-438` moved (lines added at common 425-454, mcp_log 12-15 and 68-114, host_log 17-20 and 282-394).
- §4 Span coverage, re-read and unmoved: escher-driver 2 `info_span!` sites and no event, `command`'s eight fields (`stand_act_spans`), `tool_call`'s two fields, `VERBS` 12, causes 8, though `execute.rs` was edited.
- obs-plan:75, refusal detection in front of `click` and `type`: `covered`'s meaning gains a clause on an option of a closed select, its name, remedy and order unchanged; `type` on any select focuses with no click; a closed word on the span, no call text.
- obs-plan:75, `Outcome::Shot` and `Outcome::Linted` never logged or fielded: re-read unmoved, the citations `json.rs:310-314` and `json.rs:367-371` hold (json.rs's lines added at 868-869 only), the shot's fields unmoved.
- Citations into `packages/blitz-dom/src/document.rs` moved by the 340 added lines (10-11, 343-344, 562, 761-1089, 1408-1410, 2404-2406): `:1807` in §2, `:2237-2238` in the upstream focus row, `:1717` and `:1804-1823` in §7, `:1808-1813` in §8.
- Citations into `packages/blitz-dom/src/events/pointer.rs` moved by 131 added lines: `:1024-1030` in §1, `:1025` and `:1030` in §8.
- Citations into `packages/blitz-paint/src/render.rs` moved by lines added at 7 and 246-247: `:248-278` and `:1353-1369` in §1, the devtools overlays now following `paint_select_list`.
- Citations that hold because the edit sits after them or outside them: `blitz-dom/src/lib.rs:26-29` (added 108-111), `blitz-dom/src/resolve.rs:77` (added 133-135), `stand_id_persistence.rs:1-3` (added 98).
- §1, obs-plan:13, blitz-dom's six `tracing` call sites: the count holds, the chunk's added `packages/*.rs` lines holding no tracing token, though `events/pointer.rs` is one of the cited files.
- §8, obs-plan:254, the sink's needle statement of 0 id, name and typed-text needles and 0 bytes of a shot: `host_log` and `mcp_log` now also read 0 option needles at `trace`.
- §8, obs-plan:266, the cold-agent transcript's contents: a shot in it can now hold painted option and group labels, indirectly through the list's pixels; the flight booker's options were already named there.
- §3, obs-plan:98, no wall-clock read in blitz-dom outside `clock.rs`: re-read and unmoved through the new list state (gate entry 6, last line 0).
- §9 and any row reading `ci-leg.sh`: `stand-repeat`'s named words go fifteen → seventeen (`select_popup`, `select_popup_painted`, at 59-60), `test_ci_workflows.py:367` asserts them; no leg added, the `a11y` leg stays fourteen files, no verdict shape moved.
- §3 and §6, the agent-run log: `stand_act_popup` joins `run stand` by its prefix, raising what `run stand` reports to 237 · 0 · 7; the harness's event shapes are unmoved.
- §6, the cold-agent verdict's `wrong_calls`: the new `covered` clause and the list can move its value, unread at this session and owned by `v030-15`; the field's definition does not move.
- §8 and §9 statements that local state is gitignored: `.gitignore` gains two lines at 40-41, a route and incubator matter, moving none of them.

## New facts
- F1 · log-coverage · The popup checks `select_popup`, `select_popup_painted` and `stand_act_popup` (a `session_common` reader) are silent: the silence census reads 0 over them, gate entry 2.
- F2 · log-coverage · The list's two modules, blitz-dom `node::select_list` and blitz-paint `render::select_list`, hold no tracing or log token (gate entry 5), and the chunk's added `packages/*.rs` lines hold none (gate entry 4).
- F3 · log-coverage · `host_log`'s `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, under `#[cfg(unix)]` and the state directory `hl-popup`, reads a sink-installing host at `trace`: 0 option needles, its `command` span lines counted.
- F4 · log-coverage · `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace` reads the `mcp` role at `trace` with 0 option needles and records nothing, so no `ml-popup` directory exists.
- F5 · log-coverage · The seven_guis shared module holds `option_needles` (examples/seven_guis/tests/common/mod.rs:425-453), a select's option labels as needles for the popup log tests.
- F6 · log-coverage · `cli_commands`' `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, under the state directory `cc-popup`, reads a hosted shot of an open list equal to the in-process one.
- F7 · log-coverage · dioxus-native-dom's new public `is_select`, re-exported and read by `execute.rs`'s `locate` alone, joins the readers log-coverage:75 names, its silence holding.
- F8 · log-coverage · The driver's restated fixed texts (`click`'s, `type`'s and `press`'s help and `covered`'s meaning) are the program's own `'static` text, holding no call text and adding no log site or span field.
- F9 · log-coverage · The `Focussed node` line's count is unsettled: 0 lines through the sink on the workspace build with no base reading, so neither moved nor unmoved.
- F10 · pii-scrubbing-compliance · A shot of an open list carries a page's option labels and group `label`s as pixels, a boundary widening held for P2's halt, the shot's own fields unmoved.
- F11 · pii-scrubbing-compliance · An open list's row crosses the snapshot and the diff by its own `bounds`, `differs` naming `bounds` within its closed set, no new field, key or carrier added.
- F12 · pii-scrubbing-compliance · `SelectListRow.label` holds page text in a public read type and is never logged; a label leaves only as a painted pixel and the snapshot's existing `name`.
- F13 · pii-scrubbing-compliance · `size` on a `multiple` select is a new attribute read, parsed into a whole number capped at 65534, carrying no text out and moving no redaction shape.

## Untouched
- escher-telemetry's sink, line format, target allowlist, scrub and chaining panic hook: `packages/escher-telemetry` is outside the change, and no log field or span was added for them to meet.
- The panic at `node.rs:1775` on a key press after an ancestor gains `hidden`: its owed row is in security-plan §Error Handling with its owner at P5; `node.rs` and `events/mod.rs` are unchanged, and §7's panic hook rows stand.
- The subscriber install sites and the command line's endings: `examples/seven_guis/src/main.rs`, `session_host.rs` and the driver's `cli.rs` are outside the change, `init` keeping its two install sites.
- `scripts/agent-run.sh` and `scripts/cold-agent.sh`: their event shapes, verdict fields and state areas are unchanged; only the values a run reports can differ.
- Upstream and tool emitters and their as-is values: apps/browser, blitz-net, blitz-html, blitz-shell, blitz-vibey-script, dioxus-native and wpt/runner are outside the change.
- §2's feature wiring and §5's dependency absences: every manifest, `Cargo.lock` and `deny.toml` are unmoved, the preservation guard green.
- §5's `debug_timer`, the browser's FPS overlay, the bench and example timings and the WPT scores: no metric was added or moved, and those sources are outside the change.
- §9's artifact, publish and package-install rows: `.github/workflows/` and `.github/scripts/apt-install.sh` are outside the change, and no artifact was added.
- §10, §11 and §12: no recorded intent, and the session records none for them.
