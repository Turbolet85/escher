The operator's answers to the three technical forks, given through this session's question dialog at P4 (2026-10-10). Each line is the question as asked, the option's label as chosen, and the note typed with it, verbatim.

"Where does the instance live that the MCP tools act on?"="In process, nine tools (Recommended)"

"Which protocol era does the server speak? (Found in research: MCP's current revision, 2026-07-28, has no `initialize` handshake.)"="Handshake only"

"What is the text of a tool result?"="Snapshot as plain text"

notes: Q2: no client on hand needs the current revision, and its proof would rest on a client we write ourselves; pin it on the route with what the host client actually sends, measured first. Q3: the first agent trial named the escaped one-line snapshot as awkward to read; MCP has no shell to please. The block is to_text output byte for byte — a check holds it equal to the in-process text and to the command line text field unescaped; every other result stays the JSON line, isError as you propose..

The options as they were put (label — description):

Q1
- In process, nine tools (Recommended) — The MCP server holds the Session itself. `start <app>` boots the app inside the server, `status` answers, `stop` drops it; the server keeps the 1,800 s idle expiry itself. No socket and no host process, works on every platform, and the tool call's span parents `command` by plain nesting — so no founder fork on the wire, and v010-13's "every command" holds. Cost: a session started by MCP is not reachable from the command line, `pid` reads the server's own, and the boot the binary hands over must be callable more than once.
- In process, app fixed at spawn — `escher-session mcp <task>` boots one task when the client spawns it and offers the six instance verbs only. Cost: `start` · `status` · `stop` are not tools, which changes what v010-13 says — a founder fork.
- Client of a hosted session — `escher-session mcp --session <dir>` sends each tool call over the registered socket; unix only; the `command` span is made in the host process — parenting it needs a trace identity on the socket (a founder fork), or the parenting stops at the process edge.

Q2
- Both eras (Recommended) — answer the `initialize` handshake (revisions up to 2025-11-25) and the current revision's per-request `_meta` with `server/discover`.
- Handshake only — as the stub: `initialize`, `tools/list`, `tools/call`. Smallest untrusted-input surface and proven against this host's client. Cost: a client that speaks only the current revision fails against it; the current revision is carried to a later route entry.
- Current revision only — per-request `_meta`, `server/discover`, no handshake.

Q3
- The driver's JSON line (Recommended) — one text block holding the same line the command line prints on stdout, with `isError` true for a refusal and a session error.
- JSON line plus structuredContent — the same text block, and the same object again as `structuredContent`.
- Snapshot as plain text — `snapshot`'s text block is the screen text itself, multi-line; every other verb keeps the JSON line.
