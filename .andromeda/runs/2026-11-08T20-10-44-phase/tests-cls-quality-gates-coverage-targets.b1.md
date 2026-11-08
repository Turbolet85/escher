@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: a historical unthresholded whole-workspace line-coverage reading from 2026-10; it states no floor or rule the chunk's code or tests must honour
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=yes
why: task item 12 names the per-crate floors escher-driver 97 % and dioxus-native-dom 83 % that the new code and its tests must keep, and this entity states them
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=no
why: a reading of mutation_buffer.rs, a file the chunk neither edits nor reaches; the floor it sits inside is already carried by quality-gates-coverage-targets#2.1
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes
why: task item 13 names the stand-repeat zero-failure flakiness budget over the stand_*.rs files and seventeen targets, which the chunk's new stand checks join
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the dev-host cargo-mutants nextest tier is listed not_needed by the task (item 8), and the chunk edits no nextest config or mutation run
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=no
why: a dev-host cognitive-complexity audit ceiling that no CI leg runs and that the entity says is no ceiling on every function; the task names no complexity measure for the chunk's functions
