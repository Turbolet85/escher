# Classified — test-plan · quality-gates-coverage-targets.b1

## Answers
quality-gates-coverage-targets#1.0 · no · the session re-reads only the per-crate readings at line 16 and no workspace figure, so the dated 2026-10-05 and 2026-10-14 workspace readings hold
quality-gates-coverage-targets#2.1 · yes · stale · the per-crate readings at line 16 moved, dioxus-native-dom 85.57 to 85.60 % and blitz-test-harness 94.15 to 94.44 %, escher-driver 97.99 % unchanged and the floors unchanged
quality-gates-coverage-targets#3.1 · yes · stale · the stand-repeat leg takes the two new `stand_*.rs` files by prefix, so its 48 files become 50, with 3 of 3 rounds green and no CI step changed
quality-gates-coverage-targets#4.1 · no · the mutation tier is untouched, the diff over packages is 0 and none of its measured files (timer.rs, mcp/framing.rs, mutation_writer.rs) is among the reached files
