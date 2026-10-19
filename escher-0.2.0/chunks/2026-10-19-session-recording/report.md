# Report — 2026-10-19-session-recording

**Chunk:** Session recording — a driver session via CLI or MCP kept as an owner-only replayable check, never logged;
password handling per security-plan
**Date:** 2026-10-19
**Commits:** since `last_wrap` 2026-10-18T21:27:25Z, the chunk's base `93336e886` (the parent of the oldest pre-CI
commit, `git log --format='%h %s' 93336e886..HEAD`):
- `9da7b75dd` chore(2026-10-19-session-recording): operator pre-CI commit, for the run this chunk's verdict reads
- `00a6c26bf` fix(2026-10-19-session-recording): operator fix after CI run 39199277925, gate the recorded mcp_log check
  to unix and pin the off-unix refusal
- `3cbb0eeef` fix(2026-10-19-session-recording): operator fix after CI run 39201074261, gate the_check to unix beside
  the mode it calls

## Changes (structured — detectors read this)

- **Files:** 28 source and CI files, per `gate.py scope` (`scope: clean — changed 28 · listed 28 · recorded 0`,
  base `93336e88`) and `cites.py added` (`new text: 28 file(s) · 3 new · 154 added range(s)`).
  - **New:** `packages/escher-driver/src/recording.rs`, `tests/blitz-tests/tests/stand_session_recording.rs`,
    `examples/seven_guis/tests/recording.rs`.
  - **Modified:**
    - `packages/escher-driver/src/{cli,client,command,describe,error,execute,host,json,lib,mcp,mcp_tests,session,wire}.rs`
      and `packages/escher-driver/src/mcp/result.rs`
    - `packages/dioxus-native-dom/src/{lib,snapshot}.rs`
    - `packages/blitz-dom/src/iframe.rs`
    - `tests/blitz-tests/tests/{session_common/mod,stand_session_lifecycle,virtual_clock_engine}.rs`
    - `examples/seven_guis/tests/{cli_commands,host_log,mcp_log}.rs`
    - `.github/scripts/ci-leg.sh`, `.github/scripts/test_ci_workflows.py`
  - **Chunk folder:** `scope.md`, `research.md`, `plan.md`, `inputs/` (I1–I4), `evidence/operator-pass.md`, this
    report.

- **Symbols / APIs** (new text is cited from `## New text, by line` below):
  - **`escher_driver::Session::recording_into(self, dir: &Path) -> Result<Session, SessionError>`** (new, public).
    - It creates `dir` with mode `0700` (its parent must exist), or refuses `SessionError::RecordDirNotPrivate` when
      the directory is present and open to its group or others.
    - It then creates `check-{n}.json` with mode `0600` and `create_new`, `n` the smallest from 1 naming no file. It
      never replaces a file.
    - The file holds a JSON array: the opening record `{"tool":"start","arguments":{"app":<label>}}`, then one
      record per call.
    - The check is rewritten whole after every call, through `.check-{n}.json.tmp` (`0600`) and a rename.
    - Off unix it returns `SessionError::Unsupported`.
  - **`Session::recording_failed(&mut self) -> bool`** (new, public) reads true once after a recording ended.
  - **`Session::run`** (signature, `command` span and its eight fields unchanged) now records each answered call
    when a recording is held.
    - A record is `{"tool","arguments",["masked":true],"accepted","digest"}`.
    - Arguments are written in the schema's argument order, not the order they crossed. That order makes the CLI's
      and MCP's checks byte-identical, and it is the order `validate` checks them in.
    - A call no record can spell is not recorded: a verb outside the seven instance verbs, or an argument the
      verb's schema does not name or passes twice.
    - A `type` keeps `MASKED_STAND_IN` and `masked:true` when its element resolves to a masked control, or resolves
      to none (refused `stale` or `not-found`). Otherwise its text is kept as it crossed.
    - When a write fails or a bound would be passed (400 records, 1,048,576 bytes), the recording ends: the check is
      removed, nothing further is recorded, and `recording_failed()` reads true. The call ran and is not rolled back.
    - A file the recorder did not create, such as a pre-existing temporary at its path, is never removed.
  - **`escher_driver::{Check, Step, answer_digest, MASKED_STAND_IN}`** (new public re-exports).
    - `answer_digest(&Result<Outcome, Refusal>) -> u64` is FNV-1a 64, hand-written (offset basis
      `0xcbf29ce484222325`, prime `0x100000001b3`), over `Outcome::to_json()` or `Refusal::to_json()`. It is
      written as 16 lowercase hex digits.
    - `MASKED_STAND_IN = "masked"`.
    - `Check::from_bytes(&[u8]) -> Result<Check, Refusal>` is a strict reader: at most 1,048,576 bytes, a JSON array
      of at most 400 records, closed keys, the seven instance verbs only, argument names from the verb's schema,
      `masked` true on a `type` alone, `digest` 16 lowercase hex.
      - Anything else is refused `Cause::Malformed` with no fault, holding nothing of the file and with no panic
        path.
      - It reads with `serde_json` and builds each `Call` through `command::call_of`.
    - `Check::{app, steps}` and `Step::{call, accepted, digest, masked}` read a check.
  - **`dioxus_native_dom::is_masked_input(&ElementData) -> bool`** (new, public): an `input` whose `type` is
    `password` or `file`, ASCII case-insensitive. The private snapshot `value` reader now calls it. No snapshot
    reading changed.
  - **`command::call_of`** moved from `mcp/result.rs` (`pub(super)`) to `command.rs` (`pub(crate)`), unchanged in
    behaviour. The MCP server and `Check::from_bytes` share it.
  - **`SessionError::RecordDirNotPrivate`** (kind `record-dir-not-private`) and **`SessionError::RecordingFailed`**
    (kind `recording-failed`): new variants with fixed meaning and remedy, both exit `3`.
    - `SESSION_ERRORS` grows 13 → 15.
    - `describe()`'s `session_errors` serves 15. So `help`'s line changed by the two entries; the verb table and
      `tools/list` are unchanged.
  - **The wire** moves `v2` → `v3` with one new fixed reply, `ok v3 recording-failed`
    (`Reply::RecordingFailed`).
    - The host replies with it in place of the answer when `recording_failed()` reads true after a call.
    - The client reads it as `SessionError::RecordingFailed`.
    - No other request, reply or bound changed.
  - **The MCP server:** after a held session's `run`, a `recording_failed()` reading true makes the result
    `ToolResult::failed(SessionError::RecordingFailed)` (`isError`). The `tool_call` span keeps `tool` and `ending`,
    and `serve_mcp`'s signature is unchanged.
  - **The command line:** `--record <dir>` is taken at most once, on `start <app> --session <dir> --record <dir>`,
    on the host role `serve <app> --session <dir> --record <dir>` (in that order) and on the `mcp` role
    `mcp --record <dir>`.
    - Anywhere else, given twice or with no value, it is usage (exit 2) once the call passed `validate`. A refused
      call stays a refusal. Before this chunk a stray `--record` was an unnamed argument (exit 1).
    - `start` checks the recording directory itself before it spawns, then passes `--record` to the host. A
      refusal there is exit 3 with nothing spawned.
    - `mcp --record` checks the directory before it serves. A refusal ends with exit 3, its fixed message on
      stderr and stdout empty.
    - The usage line reads `start <app> [--record <dir>]`,
      `host role: serve <app> --session <dir> [--record <dir>]` and `mcp role: mcp [--record <dir>]`.
    - Nothing the caller typed is written back.
  - **Remaining callers:** `serve` and `serve_mcp` keep their signatures, and all 12 call sites stand (research §Graph
    impact). `command_line` keeps its signature and its one caller (`examples/seven_guis/src/session_host.rs`).
  - **Ports, sockets, env vars:** none added. The new resource is a FILESYSTEM location named by the launcher's
    `--record`: one `0600` file per session in a `0700` directory, never removed by escher.
    - Escher-driver's unit tests write under the workspace `target/tmp` through the compile-time
      `CARGO_MANIFEST_DIR`, an env read already registered. No `TMPDIR` read was added.

