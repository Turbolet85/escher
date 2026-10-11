
## 2026-10-10-mcp-surface — `escher-session`'s third role, `mcp`
**Section:** §Surface: cli (the `escher-session` entry) · §Surface: desktop-native (seven_guis Home and TaskShell)
**Change:**
- cli: the binary's third role, `escher-session mcp` — `mcp` alone, any other shape after it usage, the usage line naming it — an MCP server over stdio, local to the invoking user, no listener or auth surface: JSON-RPC 2.0 requests on stdin, protocol lines only on stdout, the handshake revisions `2025-06-18` · `2025-03-26` · `2024-11-05` (`2025-06-18` to any other), the nine verbs as its tools, a tool call run on the one instance it holds in process; it echoes a request's admitted `id`; its end writes nothing on stdout, 0 on `Ok`, 3 with a fixed message on stderr on `Err`. Was "no ending writes back anything the caller typed", now scoped to the commands.
- desktop-native: the `mcp` role is a further consumer of the TaskShell mount, booting one lean task in process on a `start` tool call through the same boot, adding no markup.
**Why:** a new mode of an existing surface; the crossing it carries is the founder's ratified one (2026-10-10), recorded in the security plan.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/
