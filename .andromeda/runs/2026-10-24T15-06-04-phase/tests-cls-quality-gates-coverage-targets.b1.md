# Classified — tests · quality-gates-coverage-targets.b1

## Answers
quality-gates-coverage-targets#1.0 · no · a dated first workspace line-coverage reading with no threshold applied, which gates nothing and sets no rule the chunk's checks or floors must honour (the per-crate floors are #2.1)
quality-gates-coverage-targets#2.1 · yes · the per-crate coverage floors the leg gates in ci-leg.sh and coverage_floor.py (escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90), which the chunk must keep passing and may re-read, raise or add to for blitz-dom
quality-gates-coverage-targets#3.1 · yes · the zero-failure flakiness budget of the stand-repeat leg and its Stand checks repeated job (the stand_*.rs files and the eight process targets, three rounds, no retry), which the new stand_requirement_sweep.rs and every new stand_ check join
quality-gates-coverage-targets#4.1 · yes · the mutation-tier procedure (cargo-mutants under the nextest mutants profile, --baseline=skip, each unit's baseline read green first, dev host only, never in CI), the witness the chunk's Needs name for v020-02 and its record cites
