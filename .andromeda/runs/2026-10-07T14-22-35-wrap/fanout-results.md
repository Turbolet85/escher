# Fan-out results — 2026-10-07-act-by-id

Seven detector returns, collected from their transcripts by script (entity-decoded; the probe
`entities=0` held on every return after decoding). Proposal ids are this file's: a letter per doc
and the proposal's position in its return.

## Verdicts

- **architecture** — 22 proposals (D-arch-resources 18 · D-arch-decisions 4; graded `warning` 22). Stripped beside the list: 13 comment lines; decoding changed nothing.
- **security-plan** — 13 proposals (D-security-input 12 · D-security-auth 1; graded `escalate` 8 · `warning` 5). Stripped beside the list: 10 comment lines; decoding changed nothing.
- **design-system** — `proposals: []`. Stripping removed 3 comment lines; the raw return is `.raw-fanout-design-system.md`; decoding changed nothing.
- **layout-templates** — `proposals: []`. Stripping removed 13 comment lines; the raw return is `.raw-fanout-layout-templates.md`; decoding changed nothing.
- **test-plan** — 16 proposals (D-tests-coverage 16; graded `warning` 16). Stripped beside the list: 6 comment lines; decoding changed nothing.
- **obs-plan** — 10 proposals (D-obs-stack 5 · D-obs-instrumentation 4 · D-obs-pii 1; graded `warning` 10). Stripped beside the list: 7 comment lines; decoding changed nothing.
- **a11y-plan** — `proposals: []`. Stripping removed 41 comment lines; the raw return is `.raw-fanout-a11y-plan.md`; decoding changed nothing.

Total: 61 proposals over 4 docs; 3 docs returned none.

## Validate — dispositions

**Before the checks — the re-derivation tell.** One proposal rests on a fact the report does not carry:
**O2**'s claim that the six `stand_act_*` checks install no sink and print nothing is the detector's own grep
("The report does not itself measure print/subscriber absence in the new test files"). **O2 — REJECTED as
proposed**, and its fact re-raised as **X8** from the orchestrator's own measurement (below). Its other half,
the `session_common` reader count 11, is the report's and rides X8. Every other proposal that mentions a read
of its own (O3, O9, O10; the security return's spot-checks) rests on a count or a coordinate the report
carries and uses the read as a check of it: not the tell.

1. **Playbook.** Sixty proposals match "Accurate this-chunk addition" (routine): each names a fact the report's
   Changes carry as this chunk's work — the executor's three public items, the two refusals it returns, the
   feature it names, the counts it moves, the citations its edits shift — landing in an existing section, with
   the detector's invariant holding. "Registry over-reach" does not govern A1–A3: §Standard Contracts states
   contract shapes, and the three chunks before this one registered theirs there. The eight `escalate` grades on
   security-plan (S1–S8) are the detector's own severity for a VIOLATION of D-security-input; the detector
   reports that invariant HOLDING ("code side HOLDS … no unvalidated boundary was found"), and the plan's
   reviewed `Expected amendments (wrap)` list names the change (entry 3: the `id` row's first consumer, the
   schema row reading "executed in process", typed text still not measured; entry 1: the "Not built" list losing
   "anything that runs a `Command`") — the rule the founder ruled appended at this wrap (R2,
   `operator-rulings.md`) and amendment-flow's recorded-direction clause both apply them without a halt. S3, S4,
   S6 and S7 (and A13–A15 on the architecture side) are `dependent-of` S1 / A1: other wordings of the one
   retired claim, that nothing executes a command, validated with their primary.
   **Not a boundary widening, and why:** nothing new crosses a hardened boundary. A `Call` is built by the
   caller and an `Outcome` returned to it as values in one process; the socket's wire, host, client and error
   files are byte-identical (the preservation gate). The crossing question the route carried — "the first
   command that returns [the diff] asks the crossing question again" — was asked at this chunk's plan forks and
   answered: in process (the operator, 2026-10-07, at the P4 forks; scope.md, plan.md §Goal). Three of the
   clauses reworded carry "ratified by the founder, 2026-10-07" for the rule that a snapshot's text and a diff
   are **returned to their caller only**; that rule is kept word for word with its ratification, and the status
   clause beside it ("no driver, CLI or MCP command exposes it yet") is replaced by what is now true, under the
   operator's fork answer and named as such. No PROVISIONAL mark is touched: none stands in the seven masters
   (`grep -i PROVISIONAL`, 0 hits), and the fork answer is recorded as the operator's own.
2. **Cross-contradiction.** None. Where two docs state one fact they agree: the reader counts (A17 · T10 · T11 ·
   T12 · O3 · X8), the timer-host gap (A16 · T1 · T2), the typed-text reading (S5 · S8 · O8), the feature
   (A19–A22 · X1).
3. **Intent-consistency.** The report's deviations are each justified and none leaves the entry's intent: no
   scope-record line (`gate.py scope` clean), no new listener, port, env var, crate or binary. One deviation is
   procedural — the operator pass was driven by the implementing agent on the operator's quoted word.
4. **Absence needs evidence.** "No standing check boots the timer host": `grep -rn 'CARGO_BIN_EXE_escher-session|\.arg("…")'`
   over `examples/seven_guis/tests`, `tests/blitz-tests/tests`, `scripts`, `.github` — 2 spawn sites,
   `host_binary.rs:30` (`counter`) and `host_log.rs:77` (`crud`); both read. "The new checks install no sink":
   `grep -cE 'println!|eprintln!|print!|dbg!|escher_telemetry|tracing::|env::var|log::'` over the six
   `stand_act_*` files and `session_common/mod.rs` — 0 in each of the seven. The masters' sites were read by
   offset window (architecture.md holds 26 lines over 2 000 chars). The detectors found **nine sites the
   report's own searches missed**, each a different wording of a claim the report names: `architecture.md:154`
   ("with no feature named"), `:136` ("only by naming the feature … seven_guis names none"; "the callers of
   `to_text` are tests"), `:138` (the tick handle "held until `serve` returns"), `:263` and `:115` (the reader
   counts by kind), `security-plan.md:117` and `:118` ("…or command"), `obs-plan.md:69` ("eleven of which"),
   `test-plan.md:21` ("25 inline unit tests in six files"), `test-plan.md:99` (the reader list by name). All
   nine are applied; the report's site lists are corrected in a marked note.
5. **Expected amendments.** Entries 1–5 are covered by proposals (1: A1, A4–A12 · 2: A2, A19–A22 · 3: S1–S8 ·
   4: T2, T4–T7, T13–T15 · 5: O1, O5–O8). Entries 6, 7 and 8 drew none — no detector's invariant reaches a
   feature-reach row, a motion bullet or a bare citation — and are raised by the orchestrator, routine, the
   report substantiating each: **X1** (a11y-plan §1, the Dioxus crates row) · **X2** (design-system §Motion →
   Animation runtime) · **X3** (layout-templates, two `session_host.rs` citations). The citation-only moves the
   detectors listed outside their invariants are raised the same way: **X4** (`architecture.md:149`, `:150` ×2) ·
   **X5** (`test-plan.md:49`) · **X6** (`test-plan.md:87`) · **X7** (`session-lifecycle.md:8`). With them every
   one of the report's 30 moved coordinates has an owner, and the 11 coordinate-kept citations were each re-read
   against their claim (six `Cargo.toml:13-15`: three amended with their clause by A19–A21 and X1, three still
   true as cited — two dependencies, no `tracing`; five `session_common/mod.rs:1-6`: still the module's doc
   lines).
6. **Disproved claims.** (1) the `host_binary` gate note → the gap recorded by A16 · T1 · T2, owned on the
   route by a CARRY (P5), the note's hazard to curation. (2) "controls with an effect at boot" on four tasks →
   stated by T9 as built; the acceptance holds as written, so no premise correction. (3) the click that does not
   focus a range input → `evidence/range-input.md`; the route entry R1 creates owns it. (4) the range hypothesis,
   now measured and holding → discharged on the route into R1's entry. (5) the plain-button focus reading → added
   to the CARRY on "Stand keyboard harness" as a reading, not a controlled measurement. All five DISPOSED.

**Dispositions.** A1–A22 apply · S1–S13 apply · T1–T16 apply · O1, O3–O10 apply · O2 reject (re-raised as X8) ·
X1–X8 raised and applied. Applied text is re-derived from the report's facts, never pasted from a `change`
line. **Escalations: 0.**

**Rule appended on the founder's word (R2).** `.andromeda/playbook.md` gains the rule proposed at the
2026-10-07T12-34-00 wrap, its `pattern` that wrap's own sentence (the handoff it wrote holds it; the run dir
holds the case, `fanout-results.md` check 1, and no second wording).

**X8 — raised.** obs-plan §3 Logging stack: the six `stand_act_*` checks join the checks that install no
subscriber and print nothing (the census above, 0 in each), and `session_common/mod.rs` is read by eleven
checks — four of the five `stand_session_*`, `stand_settle` and the six `stand_act_*`.

## architecture — the parsed list (22)

### A1 · D-arch-resources · warning
- **section:** §Standard Contracts → Driver session (escher-driver)
- **change:** Register the executor beside `act`: `Session::run(&mut self, call: &Call) -> Result<Outcome, Refusal>` (packages/escher-driver/src/execute.rs:60) is the one public entry that runs a call on a held instance, in process — `validate(call)` first, its refusal returned before anything runs; `snapshot` returns `DioxusDocument::snapshot().to_text()` and dispatches nothing; `click` resolves the id and clicks the centre of the node's border box (`Harness::click_at`); `type` clicks the same way then `Harness::type_text`; `press` goes through `Harness::press_with` (`Modifiers::SHIFT` when `shift`), with one `#[cfg(target_os = "macos")]` arm that also dispatches `deleteBackward:` through `Harness::apple_keybinding` for `backspace`; `advance` hands `ms` to the session's time step; each acting verb is one settled step (snapshot before, the step, `Harness::settle`, snapshot after, `before.diff(&after)`). `Outcome` (execute.rs:15, re-exported, `Debug, Clone, PartialEq`) is `Screen { text }` · `Acted { settled, busy: Option<Busy>, diff: SnapshotDiff }` · `Advanced { settled, busy, diff, advanced_ms: u32 }`; a step that did not go quiet is a result, not rolled back. `Session::with_time(self, step: impl FnMut(&mut Harness<DioxusDocument>, u32) -> u32 + 'static) -> Session` (session.rs:60) carries the caller's time step; `advanced_ms` is `step(harness, ms).min(ms)` — the time delivered to the app, not what a counter inside it gained. Id lookup is a linear search of `element_ids()` then `get_node`; no id or `NodeId` is kept between calls. Nothing of a `Call` or an `Outcome` crosses the socket — as measured at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md.
- **sidecar:** architecture §Standard Contracts → Driver session: registered `Session::run`, `Outcome`, `Session::with_time`, the id lookup, the macOS `press` arm and the `advanced_ms` rule (2026-10-07-act-by-id).
- **rationale:** Report Changes → Symbols / APIs (report.md:16-22) lands three public symbols and their per-verb behaviour; `Session::run`, `with_time`, `timer_step`, `apple_keybinding` each grep 0 hits in architecture.md, and `Outcome` hits only the telemetry line 132. Expected amendment 1 (report.md:90) is carried by the plan.
- **basis:** architecture.md:133 (0 hits for `Session::run` / `with_time`); report.md:16-18

