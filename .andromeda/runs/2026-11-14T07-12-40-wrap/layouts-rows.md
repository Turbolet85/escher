# layout-templates · the rows 2026-11-14-seeded-randomness touches — picked 10 of 120 · rule any · pin 795

## surface-desktop-native · ## Surface: desktop-native
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement or hero/signature section; slice s01's reading did not reach the browser app's window/page layout code (slice s04's facts below cover it).
decision/crud-list-keyboard-region · CRUD list as one keyboard region — The CRUD list is one Tab stop whose ArrowDown, ArrowUp, Home and End move the selected row and fill the name fields as a click does; focus stays on the list, never a row, and no aria-activedescendant is read. · why: Ratified as a decision. · r1
decision/every-stand-control-keyed · Every stand control keyed — Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included. · r1
decision/fixed-document-skeleton-main-mount · Fixed document skeleton with main mount — Every document starts with the fixed skeleton html, head, body and main id="main", and the app mounts into main. · why: stand_snapshot's tree_follows_the_task_shell reads the single /html:0 root with nothing under head, and the bridge's own crash test counts the app's children under main · r1
decision/flight-type-select-boot-reading · Flight type select reads its two options at boot — The snapshot reads the flight booker's mode select at boot as ComboBox "Flight type" with value one-way and two ListBoxOptions, each at the select's client rect. · why: the_flight_type_select_holds_its_two_options in stand_snapshot pins role, name, value, both ListBoxOptions and their select-rect bounds in both layout modes · r1
decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. · r1 · add-to · F1
decision/held-instance-reads-as-fresh-boot · Held instance reads as a fresh boot — A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes. · guard: stand_session_fresh · r1 · add-to · F1
decision/load-page-only-network · Load page alone has a network provider — Only the load page's boot has a network provider, an in-process stand-in answering after 200 ms from a thread of its own, so a driver click on load-fetch waits it out; every other boot keeps no network provider. · guard: stand_load_page · r1
decision/open-list-rows-own-bounds · Open list rows read their own bounds — While a select list is open each row reads its own bounds in the snapshot and the diff, and after a close the rows read back at the select's client rect. · why: stand_act_popup checks a click opening the list names each row's bounds in the diff, and escape closing it puts the rows back at the select's box · r1
decision/scroll-into-view-moves-every-scroller · Into-view scroll moves every holding scroller — An into-view scroll moves every scrolling box that holds its target, innermost first, and then the viewport, in every document. · why: Ratified for every document. · guard: stand_act_scroll · except: A row of an open select list is scrolled into view by writing the list's scroll offset at once, no scrolling box or viewport moving. · r1
decision/session-roles-share-stand-boot · Session roles share the stand boot — Every escher-session role that boots a task (host, mcp, coverage) boots it through the same stand boot the command line uses, adding no markup, id, class, style, wrapper or order. · r1 · add-to · F1
decision/stand-a11y-by-attributes-alone · Stand names, roles and states by attributes alone — Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying. · r1
decision/stand-accent-colour · Stand accent colour — The stand accent is #3a5ce5 (hover #2a4cd3, active #1f3fc1) in every task CSS that declares it, on the page #f5f5f5 with text #1a1a1a, with white button text on accent controls. · why: White text on #3a5ce5 reads 5.47 against SC 1.4.3's 4.5, a pass; the lighter #4a6cf7 reads 4.39, a fail. · guard: stand_contrast · r1
decision/stand-ids-as-attributes-alone · Stand ids as attributes alone — Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them. · r1
decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD and the load page read zero visual-lint findings, in both layout modes. · guard: stand_lint · r1
decision/visible-region-through-clipping-boxes · Visible region through clipping boxes — The part of the viewport an element can be seen through is the viewport narrowed to the padding box of every clipping box that holds it, in every document. · why: Ratified for every document. · except: A row or group of an open select list is seen through the list's scrollport alone, no clipping box answering. · r1
contract/shell-provider-window-chrome/drag-window · ShellProvider window chrome controls · drag_window — Offered to a document drawing its own titlebar, to drag the window. · r1
contract/shell-provider-window-chrome/is-window-maximized · ShellProvider window chrome controls · is_window_maximized — Offered to a document drawing its own titlebar, to read whether the window is maximized. · r1
contract/shell-provider-window-chrome/request-window-close · ShellProvider window chrome controls · request_window_close — Offered to a document drawing its own titlebar, to request the window's close. · r1
contract/shell-provider-window-chrome/set-window-decorations · ShellProvider window chrome controls · set_window_decorations — Offered to a document drawing its own titlebar, to set the window's decorations. · r1
contract/shell-provider-window-chrome/set-window-maximized · ShellProvider window chrome controls · set_window_maximized — Offered to a document drawing its own titlebar, to set the window maximized. · r1
contract/shell-provider-window-chrome/set-window-minimized · ShellProvider window chrome controls · set_window_minimized — Offered to a document drawing its own titlebar, to set the window minimized. · r1
contract/stand-element-ids/cell-column-row · Stand stable element ids · cell-{A..Z}{0..25} — Stable author element id of each of the Cells task's 676 data cells, its column letter then its row number. · r1
contract/stand-element-ids/cell-input · Stand stable element ids · cell-input — Stable author element id of the Cells task's one edit input. · r1
contract/stand-element-ids/cells-grid · Stand stable element ids · cells-grid — Stable author element id of the Cells grid, one Tab stop and keyboard region named "Cells", with no role. · r1
contract/stand-element-ids/circle-canvas · Stand stable element ids · circle-canvas — Stable author element id of the Circle Drawer's canvas. · r1
contract/stand-element-ids/circle-close · Stand stable element ids · circle-close — Stable author element id of the Circle Drawer dialog's close control, present only while the dialog is open. · r1
contract/stand-element-ids/circle-diameter · Stand stable element ids · circle-diameter — Stable author element id of the Circle Drawer dialog's range input, present only while the dialog is open. · r1
contract/stand-element-ids/circle-redo · Stand stable element ids · circle-redo — Stable author element id of the Circle Drawer's redo control. · r1
contract/stand-element-ids/circle-undo · Stand stable element ids · circle-undo — Stable author element id of the Circle Drawer's undo control. · r1
contract/stand-element-ids/counter-increment · Stand stable element ids · counter-increment — Stable author element id of the Counter task's increment button. · r1
contract/stand-element-ids/counter-value · Stand stable element ids · counter-value — Stable author element id of the Counter task's value display. · r1
contract/stand-element-ids/crud-create · Stand stable element ids · crud-create — Stable author element id of the CRUD task's Create button. · r1
contract/stand-element-ids/crud-delete · Stand stable element ids · crud-delete — Stable author element id of the CRUD task's Delete button. · r1
contract/stand-element-ids/crud-filter · Stand stable element ids · crud-filter — Stable author element id of the CRUD task's filter input, named by its label's for. · r1
contract/stand-element-ids/crud-list · Stand stable element ids · crud-list — Stable author element id of the CRUD task's people list, a listbox named "People". · r1
contract/stand-element-ids/crud-name · Stand stable element ids · crud-name — Stable author element id of the CRUD task's name input, named by its label's for. · r1
contract/stand-element-ids/crud-person-id · Stand stable element ids · crud-person-{person.id} — Each CRUD row's stable author element id, from its person's model-assigned id beside the same Dioxus key, following its person under filter, Create and Delete. · r1
contract/stand-element-ids/crud-surname · Stand stable element ids · crud-surname — Stable author element id of the CRUD task's surname input, named by its label's for. · r1
contract/stand-element-ids/crud-update · Stand stable element ids · crud-update — Stable author element id of the CRUD task's Update button. · r1
contract/stand-element-ids/flight-book · Stand stable element ids · flight-book — Stable author element id of the flight booker's book button. · r1
contract/stand-element-ids/flight-booked · Stand stable element ids · flight-booked — Stable author element id of the flight booker's booked display. · r1
contract/stand-element-ids/flight-return-date · Stand stable element ids · flight-return-date — Stable author element id of the flight booker's return date input, named "Return date" by aria-label. · r1
contract/stand-element-ids/flight-start · Stand stable element ids · flight-start — Stable author element id of the flight booker's departure date input, named "Departure date" by aria-label. · r1
contract/stand-element-ids/flight-type · Stand stable element ids · flight-type — Stable author element id of the flight booker's mode select. · r1
contract/stand-element-ids/flight-type-one-way · Stand stable element ids · flight-type-one-way — Stable author element id of the flight booker's one-way mode option. · r1
contract/stand-element-ids/flight-type-return · Stand stable element ids · flight-type-return — Stable author element id of the flight booker's return mode option. · r1
contract/stand-element-ids/load-done · Stand stable element ids · load-done — Stable author element id of the load page's "Loaded" paragraph, hidden until the stylesheet the button's click links lands. · r1
contract/stand-element-ids/load-fetch · Stand stable element ids · load-fetch — Stable author element id of the load page's Load button, taking Enter and Space. · r1
contract/stand-element-ids/task-card-cells · Stand stable element ids · task-card-cells — Stable author element id of Home's Cells task-card button. · r1
contract/stand-element-ids/task-card-circle-drawer · Stand stable element ids · task-card-circle-drawer — Stable author element id of Home's Circle Drawer task-card button. · r1
contract/stand-element-ids/task-card-counter · Stand stable element ids · task-card-counter — Stable author element id of Home's Counter task-card button. · r1
contract/stand-element-ids/task-card-crud · Stand stable element ids · task-card-crud — Stable author element id of Home's CRUD task-card button. · r1
contract/stand-element-ids/task-card-flight-booker · Stand stable element ids · task-card-flight-booker — Stable author element id of Home's Flight Booker task-card button. · r1
contract/stand-element-ids/task-card-temp-converter · Stand stable element ids · task-card-temp-converter — Stable author element id of Home's Temperature Converter task-card button. · r1
contract/stand-element-ids/task-card-timer · Stand stable element ids · task-card-timer — Stable author element id of Home's Timer task-card button. · r1
contract/stand-element-ids/temp-celsius · Stand stable element ids · temp-celsius — Stable author element id of the Temperature Converter's Celsius control. · r1
contract/stand-element-ids/temp-fahrenheit · Stand stable element ids · temp-fahrenheit — Stable author element id of the Temperature Converter's Fahrenheit control. · r1
contract/stand-element-ids/timer-duration · Stand stable element ids · timer-duration — Stable author element id of the Timer task's duration input, named by its label's for. · r1
contract/stand-element-ids/timer-duration-value · Stand stable element ids · timer-duration-value — Stable author element id of the Timer task's duration value display. · r1
contract/stand-element-ids/timer-elapsed · Stand stable element ids · timer-elapsed — Stable author element id of the Timer task's elapsed display. · r1
contract/stand-element-ids/timer-progress · Stand stable element ids · timer-progress — Stable author element id of the Timer task's progress display. · r1
contract/stand-element-ids/timer-reset · Stand stable element ids · timer-reset — Stable author element id of the Timer task's reset button. · r1
absence/arbitrary-index-html-templates — Arbitrary index.html templates for a document · r1

## surface-cli · ## Surface: cli
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level or signature placement; slice s03's reading did not reach any further CLI help or argument parser beyond the `paint_bench` usage line.
decision/cold-agent-pipe-builds-nothing · Cold-agent pipe builds nothing — The cold-agent pipe builds nothing; a missing escher-session binary is a failed precondition. · r1
decision/cold-agent-prompt-names-no-id · Cold-agent prompt names no id — The cold-agent stand run uses one fixed prompt that names no element id or app slug. · r1
decision/cold-agent-uses-driver-mcp · Cold agent drives through the driver's MCP server — The cold-agent client is given the driver's own MCP server, escher-session mcp, spawned through a per-run MCP configuration. · r1
decision/mcp-server-local-stdio · MCP server local over stdio — The escher-session mcp role is an MCP server over stdio, local to the invoking user, with no listener or auth surface. · r1
decision/no-caller-input-echoed · Caller input never written back — No escher-session command's ending writes back anything the caller typed, a recording path included. · r1
decision/session-recording-replayable-check · Session recording as a replayable check — --record <dir> keeps a session as one owner-only replayable check, per call its verb, arguments, acceptance, answer digest and masked answer, read back and replayed by the check replay. · why: Ratified. · r1 · add-to · F2
contract/agent-run-cli/exit-2 · agent-run.sh test contract · exit 2 — A usage error prints the usage to stderr and exits 2. · r1
contract/agent-run-cli/run-log · agent-run.sh test contract · target/agent-run/run.log — The raw cargo output stays in this file, never on stdout. · r1
contract/agent-run-cli/stdout · agent-run.sh test contract · stdout — Every verb's stdout is JSON lines only, one object per event. · r1
contract/agent-run-cli/usage · agent-run.sh test contract · usage — agent-run.sh <verb> [selection], run from the repository root, with verbs boot, run, status, cleanup, logs and run selections stand, all or a blitz-tests file name. · r1
contract/agent-run-cli/windows-pass-through · agent-run.sh test contract · agent-run.ps1 — The Windows pass-through to agent-run.sh. · r1
contract/cold-agent-cli/exit-2 · cold-agent.sh run pipe · exit 2 — A usage error prints to stderr with empty stdout and exits 2, checked before any precondition. · r1
contract/cold-agent-cli/exit-3 · cold-agent.sh run pipe · exit 3 — run without claude, python3 or a built, executable target/debug/escher-session exits 3 with one stderr line naming the cargo build command to run. · r1
contract/cold-agent-cli/record-dir · cold-agent.sh run pipe · target/cold-agent/record — The live run's server records its kept checks into this owner-only directory, removed by cleanup with the state, the checks never printed; replays run without recording. · r1
contract/cold-agent-cli/stdout · cold-agent.sh run pipe · stdout — Every verb's stdout is JSON lines only, one object per event; the session transcript and the agent's check stay in files under target/cold-agent, never printed. · r1
contract/cold-agent-cli/usage · cold-agent.sh run pipe · usage — cold-agent.sh <verb> [task], run from the repository root, with verbs run, status, cleanup, logs and the one task stand: six apps in one client session under a fixed prompt. · r1
contract/cold-agent-cli/windows-pass-through · cold-agent.sh run pipe · cold-agent.ps1 — The Windows pass-through to cold-agent.sh. · r1
contract/escher-session-actions/answer · escher-session actions · answer — No argument; one unpaged uncoloured line {"nodes"} of id, owner, keyed and verbs with verb and cause, owner and cause left out when absent, never a value, typed text or NodeId. · r1 · add-to · F2
contract/escher-session-command/form · escher-session command line · command form — Every command is escher-session <verb> [arguments] --session <dir> over the fourteen verbs of the driver's verb table, --session exactly once on every command but help. · r1 · add-to · F2
contract/escher-session-coverage/answer · escher-session coverage role · answer — Each check replayed on a fresh boot; one uncoloured line {"unexercised":[{id,owner,event}],"checks","held"}, owner left out when absent; a failed boot is the session error, exit 3. · r1 · add-to · F2
contract/escher-session-coverage/check-bound · escher-session coverage role · check file — Each check is read bounded at 1,048,576 + 1 bytes; unreadable, oversized, not a check, or for another app is refused malformed, exit 1, holding nothing of the path or file. · r1
contract/escher-session-coverage/usage · escher-session coverage role · usage — escher-session coverage <app> <check>..., no --session or --record; any other shape is usage, exit 2. · r1 · add-to · F2
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: the answer as one line of uncoloured JSON on stdout. · r1
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: the refusal's JSON on stdout. · r1
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from the verb table and the task names, stdout empty, nothing booted and no state created. · r1
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — Session error: the error object with its fixed remedy on stdout and its fixed message on stderr. · r1
contract/escher-session-graph/answer · escher-session graph · answer — No argument; one uncoloured line {"nodes","states"} carrying ids, component names, handler event names, state names, closed flag words and booleans only, never a value. · r1
contract/escher-session-help/answer · escher-session help · answer — One uncoloured JSON line with seven keys (verbs, node_keys, key_names, busy_classes, causes, exit_statuses, session_errors), fixed schema strings, reaching no session. · r1
contract/escher-session-host/log-service-name · escher-session host log · service.name — The host's stderr log lines carry service.name=seven_guis. · r1
contract/escher-session-lint/answer · escher-session lint · answer — No argument; one uncoloured line {"findings"}, each finding keyed id, defect, bounds then its reason's keys, carrying ids, closed words and numbers only. · r1
contract/escher-session-mcp/end · escher-session MCP server · end — The server's end writes nothing on stdout: exit 0 on Ok, exit 3 with the error's fixed message on stderr on Err. · r1
contract/escher-session-mcp/protocol-revisions · escher-session MCP server · protocol revisions — Speaks four MCP handshake revisions and answers its newest one to a client asking for any revision it does not speak. · r1
contract/escher-session-mcp/result-bound · escher-session MCP server · result bound — A result holding an image is bounded at 1,048,576 bytes by its whole written form, a text-only one by its text. · r1
contract/escher-session-mcp/screenshot-result · escher-session MCP server · screenshot result — An accepted screenshot answers an image block (base64 PNG, image/png) then a text block without png, isError false; a refused shot one text block, isError true. · r1
contract/escher-session-mcp/stdio · escher-session MCP server · stdio — Reads newline-delimited JSON-RPC 2.0 requests on stdin and writes protocol lines only on stdout, uncoloured with no ESC byte, echoing a request's admitted id. · r1
contract/escher-session-mcp/tools · escher-session MCP server · tools/list — Lists the fourteen verbs of the verb table as tools in order, help last, each description the verb's help then ". Returns: " and its result fields. · r1 · add-to · F2
contract/escher-session-record/check-file · escher-session recording · check-{n}.json — Written 0600, never replacing a file, in a 0700 directory whose parent must exist, removed by escher only when its recording fails; a shot kept without its png. · r1 · add-to · F2
contract/escher-session-record/record-dir-not-private · escher-session recording · record-dir-not-private — A recording directory open to its group or others is this session error, exit 3, start spawning nothing. · r1
contract/escher-session-record/recording-failed · escher-session recording · recording-failed — A recording that ends answers this session error, exit 3; over MCP isError. · r1
contract/escher-session-refusals/fonts-unavailable · escher-session refusals · fonts-unavailable — screenshot or lint on an app with no font it can paint text with is refused fonts-unavailable, exit 1. · r1
contract/escher-session-refusals/off-screen · escher-session refusals · off-screen — An element screenshot that crops to no pixel is refused off-screen, exit 1. · r1
contract/escher-session-refusals/unknown-verb · escher-session refusals · unknown-verb — A first word the verb table does not hold is refused unknown-verb, exit 1. · r1
contract/escher-session-screenshot/answer · escher-session screenshot · answer — One uncoloured line {"width","height","png"} with base64 PNG; with --id the named element cropped to its visible bounds, without it the whole screen. · r1
contract/escher-session-served-tasks/cells · escher-session served tasks · cells — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum. · r1
contract/escher-session-served-tasks/counter · escher-session served tasks · counter — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum. · r1
contract/escher-session-served-tasks/crud · escher-session served tasks · crud — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum. · r1
contract/escher-session-served-tasks/flight-booker · escher-session served tasks · flight-booker — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum. · r1
contract/escher-session-served-tasks/load · escher-session served tasks · load — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum. · r1
contract/escher-session-served-tasks/timer · escher-session served tasks · timer — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum. · r1
contract/escher-session-snapshot/answer · escher-session snapshot · answer — One uncoloured line: {"text"} for a whole screen, {"text","part","parts"} for a screen read in parts; a part past the last gives empty text with the same parts. · r1
contract/escher-session-snapshot/part · escher-session snapshot · --part — Optional whole number 1 to 1000, part 1 without it; outside the bound refused malformed, exit 1. · r1
contract/multi-doc-cli/arguments · multi_doc command line · arguments — multi_doc takes a mode and an iteration count as positional arguments; an unknown mode ends in a panic. · r1
contract/multi-doc-cli/multi-doc-flat · multi_doc command line · MULTI_DOC_FLAT — A flag that makes multi_doc's top-level elements leaves instead of <div><p/><span/></div>. · r1
contract/multi-doc-cli/multi-doc-n · multi_doc command line · MULTI_DOC_N — Sets the document count, a default applying when absent or unparsed. · r1
contract/multi-doc-cli/multi-doc-seq · multi_doc command line · MULTI_DOC_SEQ — A flag that sets multi_doc's style threading to Sequential instead of Parallel. · r1
contract/multi-doc-cli/stdout · multi_doc command line · stdout — multi_doc's timing is printed as one line on stdout. · r1
contract/paint-bench-cli/arguments · paint_bench command line · arguments — paint_bench takes <url> [width] [height] [scale] [iters] [backend], positionally. · r1
contract/paint-bench-cli/backend · paint_bench command line · backend — One of vello (the default), cpu or hybrid. · r1
