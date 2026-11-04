@answer n=1 entity=inherited-defaults#1.1 v=no
why: publish flags and workspace versioning govern crates the chunk neither adds nor versions (task item 16 adds no workspace crate, and the probe lives in existing crates)
@answer n=2 entity=inherited-defaults#2.1 v=yes
why: task item 18 names the probe's CI gates (test leg, stand-repeat zero-failure budget, clippy, fmt, rustdoc, per-crate coverage floors), which this entity states
@answer n=3 entity=inherited-defaults#3.1 v=yes
why: the stand's no-telemetry boot and the read-only snapshot check under the `accessibility` feature (task items 5 and 17) depend on the Cargo-feature gating and ungated telemetry and driver spans this entity states, so the probe must avoid calls that enable them
