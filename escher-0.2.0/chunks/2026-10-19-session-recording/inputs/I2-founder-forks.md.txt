# Founder forks — 2026-10-19-session-recording («Session recording», v020-04)

Phase P4 of `.andromeda/runs/2026-10-19T02-56-17-phase/`. The founder is away. On the operator's directive
(inputs#I1, `relay-1.md`) every fork that is his is written here: its options whole, its dependences, a marked
recommendation, and the option that widens least named as such. The operator answers in his stead. Each answer is
recorded as a delegate's answer (`delegate the operator`), PROVISIONAL. Each fork's option ships what the playbook's
«Boundary widening» pattern names (verdict `escalate`), so whatever is answered here, the wrap still escalates it in
full. The founder's own word at the next epoch boundary discharges the PROVISIONAL marks (the playbook's «Provisional
discharge» rule).

**Why these three are his.** Today the security plan says «no log, event or file carries an id, a name, a value, the
snapshot's text or a diff's content» (security-plan §Input Validation, the 2026-10-10-driver-cli row). §Data
Protection lists no file the driver writes. A typed password crosses in and nowhere else: every screen reading of it is
the mask `MASKED_VALUE`, and its text «reaches no log, event or file». A kept check is a new carrier and a new exit
(CLAUDE.md Critical Warnings; the founder's rule of 2026-10-10: «a new carrier or a new exit still is» a widening).
- F1 decides where that carrier lives and how a recording is asked for.
- F2 decides what it holds of a password.
- F3 decides what it holds of the screen.

**What research measured** (`escher-0.2.0/chunks/2026-10-19-session-recording/research.md`):
- The host's teardown removes the socket, then the state directory with a non-recursive remove whose failure is ignored
  (`packages/escher-driver/src/host.rs:80-81`). `stand_session_quiet` pins the directory's listing to `session.sock`
  alone (`tests/blitz-tests/tests/stand_session_quiet.rs:70-80`). `stand_session_lifecycle` pins the directory gone
  after `stop` and the expiry.
- The `mcp` role takes no argument at all: `mcp --session d` is usage (`packages/escher-driver/src/cli.rs:456-458`).
  `serve_mcp` takes no location (`packages/escher-driver/src/mcp.rs:82`).
- Whether a control is masked (an `input` of type `password` or `file`) is decided inside dioxus-native-dom's private
  snapshot reader (`packages/dioxus-native-dom/src/snapshot.rs:231-243`).

---

## F1 — Where a kept check lives, and how a recording is asked for

Two things are decided together:
- the carrier (a file escher writes, or an answer the caller keeps);
- who names its place: whoever launches the driver, or the agent inside a call.

The verb table, `help` and `tools/list` change only under option D.

**A. A recording directory named by whoever launches the driver — opt-in, never named by a call. (Recommended)**
- **Off by default.** Nothing is recorded unless the launcher names a directory:
  - `escher-session start <task> --session <dir> --record <dir>` on the command line. `start` passes it to the host
    role it spawns: `serve <task> --session <dir> --record <dir>`.
  - `escher-session mcp --record <dir>` for the MCP role. The launcher is whoever writes the MCP client's server
    config: the operator, never the model.
- **What is written.** One check per session, from `start` to `stop` or the idle expiry, as one file in that directory.
  - The directory is created `0700`, or refused when it is open to its group or others. The state directory follows
    the same rule.
  - The file is created new and `0600`. It never replaces an existing file, and its name is a fixed pattern holding
    no id, label or text.
- **What stays unchanged.**
  - The state directory and its teardown: `session.sock` alone, removed at `stop`, so `stand_session_quiet` and
    `stand_session_lifecycle` read as today.
  - No tool argument, verb or result field is added, so the eleven verbs, `help`'s line and `tools/list` stay
    byte-identical. Only the closed argv reader gains the one option, on `start`, `serve` and `mcp`, and so does the
    usage line built from it.
- **What widens.** A new at-rest carrier: an owner-only file holding call content (F2) and screen readings (F3). It
  needs a new §Occupied Resources entry (arch) and a §Data Protection medium (security).
- **Why it is recommended.** It is the only option that keeps escher itself owner-only at rest, as the entry asks. No
  agent inside a call can choose where escher writes. Typed text never re-crosses the stdout or MCP stdio, and the
  session contract is untouched.
- **Dependences.** F2 and F3 decide its content. Under A the MCP role gains its first argument, an amendment to the
  ratified `mcp` crossing's grammar (layout-templates §Surface: cli; security-plan §Input Validation, the CLI
  arguments row).

