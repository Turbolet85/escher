@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: a historical workspace line-coverage reading with no threshold applied, which the chunk neither re-measures nor is bound by.
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=yes
why: the escher-driver 97 % and dioxus-native-dom 83 % per-crate floors, which the new code must stay inside (task need 12).
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=no
why: a per-file reading of mutation_buffer.rs, a file the chunk does not reach; the crate floor it counts under is already carried by #2.1.
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes
why: the stand-repeat leg's zero-failure budget over the stand_ files, three rounds with no retry, which a new stand check joins (task need 14).
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the mutation tier runs on the dev host only and the scope asks no mutation run (task item 7, not_needed).
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=no
why: the complexity ceiling is a dev-host audit reading with no CI leg and no ceiling on every function, and the scope does not name complexity.
