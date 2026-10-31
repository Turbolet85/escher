# Classified — test-plan · quality-gates-coverage-targets.b1

## Answers
quality-gates-coverage-targets#1.0 · no · the 53.23 % and 67.06 % workspace readings are history at 2026-10-05 and 2026-10-14, and the session moves only the per-crate reads at line 16
quality-gates-coverage-targets#2.1 · yes · stale · the per-crate reads at line 16 move to escher-driver 98.04 %, dioxus-native-dom 86.12 % and blitz-test-harness 94.84 % on CI#41192433140, the floors holding
quality-gates-coverage-targets#3.1 · yes · stale · stand-repeat's named targets move from ten to fifteen, so this line's ten-target `--test` list is stale
quality-gates-coverage-targets#4.1 · no · `.config/nextest.toml` is not reached and the scoped mutant files (timer.rs, framing.rs, mutation_writer.rs) sit outside the reached list, so the claim holds
quality-gates-coverage-targets#5.1 · no · the complexity gate is untouched, and its readings are the 2026-10-26 audit's past-run measures, which stay as they read
