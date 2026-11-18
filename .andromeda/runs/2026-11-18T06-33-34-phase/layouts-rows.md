# layout-templates · the rows that bind 2026-11-18-cold-agent-bar — picked 68 of 121 (29 by every draw) · rule any · pin 2041

## surface-desktop-native · ## Surface: desktop-native
decision/every-stand-control-keyed · Every stand control keyed — Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
decision/flight-type-select-boot-reading · Flight type select reads its two options at boot — The snapshot reads the flight booker's mode select at boot as ComboBox "Flight type" with value one-way and two ListBoxOptions, each at the select's client rect. · why: the_flight_type_select_holds_its_two_options in stand_snapshot pins role, name, value, both ListBoxOptions and their select-rect bounds in both layout modes
decision/open-list-rows-own-bounds · Open list rows read their own bounds — While a select list is open each row reads its own bounds in the snapshot and the diff, and after a close the rows read back at the select's client rect. · why: stand_act_popup checks a click opening the list names each row's bounds in the diff, and escape closing it puts the rows back at the select's box
contract/stand-element-ids/crud-create · Stand stable element ids · crud-create — Stable author element id of the CRUD task's Create button.
contract/stand-element-ids/crud-delete · Stand stable element ids · crud-delete — Stable author element id of the CRUD task's Delete button.
contract/stand-element-ids/crud-update · Stand stable element ids · crud-update — Stable author element id of the CRUD task's Update button.
contract/stand-element-ids/flight-book · Stand stable element ids · flight-book — Stable author element id of the flight booker's book button.
contract/stand-element-ids/flight-type · Stand stable element ids · flight-type — Stable author element id of the flight booker's mode select.
contract/stand-element-ids/flight-type-one-way · Stand stable element ids · flight-type-one-way — Stable author element id of the flight booker's one-way mode option.
contract/stand-element-ids/flight-type-return · Stand stable element ids · flight-type-return — Stable author element id of the flight booker's return mode option.
decision/crud-list-keyboard-region · CRUD list as one keyboard region — The CRUD list is one Tab stop whose ArrowDown, ArrowUp, Home and End move the selected row and fill the name fields as a click does; focus stays on the list, never a row, and no aria-activedescendant is read. · why: Ratified as a decision.
decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
decision/held-instance-reads-as-fresh-boot · Held instance reads as a fresh boot — A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes. · guard: stand_session_fresh
decision/load-page-only-network · Load page alone has a network provider — Only the load page's boot has a network provider, an in-process stand-in answering after 200 ms from a thread of its own, so a driver click on load-fetch waits it out; every other boot keeps no network provider. · guard: stand_load_page
decision/stand-a11y-by-attributes-alone · Stand names, roles and states by attributes alone — Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
contract/stand-element-ids/cell-column-row · Stand stable element ids · cell-{A..Z}{0..25} — Stable author element id of each of the Cells task's 676 data cells, its column letter then its row number.
contract/stand-element-ids/cell-input · Stand stable element ids · cell-input — Stable author element id of the Cells task's one edit input.
contract/stand-element-ids/cells-grid · Stand stable element ids · cells-grid — Stable author element id of the Cells grid, one Tab stop and keyboard region named "Cells", with no role.
contract/stand-element-ids/counter-increment · Stand stable element ids · counter-increment — Stable author element id of the Counter task's increment button.
contract/stand-element-ids/counter-value · Stand stable element ids · counter-value — Stable author element id of the Counter task's value display.
contract/stand-element-ids/crud-filter · Stand stable element ids · crud-filter — Stable author element id of the CRUD task's filter input, named by its label's for.
contract/stand-element-ids/crud-list · Stand stable element ids · crud-list — Stable author element id of the CRUD task's people list, a listbox named "People".
contract/stand-element-ids/crud-name · Stand stable element ids · crud-name — Stable author element id of the CRUD task's name input, named by its label's for.
contract/stand-element-ids/crud-person-id · Stand stable element ids · crud-person-{person.id} — Each CRUD row's stable author element id, from its person's model-assigned id beside the same Dioxus key, following its person under filter, Create and Delete.
contract/stand-element-ids/crud-surname · Stand stable element ids · crud-surname — Stable author element id of the CRUD task's surname input, named by its label's for.
contract/stand-element-ids/flight-booked · Stand stable element ids · flight-booked — Stable author element id of the flight booker's booked display.
contract/stand-element-ids/flight-return-date · Stand stable element ids · flight-return-date — Stable author element id of the flight booker's return date input, named "Return date" by aria-label.
contract/stand-element-ids/flight-start · Stand stable element ids · flight-start — Stable author element id of the flight booker's departure date input, named "Departure date" by aria-label.
contract/stand-element-ids/load-done · Stand stable element ids · load-done — Stable author element id of the load page's "Loaded" paragraph, hidden until the stylesheet the button's click links lands.
contract/stand-element-ids/load-fetch · Stand stable element ids · load-fetch — Stable author element id of the load page's Load button, taking Enter and Space.
contract/stand-element-ids/timer-duration · Stand stable element ids · timer-duration — Stable author element id of the Timer task's duration input, named by its label's for.
contract/stand-element-ids/timer-duration-value · Stand stable element ids · timer-duration-value — Stable author element id of the Timer task's duration value display.
contract/stand-element-ids/timer-elapsed · Stand stable element ids · timer-elapsed — Stable author element id of the Timer task's elapsed display.
contract/stand-element-ids/timer-progress · Stand stable element ids · timer-progress — Stable author element id of the Timer task's progress display.
contract/stand-element-ids/timer-reset · Stand stable element ids · timer-reset — Stable author element id of the Timer task's reset button.

