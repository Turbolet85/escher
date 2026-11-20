# layout-templates · the rows that bind 2026-11-20-codebase-health — picked 89 of 129 (19 by every draw) · rule any · pin 2532

## surface-desktop-native · ## Surface: desktop-native
decision/every-stand-control-keyed · Every stand control keyed — Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
decision/session-roles-share-stand-boot · Session roles share the stand boot — Every escher-session role that boots a task (host, mcp, coverage) boots it through the same stand boot the command line uses, adding no markup, id, class, style, wrapper or order.
decision/stand-a11y-by-attributes-alone · Stand names, roles and states by attributes alone — Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
decision/held-instance-reads-as-fresh-boot · Held instance reads as a fresh boot — A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes. · guard: stand_session_fresh
decision/stand-ids-as-attributes-alone · Stand ids as attributes alone — Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them.
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
decision/mcp-server-local-stdio · MCP server local over stdio — The escher-session mcp role is an MCP server over stdio, local to the invoking user, with no listener or auth surface.
decision/no-caller-input-echoed · Caller input never written back — No escher-session command's ending writes back anything the caller typed, a recording path included. · except: The layout answer's id field names the element the call's id named.
contract/escher-session-actions/answer · escher-session actions · answer — No argument; one unpaged uncoloured line {"nodes"} of id, owner, keyed and verbs with verb and cause, owner and cause left out when absent, never a value, typed text or NodeId.
contract/escher-session-command/form · escher-session command line · command form — Every command is escher-session <verb> [arguments] --session <dir> over the fifteen verbs of the driver's verb table, --session exactly once on every command but help.
contract/escher-session-coverage/usage · escher-session coverage role · usage — escher-session coverage <app> <check>..., no --session or --record; any other shape is usage, exit 2.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from the verb table and the task names, stdout empty, nothing booted and no state created.
contract/escher-session-graph/answer · escher-session graph · answer — No argument; one uncoloured line {"nodes","states"} carrying ids, component names, handler event names, state names, closed flag words and booleans only, never a value.
contract/escher-session-help/answer · escher-session help · answer — One uncoloured JSON line with seven keys (verbs, node_keys, key_names, busy_classes, causes, exit_statuses, session_errors), fixed schema strings, reaching no session.
contract/escher-session-layout/answer · escher-session layout · answer — One required --id; one uncoloured line of id, bounds and laid_out (x, y, width, height), placed (by, container, canvas, at, relative), shifts (by, holder, dx, dy), width and height (px, by, size, min, max, flex with basis, grow, shrink); ids, closed words and numbers only.
contract/escher-session-lint/answer · escher-session lint · answer — No argument; one uncoloured line {"findings"}, each finding keyed id, defect, bounds then its reason's keys, carrying ids, closed words and numbers only.
contract/escher-session-mcp/end · escher-session MCP server · end — The server's end writes nothing on stdout: exit 0 on Ok, exit 3 with the error's fixed message on stderr on Err.
contract/escher-session-mcp/stdio · escher-session MCP server · stdio — Reads newline-delimited JSON-RPC 2.0 requests on stdin and writes protocol lines only on stdout, uncoloured with no ESC byte, echoing a request's admitted id.
contract/escher-session-refusals/unknown-verb · escher-session refusals · unknown-verb — A first word the verb table does not hold is refused unknown-verb, exit 1.
contract/escher-session-screenshot/answer · escher-session screenshot · answer — One uncoloured line {"width","height","png"} with base64 PNG; with --id the named element cropped to its visible bounds, without it the whole screen.
contract/escher-session-snapshot/answer · escher-session snapshot · answer — One uncoloured line: {"text"} for a whole screen, {"text","part","parts"} for a screen read in parts; a part past the last gives empty text with the same parts.
contract/escher-session-snapshot/part · escher-session snapshot · --part — Optional whole number 1 to 1000, part 1 without it; outside the bound refused malformed, exit 1.
decision/session-recording-replayable-check · Session recording as a replayable check — --record <dir> keeps a session as one owner-only replayable check, per call its verb, arguments, acceptance, answer digest and masked answer, read back and replayed by the check replay. · why: Ratified.
contract/escher-session-coverage/answer · escher-session coverage role · answer — Each check replayed on a fresh boot; one uncoloured line {"unexercised":[{id,owner,event}],"checks","held"}, owner left out when absent; a failed boot is the session error, exit 3.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: the answer as one line of uncoloured JSON on stdout.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: the refusal's JSON on stdout.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — Session error: the error object with its fixed remedy on stdout and its fixed message on stderr.
contract/escher-session-host/log-service-name · escher-session host log · service.name — The host's stderr log lines carry service.name=seven_guis.
contract/escher-session-mcp/protocol-revisions · escher-session MCP server · protocol revisions — Speaks four MCP handshake revisions and answers its newest one to a client asking for any revision it does not speak.
contract/escher-session-mcp/result-bound · escher-session MCP server · result bound — A result holding an image is bounded at 1,048,576 bytes by its whole written form, a text-only one by its text.
contract/escher-session-mcp/screenshot-result · escher-session MCP server · screenshot result — An accepted screenshot answers an image block (base64 PNG, image/png) then a text block without png, isError false; a refused shot one text block, isError true.
contract/escher-session-mcp/tools · escher-session MCP server · tools/list — Lists the fifteen verbs of the verb table as tools in order, layout after actions and help last, each description the verb's help then ". Returns: " and its result fields.
contract/escher-session-record/record-dir-not-private · escher-session recording · record-dir-not-private — A recording directory open to its group or others is this session error, exit 3, start spawning nothing.
contract/escher-session-record/recording-failed · escher-session recording · recording-failed — A recording that ends answers this session error, exit 3; over MCP isError.
contract/escher-session-refusals/fonts-unavailable · escher-session refusals · fonts-unavailable — screenshot or lint on an app with no font it can paint text with is refused fonts-unavailable, exit 1.
contract/escher-session-refusals/off-screen · escher-session refusals · off-screen — An element screenshot that crops to no pixel is refused off-screen, exit 1.
contract/escher-session-served-tasks/cells · escher-session served tasks · cells — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/counter · escher-session served tasks · counter — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/crud · escher-session served tasks · crud — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/flight-booker · escher-session served tasks · flight-booker — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/load · escher-session served tasks · load — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/order · escher-session served tasks · order — The order page; a task start, the host role and the MCP start tool boot, listed after load in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/timer · escher-session served tasks · timer — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.

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
