# Self-description — the founder's forks

Chunk `2026-10-12-self-description` (working-route.md:81, v010-14), taken up 2026-10-12. Four forks are yours. Each either widens a boundary or changes what v010-13 says. They are written whole here, with what each depends on. The plan is written on the recommendations and marked as waiting on these answers; an answer that differs from a recommendation sends it back to re-planning before the operator approves it.

## What is being built, as decided so far

The operator answered the four technical forks (2026-10-12, `inputs#I2`). The forks below are read against this design.

- **One self-description document, written from the one table.** It holds:
  - every verb, with its level, its help, each argument (name, kind, bound, help) and each result field (name, kind, whether always present, help);
  - the twelve node keys, each with a help of its own: what `selected`, `pressed` and `invalid` mean, and the two limits of the boolean model;
  - the key names and the busy classes;
  - the eight refusal causes, each with its meaning and remedy;
  - the four exit statuses with their meanings (today these are only a code comment);
  - the thirteen session errors, each with its kind, message and remedy.

  Every text in it is a fixed string of the program. None holds anything a call supplied or a screen reads.
- **Served on both surfaces from that one document.** The command line answers it as one line of JSON on stdout. The MCP server serves it as a tool result (or through `initialize`, see fork 2). The tool descriptions in `tools/list` also gain each verb's result fields (the operator's answer).
- **Smaller decided changes.**
  - A changed node in a diff names the keys whose reading differs (`differs`: closed words from the node-key list). On the flight booker's Book step all eight changed nodes only moved; that was measured at P3, and the live agent could not tell.
  - The MCP server also speaks `2025-11-25`, the revision the trial's client asked for.
  - A session error gains a fixed remedy, and `state-dir-too-long` states its byte bound (94 on Linux, 90 on macOS). The operator made the remedy and the bound conditional on your answer to fork 1.
- **Owed checks carried here, run in this chunk:**
  - a password typed through an MCP `type` and read back, on a fixture beside the stand;
  - every refusal cause the stand can provoke, read on its served form with its remedy.

## Fork 1 — the tool's own text crossing out

**The question.** May the program's own fixed help texts cross out over the carriers you have ratified? That means help, meanings, remedies, the exit statuses' meanings and the session errors' remedies, going out on the command line's stdout, the MCP server's stdio and the session socket.

**Why it is yours.** Your closed-field rule (2026-10-10) lets a new field of a closed value set ride a ratified carrier without coming to you, and it names "authored or free text" as still a widening (`.andromeda/playbook.md:40-42`). A help text is authored text: ours, not a page's or a caller's. So the rule's letter does not settle it.

**What already crosses of this kind.** A refusal's cause, meaning and remedy, on all three carriers, ratified with the socket's and the command line's crossing (2026-10-10) and listed in the MCP crossing (fork 1 of the MCP surface, "a refusal's fixed texts"). The MCP `tools/list` already carries each verb's help and each argument's help with its bound.

**What would cross, newly.**
- The help of every result field and of every node key.
- The four exit statuses' meanings.
- The thirteen session errors' remedies, and the byte bound in one message.
- The key names and busy classes as a list.
- On MCP, each tool's description lengthened by its result fields.

Each is a `'static` string of escher-driver's table. None is assembled from a call, a page or the screen.

**What does not cross, whichever way.** No id, name, value or typed text beyond what already crosses; no `NodeId`; no path a caller gave (the bound is stated as a number, the `--session` path is never written back); nothing into a log.

