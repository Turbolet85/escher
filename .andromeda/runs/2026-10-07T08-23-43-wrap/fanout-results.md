# Fan-out results — 2026-10-07-sink-target-allowlist

Seven doc-agents, one parallel batch, 2026-10-07. Detector ids over the seven prompts: 15, the drift-base's count
(architecture 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2).
Each return was taken from the agent's transcript by script, entity-decoded and probed: `entities=0` before and after
on all seven. No raw twin is kept: no return failed the parse or the probe, and the two `proposals: []` returns were
changed by stripping only in losing their trailing comment lines, whose substance is in the verdict lines below.

## Verdicts
- **architecture** — 8 proposals (D-arch-resources 8; D-arch-decisions: no drift).
- **security-plan** — 8 proposals (D-security-input 6 · D-security-auth 2; D-security-deps: no drift). The agent
  states all three invariants HOLD and files the restatements under the nearest detector at severity `warning`.
- **design-system** — `proposals: []`. Stripped: the report's only new surface reads `tokens n/a`; no UI rendered.
- **layout-templates** — `proposals: []`. Stripped: no new surface or region; and the carried question answered — the
  `escher-session` entry's stderr sentence ("its stderr log lines carry `service.name=seven_guis`") names no id, level
  or count and is not stale; the doc cites none of the three moved files.
- **test-plan** — 10 proposals (D-tests-obs-harness 3 · D-tests-coverage 7; D-tests-framework: no drift).
- **obs-plan** — 13 proposals (D-obs-pii 6, graded `escalate` by the detector's own severity · D-obs-stack 7;
  D-obs-instrumentation: no drift). The agent states the D-obs-pii invariant HOLDS — the chunk adds no logging of user
  data and narrows what prints.
- **a11y-plan** — 1 proposal (D-a11y-obs-schema; D-a11y-surface: no drift).

Total: 40 proposals.

## Validation — the six checks
1. **Playbook.** All 40 match "Accurate this-chunk addition" (routine): each names a fact the report's Changes carry
   as this chunk's work — the drop, the new public set, the two new test targets and their shared module, the by-level
   readings, the moved counts and coordinates — landing in an existing section, with the detector's invariant holding.
   None is a boundary widening (the chunk narrows what crosses the sink), none touches a PROVISIONAL mark, and no two
   rules collide. The six `escalate` grades on obs-plan are the detector's severity for a VIOLATION of D-obs-pii; the
   detector itself reports the invariant holding, and four of the six sites are named changes of the plan's
   P5-approved `Expected amendments (wrap)` list (obs-plan §8 Values logged as-is and Scrubbing; the
   `pii-scrubbing-wire` key) — the operator's recorded direction settles them (the operator, 2026-10-07, at the plan
   review). The other two (§6 Log format, two bullets) restate the same drop where §6 says every event prints.
   Disposition: apply, no halt.
   Re-derivation: several proposals carry coordinates the report does not (`format.rs:123-198`, `lib.rs:22-23`,
   `host_log.rs:11`, `host_log.rs:1-4`, `common/mod.rs:1-4`, `telemetry_drop.rs:14-22`, `lib.rs:38-60`, `lib.rs:142-150`
   and others). None was taken from a proposal: every coordinate written into a body was measured by the orchestrator
   against the tree, and all 56 citations into the chunk's six files were resolved afterwards (0 suspect).
2. **Cross-contradiction.** None. Three security-plan proposals and two obs-plan proposals edit one bullet each in
   compatible parts (the rule, the readings, the `log.file` residual).
3. **Intent-consistency.** The report does not diverge from the route entry or the plan's acceptance criteria; the
   scope record holds no line (`scope: clean`, 0 recorded).
4. **Absence needs evidence.** The agents' "no other occurrence" notes were not relied on: the caught-all claim is
   the cascade sweep's (`cascade-dispositions.md`), run over the masters, key files, leaves, curation homes and
   judgment bases after the last edit. Line profile known before any hit was dispositioned: the amended lines run to
   7,434 chars (`security-plan.md:115`), each read whole or by an offset window.
5. **Expected amendments (10 entries).** obs-plan §8 — proposed (O1-O3) · obs-plan `pii-scrubbing-wire` — proposed
   (O4) · obs-plan §3 Logging stack and §9 Session state directory — proposed (O7, O8) · security-plan
   `logging-redaction-wire` — proposed (S6) · security-plan §Logging & Monitoring and the two §Input Validation rows —
   proposed (S1-S5) · architecture, four sections — proposed (A1, A2, A4, A5) · test-plan §3, §5, §1 and the §9 count
   chain — proposed (T1, T2, T4, T5, T8); the §3 Proof link NOT applied: `run stand` is unmoved at 72 · 0 · 3 and the
   chain carries no link for an unmoved count — the fact is in the report and the sidecar's Kept · a11y-plan §3 —
   proposed (Y1) · layout-templates — conditional entry, condition false (the sentence is not stale): no change ·
   citations 7 · 16 · 8 — all proposed and applied, one of the eight unmoved (`host_binary.rs:1-3`).