## surface-cli · ## Surface: cli
decision/cold-agent-pipe-builds-nothing · Cold-agent pipe builds nothing — The cold-agent pipe builds nothing; a missing escher-session binary is a failed precondition.
decision/cold-agent-prompt-names-no-id · Cold-agent prompt names no id — The cold-agent stand run uses one fixed prompt that names no element id or app slug.
decision/cold-agent-uses-driver-mcp · Cold agent drives through the driver's MCP server — The cold-agent client is given the driver's own MCP server, escher-session mcp, spawned through a per-run MCP configuration.
decision/mcp-server-local-stdio · MCP server local over stdio — The escher-session mcp role is an MCP server over stdio, local to the invoking user, with no listener or auth surface.
decision/no-caller-input-echoed · Caller input never written back — No escher-session command's ending writes back anything the caller typed, a recording path included. · except: The layout answer's id field names the element the call's id named.
contract/cold-agent-cli/stdout · cold-agent.sh run pipe · stdout — Every verb's stdout is JSON lines only, one object per event; the session transcript and the agent's check stay in files under target/cold-agent, never printed.
contract/cold-agent-cli/usage · cold-agent.sh run pipe · usage — cold-agent.sh <verb> [task], run from the repository root, with verbs run, status, cleanup, logs and the one task stand: six apps in one client session under a fixed prompt.
contract/cold-agent-cli/windows-pass-through · cold-agent.sh run pipe · cold-agent.ps1 — The Windows pass-through to cold-agent.sh.
contract/escher-session-actions/answer · escher-session actions · answer — No argument; one unpaged uncoloured line {"nodes"} of id, owner, keyed and verbs with verb and cause, owner and cause left out when absent, never a value, typed text or NodeId.
contract/escher-session-graph/answer · escher-session graph · answer — No argument; one uncoloured line {"nodes","states"} carrying ids, component names, handler event names, state names, closed flag words and booleans only, never a value.
contract/escher-session-host/log-service-name · escher-session host log · service.name — The host's stderr log lines carry service.name=seven_guis.
contract/escher-session-layout/answer · escher-session layout · answer — One required --id; one uncoloured line of id, bounds and laid_out (x, y, width, height), placed (by, container, canvas, at, relative), shifts (by, holder, dx, dy), width and height (px, by, size, min, max, flex with basis, grow, shrink); ids, closed words and numbers only.
contract/escher-session-mcp/tools · escher-session MCP server · tools/list — Lists the fifteen verbs of the verb table as tools in order, layout after actions and help last, each description the verb's help then ". Returns: " and its result fields.
contract/escher-session-served-tasks/cells · escher-session served tasks · cells — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/counter · escher-session served tasks · counter — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/crud · escher-session served tasks · crud — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/flight-booker · escher-session served tasks · flight-booker — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/load · escher-session served tasks · load — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/timer · escher-session served tasks · timer — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/cold-agent-cli/exit-2 · cold-agent.sh run pipe · exit 2 — A usage error prints to stderr with empty stdout and exits 2, checked before any precondition.
contract/cold-agent-cli/exit-3 · cold-agent.sh run pipe · exit 3 — run without claude, python3 or a built, executable target/debug/escher-session exits 3 with one stderr line naming the cargo build command to run.
contract/cold-agent-cli/record-dir · cold-agent.sh run pipe · target/cold-agent/record — The live run's server records its kept checks into this owner-only directory, removed by cleanup with the state, the checks never printed; replays run without recording.
contract/escher-session-coverage/answer · escher-session coverage role · answer — Each check replayed on a fresh boot; one uncoloured line {"unexercised":[{id,owner,event}],"checks","held"}, owner left out when absent; a failed boot is the session error, exit 3.
contract/escher-session-coverage/check-bound · escher-session coverage role · check file — Each check is read bounded at 1,048,576 + 1 bytes; unreadable, oversized, not a check, or for another app is refused malformed, exit 1, holding nothing of the path or file.
contract/escher-session-coverage/usage · escher-session coverage role · usage — escher-session coverage <app> <check>..., no --session or --record; any other shape is usage, exit 2.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: the answer as one line of uncoloured JSON on stdout.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: the refusal's JSON on stdout.
contract/escher-session-help/answer · escher-session help · answer — One uncoloured JSON line with seven keys (verbs, node_keys, key_names, busy_classes, causes, exit_statuses, session_errors), fixed schema strings, reaching no session.
contract/escher-session-mcp/screenshot-result · escher-session MCP server · screenshot result — An accepted screenshot answers an image block (base64 PNG, image/png) then a text block without png, isError false; a refused shot one text block, isError true.
contract/escher-session-mcp/stdio · escher-session MCP server · stdio — Reads newline-delimited JSON-RPC 2.0 requests on stdin and writes protocol lines only on stdout, uncoloured with no ESC byte, echoing a request's admitted id.
contract/escher-session-refusals/fonts-unavailable · escher-session refusals · fonts-unavailable — screenshot or lint on an app with no font it can paint text with is refused fonts-unavailable, exit 1.
contract/escher-session-refusals/off-screen · escher-session refusals · off-screen — An element screenshot that crops to no pixel is refused off-screen, exit 1.
contract/escher-session-refusals/unknown-verb · escher-session refusals · unknown-verb — A first word the verb table does not hold is refused unknown-verb, exit 1.

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
