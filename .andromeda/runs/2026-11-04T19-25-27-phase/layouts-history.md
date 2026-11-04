@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand's mount: Home skipped, one lean task in TaskShell through `task_in_shell` (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`) at the pinned 800 × 600, scale 1.0, Light viewport; this is the exact mount the probe's runs must read through the `seven_guis::stand` boots, not a probe-only screen.

@history_item n=2 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — gave the lean tasks' controls author `id`s (`counter-value` · `counter-increment`, the `crud-*` ids, …) as stable element ids, attributes only, no class/style/wrapper/order and no CSS selecting by them; these ids are what the probe's node → component and node → handler links key on, and the markup-only precedent the probe must not break.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — CRUD rows' Dioxus key moved from index `{i}` to `{person.id}` (fixture 0–2, Create from 3) so a row's identity follows its person under filter, Create and Delete; the probe's CRUD runs (a handler writing state another node reads) rest on these keys.

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — each CRUD row carries the author id `crud-person-{person.id}` beside its key, Home's cards `task-card-{slug}`; the founder's rule that every actionable element reads an author key, attributes only — the row ids the probe's node links read.

@history_item n=5 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — stand inputs named by attributes alone (`for` on `crud-filter`, `crud-name`, `crud-surname`, …) and 3 citations into `packages/dioxus-native-dom/src/dioxus_document.rs` re-pointed; precedent that the bridge file the probe reads is cited by line in this plan, so any probe edit there shifts citations.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — named the session host as a further consumer of the `task_in_shell` mount, booting through `seven_guis::stand` only and adding no markup, id, class, style, wrapper or order; the precedent for a new reader of the stand (the probe) that leaves the layout statement unchanged.

@history_item n=7 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — re-pointed the harness-defaults bullet's citations to the `Default` impl holding 800x600, scale 1, light; the baseline figures the extract's harness binding and any probe check in `tests/blitz-tests/` run against.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — added the scrolling-boxes note: an into-view scroll moves every holding scrolling box then the viewport; on the stand `crud-list` is the nested scroller clipping its rows (`task-body` does not); an engine change ratified by the founder (2026-10-07) — the into-view family the Cells grid's `scroll_to` belongs to and the CRUD figures a layout-touching probe reading meets.

@history_item n=9 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — `crud-list`'s readings once scrolled, and Home's box at 800 × 600 holding six of seven cards (Cells below, hit only once scrolled); a trap for any probe run reaching Cells via Home rather than `boot_cells`.

@history_item n=10 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — `crud-list` takes `role="listbox"`, each CRUD row `role="option"` with `aria-selected`, attributes only; part of the CRUD list's current markup the probe reads and must leave as is.

@history_item n=11 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` took `tabindex="0"` and an `onkeydown` (arrows / Home / End select rows, filling `crud-name` / `crud-surname`); CRUD's Tab sequence became six stops `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`; the listener the probe's link (2) reads and the six-stop sequence its acceptance re-checks, with no focus stop or listener added.

@history_item n=12 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` named `People` by `aria-label` alone, the six-stop Tab sequence unchanged; one more attribute on the keyboard region bound to a11y focus order that the probe must not move.

@history_item n=13 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — non-lean tasks keyed: Cells `cell-{A..Z}{0..25}` on 676 data cells and `cell-input`, attributes only, `unkeyed_actionable()` 0 on all seven tasks; the ids a probe run on Cells as the served task would key node links on.

@history_item n=14 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the in-process visual lint (0 findings at boot in both layout modes; `crud-person-14` `clipped` after twelve Creates), and its gated part made `lint` a verb on every carrier; origin of the `stand_lint` baseline the probe's acceptance re-reads.

@history_item n=15 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — dropped the visual lint note's PROVISIONAL mark on the founder's word, the lint as built «in process only»; the standing status of the lint baseline the probe's acceptance cites.

@history_item n=16 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — Cells booted by `boot_cells` and the load page in a TaskShell of its own by `boot_load`, six served tasks; the load page's `load-fetch` answered by in-process `LoadNet` after `LOAD_ANSWER_DELAY` (200 ms); flight ids became `flight-type*`; the lint at boot re-pinned as measured — the source of "one served task beside it" and of the delayed-effect handler case.

@history_item n=17 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines) — the served-screens bullet: `LoadNet` answers from a thread of its own, so a driver `click` on `load-fetch` waits it out; matters if the probe reads handler → state on the load page, where the effect lands after the click.

@history_item n=18 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid, a keyboard region and one Tab stop) — `cells-grid` got `tabindex="0"`, `aria-label` "Cells", an `onkeydown` (677th reacting element), arrows bringing the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)` because the planned async form panicked `RefCell already borrowed` and did not ship; `CELLS_TAB_ORDER`; windowed path read, not run — the direct precedent of the MountedData hazard this chunk names, and the Tab stop and constant it must hold.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's Tab sequence) — the load page's Tab sequence named by `LOAD_TAB_ORDER`, `load-fetch` taking Enter and Space, walked in both layout modes by `stand_keyboard`; one of the orders the probe's acceptance re-checks.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed) — thirteen citations re-pointed incl. 3 into `dioxus_document.rs`, 2 into `cells.rs`, 3 into `stand_lint.rs`; the document-skeleton note's citations the probe reads, to re-verify at HEAD after the 2026-11-01 upstream sync.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading) — `stand_lint` `at_boot` moved to `[0, 0, 0, 0]` in both layout modes (founder's F9), load page 0 — the exact lint baseline the probe's acceptance requires to still hold.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint's `overlap` exclusion) — `Entry.in_closed_select` retired for `Entry.in_select_box`; `at_boot` `[0, 0, 0, 0]`, load page 0 and `crud-person-14` `clipped` kept — the latest restatement of the lint baseline the probe re-reads.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the exception to the scrolling boxes rule) — `scroll_into_view` on an open select row writes the list's `scroll_y` and returns; the rest of the scrolling-boxes rule and CRUD list readings kept — the current form of the into-view rule a layout-touching probe reading meets.
