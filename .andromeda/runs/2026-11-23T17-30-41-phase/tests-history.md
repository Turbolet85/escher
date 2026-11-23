@history_item n=1 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (amd-ipy6bzhk, 2. Test Strategy: 2 rows) — added `decision/canvas-shots-cross-os-by-digest` (`stand_canvas_cross_os`: three canvas-alone fixtures, whole and element shots, both layout modes, answer digests byte-equal to one committed table, no pixels committed) and gave `decision/no-shot-in-cross-os-byte-checks` its canvas-alone exception, on the founder's F1 «byte equality required». Circle Drawer's canvas element shot joins exactly this table (the operator's P5 watch); its whole screen holds page text and stays under the ban.

@history_item n=2 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (amd-5xumfbc5, amd-gplax6l7, amd-i2ckej2w) — `decision/cross-os-checks-hold-no-shot` and `decision/no-shot-in-cross-os-byte-checks` gained the canvas-alone exception, and their reason was narrowed to what was measured: a shot holding document text differs between linux and macOS, a canvas-only shot reads equal on all three hosts. A Circle Drawer canvas holding glyphs drawn from the bundled fonts has to be a canvas alone, with no page text in its crop, to come under the exception.

@history_item n=3 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (amd-d26vi7ce, amd-tqpvxvpt, amd-wsteqwoi) — the same exception was added to `decision/no-screenshot-in-cross-os-checks` (session-lifecycle), and the reason of it and of `decision/no-screenshot-step-in-committed-checks` was narrowed to shots that hold document text. Committed recorded checks and Circle Drawer's flow checks still hold no `screenshot` step; only the canvas element digest crosses OSes.

@history_item n=4 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (amd-cvy4bwz6, amd-lbkskdc6, 6. E2E Test Strategy) — the cascade sweep narrowed the reason of `decision/checks-hold-no-screenshot-step` and `decision/no-screenshot-in-recorded-check-scripts` to document text. The precedent: a cascade sweep reaches every no-shot row whenever a shot's cross-OS reach moves again.

@history_item n=5 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** (amd-5e244nch) — added `decision/app-widget-shows-in-shots` (guard `stand_canvas_shot`): a Rust-painted widget shows in harness and driver shots, whole and cropped, both layout modes, per package and in the workspace build. Circle Drawer's shot check extends this witness to a real sample's circles and canvas text.

@history_item n=6 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (amd-siqngma2) — this is where the same-host limit started: `stand_screenshot_repeat`'s byte identity was restated as one-host-only (CI#39225375577, linux vs macOS), so a check compared byte for byte across OSes holds no `screenshot` step. It is the root of the no-shot rows the canvas exception now qualifies.

@history_item n=7 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (amd-uimtzpsg) — added `decision/cross-os-readings-by-answer-digest` (served tasks read by digest against one committed table, no shot) and gave `decision/stand-checks-bundled-font` its guard `stand_bundled_font_families`. Circle Drawer's canvas text comes from the document's font context (`Widget::fonts`), so it rests on the bundled DejaVu Sans with system fonts off.

@history_item n=8 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (amd-mmdtifvl) — the cross-OS readings table moved from six to seven served tasks, and `decision/count-and-rename-sweeps` gained its keep-a-count-worded-witness exception. Circle Drawer is not served (F1 A), so neither the served set nor that table should move. A sweep still has to run where Circle Drawer's rows move.

@history_item n=9 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (amd-yfp5ovdp) — added `decision/count-and-rename-sweeps` after a missed numeric pin and a missed renamed witness. This chunk changes counts (the `CLICKABLE` row, `stand_canvas_cross_os`'s digest rows, the shared tables) and may rename tests, so the sweep applies.

@history_item n=10 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (amd-n2omhba3) — `decision/count-and-rename-sweeps` widened to CI leg file counts held as literals in CI-script tests. A new engine check or stand file that joins a leg moves those pins.

@history_item n=11 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-aj6rkfgc) — the snapshot's `clickable` reading arrived and `stand_non_lean_tasks` gained the 7-row `CLICKABLE` table (elements reading `clickable` with no role), with Circle Drawer at 1: today's canvas `div`. The canvas `<object>` reads role `Unknown`, `clickable`, so this pin moves, and `stand_non_lean_tasks.rs` is in scope.

@history_item n=12 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (amd-iygsbz5o) — added `stand_non_lean_tasks` (3) and the non-lean pin, and `stand_requirement_sweep`'s `WITNESSES`. Non-lean tasks, Circle Drawer among them, are opened from Home by their card. This is where the file this chunk extends came from.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (amd-dbogce46) — the non-lean reacting-element pin moved 676 → 677 when `cells-grid` gained an `onkeydown`. The precedent: a pin over the listener-marked elements moves with any element gaining a listener, and the move is recorded with its reason against the forecast.

@history_item n=14 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-hf3zl3jw) — stated handler coverage's limits, the first being that a handler inside a canvas is not counted, deferred to 0.4.0, mapped in `docs/capabilities.md`. Circle Drawer's circles move into a canvas; the app's `onclick`/`ondoubleclick` on the `<object>` stay DOM listeners, while what the widget hears is uncounted (`contract/handler-coverage-limits/canvas-handler`, `decision/handler-coverage-canvas-uncounted`).

