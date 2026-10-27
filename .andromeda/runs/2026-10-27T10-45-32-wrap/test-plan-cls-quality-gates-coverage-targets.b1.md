# Classified — test-plan · quality-gates-coverage-targets.b1

## Answers
quality-gates-coverage-targets#1.0 · no · both readings are dated to their own chunks (2026-10-05, 2026-10-14) with evidence files the session does not reach, and the session re-measures no workspace coverage
quality-gates-coverage-targets#2.1 · yes · stale · .andromeda/residuals.md line 35, the owner its line 15 names (target: next), moves to Epoch 3's «Upstream sync ahead of the UI graph», and dioxus_document.rs, a conflict surface in the crate read at 85.60 %, can move that reading
quality-gates-coverage-targets#3.1 · no · ci.yml, ci-leg.sh and the 50 stand_*.rs files are unchanged, the new target is no stand_ file nor one of the ten named targets, and the stand-repeat job reads unmoved at 204 · 864 · 0 · 21 on CI#40487650912
quality-gates-coverage-targets#4.1 · no · .config/nextest.toml and coverage_floor.py are unchanged, and none of the scoped units (timer.rs, mcp/framing.rs, mutation_writer.rs) is a reached file
quality-gates-coverage-targets#5.1 · yes · stale · the cherry-picks reach blitz-dom's merge and conflict surfaces (mutator.rs, node/element.rs, layout/construct.rs), which can move the 85-over count and the blitz-dom readings it names, and no complexity reading is re-measured
