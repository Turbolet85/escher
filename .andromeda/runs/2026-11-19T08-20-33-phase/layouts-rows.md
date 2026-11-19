# layout-templates · the rows that bind 2026-11-19-build-directory-held — picked 63 of 129 (10 by every draw) · rule any · pin 2275

## surface-desktop-native · ## Surface: desktop-native
decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
decision/held-instance-reads-as-fresh-boot · Held instance reads as a fresh boot — A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes. · guard: stand_session_fresh
decision/every-stand-control-keyed · Every stand control keyed — Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
decision/flight-type-select-boot-reading · Flight type select reads its two options at boot — The snapshot reads the flight booker's mode select at boot as ComboBox "Flight type" with value one-way and two ListBoxOptions, each at the select's client rect. · why: the_flight_type_select_holds_its_two_options in stand_snapshot pins role, name, value, both ListBoxOptions and their select-rect bounds in both layout modes
decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD, the load page and the order page read zero visual-lint findings, in both layout modes. · guard: stand_lint
contract/stand-element-ids/cell-column-row · Stand stable element ids · cell-{A..Z}{0..25} — Stable author element id of each of the Cells task's 676 data cells, its column letter then its row number.
contract/stand-element-ids/cell-input · Stand stable element ids · cell-input — Stable author element id of the Cells task's one edit input.
contract/stand-element-ids/cells-grid · Stand stable element ids · cells-grid — Stable author element id of the Cells grid, one Tab stop and keyboard region named "Cells", with no role.
contract/stand-element-ids/circle-canvas · Stand stable element ids · circle-canvas — Stable author element id of the Circle Drawer's canvas.
contract/stand-element-ids/circle-close · Stand stable element ids · circle-close — Stable author element id of the Circle Drawer dialog's close control, present only while the dialog is open.
contract/stand-element-ids/circle-diameter · Stand stable element ids · circle-diameter — Stable author element id of the Circle Drawer dialog's range input, present only while the dialog is open.
contract/stand-element-ids/circle-redo · Stand stable element ids · circle-redo — Stable author element id of the Circle Drawer's redo control.
contract/stand-element-ids/circle-undo · Stand stable element ids · circle-undo — Stable author element id of the Circle Drawer's undo control.
contract/stand-element-ids/counter-increment · Stand stable element ids · counter-increment — Stable author element id of the Counter task's increment button.
contract/stand-element-ids/counter-value · Stand stable element ids · counter-value — Stable author element id of the Counter task's value display.
contract/stand-element-ids/crud-create · Stand stable element ids · crud-create — Stable author element id of the CRUD task's Create button.
contract/stand-element-ids/crud-delete · Stand stable element ids · crud-delete — Stable author element id of the CRUD task's Delete button.
contract/stand-element-ids/crud-filter · Stand stable element ids · crud-filter — Stable author element id of the CRUD task's filter input, named by its label's for.
contract/stand-element-ids/crud-list · Stand stable element ids · crud-list — Stable author element id of the CRUD task's people list, a listbox named "People".
contract/stand-element-ids/crud-name · Stand stable element ids · crud-name — Stable author element id of the CRUD task's name input, named by its label's for.
contract/stand-element-ids/crud-person-id · Stand stable element ids · crud-person-{person.id} — Each CRUD row's stable author element id, from its person's model-assigned id beside the same Dioxus key, following its person under filter, Create and Delete.
contract/stand-element-ids/crud-surname · Stand stable element ids · crud-surname — Stable author element id of the CRUD task's surname input, named by its label's for.
contract/stand-element-ids/crud-update · Stand stable element ids · crud-update — Stable author element id of the CRUD task's Update button.
contract/stand-element-ids/flight-book · Stand stable element ids · flight-book — Stable author element id of the flight booker's book button.
contract/stand-element-ids/flight-booked · Stand stable element ids · flight-booked — Stable author element id of the flight booker's booked display.
contract/stand-element-ids/flight-return-date · Stand stable element ids · flight-return-date — Stable author element id of the flight booker's return date input, named "Return date" by aria-label.
contract/stand-element-ids/flight-start · Stand stable element ids · flight-start — Stable author element id of the flight booker's departure date input, named "Departure date" by aria-label.
contract/stand-element-ids/flight-type · Stand stable element ids · flight-type — Stable author element id of the flight booker's mode select.
contract/stand-element-ids/flight-type-one-way · Stand stable element ids · flight-type-one-way — Stable author element id of the flight booker's one-way mode option.
contract/stand-element-ids/flight-type-return · Stand stable element ids · flight-type-return — Stable author element id of the flight booker's return mode option.
contract/stand-element-ids/load-done · Stand stable element ids · load-done — Stable author element id of the load page's "Loaded" paragraph, hidden until the stylesheet the button's click links lands.
contract/stand-element-ids/load-fetch · Stand stable element ids · load-fetch — Stable author element id of the load page's Load button, taking Enter and Space.
contract/stand-element-ids/order-export · Stand stable element ids · order-export — Stable author element id of the order page's Export button, which panics.
contract/stand-element-ids/order-failed · Stand stable element ids · order-failed — Stable author element id of the order page's Order failed message.
contract/stand-element-ids/order-quantity · Stand stable element ids · order-quantity — Stable author element id of the order page's quantity input.
contract/stand-element-ids/order-retry · Stand stable element ids · order-retry — Stable author element id of the order page's Retry button, present only once Submit fails.
contract/stand-element-ids/order-status · Stand stable element ids · order-status — Stable author element id of the order page's status, reading Order placed after a good Submit.
contract/stand-element-ids/order-submit · Stand stable element ids · order-submit — Stable author element id of the order page's Submit button.
contract/stand-element-ids/task-card-cells · Stand stable element ids · task-card-cells — Stable author element id of Home's Cells task-card button.
contract/stand-element-ids/task-card-circle-drawer · Stand stable element ids · task-card-circle-drawer — Stable author element id of Home's Circle Drawer task-card button.
contract/stand-element-ids/task-card-counter · Stand stable element ids · task-card-counter — Stable author element id of Home's Counter task-card button.
contract/stand-element-ids/task-card-crud · Stand stable element ids · task-card-crud — Stable author element id of Home's CRUD task-card button.
contract/stand-element-ids/task-card-flight-booker · Stand stable element ids · task-card-flight-booker — Stable author element id of Home's Flight Booker task-card button.
contract/stand-element-ids/task-card-temp-converter · Stand stable element ids · task-card-temp-converter — Stable author element id of Home's Temperature Converter task-card button.
contract/stand-element-ids/task-card-timer · Stand stable element ids · task-card-timer — Stable author element id of Home's Timer task-card button.
contract/stand-element-ids/temp-celsius · Stand stable element ids · temp-celsius — Stable author element id of the Temperature Converter's Celsius control.
contract/stand-element-ids/temp-fahrenheit · Stand stable element ids · temp-fahrenheit — Stable author element id of the Temperature Converter's Fahrenheit control.
contract/stand-element-ids/timer-duration · Stand stable element ids · timer-duration — Stable author element id of the Timer task's duration input, named by its label's for.
contract/stand-element-ids/timer-duration-value · Stand stable element ids · timer-duration-value — Stable author element id of the Timer task's duration value display.
contract/stand-element-ids/timer-elapsed · Stand stable element ids · timer-elapsed — Stable author element id of the Timer task's elapsed display.
contract/stand-element-ids/timer-progress · Stand stable element ids · timer-progress — Stable author element id of the Timer task's progress display.
contract/stand-element-ids/timer-reset · Stand stable element ids · timer-reset — Stable author element id of the Timer task's reset button.

