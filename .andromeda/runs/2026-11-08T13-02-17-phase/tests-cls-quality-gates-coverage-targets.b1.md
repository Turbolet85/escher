@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: the first workspace-wide line reading with no threshold applied; the chunk is held by the per-crate floors, not this figure
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=yes
why: the escher-driver 97 % and dioxus-native-dom 83 % floors are what the new code must stay inside (task item 12), and the #[path] test-file note bears on where the read's tests sit
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=yes
why: the dioxus-native-dom floor re-measure reads mutation_buffer.rs as one input, counted with no exclusion, so the crate reading the chunk must hold includes it
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes
why: the stand-repeat leg's zero-failure three-round budget over the stand_ files is what a new stand check joins (task item 14)
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the mutation tier runs on the dev host alone and the scope asks no mutation run (task item 7, not needed)
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=no
why: the dev-host cognitive-complexity audit has no CI leg and the chunk names no complexity ceiling, so the read's new functions are not held to it
