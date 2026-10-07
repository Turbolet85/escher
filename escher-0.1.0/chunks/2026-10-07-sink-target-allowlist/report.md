# Report — 2026-10-07-sink-target-allowlist

**Chunk:** Sink target allowlist — escher's log sink drops every record from a target outside its allowlist; session host and windowed stand print no id or name at trace
**Date:** 2026-10-07
**Commits:** `25d9b72d` chore(2026-10-07-sink-target-allowlist): operator pre-CI commit, for the run this chunk's verdict reads (basis: `git log --format='%h %s' ebd7411f..HEAD`, 1 commit; `ebd7411f` is the pre-CI commit's parent and the last wrap's commit)

## Changes (structured — detectors read this)
- **Files:** (basis: `git diff --name-status ebd7411f` over the work tree, outside `.andromeda/` and the version folder — 7 paths, 6 of them the chunk's and one the handoff the last wrap left uncommitted)
  - modified `packages/escher-telemetry/src/format.rs` (246 → 376 lines) · `packages/escher-telemetry/src/lib.rs` (145 → 150) · `examples/seven_guis/tests/host_binary.rs` (121 → 102)
  - new `examples/seven_guis/tests/host_log.rs` (159 lines) · `examples/seven_guis/tests/common/mod.rs` (32) · `tests/blitz-tests/tests/telemetry_drop.rs` (78)
  - evidence, in the chunk folder: `evidence/by-level.md`, `evidence/operator-pass.md`, and the instrument phase left there, `evidence/by-level.py` (unedited)
  - untouched, asserted by the preservation gate against `ebd7411f`: every manifest and the lockfile, `deny.toml`, `packages/escher-telemetry/src/panic.rs`, `packages/escher-driver`, every engine crate, `examples/seven_guis/src` and `assets`, the four standing `telemetry_*` checks, `stand_session_quiet.rs`, both shared blitz-tests modules, `scripts`, `.github`
- **Symbols / APIs:**
  - NEW public constant `escher_telemetry::ESCHER_TARGET_PREFIXES: &[&str]` = `["escher_"]` (`packages/escher-telemetry/src/format.rs:33`, re-exported at `packages/escher-telemetry/src/lib.rs:34-36`). It is the fifth public scrub constant beside `ENGINE_TARGET_PREFIXES` (`format.rs:21-28`), `SAFE_FIELDS` (`:37-45`), `CONTENT_FIELDS` (`:48-60`) and `REDACTED` (`:63`). No caller outside the crate reads any of the five (research's graph query; the new one has none by construction).
  - CHANGED behaviour of the sink's formatter, no signature change: `EscherFormat::format_event` (`format.rs:128-155`) resolves the record's target once (`:134-135`, `normalized_metadata` with the event's own metadata as fallback — so a bridged `log` record is judged by the target it was logged under) and, when that target starts with a prefix of neither `ENGINE_TARGET_PREFIXES` nor `ESCHER_TARGET_PREFIXES`, returns before writing anything (`:136-138`): no time, no target, no newline — zero bytes. This holds at every level, WARN and ERROR included, and whatever `RUST_LOG` names (the formatter is the last thing a record meets).
  - The private rule: `Verdict` gains `Drop` (`format.rs:70-74`); `is_escher_target` (`:82-86`) and `is_outside_target` (`:89-91`) are new; `decide(target, field)` (`:93-107`) returns `Drop` for an outside target whatever the field, and keeps the two existing rules unchanged — an engine target prints only `SAFE_FIELDS`, an escher target prints every field not in `CONTENT_FIELDS`. `ScrubVisitor::push` (`:164-187`) reads `decide` at `:168`.
  - The engine prefix set is unchanged: `blitz` · `dioxus_native` · `stylo_taffy` · `accesskit_xplat` · `debug_timer` · `js_console`. The escher set is one prefix with its underscore, `escher_`: `escher_telemetry`, `escher_telemetry::panic`, `escher_driver` and `escher_stand_probe` are admitted; `escher` alone, `style`, `dioxus_core`, `log` and the empty target are dropped (unit-tested).
  - `init` · `init_with_writer` · `ServiceIdentity` · `service_identity!` · `InitOutcome` · `InitError` are unchanged; the subscriber is still one `EnvFilter` and one fmt layer on a `Registry` (`lib.rs:119-127`), the bridge and its max level as before (`lib.rs:120-122`, `:129-132`), the install line still `INFO escher_telemetry` (`lib.rs:138`). No filter layer was added.
  - No new IPC method, endpoint, event, socket, port, env var, flag, binary or cargo feature. `RUST_LOG` stays the sink's one env read (`lib.rs:119`).
- **Crates / modules:** changed: `escher-telemetry` (its formatter and crate doc). No crate added or removed. Two new integration-test targets — `host_log` in the seven_guis package, `telemetry_drop` in blitz-tests — and one new shared test module that is no test target, `examples/seven_guis/tests/common/mod.rs` (`state_dir` `:12-14`, `clear` `:17-20`, the kill-and-reap `Host` guard `:23-32`), read by `host_binary.rs` and `host_log.rs` through `mod common;`.
- **Dependencies:** none added, none bumped (no manifest and no lockfile moved; the preservation gate covers `Cargo.toml`, `Cargo.lock`, `deny.toml` and the three package manifests).
- **Schema / config:** the sink's scrub shape gains a third outcome. Before: engine targets scrubbed to safe fields; content-named fields redacted at any target; every other target's record printed as written. After: engine targets scrubbed to safe fields (unchanged); escher targets printed with content-named fields redacted (unchanged); every other target's record dropped whole. The line format of a record that prints is unchanged: `{RFC 3339 UTC} {LEVEL} {target} service.name=… service.version=… {field}={value}…`.
- **Spec-master edits:** none (implement and the operator pass touched no master, key file or leaf).
- **Counts / qualifiers moved:**
  - Workspace test run: 140 result lines · 572 passed · 0 failed · 7 ignored → **142 · 579 · 0 · 8** (basis: `target/ci-logs/test.log` of `bash .github/scripts/ci-leg.sh fast`, read twice — implement's gate run at 08:09Z and the operator pass's entry 19 at 08:14Z, same figures). The two new result lines are the two new test targets; the 7 new passes are 5 unit tests, `host_log`'s one and `telemetry_drop`'s parent; the new ignored test is `telemetry_drop`'s child. Stated in: test-plan (`572 passed` 1 site, `140 result` 1 site, `7 ignored` 1 site) and `.claude/docs/tests-summary.md` (`140 result` 1 site).
  - escher-telemetry unit tests: 5 → **10** (4 → 9 in `format.rs`, 1 in `lib.rs`; basis: `cargo test -p escher-telemetry --locked`, `10 passed`). One renamed: `other_fields_of_non_engine_targets_print` → `other_fields_of_escher_targets_print`, its `winit::platform` assertion moved to the dropped class. Stated in: test-plan (`5 unit` 1 site).
  - `run stand`: 72 passed · 0 failed · 3 ignored — unmoved (neither new file is a `stand_*` file).
  - Sink-spawning checks: the seven_guis package now holds two files that spawn the `escher-session` binary (`host_binary`, `host_log`); the blitz-tests telemetry family now holds five files (`telemetry_scrub`, `telemetry_stdout_silent`, `telemetry_panic_hook`, `telemetry_init_idempotent`, `telemetry_drop`), two of them in the re-exec form with one `#[ignore]` child (`telemetry_stdout_silent`, `telemetry_drop`).
  - Citation coordinates moved in three files the masters cite (basis: `sites.py --count` over the seven masters and `.andromeda/registries/`: `packages/escher-telemetry/src/format.rs:` 7 occurrences — architecture 3 · security-plan 1 · test-plan 1 · obs-plan 2; `packages/escher-telemetry/src/lib.rs:` 16 — architecture 8 · security-plan 1 · test-plan 1 · obs-plan 6; `examples/seven_guis/tests/host_binary.rs:` 8 — architecture 3 · security-plan 1 · test-plan 3 · obs-plan 1; 0 in any key file). The moves, old → new:
    - `format.rs`: `ENGINE_TARGET_PREFIXES` doc+const `13-24` → `13-28` (const `17-24` → `21-28`) · `SAFE_FIELDS` `28-36` → `37-45` · `CONTENT_FIELDS` `39-51` → `48-60` · `REDACTED` `54` → `63` · `LOG_TARGET_FIELD` `58` → `67` · `Verdict` `60-64` → `69-74` · `is_engine_target` `66-70` → `76-80` · `decide` `72-84` → `93-107` · `EscherFormat` `86-89` → `109-115` · `format_event` `102-126` → `128-155` · the target's resolution `108-109` → `134-135` · `ScrubVisitor::push` `135-154` → `164-187` (its `decide` call `142` → `168`) · the unit tests `167-246` → `200-376` (the bridged-record test `220-245` → `310-323`).
    - `lib.rs`: the module doc `1-14` → `1-18` · `#![deny(missing_docs)]` `16` → `20` · the re-export `30` → `34-36` · every later line +6: the two statics `84-85` → `90-91` · `init` `90` → `96` · `init_with_writer` `99-134` → `105-140` · the `EnvFilter` `113` → `119` · the bridge's max level `114-116` → `120-122` · the layer and registry `117-121` → `123-127` · `LogTracer` `123-126` → `129-132` · the install line `132` → `138`.
    - `host_binary.rs`: `BINARY` `11` → `15` · `state_dir` `14-16`, `clear` `19-22` and the `Host` guard `25-34` left the file → `examples/seven_guis/tests/common/mod.rs:12-14`, `:17-20`, `:23-32` · every later line −19: the serving test `38` → `19` · `RUST_LOG=info` `51` → `32` · `start` `56` → `37` · the bounded exit wait `68-76` → `49-57` · the identity assertion `92-95` → `73-76` · the refusal test `102-121` → `83-102`.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none — no agent-run script, CI step, leg or status shape changed (`scripts` and `.github` are under the preservation gate). The workspace test leg now runs two more test targets by construction.
- **Cross-project / external claims:**
  - `inputs: absent — no external input snapshotted` (`inputs.py verify`, this wrap) — none was read live.
  - The fork's CI run on the pushed pre-CI commit: CI#37592418443, repo Turbolet85/escher, sha `25d9b72d9bd47dae960066903244dd9b3fb1bcbf`, `verdict: green · checks 16/16 · wall 522 s` (recorded in `evidence/operator-pass.md`). All 16 jobs `success` by name, among them MSRV Build [Rust 1.91], Test (macos), Test (windows), Test (ios), Test (android). Jobs were read by conclusion only — no job log was opened, so the macOS leg's own line for `host_log` was not read.
- **Reverted / negative API facts:** none. (During implement the drop predicate was switched off once as a control and restored byte-identical; nothing of it shipped.)
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none falsified. Three standing statements are now SUPERSEDED by this chunk's fix and readings and need restating, not correcting:
  - "a record from a target outside the allowlist prints its message and fields as written", with the by-level figures for the session host — true at `ebd7411f`, false after `25d9b72d`. Sites (basis: `sites.py --count 'as written'`): architecture 2 · security-plan 1 · test-plan 1 · obs-plan 1; leaves `CLAUDE.md` 1 · `.claude/rules/observability.md` 1 · `.claude/docs/obs-summary.md` 1 · `security-summary.md` 1 · `services/escher-telemetry.md` 1 (two further leaf hits — `services/dioxus-native-dom.md`, `session-learnings.md` — were counted and not read; they may be another sense of the phrase).
  - "the windowed stand not measured by level" (`sites.py --count 'not measured by level'`: security-plan 3 · test-plan 1; `by level`: obs-plan 1, `.claude/rules/observability.md` 1, `obs-summary.md` 1) — it is measured now, before and after (below).
  - "owed by the route entry "Sink target allowlist"" (`sites.py --count 'Sink target allowlist'`: security-plan 2 · test-plan 1 · obs-plan 1 · the key file `registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md` 1; leaves obs-summary 2 · security-summary 1 · services/escher-telemetry 1 · tests-summary 1) — the entry has delivered.
  - A unit note, not a disproof: the masters' `crud-surname` ×12 at `debug` and ×20 at `trace` (security-plan 1 site, obs-plan 1 site) are substring occurrences (the earlier evidence file says so); this chunk's instrument counts LINES holding a needle and read ×8 and ×13 for the same id on the same commit (6+2 and 8+4+1 by target in the earlier file). The two agree.
- **The by-level readings** (basis: `evidence/by-level.md`, the instrument `evidence/by-level.py`, dev host, 2026-10-07; build `cargo build -p seven_guis --bins --locked`, the package alone; before = `ebd7411f`'s sources, after = the tree `25d9b72d` carries):

  | binary | setting | stderr lines before → after | stderr bytes before → after | ids found before → after | names found before → after |
  |---|---|---|---|---|---|
  | `escher-session crud` (one `hello`, then `stop`) | unset (`warn`) | 0 → 0 | 0 → 0 | 0 of 15 → 0 | 0 of 6 → 0 |
  | | `info` | 1 → 1 | 124 → 124 | 0 → 0 | 0 → 0 |
  | | `debug` | 1165 → 1 | 892,447 → 124 | 15 of 15 → 0 | 0 → 0 |
  | | `trace` | 1501 → 1 | 1,104,899 → 124 | 15 of 15 → 0 | 6 of 6 → 0 |
  | `seven_guis_native` (windowed, Home, 10 s, nobody clicks) | unset (`warn`) | 1 → 0 | 164 → 0 | 0 of 11 → 0 | 0 of 5 → 0 |
  | | `info` | 3 → 1 | 636 → 124 | 0 → 0 | 0 → 0 |
  | | `debug` | 12,413 → 1 | 6,522,462 → 124 | 11 of 11 → 0 | 0 → 0 |
  | | `trace` | 47,482 → 1 | 19,458,805 → 124 | 11 of 11 → 0 | 5 of 5 → 0 |

  The one line left at `info`, `debug` and `trace` is the install line, `INFO escher_telemetry`. stdout is 0 bytes in all 16 readings. The host exits 0 and leaves no state directory; the windowed stand exits 124 (still up when `timeout` stopped it). Every stderr line of every reading had the sink's line shape. The host's bytes written before its socket first answered fall from 856,040 (`debug`) and 1,068,492 (`trace`) to 124. This is the FIRST by-level reading of the windowed stand: before the fix it leaked every Home author key at `debug` and `trace` and five Home texts at `trace`, and printed one third-party WARN at every setting, the default included. Typed text is NOT measured (no command can type into the host's instance yet). `log.file` occurrences were not counted. No reading is a windowed witness of the accessibility-tree refresh.
- **What the session host check reads** (`examples/seven_guis/tests/host_log.rs`, one `#[cfg(unix)]` test): its needles come from the stand itself — a CRUD task booted in the test's own process (`:42-60`: ids from `element_ids()` that hold no `/` and hold a `-`, so `main` is left out; row texts from `#crud-list > div`, label texts from `label`), asserted to be the condition under test (`crud-surname` present, at least 15 ids, at least 3 rows, none empty — `:62-71`). It spawns `CARGO_BIN_EXE_escher-session` on `crud` with the state directory `hl-trace` under `CARGO_TARGET_TMPDIR`, `RUST_LOG=trace`, both streams piped and read from the spawn on by a thread each (`:73-87`), reaches the session with `escher_driver::attach` polled under a 60 s bound (never `start`), stops it with `stop`, waits for exit under a 10 s bound, and asserts exit 0, empty stdout, no state directory, at least one stderr line, every stderr line carrying `service.name=seven_guis`, and no id needle and no name needle anywhere in stderr. A failure prints a kind and a count only. The stand itself reads 16 id needles (the instrument's fixed list holds 15; the sixteenth by the shell's source is `task-header-spacer`).
- **What the sink's process check reads** (`tests/blitz-tests/tests/telemetry_drop.rs`): the parent re-runs its own binary on one `#[ignore]` child with `RUST_LOG=warn,style=trace,dioxus_core=trace,selectors=trace`; the child installs the sink with `init` and emits a native WARN under `style::traversal`, a native TRACE under `dioxus_core::diff::node`, a bridged DEBUG under `selectors::matching`, a WARN under `escher_stand_probe` and a WARN under `blitz_dom::mutator`, each with a synthetic sentinel. The parent asserts the three third-party sentinels and targets are nowhere on stderr, the escher line is there once with its sentinel, the engine line once with `message=[redacted]`, and stdout holds no sentinel.
- **Expected amendments (from plan):** (10 entries; basis for every count: `sites.py --count`, occurrence-level, over the seven masters and every file under `.andromeda/registries/`)
  - obs-plan §8 PII Scrubbing & Compliance — Values logged as-is and Scrubbing restated to the after-reading — **carried**: Schema / config, The by-level readings, Spec claims (superseded). Sites: obs-plan `as written` 1 · `Sink target allowlist` 1 · `by level` 1 · `at \`trace\`` 1 · `unscrubbed` 5.
  - obs-plan §3 → Bootstrap phases (`pii-scrubbing-wire`, a keyed contract) — third-party-target clause discharged, upstream/WPT clause stays open — **carried**: Symbols / APIs (the drop), Spec claims (the "owed by" site). Sites: `pii-scrubbing-wire` in the key file `registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md` 1 and in `registries/obs-plan-contracts.toml` 1; 0 in the obs-plan body.
  - obs-plan §3 Logging stack and §9 Session state directory — the new host-spawning check and its state directory join the inventories — **carried**: Crates / modules, What the session host check reads (state directory `hl-trace`). Sites: obs-plan `host_binary` 2 · `hb-serve` 1 · `stand_session_quiet` 1.
  - security-plan §Bootstrap phases (`logging-redaction-wire`) — third-party-target clause discharged — **carried**: Symbols / APIs, Spec claims. Sites: security-plan `logging-redaction-wire` 1 · `Sink target allowlist` 2.
  - security-plan §Logging & Monitoring and §Input Validation (the `id` and accessible-names rows) — **carried**: The by-level readings. Sites: security-plan `outside the sink` 3 · `not measured by level` 3 · `as written` 1 · `at \`trace\`` 4 · `crud-surname` 1.
  - architecture §Standard Contracts → Telemetry bootstrap, §Cross-cutting Patterns → Logging and timing, §Existing Scopes → escher-telemetry, §Occupied Resources → Process-wide state and threads — **carried**: Symbols / APIs (the new public set, the drop), Crates / modules (the two new test targets), What the session host check reads (one more spawned child of a test). Sites: architecture `ENGINE_TARGET_PREFIXES` 1 · `as written` 2 · `unscrubbed` 3 · `host_binary` 5 · `stand_session_quiet` 3 · `hb-serve` 1.
  - test-plan §3 "Stand log format", §5 "Session host ↔ what it writes", §1 "escher-telemetry", and one dated link on each count chain (§3 Proof, §9 Local baseline) — **carried**: Counts / qualifiers moved, What the session host check reads. Sites: test-plan `outside the sink` 1 · `not measured by level` 1 · `Sink target allowlist` 1 · `572 passed` 1 · `140 result` 1 · `7 ignored` 1 · `5 unit` 1 · `host_binary` 12 · `stand_session_quiet` 7.
  - a11y-plan §3 — the sentence describing the sink as a scrub restated to include the drop — **carried**: Schema / config. Site: a11y-plan `scrubs` 1 (the plan cited `a11y-plan.md:87`; the line was not re-read at authoring).
  - layout-templates §Surface: cli → Primary screens — only if the `escher-session` entry's stderr sentence is stale after the by-level reading — **carried as a question for the detector**: the entry (layout-templates `escher-session`, line 73) states usage, exit codes and "it writes nothing …"; this chunk changed none of the usage path, the exit codes or stdout (`host_binary` 2 passed). Whether its stderr sentence names ids or levels was not read at authoring.
  - Citations to re-point by measurement — **carried**: Counts / qualifiers moved, the old → new table (7 · 16 · 8 occurrences, as the plan counted).
- **Coverage of new surfaces**
  - the sink's drop (`escher_telemetry` formatter, the two sink-installing binaries' stderr) → validation n/a (no input surface) · instrumentation n/a (it is the logging sink; no log field, event or span added) · PII redacted✓ (narrowed: a record with no scrub rule is no longer printed; engine and escher records scrub as before) · tests unit + integration (5 unit tests, `telemetry_drop`, `host_log`; the four standing sink checks and `host_binary` unedited and green) · a11y n/a (no element, role, focus or painted colour changed; `run stand` 72 and the a11y leg green) · tokens n/a
  - no new external-input surface, hot-path operation or UI element.

## Deviations from intent
- **16 id needles, not the predicted 15.** `host_log` reads its needles from the booted stand; the instrument's fixed list (research's count) holds 15. The check asserts "at least 15". Justification: the plan directs the needles be read from the stand itself.
- **The unit tests' red-before-green control was taken by switching the fix off.** The plan orders the unit tests (step 6) after the fix (step 5), so they could not be seen red in sequence; with `is_outside_target` temporarily returning `false`, 5 of 5 failed; the file was restored byte-identical (`cmp`) and 5 of 5 passed.
- **Step 4's levels were chosen at implement.** The plan names the three third-party records, not their levels: WARN native, TRACE native and DEBUG bridged, so both the directive's re-admission and the WARN drop are exercised.
- **Step 5's shape.** `decide` takes a field while `format_event` needs a target-only answer; both read one private predicate, `is_outside_target`, so they cannot diverge. `ScrubVisitor::push` was restructured to give the third verdict an arm.
- **Test-side details beyond the plan's letter:** the existing bridged-record unit test now builds its subscriber through two shared test helpers (`captured`, `bridge`), its four assertions unchanged; the shared `Host` guard's field is `pub`, so `host_binary.rs` reads `host.0` as before; `host_log` also asserts the answering pid and label, so a hit is attributable to the host it spawned.
- **`evidence/by-level.md` holds more than the plan lists:** the `telemetry_drop` red and the unit control beside the host check's red; its loss listing is computed by script from the four readings.
- **The operator pass was driven by the agent** on the operator's direction given in the implement session (quoted in `evidence/operator-pass.md`): the hygiene entry, the pre-CI commit, the fast leg with the clean-tree guard and the push, and the CI read.
- scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 6 · listed 6 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 42`, base `ebd7411f`, this wrap's P1).

## Decisions & corrections
- The four leans the operator approved at the P5 review stand as built: the drop sits in the formatter; escher's targets are the prefix `escher_`; the new set is a public constant; every level is dropped, WARN and ERROR included.
- The first concrete cost of "every level is dropped": the windowed stand's one default-level line, `WARN winit_wayland::window::state`, no longer prints. Reported to the operator in the implement report; no direction was given to change it.
- The operator directed the operator pass after the implement report (the direction is quoted in `evidence/operator-pass.md`); no other correction was given this session.
- A host fact found at implement: the project's PreToolUse Bash guard refuses a `cat`/`tee` heredoc with a file target — one call was refused whole; files go through the Write tool.
- Sweep hazards found this chunk: (1) the masters' bullets are single long lines, so `grep -c` counts LINES and under-reads occurrences — the counts above are occurrence-level; (2) `as written` and `outside it` are ordinary phrases that also occur in other senses (two leaf hits were counted and not read); (3) the same id's count reads ×12/×20 as substring occurrences and ×8/×13 as lines — a figure needs its unit beside it.

## Outcome
Acceptance criteria, each re-asserted against the diff and this run's readings:
- (obs) An outside-target record writes nothing, native and bridged alike, the bridged one judged by its `log.target` — **met**: `cargo test -p escher-telemetry --locked --lib outside_target` 5 passed; `format.rs:134-138`.
- (security) The drop holds at every level and under a naming `RUST_LOG` directive — **met**: `telemetry_drop` `1 passed; 0 failed; 1 ignored`.
- (obs) The existing scrub is unweakened — **met**: the four standing telemetry files green and unedited (the preservation gate exit 0).
- (tests) The real `escher-session` binary at `RUST_LOG=trace` shows no id and no name, every line stamped, stdout empty, exit 0, no state directory — **met**: `host_log` `1 passed; 0 failed` on the package-alone build and, inside the fast leg, on the workspace build.
- (tests) That check was seen red on the unfixed sink because needles were found — **met**: `16 of 16 id needles and 6 of 6 name needles found in 1501 stderr lines` (`evidence/by-level.md`), counts only.
- (layouts) The host's usage path and exit codes are unchanged — **met**: `host_binary` `2 passed; 0 failed`.
- (obs) `evidence/by-level.md` records both binaries before and after at four settings, with the commit, the build and the form; typed text stated as not measured — **met**.
- (obs) The report lists what the drop loses, per binary, WARN and ERROR under their own headings — **met**: the listing below.
- (security) The windowed stand at `trace` stays up, prints its install line, shows no Home id and no Home text — **met**: `windowed trace: exit 124 sink-lines 1 id-lines 0 name-lines 0`.
- (arch) The change is confined to two source files and three test files; nothing added to the registries — **met**: the preservation gate exit 0, the census `last line 2`. (The diff adds one public constant and two test targets, neither of which that criterion excludes.)
- (obs) Neither the sink nor a new check writes to stdout — **met**: the print-macro census `last line 0`.
- (a11y) The stand's accessibility, snapshot and session checks pass unedited; the seven_guis manifest still names no `accessibility` feature — **met**: `run stand` `"passed": 72, "failed": 0`, the a11y leg exit 0.
- (a11y) No windowed witness of the accessibility-tree refresh is claimed — **met**: none is claimed here or in the evidence; the carry on "Stand a11y assertions" stands.
- (tests) `ci-leg.sh fast` and `ci-leg.sh doc` exit 0; the workspace counts re-read — **met**: 142 · 579 · 0 · 8 against the standing 140 · 572 · 0 · 7.
- (tests) The fork's CI run on the pushed sha reads `verdict: green` — **met**: CI#37592418443 on `25d9b72d`.

Gates (implement's run of the block, 2026-10-07T08:07Z-08:09Z, summary `entries 20 · green 17 · red 0 · recorded 0 · timeout 0 · not-run 3`; the tree it ran on is the tree `25d9b72d` carries):
- `cargo test -p escher-telemetry --locked --lib outside_target` — green (exit 0 · lacks `ok. 0 passed` · lacks `FAILED`; 5 passed)
- `cargo test -p escher-telemetry --locked` — green (exit 0 · lacks `FAILED`; 10 passed)
- `cargo test -p blitz-tests --locked --test telemetry_drop` — green (exit 0 · contains `1 passed; 0 failed; 1 ignored`)
- `cargo test -p blitz-tests --locked --test telemetry_scrub --test telemetry_stdout_silent --test telemetry_panic_hook --test telemetry_init_idempotent` — green
- `cargo test -p seven_guis --locked --test host_log` — green (contains `1 passed; 0 failed`)
- `cargo test -p seven_guis --locked --test host_binary` — green (contains `2 passed; 0 failed`)
- `git diff --quiet ebd7411f… -- …` (the preservation entry) — green (exit 0)
- `cat packages/escher-telemetry/src/*.rs | grep -cE '^static |env::var|set_var|thread::|Listener|UdpSocket'` — green (exit 0 · last line `2`)
- `test -f … host_log.rs && test -f … telemetry_drop.rs && grep -rhoE … | grep -vc '^e'` (the print-macro census) — green (exit 1 · last line `0`)
- `cargo build -p seven_guis --bins --locked` — green
- `f=$(mktemp); RUST_LOG=trace timeout 10 target/debug/seven_guis_native …` (the windowed entry, `env` `WAYLAND_DISPLAY` set) — green (exit 0 · contains `id-lines 0 name-lines 0`)
- `bash scripts/agent-run.sh boot` — green
- `bash scripts/agent-run.sh run stand` — green (contains `"passed": 72, "failed": 0`; artifact fresh)
- `bash scripts/agent-run.sh cleanup` — green
- `bash .github/scripts/ci-leg.sh fast` — green
- `bash .github/scripts/ci-leg.sh doc` — green
- `bash .github/scripts/ci-leg.sh a11y` — green
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`: driven by hand in the operator pass, exit 0, `hygiene: clean`, fired twice
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` — `leg = 'operator'`: driven by hand, exit 0, push `ebd7411f..25d9b72d`
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — `leg = 'operator'`: driven by hand, exit 0, `verdict: green · checks 16/16`, CI#37592418443
- No entry carried `defer`; nothing was skipped or voided. Smoke: both sink-installing binaries were driven by hand on the fixed build through the instrument (four settings each) and again as the two `role = 'smoke'` entries above.

Watches: none folded.

Outcome basis: the operator pass ran, so the verdicts rest on its final state — one pass commit, `25d9b72d`, and the fork's CI run on it, both recorded in `evidence/operator-pass.md`. No fix commit followed. Implement's P4 report (held in this conversation) is the basis for the by-level readings, the red runs and the deviations; nothing between it and the pass changed source, tests or manifests.

Process hygiene (implement's census at 08:10Z, re-measured from the host's process list at this wrap, 08:27Z): `escher-session` (the instrument ×8, `host_log`, `host_binary`, the workspace runs) — started by this run — terminated, none in the list · `seven_guis_native` (the instrument ×8, the windowed gate entry) — started by this run — terminated by `timeout 10`, none in the list · `agent-run.sh` boot / run / cleanup — started by this run — terminated · gate and cargo trees under two other projects — started by other sessions — left running, not this chunk's, untouched.

### What the drop loses
Directed at the P5 review: "In the report, list what the drop loses by target, level and count for both binaries,
WARN and ERROR separately." — the operator, 2026-10-07. The listings below are computed from the four readings above
by row, not written by hand. A third-party WARN or ERROR no longer reaches stderr at any level; the readings hold one
such row, on the windowed stand.

#### Session host — `escher-session crud`

Every target·level row the before-reading holds and the after-reading lacks; a cell is that row's line count at that `RUST_LOG` setting in the before-reading (0 = the row did not emit there). After the fix every cell of every row below is 0.

##### WARN rows lost

none

##### ERROR rows lost

none

##### INFO rows lost

none

##### DEBUG rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `selectors::matching` | 0 | 0 | 112 | 112 |
| `style::data` | 0 | 0 | 4 | 4 |
| `style::invalidation::stylesheets` | 0 | 0 | 24 | 24 |
| `style::rule_cache` | 0 | 0 | 17 | 17 |
| `style::rule_tree::core` | 0 | 0 | 731 | 731 |
| `style::sharing` | 0 | 0 | 47 | 47 |
| `style::style_resolver` | 0 | 0 | 125 | 125 |
| `style::stylesheet_set` | 0 | 0 | 7 | 7 |
| `style::stylist` | 0 | 0 | 4 | 4 |
| `style::traversal` | 0 | 0 | 93 | 93 |
| **10 rows, lines** | **0** | **0** | **1164** | **1164** |

##### TRACE rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `dioxus_core::diff::node` | 0 | 0 | 0 | 12 |
| `dioxus_signals::signal` | 0 | 0 | 0 | 20 |
| `style::sharing` | 0 | 0 | 0 | 35 |
| `style::style_resolver` | 0 | 0 | 0 | 163 |
| `style::traversal` | 0 | 0 | 0 | 86 |
| `warnings::warnings` | 0 | 0 | 0 | 20 |
| **6 rows, lines** | **0** | **0** | **0** | **336** |

##### Lines off the sink's line shape

none — every stderr line of every before- and after-reading had the sink's line shape.

##### Rows kept

`INFO escher_telemetry`

#### Windowed stand — `seven_guis_native`, on Home

Every target·level row the before-reading holds and the after-reading lacks; a cell is that row's line count at that `RUST_LOG` setting in the before-reading (0 = the row did not emit there). After the fix every cell of every row below is 0.

##### WARN rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `winit_wayland::window::state` | 1 | 1 | 1 | 1 |
| **1 rows, lines** | **1** | **1** | **1** | **1** |

##### ERROR rows lost

none

##### INFO rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `wgpu_hal::vulkan::adapter` | 0 | 1 | 1 | 1 |
| **1 rows, lines** | **0** | **1** | **1** | **1** |

##### DEBUG rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `naga::front` | 0 | 0 | 3185 | 3185 |
| `naga::front::wgsl::lower` | 0 | 0 | 450 | 450 |
| `naga::front::wgsl::lower::conversion` | 0 | 0 | 1246 | 1246 |
| `naga::proc::overloads::list` | 0 | 0 | 3915 | 3915 |
| `naga::proc::typifier` | 0 | 0 | 1217 | 1217 |
| `naga::valid::expression` | 0 | 0 | 1111 | 1111 |
| `naga::valid::function` | 0 | 0 | 142 | 142 |
| `naga::valid::interface` | 0 | 0 | 65 | 65 |
| `sctk` | 0 | 0 | 13 | 13 |
| `selectors::matching` | 0 | 0 | 91 | 91 |
| `style::data` | 0 | 0 | 7 | 7 |
| `style::invalidation::stylesheets` | 0 | 0 | 27 | 27 |
| `style::rule_cache` | 0 | 0 | 14 | 14 |
| `style::rule_tree::core` | 0 | 0 | 402 | 402 |
| `style::sharing` | 0 | 0 | 67 | 67 |
| `style::style_resolver` | 0 | 0 | 80 | 80 |
| `style::stylesheet_set` | 0 | 0 | 5 | 5 |
| `style::stylist` | 0 | 0 | 8 | 8 |
| `style::traversal` | 0 | 0 | 161 | 161 |
| `wgpu_core::device::resource` | 0 | 0 | 2 | 2 |
| `wgpu_core::instance` | 0 | 0 | 6 | 6 |
| `wgpu_hal::gles::adapter` | 0 | 0 | 6 | 6 |
| `wgpu_hal::gles::egl` | 0 | 0 | 16 | 16 |
| `wgpu_hal::vulkan::adapter` | 0 | 0 | 24 | 24 |
| `wgpu_hal::vulkan::instance` | 0 | 0 | 150 | 150 |
| **25 rows, lines** | **0** | **0** | **12410** | **12410** |

##### TRACE rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `calloop::loop_logic` | 0 | 0 | 0 | 14 |
| `calloop::sources` | 0 | 0 | 0 | 11 |
| `dioxus_core::diff::node` | 0 | 0 | 0 | 20 |
| `dioxus_signals::signal` | 0 | 0 | 0 | 17 |
| `naga::back::spv::writer` | 0 | 0 | 0 | 622 |
| `naga::compact` | 0 | 0 | 0 | 1947 |
| `naga::compact::expressions` | 0 | 0 | 0 | 9870 |
| `naga::compact::functions` | 0 | 0 | 0 | 142 |
| `naga::compact::handle_set_map` | 0 | 0 | 0 | 20854 |
| `naga::proc::constant_evaluator` | 0 | 0 | 0 | 986 |
| `naga::proc::type_methods` | 0 | 0 | 0 | 4 |
| `style::sharing` | 0 | 0 | 0 | 58 |
| `style::style_resolver` | 0 | 0 | 0 | 102 |
| `style::traversal` | 0 | 0 | 0 | 156 |
| `warnings::warnings` | 0 | 0 | 0 | 17 |
| `wgpu_core::command` | 0 | 0 | 0 | 12 |
| `wgpu_core::command::pass` | 0 | 0 | 0 | 16 |
| `wgpu_core::command::render` | 0 | 0 | 0 | 12 |
| `wgpu_core::device::global` | 0 | 0 | 0 | 80 |
| `wgpu_core::device::queue` | 0 | 0 | 0 | 11 |
| `wgpu_core::device::resource` | 0 | 0 | 0 | 18 |
| `wgpu_core::device::surface_config` | 0 | 0 | 0 | 4 |
| `wgpu_core::instance` | 0 | 0 | 0 | 5 |
| `wgpu_core::resource` | 0 | 0 | 0 | 16 |
| `wgpu_hal::gles::adapter` | 0 | 0 | 0 | 1 |
| `wgpu_hal::gles::egl` | 0 | 0 | 0 | 70 |
| `wgpu_hal::vulkan::instance` | 0 | 0 | 0 | 4 |
| **27 rows, lines** | **0** | **0** | **0** | **35069** |

##### Lines off the sink's line shape

none — every stderr line of every before- and after-reading had the sink's line shape.

##### Rows kept

`INFO escher_telemetry`