@history_item n=15 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-i7b3rmrc) — added `stand_handler_coverage` with `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES` (seven tasks), held equal to the graph's reading at boot, and `GRAPH_DRIVES` read by both graph checks. Circle Drawer's per-circle `div` handlers become one canvas element's, so its rows in these shared tables move (scope: `tests/common/mod.rs`).

@history_item n=16 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-z23dex63) — `tests/common/mod.rs` gained `GRAPH_DRIVES` (seven rows, each conditional write driven once so no stand write reads `unconfirmed`). `decision/graph-drives-confirm-every-write` binds Circle Drawer's drives when its writes move to canvas clicks.

@history_item n=17 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-ckioxq2y) — stated the `unconfirmed` limit: a declared write whose handler's event passed through its element without the write running reads `unconfirmed`. This is `decision/unrun-declared-write-reads-unconfirmed`. The double-click resize path on the canvas is such a conditional write.

@history_item n=18 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (amd-lfwo5vi7, amd-kmmmilr5, amd-gpqjgetu) — `decision/handler-coverage-bubbling-counts-ancestors` was restated and renamed: a dispatch still propagating counts every listener it passed, a stopped one its innermost listener alone, and an unwound panic counts as stopped. A click synthesised on a custom-widget target has to bubble and stop as a normal click does for coverage to read it.

@history_item n=19 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (amd-2ho3wh7t) — measured that the bridge's listener attributes over-report and that the vdom walk's dynamic attributes read the current listeners exactly. This grounds `decision/listeners-read-from-vdom-walk`, which `clickable` and actionable on the canvas `<object>` rest on.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-x5hh3g7d) — one actionable rule for both readers (`unkeyed_actionable` and `actionable_nodes`), read from current listeners and never from `data-dioxus-id`. That is `decision/one-actionable-rule`. Boot keys must read `unkeyed_actionable` 0 on Circle Drawer's screens.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-a2uwkadt) — pinned `NodeState.clickable` true where the current listeners hold `click` and the element is not a disabled form control. That is `contract/snapshot-node/clickable`, which the canvas `<object>` with an app `onclick` reads.

@history_item n=22 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (amd-ybvlv3ad, amd-6u4kftng) — `advance` accepted on every session, moving the clock by exactly `ms`, with `timer_step` carrying its remainder; `stand_virtual_clock` reads two clicks with no `advance` as a double click. The F4 limit, where a second centre click with no `advance` is a double click, rests on `decision/two-clicks-without-advance-double-click`, and the engine's synthesis on a custom-widget target has to read the same virtual clock.

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (amd-oodkokb2) — the founder ratified the one-virtual-clock batch: `advanced_ms` is exactly `ms`, the harness's time is not moved by `screenshot`, `screenshot_rect`, `text_contrast` or `settle`, and the double click comes from the virtual clock. The clock rows picked here stand ratified.

@history_item n=24 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (amd-knbkpfwu) — `decision/mutation-controlled-tests` gained bound loosening for a control that removes a bound. Each new control in this chunk (the engine click on a canvas, the stand shot, the flow, the digest rows) follows it.

@history_item n=25 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (amd-stldjcwi) — `decision/mutation-controlled-tests` was restated: a case a mutation leaves green is widened until it turns red. This is the rule for the engine click-synthesis control, where other arms may route around the guard.

