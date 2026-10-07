# Fan-out results — 2026-10-07-driver-command-spans

Wrap run `.andromeda/runs/2026-10-07T22-32-46-wrap/`, resumed from `report.md` in a second window (the
first wrote the report and stopped). Seven detectors, one per spec source, one parallel batch; the
prompts' detector counts sum to the 15 `doc:` names of the drift-base (2 · 3 · 1 · 1 · 3 · 3 · 2).
Entity probe on every proposal-carrying return: 0 before, 0 after decode. The lists below are the
returns as extracted from the detectors' hand-backs by script, unedited but for one rewrite: the
security-plan return spelled 7 paths with the host's absolute prefix, written here repository-relative.

## Verdict lines

- architecture — 16 proposals (D-arch-decisions 5 · D-arch-resources 11; one graded `escalate`).
- security-plan — 7 proposals (D-security-input 6, one graded `escalate` · D-security-auth 1 · D-security-deps 0). Stripped: a leading comment block stating all three invariants hold on the code side and listing what was re-read and left.
- design-system — 0 proposals. Stripped: one comment line (no new UI; every Coverage row reads `tokens n/a`). Raw twin `.raw-fanout-design-system.md`.
- layout-templates — 0 proposals. Stripped: six comment lines (no surface added; line 74 re-read and still true). Raw twin `.raw-fanout-layout-templates.md`.
- test-plan — 16 proposals (D-tests-obs-harness 3 · D-tests-coverage 12 · D-tests-framework 1), two of them on the keyed contract `session-lifecycle`.
- obs-plan — 16 proposals (D-obs-instrumentation 3 · D-obs-stack 11 · D-obs-pii 2, one graded `escalate`). Stripped: a leading comment block stating the three invariants hold and that source line numbers in `basis` were read from the working tree.
- a11y-plan — 1 proposal (D-a11y-obs-schema). Stripped: trailing comments naming two sites of the plan's expected amendment 17 no detector covers.

56 proposals.

## Dispositions (Validate, checks 1-6)

Numbering is each list's own order. `EA n` is entry n of the plan's `Expected amendments (wrap)` list
as the report numbers them.

**The re-derivation tell.** 19 proposals cite a source location the report does not carry (two
detectors say so in their own preamble: line numbers "read from the working tree"). Each is rejected
as proposed, and its fact — which the report does carry — is raised by the orchestrator under check 5
against the named entry and applied with every citation re-measured by the orchestrator (a script
that locates each formerly cited block in the current file; listing in `cascade-dispositions.md`).
Marked `tell → raised` below.

### architecture
1. [Driver session] — apply · check 1 routine (accurate this-chunk addition) · EA 6.
2. Existing Scopes, escher-driver — apply · dependent of 1 · EA 10.
3. Occupied Resources → Names — apply · EA 9.
4. Conventions → Feature gating — apply · EA 11.
5. Inherited Defaults → Optional capabilities — apply · EA 11.
6. Standard Contracts → Driver session — apply · EA 7.
7. Process-wide state, the log target `escher_driver` — apply · EA 9.
8. Standard Contracts → Dioxus DOM bridge, the founder-ratified diff clause — **escalated** (check 1: no rule matches, the sentence is ratified by the founder, the plan's list does not name the change). Resolved at this wrap's P2 halt — word: "Qualify, PROVISIONAL (Recommended)" — the operator, 2026-10-07. Applied: the ratified sentence unreworded, the measured fact appended, marked PROVISIONAL for the founder's batch.
9. Standard Contracts → Telemetry bootstrap, the closed span's record class — **escalated** (the playbook's Boundary widening, never routine; EA 8). Resolved on the operator's word at this wrap's invocation — word: "the PROVISIONAL mark on the sink closed-span line STAYS as it is, not discharged here; the batch it belongs to waits" — the operator, 2026-10-07. Applied marked PROVISIONAL.
10. Cross-cutting Patterns → Logging and timing — as 9 (its dependent) · EA 8.
11. Existing Scopes, escher-telemetry — apply · EA 10.
12. Process-wide state, the fourth re-executing binary — apply · EA 9.
13. Existing Scopes, blitz-tests (ten files, 29 tests) — apply · EA 10.
14. Existing Scopes, blitz-tests (`session_common` readers) — apply · dependent of 13.
15. Existing Scopes, blitz-tests (`mod common;` readers 14 → 15) — apply · the report's Counts carry the figure; the site is one its fixed-string sweep did not list.
16. Conventions → Tests — apply · dependent of 13.

### security-plan
1. Logging & Monitoring, the second record class — tell → raised · EA 14 · **escalated** as architecture 9 and resolved by the same word. Applied marked PROVISIONAL, the named limit of `tracing-subscriber 0.3.23` with it.
2. Logging & Monitoring, the typed-text clause — apply · EA 14.
3. Input Validation, the accessible-names row — apply · dependent of 2.
4. Input Validation, the Driver command schema row — tell → raised · EA 12. Applied.
5. Error Handling, the `Refusal` bullet — tell → raised · EA 13. Applied.
6. Input Validation, the `id` row's three citations — tell → raised (the report's Citation moves). Applied from the orchestrator's measurement.
7. Secret Management, the `RUST_LOG` citation — apply.

