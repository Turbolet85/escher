# Classified — tests · quality-gates-coverage-targets.b1

## Answers
quality-gates-coverage-targets#1.0 · no · a historical workspace line-coverage reading (53.23 % then 67.06 %) that the chunk neither states nor re-measures; it changes no crate source and runs no coverage leg
quality-gates-coverage-targets#2.1 · no · the three per-crate coverage floors, `coverage_floor.py` and the keyboard_compat exclusion; the chunk changes no crate source and reaches neither the coverage leg nor `coverage_floor.py`, and the CI-script pin it names stays green as a re-check every change owes
quality-gates-coverage-targets#3.1 · no · the stand-repeat membership and the zero-failure budget; the chunk adds, renames and removes no stand check, named target or process check and runs none, and its two re-reads change no test body, so the budget is a re-check every change owes
quality-gates-coverage-targets#4.1 · no · the cargo-mutants procedure on the dev host; no function is written or changed, so this documentation and records chunk owes no mutant run
quality-gates-coverage-targets#5.1 · no · the cognitive-complexity ceiling and the functions it names; no function is written or changed, and none of the named functions sits in a reached file