@history_item n=26 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (amd-z6xuttzw) — added `decision/expected-value-pinned-as-literal`: a check pins its measured value as a literal and never derives it from the constant its control mutates. The `CLICKABLE` count, the digest table and the click counts are pinned this way.

@history_item n=27 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (amd-xnhp3j2e) — added the font-embolden exception to `decision/expected-value-pinned-as-literal`: a pixel-row pin that the feature moves gets a stated band. Canvas glyph pixels pinned in a stand check may meet the same feature.

@history_item n=28 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (amd-yx45xaju) — added the `embolden` step to `fast` after a macOS-only red on a paint change under `blitz-paint/font-embolden`. The trap: canvas text paint read green locally can redden `Test (macos)`.

@history_item n=29 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (amd-fcxxug5t) — added `decision/no-change-read-control-needs-pending-work`. A control of a read that must change nothing, such as Circle Drawer's shot, lint or graph reads, needs pending work.

@history_item n=30 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (amd-3quv32td, amd-ptvfk25e) — added `contract/stand-boot/boot-alone`, `Harness::from_component_alone`, `vdom_alone`, `decision/component-alone-props-as-maker` and the root-context-before-build rule. These are the precedent and the API for Circle Drawer's new in-process stand boot in `examples/seven_guis/src/stand.rs`.

@history_item n=31 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (amd-b4vvhst7) — added `decision/harness-graph-tests-enable-accessibility`: a per-package test build compiles the UI graph out unless the dev-dependency turns accessibility on. Any unit test of Circle Drawer's graph or the widget's role and name must enable the feature.

@history_item n=32 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (amd-knnsalwo) — added `contract/stand-boot/boot-order`, the stand's newest boot beside the others. It is the precedent for a new stand boot row; Circle Drawer's boot is in process only and not served.

@history_item n=33 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (amd-sam3o45d) — added `stand::boot_cells` and `boot_load`, served-page checks that declare `session_common` and boot through TaskShell, and `net_provider: None` for every boot but `boot_load`. This is the precedent for a stand boot of a non-lean task.

@history_item n=34 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (amd-umaactff) — `decision/stand-boots-offline` names its exceptions, the load page and the failing page. Circle Drawer's new boot keeps no network provider.

@history_item n=35 marker=2026-11-20-stand-test-setup-shared
**2026-11-20-stand-test-setup-shared** (amd-ke4vohyd) — `decision/shared-stand-tables-in-one-module` gained its exception: a helper whose body or message differs stays in its check, and `stand_element_ids` keeps its own boot. Circle Drawer's rows and helpers moving into `tests/common/mod.rs` follow it.

@history_item n=36 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** (amd-nlavi4cc) — added the shared `fixtures` module (`boot`, `hold`, the shared pages; no print, env read or network). `decision/shared-fixtures-in-one-module` binds any fixture page that more than one Circle Drawer check reads.

@history_item n=37 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (amd-wzxmumz2) — re-pinned the stand's shared tables and `stand_lint`'s boot findings when a stand control moved to a select. This is the precedent for re-pinning `controls`, the tab order, contrast and lint rows when a sample's element moves, here the canvas `div` to an `<object>`.

@history_item n=38 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (amd-flbkdilp) — added `stand_lint` and `Defect::ALL`, the six defects in their stated order (the `contract/lint-defects/*` rows), each lean task reading 0 at boot. Circle Drawer's boot-lint-clean criterion, `Session::lint` with 0 findings in both modes, reads through this.

@history_item n=39 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (amd-3nw7osql) — added `decision/focus-test-pins-where-focus-lands`. A canvas `<object>` takes focus on a click (`events/mod.rs:200-202`), so a check of focus after a canvas click pins the node focus lands on.

@history_item n=40 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** (amd-nsqxmuql) — `decision/complexity-ceiling-host-only` gained its exception: the engine's click handler is held below the ceiling, upstream's arms included. F2 A's click and double-click synthesis on a custom-widget target edits blitz-dom's events, so the handler must stay below 15.

