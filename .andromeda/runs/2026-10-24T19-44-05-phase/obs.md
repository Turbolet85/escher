# obs extract

## Relevance
partial — the chunk re-reads the a11y checks and widens a CI leg. It adds no telemetry. Obs applies only as the silence of the checks it touches, the ban on new log and span sites, and the a11y leg's CI log.

## Constraints
- Every check this chunk extends or re-baselines (`stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`, and any new Enter/Space or select row) must install no subscriber, read no env var, print nothing and bind no listener, with its readings going into assertions only (per obs-plan §3 Observability Harness Contract → Logging stack: the 2026-10-14-stand-a11y-assertions line for those three files and the shared module `tests/blitz-tests/tests/common/mod.rs`).
- The candidate witnesses that may join the a11y leg — `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread`, `accessibility_merged_states`, `select_interaction`, `stand_act_select` — are each recorded as silent. If one is moved or edited, that silence must still hold. Whether `accessibility_label` and `accessibility_description` are equally silent is not stated in the plan; that is research's question (per obs-plan §3 Observability Harness Contract → detail "absence · logs": the engine-debts and select lines).
- No log site, span, span field, log field or env read may be added. escher-driver's span set stays at `command` and `tool_call`, with no field added to either (per obs-plan §4 Span / Trace Coverage).
- A stand control's a11y readings (ids, accessible names, focus order, contrast ratios) reach only assertions, never a sink line. escher's sink admits engine and `escher_*` targets alone, and content-named fields are redacted (per obs-plan §8 PII Scrubbing & Compliance → Scrubbing).
- Which runner the a11y leg uses sets which engine `tracing` events exist. `cargo test -p blitz-tests` leaves `blitz-dom/tracing` off, and a workspace build turns it on. That matters only if a check installed a sink, which none may do. Whether the widened leg keeps its current invocation is research's question (per obs-plan §2 Telemetry Strategy → Feature wiring, "By invocation").
- The widened a11y leg still writes its merged output to `target/ci-logs/a11y.log`, truncated at the leg's start. It is uploaded only on failure as `ci-log-{job id}`, kept 7 days and holds no user data. Widening the leg adds no artifact (per obs-plan §9 CI Integration → detail, Per-leg CI log row).

## Patterns to follow
- The silence probe `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` must read 0 over each touched check file and the shared module (per obs-plan §3 Observability Harness Contract → Logging stack, 2026-10-14-stand-a11y-assertions line).
- State each silence census as env · print · subscriber counts per file. Also confirm escher-driver keeps its 2 `info_span!` sites and 0 events, as the select and load-wait chunks did (per obs-plan §3 Observability Harness Contract → detail "absence · logs").
- If the re-read finds an engine defect, the fix is a founder fork. Any fix that lands must add no log site, span, field or env read, and must read no wall clock in blitz-dom outside `clock.rs` (per obs-plan §3 Observability Harness Contract → detail "absence · logs", the select and upstream-take lines).

## Anti-patterns to avoid
- Installing `escher_telemetry::init`, `tracing_subscriber` or a hand-written `Subscriber` in an a11y check. Only `stand_act_spans`' re-run children and `stand_mcp_tools` are the recorded exceptions (per obs-plan §3 Observability Harness Contract → Logging stack).
- Printing or logging a reading (an id, a name, a Tab sequence or a contrast pair) to diagnose a moved table. Readings go into assertion values only (per obs-plan §8 PII Scrubbing & Compliance → Values logged as-is, "Past escher's scrub").

## Contract bindings
- obs ↔ tests §3 / CI: the a11y leg's per-leg log `target/ci-logs/a11y.log` and its failure-only upload. The leg pin (`test_ci_workflows.py` `LegScriptTest`) moves with the leg, but the log and artifact shape do not (per obs-plan §9 CI Integration).
- obs ↔ security §Logging: no a11y reading enters a sink line, and the scrub and allowlist stay unchanged (per obs-plan §8 PII Scrubbing & Compliance).
- obs ↔ a11y: the a11y-plan's violation-JSON binding to obs's log format has no coverage, because obs-plan §3 marks a log JSON schema for escher's sink NOT YET MEASURED. Nothing binds here.

## Acceptance criteria contributions
- The silence probe reads 0 over every blitz-tests file this chunk adds or edits, and over `tests/blitz-tests/tests/common/mod.rs` (per obs-plan §3 Observability Harness Contract → Logging stack).
- escher-driver still holds exactly 2 `info_span!` sites and 0 events, and `packages/escher-telemetry/` is unchanged (per obs-plan §4 Span / Trace Coverage).
- After widening, `bash .github/scripts/ci-leg.sh a11y` writes `target/ci-logs/a11y.log`, and ci.yml's a11y job keeps its failure-only `ci-log-*` upload with no new artifact (per obs-plan §9 CI Integration).
