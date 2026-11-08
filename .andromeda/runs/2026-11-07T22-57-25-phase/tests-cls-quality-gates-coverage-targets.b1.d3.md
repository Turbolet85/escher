@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: a historical line-coverage reading with no threshold, which no code or test of the chunk must honour
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=yes
why: the per-crate floors (dioxus-native-dom 83 %, escher-driver 97 %, blitz-test-harness 90 %) and the keyboard_compat.rs exclusion are the coverage bar the new reader and registry code must clear
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=yes
why: it records that a dioxus-native-dom file counts inside the 83 % floor with no exclusion, the counting the new reader and registry code falls under
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes
why: the stand-repeat leg's zero-failure budget over the stand_*.rs files and the named target list binds the stand checks the chunk changes and any new non-stand_ target that must repeat
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the mutation tier is a dev-host run never in CI and the task lists it as not needed, so no code or test of the chunk must honour it
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=no
why: a dev-host cognitive-complexity reading that no CI leg runs and that sets no ceiling on every function, and the scope names no function under it
