@answer n=1 entity=telemetry-strategy#1.1 v=yes
why: the entity fixes where engine tracing features stay off in a package-alone build and on in a workspace build, which the probe must give the same verdicts under (task item 4), and where escher-telemetry panic events compile into seven_guis_native and escher-session (task item 7)
@answer n=2 entity=telemetry-strategy#2.1 v=yes
why: a workspace build turns blitz-dom tracing on, so a sink-installing child receives engine INFO events the probe must not depend on (task item 4)
@answer n=3 entity=telemetry-strategy#3.1 v=yes
why: the probe adds no crate (task scope), so the record that no telemetry backend appears in Cargo.toml, Cross.toml, flake.nix and the workflows must keep holding
@answer n=4 entity=telemetry-strategy#3.2 v=yes
why: task item 11 keeps any probe code under examples/seven_guis free of tracing calls and other telemetry uses, which this absence over the examples slice records
@answer n=5 entity=telemetry-strategy#3.3 v=yes
why: the probe check is a new file in tests/blitz-tests (task item 1), so the absence of logging and subscriber use over the test crate slice and its zero count must keep holding over it
@answer n=6 entity=telemetry-strategy#3.4 v=no
why: the wpt runner slice is not reached; the probe touches no wpt runner file (task item not_needed 5)