### A2 · D-arch-resources · warning
- **section:** §Standard Contracts → Test harness
- **change:** The export list gains `Key` and `Modifiers` — re-exports of `keyboard_types::Key` / `keyboard_types::Modifiers`, the two types `Harness::press_with` takes — with its citation re-pointed `packages/blitz-test-harness/src/lib.rs:23-26` → `:23-27`; the input-helper list gains `apple_keybinding(&mut self, command: &str)` (packages/blitz-test-harness/src/input.rs:229), which dispatches `UiEvent::AppleStandardKeybinding(command)` to the focused element and pumps like every other input helper (it does not settle; no existing helper changes).
- **sidecar:** architecture §Standard Contracts → Test harness: exports `Key`, `Modifiers` and the input helper `apple_keybinding` registered; lib.rs citation :23-26 → :23-27 (2026-10-07-act-by-id).
- **rationale:** Report Changes → Symbols / APIs lists both as new (report.md:23-24) and Harness / gate surface repeats it (report.md:71); the contract's export list and input list name neither (0 hits for `apple_keybinding`, `Modifiers`). The measured line map moves the cited range (report.md:60).
- **basis:** architecture.md:130@c247 (`lib.rs:23-26`), @c850 (input list); report.md:23-24, :60

### A3 · D-arch-resources · warning
- **section:** §Standard Contracts → Headless stand (seven_guis, native only)
- **change:** `seven_guis::stand` also exports `TIMER_TICK_MS: u32 = 100` (examples/seven_guis/src/stand.rs:86) and `timer_step(ticks: TimerTicks) -> impl FnMut(&mut Harness<DioxusDocument>, u32) -> u32` (stand.rs:92): it delivers `ms / 100` whole ticks through `TimerTicks::deliver`, returns that count × 100, drops the remainder per call (an ask below one tick delivers 0) and leaves the harness's animation clock where it was, the ticks applying on the next pass; the section's citation moves `stand.rs:14-103` → `:14-118`.
- **sidecar:** architecture §Standard Contracts → Headless stand: `TIMER_TICK_MS` and `timer_step` registered; stand.rs citation :14-103 → :14-118 (2026-10-07-act-by-id).
- **rationale:** Report Changes → Symbols / APIs lists both as new, public (report.md:25); the stand contract enumerates the module's public surface and names neither (0 hits). Line map: report.md:62.
- **basis:** architecture.md:131@c792; report.md:25, :62, :119

### A4 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → Driver session (escher-driver)
- **change:** "the crate re-exports 32 names" → 33 names: the 32 and `Outcome`, from the private module `execute`; citation `packages/escher-driver/src/lib.rs:27-49` → `:33-57`.
- **sidecar:** architecture Driver session: re-export count 32 → 33 (`Outcome`); lib.rs citation :27-49 → :33-57.
- **rationale:** Report Crates / modules (report.md:30) and Counts (report.md:39): 33 names, the 33rd is `Outcome`; line map report.md:58.
- **basis:** architecture.md:133@c60, @c332

### A5 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → Driver session (escher-driver)
- **change:** The `Session` method list "offering `label()`, `harness()`, `harness_mut()` and `act(...)`" gains `run` and `with_time`, and "the session holds a label and the harness only" becomes: a label, the harness and an optional caller-supplied time step (`time: Option<TimeStep>`, a crate-private boxed `FnMut`; `harness` and `time` are `pub(crate)`) — still no `NodeId`, no viewport, font or scheme setting, no subscriber, and it neither reads nor drains the changed set; citation `packages/escher-driver/src/session.rs:11-79` → `:11-99`.
- **sidecar:** architecture Driver session: `Session` holds a label, the harness and an optional time step; session.rs citation :11-79 → :11-99.
- **rationale:** Report Symbols / APIs: `Session` gains one field, `time: Option<TimeStep>` (report.md:18) and Deviations (report.md:110); the `NodeId` / changed-set half is re-asserted (report.md:21, :144). Line map report.md:59.
- **basis:** architecture.md:133@c684, @c1097, @c1346

### A6 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → Driver session (escher-driver)
- **change:** "The command and refusal schema is stated once, as `'static` data held in process — no item of it is reachable from the socket and no `Session` method takes one" → no item of it is reachable from the socket; one `Session` method, `run`, takes a `Call` and executes it in process.
- **sidecar:** architecture Driver session: retired "no `Session` method takes one" — `Session::run(&Call)` does, in process.
- **rationale:** Report Counts → "Held in process" clauses names this site as moved by `Session::run(&Call)` (report.md:53); Schema / config: the schema is now executed in process where it was stated and validated only (report.md:34).
- **basis:** architecture.md:133@c4155

### A7 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → Driver session (escher-driver)
- **change:** "`advance` asks for the app's time to move by `ms` milliseconds; the step that moves it is the session's caller's to supply and is not wired" → the step is the session's caller's to supply through `Session::with_time`; a session started without it has no step and `advance` on it is refused `time-unavailable` with nothing run.
- **sidecar:** architecture Driver session: the `advance` time step is wired through `Session::with_time` (was "not wired").
- **rationale:** Report Counts → "Not built" list: the `@c5629` clause "is wired" (report.md:51); Symbols / APIs (report.md:18-19).
- **basis:** architecture.md:133@c5630

### A8 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → Driver session (escher-driver)
- **change:** "`validate` returns `unknown-verb` and `malformed` only: nothing yet returns the other six causes" → `validate` returns `unknown-verb` and `malformed` only; the executor returns `not-found` (an id no pair of `element_ids()` carries, or whose node does not resolve through `get_node`) and `time-unavailable` (`advance` on a session with no time step), each with nothing run; `stale`, `disabled`, `covered` and `off-screen` are still returned by nothing — a click on a disabled or covered target runs and its diff says what changed.
- **sidecar:** architecture Driver session: `not-found` and `time-unavailable` are now returned (by `Session::run`); four causes remain returned by nothing.
- **rationale:** Report Symbols / APIs → Refusals the executor returns (report.md:19).
- **basis:** architecture.md:133@c7353; execute.rs:80-82, :125-134 per report.md:19

