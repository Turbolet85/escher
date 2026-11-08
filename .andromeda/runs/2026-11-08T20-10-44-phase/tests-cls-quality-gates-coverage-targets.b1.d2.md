@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: the workspace-wide line reading of 2026-10-05 and 2026-10-14 is historical with no threshold, and the chunk re-measures per-crate floors, not the workspace total
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=yes
why: task item 12 requires the new code and tests to keep escher-driver at 97 % and dioxus-native-dom at 83 %, with no exclusion added for the new verb or the graph, signal registry and bridge code
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=yes
why: the chunk's re-measured dioxus-native-dom floor reads mutation_buffer.rs with no exclusion, so the crate reading it must hold counts this file's 93.97 % in
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes
why: task item 13 names the stand-repeat zero-failure budget with no retry, which the new stand_ checks for the coverage read join
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the cargo-mutants nextest tier is not_needed for this chunk, which adds no mutation run and names no mutated package
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=no
why: the cognitive-complexity ceiling is a dev-host audit reading that no CI leg runs and that is no ceiling on every function, so the chunk's new functions owe nothing