@history_item n=41 marker=2026-11-20-codebase-health
**2026-11-20-codebase-health** (amd-ihzt4hfv) — `decision/complexity-ceiling-host-only` was ratified for upstream-authored functions (F2). An escher-owned function added to blitz-dom's events for the synthesis falls under the ceiling's reading, not the upstream exemption.

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (amd-jyrjp4tc) — the engine's `change` on a checkbox and a radio, synthesised in blitz-dom with its own both-modes targets (`checkable_change_event`, `text_range_no_change_event`). This is a precedent for an engine event synthesis held by a new engine target in both layout modes.

@history_item n=43 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (amd-cixe5kae) — `stand-repeat` runs named non-`stand_` targets as literal `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds`. A new engine check of the canvas click with no `stand_` prefix reaches stand-repeat only by being named there.

@history_item n=44 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (amd-4oiy4oeg) — added `decision/requirement-witness-inside-stand-repeat`: every requirement names its witness inside stand-repeat, in a table pinned by function name. matrix#v040-09's claim needs a witness there.

@history_item n=45 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (amd-wb5mdgi4) — `pseudo_element_flow_classification`'s both-modes exception ended, and the record states which legs reach a new engine target. This is the precedent for recording the legs that reach a new engine check.

@history_item n=46 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (amd-gkbc5lsj) — the both-layout-modes rule's only exception is named upstream targets. An escher-written engine check of the click on a canvas runs both modes (`decision/stand-checks-in-both-layout-modes`).

@history_item n=47 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (amd-uqva53q6) — `contract/driver-session/acting-verbs` gained `caused`. A driver click on the canvas returns the diff, plus `caused` where the step caused something.

@history_item n=48 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (amd-wudwalw5) — `contract/driver-session/graph` (a whole graph, or `Outcome::GraphPart`) and `contract/session-wire/graph-answer` gained part counts. These are the shapes the Circle Drawer graph and flow checks read.

@history_item n=49 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (amd-rrbdvino) — the harness and session contracts gained the shot: `screenshot` and `screenshot_rect` at the harness's own time, composited over white, no font refused with nothing painted, an element shot cropping to no pixel `off-screen`. These are the `contract/harness-screenshot/*` and `contract/driver-session/screenshot` rows the canvas shot checks read.

@history_item n=50 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (amd-bcwcmey4) — `decision/png-decoding-in-checks` now covers encoding too. Shot checks in blitz-tests decode through the png dev-dependency; seven_guis uses its hand reader.

@history_item n=51 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (amd-xt3nypar) — added `decision/timer-context-read-by-delivered-ticks`, a context checked by delivered ticks after a check by elapsed text could not fail. It is related to `decision/timer-driven-by-delivered-ticks` among the picked rows.

@history_item n=52 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** (amd-472linzq) — added `test_capability_statement.py` and the claims-mapped-to-checks group of `docs/capabilities.md`. This chunk edits that statement (the canvas sample and its stated limit), so its pinned names and counts must still read green.

@history_item n=53 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (amd-m7fw5fjq, amd-zn44tekk, amd-aiakktz6, amd-755msr7u) — the capability pin now also holds the a11y leg file count and stand-repeat's named-target count against `ci-leg.sh`. Adding a named stand-repeat target for the engine check reddens ci-scripts until `docs/capabilities.md` restates it.

@history_item n=54 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-krjdo5z6) — `docs/capabilities.md`'s claim map gained its select-model limit bullets with their reader checks. This is the precedent for stating Circle Drawer's centre-click limit (F4) with the check that pins it.

@history_item n=55 marker=2026-11-18-a11y-re-read
**2026-11-18-a11y-re-read** (amd-j45pb4zi) — `decision/a11y-leg-membership` became criterion-only (SC 2.1.1 or 2.4.3 witness, tree reading, or recorded gap). A check of the canvas's `role="group"` «Drawing area» joins the a11y leg only if it meets that criterion.

@history_item n=56 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** (amd-2gzx73ga) — stand checks assert unconditionally under bundled DejaVu Sans with system fonts off, with no skip path. This is the origin of `decision/stand-checks-bundled-font-no-skip` and `decision/stand-pinned-real-font`, and it holds the canvas-text shot to the bundled font.

@history_item n=57 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** (amd-jhx4wjqd) — settle beside pump; a fetching `net_provider` is wrapped in the load counter, and `contract/session-act/settle` was set. This is the origin of `decision/harness-counts-fetching-providers-only` and the act or settle contract that the sample's flow steps return through.
