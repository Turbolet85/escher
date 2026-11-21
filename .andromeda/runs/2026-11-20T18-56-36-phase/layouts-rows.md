# layout-templates · the rows that bind 2026-11-20-driver-debts — picked 29 of 129 (22 by every draw) · rule any · pin 2748

## surface-desktop-native · ## Surface: desktop-native
decision/crud-list-keyboard-region · CRUD list as one keyboard region — The CRUD list is one Tab stop whose ArrowDown, ArrowUp, Home and End move the selected row and fill the name fields as a click does; focus stays on the list, never a row, and no aria-activedescendant is read. · why: Ratified as a decision.
decision/every-stand-control-keyed · Every stand control keyed — Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
decision/flight-type-select-boot-reading · Flight type select reads its two options at boot — The snapshot reads the flight booker's mode select at boot as ComboBox "Flight type" with value one-way and two ListBoxOptions, each at the select's client rect. · why: the_flight_type_select_holds_its_two_options in stand_snapshot pins role, name, value, both ListBoxOptions and their select-rect bounds in both layout modes
decision/stand-a11y-by-attributes-alone · Stand names, roles and states by attributes alone — Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
decision/stand-ids-as-attributes-alone · Stand ids as attributes alone — Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them.
contract/stand-element-ids/crud-list · Stand stable element ids · crud-list — Stable author element id of the CRUD task's people list, a listbox named "People".
contract/stand-element-ids/crud-person-id · Stand stable element ids · crud-person-{person.id} — Each CRUD row's stable author element id, from its person's model-assigned id beside the same Dioxus key, following its person under filter, Create and Delete.
contract/stand-element-ids/flight-type · Stand stable element ids · flight-type — Stable author element id of the flight booker's mode select.
contract/stand-element-ids/flight-type-one-way · Stand stable element ids · flight-type-one-way — Stable author element id of the flight booker's one-way mode option.
contract/stand-element-ids/flight-type-return · Stand stable element ids · flight-type-return — Stable author element id of the flight booker's return mode option.
contract/stand-element-ids/crud-name · Stand stable element ids · crud-name — Stable author element id of the CRUD task's name input, named by its label's for.
contract/stand-element-ids/crud-surname · Stand stable element ids · crud-surname — Stable author element id of the CRUD task's surname input, named by its label's for.

## surface-cli · ## Surface: cli
decision/cold-agent-uses-driver-mcp · Cold agent drives through the driver's MCP server — The cold-agent client is given the driver's own MCP server, escher-session mcp, spawned through a per-run MCP configuration.
decision/no-caller-input-echoed · Caller input never written back — No escher-session command's ending writes back anything the caller typed, a recording path included. · except: The layout answer's id field names the element the call's id named.
decision/session-recording-replayable-check · Session recording as a replayable check — --record <dir> keeps a session as one owner-only replayable check, per call its verb, arguments, acceptance, answer digest and masked answer, read back and replayed by the check replay. · why: Ratified.
contract/escher-session-command/form · escher-session command line · command form — Every command is escher-session <verb> [arguments] --session <dir> over the fifteen verbs of the driver's verb table, --session exactly once on every command but help.
contract/escher-session-coverage/answer · escher-session coverage role · answer — Each check replayed on a fresh boot; one uncoloured line {"unexercised":[{id,owner,event}],"checks","held"}, owner left out when absent; a failed boot is the session error, exit 3.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: the refusal's JSON on stdout.
contract/escher-session-graph/answer · escher-session graph · answer — No argument; one uncoloured line {"nodes","states"} carrying ids, component names, handler event names, state names, closed flag words and booleans only, never a value.
contract/escher-session-help/answer · escher-session help · answer — One uncoloured JSON line with seven keys (verbs, node_keys, key_names, busy_classes, causes, exit_statuses, session_errors), fixed schema strings, reaching no session.
contract/escher-session-mcp/result-bound · escher-session MCP server · result bound — A result holding an image is bounded at 1,048,576 bytes by its whole written form, a text-only one by its text.
contract/escher-session-mcp/tools · escher-session MCP server · tools/list — Lists the fifteen verbs of the verb table as tools in order, layout after actions and help last, each description the verb's help then ". Returns: " and its result fields.
contract/escher-session-snapshot/answer · escher-session snapshot · answer — One uncoloured line: {"text"} for a whole screen, {"text","part","parts"} for a screen read in parts; a part past the last gives empty text with the same parts.
contract/escher-session-snapshot/part · escher-session snapshot · --part — Optional whole number 1 to 1000, part 1 without it; outside the bound refused malformed, exit 1.
decision/mcp-server-local-stdio · MCP server local over stdio — The escher-session mcp role is an MCP server over stdio, local to the invoking user, with no listener or auth surface.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: the answer as one line of uncoloured JSON on stdout.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from the verb table and the task names, stdout empty, nothing booted and no state created.
contract/escher-session-served-tasks/crud · escher-session served tasks · crud — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/flight-booker · escher-session served tasks · flight-booker — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.

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
