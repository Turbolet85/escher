proposals:
  - detector: D-arch-resources
    severity: escalate
    section: §Standard Contracts → contract-driver-session
    change: >-
      The recording bullet: each record after the opening one is `tool` · `arguments` in the verb's schema order · `masked` (`true`, on a `type` alone) · `accepted` · `digest` · `answer` — `answer` the kept line inlined as its own JSON object (`record.json("answer", &kept_line(answer))`): the line `Outcome::to_json` or `Refusal::to_json` writes (private `written`), masked as on every carrier, a shot's written by `json::shot_without_image(width, height)` so `{"width":…,"height":…}` with no `png` (crate-private `kept_line`), computed once by `Recording::record`, whose sole caller is still `Session::run`; «no screen reading is kept» becomes «each answer is kept whole, a shot's pixels excepted» (the founder's F3 B); `answer_digest` unchanged (FNV-1a 64 over the whole written line, a shot's PNG included); `Check::from_bytes`' closed keys add `answer`, required and a JSON object — missing, a string, number, array or `null` refused `Cause::Malformed`, key order not enforced as for every key — its bounds (400 records, 1,048,576 bytes) unchanged; `Step` holds the answer in a private field read by `pub(crate) fn answer`, no public accessor returning a `serde_json` type, so `Step::{call, accepted, digest, masked}` stays the public reader (packages/escher-driver/src/recording.rs:47-52; packages/escher-driver/src/recording.rs:55-60; packages/escher-driver/src/recording.rs:108-113; packages/escher-driver/src/recording.rs:134-136; packages/escher-driver/src/recording.rs:317; packages/escher-driver/src/recording.rs:385-388; packages/escher-driver/src/recording.rs:412-423; packages/escher-driver/src/recording.rs:443).
    sidecar: >-
      contract-driver-session: a kept record gains `answer` (the answer's kept line, a shot's without `png`), required as a JSON object by `Check::from_bytes`; «no screen reading is kept» retired (F3 B).
    rationale: >-
      Report Changes → Symbols / APIs (the record's new `answer` key; the reader) and Schema / config (the kept-check record grammar, F3 B, inputs#I1); Counts → patterns `record-keys` and `reading` name architecture.md:186 ×2 stale (the key list ends at `digest`; «no screen reading is kept»). An unregistered key of a stated grammar is drift; the content at rest is the founder's ratified widening, so it rides the widening card.
    basis: architecture.md:186
  - detector: D-arch-resources
    severity: escalate
    section: §Established Decisions
    change: >-
      The recording decision: the calls as they crossed, a masked or unresolved `type` as one fixed stand-in, and each answer kept whole beside its 64-bit digest — the answer's line as the driver wrote it, masked as on every carrier, a shot's without its pixels (`{"width":…,"height":…}`) — so a kept check holds the screens' ids, names, values, snapshot text and diffs at rest (the founder's F3 B), never logged and never named by a tool call; the «owed at «Recorded-check replay»» clause is discharged by 2026-10-19-recorded-check-replay, which also replays a kept check (`Check::replay`, through `Session::run` alone).
    sidecar: >-
      Established Decisions: the recording decision keeps each answer whole (a shot's pixels excepted) beside its digest; «digest alone and no screen reading» and the owed clause retired.
    rationale: >-
      Report Counts → patterns `reading`, `at-rest`, `owner-title` and `replay` each name architecture.md:119 stale («each answer as a 64-bit digest alone and no screen reading», «owed at «Recorded-check replay»»); Schema / config states F3 B landed.
    basis: architecture.md:119
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: §Standard Contracts → Dioxus DOM bridge
    change: >-
      The snapshot-text exits: «nor does a kept check, which holds a recorded session's calls and its answers' digests alone» becomes: no log or event carries the snapshot or its text, while since 2026-10-19-recorded-check-replay a kept check holds it at rest — each kept step's `answer` keeps its answer's line whole, a `snapshot`'s text and an acting verb's diff included, a shot's pixels excepted (F3 B) — in an owner-only file never logged, the two committed checks under `tests/blitz-tests/checks/` the stated exception.
    sidecar: >-
      Dioxus DOM bridge: a kept check now holds the snapshot text and diffs in its `answer`s; «answers' digests alone» retired.
    rationale: >-
      Report Counts → pattern `reading` names architecture.md:192 @c21891 stale; the same retired claim as the record contract's «no screen reading is kept».
    basis: architecture.md:192
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: >-
      The recorded-session clause: `stand_session_recording` — still 3 tests — its first renamed `a_recorded_session_keeps_each_answer_but_a_shots_image_and_replays`: each record ends with its kept answer line rebuilt in the test (a shot's `{"width","height"}`, no `png` and no shot's first 64 base64 characters), the four lean tasks' scripts ending `snapshot` · `screenshot`, replayed through `Check::replay` in both layout modes — the shot path's kept form covered in process on each OS — in place of «read back and run again with every step's acceptance and digest equal».
    sidecar: >-
      blitz-tests: `stand_session_recording` asserts each kept answer and replays through `Check::replay`; «acceptance and digest equal» retired.
    rationale: >-
      Report Counts → `stand_session_recording` (renamed, assertions reversed; pattern `reading`), Changes → Removed (`session_common::replay` gone, its two call sites now `Check::replay`), Spec claims disproved 3 and Addendum inputs#I5 (b): the masters record `stand_session_recording` as the shot path's coverage on each OS.
    basis: architecture.md:333
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → contract-driver-session
    change: >-
      Add the replay: `impl Check { pub fn replay(&self, session: &mut Session) -> Replay }` — a session whose `label()` is not the check's `app()` runs nothing; otherwise each step's `call()` runs through `Session::run`, so it is validated again, and the run stops at the first step that differs — its acceptance differs, or the live answer's kept line, parsed by `serde_json::from_str` (read only), is not equal to the kept `answer` (private `change_of`); `Replay` — `holds()`, `another_app()`, `ran()`, `change() -> Option<&Change>` — and `Change` — `step()`, `accepted_then()`, `accepted_now()`, `keys() -> &[&'static str]`, the step verb's result fields that differ in table order, then `refused` (private `REFUSED`), a key no table holds not named; neither holds a reading or a `serde_json` type, both derive `Debug, Clone, PartialEq, Eq`; re-exported at the crate root; the module opens no span — each step leaves its own `command` span (packages/escher-driver/src/replay.rs:21-28; packages/escher-driver/src/replay.rs:30-38; packages/escher-driver/src/replay.rs:40-70; packages/escher-driver/src/replay.rs:72-93; packages/escher-driver/src/replay.rs:95-115; packages/escher-driver/src/replay.rs:117-139; packages/escher-driver/src/lib.rs:127; packages/escher-driver/src/lib.rs:144).
    sidecar: >-
      contract-driver-session: new public API `Check::replay` · `Replay` · `Change`, each kept call run again through `Session::run`, red at the first differing step.
    rationale: >-
      Report Changes → Symbols / APIs (the replay — new public API in `replay.rs`) and Crates / modules (two public types); Expected amendments names §Standard Contracts → Driver session for `Check::replay` · `Replay` · `Change` and the re-exports; pattern `stand-replay`/`replay` shows no arch site states it yet.
    basis: packages/escher-driver/src/replay.rs:40-70
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      «the crate re-exports 49 names» becomes 51, the list adding, since 2026-10-19-recorded-check-replay, the replay's `Replay` and `Change` from a private module (packages/escher-driver/src/lib.rs:144).
    sidecar: >-
      Driver session: re-exports 49 → 51 names (`Replay`, `Change`).
    rationale: >-
      Report Changes → Symbols / APIs: `pub use replay::{Change, Replay};` (lib.rs:144); the stated re-export count and list at architecture.md:182 omit them.
    basis: packages/escher-driver/src/lib.rs:144
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → escher-driver
    change: >-
      «fifteen private modules» becomes sixteen, adding since 2026-10-19-recorded-check-replay replay (`Check::replay`, `Replay`, `Change` — a kept check's calls run again through `Session::run`, stopping at the first step whose acceptance or kept answer differs), and recording's description gaining the kept answer line (crate-private `kept_line`, the record's `answer`) (packages/escher-driver/src/lib.rs:127).
    sidecar: >-
      escher-driver scope: fifteen → sixteen private modules (`replay`); recording keeps each answer's kept line.
    rationale: >-
      Report Changes → Crates / modules: escher-driver gains the private module `replay` and two public types; architecture.md:328 counts and lists fifteen modules.
    basis: architecture.md:328
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Headless stand (seven_guis, native only)
    change: >-
      «`TaskShell` staying private» becomes `pub fn TaskShell` with one doc line since 2026-10-19-recorded-check-replay, otherwise unchanged — the shell `stand_replay`'s test-local variant `Counter` mounts in, so its unkeyed ids read `TaskShell/Counter/…` (examples/seven_guis/src/app.rs:162; examples/seven_guis/src/app.rs:164).
    sidecar: >-
      Headless stand: `TaskShell` private → `pub fn TaskShell`.
    rationale: >-
      Report Changes → Symbols / APIs (seven_guis: `pub fn TaskShell`, it was private) and Counts → pattern `taskshell` names architecture.md:180 @c1632 stale; Deviations 1 (the variant must be `Counter` under the task's own shell for its ids).
    basis: architecture.md:180
  - detector: D-arch-resources
    severity: escalate
    section: §Occupied Resources → res-filesystem
    change: >-
      Add the committed checks: `tests/blitz-tests/checks/counter-cli.json` (2,929 B) and `counter-mcp.json` (3,720 B), git mode `100644` in the public repository — the stated exception to owner-only: two kept checks of the repository's own scripted stand-Counter sessions (CLI `start counter --record` · `snapshot` · `click counter-increment` · `advance 600` · `click counter-increment` · `snapshot` · `stop`; MCP `initialize` · `notifications/initialized` · `start counter` · `snapshot` · `press tab` ×2 · `press enter` · `press space` · `advance 100` · `scroll counter-increment` · `snapshot` · `stop`), holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls — no typed text, host path, name or secret, and no `screenshot` step (a shot's PNG bytes differ between linux x86_64 and macOS aarch64, CI#39225375577, and the checks are compared byte for byte); written by `examples/seven_guis/tests/flows/record_checks.sh` (POSIX `sh`, the binary and an existing output directory, making `rec/` with `mkdir -m 700` — `s` the state directory, `c` and `m` the recording directories — and printing only `step N` on failure), read by `stand_replay` through `Check::from_bytes`, kept fresh by seven_guis `recording`'s `the_committed_checks_are_what_the_binary_records`, uploaded by no leg — the founder's F1 A (inputs#I2).
    sidecar: >-
      res-filesystem: `tests/blitz-tests/checks/` registered — two committed kept checks in git (`100644`), written by `record_checks.sh`, no shot step (F1 A).
    rationale: >-
      Report Changes → Files (new), Schema / config (two committed checks in a public repository, F1 A, inputs#I2; the recording script), Reverted / negative API facts (the screenshot step dropped, f15c28fc); Expected amendments names §Occupied Resources → Filesystem `tests/blitz-tests/checks/`; pattern `checks-dir` 0 master sites — an unregistered resource.
    basis: examples/seven_guis/tests/flows/record_checks.sh:21-26
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Directory structure
    change: >-
      The tree's `tests/blitz-tests/` line gains `checks/` — the two committed kept checks (`counter-cli.json`, `counter-mcp.json`) `stand_replay` reads.
    sidecar: >-
      Directory structure: `tests/blitz-tests/checks/` added.
    rationale: >-
      Report Changes → Files (new `tests/blitz-tests/checks/counter-{cli,mcp}.json`); Expected amendments names §Infrastructure Patterns; pattern `checks-dir` 0 master sites.
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem
    change: >-
      «beside the recording checks' scratch directories» adds, since 2026-10-19-recorded-check-replay, `rc-fresh` under `CARGO_TARGET_TMPDIR` — seven_guis `recording`'s freshness check, holding the `rec/` (`0700`) that `record_checks.sh` makes — local only and uploaded by no CI leg.
    sidecar: >-
      Filesystem: the freshness check's scratch `rc-fresh` registered beside the recording checks' scratch.
    rationale: >-
      Report Harness / gate surface: the new seven_guis test runs `record_checks.sh` into `CARGO_TARGET_TMPDIR/rc-fresh`; Schema / config: the script makes `rec/` with `mkdir -m 700`; the Filesystem row at architecture.md:212 @c4350 names the session checks' scratch (pattern `sc-readers`).
    basis: architecture.md:212
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: >-
      Add the target `stand_replay` (since 2026-10-19-recorded-check-replay, not unix-gated): each committed check holding on a fresh stand Counter in both layout modes, each red at its first activation step (1 for the CLI check, 3 for the MCP one) on a test-local variant `Counter` that adds 2 with every earlier step held and `changed` among its keys, and a check on a session of another app running nothing — 3 tests, its failure messages holding `{mode} {file}` and a step index alone (tests/blitz-tests/tests/stand_replay.rs:81-110; tests/blitz-tests/tests/stand_replay.rs:206-237; tests/blitz-tests/tests/stand_replay.rs:239-261).
    sidecar: >-
      blitz-tests: new target `stand_replay`, the committed checks replayed (3 tests).
    rationale: >-
      Report Harness / gate surface (new blitz-tests target `stand_replay`, picked up by agent-run, stand-repeat's glob and CI's test leg on three OSes); pattern `stand-replay` 0 master sites — an unregistered target.
    basis: tests/blitz-tests/tests/stand_replay.rs:206-237
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: >-
      The `session_common` clause: «twenty-three readers» becomes twenty-five — 24 measured at the base, the miscount predating this chunk, plus `stand_replay` since 2026-10-19-recorded-check-replay, its name added to the reader list — and the helper list gains `shot_prefix(&mut Session) -> String`, the first 64 base64 characters of a whole-screen shot as the driver writes it, read by `stand_replay` and `stand_session_recording` (tests/blitz-tests/tests/session_common/mod.rs:240-250).
    sidecar: >-
      blitz-tests: `session_common` readers twenty-three → twenty-five (24 at base, + `stand_replay`); helper `shot_prefix` added.
    rationale: >-
      Report Counts → `session_common` readers 24 → 25 (pattern `sc-readers`) and Spec claims disproved 1 (architecture.md:333 @c20107 «twenty-three readers» read 24 at the base); Changes → Symbols / APIs (Added `session_common::shot_prefix`).
    basis: architecture.md:333
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → seven_guis
    change: >-
      `tests/flows/` becomes a directory of three POSIX `sh` scripts — `counter.sh` and `flight.sh`, which `cli_flow` runs, and since 2026-10-19-recorded-check-replay `record_checks.sh`, which `recording`'s `the_committed_checks_are_what_the_binary_records` runs (unix only) to record the two committed checks under `tests/blitz-tests/checks/` and compare them byte for byte — still no target (examples/seven_guis/tests/recording.rs:205-262).
    sidecar: >-
      seven_guis scope: `tests/flows/` holds three scripts, `record_checks.sh` added, run by `recording`.
    rationale: >-
      Report Changes → Files (new `examples/seven_guis/tests/flows/record_checks.sh`) and Harness / gate surface (the new seven_guis test running it through `CARGO_BIN_EXE_escher-session`); the seven_guis row states two scripts.
    basis: examples/seven_guis/tests/recording.rs:205-262
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Process-wide state and threads
    change: >-
      «and `recording` does both, with `--record`» adds: and since 2026-10-19-recorded-check-replay runs `tests/flows/record_checks.sh` under `sh` into `CARGO_TARGET_TMPDIR/rc-fresh`, both streams drained, a `Started` guard, under `run`'s 90 s bound, comparing both recorded files byte for byte with the committed copies (`include_bytes!`).
    sidecar: >-
      Process-wide state: seven_guis `recording` also runs `record_checks.sh` under its bounded run.
    rationale: >-
      Report Harness / gate surface (the new seven_guis test: unix-gated, `run`'s 90 s bound, both streams drained, a `Started` guard); architecture.md:220 states what each seven_guis target spawns.
    basis: architecture.md:220
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: >-
      `stand_screenshot_repeat`'s «a shot of one state byte-identical in two child runs and in the parent» is restated as same-host only: a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577, `Test (macos)`), so a committed check holds no `screenshot` step, while a shot's kept `answer` (width and height) holds across OSes.
    sidecar: >-
      blitz-tests: the shot's byte-identity restated as same-host only (CI#39225375577); a committed check holds no `screenshot` step.
    rationale: >-
      Report Spec claims disproved 2 (architecture.md:333 @c15414 holds on one host only, pattern `shot-os`) and Addendum inputs#I5 (a), which supersedes «no-change, qualifier owed»: the row is stale, restated same-host only with CI#39225375577 cited and the limit beside it.
    basis: architecture.md:333
    dependent-of: D-arch-resources
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions
    change: >-
      The [Driver session] decision's `serde_json` clause: «a reader of MCP requests and kept checks only that writes nothing» becomes a reader of MCP requests, of kept checks and, since 2026-10-19-recorded-check-replay, of the live answer's kept line `Check::replay` compares (`serde_json::from_str`) — still writing nothing, the founder's «serde_json, только чтение» unchanged.
    sidecar: >-
      [Driver session]: `serde_json` also reads the live answer's kept line in the replay — read only, the locked decision held.
    rationale: >-
      Report Changes → Symbols / APIs (private `change_of` parses the live answer's kept line with `serde_json::from_str`, which reads only) and Outcome (arch) MET (`serde_json` called as `from_str` / `from_slice`, never writing); Dependencies none — the locked read-only decision holds, its stated scope of reads widens.
    basis: packages/escher-driver/src/replay.rs:72-93
  - detector: D-arch-decisions
    severity: warning
    section: §Stack and Technologies
    change: >-
      The Media row's serde_json: «taken by escher-driver alone as its reader of MCP requests and of kept checks» adds «and, in the replay, of the live answer's kept line it compares».
    sidecar: >-
      Stack (Media): serde_json's reader scope adds the replay's live answer line.
    rationale: >-
      The same scope claim as the [Driver session] decision's `serde_json` clause, restated in the Stack table; Report Changes → Symbols / APIs (`change_of`, `serde_json::from_str`).
    dependent-of: D-arch-decisions
note: arch §Infrastructure Patterns holds no stand_* target list beyond the directory tree, so the plan's «stand_replay target, record_checks.sh beside the flows» lands in §Existing Scopes (blitz-tests, seven_guis) above; the `session_common` reader list at architecture.md:333 sums to 24 names before this chunk's addition, so the orchestrator should reconcile which listed name accounts for the measured 24 when writing twenty-five.