- **Crates / modules:** escher-driver gains the private module `recording` (14 → 15 private modules). No crate was
  added or removed.

- **Dependencies:** none added or bumped. The digest is hand-written, and `Cargo.toml` and `Cargo.lock` are
  unchanged (`ci-leg.sh audit` green).

- **Schema / config:**
  - The kept-check form, a JSON array:
    - opening record `{"tool":"start","arguments":{"app":…}}`
    - later records keyed `tool` · `arguments` · `masked` (optional, `type` alone) · `accepted` · `digest`, in that
      order
  - Written by `json::Object`, which gained a signed `integer` writer.
  - Bounds 400 records and 1,048,576 bytes (crate-private `MAX_CHECK_STEPS`, `MAX_CHECK_BYTES`).
  - Wire `v3`; 15 session errors.
  - The founder may widen F3 to B — each answer kept whole except a shot's pixels — in «Recorded-check replay»
    (inputs#I4). The form leaves room for an `answer` key after `digest`, which the reader refuses until then.

- **Spec-master edits:** none. No master or key file was changed by this chunk before this wrap.

- **Retired:** none.

- **Counts / qualifiers moved** (every count is the `standing` figure of its pattern's line in
  `.andromeda/runs/2026-10-19T05-20-24-wrap/cascade-2026-10-19-session-recording-pre-pass.json`; each pattern is in
  `report-patterns.toml`, the sweep `cascade.py sweep --pre-pass` at baseline `3cbb0eee`):
  - **`SESSION_ERRORS` 13 → 15** [serr]: 3 standing · 3 changed (architecture.md:175 @c5732 `[ErrorSpec; 13]` and
    @c14311 `13 session errors`; security-plan.md:357; test-plan.md:144 «the thirteen session errors») · 0 no-change.
    Leaf 3, for P2's cascade.
  - **Wire `v2` → `v3`, with the `recording-failed` reply** [wire]: 10 standing · 10 changed, each row stating the
    current grammar.
    - The rows: architecture.md:175, :188; security-plan.md:78, :123, :208, :357; test-plan.md:22, :144;
      obs-plan.md:188; `registries/contracts/test-plan/session-lifecycle.md:7`.
    - On those lines, two offsets are dated history and are no-change: security-plan.md:78 @c3606 «the wire staying
      `v2`» and test-plan.md:22 @c8570 «the `v2` wire's rows at … driver-cli».
    - Leaf 5.
  - **escher-driver private modules 14 → 15** [modules]: 3 standing.
    - architecture.md:300 «fourteen private modules» → changed.
    - obs-plan.md:70 @c2990 «the private modules `command`, `refusal`, `schema` and `execute` install no subscriber,
      read no env var and no clock, and print nothing» → extend: `recording` holds no tracing site, env read or
      print. Its tests read the compile-time `CARGO_MANIFEST_DIR` alone.
    - architecture.md:175 @c573 «`describe` from private modules» → no-change, a true claim sharing the token.
  - **blitz-tests `.rs` files 134 → 135, test targets 133 → 134** [btfiles]: 27 standing · 1 changed
    (test-plan.md:67 @c2428 «134 `.rs` files, 133 test targets») · 26 no-change. The 26 are line-range citations
    `…:134-…` sharing the digits.
  - **stand-repeat `stand_*` files 42 → 43** [standf]: 2 standing.
    - test-plan.md:347 «the 42 `stand_*.rs` files and seven_guis' seven process …» → changed (43, and eight).
    - architecture.md:305 @c11572 «`stand_act_states`, 42 tests» → no-change, another fact.
    - Leaf 3.
  - **seven_guis test targets 7 → 8; the stand-repeat seven_guis list gains `--test recording`** [sgtargets]:
    4 standing · 4 changed (architecture.md:183 target list; architecture.md:192 «seven seven_guis test targets»;
    test-plan.md:10 @c5993/@c6090, the leg's command; test-plan.md:318 list). Leaf 2.
  - **`session_common` readers 23 → 24, and its readers `replay` · `kept` · `mode`** [readers]: 15 standing.
    - 3 changed:
      - test-plan.md:67 @c1090 «by 23 checks»
      - test-plan.md:102 @c2393 «23 readers»
      - architecture.md:305 @c19440 «read through `mod session_common;` by four of the session checks» → five, with
        `stand_session_recording`
    - 1 extend: test-plan.md:102 @c43, the module's stated helpers, gains `replay`, `kept` and the unix-only `mode`.
    - 11 no-change:
      - coordinate citations of `session_common/mod.rs`, which P2's citation sweep re-points: architecture.md:191,
        :192, :305 @c20282/@c20333; security-plan.md:318; test-plan.md:27, :50; obs-plan.md:339;
        `session-lifecycle.md:8`
      - architecture.md:129, a convention
      - obs-plan.md:74 and :75, which name other readers
    - Leaf 7.
  - **`serde_json` "a reader of MCP requests only" → it also reads kept checks, and still writes nothing**
    [serde]: 10 standing.
    - 3 changed:
      - architecture.md:115 @c2418 «a reader of MCP requests only that writes nothing» → stale
      - architecture.md:175 @c25200 «reads MCP requests and writes nothing» → extend
      - obs-plan.md:70 @c2708 «`serde_json` reading requests and writing nothing» → extend
    - 7 no-change: architecture.md:57, :63, :196, :300; security-plan.md:83 (the MCP server's row); security-plan.md:251;
      a11y-plan.md:14 — dependency facts, unchanged.
    - Leaf 7.
  - **The usage line and the role shapes gain `[--record <dir>]`** [usage]: 38 standing · 10 changed · 28
    no-change.
    - Changed rows: architecture.md:175 (@c27684, @c29558, @c30037); architecture.md:180 (@c2644, @c2757
      «`mcp` alone»); security-plan.md:24; security-plan.md:82 (@c398, @c3495, @c4435); layout-templates.md:10
      @c1517; layout-templates.md:73 (@c2727, @c4574); test-plan.md:10 @c1250; test-plan.md:104 @c840;
      obs-plan.md:183 (@c1612, @c1637); `session-lifecycle.md:9` (@c645, @c724).
    - The 28 no-change rows name the `mcp` or host role for a fact this chunk did not move.
    - Leaf 27.
  - **«no log, event or file carries an id, a name, a value, the snapshot's text or a diff's content»**
    [nofile]: 2 standing · 2 changed.
    - architecture.md:178 @c21771 and security-plan.md:123 @c6330 are restated: no log or event does, while a kept
      check carries call content — target ids, a `press` key, an `advance`'s ms, unmasked typed text — and no screen
      reading.
    - Their second offsets are no-change: @c26395 and @c9683 «no log, event or file carries a diff» still hold.
  - **At rest — per medium** [atrest]: 1 standing · 1 extend (security-plan.md:187 gains the kept check as a medium).
  - **The state directory** [statedir]: 8 standing · 8 no-change. The check lives in the launcher's recording
    directory, and the state directory still holds `session.sock` alone and is removed at `stop`.
    - Architecture §Occupied Resources gains a sibling row for the recording directory (expected amendment 1).
  - **`virtual_clock_engine` 4 → 5 tests, and its reads widen to pan, the fling and a sub-document's config**
    [vce]: 6 standing.
    - Changed: test-plan.md:27 @c46343 «`virtual_clock_engine` 4».
    - Extend: test-plan.md:16.
    - No-change: test-plan.md:27 @c46845 (a citation); test-plan.md:67; test-plan.md:328 and :334 (dated tallies);
      obs-plan.md:75. The new pan test is silent, so its claim still holds.
    - Leaf 6.
  - **A coverage reading** [cov]: 1 standing · 1 no-change (test-plan.md:346, a dated reading at another tree).
    This chunk read escher-driver 98.26 % (4906/4993).
  - **`call_of`'s module** [callof]: no master site. The sweep refused it: «control: callof, title never fired over
    the seven masters' pre-pass text (3cbb0eee)».
  - **Owner pattern** [title] «Session recording»: control unfired, the same refusal line its basis. [owned]:
    2 standing, both owned by other entries (architecture.md:90 the upstream-sync CARRY; security-plan.md:123
    @c8547) → not this chunk's.
  - **Long lines** — a disposition per (line, fact) pair, every pattern standing on a line over 2,000 chars, per the
    rows and the `by line` block:

    | line | pairs |
    |---|---|
    | architecture.md:115 | serde stale · usage no-change |
    | architecture.md:129 | readers no-change |
    | architecture.md:175 | serr changed · wire changed · serde extend · usage changed (@c27684, @c29558, @c30037), no-change elsewhere · modules no-change |
    | architecture.md:178 | nofile changed @c21771, no-change @c26395 · usage no-change · btfiles no-change |
    | architecture.md:180 | usage changed (@c2644, @c2757), no-change elsewhere |
    | architecture.md:188 | wire changed · statedir no-change · usage no-change |
    | architecture.md:192 | sgtargets changed · readers no-change · usage no-change |
    | architecture.md:305 | standf no-change · btfiles no-change · readers changed @c19440, no-change elsewhere |
    | security-plan.md:78 | wire changed, no-change @c3606 |
    | security-plan.md:82 | usage changed (@c398, @c3495, @c4435), no-change elsewhere |
    | security-plan.md:123 | wire changed · nofile changed @c6330, no-change @c9683 · usage no-change · owned no-change |
    | security-plan.md:124 | usage no-change |
    | security-plan.md:357 | serr changed · wire changed |
    | security-plan.md:403 | usage no-change |
    | layout-templates.md:10 | usage changed @c1517, no-change @c1885 |
    | layout-templates.md:73 | usage changed |
    | test-plan.md:10 | usage changed @c1250, no-change @c4298 · sgtargets changed |
    | test-plan.md:22 | wire changed, no-change @c8570 · usage no-change |
    | test-plan.md:27 | readers no-change · btfiles no-change · vce changed @c46343, no-change @c46845 |
    | test-plan.md:67 | readers changed @c1090, no-change elsewhere · btfiles changed @c2428 · vce no-change |
    | test-plan.md:102 | readers extend @c43, changed @c2393, no-change elsewhere |
    | test-plan.md:104 | usage changed @c840, no-change elsewhere |
    | test-plan.md:144 | serr changed · wire changed |
    | test-plan.md:327 | usage no-change |
    | test-plan.md:328 | vce no-change |
    | test-plan.md:334 | vce no-change |
    | obs-plan.md:70 | modules extend · readers no-change · serde extend · usage no-change |
    | obs-plan.md:183 | usage changed (@c1612, @c1637), no-change elsewhere |
    | `session-lifecycle.md:9` | usage changed (@c645, @c724), no-change @c1840 |

  - **Sweep reach:** 15 patterns · 15 moved facts, 15 with a pattern (the report's own count) · owner pattern control
    unfired · by line 28 lines · 85 offsets. The total is `total (15 patterns) · 201 rows over 27 files`. The leaf
    rows (66) and the base rows (2) are P2 cascade's.

- **Dev-tool versions:**
  - The rustup target `x86_64-pc-windows-gnu` (the Rust standard library for that target, a cross-compile target and
    no binary) was installed on the dev host around 05:02Z on 2026-10-19 by the operator, on the founder's word
    «Поставить (Рекомендую)» (inputs#I4). This session did not install it.
  - `rustup target list --installed` now reads `x86_64-pc-windows-gnu`, `x86_64-unknown-linux-gnu`.
  - No lockfile-resolved crate is this line's subject.

- **Harness / gate surface:**
  - `.github/scripts/ci-leg.sh` stand-repeat: the seven_guis list adds `--test recording`, and the new
    `stand_session_recording` joins by its `stand_` prefix.
  - `.github/scripts/test_ci_workflows.py` pins both words (`recording`, `stand_session_recording`) in
    `test_stand_repeat_leg_runs_three_rounds`.
  - No agent-run or CI workflow step changed.

- **Cross-project / external claims:**
  - **CI runs on the pushed shas** (`ci.py conclusion --sha HEAD --wait 1800`, recorded in `evidence/operator-pass.md`):
    - CI#39199277925 on `9da7b75dd4ec`: **red**. `Test (windows)` failed: the recorded `mcp_log` test was not unix-gated.
    - CI#39201074261 on `00a6c26bf24c`: **red**. `Test (windows)` failed to compile: `the_check` called the
      unix-gated `mode`. It was read while 3 jobs still ran.
    - CI#39202401256 on `3cbb0eeef1d8`: **green**, checks 17/17, wall 1027 s. This is the final verdict. This wrap's
      commit adds to that tree.
  - **Inputs** (`inputs.py verify`: `inputs: 4 entries — … n/a 4 · uncited 1 · unparsed 0`):
    - I1 · `inputs/I1-relay-1.md.txt` · copy message · n/a. The operator's take-up directive; the founder is away,
      his forks are answered by the operator as delegate, PROVISIONAL.
    - I2 · `inputs/I2-founder-forks.md.txt` · copy message · n/a. The delegate answers, PROVISIONAL: F1 A, F2 B, F3 C.
    - I3 · `inputs/I3-relay-3.md.txt` · copy message · n/a. F3 C covers answers only; the calls kept as they crossed
      are the F1/F2 widening, to be listed by name on the wrap's Boundary widening card.
      - Its origin names the P5 card `.andromeda/runs/2026-10-19T02-56-17-phase/p5-card-1.md`, and the fold card
        `.andromeda/runs/2026-10-19T02-56-17-phase/p5-card-2.md` is the newest. `## Beyond the plan` reads `none`
        in both (the operator's word at the P5 review, 2026-10-19 03:29Z).
    - I4 · `inputs/I4-relay-1.md.txt` · copy message · n/a — **cited here**. The founder's word, relayed by the
      operator from the overseer dialog at 05:02Z:
      - «Поставить (Рекомендую)» — the windows-gnu target installed.
      - F1 A and F2 B ratified as his.
      - F3 widens to B in «Recorded-check replay», not in this chunk.
    - No entry reads drifted, vanished or broken.

- **Reverted / negative API facts:**
  - A first `Recording` removed its own `.check-{n}.json.tmp` on any failure, including one it had not created. A
    unit test caught it, and the remove was narrowed to the check alone (`recording.rs`'s `imp::rewrite` removes
    only a temporary it wrote).
  - The seven_guis `recording.rs` lost its file-wide `#![cfg(unix)]` for per-item gates in fix 1, so an off-unix
    check could live beside the unix checks.

- **Insufficient fixes (written, kept, not the remedy):**
  - Fix 1 (`00a6c26bf`): its `mcp_log` gate and its off-unix tests are correct and kept.
  - It did not make the Windows leg green: its own de-gating left `the_check` calling the unix-only `mode`.
  - Fix 2 (`3cbb0eeef`) is the remedy, and nothing is left over (CI#39202401256 green).

- **Spec claims disproved by measurement:**
  - **Plan step 8:** «The verb table, `help`'s line and `tools/list` are byte-identical to before the chunk». Step 5's
    15 session errors are served by `describe()`, so `help`'s line gains two entries; this is pinned in
    `describe.rs`'s count row `[11, 13, 12, 3, 8, 4, 15]`.
    - The acceptance's own reading holds: `help` and `tools/list` are byte-identical with and without `--record`
      (`examples/seven_guis/tests/recording.rs`).
    - No master site: a plan claim. → P2 Validate check 6, disposed against the plan.
  - **The plan's touchpoints note:** «under F1 A … `stand_session_quiet.rs` and `stand_session_lifecycle.rs` stay
    unchanged». `stand_session_lifecycle.rs` pinned raw `call v2 …` grammar lines, and the first gate block went
    red on 4 entries with one cause (`stand_session_lifecycle.rs:135`).
    - Its grammar rows moved to `v3`, and `call v2 snapshot` now reads `refused version`.
    - No master site: a plan claim, its fact carried by [wire].
  - **The folded CARRY's hypothesis:** «a recorded check that scrolls replays only as deterministically as those
    reads» (`escher-0.2.0/working-route.md:27`). It is false for a driver-recorded check, which no verb pans or
    flings, and the stand holds no iframe.
    - Recorded `premise-corrected` at scope P3, and measured here: the round trip replays the `scroll` digest on
      every lean task.
    - No master site: a route CARRY, which P7's compaction archives.

- **Expected amendments (from plan):** 7 entries (`plan.md` §Implementation notes). Every search is the pre-pass
  sweep above.
  1. **architecture §Occupied Resources → Filesystem — the recording directory** (named by `--record`, `0700`,
     `check-{n}.json` `0600`, never removed by escher), a «Boundary widening»: carried by **Symbols / APIs**
     (Ports, sockets, env vars) and **Schema / config**.
     - Search [statedir]: architecture.md 3 standing rows (:115, :188, :191); its sibling row lands beside :188, the
       socket's row.
     - The founder ratified F1 A himself (inputs#I4), so P2's card cites his word.
  2. **architecture §Standard Contracts → Driver session** — `recording_into` · `recording_failed`, `Check` / `Step` /
     `answer_digest` / `MASKED_STAND_IN`, wire `v3` and `RecordingFailed`, 15 session errors, `--record` on `start` ·
     `serve` · `mcp`: carried by **Symbols / APIs** and **Counts / qualifiers moved**.
     - Search: [serr] architecture.md:175; [wire] :175, :188; [usage] :175, :180; [serde] :115, :175; [modules] :300;
       [nofile] :178.
  3. **security-plan §Data Protection (At rest) and §Input Validation** — the CLI-arguments row, the check-reader
     row, the password row, and the 2026-10-10 «no log, event or file carries…» sentence restated. Carried by
     **Symbols / APIs**, **Schema / config** and **Counts / qualifiers moved**.
     - Search: [atrest] security-plan.md:187; [nofile] :123; [wire] :78, :123, :208, :357; [usage] :24, :82;
       [serr] :357.
     - The founder card lists BY NAME the call-content class at rest — call target ids, the `press` key, the
       `advance` ms, text typed into unmasked inputs (inputs#I3). Per inputs#I4 that class is ratified with F1 A and
       F2 B.
  4. **obs-plan §9 recording-directory row; §4 both spans unchanged with recording on:** carried by **Symbols /
     APIs** and **Outcome** (`host_log` / `mcp_log` recorded runs).
     - Search: [statedir] obs-plan.md:339 (the state-directory row, sibling site); [wire] :188; [serde] :70;
       [modules] :70.
  5. **test-plan §3 Session lifecycle** (`session-wire` `v3`, `session-binary` `--record` and the two errors,
     `session-proof` the round trip) **and §10** (stand-repeat's list and the counts): carried by **Counts / qualifiers
     moved** and **Harness / gate surface**.
     - Search: [wire] test-plan.md:22, :144 and `session-lifecycle.md:7`; [serr] :144; [sgtargets] :10, :318;
       [standf] :347; [btfiles] :67; [readers] :67, :102; [vce] :16, :27; [usage] `session-lifecycle.md:9`.
  6. **layout-templates §Surface: cli → Primary screens — `--record` on three surfaces:** carried by **Symbols /
     APIs** (the command line).
     - The three: `start`, the host role, and the `mcp` role, where it is the role's first argument.
     - Search: [usage] layout-templates.md:10, :73.
  7. **design-system §Surface: cli §Component Patterns — the two session errors in the exit-3 class, no new exit:**
     carried by **Symbols / APIs**.
     - Search: [serr] design-system.md 0 standing; [usage] design-system.md:396 (the `mcp` role's surface kind), 1
       standing. This is the one site the sweep reached in that master, and the exit-3 class sentence is not
       reached by these patterns.
     - The design detector's own sweep is the control.

- **Coverage of new surfaces:**
  - `--record <dir>` on `start` · `serve` · `mcp` → validation closed argv reader + directory mode check✓ ·
    instrumentation n/a (never logged by contract; spans unchanged, `host_log` / `mcp_log`) · PII n/a (a path, never
    written back or logged) · tests unit (`cli.rs` rows) / integ (`examples/seven_guis/tests/recording.rs`,
    `cli_commands.rs`) · a11y n/a · tokens n/a
  - The kept check `check-{n}.json`, a new at-rest medium → validation writer bounds 400 / 1,048,576✓ ·
    instrumentation n/a (0 log sites; recording-needle checks in `host_log` / `mcp_log`) · PII: masked text the
    stand-in✓, no screen reading✓, call content raw by the ratified F1/F2 · tests unit (`recording.rs` 9 + off-unix 1)
    / integ (`stand_session_recording` 3, seven_guis `recording` 3 + off-unix 1) · a11y n/a · tokens n/a
  - `Check::from_bytes`, untrusted input → validation strict closed grammar, bounded, `malformed` with nothing held,
    no panic path✓ (35 refusal rows, edge rows at both bounds) · instrumentation n/a · PII n/a · tests unit · a11y n/a
    · tokens n/a
  - Wire `ok v3 recording-failed` and MCP `isError` `recording-failed` → validation fixed word✓ · instrumentation
    `tool_call` `ending=error`✓ (standing) · PII n/a · tests unit (`wire.rs`, `client.rs`, `mcp_tests.rs`) · a11y n/a
    · tokens n/a
  - `is_masked_input` → validation n/a · instrumentation n/a · PII masks✓ · tests unit (dioxus-native-dom 7 rows) ·
    a11y n/a · tokens n/a

## Deviations from intent
1. **`stand_session_lifecycle.rs` was edited**, though the plan said it stays unchanged: wire `v2` literals → `v3`.
   Research's Files-to-modify list carries the file.
2. **Arguments are written in schema order:** the CLI and MCP must be byte-identical, and MCP's map order differs
   from argv's.
3. **Calls no record can spell are not kept:** each is refused before it runs, and the reader's grammar cannot hold
   it.
4. **Recording is unix-only:** off unix it reads `Unsupported`, matching the session socket.
5. **The recorder never removes a file it did not create.** A unit test found the opposite and the code was fixed.
6. **`mcp --record <open dir>` ends as a protocol role does:** exit 3, its message on stderr, stdout empty.
7. **`--record` anywhere else is usage** (exit 2), after the call passes `validate`.
8. **Unit-test temp dirs** sit under the workspace `target/tmp` via `CARGO_MANIFEST_DIR`. No new env read.
9. **`MASKED_STAND_IN = "masked"`:** non-empty, so a replayed `type` gives the same masked diff.
10. **Unchanged after all:** `schema_tests.rs` and `examples/seven_guis/tests/common/mod.rs`. dioxus-native-dom gained
    one unit test.
11. **A `type` whose element never resolves is kept masked** (surfaced at implement as S2). The plan masks only «a
    `type` whose target was masked».
    - Off a resolved control, the target cannot be read. Keeping the stand-in protects a possible password, and
      replay fidelity holds because a refusal holds nothing of the text.
    - A `type` refused by `validate` keeps its text, since a `text` out of bound decides that refusal.
    - → P2: the security-plan password row, and the founder card's F2 text.
12. **`help`'s line changed** (surfaced as S1, Spec claims disproved).
13. **The operator pass added off-unix tests** the plan did not list, on the operator's direction after CI#39199277925:
    - `recording.rs` `off_unix_tests`
    - `session.rs` `off_unix_a_recording_is_refused_as_unsupported`
    - `mcp_tests.rs` `off_unix_a_recording_boot_answers_start_as_unsupported`
    - seven_guis `recording.rs` `off_unix_a_recording_directory_is_refused_as_unsupported`
    - the `#[cfg(unix)]` gates on `mcp_log`'s recorded test, `recorded_needles` and `the_check`

    The direction: «Add a test that off unix --record refuses with Unsupported, if one does not exist yet». All lie in
    research's lists.

scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 28 · listed 28 · recorded 0`).

## Decisions & corrections
- **The founder's word (inputs#I4), relayed by the operator from the overseer dialog at 05:02Z.**
  - «Поставить (Рекомендую)»: the `x86_64-pc-windows-gnu` target installed on the dev host.
  - Session recording F1 A and F2 B are ratified as his.
  - F3 widens to B in «Recorded-check replay», not in this chunk.
  - → P2: the PROVISIONAL marks on F1 A, F2 B and the call-content class discharge on his word, through a halt card
    citing it, per the 2026-10-10 Tier 1 rule; F3 C's mark is the card's question.
  - → P5: a `CARRY:` on «Recorded-check replay», on his word: F3 widens to B there — each answer kept whole as the
    driver wrote it, a shot's pixels excepted, as an `answer` key after `digest`.
- **The operator's direction for the operator pass** (2026-10-19): «Run the operator pass … each entry by hand in plan
  order … Stop and tell me if any entry is red; start no wrap.» The pass stopped twice on red, each time fixed only on
  the operator's next word.
- **The operator's direction after the first red:** «Gate that test and its helper with #[cfg(unix)]. Before
  committing, sweep every test this chunk added or changed … for a recording path not gated to unix … Add a test that
  off unix --record refuses with Unsupported».
  - The sweep found one gap, `mcp_log`.
  - Fix 1's de-gating then introduced a second break, `the_check` → `mode`.
- **The operator's direction after the second red:** «Do NOT add the Windows target: no host toolchain change while
  the founder is away … list each pair you checked in operator-pass.md».
  - The audit listed every gated item and each naming item, and found one break, now fixed (`evidence/operator-pass.md`
    §Fix round 2).
  - It was superseded for the target by the founder's word (inputs#I4). The windows-gnu `cargo check --tests` then
    built both packages, exit 0.
- **Learning candidate: code compiled only off unix is unverified on this host until a non-unix check runs.**
  - Measured: two consecutive Windows CI reds (CI#39199277925, CI#39201074261).
  - The first was an ungated new test. The second was a file moved from a file-wide `#![cfg(unix)]` to per-item gates
    that left a helper calling a gated item. `cfg_attr(…, allow(dead_code))` silences an unused item but cannot excuse
    a call to one configured out.
  - The remedy now available: `cargo check --tests --locked --target x86_64-pc-windows-gnu -p {crate}` (31.77 s,
    exit 0 on both packages).
- **Learning candidate: `gh api …/actions/jobs/{id}/logs` refuses a log holding escape sequences unless given
  `--allow-escape-sequences`.**
  - Measured: the first fetch's file held only «the response contains terminal escape sequences; pass
    --allow-escape-sequences to output it anyway». A grep over it read 0 failures, an empty result that was not an
    absence.
  - `ci.py`'s own `log:` line prints the flag.
- **Sweep hazard:** the bare number `\b134\b` matched 26 line-range citations (`…:134-…`) for the one file-count site.
  A count of files or targets is swept by its noun-bound phrase («134 `.rs` files»), never the bare digits.
- **Guard refusals** (`bash-guard.sh` / `bash-guard.py`), each re-issued once:
  - `cd` into a chunk or skill directory ×4 — a recurrence of host-linux «Paths & the working directory»
  - a `cat` heredoc append to `iframe.rs` (→ the Edit tool)
  - a recursive `rm` of a smoke dir (the host's deny rule → a fresh unique dir; `target/tmp/smk-rec-0408/` is
    left, gitignored)
- **Implement self-corrections** (`.andromeda/runs/2026-10-19T03-31-48Z-implement/outcome-2026-10-19-session-recording.md`):
  - An inline python heredoc nesting raw triple quotes failed with a SyntaxError and changed nothing. It was re-run
    from a scratchpad file.
  - The recorder's foreign-temp removal was caught by a unit test.
- **The delegate frame:**
  - inputs#I1: the founder is away, and the operator answers his forks as delegate, PROVISIONAL.
  - inputs#I2: the delegate answers F1 A, F2 B and F3 C.
  - inputs#I3: the call-content class is the F1/F2 widening, listed by name on the founder card.
- **Implement's surfaced items S1 and S2:** S1 is under Spec claims disproved; S2 is Deviation 11.

## Outcome
**Acceptance criteria**, re-asserted against the diff:
- **(arch)** met.
  - The recording code is in `packages/escher-driver/`, and escher-driver's dependencies and features are
    unchanged (`Cargo.toml` and `Cargo.lock` untouched, audit green).
  - `serde_json` writes nothing: it reads MCP requests and now kept checks, still a reader only.
  - `serve` and `serve_mcp` keep their signatures.
- **(security, owner-only)** met — `stand_session_recording`
  `a_recording_directory_open_to_its_group_is_refused_with_nothing_written`, seven_guis `recording`
  `a_recording_directory_open_to_its_group_is_refused_before_anything_runs`. The smoke read `-rw-------` in
  `drwx------`.
- **(security, password)** met — `a_password_typed_in_a_recorded_session_is_kept_as_the_stand_in`: 0 sentinel
  occurrences, and a secret of another length leaves byte-identical files.
- **(security, no screen reading)** met — `a_recorded_session_replays_and_keeps_no_screen_reading`: 0 names, values
  and lines at rest, each excluding the script's own typed text, which a check keeps as it crossed (inputs#I3).
- **(security, reader)** met — `recording.rs` `a_check_outside_the_grammar_is_refused_as_malformed` (35 rows) and
  the bound rows.
- **(tests, round trip)** met — the four lean tasks, both layout modes, every step's acceptance and digest equal.
- **(tests, CLI = MCP)** met — `the_command_line_and_the_mcp_role_keep_the_same_check`, with the state directory gone
  after `stop`.
- **(obs)** met — `a_recording_host_logs_nothing_of_its_check_at_trace` and
  `a_recording_mcp_role_logs_nothing_of_its_checks_at_trace`: 0 of the recording needles, span counts unchanged.
  The six standing session, mask and span targets are green.
- **(obs, silence)** met — the probe's last line `0`.
- **(layouts / design)** met — listing and `help` identical with and without `--record`; the usage line names
  `--record` on the three (`cli_commands`); no ESC byte.
- **(tests, CARRY)** met — `virtual_clock_engine`
  `a_finger_pan_and_its_fling_move_only_with_the_clock` (3 cases × 2 modes), and blitz-dom `iframe`
  `a_sub_documents_config_carries_its_parents_clock` (the same `Arc`).
- **(a11y)** met — `ci-leg.sh a11y` green.
- **(tests, legs)** met.
  - fast, doc, coverage (escher-driver 98.26 %) and stand-repeat (`3 of 3 rounds green`) are green.
  - CI read `verdict: green` on the pushed sha `3cbb0eeef`, run CI#39202401256, after two reds on earlier shas
    (Cross-project / external claims).

**Gates**, by `run`. The final block run is `.andromeda/runs/2026-10-19T03-31-48Z-implement/gate-2026-10-19-session-recording.json`,
run 2: `entries 17 · green 14 · red 0`.
- `cargo test -p escher-driver --locked` green, exit 0 (87 unit tests after the operator pass)
- `cargo test -p dioxus-native-dom --locked` green
- `cargo test -p blitz-dom --locked --lib iframe` green (1 selected)
- `cargo test -p blitz-tests --locked --test stand_session_recording` green
- `cargo test -p blitz-tests --locked --test virtual_clock_engine` green
- `cargo test -p blitz-tests --locked --test stand_session_quiet … --test stand_act_filled` green; it was red in run 1
  on `stand_session_lifecycle.rs:135`, which was fixed
- `cargo test -p seven_guis --locked` green
- the silence probe (`awk … recording.rs stand_session_recording.rs`) green, `last line 0`
- `bash .github/scripts/ci-leg.sh fast` green
- `bash .github/scripts/ci-leg.sh doc` green
- `bash .github/scripts/ci-leg.sh stand-repeat` green, `contains stand-repeat: 3 of 3 rounds green`
- `bash .github/scripts/ci-leg.sh a11y` green
- `bash .github/scripts/ci-leg.sh coverage` green, `contains coverage floor: 3 of 3 crates at or above their floors`
- `bash .github/scripts/ci-leg.sh audit` green
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (`leg = 'operator'`): green,
  `hygiene: clean`, fired before the pre-CI commit
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
  build/escher-0.2.0` (`leg = 'operator'`): fired three times, each green; the last pushed `00a6c26b→3cbb0eee`
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
  (`leg = 'operator'`): red ×2, then **green** on `3cbb0eeef` (CI#39202401256, 17/17). All three are recorded in
  `evidence/operator-pass.md`.

No `defer`, no `--skip`, no `round` or `live` leg. **Smoke** (the boot path changed: the command line) passed.
- `agent-run.sh run stand_session_recording` 3 passed.
- A real `escher-session start counter … --record …`, then `click`, `snapshot` and `stop`, left one check `0600` in
  `0700`, and the state directory was gone.

**Off-unix compile** (the founder's word, inputs#I4): `cargo check --tests --locked --target x86_64-pc-windows-gnu
-p escher-driver -p seven_guis` exit 0. Its two warnings are outside this chunk: blitz-paint `Intrinsic`, and an
unused import in `host_binary.rs`. It was a check, not a run.

**Watches:** none folded.

**Outcome basis:**
- The operator pass ran, so the verdicts above rest on its final state: Setup 4's commit list (`9da7b75dd`,
  `00a6c26bf`, `3cbb0eeef`) and the final HEAD's CI run CI#39202401256 green, recorded in `evidence/operator-pass.md`.
- This window holds implement's run whole, as its conversation. Its persisted report
  `.andromeda/runs/2026-10-19T03-31-48Z-implement/outcome-2026-10-19-session-recording.md` agrees and is superseded
  only by:
  - the two operator fixes, which added the off-unix tests and gates (Deviation 13)
  - the founder's word inputs#I4
- **Uncommitted at this report:** `evidence/operator-pass.md`'s last two sections (the off-unix compile and the final
  verdict), `inputs/I4-relay-1.md.txt` with its `andromeda-inputs.json` row, and the implement run dir's `relay-1.md`
  and `inputs-…json`. They ride this wrap's commit.

**Process hygiene:**
- Implement's census and this wrap's re-measure (`gate.py census --comm escher-session --comm seven_guis_nati --args
  'serve counter' --args 'serve crud' --args 'escher-session mcp'`) both read `census: none running`.
- The smoke's host (`start` counter) was terminated by `stop`.
- The gate entries' hosts and children were each reaped by their guards.
## Addendum — resumed 2026-10-19
- inputs#I5 · `inputs/I5-relay-1.md.txt` · copy message · n/a — the operator's resume directive (2026-10-19 05:28Z): the
  Boundary widening card cites inputs#I4; F1 A, F2 B and the kept call content are the founder's, and F3 C stands as this
  chunk's interim form on his word, the B widening owed at «Recorded-check replay» by the CARRY; no PROVISIONAL mark for
  these; the wrap stops once on the card, then goes through to the commit
  - bears on **Decisions & corrections**: supersedes «F3 C's mark is the card's question» — F3 C carries no PROVISIONAL
    mark; the card cites inputs#I4 and lists what it covers
- inputs.py verify, re-fired before this paste: `inputs: 5 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 ·
  altered 0 · unreachable 0 · n/a 5 · uncited 1 · unparsed 0` (the uncited one I5, cited by this block); moved since the
  report: an entry added (I5), no verdict moved
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 93336e88 (the parent of the oldest pre-CI commit 9da7b75d) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/ci-leg.sh — added 1 line(s) in 1 range(s)
added: 57
### .github/scripts/test_ci_workflows.py — added 2 line(s) in 1 range(s)
added: 358-359
### examples/seven_guis/tests/cli_commands.rs — added 17 line(s) in 3 range(s)
added: 477 · 483-487 · 502-512
### examples/seven_guis/tests/host_log.rs — added 67 line(s) in 6 range(s)
added: 9-11 · 27-66 · 125 · 127-133 · 135 · 300-314
- 30-34 @32 «fn a_recording_host_logs_nothing_of_its_check_at_trace() {»
- 36-61 @39 «fn recorded_needles(kept: &std::path::Path) -> Vec<String> {»
  - 40-43 «let names: Vec<_> = std::fs::read_dir(kept)»
  - 48-52 «let dir_name = kept»
  - 53-60 «std::iter::once(dir_name)»
  - 131-133 «if record {»
  - 301-304 «assert!(»
  - 305-314 «if record {»
### examples/seven_guis/tests/mcp_log.rs — added 61 line(s) in 4 range(s)
added: 9-12 · 53-95 · 140-142 · 322-332
- 56-64 @59 «fn a_recording_mcp_role_logs_nothing_of_its_checks_at_trace() {»
- 66-91 @69 «fn recorded_needles(kept: &std::path::Path) -> Vec<String> {»
  - 70-73 «let names: Vec<_> = std::fs::read_dir(kept)»
  - 78-82 «let dir_name = kept»
  - 83-90 «std::iter::once(dir_name)»
  - 140-142 «if let Some(kept) = record {»
  - 323-332 @324 «if let Some(kept) = record {»
### examples/seven_guis/tests/recording.rs — new file · 284 line(s)
- 38-46 @40 «fn mode(path: &Path) -> u32 {»
  - 41-45 «fs::metadata(path)»
- 48-53 @49 «fn fresh(name: &str) -> std::path::PathBuf {»
- 55-72 @58 «fn the_check(dir: &Path, surface: &str) -> Vec<u8> {»
  - 59-62 «let names: Vec<_> = fs::read_dir(dir)»
  - 63-67 «assert!(»
- 74-76 «fn request(id: u32, method: &str, params: &str) -> String {»
- 78-84 «fn tool_call(id: u32, name: &str, arguments: &str) -> String {»
  - 79-83 «request(»
- 86-93 «fn mcp(record: Option<&Path>) -> Speaker {»
  - 89-91 «if let Some(dir) = record {»
- 95-105 @97 «fn answered(speaker: &mut Speaker, line: &str, step: &str) -> String {»
  - 100-103 «assert!(»
- 107-178 @109 «fn the_command_line_and_the_mcp_role_keep_the_same_check() {»
  - 113-123 «let started = client(»
  - 125-128 «let _session = Started {»
  - 129-138 «for (step, args) in [»
  - 150-155 «for (id, (step, name, arguments)) in [»
  - 157-164 «.enumerate()»
  - 169-174 «assert!(»
  - 175-177 «for dir in [kept, kept_by_mcp] {»
- 180-220 @182 «fn a_recording_directory_open_to_its_group_is_refused_before_anything_runs() {»
  - 189-199 «let refused = client(»
  - 202-205 «assert!(»
  - 211-214 «assert!(»
  - 215-218 «assert!(»
- 222-247 @224 «fn the_mcp_roles_listing_and_help_are_the_same_with_a_recording_directory() {»
  - 228-237 «for record in [None, Some(kept.as_path())] {»
  - 238-241 «assert!(»
  - 242-245 «assert!(»
- 249-284 @251 «fn off_unix_a_recording_directory_is_refused_as_unsupported() {»
  - 258-262 «assert!(»
  - 266-276 «let started = client(»
  - 278-281 «assert!(»
### packages/blitz-dom/src/iframe.rs — added 28 line(s) in 1 range(s)
added: 186-213
- 187-213 @188 «mod tests {»
  - 194-212 @195 «fn a_sub_documents_config_carries_its_parents_clock() {»
### packages/dioxus-native-dom/src/lib.rs — added 1 line(s) in 1 range(s)
added: 35
### packages/dioxus-native-dom/src/snapshot.rs — added 56 line(s) in 6 range(s)
added: 226-234 · 236 · 240-244 · 246 · 251 · 687-725
- 226-233 @228 «pub fn is_masked_input(element: &ElementData) -> bool {»
  - 229-232 «&*element.name.local == "input"»
  - 687-724 @688 «fn a_password_or_file_input_is_masked_and_nothing_else_is() {»
### packages/escher-driver/src/cli.rs — added 269 line(s) in 40 range(s)
added: 17 · 34-36 · 57-60 · 64-66 · 68-72 · 118-122 · 124 · 127 · 131 · 145 · 148-154 · 161-163 · 169 · 179-180
       188-193 · 197-201 · 205-216 · 221-224 · 234 · 241-244 · 273 · 356-358 · 361-362 · 396-407 · 414-430 · 446-453
       455-460 · 463-467 · 485 · 513-531 · 545-546 · 548-574 · 583-590 · 593-598 · 684-757 · 887 · 959-960 · 962
       1003-1005 · 1026
  - 189-193 «let record = match records.as_slice() {»
- 205-213 @207 «fn record_option(words: &[String]) -> Option<Option<PathBuf>> {»
  - 208-212 «match words {»
- 446-452 @447 «fn recording_into(session: Session, record: Option<&Path>) -> Result<Session, SessionError> {»
  - 448-451 «match record {»
  - 458-460 «if let Some(dir) = record {»
  - 464-466 «if let Some(dir) = record {»
  - 517-525 «fn recorded_start(app: &str) -> Line {»
### packages/escher-driver/src/client.rs — added 5 line(s) in 4 range(s)
added: 77-78 · 106 · 367 · 373
### packages/escher-driver/src/command.rs — added 32 line(s) in 2 range(s)
added: 3-4 · 32-61
- 32-60 @35 «pub(crate) fn call_of(»
  - 40-55 «for (name, value) in arguments {»
  - 56-59 «Ok(Call {»
### packages/escher-driver/src/describe.rs — added 8 line(s) in 2 range(s)
added: 145 · 154-160
### packages/escher-driver/src/error.rs — added 29 line(s) in 9 range(s)
added: 40-44 · 80 · 163-176 · 196-197 · 251-252 · 254 · 278 · 293-294 · 296
  - 163-168 «ErrorSpec {»
  - 169-176 «ErrorSpec {»
### packages/escher-driver/src/execute.rs — added 31 line(s) in 9 range(s)
added: 8 · 131-134 · 141-142 · 144-149 · 153-156 · 171-176 · 308-309 · 339-343 · 349
  - 339-343 «let masked = harness»
### packages/escher-driver/src/host.rs — added 11 line(s) in 2 range(s)
added: 25-27 · 199-206
### packages/escher-driver/src/json.rs — added 7 line(s) in 3 range(s)
added: 84-87 · 705-706 · 708
  - 84-86 «pub(crate) fn integer(&mut self, key: &str, value: i64) {»
### packages/escher-driver/src/lib.rs — added 9 line(s) in 3 range(s)
added: 82-88 · 122 · 138
### packages/escher-driver/src/mcp.rs — added 10 line(s) in 4 range(s)
added: 22 · 35 · 66-68 · 225-229
### packages/escher-driver/src/mcp/result.rs — added 1 line(s) in 1 range(s)
added: 8
### packages/escher-driver/src/mcp_tests.rs — added 103 line(s) in 1 range(s)
added: 587-689
- 587-600 @589 «fn off_unix_a_recording_boot_answers_start_as_unsupported() {»
  - 593-599 «assert_eq!(»
- 602-610 @604 «fn scratch(name: &str) -> std::path::PathBuf {»
  - 605-606 «let dir = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/tmp"))»
- 612-656 @614 «fn each_session_a_recording_boot_starts_is_kept_as_one_check() {»
  - 619-628 «let sent = [»
  - 632-635 «assert_eq!(»
  - 640-644 «let verbs: Vec<&str> = first»
- 658-688 @660 «fn a_call_that_ends_the_recording_is_answered_as_its_session_error() {»
  - 664-667 «std::fs::set_permissions(»
  - 671-675 «let sent = [»
  - 678-681 «assert_eq!(»
### packages/escher-driver/src/recording.rs — new file · 901 line(s)
- 36-47 @41 «pub fn answer_digest(answer: &Result<Outcome, Refusal>) -> u64 {»
  - 42-45 «let written = match answer {»
- 49-53 «fn fnv1a(bytes: &[u8]) -> u64 {»
  - 50-52 «bytes.iter().fold(FNV_OFFSET_BASIS, |hash, byte| {»
- 55-58 @56 «fn hex(digest: u64) -> String {»
- 60-68 @61 «fn opening(app: &str) -> String {»
- 70-74 @71 «const START_APP: &str = match schema::START.args {»
- 76-90 @78 «fn kept_verb(call: &Call) -> Option<&'static VerbSpec> {»
  - 80-88 «for (index, (name, _)) in call.args.iter().enumerate() {»
- 92-118 @94 «fn record_of(verb: &VerbSpec, call: &Call, masked: bool, accepted: bool, digest: u64) -> String {»
  - 96-108 «for arg in verb.args {»
  - 112-114 «if masked {»
- 120-123 @121 «fn check_text(records: &[String]) -> String {»
- 125-130 @126 «pub(crate) struct Recording {»
- 132-176 «impl Recording {»
  - 133-144 @135 «pub(crate) fn create(dir: &Path, app: &str) -> Result<Recording, SessionError> {»
  - 146-175 @150 «pub(crate) fn record(»
- 178-183 @181 «pub(crate) fn prepare_dir(dir: &Path) -> Result<(), SessionError> {»
- 185-262 @186 «mod imp {»
  - 194-196 «fn io_error(error: io::Error) -> SessionError {»
  - 198-211 «pub(super) fn prepare_dir(dir: &Path) -> Result<(), SessionError> {»
  - 213-220 @214 «fn create_new(path: &Path) -> io::Result<fs::File> {»
  - 222-243 @224 «pub(super) fn create_check(»
  - 245-257 @248 «pub(super) fn rewrite(file: &Path, temporary: &Path, text: &[u8]) -> io::Result<()> {»
  - 259-261 «pub(super) fn remove(file: &Path) {»
- 264-287 @265 «mod imp {»
  - 271-273 «pub(super) fn prepare_dir(_dir: &Path) -> Result<(), SessionError> {»
  - 275-280 «pub(super) fn create_check(»
  - 282-284 «pub(super) fn rewrite(_file: &Path, _temporary: &Path, _text: &[u8]) -> io::Result<()> {»
- 289-294 @291 «pub struct Check {»
- 296-303 @298 «pub struct Step {»
- 305-346 «impl Check {»
  - 306-335 @316 «pub fn from_bytes(bytes: &[u8]) -> Result<Check, Refusal> {»
  - 337-340 @338 «pub fn app(&self) -> &str {»
  - 342-345 @343 «pub fn steps(&self) -> &[Step] {»
- 348-368 «impl Step {»
  - 349-352 @350 «pub fn call(&self) -> &Call {»
  - 354-357 @355 «pub fn accepted(&self) -> bool {»
  - 359-362 @360 «pub fn digest(&self) -> u64 {»
  - 364-367 @365 «pub fn masked(&self) -> bool {»
- 370-379 @371 «fn object_of<'a>(value: &'a Value, keys: &[&str]) -> Option<&'a Map<String, Value>> {»
  - 372-374 «let Value::Object(object) = value else {»
  - 375-378 «object»
- 381-389 «fn opening_app(record: &Value) -> Option<String> {»
  - 383-385 «if record.get("tool")?.as_str()? != schema::START.name {»
- 391-421 «fn step_of(record: &Value) -> Option<Step> {»
  - 392-395 «let record = object_of(»
  - 396-397 «let verb =»
  - 401-405 «let masked = match record.get("masked") {»
  - 408-411 «let is_hex = digest.len() == 16»
  - 412-414 «if !is_hex {»
  - 415-420 «Some(Step {»
- 423-437 @424 «mod off_unix_tests {»
  - 427-436 @428 «fn off_unix_a_recording_is_refused_as_unsupported() {»
- 439-901 @440 «mod tests {»
  - 448-456 @449 «fn scratch(name: &str) -> PathBuf {»
  - 458-460 «fn text(value: &str) -> ArgValue {»
  - 462-470 «fn call(verb: &str, args: &[(&str, ArgValue)]) -> Call {»
  - 472-482 «fn acted() -> Result<Outcome, Refusal> {»
  - 484-490 «fn mode(path: &Path) -> u32 {»
  - 492-519 @493 «fn the_digest_is_fnv_1a_64_of_the_written_answer() {»
  - 521-591 @522 «fn a_recording_reads_back_as_the_calls_it_kept() {»
  - 593-635 @594 «fn a_masked_type_keeps_the_stand_in_and_nothing_of_the_text() {»
  - 637-658 @638 «fn a_call_no_record_can_spell_is_not_kept() {»
  - 660-704 @661 «fn the_check_and_its_directory_are_owner_only_and_no_file_is_replaced() {»
  - 706-756 @707 «fn a_recording_that_would_pass_its_bound_or_cannot_write_ends_and_removes_its_check() {»
  - 759-760 «const CLICK: &str =»
  - 762-764 «fn check(records: &[&str]) -> Vec<u8> {»
  - 766-794 @767 «fn a_check_in_the_grammar_is_read() {»
  - 796-871 @797 «fn a_check_outside_the_grammar_is_refused_as_malformed() {»
  - 873-900 @874 «fn a_check_at_its_bounds_is_read_and_one_past_them_is_refused() {»
### packages/escher-driver/src/session.rs — added 110 line(s) in 7 range(s)
added: 4 · 10 · 89-93 · 99-100 · 122-123 · 127-156 · 265-333
  - 127-149 @146 «pub fn recording_into(mut self, dir: &Path) -> Result<Session, SessionError> {»
  - 151-155 @153 «pub fn recording_failed(&mut self) -> bool {»
  - 266-271 «fn empty() -> Session {»
  - 273-282 @275 «fn scratch(name: &str) -> std::path::PathBuf {»
  - 284-293 @286 «fn off_unix_a_recording_is_refused_as_unsupported() {»
  - 295-308 @297 «fn a_recording_directory_open_to_others_is_refused_with_nothing_written() {»
  - 310-333 @312 «fn a_recording_that_ends_reads_failed_once() {»
### packages/escher-driver/src/wire.rs — added 92 line(s) in 34 range(s)
added: 34 · 37 · 61-62 · 246 · 274 · 352-355 · 375 · 379 · 418 · 420-422 · 425 · 429 · 441-443 · 452-453 · 459-461
       464-465 · 467 · 469 · 471 · 473-474 · 476-477 · 479-481 · 483-491 · 493-497 · 499-504 · 506-510 · 525 · 529-530
       532-551 · 557 · 566-567 · 588 · 599 · 734
### tests/blitz-tests/tests/session_common/mod.rs — added 35 line(s) in 2 range(s)
added: 20 · 240-273
- 240-251 @242 «pub fn replay(session: &mut Session, check: &Check) -> Vec<(bool, u64)> {»
  - 243-250 «check»
- 253-260 @254 «pub fn kept(check: &Check) -> Vec<(bool, u64)> {»
  - 255-259 «check»
- 262-272 @264 «pub fn mode(path: &Path) -> u32 {»
  - 267-271 «fs::metadata(path)»
### tests/blitz-tests/tests/stand_session_lifecycle.rs — added 11 line(s) in 3 range(s)
added: 126 · 128-130 · 140-146
  - 140-146 «for request in [&b"call v1 snapshot\n"[..], &b"call v2 snapshot\n"[..]] {»
### tests/blitz-tests/tests/stand_session_recording.rs — new file · 343 line(s)
- 28-31 «use session_common::{»
- 40-75 @42 «fn script(task: LeanTask) -> Vec<Call> {»
  - 43-72 «let mut calls = match task {»
- 77-82 @78 «fn fresh(name: &str) -> PathBuf {»
- 84-93 @86 «fn the_check(dir: &Path) -> Vec<u8> {»
  - 87-90 «let names: Vec<_> = fs::read_dir(dir)»
- 95-112 @96 «fn escaped(text: &str) -> String {»
  - 98-110 «for character in text.chars() {»
- 114-124 @115 «fn sent(calls: &[Call]) -> Vec<String> {»
  - 116-123 «calls»
- 126-238 @127 «fn a_recorded_session_replays_and_keeps_no_screen_reading() {»
  - 128-237 «for incremental in [false, true] {»
- 240-247 @241 «fn password_fixture() -> Element {»
  - 242-246 «rsx! {»
- 249-257 «fn hold_fixture(incremental: bool) -> Session {»
  - 250-255 «Session::start("masked", || {»
- 259-272 @261 «fn record_typed(dir: &Path, secret: &str, incremental: bool) -> Vec<(bool, u64)> {»
  - 262-264 «let mut session = hold_fixture(incremental)»
  - 265-271 «[type_into("fx-secret", secret), snapshot()]»
- 274-323 @275 «fn a_password_typed_in_a_recorded_session_is_kept_as_the_stand_in() {»
  - 276-322 «for incremental in [false, true] {»
- 325-343 @326 «fn a_recording_directory_open_to_its_group_is_refused_with_nothing_written() {»
  - 331-337 «assert!(»
  - 338-341 «assert!(»
### tests/blitz-tests/tests/virtual_clock_engine.rs — added 74 line(s) in 2 range(s)
added: 8-13 · 221-288
- 221-230 @222 «fn harness_offset_y(harness: &Harness<HtmlDocument>) -> f64 {»
  - 224-229 «harness»
- 232-287 @233 «fn a_finger_pan_and_its_fling_move_only_with_the_clock() {»
  - 239-286 «for incremental in [false, true] {»
