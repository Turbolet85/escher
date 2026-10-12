# layouts extract

## Relevance
partial. The chunk changes what the `cli` surface's `escher-session` binary serves on both of its roles: the command line and `escher-session mcp`. It adds no screen, region, focusable element, modal or breakpoint to any visual surface. The `cli` surface's own header marker (NOT YET MEASURED) covers only its tooling context, expression level and signature placement. Nothing below is drawn from those facets.

## Constraints
- layout-templates §Surface: cli → Primary screens (`escher-session`) requires one command shape: `escher-session <verb> [arguments] --session <dir>` over the verbs of the driver's verb table. Argument flags are spelt as the schema spells them, and `--session <dir>` appears exactly once on every command. Any help form this chunk adds (a `help` verb, CARRY 1) must fit this grammar or amend it. Whether a tenth table verb or a form served outside `VERBS` is the right shape is P4's fork (scope "Who decides").
- layout-templates §Surface: cli → Primary screens (`escher-session`) requires exactly four endings:
  - accepted: one line of uncoloured JSON on stdout, exit 0
  - refused: the refusal's JSON on stdout, exit 1
  - usage: one fixed line on stderr built from the verb table and the task names, stdout empty, exit 2, with nothing booted and no state created
  - session error: the error object on stdout and its fixed message on stderr, exit 3

  Served help must land in one of these endings. Which ending it takes, and whether the exit statuses' meanings (CARRY 1) move into the table, are open questions for P3 and P4.
- layout-templates §Surface: cli → Primary screens (`escher-session`) requires that no command's ending write back anything the caller typed. Served help, schemas, causes, remedies and session-error text (CARRY 1, CARRY 3) must therefore be fixed program text only.
- layout-templates §Surface: cli → Primary screens (`escher-session`) fixes the `mcp` role's shape:
  - `mcp` alone is accepted, and any other shape after it is usage. The usage line names the role (`; mcp role: mcp`).
  - stdout carries protocol lines only, uncoloured, with no ESC byte.
  - the role speaks the handshake revisions `2025-06-18` · `2025-03-26` · `2024-11-05` and answers `2025-06-18` to any other.
  - it lists the table's verbs as its tools, in table order.

  If CARRY 6 decides to take up a new revision or answer `server/discover`, this row needs amending.
- layout-templates §Surface: desktop-native → Primary screens requires that the host role and the `mcp` role add no markup, id, class, style, wrapper or order to the lean-task TaskShell mount. A held instance must read as a fresh boot. CARRY 7's password check needs a password input, and that input belongs in a fixture beside the stand, never in the lean tasks' markup.

## Patterns to follow
- The usage line is "built from the verb table and the task names", per layout-templates §Surface: cli → Primary screens (`escher-session`). Any served verb list, help or shape should be built from the one table in the same way, not restated.
- `scripts/agent-run.sh` and `scripts/cold-agent.sh` follow one pattern, per layout-templates §Surface: cli → Primary screens:
  - usage goes to stderr with exit 2, checked before any precondition.
  - stdout is JSON lines only.
  - the grammar is documented in test-plan §3.

  This is the existing shape for an agent-facing command's self-description.
- The MCP tool listing follows the verb table's order, per layout-templates §Surface: cli → Primary screens (`escher-session`). Anything the chunk adds to that listing, such as result shapes, causes or node keys, should keep table order.

## Anti-patterns to avoid
- No coloured output or ANSI cursor/escape sequences on the driver's stdout. layout-templates §Surface: cli → Output structure — wpt runner records the WPT runner's in-place ANSI rewriting, but §Surface: cli → Primary screens (`escher-session`) requires uncoloured output with no ESC byte.
- No caller-typed text echoed in served help, usage or error text, per layout-templates §Surface: cli → Primary screens (`escher-session`).
- No id, class, wrapper or reorder added to the stand's lean tasks to serve a fixture or a check, per layout-templates §Surface: desktop-native → Primary screens.

## Contract bindings
- cli ↔ tests: layout-templates §Surface: cli → Primary screens (`escher-session`) places the session contract the binary serves in test-plan §3 → Session lifecycle. Any change to the endings, the usage line or the `mcp` role's listing binds to that contract and to the checks that read it.
- cli ↔ driver schema: the usage line and the MCP tool listing are both read from the driver's one verb table, per layout-templates §Surface: cli → Primary screens (`escher-session`). Whether both surfaces' new self-description also reads only from that table is research's question.

## Acceptance criteria contributions
- (layouts) A usage error still writes exactly one fixed line on stderr, built from the verb table and naming the `mcp` role, with stdout empty, exit 2 and nothing booted. Every new help form ends in one of the four endings (per layout-templates §Surface: cli → Primary screens).
- (layouts) Every byte the chunk adds to stdout on either role holds no ESC byte, no colour and no text the caller typed (per layout-templates §Surface: cli → Primary screens).
- (layouts) `escher-session mcp` followed by any extra argument still ends as usage, and the MCP tool listing keeps the verb table's order (per layout-templates §Surface: cli → Primary screens).
- (layouts) The lean tasks' TaskShell mount holds no new markup, id, class or order after the chunk. A password input added for CARRY 7 lives in a fixture beside the stand (per layout-templates §Surface: desktop-native → Primary screens).
