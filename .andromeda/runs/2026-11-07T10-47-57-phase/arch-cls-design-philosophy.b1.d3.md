@answer n=1 entity=design-philosophy#1.1 v=no
why: a values statement on separately usable crates and renderers; the chunk adds only a crate-private module inside dioxus-native-dom and moves no crate boundary, painter or dependency
@answer n=2 entity=design-philosophy#2.1 v=yes
why: states that DioxusDocument pushes UI events with handle_ui_event and flushes changes with poll, and the chunk changes poll and initial_build to buffer then replay
@answer n=3 entity=design-philosophy#3.1 v=yes
why: states interaction-state teardown on node removal matches WebKit/Blink semantics, and the scope requires the replayed buffer to keep removed-node teardown and the clearing or retargeting of interaction state
@answer n=4 entity=design-philosophy#4.1 v=yes
why: states incremental and non-incremental layout must match after any mutation sequence, which incremental_oracle re-runs as a witness, and that dropped node ids stop resolving and interaction state survives reconstruction, which the replay must keep
@answer n=5 entity=design-philosophy#5.1 v=no
why: a values statement on the headless harness, WPT runner and benchmarks; the chunk's tests run on the existing fixtures, and the scope names no harness behaviour or unit that this rule rules over