### A9 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → Driver session (escher-driver)
- **change:** "`Command`, `Call` and `ArgValue` derive `Debug`, which prints the id and the text they hold; nothing in the crate prints, logs or fields them" → add `Outcome`, whose `Debug` prints ids, accessible names and control values (a password's or file input's value as the snapshot's mask); still nothing in the crate prints, logs or fields any of the four.
- **sidecar:** architecture Driver session: `Outcome` joins the `Debug`-deriving types that print ids / names / values; the crate still prints and logs none.
- **rationale:** Report Deviations: `Outcome` derives `Debug`, the class the handoff records for `Command`, `Call` and `ArgValue` (report.md:111); Coverage PII note (report.md:101).
- **basis:** architecture.md:133@c7428

### A10 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → Driver session (escher-driver)
- **change:** "The three modules carry no `cfg` gate and use no serialization crate (…command.rs:161; packages/escher-driver/src/lib.rs:29-49)" → still true of command, refusal and schema; the fourth schema-side module, `execute`, carries exactly one — the `#[cfg(target_os = "macos")]` backspace arm (execute.rs:155-158); citation `lib.rs:29-49` → `:35-57`.
- **sidecar:** architecture Driver session: `execute` carries the crate's one `cfg` gate (macOS backspace); lib.rs citation :29-49 → :35-57.
- **rationale:** Report Symbols / APIs → macOS arm of `press`: "the one `cfg` gate in `execute.rs`" (report.md:22); line map report.md:58.
- **basis:** architecture.md:133@c7635, @c7775

### A11 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → Driver session (escher-driver)
- **change:** The "Not built" list loses "anything that runs a `Command` — the verb set is stated and validated in process and executed nowhere" and "act by id" (both built, in process); its parenthetical "a step settles in process only, through `act`" becomes "through `act` and `run`"; it keeps: a verb, a settle verb or busy-source reply on the socket (the wire reads `hello` and `stop` only), CLI JSON, an MCP tool, an idle expiry; the provenance tail gains "the executor at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md".
- **sidecar:** architecture Driver session: "Not built" list drops "anything that runs a `Command`" and "act by id"; socket verbs, CLI JSON, MCP tool and idle expiry remain unbuilt.
- **rationale:** Report Counts → "Not built" list (report.md:51): both are built; the socket verbs, CLI JSON, an MCP tool and an idle expiry are not. "Nothing crosses the socket" (report.md:27).
- **basis:** architecture.md:133@c7790, @c8035, @c8051

### A12 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Existing Scopes → escher-driver
- **change:** "eight private modules" → nine: add `execute` (`Session::run`, `Outcome` — the in-process executor, carrying the crate's one `cfg` gate) to the module list, and `with_time` beside `act` under session; the "held in process only and under no `cfg` gate" qualifier stays on command, refusal and schema only; citation `packages/escher-driver/src/lib.rs:29-49` → `:35-57`.
- **sidecar:** architecture §Existing Scopes → escher-driver: eight → nine private modules (`execute`); lib.rs citation :29-49 → :35-57.
- **rationale:** Report Crates / modules (report.md:30) and Counts (report.md:40): nine private modules, basis the `mod` lines of lib.rs:35-44; line map report.md:58.
- **basis:** architecture.md:257@c276, @c1022

### A13 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → Dioxus DOM bridge
- **change:** Snapshot clause: "and no driver, CLI or MCP command exposes the snapshot or that text yet" → the driver's `snapshot` command (`escher_driver::Session::run`) returns that text in process, to its caller only; no CLI or MCP command exists and nothing of it crosses the socket.
- **sidecar:** architecture Dioxus DOM bridge (snapshot): a driver command now returns the snapshot text in process; no CLI or MCP command.
- **rationale:** Report Counts → "No driver … command" clauses (report.md:48): the snapshot text at `architecture.md:136@c12554` is now returned by a driver command in process, to its caller only.
- **basis:** architecture.md:136@c12585

### A14 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → Dioxus DOM bridge
- **change:** `to_text` clause: "the text leaves the process through the returned value only — no log, event, socket, file or command carries the snapshot or its text, and the callers of `to_text` are tests" → still the returned value only — no log, event, socket or file carries it; one non-test caller exists, escher-driver's executor (packages/escher-driver/src/execute.rs:63), which returns it in process as `Outcome::Screen` to the caller of `Session::run`.
- **sidecar:** architecture Dioxus DOM bridge (`to_text`): first non-test caller, `execute.rs:63`; the text still leaves the process by returned value only.
- **rationale:** Report Counts → "Callers are tests" clauses (report.md:49): `Snapshot::to_text` now has one non-test caller, `execute.rs:63`. The report's search (`callers are tests`) listed only the `diff` site for architecture; this `callers of \`to_text\` are tests` wording is a second occurrence it did not hit.
- **basis:** architecture.md:136@c14961, @c15011

### A15 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → Dioxus DOM bridge
- **change:** Diff clause: "no log, event, socket, file or command carries it, no driver, CLI or MCP command exposes it yet, and the callers of `diff` are tests" → no log, event, socket or file carries it; the driver's four acting commands return it in process (`Outcome::Acted` / `Outcome::Advanced`), to the caller of `Session::run` only; no CLI or MCP command exists; `diff` has one non-test caller, packages/escher-driver/src/execute.rs:108. The neighbouring `unkeyed_actionable` clause ("no driver, CLI or MCP command exposes it yet") stays as worded.
- **sidecar:** architecture Dioxus DOM bridge (diff): returned by the driver's acting commands in process; first non-test caller `execute.rs:108`; the `unkeyed_actionable` clause unmoved.
- **rationale:** Report Counts (report.md:48-49): the diff at `architecture.md:136@c17456` is now returned by a driver command in process; `@c17510` "callers of `diff` are tests" has one non-test caller; the `unkeyed_actionable` clause at `@c19896` is unmoved.
- **basis:** architecture.md:136@c17437, @c17511

### A16 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Standard Contracts → CLIs
- **change:** `escher-session`: "the timer through `stand::boot_timer`, its tick handle held until `serve` returns" → the timer through `stand::boot_timer`, its tick handle given to `stand::timer_step` and that step attached to the session through `Session::with_time` before `serve` (the other three tasks get no step); argv check still first, no argument added, nothing printed, exit codes 0 · 1 · 2 unchanged; citation `examples/seven_guis/src/session_host.rs:5-57` → `:5-61`.
- **sidecar:** architecture §Standard Contracts → CLIs: `escher-session timer` hands `stand::timer_step` to its session via `with_time`; session_host.rs citation :5-57 → :5-61.
- **rationale:** Report Symbols / APIs → `escher-session` host (report.md:26); line map report.md:63. Covered by hand only — no standing check boots the timer host (report.md:83, :105).
- **basis:** architecture.md:138@c2190, @c2541

### A17 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Existing Scopes → blitz-tests
- **change:** Register the six act-by-id checks beside the session checks — `stand_act_ids`, `stand_act_diff`, `stand_act_timer`, `stand_act_refused`, `stand_act_keys`, `stand_act_range` (17 tests): driver calls run through `Session::run` on the four lean tasks, in both layout modes; and restate the two reader counts: `mod common;` is read by 14 stand checks (was eleven: + `stand_act_ids`, `stand_act_diff`, `stand_act_keys`) and `mod session_common;` by 11 (was "four of them and by `stand_settle`": + the six `stand_act_*`), `session_common` now also holding the driver's call builders (`click`, `type_into`, `press`, `advance`), the outcome reader (`Acted`, `act`) and `focused`.
- **sidecar:** architecture §Existing Scopes → blitz-tests: six `stand_act_*` checks registered; `mod common;` readers eleven → 14, `mod session_common;` readers five → 11.
- **rationale:** Report Files (report.md:10) and Counts (report.md:46-47). The report locates the reader counts at test-plan.md:65 only; the blitz-tests row restates both ("by eleven stand checks", "by four of them and by `stand_settle`") and would survive a test-plan-only apply. Its `session_common/mod.rs:1-6` citation keeps its coordinate but the range holds an edited line (report.md:65) — re-read on apply.
- **basis:** architecture.md:263@c3256, @c4479

### A18 · D-arch-resources · warning · dependent-of D-arch-resources
- **section:** §Conventions → Tests
- **change:** "the checks that hold a `Session` — the session checks and the settle check `stand_settle` — keep what they share in a second such module, `tests/blitz-tests/tests/session_common/mod.rs`" → the checks that hold a `Session` — the session checks, the settle check `stand_settle` and the six act-by-id checks `stand_act_*` — keep what they share there.
- **sidecar:** architecture §Conventions → Tests: the `session_common` readers now include the six `stand_act_*` checks.
- **rationale:** Report Counts (report.md:46): `mod session_common;` readers five → 11 (+ the six `stand_act_*`); this sentence enumerates the readers by kind and omits them. Its `session_common/mod.rs:1-6` citation is coordinate-kept with an edited line in range (report.md:65).
- **basis:** architecture.md:115@c3190

### A19 · D-arch-decisions · warning
- **section:** §Established Decisions → [Driver session]
- **change:** "The library names no `dioxus-native-dom` feature — the entry that first reads the snapshot through a session decides it" → the library names one feature, `accessibility`, on `dioxus-native-dom`, unconditionally and with no `[features]` table of its own (packages/escher-driver/Cargo.toml:15) — decided by 2026-10-07-act-by-id, the entry that first reads the snapshot through a session; through `seven_guis → escher-driver` both seven_guis binaries (`seven_guis_native` and `escher-session`) now compile dioxus-native-dom's accessibility modules and still build no platform adapter (`accesskit_xplat` / `accesskit_winit` in neither graph; `Cargo.lock` byte-identical); it still depends on blitz-test-harness and dioxus-native-dom only (`Key` / `Modifiers` come through the harness's re-exports), reads no env var, has no default state location, and carries no `tracing` dependency, span or event — the feature as measured at escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/feature-reach.md.
- **sidecar:** architecture §Established Decisions → [Driver session]: escher-driver names `accessibility` on dioxus-native-dom (was: names no feature, deferred to the first session snapshot reader); reach into both seven_guis binaries recorded.
- **rationale:** Report Dependencies (report.md:32): no dependency added or bumped, one feature named at `packages/escher-driver/Cargo.toml:15`; Counts → "Names no feature" clauses marks `architecture.md:101@c795` stale (report.md:52); acceptance (report.md:142, :160); expected amendment 2 (report.md:91). No new library or runtime, and no other locked decision is contradicted — the decision itself deferred this choice to this entry, so it is a restatement, not an escalation. The `Cargo.toml:13-15` citation keeps its coordinate while holding the edited line (report.md:65).
- **basis:** architecture.md:101@c796; packages/escher-driver/Cargo.toml:15 per report.md:32

### A20 · D-arch-decisions · warning · dependent-of D-arch-decisions
- **section:** §Occupied Resources → Names
- **change:** "escher-driver, a `publish = false` library with no features and no binary target, … depending on blitz-test-harness and dioxus-native-dom with no feature named" → … with no features of its own and no binary target, depending on blitz-test-harness and on dioxus-native-dom with its `accessibility` feature named (packages/escher-driver/Cargo.toml:13-15).
- **sidecar:** architecture §Occupied Resources → Names: escher-driver's dioxus-native-dom dependency now names `accessibility` (was "with no feature named").
- **rationale:** Same retired claim as the [Driver session] decision, worded "with no feature named" — outside the report's `names no .{0,40}feature` search (3 hits: architecture.md:101, :257, a11y-plan.md:14), so a three-site apply would leave it. Report Dependencies (report.md:32).
- **basis:** architecture.md:154@c1140

### A21 · D-arch-decisions · warning · dependent-of D-arch-decisions
- **section:** §Existing Scopes → escher-driver
- **change:** "it depends on blitz-test-harness and dioxus-native-dom only, names no feature of either and no app" → it depends on blitz-test-harness and dioxus-native-dom only, names one feature — dioxus-native-dom's `accessibility` — and no app (packages/escher-driver/Cargo.toml:13-15).
- **sidecar:** architecture §Existing Scopes → escher-driver: "names no feature of either" → names dioxus-native-dom's `accessibility`.
- **rationale:** Report Counts → "Names no feature" clauses: `architecture.md:257@c955` stale (report.md:52); Dependencies (report.md:32).
- **basis:** architecture.md:257@c956, @c1059

### A22 · D-arch-decisions · warning · dependent-of D-arch-decisions
- **section:** §Standard Contracts → Dioxus DOM bridge
- **change:** Feature-gating clause: "a crate taking either Dioxus crate with `default-features = false` (the workspace pin) gets the override, the snapshot model, its text form, its diff and the actionable-key check only by naming the feature — blitz-tests names it on dioxus-native-dom, seven_guis names none" → … by naming the feature or by depending on a crate that names it — blitz-tests and escher-driver name it on dioxus-native-dom; seven_guis names none in its own manifest and gets it through its `escher-driver` edge, so both its binaries compile these modules, with no platform adapter (add `packages/escher-driver/Cargo.toml:15` to the clause's citations).
- **sidecar:** architecture Dioxus DOM bridge (feature gating): escher-driver names `accessibility`; seven_guis names none but now builds it through escher-driver.
- **rationale:** Restates the retired "the session path names no feature" claim by actor: "seven_guis names none" is true of its manifest (byte-identical, report.md:12) but the clause asserts the feature is reached "only by naming" it, which the measured reach contradicts — both seven_guis binaries now compile dioxus-native-dom's accessibility modules (report.md:32). Not in the report's site list for architecture.
- **basis:** architecture.md:136@c9704

### architecture — the comment lines stripped from the return

```text
  # ---------- D-arch-resources: primaries (new symbols the registry sections do not carry) ----------
  # ---------- D-arch-resources: other occurrences of the claims the primary retires ----------
  # ---------- D-arch-decisions ----------
# Not proposals — citation-only moves in architecture that fall outside both detectors' invariants
# (no claim changes; coordinates from the report's measured line map, report.md:56-65):
#   architecture.md:149@c3659  tests/blitz-tests/tests/session_common/mod.rs:70-78  -> :194-202   (§Occupied Resources → Filesystem)
#   architecture.md:150@c4321  tests/blitz-tests/tests/session_common/mod.rs:81-145 -> :205-269   (§Occupied Resources → Process-wide state and threads)
#   architecture.md:150@c5061  examples/seven_guis/src/session_host.rs:5-55         -> :5-59      (same entry)
# Checked and found unmoved: §Stack (no library or runtime added; keyboard-types and accesskit already listed),
# §Occupied Resources → Network ports and listeners (line 146: the socket still carries `hello` and `stop` only; no new
# socket, port, listener, env var, IPC method, crate or binary — report.md:27-28), §Inherited Defaults (line 221, 224),
# and the `unkeyed_actionable` "no driver, CLI or MCP command exposes it yet" clause (architecture.md:136@c19896).
# Offsets (@cN) are 1-based character offsets measured on the current architecture.md; the report's are one lower.
```

## security-plan — the parsed list (13)

### S1 · D-security-input · escalate
- **section:** §Input Validation → Driver command schema (escher-driver)
- **change:** Replace "held in process only — no socket, CLI or MCP tool reaches it yet, so it is the check every later surface's input passes through and not yet an external-input surface" with: executed in process since 2026-10-07-act-by-id — `Session::run(&Call)` calls `validate` first and returns its refusal before anything else runs, and nothing public runs a `Command` that did not come from `validate`; an `id` is then looked up by a linear search of `DioxusDocument::element_ids()` and its node resolved through `get_node`, an id no pair carries or whose node does not resolve returning `not-found` with nothing run; `advance` on a session with no time step returns `time-unavailable` with nothing run, and the reported `advanced_ms` is clamped to the `ms` asked; the executor has no `unwrap`, `expect`, `panic!` or `unreachable!` on what a call supplies and keeps no id or `NodeId` between calls; still no socket, CLI or MCP tool reaches it — a `Call` is caller-built and an `Outcome` returned as values in process — so it is not yet an external-input surface; `Outcome` joins `Command`, `Call` and `ArgValue` in deriving `Debug`, which prints the ids, accessible names and control values it holds (a password's or file input's value as the mask), and nothing in the crate prints, logs or fields them (add packages/escher-driver/src/execute.rs:60; packages/escher-driver/src/execute.rs:125-134) — as measured at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md
- **sidecar:** 2026-10-07-act-by-id: Driver command schema row — the schema is now executed in process by `Session::run` (validate first, id resolved through `element_ids()`/`get_node`, `not-found` and `time-unavailable` returned); still no socket, CLI or MCP reach.
- **rationale:** Report Symbols/APIs: `Session::run` (execute.rs:60) is new and public, "calls `validate(call)` first and returns its refusal before anything else runs"; Schema/config: the schema "is now executed in process where it was stated and validated only"; Counts "Held in process" names `security-plan.md:76@c1140` as moved by `Session::run(&Call)`; Expected amendment 3 (carried) asks the row to read "executed in process". Refusals, id lookup and the `.min(ms)` clamp are from the report's Symbols/APIs bullets; `Outcome` deriving `Debug` from Deviations. The validation mechanism itself is present (Coverage: validation mechanism✓) — this is the row's wording, not an unvalidated boundary.
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:76 ; packages/escher-driver/src/execute.rs:60-61

### S2 · D-security-input · escalate · dependent-of D-security-input
- **section:** §Input Validation → Markup attributes · `id` (stable element id)
- **change:** Add the id's first consumer that takes an id IN: since 2026-10-07-act-by-id the driver's `click` and `type` verbs, through `Session::run`, take a caller-supplied id (already bounded 1 to 1024 bytes by `validate`), find it by a linear search of `element_ids()` for the first pair whose id equals the argument and resolve its node through `BaseDocument::get_node` (never an index); an id no pair carries, or whose node does not resolve, returns `not-found` with nothing run and no panic; the session keeps no id and no `NodeId` between calls and no field of `Session`, `Outcome` or `Refusal` holds one; in process only — nothing crosses the socket, so the platform adapter stays the only exit escher's own code gives the id (packages/escher-driver/src/execute.rs:125-134; tests/blitz-tests/tests/stand_act_ids.rs; tests/blitz-tests/tests/stand_act_refused.rs).
- **sidecar:** 2026-10-07-act-by-id: `id` row — first consumer that takes an id in (`Session::run`, in process), resolved through `element_ids()` and `get_node`, `not-found` otherwise.
- **rationale:** Report Expected amendment 3 (carried): "the `id` row gains its first consumer that takes an id in, in process, resolved through `element_ids()` and `get_node`"; Symbols/APIs "Id lookup" and "Refusals the executor returns"; Outcome (v010-03) and (security) rows met by `stand_act_ids` / `stand_act_refused`. The row lists four readers of the id and no consumer of one as input.
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:116 ; packages/escher-driver/src/execute.rs:126-134

### S3 · D-security-input · escalate · dependent-of D-security-input
- **section:** §Input Validation → Markup attributes · `id` (stable element id)
- **change:** In the `Snapshot::to_text` clause, replace "no driver, CLI or MCP command exposes either yet, and `to_text`'s callers are tests" with: since 2026-10-07-act-by-id the driver's `snapshot` verb returns that text (`Outcome::Screen { text }`) through `Session::run`, in process and to its caller only — `to_text`'s one non-test caller (packages/escher-driver/src/execute.rs:63); no CLI or MCP command exists and nothing crosses the socket. Keep "no log, event, socket or file carries the snapshot or its text" and the "platform adapter stays the only exit" conclusion.
- **sidecar:** 2026-10-07-act-by-id: `id` row — the snapshot text is now returned by the driver's `snapshot` verb in process; `to_text` has one non-test caller (execute.rs:63).
- **rationale:** Report Counts '"No driver … command" clauses that the executor moves': the snapshot text at `security-plan.md:116@c3709` is "now returned by a driver command in process, to its caller only; no CLI or MCP command exists"; '"Callers are tests" clauses': `Snapshot::to_text` now has one non-test caller, `execute.rs:63` (`security-plan.md:116@c3775`).
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:116 ; packages/escher-driver/src/execute.rs:63

### S4 · D-security-input · escalate · dependent-of D-security-input
- **section:** §Input Validation → Markup attributes · `id` (stable element id)
- **change:** In the `Snapshot::diff` clause, replace "no driver, CLI or MCP command exposes it yet, and its callers are tests" with: since 2026-10-07-act-by-id each acting driver verb (`click`, `type`, `press`, `advance`) returns the diff of its one settled step in its `Outcome` through `Session::run`, in process and to its caller only — `diff`'s one non-test caller (packages/escher-driver/src/execute.rs:108); no CLI or MCP command exists and nothing crosses the socket. Keep "no log, event, socket or file carries it" and the adapter conclusion. Leave the `unkeyed_actionable` clause ("no driver, CLI or MCP command yet") as it is.
- **sidecar:** 2026-10-07-act-by-id: `id` row — a diff is now returned by the driver's acting verbs in process; `diff` has one non-test caller (execute.rs:108); the `unkeyed_actionable` clause is unmoved.
- **rationale:** Report Counts: the diff at `security-plan.md:116@c5387` is "now returned by a driver command in process, to its caller only"; `Snapshot::diff` has one non-test caller, `execute.rs:108` (`security-plan.md:116@c5441`); "The two `unkeyed_actionable` clauses (… `security-plan.md:116@c4745`) are unmoved — no driver command exposes that check."
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:116 ; packages/escher-driver/src/execute.rs:108

### S5 · D-security-input · escalate · dependent-of D-security-input
- **section:** §Input Validation → Markup attributes · `aria-label` · `<label for>` (accessible names)
- **change:** Replace "typed text is not measured, since no command can type yet" with: typed text in a host's log is still not measured — since 2026-10-07-act-by-id a typing command exists (the driver's `type` verb) and types into a held instance in process, but none reaches a sink-installing host's instance.
- **sidecar:** 2026-10-07-act-by-id: accessible-names row — 'no command can type yet' retired; a typing command exists in process, none reaches a sink-installing host, typed text in a host's log stays not measured.
- **rationale:** Report Counts '"No command can type" clauses': "a typing command now exists and types into a held instance in process; none reaches a sink-installing host's instance, so typed text in a host's log stays not measured. Stale as worded: `security-plan.md:117@c1260`". Outcome (obs): "no typing command reaches the `escher-session` binary's instance; typed text recorded as still not measured — met".
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:117

### S6 · D-security-input · escalate · dependent-of D-security-input
- **section:** §Input Validation → Markup attributes · `aria-label` · `<label for>` (accessible names)
- **change:** In the `Snapshot::to_text` clause, replace "neither the snapshot nor its text reaches a log, event, socket, file or command" with: neither the snapshot nor its text reaches a log, event, socket or file; since 2026-10-07-act-by-id the text is returned by the driver's `snapshot` verb in process, to its caller only, and no CLI or MCP command exists. In the `Snapshot::diff` clause that follows, add that a diff carrying those names is now returned by the driver's acting verbs in process, to its caller only.
- **sidecar:** 2026-10-07-act-by-id: accessible-names row — the '…or command' half of the snapshot-text claim retired; a driver command returns the text and the diff in process, to its caller only.
- **rationale:** Same retired claim as the `id` row's, restated here without the searched tokens (the report's search `no driver, CLI or MCP command` does not hit this wording). Report Counts: the snapshot text and the diff "are now returned by a driver command in process, to its caller only"; Coverage: an `Outcome` carries ids, accessible names and control values in the returned value only.
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:117

