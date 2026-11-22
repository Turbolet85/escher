@history_item n=1 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — gave `crud-list` `tabindex="0"` and an `onkeydown` (ArrowDown/ArrowUp/Home/End select and fill the name fields, focus on the list never the row, no `aria-activedescendant`), the origin of decision/crud-list-keyboard-region. Moving escher's additions out of `handle_key_or_input_event` (`packages/blitz-dom/src/events/keyboard.rs`) must leave this key handling and the six-stop Tab sequence unchanged, and the stand keyboard checks are the ones that prove it.

@history_item n=2 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — named `crud-list` `People` by `aria-label` alone and kept the current-option gap stated. That name is read through `build_accessibility_node` (`packages/blitz-dom/src/accessibility.rs`), so the moved accessibility additions must keep the tree's name and the stated gap as they are.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — named the six stand inputs by attributes alone (`for`, `aria-label`), the start of decision/stand-a11y-by-attributes-alone. The accessible name computed from `for`/`aria-label` is the kind of escher addition inside `build_accessibility_node` that this chunk moves out. Every name must read the same after the move.

@history_item n=4 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — added role and state attributes (`aria-invalid`, `role="listbox"`/`option`, `aria-selected`; the `aria-pressed` it added was later retired), with the classes that style those states staying. This completes decision/stand-a11y-by-attributes-alone. The states the snapshot reads come from escher's additions in `build_accessibility_node`, and possibly from the presentational-hint path. The checks that read them are the behaviour bar for the move.

@history_item n=5 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — replaced the flight booker's toggle buttons with `select#flight-type` (`flight-type-one-way` · `flight-type-return`, chosen by `selected`). The snapshot gives each option its select's client rect, the reading behind decision/flight-type-select-boot-reading. It also made the host and `mcp` roles boot six served tasks through the same mount. Select/option roles and bounds in `build_accessibility_node`, and the select's presentational hints, must read the same after the move.

@history_item n=6 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines; the citations sub-entry) — the mode-region bullet fixed the select's styling (15px, weight 400; options showing the chosen one by the engine's select rule), which may rest on `compute_presentational_hints`. The citations sub-entry is the precedent for a take that moved code between files (blitz-vibey-script split into `runtime/` and `dom/element/` modules): the old paths were rewritten, not only their digits. Splitting `packages/escher-driver/src/command.rs` into modules needs the same path re-pointing for any row or guard that cites it.

@history_item n=7 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint; the load page's keys; the Cells grid) — the snapshot still gives a closed select's options their select's client rect (decision/flight-type-select-boot-reading). `load-fetch` takes Enter and Space. `cells-grid` became a second one-Tab-stop keyboard region. These are keyboard paths through `handle_key_or_input_event` and its neighbours, so the keyboard checks (`stand_keyboard`, `stand_non_lean_tasks`, `stand_lint`) are among those to name. The synchronous-handle trap (an async form panicked with `RefCell already borrowed`) is a warning for any reshaping of event handling.

@history_item n=8 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the mode list; the scrolling-boxes exception; the lint's `Entry.in_select_box`) — the open select list is opened by click, Enter or Space and driven by keys while focus stays on the select. Each row reads its own bounds while open, and the rows read back at the select's client rect after a close (decision/open-list-rows-own-bounds). This entry also added the open-list exceptions to decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes. The select's key handling is likely escher's addition in `handle_key_or_input_event`, and the per-row bounds are likely in `build_accessibility_node`. The `stand_act_popup` and `stand_act_scroll` guards must stay green.

@history_item n=9 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the scrolling-boxes note: an into-view scroll moves every holding scroller and then the viewport, and `visible_region` reads what an element can be seen through. The founder ratified this engine rule (2026-10-07), the origin of decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes. It is a ratified behaviour that the restructure must not move.

@history_item n=10 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — states what a scrolled `crud-list` reads: four figures, unnamed in a scroll's diff, and a click landing on the row at its centre at 12 and at 14 Creates. These readings extend the scroller rows and are what the existing checks pin.

@history_item n=11 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — added the session host as a consumer of the `task_in_shell` mount: a held instance reads as a fresh boot, and the mount and pinned viewport hold after commands. This is the origin of decision/held-instance-reads-as-fresh-boot (guard `stand_session_fresh`), which the `command.rs` split must keep.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made `escher-session` the driver's command line (verbs, the host role `serve`, endings 0/1/2/3). The binary in its host role is the mount's consumer, and it adds no markup and keeps a held instance as a fresh boot. This is the command-line code in `packages/escher-driver/src/command.rs` that the split carries. Its endings and usage line are behaviour that must not move.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role as a further consumer of the TaskShell mount, booting in process through the same boot. The `command.rs` split must keep this role's paths and its fresh-boot reading.

@history_item n=14 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a further consumer of the mount; escher-session's coverage role) — `command_line` gained the `coverage` role. It replays each check on a fresh boot through the same boot closure, and a held instance still reads as a fresh boot in both layout modes. This code, with its usage clause and its `MAX_CHECK_BYTES` bound, lives in the command module that this chunk splits. Ratified later (amd-pjh52idb, amd-jc4tin5a).

@history_item n=15 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the coverage role a further consumer of the TaskShell mount; ratified — escher-session's coverage role) — the founder's own word ratified the coverage role entries, so they stand as ratified behaviour that the `command.rs` split must leave unmoved.

@history_item n=16 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the verb table grew to fifteen (`layout` after `actions`). decision/no-caller-input-echoed gained the `layout` id exception. This is the current verb table and command form that the split must keep, along with the public paths `escher_driver::…`.

@history_item n=17 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `graph` took `--part` and answers in parts. This is the chunk that added the 72 lines that pushed `command.rs` from 776 to 848, the size debt this chunk cures. Its paged `graph` answer is among the behaviours the split must keep.

@history_item n=18 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into rows (decisions, contracts, absences) verified against the code. This explains the picked rows' `why`/`guard`/`except` fields and is the form in which any guard or citation that the split moves would be re-read.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the screens' source citations after the graph registrations) — re-pointed citations into the driver's `schema.rs` and `dioxus_document.rs` after line shifts, with no claim moved. This is the precedent for a behaviour-neutral source change in the driver that only re-points citations.
