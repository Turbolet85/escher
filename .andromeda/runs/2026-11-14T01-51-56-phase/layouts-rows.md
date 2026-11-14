# layout-templates · the rows that bind 2026-11-14-seeded-randomness — picked 26 of 120 (17 by every draw) · rule any · pin 741

## surface-desktop-native · ## Surface: desktop-native
decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
decision/held-instance-reads-as-fresh-boot · Held instance reads as a fresh boot — A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes. · guard: stand_session_fresh
decision/session-roles-share-stand-boot · Session roles share the stand boot — Every escher-session role that boots a task (host, mcp, coverage) boots it through the same stand boot the command line uses, adding no markup, id, class, style, wrapper or order.
decision/fixed-document-skeleton-main-mount · Fixed document skeleton with main mount — Every document starts with the fixed skeleton html, head, body and main id="main", and the app mounts into main. · why: stand_snapshot's tree_follows_the_task_shell reads the single /html:0 root with nothing under head, and the bridge's own crash test counts the app's children under main

## surface-cli · ## Surface: cli
decision/mcp-server-local-stdio · MCP server local over stdio — The escher-session mcp role is an MCP server over stdio, local to the invoking user, with no listener or auth surface.
decision/no-caller-input-echoed · Caller input never written back — No escher-session command's ending writes back anything the caller typed, a recording path included.
decision/session-recording-replayable-check · Session recording as a replayable check — --record <dir> keeps a session as one owner-only replayable check, per call its verb, arguments, acceptance, answer digest and masked answer, read back and replayed by the check replay. · why: Ratified.
contract/escher-session-command/form · escher-session command line · command form — Every command is escher-session <verb> [arguments] --session <dir> over the fourteen verbs of the driver's verb table, --session exactly once on every command but help.
contract/escher-session-help/answer · escher-session help · answer — One uncoloured JSON line with seven keys (verbs, node_keys, key_names, busy_classes, causes, exit_statuses, session_errors), fixed schema strings, reaching no session.
contract/escher-session-mcp/tools · escher-session MCP server · tools/list — Lists the fourteen verbs of the verb table as tools in order, help last, each description the verb's help then ". Returns: " and its result fields.
contract/escher-session-record/check-file · escher-session recording · check-{n}.json — Written 0600, never replacing a file, in a 0700 directory whose parent must exist, removed by escher only when its recording fails; a shot kept without its png.
contract/escher-session-served-tasks/cells · escher-session served tasks · cells — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/counter · escher-session served tasks · counter — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/crud · escher-session served tasks · crud — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/flight-booker · escher-session served tasks · flight-booker — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/load · escher-session served tasks · load — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/timer · escher-session served tasks · timer — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-snapshot/answer · escher-session snapshot · answer — One uncoloured line: {"text"} for a whole screen, {"text","part","parts"} for a screen read in parts; a part past the last gives empty text with the same parts.
contract/escher-session-actions/answer · escher-session actions · answer — No argument; one unpaged uncoloured line {"nodes"} of id, owner, keyed and verbs with verb and cause, owner and cause left out when absent, never a value, typed text or NodeId.
contract/escher-session-coverage/answer · escher-session coverage role · answer — Each check replayed on a fresh boot; one uncoloured line {"unexercised":[{id,owner,event}],"checks","held"}, owner left out when absent; a failed boot is the session error, exit 3.
contract/escher-session-coverage/check-bound · escher-session coverage role · check file — Each check is read bounded at 1,048,576 + 1 bytes; unreadable, oversized, not a check, or for another app is refused malformed, exit 1, holding nothing of the path or file.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: the refusal's JSON on stdout.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from the verb table and the task names, stdout empty, nothing booted and no state created.
contract/escher-session-graph/answer · escher-session graph · answer — No argument; one uncoloured line {"nodes","states"} carrying ids, component names, handler event names, state names, closed flag words and booleans only, never a value.
contract/escher-session-lint/answer · escher-session lint · answer — No argument; one uncoloured line {"findings"}, each finding keyed id, defect, bounds then its reason's keys, carrying ids, closed words and numbers only.
contract/escher-session-record/record-dir-not-private · escher-session recording · record-dir-not-private — A recording directory open to its group or others is this session error, exit 3, start spawning nothing.

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