### S7 · D-security-input · escalate · dependent-of D-security-input
- **section:** §Input Validation → Markup attributes · password and file `input` value (the snapshot's `value`)
- **change:** Replace "which is returned to its caller only and, like the snapshot, reaches no log, event, socket, file or command" with: which is returned to its caller only and, like the snapshot, reaches no log, event, socket or file; since 2026-10-07-act-by-id the driver's `snapshot` verb returns the text and its acting verbs a diff, in process and to their caller only — a password typed through the driver's `type` verb occurs 0 times in what the call returns and the entry's value reads `MASKED_VALUE` (tests/blitz-tests/tests/stand_act_diff.rs) — as measured at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md.
- **sidecar:** 2026-10-07-act-by-id: password/file value row — the '…or command' claim retired; a driver command returns text and diff in process; a driver-typed password reads `MASKED_VALUE` and occurs 0 times in the returned outcome.
- **rationale:** Same retired claim, third wording (not hit by the report's token search). Report Outcome (security): "a password typed through the driver occurs 0 times in what the call returns and the entry's value reads `MASKED_VALUE` — met (`stand_act_diff`)"; Coverage: an `Outcome` carries "a password's or a file input's value as the snapshot's mask".
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:118

### S8 · D-security-input · escalate · dependent-of D-security-input
- **section:** §Logging & Monitoring → Log format and backends (escher's own sink)
- **change:** Replace "typed text is not measured (no command can type into the held instance yet)" with: typed text is not measured (since 2026-10-07-act-by-id a typing command exists and types into a held instance in process; none reaches a sink-installing host's instance).
- **sidecar:** 2026-10-07-act-by-id: Logging & Monitoring sink bullet — 'no command can type into the held instance yet' retired; typed text in a host's log stays not measured.
- **rationale:** Report Counts '"No command can type" clauses': stale as worded at `security-plan.md:379@c2587` ("no command can type into the held instance yet"); a typing command now types into a held instance in process and reaches no sink-installing host's instance.
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:379

### S9 · D-security-input · warning
- **section:** §Input Validation → Driver session label (escher-driver)
- **change:** Citation re-point only, no claim change: `packages/escher-driver/src/session.rs:11-46` → `packages/escher-driver/src/session.rs:11-52`.
- **sidecar:** 2026-10-07-act-by-id: session-label row citation re-pointed (session.rs:11-46 → :11-52); claim unchanged.
- **rationale:** Report line map: `security-plan.md:75@c293` `:11-46` → `:11-52` (`Session` gained the `time` field and `with_time`; `start` keeps its signature and behaviour).
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:75 ; packages/escher-driver/src/session.rs:44-52

### S10 · D-security-input · warning
- **section:** §Input Validation → CLI arguments (escher-session)
- **change:** Citation re-point only, no claim change: `examples/seven_guis/src/session_host.rs:47-53` → `examples/seven_guis/src/session_host.rs:51-57` (the `:13-32` citation beside it stays).
- **sidecar:** 2026-10-07-act-by-id: escher-session argv row citation re-pointed (session_host.rs:47-53 → :51-57); claim unchanged.
- **rationale:** Report line map: `security-plan.md:77@c526` `:47-53` → `:51-57`; Symbols/APIs: 'Argv check still first, no argument added, nothing printed; exit codes 0 · 1 · 2 unchanged'.
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:77 ; examples/seven_guis/src/session_host.rs:51-57

### S11 · D-security-input · warning
- **section:** §Threat Model Summary → Attack surface → local IPC — the driver session's socket (Entry point)
- **change:** Citation re-point only, no claim change: `examples/seven_guis/src/session_host.rs:36-44` → `examples/seven_guis/src/session_host.rs:37-49`.
- **sidecar:** 2026-10-07-act-by-id: local-IPC entry-point citation re-pointed (session_host.rs:36-44 → :37-49); claim unchanged.
- **rationale:** Report line map: `security-plan.md:24@c328` `:36-44` → `:37-49`; the report adds no socket, port, listener or IPC method, so the entry-point claim holds.
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:24 ; examples/seven_guis/src/session_host.rs:37-49

### S12 · D-security-input · warning
- **section:** §Logging & Monitoring → Log format and backends (Stdout output)
- **change:** Citation re-point only, no claim change: `examples/seven_guis/src/session_host.rs:21-51` → `examples/seven_guis/src/session_host.rs:21-55`.
- **sidecar:** 2026-10-07-act-by-id: stdout-output bullet citation re-pointed (session_host.rs:21-51 → :21-55); claim unchanged.
- **rationale:** Report line map: `security-plan.md:380@c281` `:21-51` → `:21-55`; the host still prints nothing to stdout (Outcome, layouts row: met).
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:380 ; examples/seven_guis/src/session_host.rs:21-55

### S13 · D-security-auth · warning
- **section:** §Secret Management → Environment values read (none secret-bearing)
- **change:** Citation re-point only, no claim change and the auth invariant holds: `tests/blitz-tests/tests/session_common/mod.rs:71` → `tests/blitz-tests/tests/session_common/mod.rs:195`.
- **sidecar:** 2026-10-07-act-by-id: env-values bullet citation re-pointed (session_common/mod.rs:71 → :195); no env var added, claim unchanged.
- **rationale:** Report line map: `security-plan.md:295@c450` `:71` → `:195`; the report states no new env var and the env census over the driver's changed files reads 0, so 'reads no env var beyond `RUST_LOG`' holds.
- **basis:** /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:295 ; tests/blitz-tests/tests/session_common/mod.rs:195

### security-plan — the comment lines stripped from the return

```text
# security-plan drift — chunk 2026-10-07-act-by-id
# Detector verdicts:
#   D-security-input — code side HOLDS (report Coverage: `Session::run` validation mechanism present, `validate` first; report: "No new socket, port, listener, env var, IPC method").
#     The drift is in security-plan's own wording: 8 claim sites now state a retired fact (1 primary + 7 dependents), plus 4 citation re-points.
#     Severity `escalate` is copied from the detector entry; no unvalidated boundary was found.
#   D-security-auth  — no drift. The report touches no identity / token / key / secret source and adds no env var. One citation re-point only (last proposal).
#   D-security-deps  — no drift. Report Dependencies: "none added, none bumped"; `Cargo.lock` byte-identical; one feature named on an existing dependency; `ci-leg.sh audit` reads `advisories ok`. No proposal.
# Read and left alone (still true per the report): security-plan.md:25 (wire is lifecycle only), :194 and :217 (one local listener), :203 (no driver command waits on a load), :182 (file path masked),
#   :333 ("nothing prints, logs or sends one yet" — report: holds), :116 `unkeyed_actionable` clause (report: unmoved), :379 `session_host.rs:20-22` (not in the report's moved map).
# New coordinates below come from the report's line map; I spot-checked execute.rs:60/63/108/126-134, session_host.rs:37-57, session.rs:44-52 and session_common/mod.rs:195 against the tree and they match.
```

## test-plan — the parsed list (16)

### T1 · D-tests-coverage · warning
- **section:** §1 Test Scope Summary → Coverage scope → apps (rdme, bump, examples, accesskit_xplat)
- **change:** State that seven_guis' library holds its first unit test since 2026-10-07-act-by-id — `stand::tests::the_timer_step_maps_milliseconds_onto_whole_ticks` (5 rows × 2 modes), run as `cargo test -p seven_guis --locked --lib stand::` — and that `host_binary` boots `counter` and `host_log` boots `crud`, so no standing check boots `escher-session timer`, the one host branch whose session carries a time step (`with_time(stand::timer_step(..))`); that branch is booted by hand only (escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/timer-host-boot.md).
- **sidecar:** 2026-10-07-act-by-id: §1 apps row — seven_guis lib unit tests 0 → 1 (timer_step); recorded gap: no standing check boots the `escher-session timer` host branch (by hand only).
- **rationale:** Report Coverage of new surfaces: "`escher-session timer` (the host's Timer branch) → tests ✗ standing (no listed check boots the timer host) / by hand✓"; Spec claims disproved #1 (host_binary.rs:30 boots counter, host_log.rs:77 crud); Counts: "seven_guis library unit tests 0 → 1". A new path with no standing test — the detector's drift — and the row does not record it.
- **basis:** .andromeda/test-plan.md:10

### T2 · D-tests-coverage · warning · dependent-of D-tests-coverage
- **section:** §3 → Session lifecycle
- **change:** session-binary: add that the `timer` task's session is given `stand::timer_step(handle)` through `Session::with_time` before `serve` while the other three tasks get no step, with no argument added, nothing printed and exit codes unchanged; that no standing check boots the `timer` branch (`host_binary` boots `counter`, `host_log` `crud`; by hand at escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/timer-host-boot.md); and re-point the citation `examples/seven_guis/src/session_host.rs:5-55` → `:5-59`.
- **sidecar:** 2026-10-07-act-by-id: session-binary — Timer session carries the stand's time step via `with_time`; timer branch has no standing check; session_host.rs cite :5-55 → :5-59.
- **rationale:** Report Symbols / APIs "`escher-session` host (session_host.rs:45-48)"; Coverage "tests ✗ standing"; line map row `session-lifecycle.md:9@c332 :5-55 → :5-59`. Second site of the timer-host coverage gap.
- **basis:** .andromeda/registries/contracts/test-plan/session-lifecycle.md:9

### T3 · D-tests-coverage · warning
- **section:** §3 Test Harness Contract → blitz-test-harness (`Harness`) → Input helpers
- **change:** Add `apple_keybinding(command)` to the helper list — it dispatches `UiEvent::AppleStandardKeybinding(command)` to the focused element and pumps, as every other input helper does (packages/blitz-test-harness/src/input.rs:229) — re-point the list's citation `input.rs:95-232` → `:95-239`, and state its coverage: exercised only on the macOS CI leg, through the driver's `press backspace` in `stand_act_keys`; no standing check calls it on Linux or Windows.
- **sidecar:** 2026-10-07-act-by-id: §3 Input helpers — `apple_keybinding` added (input.rs:229; list cite → :95-239); covered on the macOS CI leg only.
- **rationale:** Report Symbols / APIs "`blitz_test_harness::Harness::apple_keybinding`… new input helper"; Coverage "tests integ, on the macOS CI leg only (through `press backspace` in `stand_act_keys`; on Linux and Windows no standing check calls it)"; line map `test-plan.md:90@c207 :95-232 → :95-239`. The helper list at this site is now incomplete and the path's test exists on one platform leg only.
- **basis:** .andromeda/test-plan.md:90

### T4 · D-tests-coverage · warning
- **section:** §4 Unit Test Strategy → What unit tests cover → escher-driver
- **change:** Replace "the schema is held in process, so no test drives it over the socket or through a `Session`" with: the schema is executed in process by `Session::run(&Call)` (packages/escher-driver/src/execute.rs:60) — no test drives it over the socket (nothing but `hello` and `stop` crosses it), and the six `stand_act_*` files drive it through a `Session` (§5); add that execute.rs holds 1 unit test, the executor's key table (crate total 26); re-point `packages/escher-driver/src/session.rs:81-103` → `:101-123`; cite escher-0.1.0/chunks/2026-10-07-act-by-id/report.md for the executor.
- **sidecar:** 2026-10-07-act-by-id: §4 escher-driver — schema now executed in process by `Session::run` and driven through a `Session` by `stand_act_*`; +1 execute.rs unit test; session.rs cite :81-103 → :101-123.
- **rationale:** Report Counts "Held in process clauses": `test-plan.md:142@c2025` ("no test drives it … through a `Session`") is "moved by `Session::run(&Call)`"; "escher-driver unit tests 25 → 26 (… execute 1 …)"; Schema / config: the schema "is now executed in process where it was stated and validated only"; line map `test-plan.md:142@c2147 :81-103 → :101-123`.
- **basis:** .andromeda/test-plan.md:142

### T5 · D-tests-coverage · warning · dependent-of D-tests-coverage
- **section:** §1 Test Scope Summary → Coverage scope → escher-driver
- **change:** "25 inline unit tests in six files" → "26 inline unit tests in seven files", adding execute.rs 1 (the executor's key table) to the per-file list; keep "host.rs and client.rs hold no unit test" and add that `Session::run`'s verbs and refusals are covered by the six `stand_act_*` integration files; re-point `packages/escher-driver/src/session.rs:81-103` → `:101-123`; add "the count of 26 at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md" beside the count of 25.
- **sidecar:** 2026-10-07-act-by-id: §1 escher-driver row — 25 unit tests in six files → 26 in seven (+execute.rs 1); session.rs cite :81-103 → :101-123.
- **rationale:** Report Counts: "escher-driver unit tests 25 → 26 (`cargo test -p escher-driver --locked`: 26 passed; per file command 5 · error 3 · execute 1 · refusal 3 · schema 4 · session 2 · wire 8)"; line map `test-plan.md:21@c1867 :81-103 → :101-123`. Duplicate site of the crate's test count.
- **basis:** .andromeda/test-plan.md:21

### T6 · D-tests-coverage · warning · dependent-of D-tests-coverage
- **section:** §3 → Session lifecycle
- **change:** session-proof: add `stand_act_ids` 2 · `stand_act_diff` 6 · `stand_act_timer` 4 · `stand_act_refused` 2 · `stand_act_keys` 2 · `stand_act_range` 1 (17, `Session::run` in each, both layout modes) and seven_guis' library unit test 1 (`timer_step`); keep "lifecycle unit tests 13 (error.rs 3 · session.rs 2 · wire.rs 8)" and reword "the crate's other 12 pin the command and refusal schema" to "of the crate's other 13, 12 pin the command and refusal schema and 1 the executor's key table, none part of this contract"; add "the crate's 26 at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md" and the witness "the fork's CI run 37633611745 on `f8eb42c8` (16 of 16 jobs green; all six `stand_act_*` files read as run in the macOS and windows job logs, the iOS and android logs not opened)".
- **sidecar:** 2026-10-07-act-by-id: session-proof — +17 `stand_act_*` tests, +1 seven_guis unit; crate's other unit tests 12 → 13 (executor key table); CI run 37633611745 on f8eb42c8 as witness.
- **rationale:** Report Counts: "lifecycle unit tests 13 … and 'the crate's other 12 pin the command and refusal schema' — the 13th other is the executor's key table (`registries/contracts/test-plan/session-lifecycle.md:11@c431`)"; Files New (7) with 17 tests; Cross-project claims: CI#37633611745, verdict green 16/16, six files read on aarch64-apple-darwin and x86_64-pc-windows-msvc, ios/android logs not opened.
- **basis:** .andromeda/registries/contracts/test-plan/session-lifecycle.md:11

### T7 · D-tests-coverage · warning · dependent-of D-tests-coverage
- **section:** §3 → Session lifecycle
- **change:** session-start: beside `act(step)`, state `Session::run(&Call) -> Result<Outcome, Refusal>` (packages/escher-driver/src/execute.rs:60) — it validates the call first and returns its refusal with nothing run, resolves an id through `element_ids()` (`Cause::NotFound` otherwise), runs each acting verb as one settled step and returns `Outcome::Screen` / `Acted` / `Advanced` with the diff, a step that did not go quiet being a result (`settled` false, its `busy` class), not rolled back — and `Session::with_time(step)` (session.rs:60), the caller's time step, without which `advance` is refused as `Cause::TimeUnavailable`; both are in process, nothing they read or return crossing the socket; re-point `packages/escher-driver/src/session.rs:35-78` → `:40-98`.
- **sidecar:** 2026-10-07-act-by-id: session-start — gains `Session::run(&Call)` and `Session::with_time`; session.rs cite :35-78 → :40-98.
- **rationale:** Report Expected amendment 4 ("`session-start` gains `run` and `with_time`" — carried); Symbols / APIs for `Session::run`, `Outcome`, `Session::with_time`, the two refusals, "Nothing crosses the socket"; line map `session-lifecycle.md:5@c652 :35-78 → :40-98`. The row names `act` as the one way a check runs a step on a held session.
- **basis:** .andromeda/registries/contracts/test-plan/session-lifecycle.md:5

### T8 · D-tests-coverage · warning · dependent-of D-tests-coverage
- **section:** §5 Integration Test Strategy → Boundaries covered → Session ↔ held instance
- **change:** Add the `Session::run` boundary: a caller-built `Call` run in process on a held session — `snapshot`, and `click` · `type` · `press` · `advance` addressed by stable id — returns after settle with the diff of the snapshots before and after, equal to the check's own `before.diff(&after)` and across the two layout modes; a refused call (`not-found`, `time-unavailable`, `malformed`) leaves the instance unchanged and its `Display` holds none of the supplied bytes; a typed password occurs 0 times in what the call returns and reads `MASKED_VALUE`; with no sleep or clock read in the checks (`stand_act_ids`, `stand_act_diff`, `stand_act_timer`, `stand_act_refused`, `stand_act_keys`, `stand_act_range`) — as measured at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md.
- **sidecar:** 2026-10-07-act-by-id: §5 Session ↔ held instance — `Session::run(&Call)` boundary added, covered by the six `stand_act_*` files.
- **rationale:** Report Coverage: "`Session::run` … tests integ✓ (17 tests, both layout modes) + unit✓"; Outcome v010-09 / v010-06 / v010-03 / v010-10 and the security criteria (`stand_act_refused` 4 rows × 2 modes; `stand_act_diff` password). The row states `Session::act` as the only driven step on a held session.
- **basis:** .andromeda/test-plan.md:179

### T9 · D-tests-coverage · warning · dependent-of D-tests-coverage
- **section:** §1 Test Scope Summary → Coverage scope → tests/blitz-tests
- **change:** Append the act-by-id coverage: driver calls run through `Session::run` on a held stand session, in both layout modes — `stand_act_ids` 2 (77 snapshot nodes per mode: snapshot id = `author_id` = the id `element_ids()` resolves = the id a driver `click` accepts; the 8 of 15 controls with an effect at boot each equal to a twin's selector click and not empty, the Timer's Reset read after an `advance`), `stand_act_diff` 6 (each step's returned diff naming exactly the stated nodes, empty for a click on the heading, a typed password masked), `stand_act_timer` 4 (`advance` 300 → settled, `advanced_ms` 300, `Elapsed: 0.3s` in the returned diff; 50 → 0, 250 → 200, never more than asked; the harness clock unmoved), `stand_act_refused` 2 (a refused call changes nothing; an id of 1024 bytes is looked up, 1025 is `malformed`), `stand_act_keys` 2 (backspace deletes one typed character; Tab and Shift+Tab return exactly the controls whose focus moved), `stand_act_range` 1 (the Timer's slider driven through the driver alone reads value 15 after each of eleven calls) — as measured at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md.
- **sidecar:** 2026-10-07-act-by-id: §1 tests/blitz-tests row — six `stand_act_*` files (17 tests) added to the coverage scope.
- **rationale:** Report Files New (7): the six files with per-file test counts (2 · 6 · 4 · 2 · 2 · 1 = 17); Outcome criteria v010-03/06/09/10, (security), (a11y), (measurement); Counts "blitz-tests integration files 87 → 93". The row enumerates every stand file's coverage and stops at `stand_settle`. (Its `session_common/mod.rs:1-6` citation keeps its coordinate; the cited range holds an edited line — re-read at apply.)
- **basis:** .andromeda/test-plan.md:26

### T10 · D-tests-coverage · warning
- **section:** §2 Test Strategy → Test directory + naming conventions → Directory pattern
- **change:** "`mod common;` by eleven `stand_*.rs` checks" → "by 14 `stand_*.rs` checks" (+ `stand_act_ids`, `stand_act_diff`, `stand_act_keys`), and "`mod session_common;` by five checks — four `stand_session_*.rs` and `stand_settle.rs`" → "by 11 checks — four `stand_session_*.rs`, `stand_settle.rs` and the six `stand_act_*.rs`", keeping that `stand_settle.rs` declares it and not `mod common;`.
- **sidecar:** 2026-10-07-act-by-id: §2 Directory pattern — `mod common;` readers eleven → 14, `mod session_common;` readers five → 11.
- **rationale:** Report Counts: "`mod common;` readers eleven → 14 `stand_*.rs` checks (+ `stand_act_ids`, `stand_act_diff`, `stand_act_keys`) and `mod session_common;` readers five → 11 (+ the six `stand_act_*`) — stated at `test-plan.md:65@c441` and `@c539`". (`session_common/mod.rs:1-6` keeps its coordinate; range holds an edited line.)
- **basis:** .andromeda/test-plan.md:65

### T11 · D-tests-coverage · warning · dependent-of D-tests-coverage
- **section:** §3 Test Harness Contract → Crate-local test helpers → blitz-tests (stand checks)
- **change:** Extend the list of checks that declare `mod common;` — after `stand_snapshot_text` and the five `stand_session_*` checks add `stand_act_ids`, `stand_act_diff` and `stand_act_keys` (14 readers); `stand_act_timer`, `stand_act_refused` and `stand_act_range` declare `mod session_common;` only.
- **sidecar:** 2026-10-07-act-by-id: §3 stand checks helper row — `mod common;` reader list gains `stand_act_ids`, `stand_act_diff`, `stand_act_keys`.
- **rationale:** Same retired claim as the Directory pattern row, restated here as an enumeration of eleven readers (six named + the five `stand_session_*`); report Counts: readers eleven → 14.
- **basis:** .andromeda/test-plan.md:99

### T12 · D-tests-coverage · warning · dependent-of D-tests-coverage
- **section:** §3 Test Harness Contract → Crate-local test helpers → blitz-tests (session checks)
- **change:** State that `session_common` also holds the driver's call builders `click`, `type_into`, `press` and `advance`, an outcome reader (`Acted`, `act`) and `focused`; that `hold(task, incremental)` attaches the stand's time step to the Timer's session and returns the same pair (the session and the timer's tick handle); that the six `stand_act_*` checks declare `mod session_common;` beside the five already named (11 readers); and re-point `tests/blitz-tests/tests/session_common/mod.rs:1-145` → `:1-269`.
- **sidecar:** 2026-10-07-act-by-id: §3 session checks helper row — call builders, `Acted`/`act`, `focused` added; `hold` attaches the time step; readers five → 11; cite :1-145 → :1-269.
- **rationale:** Report Counts: "`session_common` now also holds the driver's call builders (`click`, `type_into`, `press`, `advance`), an outcome reader (`Acted`, `act`) and `focused`; `hold` attaches the stand's time step to the Timer's session and returns the same pair"; readers five → 11; line map `test-plan.md:100@c1096 :1-145 → :1-269`. Restates the five-reader claim by name.
- **basis:** .andromeda/test-plan.md:100

### T13 · D-tests-coverage · warning
- **section:** §3 Test Harness Contract → Agent-run contract → Proof
- **change:** Append: re-counted at 2026-10-07-act-by-id: `run stand`'s `run.end` reads passed 97 · failed 0 · ignored 3 (+17 over the six `stand_act_*` files, picked up by their `stand_` prefix with no script change; the selection lists 26 files) — as measured at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md.
- **sidecar:** 2026-10-07-act-by-id: §3 agent-run Proof — `run stand` 80 · 0 · 3 over 20 files → 97 · 0 · 3 over 26.
- **rationale:** Report Counts: "`stand_*.rs` files 20 → 26; `agent-run.sh run stand` 80 passed · 0 failed · 3 ignored over 20 files → 97 · 0 · 3 over 26 (its `run.start` and `run.end` events; +17) — stated at `test-plan.md:115@c2997`"; Harness / gate surface: selected by prefix with no script change.
- **basis:** .andromeda/test-plan.md:115

### T14 · D-tests-coverage · warning · dependent-of D-tests-coverage
- **section:** §9 CI Integration → Pipeline facts → Local baseline (`cargo test --workspace`)
- **change:** Append: re-counted at 2026-10-07-act-by-id: 149 result lines, 629 passed · 0 failed · 8 ignored (+17 in the six `stand_act_*` files, six new result lines; +1 `escher-driver` unit test, `execute`, in the crate's existing lib result line; +1 `seven_guis` library unit test, the crate's first), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run in its operator pass on the tree its pre-CI commit f8eb42c8 carries (escher-0.1.0/chunks/2026-10-07-act-by-id/report.md).
- **sidecar:** 2026-10-07-act-by-id: §9 baseline — workspace 143 result lines · 610 · 0 · 8 → 149 · 629 · 0 · 8.
- **rationale:** Report Counts: "Workspace 143 result lines · 610 passed · 0 failed · 8 ignored → 149 · 629 · 0 · 8 (`target/ci-logs/test.log` of the operator pass's fast leg; +6 lines for the six new targets, +19 passed: 17 integration + 1 escher-driver unit + 1 seven_guis unit) — stated at `test-plan.md:324@c6737`". Second site of the latest-count claim.
- **basis:** .andromeda/test-plan.md:324

### T15 · D-tests-coverage · warning
- **section:** §8 Mocking & Stubbing Discipline → Hand-written fakes and stubs → Time (seven_guis timer)
- **change:** Beside `TimerTicks`, state the driver's `advance`: `seven_guis::stand::timer_step(ticks)` is the time step a session carries through `Session::with_time` — an `advance` of `ms` delivers `ms / 100` whole ticks (`TIMER_TICK_MS` 100) through `TimerTicks::deliver`, reports that count × 100 and never more than asked as `advanced_ms` (50 → 0, 250 → 200), drops the remainder per call and leaves the harness's animation clock where it was; `advanced_ms` is the time delivered to the app, not what the Timer's counter gained (60000 is reported while the elapsed display stops at its 15 s duration); and add `Session::run` to the passes that apply a delivery (`Harness::settle`, `Session::act`, `Session::run`) (examples/seven_guis/src/stand.rs:86; examples/seven_guis/src/stand.rs:92; packages/escher-driver/src/execute.rs:80-109; `stand_act_timer`).
- **sidecar:** 2026-10-07-act-by-id: §8 Time (seven_guis timer) — `advance` through `stand::timer_step` / `Session::with_time` stated beside `TimerTicks`; `Session::run` added to the settling passes.
- **rationale:** Report Expected amendment 4 ("§8 Time — `advance` beside the tick stand-in", `Time (seven_guis timer)` 1 hit at `:280`); Symbols / APIs `TIMER_TICK_MS`, `timer_step`, `advanced_ms` (R5), "Each acting verb is one settled step"; Coverage: `stand::timer_step` unit✓ (5 rows × 2 modes) + integ✓. The row names `TimerTicks::deliver` as the only way test time moves and lists the settling passes without `Session::run`.
- **basis:** .andromeda/test-plan.md:280

### T16 · D-tests-coverage · warning · dependent-of D-tests-coverage
- **section:** §3 Test Harness Contract → blitz-test-harness (`Harness`) → Construction
- **change:** Add to the stand's drive surface `seven_guis::stand::TIMER_TICK_MS` (100) and `timer_step(TimerTicks) -> impl FnMut(&mut Harness<DioxusDocument>, u32) -> u32`, the time step a `Session` takes through `with_time` (§8 Time), and re-point `examples/seven_guis/src/stand.rs:58-92` → `:58-107`.
- **sidecar:** 2026-10-07-act-by-id: §3 Construction — stand drive surface gains `TIMER_TICK_MS` and `timer_step`; stand.rs cite :58-92 → :58-107.
- **rationale:** Report Symbols / APIs: "`seven_guis::stand::TIMER_TICK_MS: u32 = 100` (stand.rs:86) and `seven_guis::stand::timer_step(…)` (stand.rs:92) — new, public"; line map `test-plan.md:86@c965 :58-92 → :58-107`. The row states `boot` / `boot_timer` / `options` as the whole drive surface the stand checks use.
- **basis:** .andromeda/test-plan.md:86

### test-plan — the comment lines stripped from the return

```text
# Detector verdicts: D-tests-framework — no drift (report gates run cargo test / agent-run.sh / ci-leg.sh, the runners of test-plan §2-§4; no framework or dependency added). D-tests-obs-harness — no drift (report "Harness / gate surface": agent-run.sh, ci-leg.sh, workflows byte-identical, "No status, verdict or log shape changes"; obs-plan.md:84 and :86 read and still agree with test-plan §3; test-plan.md:102 and :181 "no command can type into the host's / binary's instance yet" hold as worded per the report). D-tests-coverage — two real gaps (proposals 1 and 3) plus the stale coverage record of the new paths (the rest; report Expected amendment 4).
# Outside the three detectors (citation-only re-points from the report's line map; no claim changes; listed so they are not lost at apply):
#   test-plan.md:49  (§2 Process-lifecycle checks)  tests/blitz-tests/tests/session_common/mod.rs:60-145 → :184-269
#   test-plan.md:87  (§3 Harness → Core)            packages/blitz-test-harness/src/lib.rs:26 → :27 (line 26 is now the `Key`, `Modifiers` re-export)
#   session-lifecycle.md:8 (session-state)          tests/blitz-tests/tests/session_common/mod.rs:66-78 → :190-202
# Read and left unchanged (still true as worded per the report): test-plan.md:102 and :181 "no command can type into the host's / binary's instance yet"; test-plan.md:106 (only `stand_session_lifecycle` and `stand_session_quiet` start a host process); test-plan.md:209 (the binary answers `hello` and `stop` only); session-lifecycle.md:7 session-wire (no id, name, value, snapshot text or diff crosses the socket).
```

## obs-plan — the parsed list (10)

### O1 · D-obs-stack · warning
- **section:** §3 Observability Harness Contract → Logging stack (subscriber installation), the headless-stand bullet (obs-plan.md:69)
- **change:** Where the bullet says the session library is silent with "its settled step `Session::act` and its command and refusal schema included", also name the executor: `Session::run` and `Session::with_time` and the private module `execute` install no subscriber, read no env var and no clock, and print or log nothing; `Outcome` joins `Command`, `Call` and `ArgValue` as a `Debug`-deriving type (it prints ids, accessible names and control values, a password's or file input's value as the snapshot's mask) that nothing in the crate prints, logs or fields; add `as measured at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md` to the bullet's provenance.
- **sidecar:** 2026-10-07-act-by-id: §3 Logging stack silence census extended to escher-driver's executor (`Session::run`, `Session::with_time`, module `execute`, `Outcome` under `Debug`); the crate still carries no `tracing` dependency.
- **rationale:** Report Symbols/APIs adds `Session::run` (execute.rs:60), `Outcome` (execute.rs:15, derives Debug) and `Session::with_time` (session.rs:60); Coverage row for `Session::run` says the crate carries no `tracing` dependency, span or event and the census gate reads 0; Outcome (obs) row: "the crate ends silent, the executor included"; Expected amendment 5 names this exact edit. The telemetry stack is unchanged, so this is a census gap, not an off-spec logger.
- **basis:** report.md:16-18, :94, :101, :157, :175; obs-plan.md:69

### O2 · D-obs-stack · warning · dependent-of D-obs-stack
- **section:** §3 Observability Harness Contract → Logging stack (subscriber installation), the headless-stand bullet (obs-plan.md:69)
- **change:** Replace "for the five `stand_session_*` checks, for `stand_settle`, and for the module four of those five and `stand_settle` read" with wording that adds the six `stand_act_*` checks (`stand_act_ids`, `_diff`, `_timer`, `_refused`, `_keys`, `_range`) to the checks that install no sink, and states that `session_common/mod.rs` is read by eleven checks: four of the five `stand_session_*`, `stand_settle` and the six `stand_act_*`.
- **sidecar:** 2026-10-07-act-by-id: §3 Logging stack names the six `stand_act_*` checks as sink-less and moves the `session_common` reader count from five to 11.
- **rationale:** Report Files lists six new `stand_act_*.rs` checks; Counts: "`mod session_common;` readers five → 11 (+ the six `stand_act_*`)". The bullet's enumeration of sink-less checks and its reader count are stale. The report does not itself measure print/subscriber absence in the new test files; my grep over them returned 0 hits.
- **basis:** report.md:10, :46; obs-plan.md:69; own grep `^mod session_common;` = 11 files

### O3 · D-obs-stack · warning · dependent-of D-obs-stack
- **section:** §3 Observability Harness Contract → Logging stack (subscriber installation), the headless-stand bullet (obs-plan.md:69)
- **change:** Replace "driven in-process by the `stand_*` checks, eleven of which read the shared module `tests/blitz-tests/tests/common/mod.rs`" with "…, 14 of which read the shared module …".
- **sidecar:** 2026-10-07-act-by-id: §3 Logging stack `mod common;` reader count eleven → 14 (+ `stand_act_ids`, `stand_act_diff`, `stand_act_keys`).
- **rationale:** Report Counts: "`mod common;` readers eleven → 14 `stand_*.rs` checks (+ `stand_act_ids`, `stand_act_diff`, `stand_act_keys`)". The report lists the stated site as test-plan.md:65 only; obs-plan.md:69 restates the same count and is a second occurrence its site search did not list.
- **basis:** report.md:46; obs-plan.md:69; own grep `^mod common;` over stand_*.rs = 14

### O4 · D-obs-stack · warning · dependent-of D-obs-stack
- **section:** §3 Observability Harness Contract → Logging stack (subscriber installation), the headless-stand bullet's citation list (obs-plan.md:69)
- **change:** Re-point the citation `examples/seven_guis/src/stand.rs:1-103` to `examples/seven_guis/src/stand.rs:1-118`.
- **sidecar:** 2026-10-07-act-by-id: §3 citation `stand.rs:1-103` → `:1-118` (the file gained `TIMER_TICK_MS` and `timer_step`).
- **rationale:** Report line map: "`obs-plan.md:69@c2289` `:1-103` → `:1-118`"; `stand.rs` gained `TIMER_TICK_MS` (:86) and `timer_step` (:92).
- **basis:** report.md:25, :62

### O5 · D-obs-instrumentation · warning
- **section:** §4 Span / Trace Coverage, the escher-driver bullet (obs-plan.md:105)
- **change:** Replace "held in process, reached by no socket and taken by no `Session` method" with "executed in process by `Session::run(&Call)` (which calls `validate` first), reached by no socket"; add `Session::run`, `Session::with_time` and `Outcome` to the bullet's list of the session library's span-less surface; state that the command executor now exists and carries no span, the one span per driver command still owed by the route entry "Driver command spans"; add the act-by-id report to the provenance.
- **sidecar:** 2026-10-07-act-by-id: §4 escher-driver bullet — the command schema is now executed in process by `Session::run`; the executor exists and carries no span; the span stays owed by "Driver command spans".
- **rationale:** Report Counts, "Held in process" clauses: "`obs-plan.md:105@c320` (\"taken by no `Session` method\") … moved by `Session::run(&Call)`"; Schema/config: the schema "is now executed in process where it was stated and validated only"; Coverage: instrumentation n/a for `Session::run` because the route entry "Driver command spans" owns the driver's spans. The new hot path is uninstrumented by recorded decision, so §4 must say it exists rather than that nothing takes a command.
- **basis:** report.md:16, :27, :34, :53, :101; obs-plan.md:105

### O6 · D-obs-instrumentation · warning · dependent-of D-obs-instrumentation
- **section:** §4 Span / Trace Coverage, the escher-driver bullet (obs-plan.md:105)
- **change:** Replace "or in the settle wait `act` runs, `Harness::settle`" with "or in the settle wait `act` and each acting verb of `run` run, `Harness::settle`" (the `click`, `type`, `press` and `advance` steps each settle once; `snapshot` dispatches nothing).
- **sidecar:** 2026-10-07-act-by-id: §4 — the span-less settle wait is now run by `Session::run`'s acting verbs as well as `Session::act`.
- **rationale:** Report Symbols/APIs: "Each acting verb is one settled step: the snapshot before, the step, `Harness::settle`, the snapshot after" (execute.rs:100-109). The bullet names `act` as the settle wait's only caller, the same no-executor claim restated.
- **basis:** report.md:16; obs-plan.md:105

### O7 · D-obs-instrumentation · warning · dependent-of D-obs-instrumentation
- **section:** §4 Span / Trace Coverage, the escher-driver bullet (obs-plan.md:105)
- **change:** In the cause-name sentence, state that two of the eight names are now returned by code — `not-found` (an id no `element_ids()` pair carries, or whose node does not resolve) and `time-unavailable` (`advance` on a session with no time step) — while `stale`, `disabled`, `covered` and `off-screen` are still returned by nothing; and extend the `Debug` hazard to `Outcome`: an `Outcome` fielded with `Debug` would print ids, accessible names and control values (a password's or file input's value as the snapshot's mask).
- **sidecar:** 2026-10-07-act-by-id: §4 — `not-found` and `time-unavailable` are the first screen/session-level causes any code returns; `Outcome` added to the types a span must not field with `Debug`.
- **rationale:** Report Symbols/APIs, "Refusals the executor returns" (execute.rs:125-134, :80-82) and Deviations: "`Outcome` derives `Debug` … an id, a name and a value print under `Debug`, and no span or log fields them". The bullet's guidance for the owed span's refusal-cause field and its Debug hazard list both predate the executor.
- **basis:** report.md:19, :101, :111; obs-plan.md:105

### O8 · D-obs-pii · warning
- **section:** §8 PII Scrubbing & Compliance → Values logged as-is, the "Past escher's scrub" bullet (obs-plan.md:290)
- **change:** Replace "Typed text is NOT measured — no command can type into the held instance yet" with "Typed text is NOT measured — a typing command now exists (`type` through `Session::run`) and types into an instance held in process only; no command reaches the instance of a sink-installing host (`escher-session`), so typed text in a host's log has not been read".
- **sidecar:** 2026-10-07-act-by-id: §8 — typed text in a host's log stays NOT measured; the reason is restated now that an in-process typing command exists.
- **rationale:** Report Counts, "No command can type" clauses: "a typing command now exists and types into a held instance in process; none reaches a sink-installing host's instance, so typed text in a host's log stays not measured. Stale as worded: … `obs-plan.md:290@c1497`"; Outcome (obs): "no typing command reaches the `escher-session` binary's instance; typed text recorded as still not measured". The PII invariant itself holds (nothing is logged or printed; a typed password reads `MASKED_VALUE` in the returned value), which is why this is a warning and not an escalation.
- **basis:** report.md:50, :94, :101, :150, :158; obs-plan.md:290

### O9 · D-obs-instrumentation · warning
- **section:** §6 Log Coverage → Logged events → examples, the `escher-session` stderr-lines citation (obs-plan.md:172)
- **change:** Re-point the citation `examples/seven_guis/src/session_host.rs:50` to `examples/seven_guis/src/session_host.rs:54` (the `eprintln!` of the `SessionError` message); `:21` and `:30` are unmoved.
- **sidecar:** 2026-10-07-act-by-id: §6 citation `session_host.rs:50` → `:54` (the Timer session's `with_time` wiring at :45-48 shifted the line).
- **rationale:** Report line map: "`obs-plan.md:172@c618` `:50` → `:54`". Citation re-point only; the claim (three fixed-string `eprintln!`s, nothing on stdout) is unchanged per report: "nothing printed; exit codes 0 · 1 · 2 unchanged". No detector invariant is violated; carried so the coordinate is not lost.
- **basis:** report.md:26, :63; examples/seven_guis/src/session_host.rs:54 (own read: `eprintln!("{error}");`)

### O10 · D-obs-stack · warning
- **section:** §9 CI Integration → Telemetry artifact handling, the Session state directory row's Source cell (obs-plan.md:325)
- **change:** Re-point the citation `tests/blitz-tests/tests/session_common/mod.rs:70-78` to `tests/blitz-tests/tests/session_common/mod.rs:194-202` (`state_dir` and `clear`).
- **sidecar:** 2026-10-07-act-by-id: §9 citation `session_common/mod.rs:70-78` → `:194-202` (the module gained the driver's call builders above it).
- **rationale:** Report line map: "`obs-plan.md:325@c525` `:70-78` → `:194-202`"; `session_common` gained the call builders, the outcome reader and `focused`. Citation re-point only; the row's claim is unchanged. No detector invariant is violated; carried so the coordinate is not lost.
- **basis:** report.md:47, :64; tests/blitz-tests/tests/session_common/mod.rs:194-202 (own read: `state_dir`, `clear`)

### obs-plan — the comment lines stripped from the return

```text
# Notes for the orchestrator (YAML comments; the proposals follow)
# - No detector invariant is violated in the code: the chunk adds no telemetry dependency (report Dependencies: "none added, none bumped"), escher-driver stays silent with the executor included (report Outcome, (obs) rows; census over execute.rs/session.rs/lib.rs reads 0), and nothing logs typed text. Every proposal below is obs-plan wording the report's Changes retire or move, filed under the detector whose section holds it. All are severity warning; nothing escalates.
# - D-obs-pii: the invariant HOLDS (no raw user input logged). Its one proposal is a stale qualifier in §8, so I filed it as warning, not the detector's escalate.
# - Keyed contract "Bootstrap phases" (otel-sdk-install, pii-scrubbing-wire): read whole; no Change touches it (no OTel crate, escher-telemetry byte-identical). No proposal.
# - Coordinate-kept citations the report asks to be re-read: `packages/escher-driver/Cargo.toml:13-15` (obs-plan.md:69, :105) still shows two dependencies and no `tracing` (line 15 now names the `accessibility` feature) and `tests/blitz-tests/tests/session_common/mod.rs:1-6` (obs-plan.md:69) is still the module doc with no subscriber; both claims hold, no change.
# - The last two proposals are citation re-points from the report's measured line map, in §6 and §9. They fall under no detector invariant; I carried them under the nearest detector so they are not lost. Drop them if the line map is applied mechanically elsewhere.
# - My own read-only checks beyond the report (labelled in each basis): `grep -E 'println!|eprintln!|print!|dbg!|escher_telemetry|tracing|env::var|log::'` over the six `stand_act_*.rs`, `session_common/mod.rs`, `execute.rs`, `session.rs` returned 0 hits; `grep -l '^mod common;'` over `stand_*.rs` = 14; `grep -l '^mod session_common;'` = 11.
```
