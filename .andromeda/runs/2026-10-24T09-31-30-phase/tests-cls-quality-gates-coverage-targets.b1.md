# Classified — tests · quality-gates-coverage-targets.b1

## Answers

quality-gates-coverage-targets#1.0 · no · historical workspace line readings with no threshold applied, so the chunk binds only the per-crate floors of #2.1, not this figure
quality-gates-coverage-targets#2.1 · yes · the coverage leg's three per-crate line floors (dioxus-native-dom 83 %, escher-driver 97 %, blitz-test-harness 90 %) that the chunk's coverage item must hold, with the keyboard_compat.rs exclusion that drops only if the take carries 286874cc1
quality-gates-coverage-targets#3.1 · yes · the stand-repeat leg's zero-failure budget of three rounds with no retry over the 48 stand_*.rs files and seven_guis' eight process checks, which the chunk's stand-repeat item requires
quality-gates-coverage-targets#4.1 · no · the mutation tier runs on the dev host alone and is listed under the task's Not needed as not owed by a sync, so the chunk runs no mutants and changes no nextest profile
