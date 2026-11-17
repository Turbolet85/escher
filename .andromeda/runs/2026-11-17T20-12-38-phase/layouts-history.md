@history_item n=1 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the 0.1.0 sweep keyed the three non-lean tasks (`temp-*`, `circle-*`, `cell-{A..Z}{0..25}`, `cell-input`) by `id:` lines alone, `unkeyed_actionable()` reading 0 on all seven tasks; this was the same-named predecessor of this chunk (operator's "Key all three; C8 deferred"). It is the precedent for decision/every-stand-control-keyed and the stand-element-ids rows this sweep re-proves, and it shows that a sweep may add stand attributes only, never structure.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand's `task_in_shell` mount (`main#main > #task-shell`, header over `#task-body`) at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. This is the origin of decision/headless-stand-pinned-viewport and the "stand layout held" baseline the scope names (viewport, font, screens' structure).

@history_item n=3 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — re-pointed the harness-defaults citations (800x600, scale 1, light `Default` impl) by measurement because a mechanical shift had left one on a closing brace. The pinned-viewport row rests on these lines, and the trap is that a citation must be re-measured, not shifted, when harness files grow.

@history_item n=4 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls got author ids (counter, flight, timer, crud), no class, style, wrapper or order added and no CSS selecting them (markup-only gate). This is the origin of contract/stand-element-ids/* and decision/stand-ids-as-attributes-alone, which the sweep must read as still holding.

@history_item n=5 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — CRUD row keys moved from index `{i}` to `{person.id}` (fixture 0–2, Create from 3) so an id follows its person under filter, Create and Delete. This is the basis of contract/stand-element-ids/crud-person-id.

@history_item n=6 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — CRUD rows got the author id `crud-person-{person.id}` beside the key, and Home got the seven `task-card-{slug}` ids, attributes only (the founder's rule that every actionable element reads an author key). These are the origin of the crud-person-id and task-card-* rows and of decision/every-stand-control-keyed.

@history_item n=7 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the six task inputs were named by attributes alone (`for` on the labels of timer-duration, crud-filter, crud-name and crud-surname; `aria-label` "Departure date" / "Return date"), the markup probe confirming attribute-only diffs. This is the origin of decision/stand-a11y-by-attributes-alone and the "named by its label's for" / aria-label rows.

@history_item n=8 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — roles and states stated by attributes alone (`aria-invalid` on the date inputs, `role="listbox"` and `role="option"` with `aria-selected` on CRUD), keeping the styling classes. The flight toggle buttons' `aria-pressed` part was later retired (see 2026-10-25-cold-agent-bar), and decision/stand-a11y-by-attributes-alone carries the rest.

@history_item n=9 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` got `tabindex="0"` and an onkeydown with ArrowDown/Up/Home/End selecting rows and filling the name fields; CRUD's Tab sequence has six stops; the tree's focus stays on the list, with no `aria-activedescendant`. This is the origin of decision/crud-list-keyboard-region, and the tree-focus gap is owned elsewhere (Stand a11y / «A11y re-read», outside this chunk).

@history_item n=10 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` was named `People` by `aria-label` alone, Tab sequence unchanged. This is the basis of contract/stand-element-ids/crud-list.

@history_item n=11 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — added the scrolling-boxes note: into-view scroll moves every holding scroller then the viewport, and `visible_region`; on the stand the nested scroller is `crud-list` (24, 112.796875, 369 × 419.203125 at boot); after twelve Creates the last row is refused `off-screen` until a driver `scroll`. The engine rule was ratified by the founder (2026-10-07). These are the origin of decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes, with measured figures a stand-layout check can re-read.

@history_item n=12 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — after a scroll, `crud-list` reads the same four figures and a click lands on the centre row at 12 and 14 Creates. Home's `#home` box is 600 high and holds six of the seven cards, Cells (615–684) showing only once scrolled. A trap for any check that clicks `task-card-cells`: a standing check went red on this.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list) — the engine's open select list in every document: placement, keys, focus staying on the select, and each row reading its own `bounds` while open and back at the select's client rect after close. These are the origin of decision/open-list-rows-own-bounds and of the "select list" change whose stand layout this sweep must re-read (founder forks F1–F4, F7). It kept the boot snapshot's `ComboBox "Flight type"` with two ListBoxOptions (decision/flight-type-select-boot-reading).

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the exception to the scrolling boxes rule) — added the `except` clauses of decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes: an open list's row is seen through the list's scrollport, and `scroll_into_view` writes the list's `scroll_y` at once. No ruling names this exception, which records what the code does, so any witness the sweep names for these rows must cover both the rule and its exception.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint's `overlap` exclusion) — `Entry.in_closed_select` was retired for `Entry.in_select_box` (closed-select options and open-list rows skipped); `closed_select_of` reads the element, never the role, so CRUD's option rows stay linted. It kept `at_boot` `[0, 0, 0, 0]`, the load page 0 and `crud-person-14` `clipped` (founder F10). This bears on decision/stand-lints-clean-at-boot.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (citations re-pointed) — six citations moved (`stand_contrast.rs` 4, `render.rs`, `scrolling.rs`). Precedent: edits to `stand_*` checks shift master citations, which the wrap re-points.

@history_item n=17 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads 0 lint findings at boot in both layout modes, and after twelve Creates `crud-person-14` reads `clipped` (367 × 11.203125 inside `crud-list`), agreeing with the `off-screen` refusal. The first entry under this marker also covers the gated part: `escher-session lint` on every carrier and the `{"findings"}` answer. These are the origin of decision/stand-lints-clean-at-boot and contract/escher-session-lint/answer.

@history_item n=18 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — dropped the visual lint's PROVISIONAL "in process only" mark on the founder's word. It is lineage of the lint row this sweep re-proves.

@history_item n=19 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — six served tasks (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load`) via `boot_cells` and `boot_load`; flight ids became `flight-type` / `flight-type-one-way` / `flight-type-return` (a select, no `aria-pressed`); added the load page's `load-fetch` and `load-done` (`LOAD_ANSWER_DELAY` 200 ms) and the lint at boot including the load page. These are the origin of the contract/escher-session-served-tasks/* rows, the flight-type and load-* id rows, and decision/stand-lints-clean-at-boot's five screens.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading) — `at_boot` moved from `[0, 1, 0, 0]` to `[0, 0, 0, 0]` in both modes by skipping closed-select options (founder F9), retiring the residual line. This is why decision/stand-lints-clean-at-boot names the flight booker.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid) — `cells-grid` became a keyboard region and one Tab stop (author id, `tabindex`, `aria-label` "Cells", no role), the 677th reacting element `stand_non_lean_tasks` reads (676 forecast falsified), with `CELLS_TAB_ORDER` (founder F13). This is the basis of contract/stand-element-ids/cells-grid, and the trap is that counts the sweep re-states must be measured, not forecast.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's Tab sequence) — the load page reads `LOAD_TAB_ORDER` in both modes, and `load-fetch` takes Enter and Space (`stand_keyboard`). This is the basis of contract/stand-element-ids/load-fetch.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`stand_contrast` over the load page and Cells) — a stale claim corrected by the wrap's sweep. Precedent that a sweep finds and restates stale stand claims, and `stand_contrast` is a standing witness the sweep can name.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed) — thirteen citations moved (`cells.rs`, `stand_load_page.rs`, `stand_contrast.rs`, `dioxus_document.rs`, `scrolling.rs`, `stand_lint.rs`). Expect the same when this chunk edits `stand_*` checks.

@history_item n=25 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the stand's colour, depth, spacing and radius rows landed on desktop-native (accent `#3a5ce5`, white on accent 5.47 per `stand_contrast`, spacing, radius). These are part of "the stand's surface as stated" that "stand layout held" re-reads.

@history_item n=26 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines; F2 ratified) — the served-screens, mode-region (select 15px/400), invalid-state and type lines moved here, and the PROVISIONAL gap marker dropped by the founder's ratification. These complete the stand surface values the sweep's layout-held reading checks.

@history_item n=27 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host became a third consumer of the `task_in_shell` mount via `seven_guis::stand` only, adding no markup, and a held instance reads as a fresh boot with the mount and the pinned viewport holding after commands. This is the origin of decision/held-instance-reads-as-fresh-boot and decision/session-roles-share-stand-boot, and of the `service.name=seven_guis` stderr line (contract/escher-session-host/log-service-name) that the stderr content-free measurement reads.

@history_item n=28 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session <verb> [arguments] --session <dir>` with endings exit 0/1/2/3, stdout one uncoloured JSON line, the host role `serve`. This is the origin of contract/escher-session-command/form and the exit-code rows; `service.name=seven_guis` is kept.

@history_item n=29 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role (stdio, end writes nothing on stdout, exit 3 with a fixed stderr message), a further TaskShell-mount consumer, and scoped "no ending writes back caller input" to the commands. These are the origin of contract/escher-session-mcp/end and decision/no-caller-input-echoed, and of the mcp_log stderr surface the sweep measures.

@history_item n=30 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` was added as the tenth verb, with `--session` exactly once on every command but `help`. It is lineage of contract/escher-session-command/form's verb-count wording.

@history_item n=31 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added the `screenshot` verb, and a recorded shot is kept as `{"width","height"}` without its png. This bears on contract/escher-session-record/check-file ("a shot kept without its png") and on what a recording answer may carry.

@history_item n=32 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>`: `check-{n}.json` 0600 in a 0700 dir, with `record-dir-not-private` and `recording-failed` as exit-3 session errors (founder F1 A / F2 B). These are the origin of contract/escher-session-record/* and decision/session-recording-replayable-check, the recording on which "masks held" (`MASKED_STAND_IN`) is asserted.

@history_item n=33 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — a check keeps per call its verb, arguments, acceptance, digest and the answer "masked as on every carrier" (founder F3 B). This is the source of decision/session-recording-replayable-check's masked answer, which the sweep's masks requirement must read on 0.3.0's answers.

@history_item n=34 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` paging (1–1000, out-of-bound refused `malformed`). It is a prior widening of the command line whose answers carry screen readings, relevant to which answers the masks and stderr sweeps must cover.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — `graph` as the ninth instance verb, `{"nodes","states"}` carrying ids, component names, handler event names, state names, flags and booleans, "never a value". This is the origin of contract/escher-session-graph/answer, a 0.3.0 verb whose stderr content-freedom and masks the scope sweeps; ratified at 2026-11-15T05-21-42-wrap.

@history_item n=36 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the screens' source citations) — citations into the task files, `dioxus_document.rs` and `schema.rs` were re-pointed after `use_graph_signal` lines, with no served screen's markup moved. Precedent for the layouts outcome measuring that no stand markup moved.

@history_item n=37 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — `actions` answers `{"nodes"}` of id, owner, keyed and verbs/cause, "never a value, typed text or NodeId", unpaged. This is the origin of contract/escher-session-actions/answer, a 0.3.0 verb under the stderr and masks sweep.

@history_item n=38 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the tab-order citations after the actionable-at-boot table) — `tests/blitz-tests/tests/common/mod.rs` gained `ACTIONABLE_AT_BOOT`, moving the cited `LOAD_TAB_ORDER` / `CELLS_TAB_ORDER` lines. The trap is that any addition this chunk makes to `common/mod.rs` shifts these master citations.

@history_item n=39 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a further consumer of the TaskShell mount) — the `coverage` role boots each check's app through the same boot, adding no markup, a held instance still reading as a fresh boot in both modes (ratified at 2026-11-15T05-21-42-wrap). It is part of decision/session-roles-share-stand-boot (host, mcp, coverage).

@history_item n=40 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's coverage role) — `coverage <app> <check>...` with no `--session`/`--record`; a check read bounded at 1,048,576 + 1 bytes and refused `malformed` holding nothing of the path; answer `{"unexercised","checks","held"}`; a failed boot exits 3. This is the origin of contract/escher-session-coverage/*, a 0.3.0 role whose stderr content-freedom the scope must measure (ratified at 2026-11-15T05-21-42-wrap).

@history_item n=41 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the bridge's skeleton and head notes re-cited) — citations into `dioxus_document.rs` for the fixed skeleton moved one line. This bears on decision/fixed-document-skeleton-main-mount's grounding.

@history_item n=42 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (121 rows; decision 21 · contract 99 · absence 1), each verified against code. This is the form every picked row now takes, and why a layout change this chunk makes lands as `+`/`~`/`−` row amendments at the wrap.

@history_item n=43 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `~ contract/agent-run-cli/usage`: `run` gained the closed selection word `session` (escher-session flows in seven_guis), matched before a file name. The "agent-run re-counted" work re-counts what `run stand|all|session` selects against this row.

@history_item n=44 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `~ contract/escher-session-command/form` (fifteen verbs), `~ contract/escher-session-mcp/tools` (layout after actions), `~ decision/no-caller-input-echoed` (except: the layout answer's id field), `+ contract/escher-session-layout/answer`. v030-11's witness row (CARRY `cry-4cof7vvk`) now names this chunk's witnesses, and `layout` is a 0.3.0 verb under the stderr and masks sweep; the echo exception bounds what "content-free" may allow.

@history_item n=45 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — ratified by the founder's own word the delegate confirm behind 2026-11-17-layout-on-the-carriers' rows (`layout --id`, the no-echo exception). The rows the sweep re-proves stand ratified, not PROVISIONAL.
