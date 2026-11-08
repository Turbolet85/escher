@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: the workspace line readings of 2026-10-05 and 2026-10-14 are history with no threshold, and the chunk re-measures per-crate floors, not the workspace total
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=yes
why: it states the per-crate floors escher-driver 97 % and dioxus-native-dom 83 % that the new code and its tests must keep (task item 12), and how a moved test module moves the crate's reading
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=yes
why: it states that dioxus-native-dom's 83 % floor counts mutation_buffer.rs with no exclusion, the reading the chunk's floor re-measure reads over the crate the chunk edits
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes
why: it states the stand-repeat zero-failure budget over stand_*.rs, the seventeen named targets, seven_guis process checks, three rounds and no retry, which the new checks join (task item 13)
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the cargo-mutants nextest tier runs on the dev host alone over timer.rs, framing.rs and mutation_writer.rs, none reached by the chunk, and is not_needed (task item 8)
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=no
why: the cognitive-complexity ceiling is a dev-host audit read over named functions with no CI leg and no ceiling on every function, and it names no unit the chunk edits
