@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: a historical first line-coverage reading with no threshold applied, which the chunk neither states nor re-measures
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=yes
why: names the three per-crate floors (escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90) and the keyboard_compat.rs exclusion that the chunk's new reader and registry code must count under
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=yes
why: states that dioxus-native-dom's 83 floor counts crate-private code with no exclusion, the rule the new reader and registry code in that crate fall under
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes
why: the stand-repeat leg's zero-failure budget over the stand_*.rs files and the seventeen named targets, which the chunk's new stand checks and named-list entries join
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the mutation tier is a dev-host run never in CI, and the task marks it not_needed (item 9)
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=no
why: the dev-host cognitive-complexity audit has no CI leg and no ceiling on every function, and the task names no complexity bound