## surface-cli · ## Surface: cli
decision/cold-agent-pipe-builds-nothing · Cold-agent pipe builds nothing — The cold-agent pipe builds nothing; a missing escher-session binary is a failed precondition.
contract/agent-run-cli/exit-2 · agent-run.sh test contract · exit 2 — A usage error prints the usage to stderr and exits 2.
contract/agent-run-cli/run-log · agent-run.sh test contract · target/agent-run/run.log — The raw cargo output stays in this file, never on stdout.
contract/agent-run-cli/stdout · agent-run.sh test contract · stdout — Every verb's stdout is JSON lines only, one object per event.
contract/agent-run-cli/usage · agent-run.sh test contract · usage — agent-run.sh <verb> [selection], run from the repository root, with verbs boot, run, status, cleanup, logs and run selections stand, all, session or a blitz-tests file name; session is a closed word matched before a file name and runs the escher-session binary's flows in seven_guis.
contract/agent-run-cli/windows-pass-through · agent-run.sh test contract · agent-run.ps1 — The Windows pass-through to agent-run.sh.
contract/cold-agent-cli/exit-3 · cold-agent.sh run pipe · exit 3 — run without claude, python3 or a built, executable target/debug/escher-session exits 3 with one stderr line naming the cargo build command to run.
contract/cold-agent-cli/record-dir · cold-agent.sh run pipe · target/cold-agent/record — The live run's server records its kept checks into this owner-only directory, removed by cleanup with the state, the checks never printed; replays run without recording.
decision/session-recording-replayable-check · Session recording as a replayable check — --record <dir> keeps a session as one owner-only replayable check, per call its verb, arguments, acceptance, answer digest and masked answer, read back and replayed by the check replay. · why: Ratified.
contract/cold-agent-cli/usage · cold-agent.sh run pipe · usage — cold-agent.sh <verb> [task], run from the repository root, with verbs run, status, cleanup, logs and the one task stand: seven apps in one client session under a fixed prompt.
contract/escher-session-record/check-file · escher-session recording · check-{n}.json — Written 0600, never replacing a file, in a 0700 directory whose parent must exist, removed by escher only when its recording fails; a shot kept without its png.

# layout-templates · its absences, names and overviews, whole

## surface-desktop-native · ## Surface: desktop-native
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement or hero/signature section; slice s01's reading did not reach the browser app's window/page layout code (slice s04's facts below cover it).
absence/arbitrary-index-html-templates — Arbitrary index.html templates for a document

## surface-mobile-native · ## Surface: mobile-native
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement, primary screens beyond the browser frame, or hero, content and footer components; slice s01's reading did not reach mobile layout code.

## surface-cli · ## Surface: cli
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level or signature placement; slice s03's reading did not reach any further CLI help or argument parser beyond the `paint_bench` usage line.

## surface-web-spa · ## Surface: web-spa
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement, navigation, header, footer or IA.

## decisions-log · ## Decisions Log
> NO RECORDED INTENT