**B. Inside the session's own state directory, `{session}/` — hosted sessions only.**
- The host writes the check beside `session.sock` and leaves it at `stop`. The state directory then outlives the
  session (its non-recursive remove fails on the file), and the session contract changes:
  - the `session-state` label, «`session.sock` alone, absent after `stop`»;
  - the obs-plan §9 state-directory row;
  - `stand_session_quiet`'s listing and `stand_session_lifecycle`'s absence pins.
- The MCP role has no state directory. MCP recording would still need A's `mcp --record <dir>`, or would not exist,
  which breaks the entry's «via CLI or MCP».
- **What widens:** the same new carrier as A, plus a change to the ratified session-state contract.
- **Dependence:** with no MCP half, B meets the entry only together with A for the MCP role.

**C. No file from escher: the check leaves in an answer and the caller keeps it.**
- The recording is held in memory and returned whole. The return is either `stop`'s answer or a new `record`-style
  answer, on the command line's stdout and as an MCP result.
- Escher writes no file, adds no §Occupied Resources entry and needs no new argument if the hand-back is `stop`'s.
- **What widens.** A new exit for typed text, which today crosses in only. The command line's rule «no ending writes
  back anything the caller typed» breaks. Every typed text, a password's included unless F2 masks it, rides stdout and
  the MCP stdio to the client, and from there into whatever the client keeps (a model's transcript included).
  «Owner-only at rest» becomes the caller's concern, not escher's.
- **Dependence:** F2 then decides what of a password crosses that exit. A new verb, if chosen for the hand-back, also
  widens the verb table: the founder's call (arch §Standard Contracts, Driver session).

**D. A file at a path a tool call names (an argument of `start`, on the command line and as an MCP tool argument).**
- The agent chooses where escher writes, per session.
- **What widens.**
  - A write-anywhere primitive given to the MCP client (the model): any new file path the user can create.
  - A new argument kind (a path) in the verb table, `help` and `tools/list`.
  - A new carrier, as in A.
- This is the widest option.

**Widens least:** **A.** It is the narrowest option that meets the entry's «owner-only» and «via CLI or MCP».
- B adds a state-contract change on top of A's carrier.
- C sends typed text out over two ratified carriers and into the client's own records, where escher's owner-only rule
  cannot follow it.
- D adds a write-anywhere argument.

---

## F2 — What a kept check holds of text typed into a masked control (a password or file input)

Typed text into every other control is kept verbatim. It is the text the screen already reads back as that control's
`value` (a textarea and a text input are unmasked by design: a11y history, 2026-10-09-audit-corrections-agent-surfaces).
Only a `type` whose target is masked is at stake.

**A. Kept verbatim, at rest in the owner-only file.**
- A replay types exactly what was typed, so the replay is exact.
- **What widens.** A typed secret at rest for the first time. The security plan's «reaches no log, event or file»
  is broken for the password's own text: amended, not kept.

**B. Replaced by a fixed stand-in; the step marked masked. (Recommended)**
- The kept step holds a fixed placeholder text (one fixed string, the same for every password, never the typed
  text's length) and a closed flag saying the target was a masked control.
- A replay types the placeholder.
- Every kept screen reading of that control is already `MASKED_VALUE` (F3), so the control's own reading replays the
  same.
- **Stated limit.** A behaviour that depends on the secret's content (a "too short" message, a login that accepts only
  the real password) can replay differently. The masked flag lets the next entry («Recorded-check replay») name such
  a step instead of reporting an unexplained change.
- **What widens.** Nothing of the secret: not its text, its length or a digest. The file holds only the fact that a
  masked control was typed into.
- **Why it is recommended.** It keeps the security plan's password rule whole while every session stays recordable,
  and its one limit is named in the check itself.
- **Dependences.**
  - The driver needs to know a `type` target is masked. That is a public reader in dioxus-native-dom beside the
    private one (`snapshot.rs:231-243`): a technical change inside escher's own crate, no boundary.
  - Under F1 C the placeholder, not the secret, is what crosses the new exit.

**C. Refused: a session that types into a masked control keeps no check.**
- The `type` still runs and answers as today. The recording is dropped, and the session's check is never written. Its
  ending says so in a fixed word (F1 decides where that word is read).
- No false red can come from a stand-in, but no session with a login in it can be recorded.
- **What widens.** Nothing of a masked control reaches the file: not even the fact that one was typed into.

**Widens least:** **C.** Nothing of a masked control is kept at all. B keeps one closed flag and a fixed placeholder,
no byte of the secret. A keeps the secret.

---

## F3 — What a kept check holds of the screen each step answered

For the next entry's replay to be «red on behaviour change, green otherwise», a step needs something to compare.
Every reading already holds a password and a file input as `MASKED_VALUE`, and a shot paints a password as bullets.

**A. Each step's answer whole, as the driver wrote it — a shot's PNG included.**
- The snapshot text, the `to_json` line with its diff, `in_view` and `advanced_ms`, and a shot's base64 PNG: screen
  pixels at rest.
- A replay can say exactly what changed and compare pixels.
- **What widens.** Ids, names, values, snapshot text, diffs and pixels in a file for the first time. That breaks the
  kept «no … file carries …» sentence and puts an in-band image on an at-rest carrier. Size grows by up to about 1 MiB
  per shot.

**B. Each step's answer whole except a shot's pixels: a shot step keeps its width and height. (Recommended)**
- As A, but a `screenshot` step keeps whether it was accepted and its dimensions, never its PNG.
- A replay can name what changed for every verb but `screenshot`. A shot is checked as having run at the same size;
  pixel comparison stays out of this version.
- **What widens.** Ids, names, values (masked as on every carrier), snapshot text and diffs in the owner-only file.
  This is the same content the ratified socket and stdio already carry, now at rest. Pixels never are.
- **Why it is recommended.** A replay that turns red has to say what changed, or an agent cannot act on it. The
  content is what the same owner's session already shows them, and pixels — the largest content, and the one a
  masked password is painted in — stay out of the file.

**C. Each step's answer as a digest only (a fixed 64-bit hash, hand-written: no new dependency).**
- No screen content at rest. A replay detects any change of an answer but cannot say what changed: to see it, an agent
  re-runs the session live.
- **What widens.** No id, name, value or pixel reaches the file. The hash of each answer does, a closed fixed-width
  word.

**D. No answer kept: the calls and whether each was accepted, nothing else.**
- A replay is red only when a step is refused where it was accepted, or the reverse. A changed value, name or
  screen goes unnoticed, so v020-04's «failing when the UI's behaviour changes» is met only for refusals.
- **What widens:** nothing of the screen.

**Widens least:** **D**, at the cost of a replay that misses every change but a refusal. **C** is the least-widening
option that still detects every changed answer.

---

## Dependences across the three
- F1 C turns F2 and F3 from «what is at rest» into «what crosses a new exit to the client». The same option letters
  then apply to that exit.
- F2 B relies on F3 keeping the masked control's own readings as `MASKED_VALUE`, which every F3 option does, since
  the readings come from the snapshot.
- F3 C or D leaves the next entry's replay with less to report. Its red stays a red, but without «what changed».
- The technical forks that follow these answers — the file's form, where the recorder sits, whether the in-process
  round trip is this chunk's — are the operator's, asked in the dialog once these three are answered.

## Answers (the operator, in the founder's stead — a delegate's answers, PROVISIONAL)
_Appended here verbatim when they arrive; then this file is snapped (`--message-file`, `--kind relay`) and cited
`inputs#I{n}` where each answer stands._

The operator's message, 2026-10-19 (received 03:12Z), verbatim:

> Founder forks, answered in his stead (delegate operator, PROVISIONAL), choosing the least widening that still meets
> the entry, because these are boundary widenings and he is away: F1 A (opt-in directory named only by the launcher,
> 0600 in 0700). F2 B (a fixed stand-in plus the masked flag: no byte, length or digest of the secret). F3 C (each
> answer as the fixed 64-bit digest only: no id, name, value or pixel at rest; a red still turns red). His word may
> widen F3 to B later, and the plan should say so. Record them and go on to the technical forks.

- F1 → **A** — delegate the operator, PROVISIONAL.
- F2 → **B** — delegate the operator, PROVISIONAL.
- F3 → **C** — delegate the operator, PROVISIONAL; the founder's own word may widen it to **B** later.
