@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand: skip Home, mount one task in TaskShell through `task_in_shell` at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans (`decision/headless-stand-pinned-viewport`). Circle Drawer's new in-process stand boot (F1 A) is a further mount on this shape and must hold the same viewport and shell.

@history_item n=2 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — lean tasks' controls got author ids as attributes alone, no class, style, wrapper or order, no CSS selecting by them: the origin of `decision/stand-ids-as-attributes-alone`, which the canvas `<object>` keeping `circle-canvas` must respect.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — stand inputs named by attributes alone (`for`, `aria-label`) with no element, class, style or order changed: the precedent of `decision/stand-a11y-by-attributes-alone`, which binds the canvas's `role="group"` and «Drawing area» name (F5 A).

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — Home's seven task-card ids `task-card-{slug}` added, `circle-drawer` among them, under the founder's rule that every element an agent can act on reads an author key: the source of `contract/stand-element-ids/task-card-circle-drawer` and `decision/every-stand-control-keyed`. The stand checks open Circle Drawer by that card.

@history_item n=5 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host was named a third consumer of the `task_in_shell` mount, adding no markup, and a held instance reads as a fresh boot with the mount and pinned viewport holding after commands. This is the origin of `decision/held-instance-reads-as-fresh-boot`, which Circle Drawer's new stand boot extends.

@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — restated the host consumer of the mount as the binary's host role, with the layout statement unchanged (no markup, a held instance reads as a fresh boot). It constrains the stand boot, which F1 A keeps out of `escher-session`'s served tasks.

@history_item n=7 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — Home's column sits in a scrolling box: at 800 × 600 unscrolled `#home` shows six of the seven cards, Cells below the fold. This matters because checks reach Circle Drawer from Home by `task-card-circle-drawer` (the sixth card, shown without a scroll).

@history_item n=8 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — the stand states roles and states (`role="listbox"`, `role="option"`, `aria-selected`, `aria-invalid`) by attributes alone, and the classes that style those states stay. It is the wording `decision/stand-a11y-by-attributes-alone` now holds, and the canvas's role and name follow it.

