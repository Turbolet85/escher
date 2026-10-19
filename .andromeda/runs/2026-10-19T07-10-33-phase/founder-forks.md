# Founder forks — 2026-10-19-recorded-check-replay («Recorded-check replay», v020-04)

Phase P4 of `.andromeda/runs/2026-10-19T07-10-33-phase/`. On the operator's directive (inputs#I1, `relay-1.md`),
every fork that is the founder's is written here: its options whole, its dependences, a marked recommendation, and
the option that widens least named as such. There is one.

**What is already his and settled.** The founder widened F3 to B on 2026-10-19
(`escher-0.2.0/chunks/2026-10-19-session-recording/inputs/I4-relay-1.md.txt`). A kept check now keeps each answer
whole, as the driver wrote it, under an `answer` key after `digest`. It is masked as on every carrier, and a shot keeps
its size but never its pixels. That widening is at rest in the owner-only file of F1 A: `check-{n}.json`, `0600`, in
a `0700` directory the launcher names. This chunk carries it out, and it goes on this chunk's wrap card as ratified.
It is not asked again here.

**Why the fork below is his.** v020-04's acceptance asks for recorded checks to run «as ordinary tests in CI»,
passing on the unchanged app and failing when its behaviour changes, «with no hand edit of the recording». The
natural form is a kept check committed to the repository as test input. But a committed file is not owner-only. It
sits in git as an ordinary file, and it is pushed to `Turbolet85/escher`, which is a **public** repository
(`gh repo view`: `"visibility":"PUBLIC"`). The security plan today keeps every kept check owner-only (§Data
Protection, «Files (kept checks)»). A kept check's content on a medium never ratified is what the playbook's
«Boundary widening» pattern names (verdict `escalate`). So whether a kept check may live in the repository is his
call, not the operator's.

**What research measured** (`escher-0.2.0/chunks/2026-10-19-recorded-check-replay/research.md`):
- A replay target named `stand_*` in `tests/blitz-tests/tests/` is picked up, with no script change, by
  `agent-run.sh run stand` (`scripts/agent-run.sh:181-185`), by the `stand-repeat` leg's glob
  (`.github/scripts/ci-leg.sh:50-52`), and by the CI `test` leg on linux, windows and macos.
- The binary's command line and its `mcp` role already keep byte-identical checks of one script
  (`examples/seven_guis/tests/recording.rs:143-144`). A committed check can therefore be re-recorded by the binary and
  compared byte for byte. That comparison is the proof of «no hand edit».
- The stand's screen readings are already in the public repository as committed evidence: the cold-agent pipe's
  `escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/live-check.json` and two host-path-masked transcripts. They
  hold stand ids, names and values. A stand task's ids, names and fixed text are in its own source.

---

## F1 — Where the checks that CI replays come from: committed to the repository, or recorded inside each test run

Either way, these things are the same:
- The `answer` key (F3 B) lands in the owner-only recorder.
- Replay runs every kept call through `Session::run`, which validates first.
- A test-only variant of a stand task proves that a replay reads red when behaviour changes, naming the step and the
  answer keys that differ. A red names only a step index and closed schema words, never what a screen reads.

