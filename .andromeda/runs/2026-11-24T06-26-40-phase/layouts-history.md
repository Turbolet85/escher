@history_item n=1 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — 4 `file:line` citations into merged upstream files re-pointed by the merge's measured line map, no claim text changed. The first sync's precedent: this merge also moves lines in upstream files the plan cites (`blitz-dom` layout, `blitz-paint/src/render.rs`, dioxus-native-dom), so citations are re-pointed by measured line map, never by a mechanical shift.

@history_item n=2 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the `bump` line left §Surface: cli because upstream's `23b9b26a9` deleted the crate in the take. Precedent: an upstream commit the sync takes can change this plan's CLI surface, so the six commits are read for CLI-surface effects.

@history_item n=3 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — upstream's root example `multi_doc` was added to §Surface: cli beside `paint_bench` once the sync took it. Direct precedent for this take's new root examples `examples/dnd.rs` (`82a6573ee`) and `examples/file_drop.rs` (`29a439e05`): whether each is a command-line surface the doc must describe is read at this chunk.

@history_item n=4 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations into the files the take moved) — 4 citations into `apps/browser/src/{main,tab}.rs` and blitz-shell's `window.rs` re-pointed, and 2 blitz-vibey-script citations given new paths after upstream split the files. Precedent for this take: blitz-shell gains `dnd.rs` and its manifest moves, so a cited path that moved is rewritten, not only its digits.

@history_item n=5 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — `~ contract/paint-bench-cli/backend`: the backend list renamed with upstream (`hybrid` → `gpu`). The previous sync's row-form precedent: an upstream rename lands as a `~` row edit on the cli surface; this take's CLI-facing moves are recorded the same way.

@history_item n=6 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/escher-session-layout/answer` (bounds, laid_out, placed, shifts, width/height with min/max/flex basis, grow, shrink) and `~ decision/no-caller-input-echoed` (except: the layout answer's id). This take moves Taffy 0.15 git → 0.16.0 and adapts `layout_reason.rs` to `taffy::CalcHandle`, so the layout answer's readings (`Session::layout`, `stand_layout`) are a surface of the merge and must read unchanged.

@history_item n=7 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — the founder ratified the delegate confirm that landed `contract/escher-session-layout/answer` and the `decision/no-caller-input-echoed` exception. Both rows are standing, on his own word; the Taffy move must keep them as written.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — added the IA note that an into-view scroll moves every scrolling box holding its target, then the viewport, and that `visible_region` reads what an element is seen through (the CRUD list as the nested scroller); the engine change was ratified by the founder for every document. This is the origin of `decision/visible-region-through-clipping-boxes` and `decision/scroll-into-view-moves-every-scroller`; `f65baeca5` (one-axis `overflow: clip` in paint) touches that clip model.

@history_item n=9 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the scrolled `crud-list`'s bounds and the hit fixed in the engine, plus Home's scrolling box (Cells hidden until scrolled). A trap: a standing check went red when the hit changed. `f65baeca5` moves paint for a one-axis clip while the hit still stops at the whole padding box, so hit and paint readings are re-measured, not assumed.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — four sub-entries: the flight booker's mode list painted by `paint_scene` after the root render (open list rows read their own bounds, rows back at the select's rect on close); citations into `render.rs` and `scrolling.rs` re-pointed; the open-list exception to the scrolling-boxes rule (`visible_region` answers the list's scrollport, `scroll_into_view` writes `scroll_y`); the lint's `Entry.in_select_box`. These are the origins of `decision/open-list-rows-own-bounds` and of the `except:` clauses on `decision/visible-region-through-clipping-boxes` and `decision/scroll-into-view-moves-every-scroller`. `f65baeca5` changes `blitz-paint/src/render.rs`, where the list is painted and which the plan cites.

@history_item n=11 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — six served screens: the load page in its own TaskShell with the in-process `LoadNet` answering after 200 ms; the flight type select (`flight-type` with two options at the select's client rect); the lint at boot. These are the sources of `decision/load-page-only-network`, `decision/flight-type-select-boot-reading` and `decision/headless-stand-pinned-viewport`. The merge's new drop input and the dioxus-native-dom `std::fs` reader must add no network or input provider to any other boot.

@history_item n=12 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (PROVISIONAL) — moved the stand colour rows here: accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`), white text reading 5.47 against 4.5, `#4a6cf7` at 4.39 a fail, all read by `stand_contrast`. This is the source of `decision/stand-accent-colour`. The cross-OS readings and contrast checks must pass unchanged after the merge's paint change.

@history_item n=13 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines; F2 ratified) — landed the served-screens bullet (LoadNet answering from its own thread, so a driver click on `load-fetch` waits it out), and dropped the stand rows' PROVISIONAL gap marker on the founder's word. These are the ratified sources of `decision/load-page-only-network` and `decision/stand-accent-colour`, taken in the earlier upstream-sync chunk.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — thirteen citations re-pointed (`dioxus_document.rs`, `scrolling.rs`, `stand_lint.rs`, `stand_load_page.rs` and others); `stand_contrast` extended to the load page and Cells; the flight booker's boot lint moved to `[0, 0, 0, 0]` with closed-select options skipped. Precedent for re-pointing citations into engine files a change moves. It is also the source of the `decision/stand-lints-clean-at-boot` readings the merge must keep.

