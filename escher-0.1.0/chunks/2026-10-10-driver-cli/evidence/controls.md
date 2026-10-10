# Seen red first — the one-shot mutation controls (step 14)

Taken at implement, 2026-10-10, on the chunk's tree after its checks were written. Each control: the file's bytes
saved, one site mutated (matched exactly once), the listed check run — red — the saved bytes written back, the
file's sha256 compared with the one taken before, and only then the same check run again — green. Each check ran as
`cargo test -p {package} --locked --no-fail-fast [--test {target}]`. sha256 is given by its first 16 hex digits. The
instrument was a scratch script; it is in no commit. The listed gate entries guard the same properties on every
re-run.

| # | Mutation | File · sha256 before → mutated → restored | Check | Red reading | Green reading after the restore |
|---|---|---|---|---|---|
| 1 | the executor writes the typed text as a field of a log event (`tracing::info!` under `escher_driver`, a field named outside the sink's scrub set) | `packages/escher-driver/src/execute.rs` · `f784083de4f6a280` → `6572ca7126653bfc` → `f784083de4f6a280` | seven_guis `host_log` | exit 101 · 0 passed · 1 failed: `the_host_logs_no_id_no_name_and_no_typed_text_at_trace` | exit 0 · 1 passed |
| 2 | the JSON writer writes a `"` inside a string as it is | `packages/escher-driver/src/json.rs` · `e7131f7720ffcc92` → `077a338e94503e62` → `e7131f7720ffcc92` | escher-driver (unit) | exit 101 · 48 passed · 2 failed: `json::tests::a_string_is_written_with_its_escapes`, `json::tests::each_outcome_is_written_with_its_verbs_fields_in_order` | exit 0 · 50 passed |
| 2 | (the same mutation) | (the same file) | seven_guis `cli_commands` | exit 101 · 6 passed · 2 failed: `a_snapshot_is_the_in_process_text_as_one_line_inside_its_budget`, `each_verb_is_accepted_and_answers_its_fields` | exit 0 · 8 passed |
| 3 | `type` does not select the control's content before typing | `packages/escher-driver/src/execute.rs` · `f784083de4f6a280` → `0898ce5cb4db0d3f` → `f784083de4f6a280` | blitz-tests `stand_act_filled` | exit 101 · 1 passed · 3 failed: `a_type_leaves_exactly_its_text_in_a_filled_control`, `a_type_with_an_empty_text_clears_the_control`, `a_masked_controls_written_form_holds_the_mask_and_never_the_typed_text` | exit 0 · 4 passed |
| 4 | the counter flow expects the value `2` after one click | `examples/seven_guis/tests/flows/counter.sh` · `7df798a8a9ac0bb3` → `bfc5489ae5983fb0` → `7df798a8a9ac0bb3` | seven_guis `cli_flow` | exit 101 · 1 passed · 1 failed: `each_flow_completes_under_plain_sh_and_leaves_nothing` | exit 0 · 2 passed |
| 5 | the host's idle expiry never fires | `packages/escher-driver/src/host.rs` · `17ab3c2ebd3aee92` → `d8f64bf5384be11d` → `17ab3c2ebd3aee92` | blitz-tests `stand_session_lifecycle` | exit 101 · 1 passed · 1 failed: `a_host_with_no_request_for_its_expiry_ends_as_stop_does` (red at its 20 s bound) | exit 0 · 2 passed |
| 6a | the answer bound's comparison refuses an answer of exactly the bound (`>` → `>=`) | `packages/escher-driver/src/wire.rs` · `0f47aa0a19dce8e4` → `dfb6da286019a94c` → `0f47aa0a19dce8e4` | escher-driver (unit) | exit 101 · 49 passed · 1 failed: `wire::tests::an_answer_at_its_bound_is_carried_and_one_byte_over_is_not` | exit 0 · 50 passed |
| 6b | the answer bound's comparison admits one byte more (`> MAX` → `> MAX + 1`) | `packages/escher-driver/src/wire.rs` · `0f47aa0a19dce8e4` → `3a6ec7ad1d30a88d` → `0f47aa0a19dce8e4` | escher-driver (unit) | exit 101 · 48 passed · 2 failed: `wire::tests::an_answer_at_its_bound_is_carried_and_one_byte_over_is_not`, `client::tests::a_calls_reply_reads_as_its_answer_or_as_a_session_error` | exit 0 · 50 passed |
| 7 | the measured-limit sentence is removed from `covered`'s meaning | `packages/escher-driver/src/refusal.rs` · `f7e9aab44eee5250` → `32fe6d84f34575f5` → `f7e9aab44eee5250` | escher-driver (unit) | exit 101 · 48 passed · 2 failed: `refusal::tests::each_cause_states_its_meaning_and_its_remedy`, `json::tests::each_refusal_is_written_from_its_causes_fixed_strings` | exit 0 · 50 passed |

Every restore read equal to the bytes saved before its mutation. The plan lists the answer bound as one control
("the comparison moved by one byte"); it was run in both directions (6a, 6b), since each direction turns a different
row red: the row at the bound, and the row one byte over it.

## Two reds this run met that no control planted

- `stand_act_disabled` (an unedited standing check) went red on the first full gate pass: it pinned the old reading
  of `type` on a filled control — "the typed text is all that the value gained". The founder's fork 8 changed that
  reading, so the assertion was restated to "the value reads the typed text, and nothing else"
  (`scope-record.md`, a companion). It is this chunk's change seen red by a check the plan called unaffected.
- `stand_act_spans` (the same class): it reads every row of the verb table back from a command span, and the table
  grew by three rows no session runs; restated to the instance-level rows (`scope-record.md`, a companion).
