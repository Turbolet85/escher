@answer n=1 entity=telemetry-strategy#1.1 v=yes
why: the feature wiring (engine tracing off in a -p seven_guis build, on in a workspace build) and escher-telemetry's panic events in both native binaries bind the same-verdicts-under-both-builds need and the MountedData panic capture need
@answer n=2 entity=telemetry-strategy#2.1 v=yes
why: a workspace build turns blitz-dom tracing on while cargo test -p blitz-tests leaves it off, and the probe must give the same verdicts under both builds
@answer n=3 entity=telemetry-strategy#3.1 v=yes
why: the scope adds no crate, so the absence of telemetry backends over Cargo.toml and the workflows is a rule the probe must keep holding
@answer n=4 entity=telemetry-strategy#3.2 v=yes
why: need 11 keeps probe code under examples/seven_guis free of tracing calls, the same rule this absence records over the examples slice
@answer n=5 entity=telemetry-strategy#3.3 v=yes
why: the new probe check sits under tests/blitz-tests, so the test-crate slice census (no subscriber, env read, print, tracing) gains a file it must still read 0 over
@answer n=6 entity=telemetry-strategy#3.4 v=no
why: the wpt runner's 12 files are not reached by the chunk, which adds no span, crate or log site to them
