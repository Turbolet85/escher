# Controls — 2026-10-07-command-and-refusal-schema (plan step 7)

Implement's hand record, 2026-10-07. Each mutation was applied to the working tree, the crate's unit
tests were run once under it — `cargo test -p escher-driver --locked --lib`, exit read from the bare
command — the named test was read red, and the mutation was reverted. No listed gate re-produces this
file (plan §Test Commands).

The run before any mutation: 25 passed · 0 failed (gate entry 4 of the first block run).

## Readings

| # | mutation | site | exit | result line | named test | its reading | also red |
|---|---|---|---|---|---|---|---|
| 1 | the passed-twice check removed (the `if slot.is_some()` return dropped) | `packages/escher-driver/src/command.rs:170-172` | 101 | `FAILED. 23 passed; 2 failed` | T10 `command::tests::a_malformed_call_is_refused_with_the_rule_it_broke` | red — `row 11` (an argument passed twice) | T11 `a_call_that_breaks_two_rules_gets_the_first_in_order` — `row 1` |
| 2 | the id bound compared with `<` where it reads `<=` | `packages/escher-driver/src/command.rs:208` | 101 | `FAILED. 23 passed; 2 failed` | T8 `command::tests::an_admitted_call_becomes_its_command` | red — `row 3` (an id of exactly the bound) | T12 `every_verb_of_the_table_is_reached_and_names_its_own_spec` — `row 3 is admitted` |
| 3 | two remedies swapped (`unknown-verb` and `not-found`) | `packages/escher-driver/src/refusal.rs:77` · `:82` | 101 | `FAILED. 24 passed; 1 failed` | T2 `refusal::tests::each_cause_states_its_meaning_and_its_remedy` | red — `assertion left == right failed: unknown-verb` | none |
| 4 | one verb dropped from the table (`click` removed from `VERBS`) | `packages/escher-driver/src/schema.rs:270` | 101 | `FAILED. 17 passed; 8 failed` | T4 `schema::tests::the_five_verbs_are_named_in_order` and T12 `command::tests::every_verb_of_the_table_is_reached_and_names_its_own_spec` | T4 red — `assertion left == right failed` at the row count; T12 red — `row 1 is admitted` | T5, T6, T7 (schema) and T8, T10, T11 (command) |

Line numbers are the unmutated files'. A row number in a reading is the index of the row in the
test's stated table; no assertion message of `command.rs` carries a value of a call.

## Reverts

Each file was hashed before the first mutation and after the last revert; `command.rs` and
`refusal.rs` were also hashed after their own reverts. Every reading equals the first.

| file | sha256 before the controls | after its reverts | `cmp` against the saved copy |
|---|---|---|---|
| `packages/escher-driver/src/command.rs` | `6ac893e51f7fc7bbf1a486d66af2c21fe972682bfe78d0f6984b3296822706ea` | the same, read after mutation 1, after mutation 2 and at the end | 0 |
| `packages/escher-driver/src/refusal.rs` | `2c5a6dd8b38d90157c08c9be2b3230a5c0296157a1151d88ae1f96e1895226f5` | the same, read after mutation 3 and at the end | 0 |
| `packages/escher-driver/src/schema.rs` | `6a71d4f4161106b37f599002f2893d41b23a720030cfab1fe4d09ee8e8a81331` | the same, read at the end | 0 |

The whole gate block was run again after the last revert; its listing is the second `run` record of
this chunk's gate trail in the implement run dir.

## Notes

- Mutation 1 left the bound name `arg` unused, so it was written `let _ = arg;` in place of the
  check: the mutation removes the check and nothing else.
- Mutations 1, 2 and 4 turn more tests red than the plan names. The plan names the test each must
  turn red; the others are recorded, not required.
