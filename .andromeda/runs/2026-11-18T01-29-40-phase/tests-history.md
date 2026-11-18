@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (CI on the build branch, legs through ci-leg.sh) — every linux CI job runs one leg of `.github/scripts/ci-leg.sh`, the same command the dev host runs; the origin of decision/ci-legs-host-reproducible, so the `a11y` leg's re-assertions must stay reproducible by `bash .github/scripts/ci-leg.sh a11y` on the host.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — created the `a11y` leg (`--test accessibility_hidden --test accessibility_roles --test focusability_updates`) with `LegScriptTest` pinning it in `test_ci_workflows.py`; the leg this chunk's checks run in, and its pin the site any new `--test` word must also land in.

@history_item n=3 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, the pre-existing font tests alone skipping; origin of decision/stand-checks-bundled-font(-no-skip) and decision/stand-pinned-real-font, which bind every re-read contrast and focus reading here (no skip path allowed).

@history_item n=4 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — `stand_accessibility_ids` (roles, names, Tab order unchanged) and `accessibility_names` added, riding the workspace `test` leg and not the `a11y` leg; `accessibility_names` stays outside the leg to this day, a membership precedent for this chunk.

@history_item n=5 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — trap: a headless key press is not the same on every platform (the editor's Backspace arm is compiled out on macOS, green on Linux, red on macOS CI); prove a state change by clicks and typed characters, binding any new keyboard re-assertion.

@history_item n=6 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the stand checks' shared module `tests/blitz-tests/tests/common/mod.rs` with its `controls` and `rendered` tables, the rows `stand_contrast` and `stand_keyboard` later iterate; a 0.3.0 surface extended here lands in those tables, not restated in-file. Trap: a sed/grep address ending at a function name also selects every item whose name opens with it.

@history_item n=7 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — `settle` stated (quiet pass, `NotSettled { busy: Loads }` at once, `SETTLE_PASS_LIMIT` 64, no clock read, no time advanced, changed set untouched); the source of the contract/harness-settle/* rows and decision/settle-never-advances-time. Traps: a delivered tick after an input helper needs a pass of its own; a fresh harness's changed set is non-empty.

@history_item n=8 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the harness re-exports `Key` and `Modifiers`; `apple_keybinding` listed; rules set: a driver check's failure message names a layout mode, task and index, never an id or what a screen reads; no deleting key in any check but `stand_act_keys`' one — rules any new keyboard witness inherits.

@history_item n=9 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the same check file read green per package and red in the workspace leg: a green per-package run is not the workspace leg's green (decision/per-package-green-not-workspace-green); a re-assertion must read green under both builds.

@history_item n=10 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (the CI-scripts leg; `BlitzTestsTargetsTest`; engine feature split) — `test_blitz_tests_targets.py` pins one target per file and the unbuilt `tests/all.rs`, a file this chunk's scope names; the workspace build compiles a different bounds-reader body (`writing-mode`) than the per-package build, so a stand check's bounds reading differs by runner.

@history_item n=11 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — a raw harness `click` on an element outside the padding box of a box that clips it no longer lands (contract/harness-input/clipped-click); layout_rect/center_of on a scrolled box. Trap: a check that leaned on the old hit is found only by running the stand gates, no sweep word finds it.

@history_item n=12 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (three engine checks) — each of the bounds reader's two bodies holds its own check, read under its own build only (decision/bounds-reader-bodies-each-checked); a focus or hit re-reading that touches bounds must hold under both runners.

@history_item n=13 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — `accessibility_states` added, run by the workspace `test` leg and not the `a11y` leg; the platform-adapter (AT) reading of states is untested, no stand binary building an adapter — the gap later owned by residual `res-llkgqygv`, which this chunk keeps out of scope.

@history_item n=14 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (harness and session contracts gain the shot) — `screenshot` paints at the harness's own time, `VelloCpuImageRenderer`, white composite, `FontsUnavailable` refused with nothing painted; the source of decision/headless-screenshot and contract/harness-screenshot/*, which `stand_contrast`'s painted-background check reads.

@history_item n=15 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — the contrast reader: `text_contrast` in the harness's `contrast` module, read-only, computed styles only, no span or log; WCAG 2.x ratio table and threshold (3.0 large text: 24 px or 14 pt bold, else 4.5); `stand_contrast` 2 over 18 `controls(task)` rows with exempt-inactive rows; source of decision/contrast-reader-reads-only, wcag-2-contrast-ratio, contrast-required-ratio and contract/harness-contrast/* — the SC 1.4.3 instrument this chunk re-reads.

@history_item n=16 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `stand_keyboard` 11: Tab walk with wrap and three focus readings agreeing, Enter and Space as click on 20 rows, `disabled` of any value skipped, the `<div disabled tabindex=0>` limit pinned, the Apple delete binding on every OS, no `inert`; `tab_order` and `focused` moved into `common`; a click focusing a button and `disabled` by presence ratified — the SC 2.1.1/2.4.3 baseline this chunk re-asserts.

@history_item n=17 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — the last assertion of the three SCs: `stand_contrast` 5 (SC 1.4.3 over `TEXT_ROWS` 14 and `STATE_ROWS` 19), `stand_keyboard` 14 (enabled-state walks, every control reached by Tab), `stand_accessibility_ids` 8; `ENABLED_TAB_ORDER` and `REGION_NAMES` in the shared module; the `a11y` leg widened to six files on the founder's F4; the adapter gap re-homed to a residual — the direct predecessor this chunk re-reads at HEAD.

@history_item n=18 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `stand_requirement_sweep` with `WITNESSES`, the requirement-to-check record; a renamed or new a11y witness must be reflected there.

@history_item n=19 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the `stand-repeat` leg and its zero-failure flakiness budget (three rounds, no retry; decision/stand-checks-zero-failure-budget, zero-failure-stand-repeat); no `document.rs` font test skips — the skip was the `qual_name!` fixture, not missing fonts (decision/font-metric-tests-never-skip). A stand-facing re-assertion here must hold three rounds.

@history_item n=20 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the merged ARIA checks) — upstream's `accessibility_label` and `accessibility_description` asserted through test-that's `verify_that!` (the exception in decision/rust-tests-on-built-in-harness), run by the workspace leg and not then by the `a11y` leg.

@history_item n=21 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the harness re-exports `NamedKey`) — keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey` re-exported (decision/harness-reexports-key-types), the types a keyboard re-assertion names.

@history_item n=22 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§1 coverage) — `inert_subtree` 4 added, `stand_keyboard`'s `disabled` and `inert` readings restated, a truthy `inert` honoured; `accessibility_merged_states` and `accessibility_active_descendant_unread` added — the `inert` and `disabled` surfaces this chunk's scope lists as changed since the last assertion.

@history_item n=23 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (`advance` on every session; the harness's clock) — `advance` moves the clock by exactly `ms`, `timer_step` carries its remainder; the harness's time is a `VirtualClock`, never moved by `screenshot`, `screenshot_rect`, `text_contrast` or `settle`; `tick` with a negative or non-finite dt moves nothing — the source of decision/one-virtual-clock, harness-virtual-clock, advance-moves-exactly-the-ms-asked, timer-step-carries-remainder and contract/harness-core/tick.

@history_item n=24 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (the clock checks counted) — `virtual_clock_engine` and `stand_virtual_clock` added; `into_config` puts the harness clock in `DocumentConfig.clock`, a wrapped document keeping its own (decision/wrapped-document-keeps-own-clock); two clicks with no `advance` a double click.

@history_item n=25 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — the founder ratified the one-virtual-clock marks (exactly `ms`, harness time untouched by `text_contrast` and `settle`, the shot after a scroll repeating until `advance`); those rows read standing.

@history_item n=26 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — upstream's `focus_within` and `focus_redraw` taken as their own targets, run by the workspace leg and not by the `a11y` leg — focus checks deliberately left out of the leg, relevant to any membership change here.

@history_item n=27 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — `tests/blitz-tests/tests/fixtures/mod.rs`: a page more than one check reads is stated once as an `rsx!` component booted with the stand's options, a page with one reader stays in its file (decision/shared-fixture-pages); `HeldNet` replaces file-private network stand-ins.

@history_item n=28 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the driver's acting verbs wait on a load in flight up to `LOAD_WAIT`, `snapshot` and `screenshot` waiting on nothing; `LateNet`; the origin of the split behind decision/harness-click-waits-on-nothing, which a load-page re-read must respect.

@history_item n=29 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (`stand-repeat` runs ten named targets) — non-`stand_` witnesses (`inert_subtree`, `accessibility_states`, `select_interaction` …) join `stand-repeat` as literal `--test` words in `ci-leg.sh`, pinned by `test_stand_repeat_leg_runs_three_rounds`; `run stand` keeps its prefix rule. The route a new non-`stand_` a11y witness must take into `stand-repeat`, as this chunk's scope names.

@history_item n=30 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg named fourteen `--test` words on one line (tier-0 witnesses plus the five tree-reading checks, the founder's «Плюс проверки дерева»): each file an SC 2.1.1 or 2.4.3 witness, a tree reading or a recorded gap; `focus_redraw`, `focus_within`, `accessibility_names` not added; a joining file keeps its legs (`stand_act_select` in both `a11y` and `stand-repeat`); the a11y pin asserts each word by one `assertIn`, in no order; per-job tallies strip ANSI written as literal `^[`; the adapter probe reads 0. Origin of decision/a11y-leg-membership and joining-file-keeps-its-legs — the precedent this chunk most directly extends.

@history_item n=31 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `CapabilityStatementTest` pins `docs/capabilities.md`'s `## Where each claim is proven`, each claim naming its check, so a renamed check changes it by hand (decision/capability-claims-name-their-checks); the adapter-crossing gap owned by the residual read `re-carried:0.2.0`.

@history_item n=32 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages' boots, `LoadNet`) — Cells booted by `stand::boot_cells`, the load page by `stand::boot_load` with `LoadNet` answering after `LOAD_ANSWER_DELAY`; every other stand boot offline; `stand_cells_served` and `stand_load_page` added — the served pages this chunk's SC re-assertion must cover, and the source of contract/stand-boot/boot-cells, boot-load and decision/stand-load-page-fixed-answer.

@history_item n=33 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the stand's tables re-pinned to the flight booker's select) — `controls(task)` 17 rows, `flight-type` a combo box; `ENABLED_TAB_ORDER` five stops through `flight-type`; `stand_contrast` 17 rows, `STATE_ROWS` 18; `stand_keyboard` 15 moving the select's choice by arrows — the current shape of the tables the re-read walks.

@history_item n=34 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the counts) — the a11y leg 14 result lines, 85 passed, `stand_keyboard` 15; `stand-repeat` over the `stand_*` files — a dated point in the leg's tally series.

@history_item n=35 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the 0.2.0 sweep test's reach) — the sweep test reads `ci-leg.sh` as one text by `contains`, so a target dropped from `stand-repeat` alone reads green while the `a11y` leg still names it; membership held jointly with `test_stand_repeat_leg_runs_three_rounds` — a trap for a witness named in both legs.

@history_item n=36 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the a11y leg reads 14 · 112) — `stand_contrast` 5 → 7, `stand_keyboard` 15 → 21, `accessibility_states` 5 → 6, `inert_subtree` 4 → 6, `select_interaction` 7 → 18, `stand_act_select` 5 → 10 — the engine debts this chunk's scope names as changed since the last assertion.

@history_item n=37 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' grid as one Tab stop) — `cells-grid` with `tabindex` 0, `aria_label` `Cells`, no role; `stand_keyboard` 21 with Cells' Tab sequence, arrows, Enter/Tab committing, the load page's sequence; `stand_contrast` 7 with `the_load_page_meets_its_ratios`, `cells_meets_its_ratios` and the `.cell.current` outline `#3a5ce5`; `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` in `common` — the served pages' existing SC coverage this chunk must audit.

@history_item n=38 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the focus-fixup limit ends in `inert_subtree`) — a node coming under `inert` gives focus up with no `blur`/`focusout`, over five ways of being made inert — the `is_inert()`-only fixup that leaves `hidden`/`display:none` focus kept, the CARRY `cry-c2zq6evs` this chunk must pin as an SC 2.4.3 gap.

@history_item n=39 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`change` from the engine; the `select` model; the chosen-row pair; `toggled`; the load page's rows; `:open`) — engine `change` only (decision/change-event-from-engine-only); `multiple` and `optgroup` modelled; the chosen select row `#c8c8c8` on `#000000` reads 12.56:1 (decision/chosen-select-row-colours); a native checkbox/radio writes `toggled`; the load page's rows asserted by `stand_keyboard` and `stand_contrast`; `open_state` pins `:open` (decision/open-pseudo-class-follows-attribute).

@history_item n=40 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`stand-repeat` names fifteen targets; fixture readers 28) — `open_state`, `checkable_change_event`, `text_range_no_change_event` and two more join `stand-repeat`'s words; `mod fixtures;` read by 28 — the counts a new witness moves.

@history_item n=41 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`type` focuses any select with no click) — `type` on any select focuses it with no click and fires no DOM focus event — a focus-order fact for SC 2.4.3 on selects.

@history_item n=42 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the a11y leg 114) — the a11y leg 14 files and 114 tests, `stand_keyboard` 23; the tier-0 witnesses' 88 is arithmetic (114 less the tree-reading checks' 26), not a reading, and 59 was already stale at the base — a trap for this chunk's re-count.

@history_item n=43 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`select_popup`; `select_popup_painted`; `stand_act_popup`) — the list opened by click, Enter or Space, Tab choosing and closing before focus moves, focus staying on the select, the highlight and disabled-row colours the engine's constants; `select_popup` and `select_popup_painted` named `stand-repeat` words, `stand_act_popup` by prefix, none of them in the `a11y` leg's fourteen — the 0.3.0 select surface this chunk must re-assert for SC 2.1.1/2.4.3/1.4.3 and decide leg membership for.

@history_item n=44 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the list leaves unmeasured) — the list paints through `paint_scene` alone, driven headlessly (decision/select-list-painted-by-paint-scene); stated limits: a highlighted row's contrast unread by the lint, a larger authored font clips, the highlight in no snapshot or tree, AT reading `unrunnable-here` — limits an SC 1.4.3 re-read of the list meets.

@history_item n=45 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the standing select checks; `STATE_ROWS` 20; the label click; `scroll_into_view`) — `stand_keyboard` 23 with the flight-type walk holding with its list open; `STATE_ROWS` 20 reached states, 19 pass, 1 exempt, the two new rows' class unnamed; a label's click focuses the select and opens nothing; `scroll_into_view` on an open-list row moves its `scroll_y` alone (contract/harness-input/scroll-into-view).

@history_item n=46 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the macOS-only red; `font-embolden`) — a paint change under `blitz-paint/font-embolden` read green locally and red on macOS only; `select_popup_painted` under the feature is the local witness (decision/embolden-local-witness); cargo stops at the first failing target, so a red job's later targets go unread.

@history_item n=47 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired) — a key press on a focused control whose anonymous block was dropped (`hidden`, `display:none`, CSS) no longer panics, focus stays and the key still acts, by `key_press_after_container_lost_its_box` in both modes (decision/key-after-dropped-anonymous-block) — the exact founder's F1 A whose left-over limit (focus kept on a no-longer-shown control) is this chunk's CARRY; the target is in neither `stand-repeat` nor the `a11y` leg.

@history_item n=48 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load ends the wait; `text_contrast` past a blend) — the load counter forwards `NetHandler::error`, so a failed stylesheet ends the settle wait (decision/failed-load-ends-settle-wait, contract/harness-settle/failed-load); `text_contrast` reads `None` past a non-normal `mix-blend-mode` (contract/harness-contrast/blend-mode), `Session::lint`'s contrast reading the same with no second formula (decision/one-contrast-formula). Trap: a take that extends a trait sweeps its implementers.

@history_item n=49 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `fast` ends on the `embolden` leg, pinned by `test_fast_leg_ends_on_the_font_embolden_witness` — the local gate a paint-affecting a11y fix must pass.

@history_item n=50 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — owner sentences re-pointed from `.andromeda/residuals.md` to residual rows, among them `res-llkgqygv`, the windowed AT witness this chunk's scope keeps out.

@history_item n=51 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (residual owners re-said) — the adapter-crossing gap's owner reads residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), at both sites — the out-of-scope boundary this chunk must not touch.

@history_item n=52 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (one rule for two readers) — both actionable readers read one rule from current listeners, never the `data-dioxus-id` mark (decision/one-actionable-rule); `clickable` true where listeners hold `click` and no disabled form control (contract/snapshot-node/clickable).

@history_item n=53 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (`stand_actions`) — a fetch put in flight with `harness.click("#load-fetch")`, which waits on nothing, where a driver `click` waits out `LOAD_WAIT` — the precedent behind decision/harness-click-waits-on-nothing for any load-page contrast or keyboard reading.

@history_item n=54 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (`stand_handler_coverage`) — a refused step exercises no handler and a keyboard activation exercises the element's click handler (contract/handler-coverage/refused-step), a keyboard-equivalence fact SC 2.1.1 rests on.

@history_item n=55 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the test-plan master became rows (decision · contract · absence · overview), each verified against the code; from here an amendment names a row by handle, the form this chunk's own amendments take.

@history_item n=56 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (5. Integration Test Strategy) — `~ decision/layout-tests-in-both-layout-modes` gained its except: upstream targets taken in a sync stay as written, one-mode ones named exceptions; every new stand check still runs both modes (decision/stand-checks-in-both-layout-modes).

@history_item n=57 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** (3. Test Harness Contract) — `+ contract/harness-inspect/focused`: with no node focused the harness's `focused` answers the root html element, never none, so a check that focus is cleared asserts the root element — directly binding SC 2.4.3 readings after a control turns disabled, hidden or inert.

@history_item n=58 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** (3. Test Harness Contract) — `~ contract/harness-construction/from-vdom`: `from_vdom` provides `SeededRandom::from_seed(DEFAULT_SEED)` where the root holds none, a root context the builder gave standing — the construction the stand boots ride.

@history_item n=59 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (2. Test Strategy) — `~ decision/stand-checks-bundled-font` guard `stand_bundled_font_families`: every generic family and an unavailable named family read the bundled font's extent; the bundled-fonts change whose glyph metrics, large-text threshold and sampled text this chunk re-reads for SC 1.4.3.

@history_item n=60 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (4. Unit Test Strategy; 5. Integration Test Strategy) — `+ decision/no-change-read-control-needs-pending-work`: a pump on a settled screen is invisible, so a control of a read that must change nothing needs pending work — binding any control of `text_contrast`'s read-only promise; `+ contract/session-layout/changes-nothing`.

@history_item n=61 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (5. Integration Test Strategy) — `~ contract/session-layout/changes-nothing`: a layout read, in process or through the `layout` verb, changes nothing, an open list staying open, in both layout modes.

@history_item n=62 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (4. Unit Test Strategy) — `~ decision/mutation-controlled-tests`: a new check is controlled red-before-green, a bound-removing control loosening the bound to twice the budget — the control every re-assertion this chunk adds must carry.

@history_item n=63 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (4. Unit Test Strategy) — `+ decision/expected-value-pinned-as-literal`: a check pins its measured value as a literal, never deriving it from the constant its control mutates — binding contrast ratios and Tab-order tables re-pinned here.

@history_item n=64 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (7. Test Data & Fixtures) — `~ decision/stand-boots-offline` and `~ decision/stand-load-page-fixed-answer`: every stand boot offline but the load page's `LoadNet` and the error-text check's `FailingNet` page — any new served-page or fixture boot here must keep that.

@history_item n=65 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (3. Test Harness Contract) — `+ contract/harness-construction/from-component-alone`, `+ contract/stand-boot/boot-alone`, `+ decision/component-alone-props-as-maker`, `+ decision/component-alone-root-context-before-build` (TimerTicks provided before the build) — the boots available if a component's SC reading is taken alone.

@history_item n=66 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (2. Test Strategy) — `+ decision/requirement-witness-inside-stand-repeat`: every release requirement names its witness inside `stand-repeat` in a table pinned by function name, guard `stand_requirement_sweep`, so a renamed or moved a11y witness reads red there and must be updated with it.
