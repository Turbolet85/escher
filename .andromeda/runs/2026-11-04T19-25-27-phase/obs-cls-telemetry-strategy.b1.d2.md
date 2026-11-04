@answer n=1 entity=telemetry-strategy#1.1 v=yes
why: names which crates compile the span and panic-event macros into seven_guis_native and escher-session, which the MountedData panic reading (item 7) and the untouched-span rule (item 6) depend on
@answer n=2 entity=telemetry-strategy#2.1 v=yes
why: blitz-dom tracing is on under a workspace build and off under cargo test -p blitz-tests, the split the probe must give the same verdicts across (item 4)
@answer n=3 entity=telemetry-strategy#3.1 v=no
why: observed absence of metrics and telemetry backends in workflows and Cargo manifests, and the probe adds no crate, timing or counter, so nothing it touches moves this record
@answer n=4 entity=telemetry-strategy#3.2 v=yes
why: examples/seven_guis holds no tracing, log or env_logger use, and probe code kept there must keep it so (needs item 11)
@answer n=5 entity=telemetry-strategy#3.3 v=no standing=S1
why: the silence census over the test crate slice restates the silent-check convention (no subscriber, no env read, no print), a convention the task lists
@answer n=6 entity=telemetry-strategy#3.4 v=no
why: absence over the wpt runner slice, and the wpt runner is out of scope (not-needed item 5) and the probe does not touch it