**A. Two checks committed to the repository, written by the binary and never edited. (Recommended)**
- **What is committed.** One check recorded through the command line (`escher-session start <task> --record`) and one
  through the `mcp` role (`escher-session mcp --record`). Each is a short scripted session on a lean stand task.
  Both are committed byte for byte as the binary wrote them, under `tests/blitz-tests/checks/`.
  - **What they hold:** the calls (ids, keys, `advance` ms, and only the script's own synthetic typed text) and each
    answer whole (ids, names, values, snapshot text and diffs of the stand's own screens). A shot keeps no pixels.
  - **What they never hold:** a password's text, a user's session, or a host path. The scripts type into no masked
    control, and any masked `type` keeps the fixed stand-in.
- **What runs.**
  - A `stand_replay` target replays both checks on freshly booted stand sessions in both layout modes. It reads green
    while the stand holds.
  - The target reads red, at the step that changed, against the variant.
  - A seven_guis process check records the same two scripts through the binary again and compares the bytes with the
    committed files. That proves they are current and unedited.
- **What it buys.** These are real regression checks, matching the entry's «failing when the UI's behaviour changes».
  A later chunk that changes a lean task's behaviour turns them red until someone deliberately re-records them, and
  the red says which step changed and which keys differ.
- **What widens.** A kept check's content gets a new at-rest medium: a world-readable file in a public git
  repository, not `0600` in `0700`. The content is the stand's own fixed screens plus a test script's synthetic text,
  the same class already committed as cold-agent evidence. The rule «every kept check is owner-only» then gains a
  stated exception: a check the repository itself records of its own stand, as test input. A check a launcher
  records of any other session stays owner-only and is never committed.
- **Why it is recommended.** It is the only option where a replay can turn red because the code changed between two
  commits, which is v020-04's purpose. What it puts at rest is already public: the stand's source and its committed
  evidence. No user content can reach it, because the committed files are only ever the two scripted stand sessions,
  and a byte-equality check refuses any other content.
- **Dependences.**
  - Arch §Occupied Resources gains the `tests/blitz-tests/checks/` registration.
  - The security plan's §Data Protection «Files (kept checks)» and obs-plan §9's «Recording directory» row gain the
    committed-check exception.
  - Cross-OS determinism is unmeasured. The CI run on windows and macos is its first reading. If a reading such as a
    sub-pixel bound differs by OS, the committed scripts keep to steps that hold everywhere, and the limit is recorded.

**B. No committed check: each test records its session at run time, then replays it.**
- Each replay test drives a scripted session with recording on, into an owner-only directory under `target/tmp/`
  (`0700` / `0600`, as today). The CLI case runs through the binary (seven_guis). The MCP case runs through the `mcp`
  role or `serve_mcp` in process. The test then reads the check back, replays it on a fresh session (green) and on
  the variant (red).
- **What widens:** nothing. Every kept check stays owner-only and local, as the founder's F1 A set it. Nothing new
  rests in the repository.
- **What it costs.**
  - The recording and its replay always run on the same code, so a real behaviour change between commits re-records
    itself and stays green. Only the in-test variant ever reads red.
  - v020-04 is then met in form (recorded and replayed in CI, red on a changed app) but not in its use (a check that
    catches a regression across commits). The CLI half can run under agent-run only if it is recorded in process,
    since blitz-tests cannot run the binary.

**Widens least:** **B**. Nothing new is at rest anywhere.
- **A** adds one bounded exception: the repository's own two scripted stand checks, committed and public.
- **A** is the least-widening option that still makes a replay red on a real change between commits.

---

## Dependences on the technical forks that follow (the operator's, asked in the dialog once F1 is answered)
- **T1, the replay runner.** A public `escher-driver` API, or a test-only helper. Under A or B alike.
- **T2, the variant's mount.** `TaskShell` made `pub` in seven_guis (a visibility change, no markup), or the shell's
  markup replicated in the test. Under A or B alike.
- **Where the committed checks live** (`tests/blitz-tests/checks/`) and the freshness check apply under A only.

## Answer
_Appended here verbatim when it arrives; then this file is snapped (`--message-file`, `--kind relay`) and cited
`inputs#I{n}` where the answer stands._

The operator's message, 2026-10-19, verbatim:

> The founder answered F1 by dialog in the overseer session (2026-10-19 08:04Z), relayed by the overseer:
> «Коммитить две проверки (Рекомендую)», that is option A. Record it as his own word (not PROVISIONAL), append it to
> founder-forks.md, snapshot it, and go on to T1 and T2 in your dialog.

- F1 → **A** — the founder's own word, «Коммитить две проверки (Рекомендую)» (the founder, 2026-10-19 08:04Z, by
  question dialog in the overseer session, relayed by the overseer). Ratified by the founder (2026-10-19); not
  PROVISIONAL.
