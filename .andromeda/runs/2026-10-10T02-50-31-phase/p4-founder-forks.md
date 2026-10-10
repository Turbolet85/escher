# Driver CLI — the nine founder forks (P4 of 2026-10-10-driver-cli)

Written for one sitting with the founder. Nothing below is chosen: the first dialog was returned unanswered by the operator ("this is the operator, not the founder … Do not take any option as chosen"). Every option is whole; a recommendation is marked and argued, never assumed. An answer in his own words that fits no option is an answer too.

Why these are his (the operator's directive at the take-up, `inputs#I1`): "A fork that shapes what the agent meets (command and argument names, the shape of stdout, exit codes, how a session is started and addressed) is the founder decision." The purpose he restated on 2026-10-09: an agent sees, drives and verifies a UI and learns the tool from the tool.

## What is already settled, and by whom
- **Technical, the operator, 2026-10-10 (`inputs#I2`):** JSON is written by hand inside `escher-driver` (no dependency enters; the question re-opens at "MCP surface", where a parser is needed anyway) · an id longer than 1024 bytes is not kept in the session's record of ids · a client waits at most 30 s for a command's answer (`hello` and `stop` keep 2 s).
- **Standing direction, the operator, 2026-10-07 (the carried note "one verb table"):** this entry's own commands join the one verb table; no second table is stated.
- **As built and not in question:** six verbs (`snapshot` · `click` · `type` · `press` · `advance` · `scroll`), eight refusal causes each with a fixed remedy, a password's and a file input's value read as a fixed mask, the command span's eight fields, no network port, unix only where a session crosses processes.

## How the forks depend on each other
| Earlier answer | What falls away |
|---|---|
| Fork 1 = "Hold the crossing" | Forks 2, 3, 4, 6 and 7 fall away whole (no command line is built). Forks 5, 8 and 9 still stand: they decide in-process forms and behaviour. |
| Fork 1 = "One process, no socket crossing" | Fork 2: its three "App binary …" options collapse into one (there is no session address; only "app binary" against "generic client" is left). Fork 3 falls away (no detached host: the process ends with its input). Fork 6: only a whole-script status exists, so its options decide what that one status means. Forks 4, 5, 7, 8, 9 stand. |
| Fork 2 ≠ "App binary, named sessions" | Fork 3: "List-and-stop command" and "Both" fall away — with path addresses there is no place to enumerate. "Idle expiry" and "As built, stated" stand. |
| Fork 5 = any | Independent of forks 2 to 4 and 6. |
| Forks 8 and 9 | Independent of every other fork. |

---

## Fork 1 — the crossing (a boundary widening)

A command-line process ends when its command returns, so the instance lives in a host process and each command's call and answer must cross between the two. Today the session socket carries only `hello` and `stop`; the snapshot text and the diff are ratified as "returned to their caller only" (the founder, 2026-10-07), and on 2026-10-09 he ruled that an in-process call returning them is not a widening. An out-of-process call is the case not yet ruled. The playbook names it: **Boundary widening — verdict: escalate** ("a subprocess/IPC boundary gains a new crossing"); it is ratified only by his own word.

**Question:** does a call and its answer cross the session socket, with ids, accessible names, control values, typed text and the snapshot's text on it and on stdout?

### 1A — The socket carries commands (recommended)
The one registered socket gains a third request beside `hello` and `stop`: a call goes in, its answer comes out. Access stays what it is — a `0700` directory the caller names, a `0600` socket, no other authentication — and no second listener or port is added. The wire states new bounds (a request up to 16 KiB, an answer up to 1 MiB, a 30 s read bound for an answer). A password's and a file input's value stay the fixed mask; nothing of a call or an answer enters a log line or a span field; no `NodeId` crosses.
Why recommended: it is the only form that gives every command its own process and its own exit code, which is what requirement v010-12 says ("an exit code meaning accepted or refused").
```
client (one process per command)        host (holds the instance)

$ escher-session click --session <dir> --id counter-increment
      |  session.sock  (0600, inside a 0700 dir)   |
      |-- call v2 click id=counter-increment ----->|  validate -> Session::run
      |<- ok v2 accepted {"settled":true,...} -----|
  stdout: {"settled":true,"added":[],"removed":[],"changed":[...]}
  exit:   accepted

crosses the socket and reaches stdout:
  ids, names, values (password and file: masked), typed text,
  the snapshot's text, the diff
never: a log line, a span field, a NodeId, a path
```

### 1B — One process, no socket crossing
The socket stays `hello` and `stop`. A single long-lived process holds the instance, reads one command per line on stdin and writes one JSON answer per line on stdout. Ids, names and values still leave the process on stdout (the requirement's "JSON output" cannot avoid that), but nothing of them touches the socket.
Cost: a command has no process of its own, so it has no exit code of its own — v010-12 can only be met for a script as a whole — and a shell flow that must read an answer before choosing its next command has to hold a coprocess or a FIFO open, which is not "shell alone" in the plain sense.
```
$ escher-session run counter <<'EOF'
snapshot
click --id counter-increment
snapshot
EOF
{"text":"..."}
{"settled":true,"added":[],"removed":[],"changed":[...]}
{"text":"..."}

exit: 0 only if every line was accepted; no per-command status
socket: hello, stop only — unchanged
```

### 1C — Hold the crossing
Nothing of a call or an answer leaves the process in this chunk. The chunk is cut down to what needs no ruling: the written JSON forms of an outcome and a refusal (proved in process), the bound on the id record, the measurement of `type` on a filled control.
Cost: no command an agent runs exists at its wrap; v010-12 and v010-04 stay unclaimed, and the typed-sentinel proof in a host's log and the timer host's standing boot stay owed, since both need a hosted command. The MCP surface behind it meets the same question.

---

## Fork 2 — what the agent runs, and how a session is started and addressed

An app must be compiled into whatever hosts it, so some binary is per app. Two facts from research bound the options: a check can reach a real binary only as its own package's (`CARGO_BIN_EXE_*`), and no manifest under `packages/` may name the stand. In every option `start` brings a host up detached (its stdin and stdout closed, so `$(…)` returns), waits until it answers, and prints the session's facts; `stop` ends it and removes the socket and the directory; the host can also be run in the foreground, its log on stderr.
*Depends on fork 1: under 1B only the "app binary / generic client" half of this fork is left; under 1C it falls away.*

### 2A — App binary, path address (recommended)
The framework's library carries the whole command line; each app ships one binary that hands it argv and a way to boot. The stand's is `escher-session`, which both hosts and drives. A session is the directory the agent names on every command with `--session` — exactly the caller-named private state directory that is registered today.
Why recommended: nothing new is registered (no default location, no env var, no workspace crate), the agent learns one binary and one address rule, and the stand's own tests can reach the whole surface. Cost: the path is repeated on each command.
```
$ escher-session start counter --session <dir>
$ escher-session snapshot --session <dir>
$ escher-session click --session <dir> --id counter-increment
$ escher-session stop --session <dir>

host role, in the foreground (what `start` spawns):
$ escher-session serve counter --session <dir>
```

### 2B — App binary, named sessions
Same single binary, but a session is a short name and the tool keeps every session under one per-user root. Shorter to type, and a root the tool knows makes a `list` command possible (fork 3).
Cost: a default location and an environment read are new registered resources — architecture and the security plan both state today that the state directory is a path its caller names, "never an env read or a cwd-relative default" — so this option is a second ruling on top of the crossing.
```
$ escher-session start counter --session s1
$ escher-session click --session s1 --id counter-increment
$ escher-session stop --session s1

s1 -> $XDG_RUNTIME_DIR/escher/s1      (0700, per user)
      fallback /tmp/escher-<uid>/s1 when the variable is unset
new registrations: an env read, a default state root, the name grammar
```

### 2C — App binary, env-var address
Same single binary; the address is still a caller-named directory, but it may be given once in an environment variable so later commands are bare. `--session` still works and wins.
Cost: a new environment read (the list of env reads is closed and registered), and a command's target becomes invisible on its own line — an agent that forgets the export drives nothing, or the wrong session.
```
$ export ESCHER_SESSION=<dir>
$ escher-session start counter
$ escher-session click --id counter-increment
$ escher-session stop
```

### 2D — Generic `escher` client
A new workspace crate ships one client binary, `escher`, the same for every app; each app still ships its host binary. The agent types `escher` everywhere and gives the host's own command line once, at start.
Cost: a new crate (a registered name; manifest lines that shift every root-manifest citation in the seven spec documents), the agent must know the host's command line to start anything, and the stand's checks cannot reach a binary of another package, so the flow's proof needs a path-finding workaround or lives outside the stand.
```
$ escher start --session <dir> -- escher-session counter <dir>
$ escher click --session <dir> --id counter-increment
$ escher stop --session <dir>
```

---

## Fork 3 — an orphaned host

A host whose client died without `stop` keeps running today: a session has no idle expiry. The carried note says this entry answers it — "an idle expiry, or a list-and-stop command".
*Depends on fork 1 (falls away under 1B and 1C) and on fork 2 (3B and 3C exist only with 2B).*

### 3A — Idle expiry (recommended)
A host stops itself, exactly as `stop` does, when no request has arrived for 30 minutes; every answered request restarts the count. The wait is on the host's own accept loop and touches no clock inside the instance: settle still reads none and time still moves only by `advance`.
Why recommended: an orphan cannot outlive the expiry, with no command to learn and no root directory to register. Cost: an agent that pauses longer finds "no session is running" on its next command and has lost the instance's state; `start`'s answer states the expiry so that is learnable. The 30 minutes is his to change.
```
$ escher-session start counter --session <dir>
{...,"idle_expiry_s":1800}
... 30 minutes with no request ...
host: stops, removes session.sock and <dir>
$ escher-session snapshot --session <dir>
-> session error: no session is running
```

### 3B — List-and-stop command
No expiry; a `list` command prints the sessions that answer and the agent stops what it no longer needs. Nothing dies under a thinking agent.
Cost: listing needs a place to look, so it exists only with named sessions under a per-user root (2B), and an orphan lives until someone lists.
```
$ escher-session list
{"sessions":[{"session":"s1","label":"counter","pid":4242}]}
$ escher-session stop --session s1
```

### 3C — Both
Idle expiry as the floor and `list` for an agent that wants to see what runs. It carries both costs: lost state after a long pause, and the per-user root `list` needs (2B).

### 3D — As built, stated
No expiry and no list. A host runs until `stop`; `start`'s answer says so and the orphan stays an owned item on the route for a later entry. Smallest change and no lost state; an agent that crashes leaves a process and a directory behind until a person removes them.

---

## Fork 4 — command and argument names

The six verbs keep the table's names. The session's own commands are proposed as `start`, `status` (does a session answer here, and what is it) and `stop`, with `serve` for the host role; any of these four words is his to rename. What forks is how an argument is written. The table's argument names are `id` · `text` · `key` · `shift` · `ms`.
*Depends on fork 1: under 1B the same form applies to each line of input; under 1C it falls away.*

### 4A — Named flags, the schema's own words (recommended)
Why recommended: the words on the command line are the words in the schema, in a refusal ("the required argument `id` is missing") and later in the MCP tool's arguments, so there is one vocabulary to learn; order never matters; a value that begins with `-` is unambiguous after its flag. Cost: the longest lines.
```
escher-session click   --session <dir> --id counter-increment
escher-session type    --session <dir> --id crud-name --text Ada
escher-session press   --session <dir> --key tab --shift
escher-session advance --session <dir> --ms 300
escher-session scroll  --session <dir> --id crud-person-12
```

### 4B — Positional, in the schema's order
Shortest; reads like a sentence. Optional arguments stay flags.
Cost: the order is learned from help, not from the line; a refusal names an argument (`id`) by a word the agent never typed; an id or a text beginning with `-` needs a `--` guard; a verb that later gains a second optional argument cannot express it by position.
```
escher-session click   --session <dir> counter-increment
escher-session type    --session <dir> crud-name Ada
escher-session press   --session <dir> tab --shift
escher-session advance --session <dir> 300
```

### 4C — `name=value` pairs
The line is the call itself: one rule for every argument kind, no flag grammar (`shift=true`).
Cost: unfamiliar on a command line, and it reads unlike `--session`, so the tool would carry two argument styles unless the address becomes `session=<dir>` too.
```
escher-session click session=<dir> id=counter-increment
escher-session type  session=<dir> id=crud-name text=Ada
escher-session press session=<dir> key=tab shift=true
```

---

## Fork 5 — the shape of stdout

Common to every option: an answer is exactly one line of JSON on stdout (so plain `sh` can match it with `case` and it survives `$(…)`), UTF-8, with no escape sequence, byte-identical in a terminal and in a pipe. An accepted acting verb answers one object whose keys are the schema's own result fields — `settled`, `busy` (only when not settled), `added`, `removed`, `changed`, plus `advanced_ms` or `in_view`. A node is one object with the schema's nine node fields (`id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `focused` · `bounds`), a field that does not apply left out, `bounds` as `[x, y, width, height]`. A refusal is `{"refused":{"cause":…,"meaning":…,"remedy":…}}`, with `"fault"` beside a malformed one. If he wants any of that different, his words change it. What forks is how `snapshot` carries the screen.
*Independent of forks 2 to 4 and 6; it stands even under 1C, where it decides the in-process form.*

```
accepted click (illustrative values):
{"settled":true,"added":[],"removed":[],"changed":[{"id":"counter-value","parent":"task-body","role":"Paragraph","name":"1","focused":false,"bounds":[24,112,752,40]}]}

refused:
{"refused":{"cause":"disabled","meaning":"the element carries the `disabled` attribute, which the snapshot reads as not enabled","remedy":"the control is disabled: change the state that disables it, then act again"}}
```

### 5A — The text, as one string field (recommended)
`snapshot` answers `{"text":"…"}`: the snapshot's text form, one line per element, nested by indent.
Why recommended: it is what the schema's `snapshot` row states today, it is the compact form the 10,000-byte budget was measured on (the four stand screens read 755 to 2034 bytes), and requirement v010-04 is proven with what is built. An agent reads an indented tree; a script matches a substring. Cost: the tool has two notations for an element — a line of text in a snapshot, an object in a diff.
```
(illustrative values)
{"text":"GenericContainer \"\" id=\"main\" @0,0 800x600\n  Button \"Count\" id=\"counter-increment\" enabled @24,160 80x28\n  ..."}
```

### 5B — A list of node objects
`snapshot` answers `{"nodes":[{…},{…}]}`: every element in document order, each the same object a diff uses (with `parent`). One notation everywhere, ready for `jq`.
Cost: larger — an estimated two to three times the text, so the budget is re-measured on every stand screen and v010-04's "fits in a single tool result" is re-proven on the new form; the schema's `snapshot` row changes its one field from `text` to `nodes`; the text form stays in the library and leaves the command.
```
(illustrative values)
{"nodes":[{"id":"main","role":"GenericContainer","name":"","focused":false,"bounds":[0,0,800,600]},{"id":"counter-increment","parent":"task-body","role":"Button","name":"Count","enabled":true,"focused":false,"bounds":[24,160,80,28]}, ...]}
```

### 5C — Both fields in one answer
`{"text":"…","nodes":[…]}`. Nothing to choose at call time. Cost: the largest answer (text plus objects, an estimated three to four times the text), and two statements of one screen that must be kept equal.

### 5D — Text by default, nodes on request
`snapshot` answers `{"text":"…"}`; `snapshot --nodes` answers `{"nodes":[…]}`. Compact by default, objects when wanted. Cost: one more argument in the schema (the first on `snapshot`), and two forms to prove and keep.

---

## Fork 6 — exit codes, and what each ending prints

A command line has more endings than the entry's "accepted or refused": the command ran and the app went quiet; it ran and the app did not go quiet (a result, not a refusal — the step is not rolled back); it was refused (eight causes, a wrong verb and a wrong argument among them); the line was not a command at all; the session could not be reached (none, dead, timed out, already running, unsupported platform).
*Depends on fork 1: under 1B there is one status for a whole script, so the options decide what that one status means; under 1C it falls away.*

### 6A — Four classes, the repository's standing grammar (recommended)
`0` accepted (went quiet or not — the JSON's `settled` says which) · `1` refused · `2` usage · `3` session error.
A wrong verb or a wrong argument **is a refusal**: exit 1 with the refusal's JSON (`unknown-verb`, or `malformed` naming the argument and the remedy), so the tool teaches the fix. Usage (exit 2) is only a line with no command word or no session address: a usage line on stderr, stdout empty. A session error prints `{"error":{"kind":"no-session","message":"no session is running"}}` on stdout and the same fixed message on stderr.
Why recommended: `0` means "it ran", so `cmd && next` behaves; refused, mistyped-beyond-repair and no-session are told apart without parsing; and it is the grammar the project's own scripts already use (`0` · `1` · `2` · `3`).
```
0  accepted   stdout: the verb's result            stderr: —
1  refused    stdout: {"refused":{...}}            stderr: —
2  usage      stdout: —                            stderr: usage line
3  session    stdout: {"error":{"kind":...}}       stderr: the fixed message
```

### 6B — Five classes: not-quiet has its own code
As 6A, but an accepted step whose app did not go quiet exits `4`. A script can stop on "ran, still busy" without reading JSON.
Cost: `0` no longer means "the command ran", so `cmd && next` skips `next` after a step that did run; one more code to learn. No stand flow produces it (the stand's tasks settle), so it is proven on a fixture only.

### 6C — Two classes
`0` accepted · `1` anything else; the JSON (`refused` or `error`) or stderr says which. The requirement's words, literally, and the simplest rule.
Cost: a script cannot tell "the app refused" from "there is no session" or "I mistyped" without reading the answer.

### 6D — One code per refusal cause
`0` accepted · `10` to `17` one per cause, in the cause set's order (`unknown-verb` … `time-unavailable`) · `2` usage · `3` session error. A script branches on the cause — `off-screen`, then `scroll` — with no parsing.
Cost: eight more codes to learn and a code table that must track the cause set for as long as the tool lives.

---

## Fork 7 — is the actionable-key check a command?

`unkeyed_actionable` lists the elements an agent can act on that read no author key, each with its remedy. It is a library function no command exposes; on 2026-10-06 the founder named the CLI and the driver as its reusers. It reads empty on the four stand tasks a host serves, and 2 · 3 · 676 on the three other tasks.
*Depends on fork 1: falls away under 1C.*

### 7A — Not in this chunk (recommended)
It stays a library check, asserted on the stand by its standing test, and the question is carried to "Self-description" — the entry that serves the verb list and help — where a check for app authors sits beside the tool describing itself.
Why recommended: it keeps this chunk's crossing to the six verbs' answers, and the check is about how an app was written, not a step of driving it. Cost: an agent driving an app cannot yet ask the tool which controls have no stable key.

### 7B — A command now
`escher-session unkeyed --session DIR` answers every such element. A seventh verb in the one table and a second crossing for ids (positional ones this time), ratified together with fork 1; the process-local node handle is left behind.
Cost: the answer has no natural bound (676 entries on one stand task), so a size rule or a cap is stated for it; one more verb to prove through the host.
```
(illustrative values; the remedy text is the library's own)
{"unkeyed":[{"id":"cells/table/tr:3/td:2","tag":"td","role":"Cell","focusable":false,"interactive_role":false,"listener":true,"remedy":"give the `td` element reading `cells/table/tr:3/td:2` an HTML `id` attribute that is non-empty, holds no `/` and is unique in the document"}]}
```

---

## Fork 8 — what `type` does to text already in a control

Read from the code, not yet measured: `type` clicks the centre of the control and types at the caret, and a click in a text input puts the caret where the click falls — so in a control that already holds text the typed text lands inside the old value, at a place that depends on the control's width and font. The stand's Flight Booker dates are filled at boot. No verb clears a control; `backspace` and `delete` exist, one press per character. Whatever is chosen, this chunk's first check measures the as-built behaviour in both layout modes.
*Independent of every other fork (it is a property of the verb, in process too).*

### 8A — `type` replaces what the control holds (recommended)
After focusing, the control's content is selected (the engine's own select-all) and the typed text replaces it; `type` with an empty text clears. An empty control behaves as today.
Why recommended: after an accepted `type` the control's value is the text — the easiest thing for an agent to predict and to verify — and a date is changed in one command. Cost: a behaviour change of a built verb (its help and its checks restated); text can no longer be inserted into the middle of a value by `type`; the select-all goes through the platform's own modifier key, so it is proven on the Linux and the macOS CI jobs.

### 8B — `type` appends
The caret is moved to the end of the value, then the text is typed: the value becomes old text plus new. Predictable, and nothing is destroyed.
Cost: a filled value can still only be changed by pressing `backspace` once per character, and a deleting key behaves differently per platform, so a "change the date" flow is long and fragile.

### 8C — As built, measured and stated
`type` inserts where the click lands. The chunk measures it, the verb's help says it, and replacing or clearing is carried as its own route item. No behaviour change now.
Cost: the result of typing into a filled control depends on geometry — the least predictable thing an agent could meet — and the shell flow avoids filled controls.

### 8D — Both, chosen per call
`type` appends by default; `type --replace` selects everything first. Everything is expressible.
Cost: one more argument in the schema (`replace`), and the agent must choose every time.

---

## Fork 9 — two engine findings whose readings now leave the process

Both were measured on the stand at 2026-10-07-refusal-detection and both carry a hypothesis not yet measured. Each fix changes behaviour for every document, in upstream-owned files, which the carried notes mark as "the founder's word"; both are upstreamable.
1. **A scrolled box's own `bounds` read shifted by its own scroll offset** while the box has not moved (`crud-list` read y 112.796875 before a `scroll` and 94.796875 after, and that step's diff names the list as changed). Hypothesis: a `click` naming such a box lands off its centre by that offset.
2. **The engine's hit reaches a row scrolled out of its box.** Hypothesis: a control lying where a scrolled-out row extends reads `covered`, and a pointer click there lands on the hidden row.
*Independent of every other fork.*

### 9A — Measure and state; fix neither here (recommended)
This chunk measures both hypotheses on the stand, in process, in both layout modes, and records the readings; the limits are stated where the tool describes `bounds` and `covered`; no engine file is edited; the two fixes stay owned on the route.
Why recommended: it keeps a large chunk from growing an every-document engine change under it, and a measured limit is honest for a first surface. Cost: `bounds` of a scrolled box and the diff's "changed" for it are known-wrong when they first leave the process.

### 9B — Fix the bounds reading here
The bounds reader stops subtracting the element's own scroll offset (both of its bodies). `bounds` and the diff are right before they first leave the process.
Cost: an edit in upstream-owned merge surface, `getBoundingClientRect` changes for every document (upstream's own tests re-read), and finding 2 is still only measured and stated.

### 9C — Fix both here
9B, and the hit walk is clipped at a scrolling box. Both readings are right at their first exit.
Cost: every pointer event of every document changes as well; two engine edits under a command-line chunk, and the largest test surface.

---

## Where this run stands
Chunk `2026-10-10-driver-cli` is promoted (`pending`), with `scope.md` and `research.md` written and closed. The plan is not written: it waits for these nine answers. The run dir is `.andromeda/runs/2026-10-10T02-50-31-phase/`; this session resumes at P4 on the founder's words, given as text.
