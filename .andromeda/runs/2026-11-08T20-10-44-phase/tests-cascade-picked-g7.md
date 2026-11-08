@account n=1 entity=quality-gates-coverage-targets#2.1 v=carried
why: the chunk adds code and tests to packages/dioxus-native-dom (and to packages/escher-driver if the command is a verb), whose per-crate floors 83 % and 97 % the coverage leg must keep
@account n=2 entity=quality-gates-coverage-targets#3.1 v=not_in_scope
why: the reading of mutation_buffer.rs, a file the chunk neither edits nor re-measures; the crate floor it counts toward is a re-check every change owes beyond that file
@account n=3 entity=quality-gates-coverage-targets#4.1 v=carried
why: the chunk's new stand checks (the coverage check, the owed actions replay test) join the stand-repeat leg's zero-failure budget, and its stated count of stand_*.rs files moves with a new file