### test-plan
1. §3 Stand log format, the second record class — tell → raised · EA 15 · the boundary class, marked by pointer to obs-plan §6, which owns the mark. Applied.
2. §3 Stand log format, typed text — apply · EA 15.
3. §5 Session host ↔ what it writes — tell → raised · EA 15. Applied.
4. §1 escher-driver — apply · EA 15.
5. §1 escher-telemetry — tell → raised · EA 15. Applied.
6. §1 tests/blitz-tests, the tenth file — tell → raised · EA 15. Applied.
7. §2 Directory pattern — tell → raised (the report's Counts). Applied.
8. §3 stand checks helper — apply.
9. §3 session checks helper — apply.
10. §3 Agent-run contract → Proof — apply · EA 15.
11. §4 What unit tests cover, escher-driver — tell → raised · EA 15. Applied.
12. §5 Session ↔ held instance — apply · EA 15.
13. §9 Local baseline — apply · EA 15.
14. §3 → Session lifecycle (`session-proof`) — apply, in the key file.
15. §3 → Session lifecycle (`session-start`) — tell → raised (the report's Citation moves). Applied in the key file.
16. §9, engine features by runner — apply · check 6: this is the disposition of the report's disproved claim (the plan's "the capture grows by exactly one line") on the master side. Its cause was read by this wrap from cargo's feature graph (`evidence/feature-unification.md` in the chunk folder).

### obs-plan
1. §4 Span / Trace Coverage — tell → raised · EA 1. Applied.
2. §3 Logging stack, the session-library clauses — apply · EA 2.
3. §6 Logged events, the driver's group — tell → raised (the report: what reaches a log from a command). Applied.
4. §6 Log format, the closed span's line — tell → raised · EA 3 · **escalated** as architecture 9, resolved by the same word. Applied marked PROVISIONAL: this section owns the mark the other plans point at.
5. §3 Logging stack, `sink_layer` — tell → raised (the report: the one statement of the layer). Applied.
6. §3 Logging stack, the census exception — tell → raised · EA 2. Applied.
7. §3 Agent-run harness log — apply · dependent of 6.
8. §3 Logging stack, the three counts — tell → raised (the report's Counts). Applied. "fifteen of the sixteen" is the body's own sum of counts it states (5 + 1 + 10).
9. §2 Telemetry mechanism — apply · EA 5.
10. §2 Feature wiring — apply · check 2: same fact and same direction as test-plan 16, no contradiction; test-plan §9 carries the full statement and this bullet points at it.
11. §8 Scrubbing — tell → raised · EA 4 · **escalated** as architecture 9, resolved by the same word. Applied marked PROVISIONAL by pointer to §6.
12. §8 Values logged as-is, typed text — apply · EA 4.
13. §1 Instrumentation scope — apply.
14. §3 Service identity, citation — tell → raised (the report's Citation moves). Applied from the orchestrator's measurement.
15. §6 Log format, the `EnvFilter` citation — apply.
16. §6 Logged events, the install-event citation — apply.

### a11y-plan
1. §3 closing note, the two record classes — apply · EA 17. The note says where the mark lives (obs-plan §6) and carries none of its own.

### Raised by the orchestrator beyond the proposals
- architecture §Occupied Resources → Environment variables: the `EnvFilter` citation moved (`lib.rs:119` → `:144`). No detector proposed it; the report's Citation moves carry it. Routine.
- a11y-plan §1 → Dioxus crates and §7 (EA 17, the two sites the a11y detector named and could not propose): "the lockfile is byte-identical" restated as that chunk's measurement, with this chunk's one line; the dependency citation `13-15` → `13-16`; the `execute.rs` citation `205-223` → `289-307`. Routine.
- obs-plan §2 Telemetry mechanism and §8 Scrubbing: "the engine `tracing` features stay off" narrowed to the build it was measured on (a package-alone build), from this wrap's own reading of the feature graph. A scope narrowed to its measurement; routine.
- security-plan §Input Validation, the `id` row: the founder-ratified diff clause stands there a second time, found by the cascade sweep (`diff-clause`). Qualified exactly as architecture 8, on the same word — its duplicate.
- layout-templates (EA 16): line 74 re-read by the orchestrator — "its stderr log lines carry `service.name=seven_guis`" — still true of a sink that prints closed spans; no amendment.

### Totals
- 56 proposals: 37 applied as proposed · 19 rejected as proposed and raised by the orchestrator, all applied · 0 dropped.
- 5 raised beyond the proposals: 4 applied, 1 re-read with no change.
- 2 escalations, both resolved with the operator: the sink's closed-span record class (kept PROVISIONAL on the invocation's word) and the diff clause (qualified, PROVISIONAL, at the P2 halt).
- No playbook rule and no detector proposed: the first is the never-routine class; the second is one occurrence.

## Parsed lists (as returned)

### architecture

~~~yaml
proposals:
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Driver session]"
    change: >-
      Retire "it still depends on blitz-test-harness and dioxus-native-dom only … and carries no `tracing` dependency, span or event": the crate now has three dependencies — blitz-test-harness, dioxus-native-dom and `tracing` (`{ workspace = true }`, ungated, no cargo feature; packages/escher-driver/Cargo.toml:13-16) — and opens one `tracing` span per `Session::run` call (target `escher_driver`, name `command`, INFO), while it still installs no subscriber, reads no env var and no clock, has no `[features]` table and no edge to escher-telemetry or an app; keep "the key types its `press` needs come through the harness's re-exports" but drop "not a third dependency" as the count (the third dependency is `tracing`); qualify "`Cargo.lock` is byte-identical" as that earlier measurement only — the lock has since gained one line, `"tracing",` in the escher-driver entry, and no package; cite as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md.
    sidecar: >-
      2026-10-07-driver-command-spans — [Driver session]: escher-driver takes `tracing` ungated as its third dependency and emits one span per command; "no `tracing` dependency, span or event" retired; lock +1 line, no package.
    rationale: >-
      Report Dependencies (report.md:44) lists `escher-driver → tracing` and "dependencies are now three"; Symbols / APIs (report.md:16) lands the span; the locked decision states the opposite. The fork answer for the ungated call site carries no PROVISIONAL mark (report.md:136). Report's own expected amendment 6 (report.md:93) names this site.
    basis: ".andromeda/architecture.md:101"
  - detector: D-arch-decisions
    severity: warning
    section: "§Existing Scopes → escher-driver"
    change: >-
      Replace "it depends on blitz-test-harness and dioxus-native-dom only" with "it depends on blitz-test-harness, dioxus-native-dom and `tracing` (ungated)", and add to the execute module's description that `Session::run` opens the one command span (still nine private modules, 33 re-exports); re-cite packages/escher-driver/src/lib.rs:43-65 → :52-74 and packages/escher-driver/Cargo.toml:13-15 → :13-16.
    sidecar: >-
      2026-10-07-driver-command-spans — Existing Scopes escher-driver row: dependency list gains `tracing`; execute carries the command span; citations moved.
    rationale: >-
      Same retired claim as [Driver session], restated in the scope table; report.md:42, :44, :62 (dependencies 2 → 3; sites architecture :101, :257), citation moves report.md:107-108.
    basis: ".andromeda/architecture.md:257"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Occupied Resources → Names"
    change: >-
      In the escher-driver entry replace "depending on blitz-test-harness and on dioxus-native-dom with its `accessibility` feature named" with "depending on blitz-test-harness, on dioxus-native-dom with its `accessibility` feature named, and on `tracing` (workspace, ungated)"; "no features of its own and no binary target" stands; re-cite packages/escher-driver/Cargo.toml:13-15 → :13-16.
    sidecar: >-
      2026-10-07-driver-command-spans — Names: escher-driver's dependency list gains `tracing`.
    rationale: >-
      The registry's dependency list for the crate omits the edge the report's Dependencies bullet adds (report.md:44); report's expected amendment 9 (report.md:96).
    basis: ".andromeda/architecture.md:154"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Conventions → Feature gating"
    change: >-
      Beside "the escher subscriber crate escher-telemetry has no `[features]` and emits its startup and panic events ungated" state the second ungated escher crate: escher-driver takes `tracing` with no feature and builds its command span by hand with `tracing::info_span!` (no `#[instrument]`, no `cfg(feature = "tracing")` gate; packages/escher-driver/src/execute.rs:64-79); re-cite packages/escher-telemetry/src/lib.rs:138 → :161.
    sidecar: >-
      2026-10-07-driver-command-spans — Feature gating: escher-driver named under the ungated form beside escher-telemetry.
    rationale: >-
      The convention names escher-telemetry as the one escher crate using `tracing` ungated; report.md:16-17 and :44 make escher-driver the second (a `tracing` cargo feature on escher-driver is listed as rejected, report.md:78). Expected amendment 11 (report.md:98).
    basis: ".andromeda/architecture.md:111"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Inherited Defaults → Optional capabilities"
    change: >-
      Replace "escher-telemetry is ungated" with "escher-telemetry and escher-driver (its one command span) are ungated", adding the citation packages/escher-driver/Cargo.toml:13-16.
    sidecar: >-
      2026-10-07-driver-command-spans — Optional capabilities: escher-driver joins escher-telemetry as ungated.
    rationale: >-
      Same claim as §Conventions → Feature gating restated in the defaults list; report.md:44, :98.
    basis: ".andromeda/architecture.md:226"
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: >-
      Register the command span: every call handed to `Session::run` opens one `tracing` span — target `escher_driver`, name `command`, level INFO — entered before `validate` and held until `run` returns, with eight fields each created unrecorded and recorded only where it applies: `verb` (the schema row's own name, for a known verb, run or refused; never the caller's text), `cause` (`Cause::name()` of any returned refusal), `settled`, `busy` (the schema's busy-class word), `passes` (`Settled.passes`), `added` · `removed` · `changed` (the three list lengths of the returned `SnapshotDiff`); `snapshot` records `verb` alone, a refused call `verb` (when known) and `cause` only; never fielded — `in_view`, `advanced_ms`, the screen text, the label, the record of ids, any argument or any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`; the crate installs no subscriber, so with none installed the span is disabled and writes nothing, and `run`'s signature and return are unchanged (its body is the private `Session::execute`); restate "nothing in the crate prints, logs or fields any of the four" as "the crate prints nothing and fields none of the four — it fields only those eight fixed-word or count values on the one span"; re-cite lib.rs:41-65 → :50-74, lib.rs:43-65 → :52-74 and the execute.rs ranges per the report's citation moves (file 346 → 472 lines; `run` :82 → :108); cite as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md.
    sidecar: >-
      2026-10-07-driver-command-spans — Driver session contract: the `escher_driver` / `command` span and its eight fields registered; the no-log clause restated around them; execute.rs and lib.rs citations moved.
    rationale: >-
      A new emitted record (span, target, name, level, field set) lands in report Symbols / APIs (report.md:16-25) and appears nowhere in the registry sections; the contract still reads as a crate that logs nothing. Expected amendment 7 (report.md:94).
    basis: ".andromeda/architecture.md:133"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process-wide state and threads"
    change: >-
      Add the log target `escher_driver` to the registered log targets (beside `js_console`, `escher_telemetry`, `escher_telemetry::panic`): emitted by escher-driver's command span at INFO, admitted by `ESCHER_TARGET_PREFIXES`, written only in a process that has the sink installed at `info` or below — today only the two children of `stand_act_spans`; `escher-session` and `seven_guis_native` install the sink and run no command; re-cite packages/escher-telemetry/src/lib.rs:90-91 → :96-97 and :119-138 → :144-161.
    sidecar: >-
      2026-10-07-driver-command-spans — Process-wide state: log target `escher_driver` registered.
    rationale: >-
      The section is the registry of log targets and the new target is absent from it; report.md:16, :37-38. Expected amendment 9 (report.md:96).
    basis: ".andromeda/architecture.md:150"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      Qualify the diff clause "a diff leaves the process through the returned value only — no log, event, socket or file carries it (ratified by the founder, 2026-10-07 …)" without rewording the ratified sentence: append that since 2026-10-07-driver-command-spans the three list lengths of a returned diff (`added` · `removed` · `changed`, counts only — no node, id, name or value) are recorded on escher-driver's command span and reach escher's sink at `info`; the snapshot-text clause stands unchanged (no text or length is fielded); re-cite packages/escher-driver/src/execute.rs:86-88 and :257-267 per the report's citation moves.
    sidecar: >-
      2026-10-07-driver-command-spans — Dioxus DOM bridge: the diff's three list lengths now reach a log through the driver span; the founder-ratified "no log carries it" clause qualified, not reworded.
    rationale: >-
      Found by reading for the claim, not listed among the report's seventeen expected amendments: report.md:20 records the diff's three lengths on a span the sink prints (report.md:31 shows `added=1 removed=0 changed=1` in a real capture), while this contract says no log carries a diff, on the founder's word. The diff's content is still unlogged (report.md:23, :168), so whether sizes touch the ratified sentence is the operator's / founder's call — escalated, alongside the PROVISIONAL sink widening already bound for the founder's batch (report.md:136).
    basis: ".andromeda/architecture.md:136"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Telemetry bootstrap (escher-telemetry)"
    change: >-
      Replace "Line shape, one per printed event" with two record classes — one line per printed event and one line per closed span whose target the allowlist admits (no line when a span is created, recorded to, entered or exited): the span line is `{time} {LEVEL} {target} service.name=… service.version=…` then `span={the span's name}`, the span's own fields in the order recorded, then the layer's close fields `message` · `time.busy` · `time.idle`, level and target the span's, `span` a reserved field name; every pair, `span` included, prints as `decide(target, name)` says — the rule that judges an event's field, so under an engine target `span`, `message` and both timings read `[redacted]` — an outside-target span writes no byte, a `CONTENT_FIELDS` field's value is never stored, and an event's line is unchanged (it gains no span name or span field); no public item is added or changed; mark the closed-span record class "a boundary widening on the operator's answer, PROVISIONAL — listed for the founder's batch at the Epoch 4 boundary"; re-cite lib.rs:34-140 → :40-163, format.rs:123-198 → :134-286 region (`format_event` :138-177, `SpanFields` :200, `ScrubVisitor` :252), format.rs:21-107 → :22-111.
    sidecar: >-
      2026-10-07-driver-command-spans — Telemetry bootstrap: second sink record class (closed span line, reserved field `span`) registered, PROVISIONAL pending the founder's word; citations moved.
    rationale: >-
      The sink's line format gains a record class and a reserved field name (report.md:27-36, :46, :61) that the contract — which states the line shape as "one per printed event" — does not register. The report marks the widening PROVISIONAL on the operator's instruction to record it so (report.md:95, :136); severity kept at warning because the recording form is already decided, the founder's ratification is routed separately.
    basis: ".andromeda/architecture.md:132"
  - detector: D-arch-resources
    severity: warning
    section: "§Cross-cutting Patterns → Logging and timing"
    change: >-
      Extend the description of the one fmt layer: it is stated once in the private `sink_layer(identity, writer)` (packages/escher-telemetry/src/lib.rs:102-117) as `fmt::layer().with_span_events(FmtSpan::CLOSE).fmt_fields(SpanFields).event_format(EscherFormat::new(identity)).with_ansi(false).with_writer(writer)`, so it prints a closed span's line beside an event's line and "scrubs the records that print" covers a span's fields by the same `decide` rule; "no filter layer is added, the drop sits in the formatter" stands; add that escher-driver emits its command span through `tracing` ungated and installs no subscriber; mark the closed-span printing PROVISIONAL as in the Telemetry bootstrap contract; re-cite lib.rs:96-140 → :121-163 and format.rs:123-155 → :134-177.
    sidecar: >-
      2026-10-07-driver-command-spans — Logging and timing: the layer's span-close setting and field formatter (`sink_layer`) stated; closed spans print, PROVISIONAL.
    rationale: >-
      Restates the sink's mechanism as an event-only layer; report.md:32-34 changes the layer's construction. Expected amendment 8 (report.md:95).
    basis: ".andromeda/architecture.md:193"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → escher-telemetry"
    change: >-
      Describe module format as "the event formatter with the target allowlist — its drop of every outside-target record and its field scrub — and the crate-private span field formatter `SpanFields`, through which a closed span's line is printed under the same rule"; re-cite packages/escher-telemetry/src/lib.rs:96-140 → :121-163 (the module declarations cited at :22-23 sit below a crate doc that grew from :1-18 to :1-22 — re-read for the new lines).
    sidecar: >-
      2026-10-07-driver-command-spans — Existing Scopes escher-telemetry row: format module also holds the span field formatter; citations moved.
    rationale: >-
      The row describes format as an event formatter only; report.md:32-33, :42. Expected amendment 10 (report.md:97).
    basis: ".andromeda/architecture.md:256"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process-wide state and threads"
    change: >-
      Replace "three blitz-tests integration binaries each re-execute their own test binary once under `cargo test`" with four, the first three as stated (once each) plus `stand_act_spans`, which runs `std::env::current_exe()` twice with `--ignored --exact {child} --nocapture` — once with `RUST_LOG=info` set on the child and once with `RUST_LOG` removed — each child installing the sink with `init_with_writer` over an in-memory capture (test-only, a fixed argv, no socket or port; tests/blitz-tests/tests/stand_act_spans.rs); cite as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md.
    sidecar: >-
      2026-10-07-driver-command-spans — Process-wide state: fourth self-re-executing blitz-tests binary (`stand_act_spans`, two children, `RUST_LOG` set / removed, sink over a capture).
    rationale: >-
      A new child-process spawn with an env var set on it lands (report.md:59) and the registry still counts three; `RUST_LOG` itself stays the one env read (report.md:40), so §Occupied Resources → Environment variables does not move.
    basis: ".andromeda/architecture.md:150"
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: >-
      In the driver-calls clause replace "the nine files … 27 tests" with "the ten files `stand_act_ids` … `stand_act_scroll` and `stand_act_spans`, 29 tests run plus 2 ignored children", add to the covered list "and one command span per call read from a sink capture — one `escher_driver` line per call with its verb, cause, settle reading and diff lengths, none at the default level, and no id, name or typed text in the capture", and cite tests/blitz-tests/tests/stand_act_spans.rs.
    sidecar: >-
      2026-10-07-driver-command-spans — Existing Scopes blitz-tests row: `stand_act_*` 9 → 10 files, 27 → 29 tests + 2 ignored; `stand_act_spans` registered.
    rationale: >-
      New test target (report.md:11) not in the scope row; counts moved per report.md:53 (site architecture :263). Expected amendment 10 (report.md:97).
    basis: ".andromeda/architecture.md:263"
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: >-
      In the `tests/session_common/mod.rs` clause replace "by the nine `stand_act_*`, fourteen readers" with "by the ten `stand_act_*`, fifteen readers".
    sidecar: >-
      2026-10-07-driver-command-spans — Existing Scopes blitz-tests row: `session_common` readers 14 → 15.
    rationale: >-
      report.md:54 (readers of `tests/session_common/mod.rs` 14 → 15, basis grep 15; site architecture :263).
    basis: ".andromeda/architecture.md:263"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: >-
      In the `tests/common/mod.rs` clause replace "read through `mod common;` by 14 stand checks" with "by 15 stand checks".
    sidecar: >-
      2026-10-07-driver-command-spans — Existing Scopes blitz-tests row: `mod common;` stand readers 14 → 15.
    rationale: >-
      report.md:55 moves the count of `stand_*` files reading `tests/common/mod.rs` 14 → 15 (basis `grep -c 'mod common;'`, 15 files); the report's fixed-string sweep (`14 of which`) found only obs-plan :69 — architecture states the same count in other words ("by 14 stand checks"), so this site is outside the report's listed sites.
    basis: ".andromeda/architecture.md:263"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Conventions → Tests"
    change: >-
      Replace "the nine driver-action checks `stand_act_*`" with "the ten driver-action checks `stand_act_*`" in the clause on the checks that hold a `Session` and read `session_common`.
    sidecar: >-
      2026-10-07-driver-command-spans — Conventions Tests: driver-action checks nine → ten.
    rationale: >-
      report.md:53 (`nine driver-action`: architecture 1, :115).
    basis: ".andromeda/architecture.md:115"
    dependent-of: D-arch-resources
~~~

### security-plan

~~~yaml
# security-plan drift pass — chunk 2026-10-07-driver-command-spans
# Strict reading of the three invariants against the report's Changes:
#   D-security-input: HOLDS on the code side. The chunk adds no external-input surface (report :38 "No surface
#     carries a call out of the process yet"; :40 no listener, port, env var; nothing of a span, call or outcome
#     crosses the socket; wire.rs/host.rs/client.rs unmoved). Coverage (:114-115): the span has no input of its own;
#     the sink's closed-span line is judged by the target allowlist and `decide` on every pair. No unvalidated boundary.
#     The proposals below are filed under it because the baseline rows that state how this surface is guarded
#     restate claims the chunk retires (report's own expected amendments 12, 13, 14) — doc staleness, not a code gap.
#   D-security-auth: HOLDS. No identity, session-auth, token, key or secret source touched; `RUST_LOG` stays the one
#     env read (:40). One citation-only proposal in §Secret Management.
#   D-security-deps: HOLDS, no proposal. The one new edge is `escher-driver` -> `tracing` (`workspace = true`,
#     ungated; lock gains one line and no package; `ci-leg.sh audit` green — report :44). §Dependency Security
#     states no ban list (§Security Anti-Patterns reads NO RECORDED INTENT), and no §Dependency Security claim moves.
# Re-read and left as true: `reads no clock` (:76 x2, :203 x2 — the line's timings are the layer's); :116 "written to
#   no log" for ids (0 of 36 id needles); :177 and :259 (sink redacts/drops "records" — wording already covers spans);
#   :203 settle row; :118 password-mask "typed text" hits (another subject).
# Left as baseline history, not proposed: :359 and :385 "observed absent — searched: …info_span|…|tracing::span…
#   over the 15 s05 files" — slice-scoped past searches; note for the orchestrator that `tracing::info_span!` now
#   exists at packages/escher-driver/src/execute.rs:74, outside that slice.
# Line numbers below were checked against the working tree (execute.rs, format.rs, lib.rs) and match the report.
proposals:
  - detector: D-security-input
    severity: escalate
    section: "§Logging & Monitoring → Log format and backends (the `escher's own sink` bullet)"
    change: >-
      State that the sink has two record classes, not one: beside one line per printed event it prints one line per
      CLOSED span whose target the allowlist admits (`fmt::layer().with_span_events(FmtSpan::CLOSE).fmt_fields(SpanFields)`,
      built once in `sink_layer`), and nothing when a span is created, recorded to, entered or exited; the line is
      `{time} {LEVEL} {target} service.name=… service.version=…` then `span={name}`, the span's own fields, then the
      layer's `message`, `time.busy`, `time.idle`, every pair — `span` included — judged by the same `decide(target, field)`
      as an event's field (engine target: only the seven safe fields print, the rest `[redacted]`; escher target: all but
      the eleven content-named fields); a span of an outside target writes no byte at any point; the value of a
      content-named span field is never stored (other span field values, of engine and outside targets too, are held in
      process memory until the span closes and never written); an event's line is unchanged, inside a span included.
      Mark it a boundary widening, PROVISIONAL — the operator's answer (2026-10-07), not the founder's word, listed for
      his batch at the Epoch 4 boundary. Record the named residual, read from `tracing-subscriber 0.3.23` and not
      exercised — if a field formatter errors at span creation the layer itself `eprintln!`s the span's attributes with
      `Debug` to stderr, unscrubbed; `SpanFields` writes into a `String` and returns no error. Refresh the bullet's
      citations — `format.rs:21-107` → `:22-111`, `format.rs:134-138` → `:144-148`, adding `:162-173` (the closed-span
      path), `:200-240` (`SpanFields` and its store) and `packages/escher-telemetry/src/lib.rs:102-117` (`sink_layer`).
    sidecar: >-
      2026-10-07-driver-command-spans — §Logging & Monitoring: escher's sink gains a second record class, the closed
      span's line, under the same target allowlist and scrub; PROVISIONAL boundary widening (operator's answer, founder's
      word owed); tracing-subscriber's stderr fallback on a field-formatter error recorded as an unexercised residual.
    rationale: >-
      Report Symbols/APIs "The sink prints a closed span (new record class)" (:27-36) and Schema/config (:46); the bullet
      at security-plan :379 describes the formatter for "an event" only. Escalated not for an unvalidated boundary — the
      scrub is asserted and mutation-controlled (:115, :169) — but because the report's Decisions (:136) and expected
      amendments 3, 8, 14 carry it as a PROVISIONAL widening the founder has not ratified, and this document records
      each widening with its ratifier.
    basis: packages/escher-telemetry/src/format.rs:138-177

  - detector: D-security-input
    severity: warning
    section: "§Logging & Monitoring → Log format and backends (the `escher's own sink` bullet — its closing typed-text clause)"
    change: >-
      Replace "typed text is not measured (since 2026-10-07-act-by-id a typing command exists and types into an instance
      held in process, where no sink is installed; none reaches a sink-installing host's instance)" with — typed text
      driven in process with the sink installed is measured absent: `stand_act_spans` re-executes itself, each child
      installing the sink with `init_with_writer` over an in-memory capture at `RUST_LOG=info`, and the capture holds 0
      occurrences of the supplied texts (the typed text, the id naming no element, the unknown verb's text), 0 of 36 id
      needles and 0 of 8 name needles, in both layout modes; typed text in a sink-installing HOST's log is still not
      measured — `escher-session` installs the sink and runs no command (its wire is `hello` and `stop`),
      `seven_guis_native` runs none, and no surface carries a call into either — as measured at
      escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md. Keep the by-level readings as they are (re-measured
      unchanged: 0 · 1 · 1 · 1 lines, 0 of 15 ids, 0 of 6 names, stdout 0 B).
    sidecar: >-
      2026-10-07-driver-command-spans — §Logging & Monitoring: "no sink is installed" beside the driver's `type` retired;
      typed text read absent from an in-process sink capture (stand_act_spans); a host's log still unmeasured.
    rationale: >-
      Report Counts (:59, the fourth self-re-executing binary whose children install the sink), Coverage (:114) and
      Outcome (:168) against the clause at security-plan :379; report expected amendment 14 names this site. "Where no
      sink is installed" is false as of this chunk for the new check's children; "none reaches a sink-installing host's
      instance" stands (:38).
    basis: .andromeda/security-plan.md:379

  - detector: D-security-input
    severity: warning
    section: "§Input Validation → Markup attributes | `aria-label` · `<label for>` (accessible names)"
    change: >-
      Restate the same clause in this row — "typed text in a host's log is not measured: … the driver's `type` … types
      into an instance held in process, where no sink is installed, and none reaches a sink-installing host's instance"
      — as: a host's log is still not measured (no surface carries a call into a sink-installing host), while typed text
      driven in process with the sink installed over a capture reads 0 occurrences, with 0 of 8 name needles, in both
      layout modes (`stand_act_spans`, as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md).
    sidecar: >-
      2026-10-07-driver-command-spans — §Input Validation (accessible-names row): duplicate of the "no sink is installed"
      typed-text clause restated for the in-process sink reading.
    rationale: >-
      Second occurrence of the claim retired at :379 (fixed-string `no sink is installed` counts 2 in security-plan: :117
      and :379); report expected amendment 14 lists `typed text` at :117. Left alone, the row would still assert no sink
      is installed where the driver types.
    basis: .andromeda/security-plan.md:117
    dependent-of: D-security-input

  - detector: D-security-input
    severity: warning
    section: "§Input Validation → Driver command schema (escher-driver)"
    change: >-
      Restate the row's close "nothing in the crate prints, logs or fields any of them" around what the crate now fields:
      since 2026-10-07-driver-command-spans every call handed to `Session::run` opens one `tracing` span — target
      `escher_driver`, name `command`, level INFO, built by hand with `tracing::info_span!` (no `#[instrument]`), entered
      before `validate` so a refusal is inside it — with eight fields, each unrecorded until it applies: `verb` (the
      schema table's own word, never the caller's verb text), `cause` (`Cause::name()` of a returned refusal), `settled`,
      `busy` (the schema's class word), `passes`, `added` · `removed` · `changed` (the three list lengths of the returned
      diff) — fixed words, bools and counts only; never fielded: any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal`
      or `Fault`, any argument, id, typed text, accessible name, control value, the screen text or its length, the
      session's label, the record of ids, `in_view`, `advanced_ms`; the crate still prints nothing, installs no
      subscriber and reads no env var or clock, so with no subscriber the span is disabled and writes nothing, and under
      escher's sink it is one line per call at `info` or below and none at the default `warn`; "still no socket, CLI or
      MCP tool reaches it … not yet an external-input surface" and both "reads no clock" stand. Refresh the row's
      citations — `execute.rs:59-142` → `:81-179`, `execute.rs:186-253` → `:270-337`, adding `:64-79` (the span and its
      field list) and `:183-217` (`record_result`) — and add this chunk's report to the row's "as measured at" tail.
    sidecar: >-
      2026-10-07-driver-command-spans — §Input Validation (Driver command schema row): "nothing in the crate prints,
      logs or fields" restated — one `escher_driver`/`command` span per `Session::run` call with eight fixed-word or
      count fields, no caller-supplied or screen content; execute.rs citations refreshed.
    rationale: >-
      Report Symbols/APIs "The span of a driver command" (:16-24) and `Session::run` (:25); expected amendment 12 names
      this row (`prints, logs or fields` security-plan 1, :76). The sentence is still literally true of the four Debug
      types it names (acceptance :170, census 0) but a reader takes it as "the crate logs nothing", which the chunk
      retires. The validated surface itself is unchanged — verb table, argument kinds and refusal texts did not move (:46).
    basis: packages/escher-driver/src/execute.rs:108-117

  - detector: D-security-input
    severity: warning
    section: "§Error Handling → Error format (typed errors) (the escher-driver `Refusal` bullet)"
    change: >-
      Replace "held in process only — nothing prints, logs or sends one yet" with — a `Refusal` is still returned in
      process and sent nowhere, and since 2026-10-07-driver-command-spans exactly one thing of it reaches a log: its
      cause's fixed name, recorded as the `cause` field of the call's `escher_driver` span at one site (`record_result`)
      for every `Err`, whichever of `validate` and the executor returned it; no `Fault`, meaning, remedy or `Display`
      text is fielded (a `malformed` call's line reads `cause="malformed"` and nothing of the rule it broke), and since
      no cause name can hold anything a call supplied the logged value cannot either — citing
      `packages/escher-driver/src/execute.rs:183-188` and this chunk's report beside the standing ones.
    sidecar: >-
      2026-10-07-driver-command-spans — §Error Handling (Refusal bullet): "nothing prints, logs or sends one yet"
      retired; a refusal's cause name is logged as the span field `cause`, nothing else of a refusal is.
    rationale: >-
      Report Symbols/APIs `cause` (:19) and expected amendment 13, which names this bullet at security-plan :333. Same
      retired claim as the Driver command schema row (the driver leaves nothing in a log), worded differently and with
      a different actor (the refusal), so it would survive a single-site apply at :76.
    basis: packages/escher-driver/src/execute.rs:186
    dependent-of: D-security-input

  - detector: D-security-input
    severity: warning
    section: "§Input Validation → Markup attributes | `id` (stable element id)"
    change: >-
      Citation refresh only, no claim moves (the row's "written to no log, event, socket or file" for ids was re-measured
      true: 0 of 36 id needles in the sink capture) — `packages/escher-driver/src/execute.rs:86-88` → `:123-125` (the
      `snapshot` arm returning `to_text`), `execute.rs:257-267` → `:342-359` (`settled_step`), `execute.rs:165-203` →
      `:249-287` (`locate` and `resolve`).
    sidecar: >-
      2026-10-07-driver-command-spans — §Input Validation (`id` row): three execute.rs line citations moved with the
      span code (file 346 → 472 lines); no statement changed.
    rationale: >-
      Report Citation moves (:106) counts execute.rs citations in security-plan as :76 x2 and :116 x3 and gives the
      shifts (`run` body :83-141 → :120-178; `locate` :167 → :251; `resolve` :189 → :273; `settled_step` :257 → :342).
      Not an invariant violation — listed so the stale line numbers do not outlive the pass.
    basis: packages/escher-driver/src/execute.rs:342

  - detector: D-security-auth
    severity: warning
    section: "§Secret Management → Environment values read (none secret-bearing) (the `RUST_LOG` bullet)"
    change: >-
      Citation refresh only — `packages/escher-telemetry/src/lib.rs:119` → `:144` (the `EnvFilter::try_from_default_env`
      read); the statement stands: `RUST_LOG` is still the one env read, defaulting to `warn`, not secret.
    sidecar: >-
      2026-10-07-driver-command-spans — §Secret Management: the `RUST_LOG` filter's citation moved, lib.rs:119 → :144;
      no secret source or env read added.
    rationale: >-
      Report Citation moves (:110, "the filter :119 → :144"; security-plan 1 at :294) and Unchanged surfaces (:40,
      `RUST_LOG` stays the one env read). The D-security-auth invariant holds — no auth, crypto or secret handling was
      touched; this is the one §Secret Management line whose pointer moved.
    basis: packages/escher-telemetry/src/lib.rs:144
~~~

### test-plan

~~~yaml
proposals:
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Stand log format"
    change: >-
      Replace "one line per printed event, no ANSI" with two record classes: one line per printed event and one line per closed span whose target the allowlist admits (no line when a span is created, recorded to, entered or exited) — the span line reads `{time} {LEVEL} {target} service.name=… service.version=… span={the span's name} {the span's fields in the order recorded} message="close" time.busy=… time.idle=…`, level and target the span's, every pair (`span`, `message` and the two timings included) judged by the same allowlist and scrub as an event's field, an outside-target span writing no byte, an event's line unchanged inside a span; neither seven_guis binary runs a driver command, so neither writes a span line today (stderr by level unchanged, 0 · 1 · 1 · 1); add to the "exercised by" list escher-telemetry's five span unit tests and `stand_act_spans` (tests/blitz-tests/tests/stand_act_spans.rs:1-19); carry the mark PROVISIONAL (boundary widening on the operator's answer, inputs#I1) as obs-plan's Log format does; as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md.
    sidecar: "test-plan §3 Stand log format: the sink's line format gains a second record class, the closed span's line (PROVISIONAL, inputs#I1) — 2026-10-07-driver-command-spans"
    rationale: >-
      Report "Harness / gate surface" (report.md:68) states the sink's log format changed and that a one-sided statement is the tests↔obs bind's drift; "Schema / config" (report.md:46) and Symbols / APIs (report.md:27-35) give the span line's shape; sink record classes 1 → 2 names test-plan :102 as a `per printed event` site (report.md:61). obs-plan's own statement (obs-plan.md:145, "One line per printed event") is still the old wording too (0 hits of "closed span" in either doc) — the two must be applied together, with the same shape and the same PROVISIONAL mark (report.md:90, :136).
    basis: ".andromeda/test-plan.md:102"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Stand log format"
    change: >-
      Restate the clause "typed text is not measured (no command can type into the host's instance yet)" as: typed text is measured in process only — `stand_act_spans` drives `type` through `Session::run` with the sink installed over a capture at `RUST_LOG=info` and reads 0 occurrences of the supplied texts, 0 of 36 ids and 0 of 8 names per layout mode; on the two binaries' own stderr it is still not measured (no surface carries a call out of the process yet).
    sidecar: "test-plan §3 Stand log format: typed text now measured through the sink in process (stand_act_spans), still unmeasured on a host binary — 2026-10-07-driver-command-spans"
    rationale: >-
      Report Outcome (report.md:168) and Coverage of new surfaces (report.md:114): the capture holds 0 of the supplied texts; report.md:38: only the new check's two children run a command with the sink installed, no surface carries a call out of the process. Expected amendment 15 lists `typed text` at test-plan :102 (report.md:102); obs-plan §8's twin clause is expected amendment 4 (report.md:91).
    basis: ".andromeda/test-plan.md:102"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§5 Integration Test Strategy → Session host ↔ what it writes"
    change: >-
      Restate the closing clause "typed text is not measured (no command can type into the binary's instance yet)" as: typed text is not measured on either binary's log (no command can type into the binary's instance yet); it is measured in process, where `stand_act_spans` runs the driver's `type` with escher's sink installed over a capture and reads 0 occurrences of the typed text, of 36 ids and of 8 names (tests/blitz-tests/tests/stand_act_spans.rs:1-19; as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md). The `stand_session_quiet` sentence ("installs no log sink") stands unchanged.
    sidecar: "test-plan §5 Session host ↔ what it writes: typed-text clause restated for the in-process sink reading — 2026-10-07-driver-command-spans"
    rationale: >-
      Second occurrence of the typed-text clause (report.md:102 lists `typed text` at test-plan :181×2; the first hit, stand_session_quiet's, is still true — that check is unedited and green, report.md:167). Same measured facts as the §3 clause (report.md:114, :168, :38).
    basis: ".andromeda/test-plan.md:181"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → escher-driver"
    change: >-
      "27 inline unit tests in seven files" → 29; "execute.rs 1" → execute.rs 3 (the key table; the command span has its eight fields — verb · cause · settled · busy · passes · added · removed · changed — and none is an argument's name; each busy class reads as its schema word); "covered by the nine `stand_act_*` integration files" → ten; citation `packages/escher-driver/src/execute.rs:316-346` → `:409-472`; append to the provenance tail "execute.rs's two span tests and the count of 29 at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md".
    sidecar: "test-plan §1 escher-driver: unit tests 27 → 29 (execute.rs 1 → 3), driver-action files nine → ten, execute.rs citation moved — 2026-10-07-driver-command-spans"
    rationale: >-
      The code-side invariant holds — every new path carries a unit and an integration test, each of the nine new tests seen red under a named mutation (report.md:114-116, :176; tier 0, no mandated tier; the `busy` word is unit-only, report.md:127) — but test-plan's record of that coverage states the retired counts. Report Counts: `escher-driver` unit tests 27 → 29, both new in execute.rs, site test-plan :21 (report.md:51); `stand_act_*` files 9 → 10 (report.md:53); citation move `mod tests` :317 → :410, file 472 lines (report.md:106).
    basis: ".andromeda/test-plan.md:21 · packages/escher-driver/src/execute.rs:410-472"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → escher-telemetry"
    change: >-
      "10 inline unit tests — 9 in `format.rs`" → 15 inline unit tests — 14 in `format.rs`: the nine stated, and five on the closed span (an escher span closes into one line and writes nothing before; a content-named span field is redacted and a value stays one pair; an outside-target span writes no byte; an engine-target span prints only its safe fields; an event inside a span reads as it does outside one); citations `format.rs:200-376` → `:288-598` and `lib.rs:142-150` → `:165-173`; provenance gains "the five span tests and the count of 15 at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md"; "the five `telemetry_*` integration files" stands.
    sidecar: "test-plan §1 escher-telemetry: unit tests 10 → 15 (format.rs 9 → 14), citations moved — 2026-10-07-driver-command-spans"
    rationale: >-
      Report Counts: `escher-telemetry` unit tests 10 → 15, in format.rs 9 → 14, site test-plan :20 (report.md:52); citation moves `mod tests` format.rs :200-376 → :288-598, lib.rs :142-150 → :165-173 (report.md:109-110); coverage of the closed-span line by five unit tests and four mutation controls (report.md:115).
    basis: ".andromeda/test-plan.md:20 · packages/escher-telemetry/src/format.rs:472-598"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: >-
      After `stand_act_scroll` 4 add the tenth driver-action file: `stand_act_spans` 2 (and 2 `#[ignore]` children, the test binary re-run on one each — `RUST_LOG=info` on one, `RUST_LOG` removed on the other — each installing escher's sink through `init_with_writer` over an in-memory capture) — in both layout modes each of 27 calls run through `Session::run` (the six verbs, and a refused call for each of the eight causes) leaves exactly one `escher_driver` INFO line, `span="command"`, any other line of a call under an engine target; an acting call's line carries its settle reading and the three lengths of the returned diff, a refused call's its cause and none of the other six fields, a `snapshot`'s `verb` alone; no field name outside the stated eight, no value redacted, 0 of 36 ids, 0 of 8 names and 0 of the supplied texts in the capture; at the default level 0 driver lines beside one WARN probe; no step reads not settled, so `busy` is unit-covered only; add `tests/blitz-tests/tests/stand_act_spans.rs:1-19` to the citation list and "`stand_act_spans` at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md" to the provenance tail.
    sidecar: "test-plan §1 tests/blitz-tests: tenth driver-action file stand_act_spans (2 + 2 ignored children) recorded — 2026-10-07-driver-command-spans"
    rationale: >-
      New file `tests/blitz-tests/tests/stand_act_spans.rs` (report.md:11); its readings in Outcome (report.md:165-168) and Counts (27 → 29 run plus 2 ignored children, report.md:53; the re-execution, report.md:59); deviation 1 (one driver line per call, report.md:120) and deviation 8 (`busy` not driven, report.md:127). §1's enumeration of the driver-action files stops at nine.
    basis: ".andromeda/test-plan.md:26 · tests/blitz-tests/tests/stand_act_spans.rs:1-19"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: >-
      "`mod common;` by 14 `stand_*.rs` checks" → 15; "`mod session_common;` by 14 checks — four `stand_session_*.rs`, `stand_settle.rs` … and the nine `stand_act_*.rs`, three of which (`stand_act_ids`, `stand_act_diff`, `stand_act_keys`) declare both" → by 15 checks … and the ten `stand_act_*.rs`, four of which (`stand_act_ids`, `stand_act_diff`, `stand_act_keys`, `stand_act_spans`) declare both.
    sidecar: "test-plan §2 Directory pattern: shared-module readers 14 → 15 each, driver-action files nine → ten, four declare both — 2026-10-07-driver-command-spans"
    rationale: >-
      Report Counts: readers of `tests/session_common/mod.rs` 14 → 15 (report.md:54) and `stand_*` files reading `tests/common/mod.rs` 14 → 15 (report.md:55); `stand_act_*` 9 → 10 (report.md:53). The report's fixed-string sweep names :99 and :100 only — this line restates the same three counts in other words ("by 14 `stand_*.rs` checks", "by 14 checks", "three of which") and was not listed.
    basis: ".andromeda/test-plan.md:65 · tests/blitz-tests/tests/stand_act_spans.rs:31-32"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (stand checks)"
    change: >-
      "three of the nine driver-action checks — `stand_act_ids`, `stand_act_diff` and `stand_act_keys` — do, 14 in all" → four of the ten driver-action checks — `stand_act_ids`, `stand_act_diff`, `stand_act_keys` and `stand_act_spans` — do, 15 in all.
    sidecar: "test-plan §3 stand checks helper: `mod common;` readers 14 → 15, four of ten driver-action checks — 2026-10-07-driver-command-spans"
    rationale: >-
      Report Counts: `stand_*` files that read `tests/common/mod.rs` 14 → 15, basis `grep -c 'mod common;'` (report.md:55); `nine driver-action` sited at test-plan :99 (report.md:53). The shared module itself is unedited (report.md:13).
    basis: ".andromeda/test-plan.md:99"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (session checks)"
    change: >-
      "so do the nine driver-action checks `stand_act_*` (14 readers)" → so do the ten driver-action checks `stand_act_*` (15 readers); keep the historical "the fourteen readers at …refusal-detection/report.md" and append "the fifteenth reader, `stand_act_spans`, at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md".
    sidecar: "test-plan §3 session checks helper: `mod session_common;` readers 14 → 15, driver-action checks nine → ten — 2026-10-07-driver-command-spans"
    rationale: >-
      Report Counts: readers of `tests/session_common/mod.rs` 14 → 15, basis `grep -l 'mod session_common;'`, sites test-plan :100 ("nine driver-action", "the fourteen readers") (report.md:53-54). The module is unedited (report.md:13).
    basis: ".andromeda/test-plan.md:100"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: >-
      Append the re-count: re-counted at 2026-10-07-driver-command-spans — `run stand`'s `run.end` reads passed 109 · failed 0 · ignored 5 (+2 `stand_act_spans`, picked up by its `stand_` prefix with no script change, and +2 ignored, its two children; the selection lists 30 files) — as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md.
    sidecar: "test-plan §3 Agent-run Proof: run stand re-counted 107 → 109 passed, 3 → 5 ignored, 29 → 30 files — 2026-10-07-driver-command-spans"
    rationale: >-
      Report Counts: stand files 29 → 30, `agent-run.sh run stand` 107 → 109 passed, 0 failed, 3 → 5 ignored, site test-plan :115 (report.md:56); gate log (report.md:207); `scripts/agent-run.sh` unedited and its events unchanged in shape (report.md:68), so only the count moves.
    basis: ".andromeda/test-plan.md:115"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: >-
      After "execute.rs pins the executor's key table …" add: and the command span — its eight field names against the table the span macro emits, none an argument's name, and each busy class reading as its schema word; "the nine `stand_act_*` files drive it through a `Session`" → the ten; add that nine mutation controls of this chunk — five on the driver's span and its check, four on the sink's span line — each turned a named test red, every one of the nine new tests red at least once (escher-0.1.0/chunks/2026-10-07-driver-command-spans/evidence/controls.md); citation `execute.rs:316-346` → `:409-472`; provenance gains "the span's two tests at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md".
    sidecar: "test-plan §4 escher-driver: the executor's two span unit tests and this chunk's nine mutation controls recorded; nine → ten stand_act files; citation moved — 2026-10-07-driver-command-spans"
    rationale: >-
      Same retired claims as §1's row restated in §4: the executor's unit coverage (one test) and "the nine `stand_act_*` files", with the same moved citation. Report: two new unit tests in execute.rs (report.md:51, :114), controls (report.md:114-115, :126, :176), citation move (report.md:106, which lists test-plan :142).
    basis: ".andromeda/test-plan.md:142 · packages/escher-driver/src/execute.rs:414-445"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Session ↔ held instance"
    change: >-
      "(the nine `stand_act_*` files, §1)" → the ten; add: with escher's sink installed at `info` each call run through `Session::run` — ran or refused — leaves exactly one `escher_driver` line holding fixed words and counts only (its verb, a refused call's cause, an acting call's settle reading and the three lengths of the returned diff) and none of the ids, names, values or typed text it handled, and at the default level no line (`stand_act_spans`, in both layout modes, each reading made in a re-run child because the level is read when the sink is installed) — as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md.
    sidecar: "test-plan §5 Session ↔ held instance: the command span's integration reading added; nine → ten stand_act files — 2026-10-07-driver-command-spans"
    rationale: >-
      Restates "the nine `stand_act_*` files" (report.md:53) and is the boundary the new check extends: Outcome's three obs criteria and the security content criterion (report.md:165-168); expected amendment 15 names §5 → Session ↔ held instance (report.md:102).
    basis: ".andromeda/test-plan.md:179"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline"
    change: >-
      Append to the `cargo test --workspace` re-count chain: re-counted at 2026-10-07-driver-command-spans: 154 result lines, 656 passed · 0 failed · 10 ignored (+2 `escher-driver` unit tests and +5 `escher-telemetry` unit tests in their crates' existing lib result lines; +2 `stand_act_spans` in one new result line; +2 ignored, its two children), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` runs at /implement and in its operator pass on the tree its pre-CI commit 44ad3887 carries (escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md).
    sidecar: "test-plan §9 Local baseline: workspace re-counted 153 → 154 result lines, 647 → 656 passed, 8 → 10 ignored — 2026-10-07-driver-command-spans"
    rationale: >-
      Report Counts: workspace 153 → 154 result lines, 647 → 656 passed, 8 → 10 ignored, +9 = 2 driver unit, 5 sink unit, 2 in the new file; sites test-plan :324 (report.md:57); gate verdict (report.md:208). Of the five `8 ignored` hits on that line only the last is the base (report.md:151) — the earlier re-counts are history and stay.
    basis: ".andromeda/test-plan.md:324"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: >-
      In `session-proof`: "the nine driver-action files, `Session::run` in each, both layout modes — … `stand_act_scroll` 4 (27)" → the ten driver-action files … `stand_act_scroll` 4 · `stand_act_spans` 2 (+2 ignored children) (29); "of the crate's other 14, 12 pin the command and refusal schema, 1 the executor's key table and 1 the session's record of ids" → of the crate's other 16, … and 2 the command span's fields and busy words; keep "the nine driver-action files and the crate's 27 at …refusal-detection/report.md" as history and append "the tenth driver-action file and the crate's 29 at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md"; append the CI witness: the fork's run 37694873705 on `44ad3887` (16 of 16 jobs green on its first attempt; in the macOS, windows and default-features job logs `stand_act_spans` read as run with both parents `ok`; the MSRV job read by conclusion, the iOS and android logs not read).
    sidecar: "test-plan key Session lifecycle (session-proof): driver-action files nine → ten (27 → 29 tests, +2 ignored), crate's unit tests 27 → 29, CI witness 37694873705 added — 2026-10-07-driver-command-spans"
    rationale: >-
      The key file carries the same retired claims: `nine driver-action` twice at session-lifecycle.md:11 (report.md:53, :102), the 27-test total and the split of the crate's unit tests (13 lifecycle + 14 others = 27, now 29, report.md:51). CI run and the job-log reading: report.md:71-72.
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:11"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: >-
      In `session-start`: re-cite `packages/escher-driver/src/execute.rs:59-142` to the moved range `:81-179` (`impl Session` at :81, `run` at :108, its former body now the private `execute` at :119-178); the row's statements of `run` stand — signature and return are unchanged and nothing of a span crosses the socket.
    sidecar: "test-plan key Session lifecycle (session-start): execute.rs citation re-pinned after the span code moved `run` :82 → :108 — 2026-10-07-driver-command-spans"
    rationale: >-
      Citation move listed for the key file: `packages/escher-driver/src/execute.rs` — key file `session-lifecycle.md` 1 (:5); `run` :82 → :108, body :83-141 → :120-178 inside `execute` (:119) (report.md:106). `Session::run`'s signature and return unchanged, nothing of a span crosses the session socket (report.md:25, :40).
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:5 · packages/escher-driver/src/execute.rs:81-179"
    dependent-of: D-tests-coverage
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline facts (new bullet beside Fonts)"
    change: >-
      Add a bullet "Feature unification across the two runners": `cargo test --workspace --locked` (CI's test leg, `ci-leg.sh fast`) builds blitz-tests with blitz-dom's `tracing` call sites compiled in — cargo unifies features across the workspace and `blitz` turns `tracing` on by default — while `cargo test -p blitz-tests --locked` (what `agent-run.sh run` and a per-file gate run) builds it with them off; under the workspace build the engine logs one INFO event beside a driver `type`, which the sink prints as `INFO blitz_dom::document … message=[redacted]`, so a check that reads a sink capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green; the Fonts bullet's "measure the same on any runner, in the `-p blitz-tests` package-alone build included" is a statement about font measurement only — as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md.
    sidecar: "test-plan §9: the workspace and per-package test runners build blitz-tests with different engine features; sink-capture checks count by target — 2026-10-07-driver-command-spans"
    rationale: >-
      The invariant itself holds — the chunk used only the on-spec runners (`cargo test`/libtest, `agent-run.sh run stand`, `ci-leg.sh fast|doc|a11y|audit`; no test framework or dependency beyond `tracing` on escher-driver, report.md:44, :187-211) and the new check follows §2's re-run-on-an-ignored-child form (report.md:59). The drift is a measured difference between test-plan's two stated runners that no master records: the same file was green under `-p blitz-tests` and red under the workspace leg (report.md:39, :83-84, :138 — "No master states which features the stand checks' own test binary is built with … stated above for whoever owns it"), and §9 Fonts' unqualified "measure the same on any runner" reads wider than it now holds. Additive; drop if the orchestrator rules the fact belongs to another master.
    basis: ".andromeda/test-plan.md:318"
~~~

### obs-plan

~~~yaml
# Detector verdicts (code side): D-obs-instrumentation HOLDS (Session::run now carries its span); D-obs-stack HOLDS (tracing 0.1.44 / tracing-subscriber 0.3.23 fmt layer, no OTel crate, no new logger); D-obs-pii HOLDS (no raw PII: eight fixed-word/count fields, capture read 0 of 36 ids, 0 of 8 names, 0 of the supplied texts).
# Every proposal below is obs-plan's own text gone stale against the report's Changes — not a code violation.
# Key file read: contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md — both labels still true as written (no OTel crate built; the scrub statement speaks of "records", which a closed span is). No proposal.
# Considered and held: §2 :55 "NOT YET MEASURED — span ... naming conventions" (one span's name is not a convention); §4 :94-104 and §2 :50-53 slice searches (historical, other slices); §8 :295 (true of the two binaries; the test-build fact is proposed once, under §2 Feature wiring).
# Line numbers in `basis` for the source files were read from the working tree; "fifteen of the sixteen" in the counts proposal is my sum (5 + 1 + 10), not a figure the report states.
proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage (the `packages/escher-driver` bullet, obs-plan.md:105)
    change: >-
      Replace "observed absent — spans, events or a `tracing` dependency in `packages/escher-driver` … silent by their chunks' constraint; the settle wait and the command executor exist and carry no span: one span per driver command … is owed by the route entry "Driver command spans"" with the span as built: every call handed to `Session::run` opens one `tracing` span — target `escher_driver`, name `command`, level INFO, built by hand with `tracing::info_span!` (no `#[instrument]`), entered before `validate` and held until `run` returns, so a refusal is inside it; eight fields created unrecorded and recorded only where they apply — `verb` (the schema row's own word, never the caller's text; unrecorded for an unknown verb), `cause` (`Cause::name()` of the returned refusal, one of the eight names, recorded at one site for every `Err`), `settled`, `busy` (the schema's busy-class word), `passes` (settle's pass count, when settle answered quiet), `added` / `removed` / `changed` (the three list lengths of the returned diff); `snapshot` records `verb` alone, a refused call records `verb` (when known) and `cause` and none of the other six; never fielded: `in_view`, `advanced_ms`, screen text or its length, the session's label, the record of ids, any argument, any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`; the driver emits no event and installs no subscriber, so with none installed the span is disabled and at the default `warn` it is filtered out before creation; still observed absent — a span, event or `tracing` dependency in `Harness::settle` (`packages/blitz-test-harness`) and a span of `Session::act`'s own (the span is `run`'s); not driven: no stand step reads not settled, so `busy` is covered by its unit test only. Citations: packages/escher-driver/Cargo.toml:13-16; packages/escher-driver/src/execute.rs:64-79; :108; :183; :342; as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md. Keep the earlier "as measured at" chain for the schema and cause-name facts.
    sidecar: §4 — the escher-driver "observed absent / span owed" bullet becomes the driver command span as built (target `escher_driver`, name `command`, INFO, eight fields); the harness settle loop stays span-less (2026-10-07-driver-command-spans).
    rationale: report Symbols / APIs "The span of a driver command (new; execute.rs)" and Expected amendments 1 — the route entry "Driver command spans" is delivered, so the line's "carry no span … is owed" and "a `tracing` dependency" absent are both false; the report states the `Harness::settle` / `Session::act` half is still true.
    basis: .andromeda/obs-plan.md:105 · packages/escher-driver/src/execute.rs:74
  - detector: D-obs-instrumentation
    severity: warning
    dependent-of: D-obs-instrumentation
    section: §3 Observability Harness Contract → Logging stack (subscriber installation) (the session-library clauses of the headless-stand bullet, obs-plan.md:69)
    change: >-
      In the `packages/escher-driver` parenthesis replace "(no `tracing` dependency, no subscriber, no env read, no print)" with "(a `tracing` dependency, ungated, since 2026-10-07-driver-command-spans; no subscriber, no env read, no print)", and in the private-modules clause replace "install no subscriber, read no env var and no clock, and print or log nothing" with "install no subscriber, read no env var and no clock and print nothing; `execute` opens the one span of a command (§4), which writes only where a process has installed a sink at `info` or below"; keep "nothing in the crate prints, logs or fields any of the four" and "nothing prints, logs or fields it either" — both still true (the span fields eight fixed words and counts, none of ids, typed text, names or values, and not the record of ids); move the citation packages/escher-driver/Cargo.toml:13-15 → :13-16 and append "the command span at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md" to the as-measured chain.
    sidecar: §3 Logging stack — escher-driver now depends on `tracing` and its executor opens the command span; "no `tracing` dependency" and "print or log nothing" retired, the four-content-kinds statement kept.
    rationale: report Dependencies (`escher-driver` → `tracing`, ungated; dependencies 2 → 3) and Expected amendments 2 ("what moved is "no `tracing` dependency" and "log nothing""; the crate still installs no subscriber, reads no env var and no clock and prints nothing); same retired claim as §4 :105 restated in the harness contract.
    basis: .andromeda/obs-plan.md:69 · packages/escher-driver/Cargo.toml:16
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage → Logged events (current truth)
    change: >-
      Add a group after the escher-telemetry one — "escher-driver (in a process that installs escher's sink at `info` or below)": one closed-span line per call to `Session::run`, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed` and the layer's `message="close"`, `time.busy`, `time.idle` — six verb words, eight cause names, three busy-class words, bools and counts; no line at the default `warn`; today only the two re-executed children of `stand_act_spans` run a command with the sink installed (`escher-session`'s wire is `hello` and `stop`, `seven_guis_native` runs none) (packages/escher-driver/src/execute.rs:64-79; :183; tests/blitz-tests/tests/stand_act_spans.rs:91; as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md).
    sidecar: §6 Logged events — new group for the driver command span's line and who can produce it today.
    rationale: report Symbols / APIs "What reaches a log from a command" and "Who runs a command with the sink installed today"; the §6 list enumerates every escher emitter and has none for `escher_driver`.
    basis: .andromeda/obs-plan.md:173-175 · packages/escher-driver/src/execute.rs:74
  - detector: D-obs-stack
    severity: warning
    section: §6 Log Coverage → Log format (escher's sink) (first bullet, obs-plan.md:145)
    change: >-
      Replace "One line per printed event on stderr, no ANSI" with "One line per printed event and one per closed span whose target the allowlist admits, on stderr, no ANSI — no line when a span is created, recorded to, entered or exited", and add the span line's shape: `{time} {LEVEL} {target} service.name=… service.version=… span={the span's name}`, then the span's own fields in the order recorded, then the layer's close fields `message`, `time.busy`, `time.idle` (level and target are the span's; the two timings are `tracing-subscriber`'s own); every pair, `span` included, passes the §8 scrub by the span's target; a string is Debug-quoted (`span="command"`, `message="close"`), a number or bool bare; a span from a target outside the allowlist writes no byte at any point; an event's line is unchanged, an event emitted inside a span included (no span name, no span field); `span` is a reserved field name. Move the citation packages/escher-telemetry/src/format.rs:123-198 → :134-286. Mark the closed-span record class PROVISIONAL — a boundary widening on the operator's answer (inputs#I1), listed for the founder's batch at the Epoch 4 boundary.
    sidecar: §6 Log format — the sink gains a second record class, the closed span's line (PROVISIONAL, inputs#I1); citation moved to format.rs:134-286.
    rationale: report Symbols / APIs "The sink prints a closed span (new record class)", Schema / config (one reserved field name `span`), Counts "Sink record classes 1 → 2 … `per printed event`: obs-plan 1 (`:145`)", Harness / gate surface (a one-sided statement would be the tests↔obs bind's drift) and Expected amendments 3 with its PROVISIONAL mark (Decisions & corrections).
    basis: .andromeda/obs-plan.md:145 · packages/escher-telemetry/src/format.rs:134-177
  - detector: D-obs-stack
    severity: warning
    dependent-of: D-obs-stack
    section: §3 Observability Harness Contract → Logging stack (subscriber installation) (the `seven_guis_native` bullet, obs-plan.md:67)
    change: >-
      Replace "one non-ANSI fmt layer using the escher formatter, writing to stderr only" with "one non-ANSI fmt layer — stated once, in the private `sink_layer`: `fmt::layer().with_span_events(FmtSpan::CLOSE).fmt_fields(SpanFields).event_format(EscherFormat::new(identity)).with_ansi(false).with_writer(writer)` — that prints each event and each closed span through the escher formatter and holds a span's fields in the crate-private `SpanFields` until it closes, writing to stderr only"; move the citation packages/escher-telemetry/src/lib.rs:105-140 → :102-117; :130-163.
    sidecar: §3 Logging stack — the sink's layer restated as `sink_layer` with span-close events and the `SpanFields` field formatter; citation moved.
    rationale: report Symbols / APIs "`sink_layer(identity, writer)` (lib.rs:102, private) is the one statement of the layer" and Citation moves (`init_with_writer` :105-140 → :130-163); the same "events only, plain escher formatter" claim as §6 :145, restated as the harness's layer.
    basis: .andromeda/obs-plan.md:67 · packages/escher-telemetry/src/lib.rs:102-116
  - detector: D-obs-stack
    severity: warning
    section: §3 Observability Harness Contract → Logging stack (subscriber installation) (the stand-checks census of the headless-stand bullet, obs-plan.md:69)
    change: >-
      Give the census its one exception: the stand and its checks install no subscriber "save `stand_act_spans`, whose two parents install none and re-execute the test binary (`current_exe()`, `--ignored --exact {child} --nocapture`) on two ignored children — one with `RUST_LOG=info` set, one with `RUST_LOG` removed — each of which installs the sink with `escher_telemetry::init_with_writer` over an in-memory capture and drives commands in process; the capture is read into assertions only and a failing parent relays the child's own failure message, never a captured line", so "a headless boot made in process has no escher sink" holds for every check but those two children; append tests/blitz-tests/tests/stand_act_spans.rs:91 and "as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md".
    sidecar: §3 Logging stack — the "stand checks install no subscriber" census gains its one exception, `stand_act_spans`'s two re-executed children (sink over an in-memory capture).
    rationale: report Counts ("Test binaries that re-execute themselves … three → four — `stand_act_spans` … each child installs the sink with `init_with_writer` over an in-memory capture"; callers of `init_with_writer` among the test files 3 → 4) and Expected amendments 2 ("the census of stand checks that install no subscriber gains one exception").
    basis: .andromeda/obs-plan.md:69 · tests/blitz-tests/tests/stand_act_spans.rs:91
  - detector: D-obs-stack
    severity: warning
    dependent-of: D-obs-stack
    section: §3 Observability Harness Contract → Agent-run harness log (the test contract's, not escher's sink) (obs-plan.md:84)
    change: >-
      Replace "no `tracing` subscriber is involved and the stand checks install no escher sink" with "no `tracing` subscriber is involved in the harness itself; of the stand checks only `stand_act_spans`'s two re-executed children install escher's sink, over an in-memory capture that reaches no harness event (Logging stack, above)".
    sidecar: §3 Agent-run harness log — "the stand checks install no escher sink" narrowed to all but `stand_act_spans`'s children.
    rationale: same retired claim as the :69 census, restated in the agent-run paragraph; report Harness / gate surface confirms `scripts/agent-run.sh` is unedited and its events unchanged in shape, so only the sink clause moves.
    basis: .andromeda/obs-plan.md:84
  - detector: D-obs-stack
    severity: warning
    section: §3 Observability Harness Contract → Logging stack (subscriber installation) (the counts of the headless-stand bullet, obs-plan.md:69)
    change: >-
      Move the three counts: "14 of which read the shared module `tests/blitz-tests/tests/common/mod.rs`" → "15 of which"; "the nine driver-action checks `stand_act_*`" → "the ten driver-action checks `stand_act_*`"; "the module fourteen of the fifteen checks that hold a session read — four of the five `stand_session_*`, `stand_settle` and the nine `stand_act_*`" → "fifteen of the sixteen … and the ten `stand_act_*`".
    sidecar: §3 Logging stack — counts re-measured: `common` readers 14 → 15, `stand_act_*` files 9 → 10, `session_common` readers 14 → 15 (of 16 session-holding checks).
    rationale: report Counts / qualifiers moved — "`stand_act_*` 9 → 10 … `nine driver-action`: obs-plan 1 (`:69`)", "`stand_*` files that read `tests/common/mod.rs` 14 → 15 … `14 of which`: obs-plan 1 (`:69`)", "Readers of `tests/session_common/mod.rs` 14 → 15"; the report states the reader count, the "of the sixteen" total is derived (5 `stand_session_*` + `stand_settle` + 10 `stand_act_*`).
    basis: .andromeda/obs-plan.md:69 (re-counted in the tree — 10 `stand_act_*` files, 15 `mod common;`, 15 `mod session_common;`)
  - detector: D-obs-stack
    severity: warning
    section: §2 Telemetry Strategy → Telemetry mechanism (current truth) (first bullet, obs-plan.md:30)
    change: >-
      Widen "Telemetry is the `tracing` crate's event macros" to "event macros and, in escher-driver, one span macro (`info_span!`, §4)", and after "escher-telemetry depends on `tracing` with no feature and always compiles its startup and panic events into the two native binaries …" add "escher-driver is the second escher crate to take `tracing` ungated (`{ workspace = true }`, no `[features]` table; workspace requirement 0.1.40, the lock holds 0.1.44): its command span is compiled into both binaries through `seven_guis → escher-driver` and is reached by no code path in either"; move the citation packages/escher-telemetry/src/lib.rs:138 → :161 and add packages/escher-driver/Cargo.toml:16.
    sidecar: §2 Telemetry mechanism — escher-driver named as the second ungated `tracing` user (one span macro); install-event citation moved to lib.rs:161.
    rationale: report Dependencies (`escher-driver` → `tracing`, `{ workspace = true }`, ungated; lock gains one line, no package) and Expected amendments 5 (the "always compiles … into the two native binaries" clause stays true; the driver's span is compiled into both and reached by none).
    basis: .andromeda/obs-plan.md:30 · packages/escher-driver/Cargo.toml:16 · packages/escher-telemetry/src/lib.rs:161
  - detector: D-obs-stack
    severity: warning
    section: §2 Telemetry Strategy → Feature wiring
    change: >-
      Add a bullet: the stand checks' own test binary is built with different engine features by invocation — `cargo test --workspace` (the CI test leg and the `fast` leg) builds `blitz-tests` with blitz-dom's `tracing` call sites compiled in (cargo unifies features across the workspace and `blitz` turns `tracing` on by default), `cargo test -p blitz-tests` with them off — so under the workspace build a test child that installs the sink at `info` also receives engine events (one `INFO blitz_dom::document … message=[redacted]` beside a driver `type`, packages/blitz-dom/src/document.rs:1693); the two seven_guis binaries, built as `-p seven_guis`, are unaffected (as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md).
    sidecar: §2 Feature wiring — new measured fact: workspace test builds compile blitz-dom's `tracing` sites into `blitz-tests`, per-package builds do not.
    rationale: report Symbols / APIs "A fact about the test builds, measured here" and Spec claims disproved ("No master states which features the stand checks' own test binary is built with; the measured fact is new and stated above for whoever owns it") — an addition, it retires no obs-plan claim; obs-plan §2 Feature wiring is the home of `tracing` feature forwarding. Drop this proposal if another master is ruled its owner.
    basis: .andromeda/obs-plan.md:41 · report.md:39
  - detector: D-obs-pii
    severity: escalate
    section: §8 PII Scrubbing & Compliance → Scrubbing (first bullet, obs-plan.md:294)
    change: >-
      State that the allowlist judges two record classes by one rule: after the three outcomes for an event add "a closed span's line is judged pair by pair by the same `decide(target, name)` — its name (`span`), its own fields and the layer's `message`, `time.busy`, `time.idle`: under an escher target every pair prints unless its name is in the content-named set, under an engine target only the safe fields print and the rest, `span` and the three close fields among them, read `[redacted]`, and a span from any other target writes no byte at creation, on a recorded value, on enter, on exit or on close; the value of a content-named span field is never stored (the marker is written at print time), every other span field's value is held in process memory until the span closes"; the five scrub sets and `decide` are unchanged; `escher_driver` now has an emitter (the command span, §4). Move the citations packages/escher-telemetry/src/format.rs:21-107 → :22-111 and :134-138 → :144-148, and add :200-221 (`SpanFields`). Mark the closed-span class PROVISIONAL (inputs#I1), as §6 Log format.
    sidecar: §8 Scrubbing — the scrub now also judges a closed span's fields by the same rule; outside-target spans write nothing; content-named span values never stored (PROVISIONAL, inputs#I1); citations moved.
    rationale: the PII invariant itself HOLDS — report Coverage of new surfaces reads "PII redacted✓" for both the span and the sink's closed-span line, and Outcome reads 0 of 36 ids, 0 of 8 names, 0 of the supplied texts. Escalated only because the report records this as a boundary widening of the scrub surface on the operator's answer, "not the founder's word", to be listed for his batch (Decisions & corrections; Expected amendments 4); the text of §8 :294 speaks of events alone. Report Schema / config: sets unchanged, "`decide` is unchanged and now also judges span fields".
    basis: .andromeda/obs-plan.md:294 · packages/escher-telemetry/src/format.rs:97-111 · :146-148 · :164-173
  - detector: D-obs-pii
    severity: warning
    section: §8 PII Scrubbing & Compliance → Values logged as-is (current truth) (the "Past escher's scrub" bullet, obs-plan.md:290)
    change: >-
      Replace "Typed text is NOT measured: … types into an instance held in process only, where no sink is installed; no command reaches the instance of a sink-installing host (`escher-session`), so typed text in a host's log has not been read" with "Typed text is measured in process only: `stand_act_spans`'s children drive the six verbs, `type` included, with the sink installed over a capture at `RUST_LOG=info`, in both layout modes, and the capture holds 0 of 36 id needles, 0 of 8 name needles and 0 occurrences of the three supplied texts (the typed text, the id naming no element, the unknown verb's text), with no value reading redacted on a driver line; typed text in a HOST's log is still not measured — no surface carries a call out of the process, so no command reaches the instance of `escher-session`"; note the by-level readings above re-measured unchanged (session host 0 · 1 · 1 · 1 lines, ids 0 of 15, names 0 of 6, stdout 0 B); append "the in-process reading at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md".
    sidecar: §8 Values logged as-is — typed text now read absent in an in-process sink capture (`stand_act_spans`); a host's log still not measured; by-level readings unchanged.
    rationale: report Outcome (security: "the capture holds zero occurrences of every stable id, row text, label text and the typed text … 0 of 36 id needles, 0 of 8 name needles, 0 of the supplied texts, per mode"), Symbols / APIs "Who runs a command with the sink installed today" and Expected amendments 4, which names this site (`:290` — "where no sink is installed"); the "no sink is installed" half is now false, the host half still true. Not escalated: it narrows a not-measured statement and widens no boundary.
    basis: .andromeda/obs-plan.md:290 · report.md:168
  - detector: D-obs-stack
    severity: warning
    section: §1 Obs Scope Summary → Instrumentation scope (the escher-telemetry bullet, obs-plan.md:11)
    change: >-
      Add "printing one line per event and one per closed span" to the allowlist-formatter description and move the citation packages/escher-telemetry/src/lib.rs:1-18 → :1-22.
    sidecar: §1 — escher-telemetry's crate-doc citation moved to lib.rs:1-22; closed-span lines named.
    rationale: report Citation moves ("The crate doc `:1-18 → :1-22`"; obs-plan cites the file at `:11`) and Symbols / APIs (the sink prints a closed span).
    basis: .andromeda/obs-plan.md:11 · packages/escher-telemetry/src/lib.rs:1-22
  - detector: D-obs-stack
    severity: warning
    section: §3 Observability Harness Contract → Service identity and line format (escher's sink) (obs-plan.md:82)
    change: >-
      Move the citation packages/escher-telemetry/src/lib.rs:38-60 → :44-66 (`ServiceIdentity` :49-54, `service_identity!` :59-66); the statement itself stands — a closed span's line carries the same `service.name` / `service.version`.
    sidecar: §3 Service identity — citation moved to lib.rs:44-66.
    rationale: report Citation moves ("the re-exports `:34-36 → :40-42`; `ServiceIdentity` `:43 → :49`"; obs-plan cites the file at `:82`); the report gives the +6 shift, the range end is read from the file.
    basis: .andromeda/obs-plan.md:82 · packages/escher-telemetry/src/lib.rs:49-66
  - detector: D-obs-stack
    severity: warning
    section: §6 Log Coverage → Log format (escher's sink) (second bullet, obs-plan.md:146)
    change: >-
      Move the citation packages/escher-telemetry/src/lib.rs:119 → :144 (the `EnvFilter` line); the statement stands and holds for a span as for an event — at the default `warn` an INFO span is filtered out before it is created.
    sidecar: §6 Log format — `EnvFilter` citation moved to lib.rs:144.
    rationale: report Citation moves ("the filter `:119 → :144`"; obs-plan cites the file at `:146`) and Symbols / APIs ("At the default level (`warn`) the span is filtered out before it is created: no line").
    basis: .andromeda/obs-plan.md:146 · packages/escher-telemetry/src/lib.rs:144
  - detector: D-obs-stack
    severity: warning
    section: §6 Log Coverage → Logged events (current truth) (the escher-telemetry install-event bullet, obs-plan.md:174)
    change: >-
      Move the citation packages/escher-telemetry/src/lib.rs:138 → :161 (the `telemetry installed` event); the statement stands.
    sidecar: §6 Logged events — install-event citation moved to lib.rs:161.
    rationale: report Citation moves ("the install event `:138 → :161`"; obs-plan cites the file at `:174`).
    basis: .andromeda/obs-plan.md:174 · packages/escher-telemetry/src/lib.rs:161
~~~

### a11y-plan

~~~yaml
proposals:
  - detector: D-a11y-obs-schema
    severity: warning
    section: "§3 A11y Assertion Harness Contract — the closing `NOT YET MEASURED` note (the sentence opening \"The obs log format is now measured for escher's own sink\")"
    change: "Restate the sink's format as two record classes: non-JSON text lines on stderr — one per printed event, `{RFC 3339 UTC time} {LEVEL} {target} service.name=… service.version=… {field}={value}…`, and one per closed span of an admitted target, the same prefix then `span={the span's name}`, the span's recorded fields and the layer's close fields `message` / `time.busy` / `time.idle` (no line at span creation, record, enter or exit) — every pair of both classes judged by the same target allowlist and field rule already stated here, an outside target's span writing nothing; keep the closing clause that an a11y violation schema is not yet defined against it."
    sidecar: "2026-10-07-driver-command-spans — §3 closing note: the obs log format a11y would emit to is now two record classes (printed event · closed span); `one non-JSON text line per printed event` retired as the sole class; no a11y violation schema exists yet, so nothing else moves."
    rationale: "The report changes the obs log schema: Schema / config (report.md:46) says the sink's line format gains a second record class (the closed span's line) and one reserved field name `span`; Symbols / APIs (report.md:27-31) gives the line shape and that `decide(target, name)` judges every pair; Counts (report.md:61) reads `Sink record classes 1 → 2` and names a11y-plan `:87` as a site stating the old value; Expected amendment 17 (report.md:104) names this note. No a11y violation schema exists to diverge (report.md:46), so the only drift is a11y-plan's own restatement of the obs §6 format, which now describes one class of two. The allowlist / redaction clauses of the same sentence stay true: the scrub sets and `decide` are unchanged (report.md:46). The obs-plan side of this record class is marked Boundary widening, PROVISIONAL (report.md:90, :136); the a11y note restates obs §6 and owns no ratification mark."
    basis: ".andromeda/a11y-plan.md:87"

# Detector verdicts with no proposal:
# - D-a11y-surface: no drift. The report adds no interactive UI element. Its Coverage of new surfaces
#   (report.md:114-116) reads `a11y n/a` for the driver span, the sink's closed-span line and
#   `stand_act_spans`, whose fixture is "a test fixture with inline geometry, not a UI surface of the
#   stand" with "no stand markup, id, class or style touched".
# - D-a11y-obs-schema duplicate sweep: the retired claim (the sink writes one line per printed event,
#   events only) occurs once in a11y-plan, at :87. Grep for `per printed event`, `printed event`,
#   `one line per`, `record class`, `log format`, `sink` and `span` over a11y-plan.md finds no second
#   site (the `one line per snapshot node` hit at :270 is the snapshot text form, another subject).
#   The one keyed contract, `bootstrap-phases-derive-for-route-setup-project.md`, does not state it.
#
# Outside both detectors — not proposed, listed because report Expected amendment 17 (report.md:104)
# names them in a11y-plan and no a11y-plan detector covers a citation or dependency line:
# - §1 → Dioxus crates (a11y-plan.md:14): cites `packages/escher-driver/Cargo.toml:13-15`; the report
#   moves `[dependencies]` to `:13-16` (report.md:108); read in the file, `tracing` is line 16. The
#   same bullet's "the lockfile is byte-identical" is the act-by-id chunk's measurement; the lock has
#   since gained this chunk's one line (report.md:44, :93). "no `[features]` table" still holds.
# - §7 (a11y-plan.md:270): cites `packages/escher-driver/src/execute.rs:205-223` for the `disabled`
#   refusal; the report moves that block 84 lines down (report.md:106, `action_point` `:207 → :291`);
#   read in the file, the doc comment and `action_point` now span `:289-307`.
~~~