6. **Disproved claims.** The report lists none falsified and three superseded statements; each site is matched by a
   proposal — "as written" (security-plan S3, test-plan T1, obs-plan O1; architecture's two hits are another sense),
   "not measured by level" (S1, S2, S3, T2, O1), "owed by the route entry" (S3, S6, T2, O3, O4). The unit note on
   ×12 / ×20 is disposed by S1 and O1, which retire the figures.

Escalations: 0.

## Dispositions
Numbering is the order of each doc's list below. Every `apply` is check 1's routine verdict; the applied text was
re-derived from the report and the site as read, not pasted from the `change` line.

- **architecture** — A1 Telemetry bootstrap: apply · A2 Process-wide state and threads: apply · A3 Filesystem: apply ·
  A4 Logging and timing: apply · A5 Existing Scopes → escher-telemetry: apply · A6 Existing Scopes → seven_guis:
  apply · A7 Environment variables (citation): apply · A8 Feature gating (citation): apply.
- **security-plan** — S1 `id` row: apply · S2 accessible-names row: apply · S3 the sink's readings and the "owed by"
  clause: apply · S4 the sink's rule and its citation: apply · S5 the `log.file` residual: apply, written as
  "recorded by construction, not measured" (the proposal's own caveat; the report states `log.file` was not counted) ·
  S6 logging-redaction-wire: apply · S7 `RUST_LOG` citation: apply · S8 the session checks' env citation: apply.
- **test-plan** — T1 Stand log format: apply · T2 Session host ↔ what it writes: apply · T3 Process-lifecycle checks:
  apply · T4 escher-telemetry row: apply · T5 tests/blitz-tests process-telemetry clause: apply · T6 apps row: apply ·
  T7 Directory pattern: apply · T8 Local baseline link: apply · T9 lifecycle-socket citation: apply · T10 key Session
  lifecycle → session-proof: apply (the key file; `registry.py check` 0 defects).
- **obs-plan** — O1 Values logged as-is: apply · O2 Scrubbing, the rule: apply · O3 Scrubbing, the reach: apply ·
  O4 key Bootstrap phases → pii-scrubbing-wire: apply (the key file; `registry.py check` 0 defects) · O5 Log format,
  first bullet: apply · O6 Log format, levels: apply · O7 Logging stack inventory: apply · O8 Session state directory:
  apply · O9-O13 citations (§3 Logging stack, §1, §2, §3 Service identity, §6 Logged events): apply, O10 with its
  one-phrase restatement of the formatter.
- **a11y-plan** — Y1 the §3 note's sink sentence: apply.
- **Raised by the orchestrator at the cascade's sweep (routine, the same claim at sites no proposal named):**
  `security-plan.md:176` §Data Protection · `test-plan.md:102` and `a11y-plan.md:87`, the "per event" duplicate.

Applied: 40 proposals + 3 folds = 43 edits' worth over five masters and two key files, recorded in five sidecar
entries (one per amended doc). Rejected: 0. Escalated: 0.

## The parsed lists
### architecture

```yaml
# Detector results — architecture, chunk 2026-10-07-sink-target-allowlist
# D-arch-decisions: NO drift. Report Dependencies = "none added, none bumped"; no new library/runtime; no §Established Decisions
#   entry speaks to the sink (only [Default features] names the `tracing` feature and [Driver session] says the driver carries no
#   tracing dependency — both untouched). The drop sits in the existing formatter on the existing Registry + EnvFilter + fmt layer.
# D-arch-resources: drift at 3 primary sites + 5 dependent sites (below).
# Sweep notes for the orchestrator:
#   - The report's "architecture `as written` 2" are BOTH another sense of the phrase, not the sink claim:
#     architecture.md:121 ("a node dropped since it was written", changed-set contract) and :136 ("something was written since the
#     last drain", snapshot diff). No proposal for either. Architecture never states "prints ... as written" for outside targets;
#     the retired claim lives in it as the Scrub clause's "a field in `CONTENT_FIELDS` is redacted at any target" (:132).
#   - "`unscrubbed` 3" (:149 x2, :193 x1) describe `target/agent-run/run.log`, `target/cold-agent/transcript.jsonl` and the upstream
#     `tracing_subscriber::fmt::init()` stdout installs — none is the escher sink, all still true. No proposal.
#   - `stand_session_quiet` x3 (:150 x2, :263 x1) and the §Existing Scopes → blitz-tests row: unchanged facts (file under the
#     preservation gate; the row enumerates no telemetry_* file, so `telemetry_drop` falsifies nothing there). No proposal.
#   - `examples/seven_guis/tests/host_binary.rs:1-3` (:264) did not move — the module doc is still lines 1-3.
#   - What changed is taken from the report's Changes only. Source files were opened solely to turn the report's old → new
#     table into exact new coordinates for ranges the table does not list verbatim (format.rs:96-165, :96-126, :17-84; lib.rs:18-19).
#   - Citation census matches the report: format.rs 3 (:132 x2, :193), lib.rs 8 (:111, :132, :150 x2, :153, :193, :256 x2),
#     host_binary.rs 3 (:149, :150, :264). All 13 moved ones are covered below; the 14th (:264) is unmoved.
proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Telemetry bootstrap (escher-telemetry)
    change: >-
      Restate the Scrub clause as a three-outcome target rule and register the new public set — the sink's allowlist of targets is two public sets, `ENGINE_TARGET_PREFIXES` (`blitz` · `dioxus_native` · `stylo_taffy` · `accesskit_xplat` · `debug_timer` · `js_console`, unchanged) and the new `ESCHER_TARGET_PREFIXES` = `["escher_"]` (underscore part of the prefix; fifth public scrub constant beside `SAFE_FIELDS`, `CONTENT_FIELDS`, `REDACTED`); an event whose target starts with a prefix of neither set is dropped whole — zero bytes, no time, target or newline — at every level, WARN and ERROR included, and whatever `RUST_LOG` names, a bridged `log` record judged by the target it was logged under; an engine target prints only `SAFE_FIELDS`, every other field (`message` included) as `{name}=[redacted]`; an escher target prints every field not in `CONTENT_FIELDS` (this replaces "a field in `CONTENT_FIELDS` is redacted at any target"); the line shape is "one per printed event". Re-point the three citations — `lib.rs:30-134` → `lib.rs:34-140`, line shape `format.rs:96-165` → `format.rs:123-198`, scrub `format.rs:17-84` → `format.rs:21-107`.
    sidecar: >-
      2026-10-07-sink-target-allowlist — Telemetry bootstrap contract: registered public `ESCHER_TARGET_PREFIXES`, scrub restated as engine-scrub / escher-redact / outside-target drop at every level; three citations re-pointed.
    rationale: >-
      Report Changes → Symbols / APIs: "NEW public constant `escher_telemetry::ESCHER_TARGET_PREFIXES`" (format.rs:33, re-exported lib.rs:34-36) and "CHANGED behaviour of the sink's formatter" — an outside target "returns before writing anything ... zero bytes ... at every level, WARN and ERROR included"; Schema / config: "the sink's scrub shape gains a third outcome". The contract names only `ENGINE_TARGET_PREFIXES` and still says content fields are redacted "at any target", i.e. every other target prints. Expected amendments names this section. Citation moves per Counts / qualifiers moved.
    basis: >-
      .andromeda/architecture.md:132 · packages/escher-telemetry/src/format.rs:33 (new const), :93-107 (`decide`), :134-138 (the drop), :123-198 (formatter + visitor) · packages/escher-telemetry/src/lib.rs:34-36, :105-140
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Process-wide state and threads
    change: >-
      Register the two new process-spawning checks and re-point three citations — "two blitz-tests integration binaries each re-execute their own test binary once" becomes three, adding `telemetry_drop`, which runs `std::env::current_exe()` with `--ignored --exact child_emits --nocapture`, its output captured and `RUST_LOG=warn,style=trace,dioxus_core=trace,selectors=trace` set on the child, the child an `#[ignore]` test that installs the sink with `init` (test-only, fixed argv, no socket or port) (tests/blitz-tests/tests/telemetry_drop.rs:14-22); and "seven_guis' `host_binary` test spawns the built `escher-session` binary" becomes two seven_guis tests, `host_binary` and `host_log`, each through `CARGO_BIN_EXE_escher-session` — `host_log` on `crud` with `RUST_LOG=trace`, both streams piped and drained from the spawn on by one test-process thread each, reached by `escher_driver::attach` polled under a 60 s bound (never `start`), stopped by `stop`, exit waited under a 10 s bound, the shared kill-and-reap `Host` guard now in `examples/seven_guis/tests/common/mod.rs:23-32` (examples/seven_guis/tests/host_binary.rs:15; examples/seven_guis/tests/host_log.rs:11; examples/seven_guis/tests/host_log.rs:73-87). Citations — `lib.rs:84-85` → `lib.rs:90-91`, `lib.rs:113-132` → `lib.rs:119-138`, `host_binary.rs:11` → `host_binary.rs:15`.
    sidecar: >-
      2026-10-07-sink-target-allowlist — Process-wide state and threads: registered `telemetry_drop`'s re-exec child and `host_log`'s spawned `escher-session` child; three citations re-pointed.
    rationale: >-
      Report Changes → Crates / modules: "Two new integration-test targets — `host_log` in the seven_guis package, `telemetry_drop` in blitz-tests"; Counts: "the seven_guis package now holds two files that spawn the `escher-session` binary (`host_binary`, `host_log`); the blitz-tests telemetry family now holds five files ..., two of them in the re-exec form with one `#[ignore]` child (`telemetry_stdout_silent`, `telemetry_drop`)"; "What the session host check reads" and "What the sink's process check reads" give the argv, env and bounds. The registry lists neither child and still counts two re-exec binaries and one host-spawning seven_guis test. Expected amendments names this section ("one more spawned child of a test").
    basis: >-
      .andromeda/architecture.md:150 · tests/blitz-tests/tests/telemetry_drop.rs:14-22, :69-72 · examples/seven_guis/tests/host_log.rs:11, :73-87 · examples/seven_guis/tests/host_binary.rs:15 · examples/seven_guis/tests/common/mod.rs:23-32 · packages/escher-telemetry/src/lib.rs:90-91, :119-138
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem
    change: >-
      Register the new state directory and re-point the citation — the session checks' own state directories are `ss-life` and `ss-quiet` (blitz-tests) and `hb-serve`, `hb-refuse` and `hl-trace` (seven_guis: `host_binary` ×2, `host_log`) under `CARGO_TARGET_TMPDIR`; the seven_guis citation `examples/seven_guis/tests/host_binary.rs:14-22` becomes `examples/seven_guis/tests/common/mod.rs:12-20` (the `state_dir` and `clear` helpers left `host_binary.rs`) with `examples/seven_guis/tests/host_log.rs:73` for the new name.
    sidecar: >-
      2026-10-07-sink-target-allowlist — Filesystem: registered `host_log`'s state directory `hl-trace` under `target/tmp/`; `state_dir`/`clear` citation moved to `tests/common/mod.rs:12-20`.
    rationale: >-
      Report "What the session host check reads": "spawns `CARGO_BIN_EXE_escher-session` on `crud` with the state directory `hl-trace` under `CARGO_TARGET_TMPDIR`"; Counts / qualifiers moved: "`state_dir` `14-16`, `clear` `19-22` and the `Host` guard `25-34` left the file → `examples/seven_guis/tests/common/mod.rs:12-14`, `:17-20`, `:23-32`". The registry enumerates the session checks' directories and omits `hl-trace`; its cited lines no longer hold the helpers.
    basis: >-
      .andromeda/architecture.md:149 · examples/seven_guis/tests/host_log.rs:73 · examples/seven_guis/tests/common/mod.rs:12-20 · examples/seven_guis/tests/host_binary.rs:26 (`hb-serve`, unchanged name)
  - detector: D-arch-resources
    severity: warning
    section: §Cross-cutting Patterns → Logging and timing
    change: >-
      The sink's description gains the drop — "bridges `log` records, applies the allowlist scrub and chains the panic hook" becomes "bridges `log` records, drops every record whose target is outside the engine and escher prefix sets (`ENGINE_TARGET_PREFIXES`, `ESCHER_TARGET_PREFIXES`) at every level and whatever `RUST_LOG` names, scrubs the records that print, and chains the panic hook" — still one `EnvFilter` and one fmt layer on a `Registry`, no filter layer added; citations `lib.rs:90-134` → `lib.rs:96-140` and `format.rs:96-126` → `format.rs:123-155`. The sentence on the upstream `tracing_subscriber::fmt::init()` installs writing to stdout unscrubbed stays as is.
    sidecar: >-
      2026-10-07-sink-target-allowlist — Logging and timing: sink described as drop-then-scrub (outside-target records dropped whole); two citations re-pointed.
    rationale: >-
      Same claim as the primary, restated as a pattern: the sink is described as a scrub only. Report Schema / config: "every other target's record dropped whole"; Symbols / APIs: "the subscriber is still one `EnvFilter` and one fmt layer on a `Registry` ... No filter layer was added". Expected amendments names this section. Citation moves per the report's `lib.rs` (+6) and `format.rs` table (`format_event` `102-126` → `128-155`).
    basis: >-
      .andromeda/architecture.md:193 · packages/escher-telemetry/src/lib.rs:96-140 · packages/escher-telemetry/src/format.rs:123-155
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → escher-telemetry
    change: >-
      The row reads "modules format (the event formatter, the target allowlist — its drop of every outside-target record — and the field scrub) and panic (the chaining panic hook) under the `init` entry point", with citations `lib.rs:18-19` → `lib.rs:22-23` and `lib.rs:90-134` → `lib.rs:96-140`.
    sidecar: >-
      2026-10-07-sink-target-allowlist — Existing Scopes, escher-telemetry row: format module described with the target drop; two citations re-pointed.
    rationale: >-
      Same claim as the primary, restated in the scope table: the format module is "the event formatter and allowlist scrub". Report Crates / modules: "changed: `escher-telemetry` (its formatter and crate doc)"; `lib.rs` module doc `1-14` → `1-18`, `#![deny(missing_docs)]` `16` → `20`, every later line +6. Expected amendments names this section.
    basis: >-
      .andromeda/architecture.md:256 · packages/escher-telemetry/src/lib.rs:22-23, :96-140
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → seven_guis
    change: >-
      "the crate's one integration-test target, `tests/host_binary.rs`" becomes "the crate's two integration-test targets, `tests/host_binary.rs` and `tests/host_log.rs` — both spawning the built `escher-session` binary — and their shared module `tests/common/mod.rs`, no test target and no test in it, read through `mod common;` by both (examples/seven_guis/tests/host_binary.rs:1-3; examples/seven_guis/tests/host_log.rs:1-4; examples/seven_guis/tests/common/mod.rs:1-4)".
    sidecar: >-
      2026-10-07-sink-target-allowlist — Existing Scopes, seven_guis row: one integration-test target → two (`host_binary`, `host_log`) plus the shared `tests/common/mod.rs`.
    rationale: >-
      The "one host-spawning test target" claim the Process-wide-state proposal retires is restated here as a count. Report Changes → Files: new `examples/seven_guis/tests/host_log.rs` and `examples/seven_guis/tests/common/mod.rs`; Crates / modules: "Two new integration-test targets — `host_log` in the seven_guis package ... and one new shared test module that is no test target, `examples/seven_guis/tests/common/mod.rs` ..., read by `host_binary.rs` and `host_log.rs` through `mod common;`".
    basis: >-
      .andromeda/architecture.md:264 · examples/seven_guis/tests/host_log.rs:1-4 · examples/seven_guis/tests/common/mod.rs:1-4 · examples/seven_guis/tests/host_binary.rs:1-3 (unmoved), :7 (`mod common;`)
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Environment variables
    change: >-
      Citation only — the `RUST_LOG` read `EnvFilter::try_from_default_env` moves from `packages/escher-telemetry/src/lib.rs:113` to `packages/escher-telemetry/src/lib.rs:119`; the entry's wording stands (`RUST_LOG` is still the sink's one env read, default `warn`).
    sidecar: >-
      2026-10-07-sink-target-allowlist — Environment variables: `RUST_LOG` citation re-pointed `lib.rs:113` → `lib.rs:119`.
    rationale: >-
      Report Counts / qualifiers moved: "the `EnvFilter` `113` → `119`"; Symbols / APIs: "No new ... env var ... `RUST_LOG` stays the sink's one env read (`lib.rs:119`)". The registry entry's coordinate now points at `if tracing::dispatcher::has_been_set()` territory, not the env read.
    basis: >-
      .andromeda/architecture.md:153 · packages/escher-telemetry/src/lib.rs:119
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → Feature gating
    change: >-
      Citation only — the ungated startup event of escher-telemetry moves from `packages/escher-telemetry/src/lib.rs:132` to `packages/escher-telemetry/src/lib.rs:138` (the install line `INFO escher_telemetry`); `packages/escher-telemetry/Cargo.toml:1-16` and `packages/escher-telemetry/src/panic.rs:11-21` stand (manifest and `panic.rs` untouched per the preservation gate).
    sidecar: >-
      2026-10-07-sink-target-allowlist — Feature gating: escher-telemetry install-line citation re-pointed `lib.rs:132` → `lib.rs:138`.
    rationale: >-
      Report Counts / qualifiers moved: "the install line `132` → `138`"; Symbols / APIs: "the install line still `INFO escher_telemetry` (`lib.rs:138`)". This is the eighth `lib.rs:` occurrence the report counts for architecture and the only one outside the sections above.
    basis: >-
      .andromeda/architecture.md:111 · packages/escher-telemetry/src/lib.rs:138
    dependent-of: D-arch-resources
```

### security-plan

```yaml
# security-plan drift — chunk 2026-10-07-sink-target-allowlist
# Detector verdicts, strict reading of each invariant against the report's Changes:
#   D-security-input: HOLDS — no new external-input surface (report.md:19, :74; RUST_LOG stays the one env read).
#   D-security-auth:  HOLDS — no identity / session / token / key / secret source touched (report.md:18-19).
#   D-security-deps:  HOLDS — Dependencies "none added, none bumped" (report.md:21). No proposal.
# No escalate condition fired. The proposals below are baseline restatements, not invariant violations:
# the report's "Spec claims disproved by measurement" (report.md:40-44) and "Expected amendments"
# (report.md:65-66, :71) name security-plan sites whose claims this chunk superseded, and those sites sit
# in the sections D-security-input and D-security-auth read against. Each is filed under the nearest
# detector at severity warning; the orchestrator may re-file or drop them if another channel carries
# the expected amendments.
# Sweep of security-plan.md (occurrence-level, matches the report's counts): `as written` 1 (L376) ·
# `not measured by level` 3 (L115, L116, L376) · `Sink target allowlist` 2 (L257, L376) ·
# `outside the sink` 3 (L115, L116, L257) · "at `trace`" 4 (L115, L116, L257, L376) · `crud-surname` 1 (L115) ·
# format.rs: 1 (L376) · escher-telemetry/src/lib.rs: 1 (L292) · tests/host_binary.rs: 1 (L293).
# Swept and left standing (still true or another sense): L176 §Data Protection "escher's own log sink now
# redacts"; L257/L376 "upstream apps' fmt::init() ... and the WPT runner's env_logger stay unscrubbed";
# L379/L380 `unscrubbed` (run.log, transcript.jsonl); L376 "the chained std panic hook still prints the raw
# panic message" (panic.rs is under the preservation gate, report.md:12); L377 escher-session stdout sentence.
# No registry key file holds `logging-redaction-wire` (grep over .andromeda/registries: 0 hits).
proposals:
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → row `Markup attributes | id (stable element id)`"
    change: >-
      Replace the clause "that value prints on the stderr of a host that installs escher's sink at
      `RUST_LOG=debug` and `trace`, through Stylo's own records, which are outside the sink's engine
      allowlist (`crud-surname` ×12 at `debug` and ×20 at `trace` on the `escher-session` binary, 0 at the
      default level and at `info`; the windowed stand not measured by level; as measured at
      …/2026-10-07-driver-session/evidence/host-stderr-by-level.md)" with: since
      2026-10-07-sink-target-allowlist a host that installs escher's sink prints that value at no
      `RUST_LOG` level — the Stylo records that carried it (`style::*`, `selectors::matching`) are from
      targets outside both of the sink's prefix sets (the engine set and `escher_`) and the sink drops such
      a record whole, at every level; ids found on stderr, before → after: `escher-session crud` 15 of 15 →
      0 at `debug` and at `trace`, windowed `seven_guis_native` on Home 11 of 11 → 0 at `debug` and at
      `trace`, 0 at the default level and at `info` on both before and after (as measured at
      escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.md).
    sidecar: >-
      2026-10-07-sink-target-allowlist — §Input Validation `id` row: the author-key id no longer prints on a
      sink-installing host's stderr at any level (outside-target records dropped); windowed stand now
      measured by level; the `crud-surname` ×12/×20 figures retired.
    rationale: >-
      Invariant holds (no new input surface, report.md:19, :74); the row's leak clause is superseded.
      report.md:15 — `format_event` returns before writing anything for a target in neither prefix set, at
      every level and whatever `RUST_LOG` names; report.md:51-56 — ids found 15 of 15 → 0 (host) and 11 of
      11 → 0 (windowed) at `debug`/`trace`; report.md:42 — "not measured by level" is superseded, the
      windowed stand is measured before and after; report.md:44 — the ×12/×20 figures are substring
      occurrences (×8/×13 as lines), so if any before-figure is kept it needs its unit beside it;
      report.md:66 lists this row as an expected amendment.
    basis: .andromeda/security-plan.md:115
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → row `Markup attributes | aria-label · <label for> (accessible names)`"
    change: >-
      Replace "a host that installs escher's sink writes none to stderr at the default level or at
      `RUST_LOG=info` — at `trace` a name prints there through the third-party target
      `dioxus_core::diff::node`, which is outside the sink's engine allowlist (a fixture row's name ×3 on the
      `escher-session` binary; typed text not measured, since no command can type yet; the windowed stand
      not measured by level; as measured at …/host-stderr-by-level.md)" with: a host that installs escher's
      sink writes no name to stderr at any `RUST_LOG` level since 2026-10-07-sink-target-allowlist —
      `dioxus_core::diff::node` is outside both of the sink's prefix sets and its records are dropped whole;
      names found on stderr at `trace`, before → after: `escher-session crud` 6 of 6 → 0, windowed
      `seven_guis_native` on Home 5 of 5 → 0, 0 at the default level, `info` and `debug` on both; typed text
      is still not measured, since no command can type into the host's instance yet (as measured at
      escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.md).
    sidecar: >-
      2026-10-07-sink-target-allowlist — §Input Validation accessible-names row: no name prints on a
      sink-installing host's stderr at any level (`dioxus_core::diff::node` records dropped); windowed stand
      now measured by level; typed text still unmeasured.
    rationale: >-
      Same retired claim as the `id` row, second occurrence. report.md:52, :56 — names found 6 of 6 → 0
      (host) and 5 of 5 → 0 (windowed) at `trace`; report.md:17 — `dioxus_core` is in the dropped class
      (unit-tested); report.md:58 — "Typed text is NOT measured"; report.md:66 names this row.
    basis: .andromeda/security-plan.md:116
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Logging & Monitoring → Log format and backends → bullet `escher's own sink`"
    change: >-
      Replace the tail from "and a record from a target outside the engine allowlist prints its message and
      fields as written — …" through "… owed by the route entry "Sink target allowlist" (the founder's
      ruling, 2026-10-07)" with: and a record from a target outside both prefix sets — the engine set and
      escher's own `escher_` (`ESCHER_TARGET_PREFIXES`) — is dropped whole by the formatter: zero bytes, at
      every level, WARN and ERROR included, native and bridged alike (a bridged `log` record judged by the
      target it was logged under), whatever `RUST_LOG` names (the four leans approved by the operator at the
      P5 review, 2026-10-07; first cost: the windowed stand's one default-level line,
      `WARN winit_wayland::window::state`, no longer prints); stderr lines before → after — `escher-session
      crud` (one `hello`, then `stop`): 0 → 0 at the default level, 1 → 1 at `info`, 1165 → 1 at `debug`,
      1501 → 1 at `trace`; windowed `seven_guis_native` (Home, 10 s): 1 → 0, 3 → 1, 12,413 → 1, 47,482 → 1 —
      the one line left being the install line `INFO escher_telemetry`, with no stable id and no accessible
      name at any setting and stdout empty in all 16 readings; typed text is not measured (no command can
      type into the host's instance yet) (as measured at
      escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.md).
    sidecar: >-
      2026-10-07-sink-target-allowlist — §Logging & Monitoring: outside-target records are dropped whole at
      every level (was: printed as written); by-level readings restated for both binaries; the "owed by the
      route entry Sink target allowlist" clause discharged.
    rationale: >-
      report.md:41 — "a record from a target outside the allowlist prints its message and fields as written"
      is true at ebd7411f and false after 25d9b72d (security-plan 1 site); report.md:42-43 — "not measured by
      level" and "owed by the route entry" are superseded, the entry has delivered; report.md:15, :22 — the
      third outcome (drop, zero bytes, every level); report.md:47-58 — the readings; report.md:87-88 — the
      operator-approved leans and the lost winit WARN.
    basis: .andromeda/security-plan.md:376
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Logging & Monitoring → Log format and backends → bullet `escher's own sink` (the scrub rule and its citation)"
    change: >-
      Restate the rule sentence as three outcomes and re-point its citation: "a stderr-only subscriber
      whose formatter applies an allowlist scrub with a drop — an event whose target starts with `blitz`,
      `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer` or `js_console` prints only
      `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`, every
      other field (the message included) as `[redacted]`; an event whose target starts with `escher_`
      prints every field except `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`,
      `error` and `panic.payload`, which are redacted; an event from any other target is not printed at
      all; bridged `log` records pass the same rule under the target they were logged under" — replacing
      "… are redacted at any target; bridged `log` records (`js_console`) pass the same scrub" — and change
      the citation `packages/escher-telemetry/src/format.rs:17-84` to
      `packages/escher-telemetry/src/format.rs:21-107; packages/escher-telemetry/src/format.rs:134-138`.
    sidecar: >-
      2026-10-07-sink-target-allowlist — §Logging & Monitoring: the sink's rule restated as engine-scrub /
      escher-redact / drop; citation format.rs:17-84 → format.rs:21-107 and :134-138.
    rationale: >-
      "redacted at any target" described a two-outcome rule in which every non-engine target printed; the
      report's Schema / config bullet (report.md:22) makes it three outcomes, and report.md:16-17 gives the
      escher set as the one prefix `escher_`. Citation move per report.md:30: `ENGINE_TARGET_PREFIXES` const
      17-24 → 21-28, `decide` 72-84 → 93-107; the drop is at `format_event` :134-138 (report.md:15).
      report.md:29 counts security-plan's `format.rs:` citations at 1 — this one.
    basis: packages/escher-telemetry/src/format.rs:21
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Logging & Monitoring → Log format and backends → bullet `escher's own sink` (the `log.file` residual)"
    change: >-
      Narrow "an allowlisted `log.file` carries a host path for bridged third-party records at
      `RUST_LOG=info` (as measured at the chunk's smoke, …/2026-10-06-telemetry-bootstrap/report.md)" to:
      an allowlisted `log.file` can still carry a host path for a bridged record under an engine target;
      a bridged record under any other third-party target is dropped whole since
      2026-10-07-sink-target-allowlist, its `log.file` with it — at `RUST_LOG=info` both sink-installing
      binaries now print the install line only; `log.file` occurrences were not counted in that chunk.
    sidecar: >-
      2026-10-07-sink-target-allowlist — §Logging & Monitoring: the `log.file` host-path residual narrowed
      to engine-target bridged records; third-party-target records no longer print.
    rationale: >-
      Same retired claim (third-party records print) in another wording. Derived, not directly measured:
      report.md:15 — an outside-target record writes zero bytes, bridged records judged by `log.target`;
      report.md:50, :54 — at `info` the host reads 1 → 1 line and the windowed stand 3 → 1, the one line the
      install line; report.md:215-218 — the windowed INFO row lost is `wgpu_hal::vulkan::adapter`. Caveat
      for the orchestrator: report.md:58 states "`log.file` occurrences were not counted", so the engine-
      target half rests on `log.file` staying in `SAFE_FIELDS` (report.md:16, engine rule unchanged), not on
      a reading; drop this proposal if the pass requires a measured basis.
    basis: .andromeda/security-plan.md:376
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Bootstrap phases (derive for route / setup-project) → logging-redaction-wire"
    change: >-
      Restate the entry: discharged for escher's own log sink — `escher_telemetry::init`'s formatter
      redacts engine-target messages and fields, redacts content-named fields on escher targets, and since
      2026-10-07-sink-target-allowlist drops every record from a target outside its two prefix sets, at
      every level, in the two binaries that install it, `seven_guis_native` and `escher-session` — see
      Logging & Monitoring; still open for the upstream apps' `fmt::init()` stdout subscribers and the WPT
      runner's `env_logger`, which stay unscrubbed. Remove the clause "open for records from targets outside
      the sink's engine allowlist, which print unredacted — stable ids at `RUST_LOG=debug`, accessible names
      at `trace` — owed by the route entry "Sink target allowlist" (the founder's ruling, 2026-10-07)" and
      the wording "content-named fields of any target".
    sidecar: >-
      2026-10-07-sink-target-allowlist — bootstrap phase logging-redaction-wire: third-party-target clause
      discharged (records dropped); the upstream-apps / WPT-runner clause stays open.
    rationale: >-
      report.md:65 — expected amendment "security-plan §Bootstrap phases (`logging-redaction-wire`) —
      third-party-target clause discharged — carried"; report.md:43 — the "owed by the route entry" claim is
      superseded (security-plan 2 sites: this one and the Logging & Monitoring bullet); report.md:15, :96 —
      the drop holds at every level and under a naming `RUST_LOG` directive (`telemetry_drop` 1 passed).
      The report changes nothing about the upstream apps' subscribers or the WPT runner, so that clause is
      kept as written.
    basis: .andromeda/security-plan.md:257
    dependent-of: D-security-input
  - detector: D-security-auth
    severity: warning
    section: "§Secret Management → Environment values read (none secret-bearing) → bullet `RUST_LOG`"
    change: >-
      Re-point the citation `packages/escher-telemetry/src/lib.rs:113` to
      `packages/escher-telemetry/src/lib.rs:119`; the sentence itself stands (`RUST_LOG` is still the sink's
      one env read, defaulting to `warn`).
    sidecar: >-
      2026-10-07-sink-target-allowlist — §Secret Management: `RUST_LOG` citation lib.rs:113 → lib.rs:119
      (the crate doc grew by 6 lines; no env read added).
    rationale: >-
      Invariant holds — no secret source or env read added ("`RUST_LOG` stays the sink's one env read
      (`lib.rs:119`)", report.md:19). Citation-only move: report.md:31 — "the `EnvFilter` `113` → `119`";
      report.md:29 counts security-plan's `escher-telemetry/src/lib.rs:` citations at 1 — this one.
      Confirmed by one read: `EnvFilter::try_from_default_env()` is at lib.rs:119.
    basis: packages/escher-telemetry/src/lib.rs:119
  - detector: D-security-auth
    severity: warning
    section: "§Secret Management → Environment values read (none secret-bearing) → bullet `env!(\"CARGO_BIN_EXE_escher-session\")` and `env!(\"CARGO_TARGET_TMPDIR\")`"
    change: >-
      Re-point the citation `examples/seven_guis/tests/host_binary.rs:11-15` to
      `examples/seven_guis/tests/host_binary.rs:15; examples/seven_guis/tests/host_log.rs:11;
      examples/seven_guis/tests/common/mod.rs:12-14` (the `tests/blitz-tests/tests/session_common/mod.rs:71`
      citation is unmoved); the sentence stands — the two build-time values are still the only ones the
      session checks read, now from two host-spawning files and their shared module.
    sidecar: >-
      2026-10-07-sink-target-allowlist — §Secret Management: session-check env citation
      host_binary.rs:11-15 → host_binary.rs:15, host_log.rs:11, common/mod.rs:12-14 (`state_dir` moved to the
      shared test module; `host_log` is a second reader).
    rationale: >-
      Invariant holds — no new env var (report.md:19). Citation-only move: report.md:32 — `BINARY` `11` →
      `15`, and `state_dir` `14-16` left the file → `examples/seven_guis/tests/common/mod.rs:12-14`;
      report.md:20, :59 — `host_log.rs` is a new check spawning `CARGO_BIN_EXE_escher-session` with a state
      directory under `CARGO_TARGET_TMPDIR`. report.md:29 counts security-plan's `host_binary.rs:` citations
      at 1 — this one. Confirmed by one read: `CARGO_BIN_EXE_escher-session` at host_binary.rs:15 and
      host_log.rs:11, `CARGO_TARGET_TMPDIR` at common/mod.rs:13.
    basis: examples/seven_guis/tests/host_binary.rs:15
```

### test-plan

```yaml
# drift-detector: test-plan · chunk 2026-10-07-sink-target-allowlist
# D-tests-framework: no drift — the report's runner is cargo's built-in harness through `cargo test … --locked`, `ci-leg.sh` and `agent-run.sh`; the two new checks use the forms §2 already states (`CARGO_BIN_EXE_*`, the re-exec on one `#[ignore]` child, `cfg(unix)`); Dependencies: none added, none bumped (report.md:21).
# D-tests-coverage: the invariant holds for the CODE (5 unit tests + `telemetry_drop` + `host_log`, report.md:73); what drifted is test-plan's own inventory, counts and citations — proposals 4-10.
# D-tests-obs-harness: the 5-command harness and status shape are unchanged (report.md:34); the sink's scrub shape gained a third outcome (report.md:22), which test-plan §3 "Stand log format" restates in prose, so it must move with obs-plan §3/§8 (obs-plan.md:290, :295 still carry the old claim; obs-plan's own detector owns that side) — proposals 1-3.
# Not proposed, read and left: §3 Agent-run Proof (`run stand` 72 · 0 · 3 unmoved, report.md:27; the chain has no precedent of a link for an unmoved count — the plan's "one dated link" on §3 Proof is not drift); test-plan.md:185 and :208 (true as written, name `host_binary` without claiming it is the only one); test-plan.md:106, :114 (`run stand` selection unchanged); key `Session lifecycle` → session-checks and session-binary (still true); key `Bootstrap phases` (untouched).
# Coordinates below were measured on the work tree by grep this pass (format.rs test module 200-376, lib.rs 142-150, host_binary.rs tests 18-81 and 83-102, host_log.rs test 25-159, seven_guis tests/common/mod.rs guard 23-32, telemetry_drop.rs 1-78) and agree with report.md:30-32.
proposals:
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Stand log format"
    change: >-
      Replace the scrub clause ("with escher-telemetry's allowlist scrub applied (obs-plan §3) — a scrub of engine targets and content-named fields: on `escher-session` stderr holds no stable id and no accessible name at the default level and at `info`, while records from third-party targets outside the allowlist print as written, carrying ids at `debug` and ids and names at `trace`; typed text and the windowed binary by level are not measured (obs-plan §8; as measured at …/2026-10-07-driver-session/evidence/host-stderr-by-level.md)") with: "with escher-telemetry's target allowlist applied (obs-plan §3) — a record from an engine target prints only the safe fields, a record from an escher target (prefix `escher_`) prints with its content-named fields redacted, and a record from any other target is dropped whole, at every level (WARN and ERROR included) and whatever `RUST_LOG` names: on both binaries stderr holds no stable id and no accessible name at the default level, `info`, `debug` and `trace` — 0 lines at the default and, at the other three, the one install line `INFO escher_telemetry` (124 bytes); typed text is not measured (no command can type into the host's instance yet) (obs-plan §8; as measured at escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.md)"; and extend the exercised-by clause: `escher-session`'s format and empty stdout by seven_guis' `host_binary` and, at `RUST_LOG=trace` with no id and no name on stderr, `host_log` (examples/seven_guis/tests/host_log.rs:25-159); the format and the drop by `telemetry_stdout_silent`, `telemetry_scrub`, `telemetry_panic_hook`, `telemetry_init_idempotent` and `telemetry_drop` (add tests/blitz-tests/tests/telemetry_drop.rs:1-78). The line shape and the boot-smoke clause stay as written.
    sidecar: "2026-10-07-sink-target-allowlist — §3 Stand log format: outside-allowlist records are dropped whole (were printed as written); both binaries read 0 ids / 0 names at four settings; `host_log` and `telemetry_drop` join the exercised-by list."
    rationale: >-
      Report Schema / config (report.md:22): every other target's record is now dropped whole; line format of a printed record unchanged. The by-level readings (report.md:47-58): both binaries 0 ids and 0 names at unset/info/debug/trace, one 124-byte install line at info and below, the windowed stand now measured by level. Spec claims superseded (report.md:41-42) names test-plan `as written` 1 site and `by level`. obs-plan §3/§8 is restated by its own expected amendment (report.md:62-63); leaving test-plan §3 on the old claim would be the one-sided change the detector names.
    basis: ".andromeda/test-plan.md:102 · report.md:22, :47-58"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§5 Integration Test Strategy → Session host ↔ what it writes"
    change: >-
      Keep the `stand_session_quiet` sentence (a host that installs no log sink; `RUST_LOG=trace` inert) and replace everything from "It does NOT cover a host that installs escher's sink — …" to "…owed by the route entry "Sink target allowlist"" with: "It does not cover a host that installs escher's sink; that host is covered by seven_guis' `host_log` — the real `escher-session` binary on `crud` at `RUST_LOG=trace`, its needles read from the stand booted in the test's own process (at least 15 stable ids, 16 read; the CRUD row and label texts), both streams drained by a thread each, reached with `attach` under a 60 s bound and stopped with `stop`: exit 0, empty stdout, no state directory, at least one stderr line, every stderr line carrying `service.name=seven_guis`, and no id and no name anywhere on stderr — seen red on the unfixed sink (16 of 16 ids and 6 of 6 names in 1501 stderr lines) — and by `telemetry_drop`, which proves the sink writes nothing for a third-party target at any level, native and bridged, under a `RUST_LOG` directive naming it. By level, both binaries read 0 ids and 0 names at the default, `info`, `debug` and `trace` (the windowed stand by the instrument, not by a standing check); typed text is not measured (no command can type into the binary's instance yet)". Citations: keep tests/blitz-tests/tests/stand_session_quiet.rs:29-102; add examples/seven_guis/tests/host_log.rs:25-159 and tests/blitz-tests/tests/telemetry_drop.rs:16-67; replace the as-measured pointer with escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.md.
    sidecar: "2026-10-07-sink-target-allowlist — §5 Session host ↔ what it writes: the sink-installing host is now covered (`host_log`, `telemetry_drop`); ids at `debug` / names at `trace`, 'windowed stand not measured by level' and 'owed by the route entry' retired."
    rationale: >-
      Same retired claim as the primary, restated with its tokens: report.md:41 (`outside the sink` 1 site in test-plan), :42 (`not measured by level` 1 site), :43 (`Sink target allowlist` 1 site — "the entry has delivered"). The replacement's content is report.md:59 (what `host_log` reads), :60 (`telemetry_drop`), :99 (the red reading), :47-58 (by-level). Typed text stays not measured (report.md:58).
    basis: ".andromeda/test-plan.md:180 · report.md:41-43, :59-60"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§2 Test Strategy → Process-lifecycle checks"
    change: >-
      Restate the closing clause "a check that captures a host which installs escher's sink at `debug` or below drains its pipe while the host runs, or captures to a file — about 1.1 MB of stderr at `trace` against a 64 KiB pipe" as: "a check that captures a host which installs escher's sink still drains its pipes while the host runs (seven_guis' `host_log` reads each stream on a thread from the spawn on): since 2026-10-07-sink-target-allowlist such a host writes one 124-byte install line at `info`, `debug` and `trace`, where before the drop it wrote about 1.1 MB at `trace` against a 64 KiB pipe"; and re-point the dead citation examples/seven_guis/tests/host_binary.rs:24-35 (the `Host` guard left that file) to examples/seven_guis/tests/common/mod.rs:23-32, adding "as measured at escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/report.md".
    sidecar: "2026-10-07-sink-target-allowlist — §2 Process-lifecycle checks: the 1.1 MB-at-`trace` figure dated to before the drop (now 124 bytes); the seven_guis guard citation re-pointed to tests/common/mod.rs:23-32."
    rationale: >-
      The same retired mechanism (a sink-installing host prints megabytes of third-party records at `trace`) restated as a figure with none of the swept tokens. Report by-level table: `escher-session crud` at `trace` 1,104,899 → 124 bytes, `debug` 892,447 → 124 (report.md:51-52). Citation move: "`state_dir` 14-16, `clear` 19-22 and the `Host` guard 25-34 left the file → examples/seven_guis/tests/common/mod.rs:12-14, :17-20, :23-32" (report.md:32). `host_log` drains both streams by a thread each (report.md:59).
    basis: ".andromeda/test-plan.md:49 · report.md:32, :51-52"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → escher-telemetry"
    change: >-
      Row should read: "10 inline unit tests — 9 in format.rs: three scrub decision branches (an engine target prints only safe fields, content fields redact for any target, the other fields of an escher target print), one bridged-`log` record judged by its `log.target`, and five on the outside-target drop (dropped whatever the field; not admitted by a shared prefix — `escher` alone, `style`, `dioxus_core`, `log` and the empty target; a native and a bridged record each writing no byte; the drop leaving engine and escher records printing) — and 1 in lib.rs, the `service_identity!()` expansion (packages/escher-telemetry/src/format.rs:200-376; packages/escher-telemetry/src/lib.rs:142-150); its process-global behaviours are the five `telemetry_*` integration files below — as measured at escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/report.md".
    sidecar: "2026-10-07-sink-target-allowlist — §1 escher-telemetry: unit tests 5 → 10 (format.rs 4 → 9), `telemetry_*` files four → five, citations re-pointed (format.rs:200-376, lib.rs:142-150)."
    rationale: >-
      Counts / qualifiers moved: "escher-telemetry unit tests: 5 → 10 (4 → 9 in format.rs, 1 in lib.rs) … Stated in: test-plan (`5 unit` 1 site)" (report.md:26); telemetry family "now holds five files" (report.md:28); what the five new tests pin (report.md:17, :95); citation moves — unit tests 167-246 → 200-376, lib.rs later lines +6 (report.md:30-31). The code carries its tests; the inventory row is what is stale.
    basis: ".andromeda/test-plan.md:20 · report.md:26, :28, :30-31"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests (the process-telemetry clause)"
    change: >-
      The clause "and process telemetry — stdout stays silent, the scrub redacts, the panic hook logs one event and chains, init is idempotent" should add a fifth behaviour: "…init is idempotent, and a record from a target outside the allowlist writes nothing at any level under a `RUST_LOG` directive naming it, native and bridged, while an escher record prints and an engine record prints redacted (`telemetry_drop`, the test binary re-run on its one `#[ignore]` child)", and its citation list should add tests/blitz-tests/tests/telemetry_drop.rs:1.
    sidecar: "2026-10-07-sink-target-allowlist — §1 tests/blitz-tests: `telemetry_drop` joins the process-telemetry inventory (five files; second re-exec file with one `#[ignore]` child)."
    rationale: >-
      Duplicate of the "four telemetry files" inventory: new integration-test target `telemetry_drop` in blitz-tests (report.md:20), what it reads (report.md:60), five files with two in the re-exec form (report.md:28).
    basis: ".andromeda/test-plan.md:26 · report.md:20, :28, :60"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → apps (rdme, bump, examples, accesskit_xplat)"
    change: >-
      Replace "`examples/seven_guis` holds one integration-test target, `tests/host_binary.rs` (2 tests) — …" with "`examples/seven_guis` holds two integration-test targets and one shared module that is no target, `tests/common/mod.rs` (`state_dir`, `clear`, the kill-and-reap `Host` guard, read through `mod common;` by both): `tests/host_binary.rs` (2 tests) — {its existing description unchanged} — run as `cargo test -p seven_guis --locked --test host_binary`; and, since 2026-10-07-sink-target-allowlist, `tests/host_log.rs` (1 test, unix only) — the `escher-session` binary on `crud` at `RUST_LOG=trace` exits 0 with nothing on stdout and no state directory, every stderr line stamped `service.name=seven_guis`, and no stable id and no accessible name on stderr, its needles read from the stand itself — run as `cargo test -p seven_guis --locked --test host_log`"; re-point the citation examples/seven_guis/tests/host_binary.rs:37-121 → :18-102 and add examples/seven_guis/tests/host_log.rs:25-159; examples/seven_guis/tests/common/mod.rs:1-32.
    sidecar: "2026-10-07-sink-target-allowlist — §1 apps row: seven_guis integration-test targets one → two (`host_log`), shared `tests/common/mod.rs` named, host_binary.rs citation 37-121 → 18-102."
    rationale: >-
      Crates / modules: "Two new integration-test targets — `host_log` in the seven_guis package … and one new shared test module that is no test target, examples/seven_guis/tests/common/mod.rs" (report.md:20); host_binary.rs 121 → 102 lines, every later line −19, serving test 38 → 19, refusal test 102-121 → 83-102 (report.md:9, :32); what `host_log` reads (report.md:59).
    basis: ".andromeda/test-plan.md:10 · report.md:20, :32, :59"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: >-
      Replace the closing clause "`examples/seven_guis` holds a `tests/` directory with one integration-test target, `host_binary.rs`" with "`examples/seven_guis` holds a `tests/` directory with two integration-test targets, `host_binary.rs` and `host_log.rs`, and one shared module that is no target, `tests/common/mod.rs`, read through `mod common;` by both (examples/seven_guis/tests/common/mod.rs:1-4)".
    sidecar: "2026-10-07-sink-target-allowlist — §2 Directory pattern: seven_guis `tests/` holds two targets and a shared `common/mod.rs`."
    rationale: >-
      Second statement of the "one integration-test target" claim the §1 apps-row change retires; same report facts (report.md:10, :20).
    basis: ".andromeda/test-plan.md:65 · report.md:20"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline (the `cargo test --workspace` count chain)"
    change: >-
      Append one dated link after the 2026-10-07-driver-session link: "; re-counted at 2026-10-07-sink-target-allowlist: 142 result lines, 579 passed · 0 failed · 8 ignored (+5 `escher-telemetry` unit tests in the crate's existing lib result line, +1 `host_log` and +1 `telemetry_drop` in two new result lines; +1 ignored, `telemetry_drop`'s child), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` runs at /implement and on its pre-CI commit 25d9b72d (escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/report.md)".
    sidecar: "2026-10-07-sink-target-allowlist — §9 Local baseline: workspace count 140 · 572 · 0 · 7 → 142 · 579 · 0 · 8."
    rationale: >-
      Counts / qualifiers moved: "140 result lines · 572 passed · 0 failed · 7 ignored → 142 · 579 · 0 · 8 … Stated in: test-plan (`572 passed` 1 site, `140 result` 1 site, `7 ignored` 1 site)" with the breakdown of the 7 passes and the 1 ignored (report.md:25); the chain's last link is no longer the current count.
    basis: ".andromeda/test-plan.md:322 · report.md:25"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Session host ↔ lifecycle socket (unix only)"
    change: >-
      Re-point the citation examples/seven_guis/tests/host_binary.rs:37-100 → examples/seven_guis/tests/host_binary.rs:18-81 (the serving test; its prose is unchanged).
    sidecar: "2026-10-07-sink-target-allowlist — §5 lifecycle socket: host_binary.rs citation 37-100 → 18-81 (shared helpers moved out, −19)."
    rationale: >-
      Third of test-plan's three `examples/seven_guis/tests/host_binary.rs:` citations (report.md:29); "every later line −19: the serving test 38 → 19 … the identity assertion 92-95 → 73-76" (report.md:32). The old range now runs past the serving test into the refusal test.
    basis: ".andromeda/test-plan.md:179 · report.md:29, :32"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: >-
      In the `session-proof` row, add the new host-spawning check and its witness: after "seven_guis `host_binary` 2" insert "· seven_guis `host_log` 1 (the `escher-session` binary at `RUST_LOG=trace`: no id and no name on stderr)", and append "; `host_log` as measured at escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/report.md, the fork's CI#37592418443 on `25d9b72d` (16/16 green, jobs read by conclusion only) its witness on the macOS leg". The other six rows stay as written.
    sidecar: "2026-10-07-sink-target-allowlist — key Session lifecycle / session-proof: `host_log` 1 added beside `host_binary` 2, with CI#37592418443 on 25d9b72d."
    rationale: >-
      The seven_guis package "now holds two files that spawn the `escher-session` binary (`host_binary`, `host_log`)" (report.md:28); `host_log` reaches the session with `attach` and stops it with `stop` (report.md:59), so it is a session check the proof row does not list; CI verdict and its limit — "no job log was opened, so the macOS leg's own line for `host_log` was not read" (report.md:37). The row's existing counts are unmoved.
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:11 · report.md:28, :37, :59"
```

### obs-plan

```yaml
# obs-plan drift pass — chunk 2026-10-07-sink-target-allowlist
# D-obs-instrumentation: no drift — the report adds no hot-path operation, span, metric, log field or event (report.md:73-74).
# D-obs-stack: the stack itself is on-spec (Dependencies: none; subscriber still one EnvFilter + one fmt layer on a Registry, no filter layer — report.md:18,21). The proposals under it are baseline re-syncs of §3 statements: one stale inventory, moved citation coordinates.
# D-obs-pii: the invariant HOLDS — the chunk adds no logging of user data and narrows what prints. The proposals under it restate §8's scrub baseline, which the chunk superseded in the tightening direction; `escalate` is the detector's own grade, not a leak finding.
# New coordinates were read once against the tree (format.rs, lib.rs, tests/common/mod.rs) and agree with the report's old → new table.
proposals:
  - detector: D-obs-pii
    severity: escalate
    section: "§8 PII Scrubbing & Compliance → Values logged as-is (the last bullet, 'Past escher's scrub: …')"
    change: >-
      Restate the bullet to the after-reading: keep "the chained std panic hook prints the raw panic message to stderr"; replace "a record from a target outside the engine allowlist prints its message and fields as written — measured on the `escher-session` binary …" with: a record from a target outside the sink's allowlist (the engine prefixes and `escher_`) is no longer printed — it is dropped whole, zero bytes, at every level; measured before → after on both sink-installing binaries at four `RUST_LOG` settings — `escher-session crud` (one `hello`, then `stop`): unset 0 → 0 stderr lines; `info` 1 → 1 (124 bytes, the install line); `debug` 1165 → 1, ids found 15 of 15 → 0; `trace` 1501 → 1, ids 15 of 15 → 0, names 6 of 6 → 0 — and the windowed `seven_guis_native` (Home, 10 s, nobody clicks), its FIRST by-level reading: unset 1 → 0; `info` 3 → 1; `debug` 12,413 → 1, ids 11 of 11 → 0; `trace` 47,482 → 1, ids 11 of 11 → 0, names 5 of 5 → 0; the one line left at `info`/`debug`/`trace` is `INFO escher_telemetry`; stdout 0 bytes in all 16 readings. Drop "the windowed `seven_guis_native` … is NOT measured by level". Keep "Typed text is NOT measured — no command can type into the held instance yet". Qualify the `log.file` clause: a bridged `log` record from an outside target is now dropped whole, so `log.file` can print only for a bridged record under an engine-prefixed target; `log.file` occurrences were not counted this chunk. If the earlier `crud-surname` ×12 / ×20 figures are kept as the before-state, give their unit (substring occurrences; ×8 / ×13 as lines). Cite escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.md.
    sidecar: "§8 Values logged as-is: outside-target records no longer print as written — dropped whole; by-level before → after readings for `escher-session` and the windowed stand (first by-level reading of the latter) replace the 2026-10-07-driver-session figures."
    rationale: >-
      Report "Spec claims disproved by measurement" names this exact statement SUPERSEDED (true at ebd7411f, false after 25d9b72d; obs-plan `as written` 1 site) and "the windowed stand not measured by level" superseded (obs-plan `by level` 1 site); the figures are the report's by-level table (report.md:45-58); the unit note is report.md:44; "`log.file` occurrences were not counted" is report.md:58. The detector's invariant holds (Coverage: "PII redacted✓ (narrowed)", report.md:73) — this restates the §8 baseline the chunk moved.
    basis: ".andromeda/obs-plan.md:290 · report.md:41-58"
  - detector: D-obs-pii
    severity: escalate
    section: "§8 PII Scrubbing & Compliance → Scrubbing (first bullet, 'escher's own sink scrubs by allowlist in its formatter')"
    change: >-
      State the three outcomes and the second prefix set: the sink's allowlist of targets is two public sets, `ENGINE_TARGET_PREFIXES` (`blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer`, `js_console` — unchanged) and `ESCHER_TARGET_PREFIXES` = `["escher_"]` (underscore included: `escher_telemetry`, `escher_telemetry::panic`, `escher_driver`, `escher_stand_probe` admitted; `escher` alone is not); an engine-target event prints only the safe fields, every other field (the message included) as `{name}=[redacted]` (unchanged); an escher-target event prints every field not in the content-named set (unchanged); an event whose target — or a bridged `log` record's `log.target` — starts with a prefix of neither set is dropped whole before anything is written: no time, no target, no newline, at every level, WARN and ERROR included, whatever `RUST_LOG` names. Replace "redacted at any target" with "redacted at any target that prints". Re-point the citation `packages/escher-telemetry/src/format.rs:17-84` → `:21-107` (the drop itself at `:134-138`).
    sidecar: "§8 Scrubbing: the scrub shape gains a third outcome — outside-target records dropped whole; `ESCHER_TARGET_PREFIXES` (`escher_`) named; format.rs citation 17-84 → 21-107."
    rationale: >-
      Report Schema / config: "the sink's scrub shape gains a third outcome … every other target's record dropped whole" (report.md:22); Symbols / APIs: the new public constant, `format_event`'s early return, `Verdict::Drop`, the admitted/dropped target list (report.md:14-17); coordinates from the old → new table (report.md:30). The bullet describes a two-rule scrub and is silent on what happens to every other target, which the retired claim filled in.
    basis: ".andromeda/obs-plan.md:294 · packages/escher-telemetry/src/format.rs:21-107,134-138"
    dependent-of: D-obs-pii
  - detector: D-obs-pii
    severity: escalate
    section: "§8 PII Scrubbing & Compliance → Scrubbing (second bullet, 'Its reach is that sink …')"
    change: >-
      Retire both clauses: replace "the scrub covers engine-prefixed targets and the content-named fields only — a record from any other target (Stylo's `style::*`, `selectors::matching`, `dioxus_core::*`) passes with its message and fields unredacted (Values logged as-is, above), and the sink dropping records from targets outside its allowlist is owed by the route entry "Sink target allowlist" (the founder's ruling, 2026-10-07)" with: the sink prints a record only from a target in its allowlist — engine prefixes scrubbed to safe fields, `escher_` targets with content-named fields redacted — and drops every other target's record whole (Stylo's `style::*`, `selectors::matching`, `dioxus_core::*`, `dioxus_signals::*`; on the windowed stand also `naga::*`, `wgpu_*`, `winit_wayland::*`, `sctk`, `calloop::*`), delivered by the route entry "Sink target allowlist" (the founder's ruling, 2026-10-07; as measured at escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/report.md); the cost: a third-party WARN or ERROR no longer reaches stderr at any setting — the windowed stand's one default-level line, `WARN winit_wayland::window::state`, is gone. Keep the engine-features-off clause and the closing clause on the upstream apps' `fmt::init()` subscribers and the WPT runner's `env_logger` staying unscrubbed.
    sidecar: "§8 Scrubbing reach: 'any other target passes unredacted' and 'owed by the route entry Sink target allowlist' retired — the entry delivered; outside targets dropped at every level, third-party WARN/ERROR included."
    rationale: >-
      Two superseded statements share this sentence: "prints … as written/unredacted" (report.md:41) and "owed by the route entry "Sink target allowlist"" — "the entry has delivered" (report.md:43; obs-plan 1 site). The lost-row listing names the dropped targets and the one lost WARN row (report.md:140-290); the cost is recorded under Decisions & corrections (report.md:87-88).
    basis: ".andromeda/obs-plan.md:295 · report.md:43,87-88,202-207"
    dependent-of: D-obs-pii
  - detector: D-obs-pii
    severity: escalate
    section: "§3 → Bootstrap phases (derive for route / setup-project)"
    change: >-
      In the `pii-scrubbing-wire` row: replace "open for records from targets outside the allowlist, which print unredacted — stable ids at `RUST_LOG=debug`, accessible names at `trace`, measured on `escher-session` — owed by the route entry "Sink target allowlist"" with: discharged also for records from targets outside the allowlist — the sink drops them whole at every level, measured on `escher-session` and the windowed `seven_guis_native` (no id and no name at `debug` or `trace`), delivered by the route entry "Sink target allowlist" (§8 → Scrubbing). Keep "discharged for engine-prefixed targets and the content-named fields …" and keep "still open for the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger`, which stay unscrubbed". The `otel-sdk-install` row and the index labels are untouched.
    sidecar: "§3 Bootstrap phases → pii-scrubbing-wire: the third-party-target clause discharged by the sink's target drop; the upstream / WPT clause stays open."
    rationale: >-
      Report Expected amendments: "obs-plan §3 → Bootstrap phases (`pii-scrubbing-wire`, a keyed contract) — third-party-target clause discharged, upstream/WPT clause stays open" (report.md:63); the "owed by" site in this key file is counted at report.md:43. The key file restates the retired claim in its own words ("which print unredacted").
    basis: ".andromeda/registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md:4"
    dependent-of: D-obs-pii
  - detector: D-obs-pii
    severity: escalate
    section: "§6 Log Coverage → Log format (escher's sink), first bullet"
    change: >-
      Narrow "One line per event on stderr" to events the sink admits: one line per event from a target in the sink's allowlist (§8), and nothing at all — not an empty line — for an event from any other target, a bridged `log` record judged by its `log.target`; the line format of a record that prints is unchanged, and every field of it passes the §8 scrub. Re-point the citation `packages/escher-telemetry/src/format.rs:96-165` → `:123-198` (the drop at `:134-138`).
    sidecar: "§6 Log format (escher's sink): one line per admitted event, zero bytes for an outside-target event; format.rs citation 96-165 → 123-198."
    rationale: >-
      "One line per event … every field passes the §8 allowlist scrub" asserts every event prints; the report's `format_event` change returns before writing anything for an outside target — "no time, no target, no newline — zero bytes" (report.md:15) — while "the line format of a record that prints is unchanged" (report.md:22). Coordinates from report.md:30.
    basis: ".andromeda/obs-plan.md:145 · packages/escher-telemetry/src/format.rs:123-198"
    dependent-of: D-obs-pii
  - detector: D-obs-pii
    severity: escalate
    section: "§6 Log Coverage → Log format (escher's sink), second bullet (per-module levels)"
    change: >-
      Add the limit to the level claim: per-module levels come from `RUST_LOG` through `EnvFilter`, defaulting to `warn` when unset or unparsable — for targets in the sink's allowlist; a directive naming a target outside it (`style=trace`, `dioxus_core=trace`, `selectors=trace`) lets the record past the filter but the formatter still drops it, the formatter being the last thing a record meets. Re-point the citation `packages/escher-telemetry/src/lib.rs:113` → `:119`.
    sidecar: "§6 Log format: a `RUST_LOG` directive does not re-admit an outside target; lib.rs citation 113 → 119."
    rationale: >-
      The bullet as written implies any per-module directive prints that module; the report states the drop holds "whatever `RUST_LOG` names (the formatter is the last thing a record meets)" (report.md:15), exercised by `telemetry_drop` under `RUST_LOG=warn,style=trace,dioxus_core=trace,selectors=trace` (report.md:60). `EnvFilter` line moved 113 → 119 (report.md:31).
    basis: ".andromeda/obs-plan.md:146 · packages/escher-telemetry/src/lib.rs:119"
    dependent-of: D-obs-pii
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (subscriber installation), third bullet (the headless stand / session checks)"
    change: >-
      Extend the inventory of host-spawning checks after "seven_guis' `host_binary` spawns the real binary at `RUST_LOG=info` and reads both streams into its assertions only": seven_guis' `host_log` (one `#[cfg(unix)]` test) spawns the same real `escher-session` binary on `crud` at `RUST_LOG=trace` under the state directory `hl-trace`, both streams piped and read into its assertions only — it is the check that reads a sink-installing host at `trace` (which `stand_session_quiet`'s zero does not) and asserts exit 0, empty stdout, every stderr line stamped `service.name=seven_guis`, and no stable id and no accessible name on stderr, a failure printing a kind and a count only; `host_binary` and `host_log` share the module `examples/seven_guis/tests/common/mod.rs` (`state_dir`, `clear`, the kill-and-reap `Host` guard), which installs no subscriber. Add the citations examples/seven_guis/tests/host_log.rs:73-87 and "as measured at escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/report.md".
    sidecar: "§3 Logging stack: `host_log` joins the host-spawning checks — the real `escher-session` binary at `RUST_LOG=trace`, streams read into assertions only; shared module `examples/seven_guis/tests/common/mod.rs` named."
    rationale: >-
      Report Crates / modules: two new integration-test targets and one new shared test module (report.md:20); "the seven_guis package now holds two files that spawn the `escher-session` binary (`host_binary`, `host_log`)" (report.md:28); what the check reads (report.md:59); Expected amendments names §3 Logging stack for this inventory (report.md:64). The bullet's inventory lists one seven_guis spawner. No off-spec logger is involved — the library invariant itself holds.
    basis: ".andromeda/obs-plan.md:69 · report.md:20,28,59"
  - detector: D-obs-stack
    severity: warning
    section: "§9 CI Integration → Telemetry artifact handling, row 'Session state directory'"
    change: >-
      Add the third seven_guis state directory and re-point the moved citation: "in the checks `ss-life` and `ss-quiet` (blitz-tests) and `hb-serve`, `hb-refuse` and `hl-trace` (seven_guis) under `target/tmp/`"; source `examples/seven_guis/tests/host_binary.rs:14-22` → `examples/seven_guis/tests/common/mod.rs:12-20` (`state_dir` `:12-14`, `clear` `:17-20`), the helpers having left `host_binary.rs`. The rest of the row (0700, `session.sock` alone, nothing after `stop`, no CI upload) stands — `host_log` asserts no state directory is left.
    sidecar: "§9 Session state directory: `hl-trace` (seven_guis `host_log`) added; source citation moved host_binary.rs:14-22 → tests/common/mod.rs:12-20."
    rationale: >-
      `host_log` spawns the host "with the state directory `hl-trace` under `CARGO_TARGET_TMPDIR`" and asserts "no state directory" after `stop` (report.md:59); "`state_dir` `14-16`, `clear` `19-22` and the `Host` guard `25-34` left the file → `examples/seven_guis/tests/common/mod.rs:12-14`, `:17-20`, `:23-32`" (report.md:32); Expected amendments names §9 Session state directory (report.md:64). The cited range no longer holds the helpers.
    basis: ".andromeda/obs-plan.md:325 · examples/seven_guis/tests/common/mod.rs:12-20 · examples/seven_guis/tests/host_log.rs:73"
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (subscriber installation), first bullet (`seven_guis_native`)"
    change: "Re-point the citation `packages/escher-telemetry/src/lib.rs:99-134` → `:105-140` (`init_with_writer`); the sentence itself stands — one `EnvFilter`, one non-ANSI fmt layer with the escher formatter on a `Registry`, the `LogTracer` bridge, no filter layer added."
    sidecar: "§3 Logging stack: lib.rs citation 99-134 → 105-140 (coordinates moved, statement unchanged)."
    rationale: "Report Counts / qualifiers moved: `init_with_writer` `99-134` → `105-140` (report.md:31); Symbols / APIs confirms the subscriber shape is unchanged (report.md:18). obs-plan holds 6 `lib.rs:` citations (report.md:29); this is the first."
    basis: ".andromeda/obs-plan.md:67 · packages/escher-telemetry/src/lib.rs:105-140"
  - detector: D-obs-stack
    severity: warning
    section: "§1 Obs Scope Summary → Instrumentation scope, the **escher-telemetry** bullet"
    change: "Re-point the citation `packages/escher-telemetry/src/lib.rs:1-15` → `:1-18` (the module doc), and name the formatter by what it now does: \"the allowlist formatter (scrub for engine and escher targets, drop for every other target)\" in place of \"the allowlist scrub formatter\"."
    sidecar: "§1 escher-telemetry: lib.rs citation 1-15 → 1-18; the formatter described as scrub-and-drop."
    rationale: "Report: module doc `1-14` → `1-18` (report.md:31); the crate doc changed with the formatter (report.md:20) and now states the drop (lib.rs:12-13)."
    basis: ".andromeda/obs-plan.md:11 · packages/escher-telemetry/src/lib.rs:1-18"
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: "§2 Telemetry Strategy → Telemetry mechanism (current truth), first bullet"
    change: "Re-point the citation `packages/escher-telemetry/src/lib.rs:132` → `:138` (the install line); the sentence stands."
    sidecar: "§2 Telemetry mechanism: lib.rs citation 132 → 138."
    rationale: "Report: the install line `132` → `138`, still `INFO escher_telemetry` (report.md:18,31)."
    basis: ".andromeda/obs-plan.md:30 · packages/escher-telemetry/src/lib.rs:138"
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Service identity and line format (escher's sink)"
    change: "Re-point the citation `packages/escher-telemetry/src/lib.rs:32-54` → `:38-60` (`ServiceIdentity` and `service_identity!`); the sentence stands."
    sidecar: "§3 Service identity and line format: lib.rs citation 32-54 → 38-60."
    rationale: "Report: the re-export `30` → `34-36`, \"every later line +6\"; `ServiceIdentity` and `service_identity!` unchanged (report.md:18,31)."
    basis: ".andromeda/obs-plan.md:82 · packages/escher-telemetry/src/lib.rs:38-60"
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: "§6 Log Coverage → Logged events (current truth) → escher-telemetry, the info `telemetry installed` bullet"
    change: "Re-point the citation `packages/escher-telemetry/src/lib.rs:132` → `:138`; the sentence stands (the `panic.rs:11-21` citation beside it is untouched — `panic.rs` is under the preservation gate)."
    sidecar: "§6 Logged events (escher-telemetry): lib.rs citation 132 → 138."
    rationale: "Report: the install line `132` → `138` (report.md:31); `packages/escher-telemetry/src/panic.rs` untouched (report.md:12)."
    basis: ".andromeda/obs-plan.md:174 · packages/escher-telemetry/src/lib.rs:138"
    dependent-of: D-obs-stack
```

### a11y-plan

```yaml
proposals:
  - detector: D-a11y-obs-schema
    severity: warning
    section: "§3 A11y Assertion Harness Contract — the closing `> NOT YET MEASURED` note (the sentence describing escher's own sink)"
    change: "Restate the sink clause as a three-outcome rule, line format untouched: `…{field}={value}…`, behind a target allowlist — a record under an engine target (`blitz*`, `accesskit_xplat`, among others) prints only its `node_id` / `status` / `waiting_nodes` / `property` / `log.*` fields; a record under an escher target (prefix `escher_`) prints with `text`, `value`, `html` and `attrs` redacted; a record under any other target is dropped whole, at every level (WARN and ERROR included) and whatever `RUST_LOG` names — so an a11y violation record reaches the log only under an `escher_` or engine target; an a11y violation schema is not yet defined against it. This retires the phrase `redacts … at any target`."
    sidecar: "2026-10-07-sink-target-allowlist — a11y-plan §3: the obs sink description restated from a two-rule scrub (engine safe fields; content fields redacted at any target) to the three-outcome allowlist (engine scrubbed · `escher_` targets redacted · every other target dropped whole at every level); line format and the 'violation schema not yet defined' statement unchanged."
    rationale: "Report Changes → Schema / config: 'the sink's scrub shape gains a third outcome … After: engine targets scrubbed to safe fields (unchanged); escher targets printed with content-named fields redacted (unchanged); every other target's record dropped whole. The line format of a record that prints is unchanged.' Symbols / APIs: the drop 'holds at every level, WARN and ERROR included, and whatever RUST_LOG names'; the escher set is the one prefix `escher_`; the engine prefix set is unchanged. a11y-plan line 87 still says the scrub 'redacts `text`, `value`, `html` and `attrs` at any target' and names no drop, so its statement of the obs log schema that a future a11y violation schema is to be defined against no longer matches obs. No a11y violation schema exists yet, so there is no schema-to-schema divergence — only this stale description. The report's Expected amendments lists this site as carried ('a11y-plan §3 — the sentence describing the sink as a scrub restated to include the drop'); it cites the token `scrubs`, but the line's actual wording is `allowlist scrub` — one occurrence."
    basis: ".andromeda/a11y-plan.md:87 (the retired claim) · escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/report.md:22 (Schema / config), :15 and :17 (the drop and the two prefix sets), :69 (the expected amendment)"

# Notes for the orchestrator (not proposals):
# - D-a11y-surface: no drift. Report line 74 "no new external-input surface, hot-path operation or UI element"; line 73 "a11y n/a (no element, role, focus or painted colour changed; `run stand` 72 and the a11y leg green)".
# - Duplicate-occurrence sweep of a11y-plan.md and its one key file (registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md — labels contrast-verification-harness-setup, a11y-ci-gate-wire; neither mentions the sink): the retired claim occurs once, at a11y-plan.md:87. Zero hits for `as written`, `unscrubbed`, `by level`, `Sink target allowlist`, `third-party`, `host_binary`, `host_log`, `telemetry_*`, `escher-telemetry` paths, or the moved counts (572 / 140 result / 72 passed). No dependent-of proposals.
# - No citation re-pointing owed: a11y-plan cites none of format.rs, lib.rs (escher-telemetry) or host_binary.rs (matches the report's 0 count for a11y-plan). §9's a11y leg statement (6 + 6 + 3 tests) is unmoved by this chunk.
```
