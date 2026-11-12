@row_none n=1 rec=fct-vrnkoqnl@1 type=4
@row_none n=2 rec=fct-654t6lta@1 type=4
@row_none n=3 rec=fct-n7fg5vzn@1 type=4
@row_add n=1 row=decision/headless-stand-pinned-viewport area=surface-desktop-native rec=fct-nxmh4fah@2
name: Headless stand pinned viewport
rule: The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
@row_add n=2 row=decision/session-roles-share-stand-boot area=surface-desktop-native rec=fct-nxmh4fah@2
name: Session roles share the stand boot
rule: Every escher-session role that boots a task (host, mcp, coverage) boots it through the same stand boot the command line uses, adding no markup, id, class, style, wrapper or order.
@row_add n=3 row=decision/held-instance-reads-as-fresh-boot area=surface-desktop-native rec=fct-nxmh4fah@2
name: Held instance reads as a fresh boot
rule: A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes.
guard: stand_session_fresh
@row_add n=4 row=decision/stand-ids-as-attributes-alone area=surface-desktop-native rec=fct-nxmh4fah@2
name: Stand ids as attributes alone
rule: Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them.
@row_add n=5 row=decision/stand-a11y-by-attributes-alone area=surface-desktop-native rec=fct-nxmh4fah@2
name: Stand names, roles and states by attributes alone
rule: Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
@row_add n=6 row=decision/every-stand-control-keyed area=surface-desktop-native rec=fct-nxmh4fah@2
name: Every stand control keyed
rule: Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
@row_add n=7 row=decision/crud-list-keyboard-region area=surface-desktop-native rec=fct-nxmh4fah@2
name: CRUD list as one keyboard region
rule: The CRUD list is one Tab stop whose ArrowDown, ArrowUp, Home and End move the selected row and fill the name fields as a click does; focus stays on the list, never a row, and no aria-activedescendant is read.
why: Ratified as a decision.
@row_add n=8 row=contract/stand-element-ids/task-card-counter area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-counter
promise: Stable author element id of Home's Counter task-card button.
@row_add n=9 row=contract/stand-element-ids/task-card-temp-converter area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-temp-converter
promise: Stable author element id of Home's Temperature Converter task-card button.
@row_add n=10 row=contract/stand-element-ids/task-card-flight-booker area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-flight-booker
promise: Stable author element id of Home's Flight Booker task-card button.
@row_add n=11 row=contract/stand-element-ids/task-card-timer area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-timer
promise: Stable author element id of Home's Timer task-card button.
@row_add n=12 row=contract/stand-element-ids/task-card-crud area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-crud
promise: Stable author element id of Home's CRUD task-card button.
@row_add n=13 row=contract/stand-element-ids/task-card-circle-drawer area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-circle-drawer
promise: Stable author element id of Home's Circle Drawer task-card button.
@row_add n=14 row=contract/stand-element-ids/task-card-cells area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-cells
promise: Stable author element id of Home's Cells task-card button.
@row_add n=15 row=contract/stand-element-ids/counter-value area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: counter-value
promise: Stable author element id of the Counter task's value display.
@row_add n=16 row=contract/stand-element-ids/counter-increment area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: counter-increment
promise: Stable author element id of the Counter task's increment button.
@row_add n=17 row=contract/stand-element-ids/flight-type area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-type
promise: Stable author element id of the flight booker's mode select.
@row_add n=18 row=contract/stand-element-ids/flight-type-one-way area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-type-one-way
promise: Stable author element id of the flight booker's one-way mode option.
@row_add n=19 row=contract/stand-element-ids/flight-type-return area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-type-return
promise: Stable author element id of the flight booker's return mode option.
@row_add n=20 row=contract/stand-element-ids/flight-start area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-start
promise: Stable author element id of the flight booker's departure date input, named "Departure date" by aria-label.
@row_add n=21 row=contract/stand-element-ids/flight-return-date area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-return-date
promise: Stable author element id of the flight booker's return date input, named "Return date" by aria-label.
@row_add n=22 row=contract/stand-element-ids/flight-book area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-book
promise: Stable author element id of the flight booker's book button.
@row_add n=23 row=contract/stand-element-ids/flight-booked area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-booked
promise: Stable author element id of the flight booker's booked display.
@row_add n=24 row=contract/stand-element-ids/timer-progress area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: timer-progress
promise: Stable author element id of the Timer task's progress display.
@row_add n=25 row=contract/stand-element-ids/timer-elapsed area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: timer-elapsed
promise: Stable author element id of the Timer task's elapsed display.
@row_add n=26 row=contract/stand-element-ids/timer-duration area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: timer-duration
promise: Stable author element id of the Timer task's duration input, named by its label's for.
@row_add n=27 row=contract/stand-element-ids/timer-duration-value area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: timer-duration-value
promise: Stable author element id of the Timer task's duration value display.
@row_add n=28 row=contract/stand-element-ids/timer-reset area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: timer-reset
promise: Stable author element id of the Timer task's reset button.
@row_add n=29 row=contract/stand-element-ids/crud-filter area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-filter
promise: Stable author element id of the CRUD task's filter input, named by its label's for.
@row_add n=30 row=contract/stand-element-ids/crud-list area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-list
promise: Stable author element id of the CRUD task's people list, a listbox named "People".
@row_add n=31 row=contract/stand-element-ids/crud-name area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-name
promise: Stable author element id of the CRUD task's name input, named by its label's for.
@row_add n=32 row=contract/stand-element-ids/crud-surname area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-surname
promise: Stable author element id of the CRUD task's surname input, named by its label's for.
@row_add n=33 row=contract/stand-element-ids/crud-create area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-create
promise: Stable author element id of the CRUD task's Create button.
@row_add n=34 row=contract/stand-element-ids/crud-update area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-update
promise: Stable author element id of the CRUD task's Update button.
@row_add n=35 row=contract/stand-element-ids/crud-delete area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-delete
promise: Stable author element id of the CRUD task's Delete button.
@row_add n=36 row=contract/stand-element-ids/crud-person-id area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-person-{person.id}
promise: Each CRUD row's stable author element id, from its person's model-assigned id beside the same Dioxus key, following its person under filter, Create and Delete.
@row_add n=37 row=contract/stand-element-ids/temp-celsius area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: temp-celsius
promise: Stable author element id of the Temperature Converter's Celsius control.
@row_add n=38 row=contract/stand-element-ids/temp-fahrenheit area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: temp-fahrenheit
promise: Stable author element id of the Temperature Converter's Fahrenheit control.
@row_add n=39 row=contract/stand-element-ids/circle-undo area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: circle-undo
promise: Stable author element id of the Circle Drawer's undo control.
@row_add n=40 row=contract/stand-element-ids/circle-redo area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: circle-redo
promise: Stable author element id of the Circle Drawer's redo control.
@row_add n=41 row=contract/stand-element-ids/circle-canvas area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: circle-canvas
promise: Stable author element id of the Circle Drawer's canvas.
@row_add n=42 row=contract/stand-element-ids/circle-diameter area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: circle-diameter
promise: Stable author element id of the Circle Drawer dialog's range input, present only while the dialog is open.
@row_add n=43 row=contract/stand-element-ids/circle-close area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: circle-close
promise: Stable author element id of the Circle Drawer dialog's close control, present only while the dialog is open.
@row_add n=44 row=contract/stand-element-ids/cell-column-row area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: cell-{A..Z}{0..25}
promise: Stable author element id of each of the Cells task's 676 data cells, its column letter then its row number.
@row_add n=45 row=contract/stand-element-ids/cell-input area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: cell-input
promise: Stable author element id of the Cells task's one edit input.
@row_none n=4 rec=fct-2nmc7hj4@1 type=S
@row_add n=46 row=decision/load-page-only-network area=surface-desktop-native rec=fct-5a7qhqx6@1
name: Load page alone has a network provider
rule: Only the load page's boot has a network provider, an in-process stand-in answering after 200 ms from a thread of its own, so a driver click on load-fetch waits it out; every other boot keeps no network provider.
guard: stand_load_page
@row_add n=47 row=contract/stand-element-ids/load-fetch area=surface-desktop-native rec=fct-5a7qhqx6@1
boundary: Stand stable element ids
item: load-fetch
promise: Stable author element id of the load page's Load button, taking Enter and Space.
@row_add n=48 row=contract/stand-element-ids/load-done area=surface-desktop-native rec=fct-5a7qhqx6@1
boundary: Stand stable element ids
item: load-done
promise: Stable author element id of the load page's "Loaded" paragraph, hidden until the stylesheet the button's click links lands.
@row_none n=5 rec=fct-bla7ziiz@1 type=4
relied: The snapshot reads the flight booker's mode select at boot as ComboBox "Flight type" with value one-way and two ListBoxOptions, each at the select's client rect.
@row_none n=6 rec=fct-zk3a7kfe@1 type=4
relied: While a select list is open each row reads its own bounds in the snapshot and the diff, and after a close the rows read back at the select's client rect.
@row_add n=49 row=contract/stand-element-ids/cells-grid area=surface-desktop-native rec=fct-xlkkhy76@1
boundary: Stand stable element ids
item: cells-grid
promise: Stable author element id of the Cells grid, one Tab stop and keyboard region named "Cells", with no role.
@row_add n=50 row=decision/stand-accent-colour area=surface-desktop-native rec=fct-xlzsjdlc@1
name: Stand accent colour
rule: The stand accent is #3a5ce5 (hover #2a4cd3, active #1f3fc1) in every task CSS that declares it, on the page #f5f5f5 with text #1a1a1a, with white button text on accent controls.
why: White text on #3a5ce5 reads 5.47 against SC 1.4.3's 4.5, a pass; the lighter #4a6cf7 reads 4.39, a fail.
guard: stand_contrast
@row_none n=7 rec=fct-7z6s6h7n@1 type=4
@row_none n=8 rec=fct-dj37firg@1 type=4
@row_none n=9 rec=fct-tgcrnro7@1 type=4
@row_none n=10 rec=fct-yzd6nis7@1 type=4
@row_none n=11 rec=fct-4wzmflh5@1 type=4
@row_none n=12 rec=fct-7moghy5x@1 type=S
@row_none n=13 rec=fct-ypw4imkp@1 type=4
@row_none n=14 rec=fct-7tw7uyf2@1 type=4
@row_none n=15 rec=fct-wryrwkrm@1 type=4
@row_none n=16 rec=fct-nz5alqsh@1 type=4
@row_none n=17 rec=fct-p5cpvfgx@1 type=4
@row_none n=18 rec=fct-soimm2g7@1 type=4
@row_none n=19 rec=fct-infp5ncw@1 type=4
@row_none n=20 rec=fct-f4ald5en@1 type=4
@row_add n=51 row=contract/shell-provider-window-chrome/request-window-close area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: request_window_close
promise: Offered to a document drawing its own titlebar, to request the window's close.
@row_add n=52 row=contract/shell-provider-window-chrome/set-window-minimized area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: set_window_minimized
promise: Offered to a document drawing its own titlebar, to set the window minimized.
@row_add n=53 row=contract/shell-provider-window-chrome/set-window-maximized area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: set_window_maximized
promise: Offered to a document drawing its own titlebar, to set the window maximized.
@row_add n=54 row=contract/shell-provider-window-chrome/is-window-maximized area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: is_window_maximized
promise: Offered to a document drawing its own titlebar, to read whether the window is maximized.
@row_add n=55 row=contract/shell-provider-window-chrome/set-window-decorations area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: set_window_decorations
promise: Offered to a document drawing its own titlebar, to set the window's decorations.
@row_add n=56 row=contract/shell-provider-window-chrome/drag-window area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: drag_window
promise: Offered to a document drawing its own titlebar, to drag the window.
@row_none n=21 rec=fct-sf4ltnp3@1 type=4
@row_none n=22 rec=fct-goa5h6cv@1 type=4
@row_none n=23 rec=fct-nasbtqcq@1 type=4
@row_none n=24 rec=fct-3yebjmlo@1 type=4
relied: Every document starts with the fixed skeleton html, head, body and main id="main", and the app mounts into main.
@row_add n=57 row=absence/arbitrary-index-html-templates area=surface-desktop-native rec=fct-wu6baq7k@1
capability: Arbitrary index.html templates for a document
@row_none n=25 rec=fct-bvy3ajht@1 type=4
@row_none n=26 rec=fct-3kjbfxzm@1 type=4
@row_none n=27 rec=fct-aswgwjrz@1 type=4
@row_none n=28 rec=fct-nlsolafi@1 type=4
@row_none n=29 rec=fct-qqfmbabe@1 type=4
@row_none n=30 rec=fct-bb6g774i@1 type=4
@row_none n=31 rec=fct-ou5kiwlz@1 type=4
@row_none n=32 rec=fct-r4mtslde@1 type=4
@row_add n=58 row=decision/scroll-into-view-moves-every-scroller area=surface-desktop-native rec=fct-kontkuux@1
name: Into-view scroll moves every holding scroller
rule: An into-view scroll moves every scrolling box that holds its target, innermost first, and then the viewport, in every document.
why: Ratified for every document.
guard: stand_act_scroll
except: A row of an open select list is scrolled into view by writing the list's scroll offset at once, no scrolling box or viewport moving.
@row_add n=59 row=decision/visible-region-through-clipping-boxes area=surface-desktop-native rec=fct-kontkuux@1
name: Visible region through clipping boxes
rule: The part of the viewport an element can be seen through is the viewport narrowed to the padding box of every clipping box that holds it, in every document.
why: Ratified for every document.
except: A row or group of an open select list is seen through the list's scrollport alone, no clipping box answering.
@row_add n=60 row=decision/stand-lints-clean-at-boot area=surface-desktop-native rec=fct-g6ui4uo3@1
name: Stand lints clean at boot
rule: At boot the counter, the flight booker, the timer, CRUD and the load page read zero visual-lint findings, in both layout modes.
guard: stand_lint
@row_none n=33 rec=fct-hgjbw4ph@1 type=S
@row_none n=34 rec=fct-crjk6am2@1 type=4
@row_none n=35 rec=fct-fwzvlguk@1 type=4
@row_none n=36 rec=fct-sqslkbda@1 type=4
@row_none n=37 rec=fct-btkcmgpp@1 type=S
@row_add n=61 row=contract/paint-bench-cli/arguments area=surface-cli rec=fct-h7hmdmzm@1
boundary: paint_bench command line
item: arguments
promise: paint_bench takes <url> [width] [height] [scale] [iters] [backend], positionally.
@row_add n=62 row=contract/paint-bench-cli/backend area=surface-cli rec=fct-h7hmdmzm@1
boundary: paint_bench command line
item: backend
promise: One of vello (the default), cpu or hybrid.
@row_add n=63 row=contract/multi-doc-cli/arguments area=surface-cli rec=fct-qxr4jcph@1
boundary: multi_doc command line
item: arguments
promise: multi_doc takes a mode and an iteration count as positional arguments; an unknown mode ends in a panic.
@row_add n=64 row=contract/multi-doc-cli/multi-doc-n area=surface-cli rec=fct-qxr4jcph@1
boundary: multi_doc command line
item: MULTI_DOC_N
promise: Sets the document count, a default applying when absent or unparsed.
@row_add n=65 row=contract/multi-doc-cli/multi-doc-flat area=surface-cli rec=fct-qxr4jcph@1
boundary: multi_doc command line
item: MULTI_DOC_FLAT
promise: A flag that sets multi_doc's layout and style threading.
@row_add n=66 row=contract/multi-doc-cli/multi-doc-seq area=surface-cli rec=fct-qxr4jcph@1
boundary: multi_doc command line
item: MULTI_DOC_SEQ
promise: A flag that sets multi_doc's layout and style threading.
@row_add n=67 row=contract/multi-doc-cli/stdout area=surface-cli rec=fct-qxr4jcph@1
boundary: multi_doc command line
item: stdout
promise: multi_doc's timing is printed as one line on stdout.
@row_add n=68 row=contract/agent-run-cli/usage area=surface-cli rec=fct-am364u2w@1
boundary: agent-run.sh test contract
item: usage
promise: agent-run.sh <verb> [selection], run from the repository root, with verbs boot, run, status, cleanup, logs and run selections stand, all or a blitz-tests file name.
@row_add n=69 row=contract/agent-run-cli/exit-2 area=surface-cli rec=fct-am364u2w@1
boundary: agent-run.sh test contract
item: exit 2
promise: A usage error prints the usage to stderr and exits 2.
@row_add n=70 row=contract/agent-run-cli/stdout area=surface-cli rec=fct-am364u2w@1
boundary: agent-run.sh test contract
item: stdout
promise: Every verb's stdout is JSON lines only, one object per event.
@row_add n=71 row=contract/agent-run-cli/run-log area=surface-cli rec=fct-am364u2w@1
boundary: agent-run.sh test contract
item: target/agent-run/run.log
promise: The raw cargo output stays in this file, never on stdout.
@row_add n=72 row=contract/agent-run-cli/windows-pass-through area=surface-cli rec=fct-am364u2w@1
boundary: agent-run.sh test contract
item: agent-run.ps1
promise: The Windows pass-through to agent-run.sh.
@row_add n=73 row=contract/cold-agent-cli/usage area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: usage
promise: cold-agent.sh <verb> [task], run from the repository root, with verbs run, status, cleanup, logs and the one task stand: six apps in one client session under a fixed prompt.
@row_add n=74 row=contract/cold-agent-cli/exit-2 area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: exit 2
promise: A usage error prints to stderr with empty stdout and exits 2, checked before any precondition.
@row_add n=75 row=contract/cold-agent-cli/exit-3 area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: exit 3
promise: run without claude, python3 or a built, executable target/debug/escher-session exits 3 with one stderr line naming the cargo build command to run.
@row_add n=76 row=contract/cold-agent-cli/stdout area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: stdout
promise: Every verb's stdout is JSON lines only, one object per event; the session transcript and the agent's check stay in files under target/cold-agent, never printed.
@row_add n=77 row=contract/cold-agent-cli/record-dir area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: target/cold-agent/record
promise: The live run's server records its kept checks into this owner-only directory, removed by cleanup with the state, the checks never printed; replays run without recording.
@row_add n=78 row=contract/cold-agent-cli/windows-pass-through area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: cold-agent.ps1
promise: The Windows pass-through to cold-agent.sh.
@row_add n=79 row=decision/cold-agent-prompt-names-no-id area=surface-cli rec=fct-b7sh5og5@1
name: Cold-agent prompt names no id
rule: The cold-agent stand run uses one fixed prompt that names no element id or app slug.
@row_add n=80 row=decision/cold-agent-pipe-builds-nothing area=surface-cli rec=fct-b7sh5og5@1
name: Cold-agent pipe builds nothing
rule: The cold-agent pipe builds nothing; a missing escher-session binary is a failed precondition.
@row_add n=81 row=decision/cold-agent-uses-driver-mcp area=surface-cli rec=fct-b7sh5og5@1
name: Cold agent drives through the driver's MCP server
rule: The cold-agent client is given the driver's own MCP server, escher-session mcp, spawned through a per-run MCP configuration.
@row_add n=82 row=contract/escher-session-command/form area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session command line
item: command form
promise: Every command is escher-session <verb> [arguments] --session <dir> over the fourteen verbs of the driver's verb table, --session exactly once on every command but help.
@row_add n=83 row=contract/escher-session-exit-codes/0 area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session exit codes
item: 0
promise: Accepted: the answer as one line of uncoloured JSON on stdout.
@row_add n=84 row=contract/escher-session-exit-codes/1 area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session exit codes
item: 1
promise: Refused: the refusal's JSON on stdout.
@row_add n=85 row=contract/escher-session-exit-codes/2 area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session exit codes
item: 2
promise: Usage: one fixed line on stderr built from the verb table and the task names, stdout empty, nothing booted and no state created.
@row_add n=86 row=contract/escher-session-exit-codes/3 area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session exit codes
item: 3
promise: Session error: the error object with its fixed remedy on stdout and its fixed message on stderr.
@row_add n=87 row=contract/escher-session-refusals/unknown-verb area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session refusals
item: unknown-verb
promise: A first word the verb table does not hold is refused unknown-verb, exit 1.
@row_add n=88 row=contract/escher-session-refusals/fonts-unavailable area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session refusals
item: fonts-unavailable
promise: screenshot or lint on an app with no font it can paint text with is refused fonts-unavailable, exit 1.
@row_add n=89 row=contract/escher-session-refusals/off-screen area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session refusals
item: off-screen
promise: An element screenshot that crops to no pixel is refused off-screen, exit 1.
@row_add n=90 row=contract/escher-session-snapshot/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session snapshot
item: answer
promise: One uncoloured line: {"text"} for a whole screen, {"text","part","parts"} for a screen read in parts; a part past the last gives empty text with the same parts.
@row_add n=91 row=contract/escher-session-snapshot/part area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session snapshot
item: --part
promise: Optional whole number 1 to 1000, part 1 without it; outside the bound refused malformed, exit 1.
@row_add n=92 row=contract/escher-session-screenshot/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session screenshot
item: answer
promise: One uncoloured line {"width","height","png"} with base64 PNG; with --id the named element cropped to its visible bounds, without it the whole screen.
@row_add n=93 row=contract/escher-session-lint/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session lint
item: answer
promise: No argument; one uncoloured line {"findings"}, each finding keyed id, defect, bounds then its reason's keys, carrying ids, closed words and numbers only.
@row_add n=94 row=contract/escher-session-graph/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session graph
item: answer
promise: No argument; one uncoloured line {"nodes","states"} carrying ids, component names, handler event names, state names, closed flag words and booleans only, never a value.
@row_add n=95 row=contract/escher-session-actions/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session actions
item: answer
promise: No argument; one unpaged uncoloured line {"nodes"} of id, owner, keyed and verbs with verb and cause, owner and cause left out when absent, never a value, typed text or NodeId.
@row_add n=96 row=contract/escher-session-help/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session help
item: answer
promise: One uncoloured JSON line with seven keys (verbs, node_keys, key_names, busy_classes, causes, exit_statuses, session_errors), fixed schema strings, reaching no session.
@row_add n=97 row=contract/escher-session-served-tasks/counter area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: counter
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_add n=98 row=contract/escher-session-served-tasks/flight-booker area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: flight-booker
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_add n=99 row=contract/escher-session-served-tasks/timer area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: timer
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_add n=100 row=contract/escher-session-served-tasks/crud area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: crud
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_add n=101 row=contract/escher-session-served-tasks/cells area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: cells
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_add n=102 row=contract/escher-session-served-tasks/load area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: load
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_add n=103 row=decision/no-caller-input-echoed area=surface-cli rec=fct-exqct2ck@4
name: Caller input never written back
rule: No escher-session command's ending writes back anything the caller typed, a recording path included.
@row_add n=104 row=decision/mcp-server-local-stdio area=surface-cli rec=fct-exqct2ck@4
name: MCP server local over stdio
rule: The escher-session mcp role is an MCP server over stdio, local to the invoking user, with no listener or auth surface.
@row_add n=105 row=contract/escher-session-mcp/stdio area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: stdio
promise: Reads newline-delimited JSON-RPC 2.0 requests on stdin and writes protocol lines only on stdout, uncoloured with no ESC byte, echoing a request's admitted id.
@row_add n=106 row=contract/escher-session-mcp/protocol-revisions area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: protocol revisions
promise: Speaks four MCP handshake revisions and answers its newest one to a client asking for any revision it does not speak.
@row_add n=107 row=contract/escher-session-mcp/tools area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: tools/list
promise: Lists the fourteen verbs of the verb table as tools in order, help last, each description the verb's help then ". Returns: " and its result fields.
@row_add n=108 row=contract/escher-session-mcp/screenshot-result area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: screenshot result
promise: An accepted screenshot answers an image block (base64 PNG, image/png) then a text block without png, isError false; a refused shot one text block, isError true.
@row_add n=109 row=contract/escher-session-mcp/result-bound area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: result bound
promise: A result holding an image is bounded at 1,048,576 bytes by its whole written form, a text-only one by its text.
@row_add n=110 row=contract/escher-session-mcp/end area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: end
promise: The server's end writes nothing on stdout: exit 0 on Ok, exit 3 with the error's fixed message on stderr on Err.
@row_add n=111 row=contract/escher-session-coverage/usage area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session coverage role
item: usage
promise: escher-session coverage <app> <check>..., no --session or --record; any other shape is usage, exit 2.
@row_add n=112 row=contract/escher-session-coverage/check-bound area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session coverage role
item: check file
promise: Each check is read bounded at 1,048,576 + 1 bytes; unreadable, oversized, not a check, or for another app is refused malformed, exit 1, holding nothing of the path or file.
@row_add n=113 row=contract/escher-session-coverage/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session coverage role
item: answer
promise: Each check replayed on a fresh boot; one uncoloured line {"unexercised":[{id,owner,event}],"checks","held"}, owner left out when absent; a failed boot is the session error, exit 3.
@row_add n=114 row=contract/escher-session-host/log-service-name area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session host log
item: service.name
promise: The host's stderr log lines carry service.name=seven_guis.
@row_add n=115 row=decision/session-recording-replayable-check area=surface-cli rec=fct-exqct2ck@4
name: Session recording as a replayable check
rule: --record <dir> keeps a session as one owner-only replayable check, per call its verb, arguments, acceptance, answer digest and masked answer, read back and replayed by the check replay.
why: Ratified.
@row_add n=116 row=contract/escher-session-record/check-file area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session recording
item: check-{n}.json
promise: Written 0600, never replacing a file, in a 0700 directory whose parent must exist, never removed by escher; a shot kept without its png.
@row_add n=117 row=contract/escher-session-record/record-dir-not-private area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session recording
item: record-dir-not-private
promise: A recording directory open to its group or others is this session error, exit 3, start spawning nothing.
@row_add n=118 row=contract/escher-session-record/recording-failed area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session recording
item: recording-failed
promise: A recording that ends answers this session error, exit 3; over MCP isError.
@row_none n=38 rec=fct-l463kzol@1 type=4
@row_none n=39 rec=fct-v4i42if5@1 type=4
@row_none n=40 rec=fct-gnuwsgqb@1 type=4
@row_none n=41 rec=fct-wduz3zi7@1 type=4
@row_none n=42 rec=fct-lhnpcvkp@1 type=4
@row_none n=43 rec=fct-lvky7wiz@1 type=4
@row_none n=44 rec=fct-2fi7fsuc@1 type=S
@row_none n=45 rec=fct-covlfp4w@1 type=4
@row_none n=46 rec=fct-rzhqzrl5@1 type=4
@row_none n=47 rec=fct-vorwvhmx@1 type=S
