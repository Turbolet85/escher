@history_item n=1 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid, a keyboard region and one Tab stop) — made `cells-grid` a keyboard region and one Tab stop (`CELLS_TAB_ORDER`), with arrows bringing the current cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)` and the editor's Enter returning focus to the grid. The handles are synchronous because the plan's async form panicked with `RefCell already borrowed` and did not ship. That is the hazard this chunk's buffered render removes, so this is the MountedData path the port must leave working. The windowed path was read and not run. 677 reacting elements and 676 data cells are the counts that must not move.

@history_item n=2 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the citations the chunk's line moves re-pointed) — re-pointed thirteen citations, including 3 into `dioxus_document.rs` that back the IA notes' document skeleton, `index.html` template and head-element notes. This is the precedent for re-pointing: when the port shifts `dioxus_document.rs` lines, those citations move with the measured line map and their prose stays unchanged.

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's Tab sequence and its button's keys) — the load page states `LOAD_TAB_ORDER`, walked in both layout modes, and `load-fetch` takes Enter and Space. This is one of the Tab sequences the extract requires to read unchanged after the port.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading) — moved the lint at boot to `stand_lint` `at_boot` `[0, 0, 0, 0]` in both layout modes, with the load page at 0. This is the reading the extract's acceptance pins after the buffered render.

@history_item n=5 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint's `overlap` exclusion, `Entry.in_select_box`) — kept `at_boot` `[0, 0, 0, 0]`, the load page's 0 at boot and no stand markup moved, as measured at that chunk's report. This is the latest statement of the lint baseline this chunk must reproduce.

@history_item n=6 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the exception to the scrolling boxes rule) — added an exception for open select-list rows to the scrolling boxes rule: `scroll_into_view` on a row writes the list's `scroll_y` only. The rule for every other element still stands, and so do the CRUD list's readings. These are the into-view geometry a MountedData `scroll_to`/`scroll` reached during render must give.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list) — the flight booker's Tab sequence holds with its select list open and after each close, and focus stays on the select. This is part of the flight booker's four-or-five-stop sequence that must read unchanged after the port.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — added the scrolling-boxes rule: into-view scrolls every holding box innermost first, then the viewport. It also pinned `crud-list` (24, 112.796875, 369 × 419.203125) and `task-body` (0, 46, 800 × 554) at boot. This is an engine rule for every document, ratified by the founder on 2026-10-07. These are the exact readings the extract's acceptance requires after the buffered render.

@history_item n=9 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — states that a scrolled `crud-list` reads the same four figures, and that Home's column sits in a scrolling box with Cells' card below the fold. These are scroll-geometry readings the buffered render must not change.

@history_item n=10 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — introduced the stand's `task_in_shell` mount: `main#main > #task-shell`, with `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport. This is the tree the buffered render must build unchanged.

@history_item n=11 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the stand mounts Cells through `boot_cells` and the load page through `boot_load` in a `TaskShell` of its own, and six served screens go through the host and `mcp` roles. These are the documents beyond the lean four that render through the buffer.

@history_item n=12 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — gave the lean tasks' controls author `id`s as stable element ids, adding no class, style, wrapper or order. Node identity and the snapshot rest on these ids, so the buffer must replay them unchanged.

@history_item n=13 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — keyed CRUD rows by `{person.id}` so a row's id follows its person. The keyed-list mutations that the buffer's replay must preserve rest on this.

@history_item n=14 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — gave CRUD rows the author `id` `crud-person-{person.id}` and gave Home's cards `task-card-{slug}`, both attributes only. These stable ids must replay unchanged.

@history_item n=15 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — added author ids to the non-lean tasks (`cell-{A..Z}{0..25}`, `cell-input`, the temp and circle ids), and `unkeyed_actionable()` reads 0 on all seven tasks. These stable ids must replay unchanged.

@history_item n=16 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — named the stand's inputs by attributes alone and re-pointed 3 citations into `dioxus_document.rs` by a measured line map. This is the precedent for re-pointing the `dioxus_document.rs` citations this port will shift.

@history_item n=17 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — re-pointed `file:line` citations into merged upstream files by the measured line map, with no claim text changed. This is the precedent for re-pointing after taking upstream code, as this chunk does with `286874cc1`.

@history_item n=18 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — made `crud-list` a keyboard region (`tabindex="0"`, `onkeydown`) with CRUD's six Tab stops. The tree's focus stays on the list, never the row. This is the focus target the inverted `set_focus` pins and the buffered render must leave unchanged.

@history_item n=19 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — named `crud-list` `People` by attribute alone, with the six-stop Tab sequence unchanged. This is part of the CRUD keyboard region the a11y binding reads.

@history_item n=20 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — introduced the stand-reads-clean note: 0 findings at boot on each lean task in both layout modes, and `crud-person-14` reads `clipped` after twelve Creates. This is the origin of the lint baseline the extract pins.

@history_item n=21 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host is a further consumer of the `task_in_shell` mount, and a held instance reads as a fresh boot. This is one more Dioxus document whose layout must come out the same under the buffered render.

@history_item n=22 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — re-pointed the harness-defaults citations that hold the pinned 800x600, scale 1, light viewport, the viewport the stand's mount is stated at. This is the precedent for re-pointing by measurement and not by a mechanical shift.
