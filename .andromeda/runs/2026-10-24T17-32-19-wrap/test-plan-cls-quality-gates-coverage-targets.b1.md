# Classified — test-plan · quality-gates-coverage-targets.b1

## Answers
quality-gates-coverage-targets#1.0 · no · its workspace line readings are dated (2026-10-05, 2026-10-14) and rest on escher-0.1.0 evidence, no reached file, while the session brings only per-crate floor readings
quality-gates-coverage-targets#2.1 · yes · stale · the session edits ci-leg.sh and test_ci_workflows.py, both reached whole and cited here at 40-47, 41-45 and test_ci_workflows.py:327, and reads the floors at 97.99 · 85.57 · 94.15 %, not the readings this entry carries
quality-gates-coverage-targets#3.1 · yes · stale · the stand-repeat leg now runs ten named targets beside the 48 stand_*.rs files and seven_guis' eight, and its tally moves from 168 · 684 · 0 · 21 to 198 · 837 · 0 · 21 at CI#40114977936
quality-gates-coverage-targets#4.1 · no · the mutants profile in .config/nextest.toml and the units it runs (timer.rs, mcp/framing.rs, mutation_writer.rs) are not reached, and the report shows the tier untouched
