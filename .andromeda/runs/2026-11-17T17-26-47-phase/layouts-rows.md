# layout-templates · the rows that bind 2026-11-17-answer-writer-by-family — picked 42 of 121 (25 by every draw) · rule any · pin 1805

## surface-desktop-native · ## Surface: desktop-native
decision/fixed-document-skeleton-main-mount · Fixed document skeleton with main mount — Every document starts with the fixed skeleton html, head, body and main id="main", and the app mounts into main. · why: stand_snapshot's tree_follows_the_task_shell reads the single /html:0 root with nothing under head, and the bridge's own crash test counts the app's children under main
decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
contract/stand-element-ids/counter-increment · Stand stable element ids · counter-increment — Stable author element id of the Counter task's increment button.
contract/stand-element-ids/counter-value · Stand stable element ids · counter-value — Stable author element id of the Counter task's value display.

## surface-cli · ## Surface: cli
decision/mcp-server-local-stdio · MCP server local over stdio — The escher-session mcp role is an MCP server over stdio, local to the invoking user, with no listener or auth surface.
decision/session-recording-replayable-check · Session recording as a replayable check — --record <dir> keeps a session as one owner-only replayable check, per call its verb, arguments, acceptance, answer digest and masked answer, read back and replayed by the check replay. · why: Ratified.
contract/escher-session-actions/answer · escher-session actions · answer — No argument; one unpaged uncoloured line {"nodes"} of id, owner, keyed and verbs with verb and cause, owner and cause left out when absent, never a value, typed text or NodeId.
contract/escher-session-coverage/answer · escher-session coverage role · answer — Each check replayed on a fresh boot; one uncoloured line {"unexercised":[{id,owner,event}],"checks","held"}, owner left out when absent; a failed boot is the session error, exit 3.
contract/escher-session-coverage/check-bound · escher-session coverage role · check file — Each check is read bounded at 1,048,576 + 1 bytes; unreadable, oversized, not a check, or for another app is refused malformed, exit 1, holding nothing of the path or file.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: the answer as one line of uncoloured JSON on stdout.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: the refusal's JSON on stdout.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — Session error: the error object with its fixed remedy on stdout and its fixed message on stderr.
contract/escher-session-graph/answer · escher-session graph · answer — No argument; one uncoloured line {"nodes","states"} carrying ids, component names, handler event names, state names, closed flag words and booleans only, never a value.
contract/escher-session-help/answer · escher-session help · answer — One uncoloured JSON line with seven keys (verbs, node_keys, key_names, busy_classes, causes, exit_statuses, session_errors), fixed schema strings, reaching no session.
contract/escher-session-layout/answer · escher-session layout · answer — One required --id; one uncoloured line of id, bounds and laid_out (x, y, width, height), placed (by, container, canvas, at, relative), shifts (by, holder, dx, dy), width and height (px, by, size, min, max, flex with basis, grow, shrink); ids, closed words and numbers only.
contract/escher-session-lint/answer · escher-session lint · answer — No argument; one uncoloured line {"findings"}, each finding keyed id, defect, bounds then its reason's keys, carrying ids, closed words and numbers only.
contract/escher-session-mcp/end · escher-session MCP server · end — The server's end writes nothing on stdout: exit 0 on Ok, exit 3 with the error's fixed message on stderr on Err.
contract/escher-session-mcp/result-bound · escher-session MCP server · result bound — A result holding an image is bounded at 1,048,576 bytes by its whole written form, a text-only one by its text.
contract/escher-session-mcp/screenshot-result · escher-session MCP server · screenshot result — An accepted screenshot answers an image block (base64 PNG, image/png) then a text block without png, isError false; a refused shot one text block, isError true.
contract/escher-session-mcp/tools · escher-session MCP server · tools/list — Lists the fifteen verbs of the verb table as tools in order, layout after actions and help last, each description the verb's help then ". Returns: " and its result fields.
contract/escher-session-record/check-file · escher-session recording · check-{n}.json — Written 0600, never replacing a file, in a 0700 directory whose parent must exist, removed by escher only when its recording fails; a shot kept without its png.
contract/escher-session-record/record-dir-not-private · escher-session recording · record-dir-not-private — A recording directory open to its group or others is this session error, exit 3, start spawning nothing.
contract/escher-session-record/recording-failed · escher-session recording · recording-failed — A recording that ends answers this session error, exit 3; over MCP isError.
contract/escher-session-refusals/fonts-unavailable · escher-session refusals · fonts-unavailable — screenshot or lint on an app with no font it can paint text with is refused fonts-unavailable, exit 1.
contract/escher-session-refusals/off-screen · escher-session refusals · off-screen — An element screenshot that crops to no pixel is refused off-screen, exit 1.
contract/escher-session-refusals/unknown-verb · escher-session refusals · unknown-verb — A first word the verb table does not hold is refused unknown-verb, exit 1.
contract/escher-session-screenshot/answer · escher-session screenshot · answer — One uncoloured line {"width","height","png"} with base64 PNG; with --id the named element cropped to its visible bounds, without it the whole screen.
contract/escher-session-snapshot/answer · escher-session snapshot · answer — One uncoloured line: {"text"} for a whole screen, {"text","part","parts"} for a screen read in parts; a part past the last gives empty text with the same parts.
contract/escher-session-snapshot/part · escher-session snapshot · --part — Optional whole number 1 to 1000, part 1 without it; outside the bound refused malformed, exit 1.
decision/no-caller-input-echoed · Caller input never written back — No escher-session command's ending writes back anything the caller typed, a recording path included. · except: The layout answer's id field names the element the call's id named.
contract/escher-session-command/form · escher-session command line · command form — Every command is escher-session <verb> [arguments] --session <dir> over the fifteen verbs of the driver's verb table, --session exactly once on every command but help.
contract/escher-session-coverage/usage · escher-session coverage role · usage — escher-session coverage <app> <check>..., no --session or --record; any other shape is usage, exit 2.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from the verb table and the task names, stdout empty, nothing booted and no state created.
contract/escher-session-host/log-service-name · escher-session host log · service.name — The host's stderr log lines carry service.name=seven_guis.
contract/escher-session-mcp/protocol-revisions · escher-session MCP server · protocol revisions — Speaks four MCP handshake revisions and answers its newest one to a client asking for any revision it does not speak.
contract/escher-session-mcp/stdio · escher-session MCP server · stdio — Reads newline-delimited JSON-RPC 2.0 requests on stdin and writes protocol lines only on stdout, uncoloured with no ESC byte, echoing a request's admitted id.
contract/escher-session-served-tasks/cells · escher-session served tasks · cells — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/counter · escher-session served tasks · counter — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/crud · escher-session served tasks · crud — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/flight-booker · escher-session served tasks · flight-booker — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/load · escher-session served tasks · load — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
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
