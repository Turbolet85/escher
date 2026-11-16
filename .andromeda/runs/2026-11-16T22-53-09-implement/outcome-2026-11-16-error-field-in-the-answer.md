# Outcome — 2026-11-16-error-field-in-the-answer

**Outcome:** green — 10 of 10 run entries green on the first firing (0 fix iterations); the three plan-named controls (four rows) hold; smoke skipped — no boot-path / UI-surface change (`host_log` and `mcp_log` drove both binaries at P2).

## Files changed (5 modified · 0 new)
- `packages/escher-telemetry/src/capture.rs` — crate-private `ANSWER_HIDDEN_FIELDS` (the eleven names, a literal list, `CONTENT_FIELDS` without `error`) read by `Values::keep`; `CONTENT_FIELDS` dropped from the `use`; module and `KeptField::value` docs; table unit test `of_the_sinks_content_names_error_alone_keeps_its_text` (12 literal rows)
- `packages/escher-telemetry/src/format.rs` — `content_fields_redact_for_any_target` gains `decide("escher_driver", "error") == Redact`
- `packages/escher-driver/src/schema.rs` — `caused`'s help: `error` dropped from the hidden list; «a field named error and every message cross as their author wrote them … such as a failed fetch's URL»; six lines replaced by six (`schema.rs:194-199`), no line below moves
- `packages/escher-driver/src/caused.rs` — module doc and `fields` doc name the answer's own set
- `tests/blitz-tests/tests/stand_act_error_text.rs` — `parse-warn` control and step 8 of `every_failure` (`the_parse_warn`); in-file `FailingNet` stand-in, `failing_load_page`, `hold_failing`, `failing_load`; new test `a_failed_fetch_answers_its_error_text_in_the_engines_error_field`; the founder sink check runs the failing load and gains four needles and the engine-line read

## Deviations
- Plan step 5 says adding the step to `every_failure` makes the MCP, socket and kept-check cases read it; those cases replay `CROSSING` and their own call list, not `every_failure`. To meet the acceptance's carrier clause, `parse-warn` was added to `CROSSING` (2 → 3 ids) and to the kept check's calls (3 → 4), the replay's changed-cut stop moving from step 2 to step 3 and the kept text also asserted to hold `PARSE_ERROR`. Same file, in the plan's list.
- Scope record: none — `gate.py scope` clean, changed 5 · listed 5 · recorded 0.

## Gates (gate.py run, `.andromeda/runs/2026-11-16T22-53-09-implement`)
- 1 `cargo test -p escher-telemetry --locked --lib` green (26 passed)
- 2 `cargo test -p escher-driver --locked --lib` green
- 3 `cargo test -p blitz-tests --locked --test stand_act_error_text` green (8 passed, 2 ignored)
- 4 `… --features dioxus-native-dom/tracing --test stand_act_error_text` green (8 passed, 2 ignored)
- 5 `command grep -c "request, error, panic.payload" …schema.rs` green (exit 1, last line 0)
- 6 companions `stand_act_errors` · `stand_replay` · `stand_act_spans` · `stand_mcp_tools` green
- 7 `telemetry_scrub` · `telemetry_drop` · `telemetry_panic_hook` green
- 8 seven_guis `host_log` · `mcp_log` · `mcp_server` · `cli_commands` green
- 9 `bash .github/scripts/ci-leg.sh fast` green (322.84 s)
- 10 `bash .github/scripts/ci-leg.sh stand-repeat` green (876.75 s)
- 11–13 not run — leg operator (hygiene, push, CI read): the operator pass's
- no guard entry; no defer; no live round

## Controls (`gate.py control`, `control-1.toml`; evidence `control-1.txt`)
control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed
- `answer-err-unit` (entry 1) · `answer-err-stand` (entry 3) — `"error"` added to `ANSWER_HIDDEN_FIELDS`
- `sink-err` (entry 1) — `"error"` removed from `CONTENT_FIELDS`
- `capture-sink-set` (entry 4) — `Values::keep` on `CONTENT_FIELDS` again; its red also shows entry 4 ran the engine branch

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / test binaries / `escher-session` children of entries 1–10 and the controls | this run (gate tool) | terminated — `gate.py census --comm escher-session --comm seven_guis_nati --comm cargo --args stand_act_error_text`: none running |

## Decisions & corrections
- Guard refusal: a `cat` heredoc writing the test-edit script to the scratchpad was refused by the bash guard; re-issued through the Write tool and run by path.
- Recurrence of a standing learning (observability rules, 2026-10-06): `tracing::warn!(target: …, panic.payload = …)` failed to parse; written as `tracing::event!(target: …, Level::WARN, { panic.payload = … }, …)`.

Next → the OPERATOR PASS (entries 11 hygiene, 12 push, 13 CI read), then `/andromeda-wrap-session`.
