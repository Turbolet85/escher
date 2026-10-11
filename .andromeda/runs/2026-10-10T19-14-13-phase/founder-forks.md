# MCP surface — the founder's forks

Chunk `2026-10-10-mcp-surface` (working-route.md:79, v010-13), taken up 2026-10-10. Four forks are yours: each widens a boundary or changes what v010-13 says. They are written whole here, with what each depends on. The plan is not written until they are answered.

## What is being built, as decided so far

The operator answered the three technical forks (2026-10-10, `inputs#I2`); the forks below are read against this design.

- **A second role of the stand's binary, `escher-session mcp`.** An agent's client spawns it as its own child and speaks MCP with it on the child's standard input and output. No socket, no port, no file, no credential. The code lives in `escher-driver`, as the command line's does.
- **The server holds the app itself, in process.** All nine verbs are tools: `snapshot` · `click` · `type` · `press` · `advance` · `scroll`, and `start` · `status` · `stop`. `start` boots the named app inside the server; the session socket is not used and its wire is unchanged.
- **The handshake revisions of MCP only** (`initialize`, `tools/list`, `tools/call` — what the cold-agent pipe's stub speaks and the `claude` client on the dev host was proven against). MCP's current revision (2026-07-28) dropped the handshake; serving it is pinned on the route, after a measurement of what the host's client actually sends.
- **A `snapshot` tool result is the screen's text itself**, as `Snapshot::to_text` writes it; every other result is the same line of JSON the command line prints; a refusal and a session error are marked as errors so the client hands them to the model.

## Fork 1 — the MCP crossing

**The question.** May a tool call and its result cross between the server and the client that spawned it?

**Why it is yours.** A boundary widening. You ratified the session socket and the command line's stdout as carriers (2026-10-10); that answer names those two, and MCP is a third.

**What would cross.**
- In, on a tool call: the tool's name and its arguments — an element id (at most 1,024 bytes), typed text (at most 4,096 bytes), a key name, a flag, a count of milliseconds, an app name — and the request's own `id`.
- Out, in a result: the screen's text and the diff — stable ids, roles, accessible names, control values (a password's or a file input's as the fixed mask), the selected · pressed · invalid readings, bounds; a refusal's fixed texts; a session's label, process id, request count and idle expiry.
- The request's `id` is written back in the reply. The protocol requires it. It is the one place something the client sent is written back; proposed bound: a number, or a string of at most 128 bytes, and any other form is answered as a protocol error with no id.

**What does not cross, whichever way.** No `NodeId`. No password or file value beyond the mask. Nothing of a call or a result into a log: stderr carries fixed words and counts only.

**Where it goes after the client.** The carrier ends at the process that spawned the server. An agent client hands tool results to its model, and with a hosted model that is the model provider. escher does not control that leg; ratifying the carrier means the screen reaches whatever client the user points at it.

**Proposed bounds** (the builder's proposal, as the socket's were): a request line of at most 65,536 bytes — the widest call the schema admits, with JSON's worst escaping, is (1,024 + 4,096) × 6 = 30,720 bytes plus the envelope — and a longer or malformed line is answered as a protocol error with nothing run; a result of at most 1,048,576 bytes, as on the socket.

**Options.**
- **A — ratify as stated.** The same content classes as the socket and the command line, over the new carrier, with the request-id echo. *Recommended*: it is what "the same commands as MCP tools" needs, and nothing crosses that does not already cross on the command line.
- **B — ratify without control values.** Every control's value crosses as the mask, not only a password's and a file input's. Cost: an agent cannot read back what it typed, so a flow that sets a date cannot confirm it through MCP, and the MCP tools are no longer the same commands as the command line's.
- **C — not ratified.** The surface is not built and the entry returns to the route. Forks 2, 3 and 4 fall away.

## Fork 2 — a JSON parser as a dependency

**The question.** MCP requests arrive as JSON, and nothing in the driver reads JSON today. Does a parser enter as a dependency?

**Why it is yours.** The driver was built with no serialization crate, its JSON written by hand, and the question was re-opened for this entry. A named dependency is a registration in architecture §Stack and changes the driver's recorded dependency set.

**Options.**
- **A — name `serde_json` in `escher-driver`, as a reader only.** It is already compiled into the driver and into the stand's binary through the Dioxus crates (1.0.151, in the lock), so no package is added to any build and no new name enters the lock; the lock gains one line in the driver's own dependency list. The driver's answers stay written by the hand writer, so a result's JSON line is the command line's bytes. Cost: the driver names four crates, not three, and how hostile JSON is parsed becomes `serde_json`'s behaviour (the line is bounded before it is parsed). *Recommended.*
- **B — a reader written by hand.** No dependency. Cost: a parser for untrusted input is ours to write and keep correct — string escapes, numbers, nesting under a depth bound, passing over whatever a client's `initialize` carries. My estimate is about 300 lines with its test tables; not measured.
- **C — the MCP SDK, `rmcp` 3.5.1.** The protocol is handled for us, both revisions. Cost: it brings an async runtime into the driver and the stand's binary, which reverses the recorded decision that the session library is `std`-only; new names enter the lock (its dependency list was not read, so they are not counted here); its HTTP transports must be kept off; the bytes on the wire, the bounds and what is written back become the SDK's behaviour. It also re-opens the operator's protocol answer and the bounds proposed in fork 1.

**Depends on.** Fork 1 (moot under 1C). Independent of forks 3 and 4.

## Fork 3 — what proves "an agent session completes a stand flow"

**The question.** v010-13 reads: "Every command available on the command line is also offered as an MCP tool, and an agent session completes a stand flow using only those tools." What is the agent session?

**Why it is yours.** The answer decides what v010-13 says, and one option sends the stand's screen to the model provider.

**What stands already.** The route's last entry, "Cold-agent test" (v010-15), swaps the pipe's stub for this MCP surface and runs a fresh agent through it. Your budget ruling was one live run, so a further one needs the operator's word, and the first live trial was run on yours. That trial (2026-10-10, on your "now, small") ran through the command line, and its findings shaped this surface — the escaped one-line snapshot it called awkward is why a `snapshot` tool result is plain text.

**Options.**
- **A — a small live trial now, as the first one was.** On the dev host, once, in the operator pass: a fresh `claude` session whose only tools are this server's does one stand flow. Recorded: the count of tool calls, the count of refused calls, whether the end state was reached — figures only, no transcript committed. With a scripted check in CI, v010-13 is verified at this chunk. Cost: one more live run, and the stand's screen reaches the model provider through MCP as it did through the command line in the first trial; the pipe's ratification covered its toy stub only. The pipe itself is not changed — that stays the Cold-agent test's. *Recommended*: the first trial paid for itself, and a defect only an agent meets is found now rather than at the version's last entry.
- **B — a scripted client, and v010-13 reworded.** A check that speaks the protocol with no model completes the flow, in CI. v010-13's second half becomes "a scripted MCP client session completes a stand flow using only those tools", verified here. No live run. Cost: the cap no longer says an agent did it.
- **C — a scripted check here, v010-13 left open.** This chunk proves the tools and a flow with a scripted client and does not mark v010-13 verified; the Cold-agent test's fresh agent, which runs through this surface, verifies it. No live run now, no rewording. Cost: v010-13 stays open until the version's last entry.

**Depends on.** Fork 1: under 1C moot; under 1B a live flow must be one whose end state reads in names, since values would be masked. Independent of forks 2 and 4.

## Fork 4 — the tool call's line in the log

**The question.** The entry says a tool call parents its trace. May the tool call leave its own span, with two fields, in escher's log?

**Why it is yours.** A new log field. Your closed-field rule (2026-10-10) is worded "with no new … log field", so it does not cover one; the last new record in the sink was treated as a widening and ratified by you (2026-10-09).

**What is proposed.** One span per tool call, `tool_call` under the target `escher_driver`, at INFO, around the call: the driver's existing `command` span becomes its child with no change to the executor. Two fields, both from closed sets: `tool` — the verb table's own word, absent when the client named none of the nine — and `ending` — `accepted`, `refused` or `error`. Nothing the client sent: not the tool's name as spelt, not the request's id, no argument, no result and no length of one. Effect: one more line per tool call on stderr at `RUST_LOG=info`, printed after its child's. The parent relation is not printed — a closed span's line names no parent, and a check proves it with its own subscriber; printing a parent would change the record you ratified and is not proposed.

**Options.**
- **A — the span with `tool` and `ending`.** *Recommended*: a session-level tool (`start` · `status` · `stop`) runs no `command` span, so without `tool` its line says nothing.
- **B — the span with no fields.** The parenting holds; the line reads `span="tool_call"` and nothing else.

**Depends on.** Fork 1 (moot under 1C). Independent of forks 2 and 3.

## What does not come to you, and why

- **Which session-level verbs are tools** — all three, so v010-13's "every command" stands as written.
- **A trace identity on the session socket** — not needed: the app is held in the server's own process, so the parenting is plain span nesting and the socket's wire stays as ratified.
- **The protocol revision** — the operator's: the handshake revisions now, the current revision pinned on the route.
- **The snapshot's form in a tool result** — the operator's: the screen text itself.

## How the answers combine

| If | then |
|---|---|
| 1C | forks 2, 3, 4 fall away; the entry returns to the route |
| 1B | 3A's flow must end in a state that reads in names |
| 2C | the operator's protocol answer and fork 1's bounds are asked again |
| 3A | one live run is spent; needs 1A or 1B |
| 3A or 3B | v010-13 is claimed by this chunk |
| 3C | v010-13 stays open; this chunk claims no capability |

An answer can be given as four labels — for the recommendations, `1A 2A 3A 4A` — or in your own words.
