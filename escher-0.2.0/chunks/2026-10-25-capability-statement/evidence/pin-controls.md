# The pin's red-before-green controls — 2026-10-25-capability-statement

`.github/scripts/test_capability_statement.py`, run against a scratch copy (`$TMPDIR/{scratchpad}/ctl/`) holding the
module, `schema.rs`, `refusal.rs`, `error.rs`, `README.md` and `docs/capabilities.md`, each mutated as below; the
module reads its root as `parents[2]` of its own file, so the copy read only the copy.

Green on the working tree: `Ran 4 tests` · `OK` · exit 0.

Control 1 — README back to «the eleven driver commands», the statement's «fifteen session errors» as «fourteen»:
exit 1, `FAILED (failures=2)` —
- `test_the_readme_counts_the_mcp_tools`: `the twelve driver commands as tools`
- `test_the_table_counts_are_stated_in_words`: `fifteen session errors`

Control 2 — every `` `lint` `` and `` `fonts-unavailable` `` code span unquoted in the statement: exit 1,
`FAILED (failures=2)` —
- `test_every_cause_is_named_as_a_code_span`: `fonts-unavailable`
- `test_every_verb_is_named_as_a_code_span`: `lint`

Every case reads red on its own drift. The first green run read red once on the real tree: «thirteen node keys» was
wrapped across two source lines; the module now reads the markdown with its whitespace folded to single spaces.
