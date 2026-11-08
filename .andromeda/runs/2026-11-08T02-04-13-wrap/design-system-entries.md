@amendment marker=2026-11-07-ui-graph
title: the cli surface's graph answer line
section: Self-Validation Protocol (body: the subjects list); §Surface: cli → Component Patterns (surface)
change:
  - A new Component Patterns item: `escher-session graph` takes no argument, and an argument is refused `malformed` with nothing run; it answers the screen's whole UI graph as one uncoloured line of JSON `{"nodes":[…],"states":[…]}` on stdout, status `0`, the same answer on the session socket's `call` and in a kept check, and over the `mcp` role its tool result is the text block of that one line.
  - The line carries ids, owner ids, component names, registered state names, handler event names, the closed flag words `undeclared` · `unconfirmed` and booleans only, never a value or typed text.
  - The subjects list gains `cli-graph` after `cli-output`.
why: the report measures `graph` answered on the socket, the command line, MCP and in a kept check, as one uncoloured JSON line, status 0, carrying names, closed flag words and booleans only. The crossing is a boundary widening ratified by the founder's P4 fork 1 answer, given by dialog in the overseer session and relayed by the operator through the P4 dialog (inputs#I4), with his P5 answer on the authored write declarations, relayed by the operator (inputs#I5). The confirm that this item lies inside those answers is the overseer's, at this wrap's halt (Q1), PROVISIONAL (delegate the overseer), standing until the founder's own word in his batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the unpaged graph, a pending limit
section: Self-Validation Protocol (body: the subjects list); §Surface: cli → Component Patterns (surface)
change:
  - A new pending Component Patterns item: the `graph` answer is unpaged, the whole screen in one line; Cells at boot answers 293,869 bytes, the stated limit «An unpaged graph» in docs/capabilities.md, owned by residual `res-7cc5bi4k`, target next.
  - The subjects list gains `cli-unpaged-graph` after `cli-graph`.
why: the report measures the Cells answer at 293,869 bytes after the readers and writers narrowings, the remainder recorded as the stated limit and residual `res-7cc5bi4k` (target next) on the operator's word in the implement session (inputs#I8), and kept so at this resume by the operator relaying the overseer (inputs#I9); no route or requirement change is asked.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: citation ranges re-read after the graph chunk's code
section: Core Colors (color-palette: the seven_guis stand row); §Surface → Tokens (platform-specific) and Platform-Specific Notes; §Surface: cli → Component Patterns (surface: the `escher-session` item)
change:
  - The facts' claims are unchanged; only their source ranges moved with the chunk's added lines.
  - The Cells current-cell focus outline: was cells.rs:551-554; now cells.rs:600-603.
  - The single select's list: was docs/capabilities.md:135-140; now :157-162.
  - An open list's row bounds: was snapshot.rs:152-155; now snapshot.rs:163-166.
  - MountedData calls during a render: was dioxus_document.rs:166-168 and :302-304; now :167-169 and :315-317.
  - The `escher-session` item: the `EXIT_STATUSES` table was schema.rs:503-541, now schema.rs:557-595; the written JSON was json.rs:317-373, now json.rs:400-462.
why: the report's New text, by line, records this chunk's additions to cells.rs, docs/capabilities.md, snapshot.rs, dioxus_document.rs, schema.rs and json.rs ahead of each cited range, so each citation's range was re-read to where its unchanged code now stands.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
