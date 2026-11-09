@amendment marker=2026-11-08-handler-coverage
title: the coverage role a further consumer of the TaskShell mount
section: §Surface: desktop-native → Primary screens (the seven_guis Home and TaskShell entry)
change:
  - The `escher-session` binary's consumers of the stand's TaskShell mount gain its `coverage` role after the `mcp` role: it boots the kept checks' app once per check through the same boot and replays each check on that fresh mount (was the host role and the `mcp` role alone).
  - The role adds no markup, id, class, style, wrapper or order; a held instance still reads as a fresh boot, and the mount and the pinned viewport hold after commands in both layout modes.
  - The entry's grounds gain the binary's module doc naming the role, its later citations moved down by those lines.
why: the report's Symbols / APIs — the binary's boot closure and `command_line` call are unchanged, the role calling the same closure once per check. The role itself is a ratified boundary widening (rul-izni57qv) on the founder's own word at P4 Q1, relayed verbatim by the overseer (inputs#I4); this edit was confirmed at the wrap's answer card Q1 by the overseer as delegate, PROVISIONAL (delegate overseer), the founder being asleep, for the founder's own word in his batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the bridge's skeleton and head notes re-cited
section: §IA notes (the fixed skeleton, the `index.html` templates TODO and the head elements notes)
change:
  - The three notes' citations into the Dioxus DOM bridge's document file move one line down, after the chunk's one added line near the file's top; each note's claim reads as before.
why: the report's New text, by line — `packages/dioxus-native-dom/src/dioxus_document.rs` gained a line at 10 beside the reached record's lines in `handle_event`; no claim of the notes moved.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: escher-session's coverage role
section: §Surface: cli → Primary screens (the `escher-session` entry)
change:
  - A fourth role after the `mcp` role (was three roles: the commands, the host role, the `mcp` role): `escher-session coverage <app> <check>...` — one app the binary boots, then one or more kept-check paths, no `--session` and no `--record` word among them; any other shape usage (stderr, exit 2).
  - The usage line names it: `; coverage role: coverage <app> <check>...;` before `; apps:`.
  - Each path read bounded at `MAX_CHECK_BYTES + 1` (1,048,576 + 1 bytes) and parsed by `Check::from_bytes`; a file that cannot be read, one past the bound, one that is no check and a check whose app is not `<app>` each refused `malformed`, exit 1, holding nothing of the path or the file.
  - Accepted, it replays each check on a fresh boot through the same boot `main` hands to the command line, so a kept check is read by this role as by `Check::replay`, and answers one uncoloured line `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}` on stdout, exit 0, `owner` left out when absent, carrying ids, owner ids, event names and two counts only; a boot that fails ends with the session error, exit 3.
  - The entry's other citations into the driver's command line and the binary's host move to their lines after the chunk's additions; their claims read as before.
why: the report's Symbols / APIs and Schema / config — `command_line` gains the role, its one shape, its endings and its usage clause, the coverage line a written form of the command line alone, no verb added. A ratified boundary widening (rul-izni57qv) on the founder's own word at P4 Q1, relayed verbatim by the overseer (inputs#I4); this edit was confirmed at the wrap's answer card Q1 by the overseer as delegate, PROVISIONAL (delegate overseer), the founder being asleep, for the founder's own word in his batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