@history_item n=15 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the visual lint: 0 findings at boot in both layout modes, in process and then as the `lint` verb (the gated part). This is the origin of `decision/stand-lints-clean-at-boot`, which the merged tree must keep in both layout modes.

@history_item n=16 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `+ decision/order-page-built-to-cause` (Submit error, the `order placed` log, the Export panic) and `~ decision/stand-lints-clean-at-boot` (gains the order page). These rows are picked here; the merge's event-driver change (`22c18e38f` preventDefault on non-cancelable, drag events) must leave the order page's caused panic, error and log as they are.

@history_item n=17 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — `~ decision/stand-lints-clean-at-boot` (gains Circle Drawer) and `+ decision/circle-drawer-circles-painted-on-canvas` (PROVISIONAL at the time). This is the chunk just before this one. Its canvas shot crosses as pixels alone, and the WATCH `cry-womrv2l4` (cross-OS `SAMPLE` digest) reads the merged paint.

@history_item n=18 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** — `~ decision/circle-drawer-circles-painted-on-canvas` state provisional → standing, and its ratification entry, on the founder's «Подтвердить (Recommended)». The row this chunk rests on is standing, not provisional, so the merge must keep it as written.

@history_item n=19 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host named as a third consumer of the `task_in_shell` mount: a held instance reads as a fresh boot, and the mount and pinned viewport hold after commands. This is the origin of `decision/held-instance-reads-as-fresh-boot` (guard `stand_session_fresh`), which must hold on the merged tree in both layout modes.

@history_item n=20 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand skips Home and mounts one task in TaskShell at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. This is the origin of `decision/headless-stand-pinned-viewport`, the base every stand reading after the merge is taken on.

@history_item n=21 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the coverage role became a further consumer of the TaskShell mount (a held instance still reads fresh), and the skeleton and head notes' citations into `packages/dioxus-native-dom/src/dioxus_document.rs` were re-pointed. This bears on `decision/fixed-document-skeleton-main-mount` and `decision/held-instance-reads-as-fresh-boot`. The take conflicts in dioxus-native-dom (`events.rs`) and adds its file reader, so citations into that crate are re-measured.

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the screens' source citations) — re-pointed citations into the task files, `dioxus_document.rs` and `schema.rs`, including the document skeleton notes. Precedent: when a chunk shifts lines in `dioxus_document.rs`, the `decision/fixed-document-skeleton-main-mount` grounds are re-pointed and the claim stays.

@history_item n=23 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — stand inputs named by attributes alone (`for`, `aria-label`), with citations into dioxus-native-dom `dioxus_document.rs` re-pointed. This is the origin of `decision/stand-a11y-by-attributes-alone`. The a11y leg must read green after the merge.

@history_item n=24 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — states set by attributes alone: `aria-invalid` on the flight dates, `role="listbox"`/`option` and `aria-selected` on CRUD, with no element added. This extends `decision/stand-a11y-by-attributes-alone`, which the a11y leg checks on the merged tree.

@history_item n=25 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` named `People` by `aria-label` alone. This extends `decision/stand-a11y-by-attributes-alone`, which the a11y leg checks after the merge.

@history_item n=26 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` with an optional `--id` answers `{"width","height","png"}` (an element cropped to its visible part, an element with no pixel refused `off-screen`). This is the origin of `contract/escher-session-screenshot/answer`. `f65baeca5` changes one-axis clip paint, so shot readings that move are re-measured and recorded, never loosened.

@history_item n=27 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` answers `{"text"}` or `{"text","part","parts"}`, and a part past the last gives empty text. This is the origin of `contract/escher-session-snapshot/answer`, which the merged tree's stand checks must keep.

@history_item n=28 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role. It scoped "no ending writes back anything the caller typed" to the commands, which is the history of `decision/no-caller-input-echoed`, a row picked here that the merge must keep.

@history_item n=29 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — added `scripts/agent-run.sh`, with raw cargo output kept in `target/agent-run/run.log` and never on stdout. This is the origin of `contract/agent-run-cli/run-log`. It meets this chunk's evidence-hygiene directive: no raw log with a temp or home path enters evidence.

@history_item n=30 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `~ contract/agent-run-cli/usage` gains the `session` selection. A neighbour of the picked `contract/agent-run-cli/run-log`: the agent-run selections this chunk's stand runs use.

@history_item n=31 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (decision · contract · absence), each verified against the code. Every picked row's handle dates from here. Edits this chunk makes land as `+`/`~`/`−` row entries, and new upstream surfaces (the drop input, the root examples) are named against these rows and the `absence/arbitrary-index-html-templates` absence.