@history_item n=9 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `screenshot` with an optional `--id` (the element's visible part) answering `{"width","height","png"}`, refused `fonts-unavailable` with no paintable font and `off-screen` for an element crop of no pixel. These are the contracts behind `contract/escher-session-screenshot/answer` and the two refusals. The Circle Drawer canvas element shot (cross-OS digest) and canvas text from bundled fonts rest on them.

@history_item n=10 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` was named by `aria-label="People"` alone, adding no element, class, style, wrapper, id or order. It is a direct precedent for naming the canvas «Drawing area» by attribute (F5 A).

@history_item n=11 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the non-lean tasks got author ids as `id:` lines alone. For Circle Drawer these are `circle-undo` · `circle-redo` · `circle-canvas`, plus `circle-diameter` · `circle-close` on the open dialog, with `unkeyed_actionable()` 0 on all seven tasks. This is the origin of the five picked `contract/stand-element-ids/circle-*` rows. `circle-canvas` must move to the `<object>` without changing its id, and no CSS may select by it.

@history_item n=12 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the in-process visual lint (`Session::lint`) at boot in both layout modes, with no stand markup moved. This is the root of `decision/stand-lints-clean-at-boot`, which this chunk's «boot lint clean: 0 findings, both modes» extends to Circle Drawer.

@history_item n=13 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — added the `lint` verb: no argument, one uncoloured line `{"findings":[…]}`, `fonts-unavailable` refusal. Each task's boot reading through the verb matches the in-process reader. This is the source of `contract/escher-session-lint/answer`. Note that a canvas `<object>` reads role `Unknown` in the lint and the tree.

@history_item n=14 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — dropped the PROVISIONAL «in process only» mark from the stand-layout lint note, so the lint stands as built. It is background for the boot-lint row this chunk extends.

@history_item n=15 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot` got `--part` and answers `{"text"}` or `{"text","part","parts"}`, the source of `contract/escher-session-snapshot/answer`. Circles moved off the DOM leave the snapshot (they cross as pixels only), so the snapshot of Circle Drawer reads no circle.

@history_item n=16 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — non-lean screens got stand boots of their own (`boot_cells`, `boot_load`) and became served tasks. The boot-lint reading was re-pinned as measured, and the load page's `LoadNet` answers after 200 ms (`decision/load-page-only-network`). This is the precedent for Circle Drawer's in-process stand boot. Under F1 A this chunk adds the boot but not the served step, and gives Circle Drawer's boot no network provider.

@history_item n=17 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — stand value rows landed on desktop-native: accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`), white on accent 5.47 against 4.5, and the circle dialog's shadow at 0.10 among the depth rows. This is the origin of `decision/stand-accent-colour`. The canvas's painted colours and the dialog must keep these values, and `stand_contrast` guards them.

@history_item n=18 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines; F2 ratified) — the load page's `LoadNet` answers from a thread of its own, so a driver click on `load-fetch` waits it out: the root of `decision/load-page-only-network`. The founder ratified the stand rows on this surface and the PROVISIONAL marker dropped. A Circle Drawer stand boot must keep no network provider.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid, a keyboard region and one Tab stop; `stand_contrast` over the load page and Cells; the flight booker's boot lint) — a non-lean task got stand-only markup (`tabindex`, `aria-label`, handler) under the attributes-alone rule. The reacting-element count `stand_non_lean_tasks` reads was measured, not forecast (677 against a planned 676). Contrast and boot-lint checks were extended to non-lean screens in both modes. This chunk edits `stand_non_lean_tasks.rs` and changes Circle Drawer's reacting elements (one canvas in place of per-circle divs), so measure its counts rather than forecast them.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the engine's select list; the lint's `overlap` exclusion) — an engine change for every document paints content that has no layout box, with the windowed path read and not run. The lint was taught to skip those rows (`Entry.in_select_box`), with `at_boot` `[0, 0, 0, 0]` and no stand markup moved. This is the closest precedent for this chunk's engine change in blitz-dom events across every document, and for keeping boot lint at 0 when content is painted rather than laid out.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb; the screens' citations re-pointed) — added `graph`, answering `{"nodes","states"}` with ids, component names, handler event names and state names, never a value. Task-file and `dioxus_document.rs` citations for the skeleton and the Home/TaskShell rows were re-pointed. Circle Drawer's circle state and its canvas handlers (`onclick`, `ondoubleclick` on the `<object>`) will read through this graph.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — added the `actions` verb: `{"nodes"}` of id, owner, keyed and verbs with cause, never a value. This is the source of `contract/escher-session-actions/answer`. The canvas `<object>` reads `clickable`, so its keyed actionability is read through this answer.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a further consumer of the mount; the bridge's skeleton notes re-cited) — the `coverage` role boots through the same boot, adding no markup, and a held instance still reads as a fresh boot in both modes. The fixed-skeleton notes into `dioxus_document.rs` were re-cited. Both bear on `decision/held-instance-reads-as-fresh-boot` and `decision/fixed-document-skeleton-main-mount`.

@history_item n=24 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (121), each verified against the code. The picked decision and contract rows, the circle ids and the stand decisions, date from this conversion. Later changes name them by handle.

@history_item n=25 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph verb; the coverage role as a mount consumer) — the founder's own word ratified the graph verb entry and the coverage-role consumer, with text standing as written. This settles `contract/escher-session-graph/answer`'s base and the held-instance row's consumers.

@history_item n=26 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (desktop-native: 8 rows; cli: 2 rows) — `~ decision/stand-lints-clean-at-boot` added the order page to the pages clean at boot. `+ contract/stand-element-ids/order-*` added the page's new ids. Served tasks grew to seven (`contract/escher-session-served-tasks/order`) and the cold-agent pipe to seven apps. This is the exact pattern this chunk follows to name Circle Drawer in `decision/stand-lints-clean-at-boot`. Under F1 A, no served-task row or count moves.

@history_item n=27 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ contract/escher-session-graph/answer` now answers whole or by part (`part`, `parts`, states on part 1 alone). This is the current form of the picked graph row that Circle Drawer's graph reading meets.