**Options.**
- **A — inside the ratified crossing, with a rule.** The program's own fixed text, held as `'static` data of the one table, may ride a ratified carrier, as a refusal's texts already do. Text from a page, a call or a screen is still a widening, and so is a new carrier or exit. The rule is written into the playbook beside your closed-field rule at this chunk's wrap. *Recommended*: it states what already holds for refusals and tool descriptions, and it is what "help served by the tool itself" needs.
- **B — ratified for this chunk, no rule.** The texts listed above cross; any later text of the kind comes to you again. Cost: the next help text (a screenshot verb's, at the next entry) is another question.
- **C — not ratified.** Only closed words cross: names, kinds, bounds as numbers, cause and error kinds. No help text, no remedy beyond what already crosses. Cost: v010-14's "help" is not met, so this chunk does not claim v010-14, and the session-error remedy and the tool-description fields fall away.

**Depends on.** Nothing. Forks 2 and 3 are moot under C only as far as their help text goes.

## Fork 2 — the help as a tenth verb of the table

**The question.** Is the help a verb of `VERBS`, served the same way on both surfaces? Or is it served outside the table, keeping the tools at nine?

**Why it is yours.** v010-13, verified, reads that `tools/list` "offers exactly the nine verbs of `escher_driver::VERBS` — every command the command line runs". A tenth verb changes what a claimed requirement says. A verb added to the validated verb set was also treated as a widening and ratified by you before (`scroll`, 2026-10-07). The operator's own direction of 2026-10-07 was "a `help` verb joins the table at this entry's promotion" (CARRY 1).

**Options.**
- **A — `help`, a tenth verb.**
  - It takes no argument and asks nothing of a session (a third level, "the tool").
  - `escher-session help` answers the document as one line of JSON, with status `0` and no `--session` needed.
  - The MCP server lists `help` as a tenth tool and answers it itself; its result is the same document.
  - `Session::run` refuses it as it refuses `start`; its `tool_call` span's `tool` is `help`, a new closed word.

  v010-13's acceptance is restated: "offers exactly the ten verbs of `escher_driver::VERBS` — every command the command line runs — in the table's order …", the rest unchanged.

  *Recommended*: one verb set served identically on both surfaces, what CARRY 1 recorded. An MCP client reaches the document by a call it already knows how to make. A test reads the two surfaces' documents as the same bytes.
- **B — outside the table.**
  - The command line takes `help` as a role word, like `mcp`, and answers the same line.
  - MCP serves the document in `initialize`'s `instructions` field, and the tools stay nine, so v010-13 stands as written.

  Cost: two shapes. A client "MAY" use `instructions` (the specification's word), and whether the `claude` client does is not measured. An agent that skips it learns the causes and exit statuses from nowhere on MCP.

**Depends on.** Fork 1. Under 1C the help verb or role serves closed words only.

## Fork 3 — the actionable-key check as a command (CARRY 2)

**The question.** `DioxusDocument::unkeyed_actionable()` lists the elements an agent can act on that read no author key. Does a driver command expose it?

**Why it is yours.** A command that exposes it would be a new exit for stable ids, and your answer of 2026-10-10 carried the question here. Its result also holds a process-local `NodeId`, which a wire form must leave behind.

**Options.**
- **A — not exposed in 0.1.0, recorded for a later version.** No 0.1.0 requirement asks for it. The rule it checks is proven on the stand by a test (`stand_actionable_keys`), and its user is an app's author, not an agent driving the app. It is recorded in `.andromeda/residuals.md` at this chunk's wrap. *Recommended.*
- **B — a `keys` verb.** It answers the positional ids of the unkeyed actionable elements, each with its remedy, and no `NodeId`. Cost: an eleventh verb, a new answer holding ids, and v010-13 restated again.
- **C — not exposed, carried on the route.** A `CARRY:` on "Cold-agent test", where a fresh agent's need for it would show.

**Depends on.** Fork 2 for the verb count under 3B.

## Fork 4 — `server/discover` and the current revision (CARRY 6)

**The question.** Does the server answer `server/discover`, the current revision's (`2026-07-28`) discovery method? The trial's client sent it first and fell back to `initialize` on `-32601`.

**Why it is yours.** A new method answered is a new input class on the MCP surface you ratified: its row says "four methods answered, else `-32601`".

**What the current revision is** (re-read 2026-10-12, research.md). It removes `initialize` and `ping`. Every request carries its revision in `_meta`, every result needs a `resultType`, and `tools/list` needs `ttlMs` and `cacheScope`. Servers "MUST implement" `server/discover`. Clients "MAY call it … as a backward-compatibility probe on STDIO", which is what the measured client did.

**Options.**
- **A — not answered now; the current revision recorded for later.** `-32601` stays, and the client's fallback stands as measured. Taking up `2026-07-28` is a stateless redesign of the server, recorded in `.andromeda/residuals.md` at this chunk's wrap. *Recommended*: the operator's revision answer already gives the client the revision it asks for.
- **B — answer `server/discover` alone.** It answers the supported revisions, capabilities and server name, and no more of `2026-07-28`. Cost: a fifth method, and a mixed revision the specification does not describe.
- **C — take up `2026-07-28` whole, as a new route entry.** Placed by the wrap.

**Depends on.** Nothing.

## What does not come to you, and why

- **The revision** — `2025-11-25` spoken: the operator's (one closed word in the ratified `initialize` answer, under your rule).
- **`differs` on a changed node** — closed words in the ratified diff, under your rule: the operator's.
- **Where MCP serves result fields** — the tool descriptions: the operator's, on your fork 1.
- **The document's shape** (one JSON line, no argument) and **a help per node key** — leaned from the table's existing pattern.

## How the answers combine

| If | then |
|---|---|
| 1A | the playbook gains the fixed-text rule at the wrap; v010-14 is claimed |
| 1B | as 1A for this chunk, no rule |
| 1C | help text, remedies and description fields fall away; v010-14 stays unclaimed |
| 2A | v010-13's acceptance is restated with "ten verbs"; four `== 9` checks move |
| 2B | v010-13 stands; MCP serves the document through `initialize` |
| 3B | an eleventh verb; v010-13 restated again |
| 4B or 4C | the MCP row in the security plan gains a method (4B) or a new entry is placed (4C) |

An answer can be given as four labels — for the recommendations, `1A 2A 3A 4A` — or in your own words.
