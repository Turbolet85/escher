@answer n=1 entity=dependency-security#1.0 v=no
why: the cargo-deny audit gate reads Cargo.lock and deny.toml, which the chunk does not edit, so it is a re-check every push owes
@answer n=2 entity=dependency-security#2.1 v=no
why: the advisory-after-green CI red and the local ci-leg.sh audit reproduction are a push procedure, not a rule the chunk's code or tests must meet
@answer n=3 entity=dependency-security#3.1 v=no
why: a record of the 2026-10-10 upstream sync's dependency set, and the chunk moves no manifest or lock entry
@answer n=4 entity=dependency-security#3.2 v=no
why: a record of the 2026-10-13 upstream sync's taffy and pin set, and the chunk changes no pin
@answer n=5 entity=dependency-security#3.3 v=no
why: a record of the 2026-10-17 upstream sync's keyboard-types and zbus-family lock set, and the chunk changes no lock entry
@answer n=6 entity=dependency-security#3.4 v=no
why: a record of the 2026-10-18 cherry-pick that moved no dependency, and the chunk takes no upstream dependency
@answer n=7 entity=dependency-security#3.5 v=no
why: a record of the 2026-10-19 cherry-pick that moved no dependency, and nothing in it the chunk must meet
@answer n=8 entity=dependency-security#3.6 v=no
why: a record of the 2026-10-24 sync's lock moves and held pins, and the chunk changes no pin or lock edge
@answer n=9 entity=dependency-security#3.7 v=no
why: a record of the 2026-10-27 sync with every manifest unchanged, and the chunk edits no manifest
@answer n=10 entity=dependency-security#3.8 v=no
why: a record of the 2026-11-01 sync with the pins held, and the chunk changes no pin
@answer n=11 entity=dependency-security#4.1 v=yes
why: serde_json reads the MCP server's stdin, Check::from_bytes and the replay's from_str, which the chunk's mcp, recording.rs and replay.rs edits reach, and the new read's recorded step must fit that reader
@answer n=12 entity=dependency-security#5.1 v=no
why: the png encoder of a shot and blitz-tests' png dev-dependency are outside the read, which writes no image
@answer n=13 entity=dependency-security#6.1 v=no
why: the color crate feeds the contrast reader of blitz-test-harness, which the chunk neither uses nor edits
@answer n=14 entity=dependency-security#7.1 v=no
why: the licence gate over the lock, which the chunk leaves unchanged, is a re-check every push owes
@answer n=15 entity=dependency-security#7.2 v=no
why: the free-disk action step of the Stand checks repeated CI job, which the chunk does not edit
@answer n=16 entity=dependency-security#7.3 v=no
why: the matrix_test free-disk step and its pin test in test_ci_workflows.py, neither of which the chunk reaches
@answer n=17 entity=dependency-security#7.4 v=no
why: the embolden locked leg in ci-leg.sh is an existing leg the chunk neither edits nor adds to
