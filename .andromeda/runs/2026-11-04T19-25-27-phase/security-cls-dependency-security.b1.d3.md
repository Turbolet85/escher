@answer n=1 entity=dependency-security#1.0 v=yes
why: the cargo-deny audit config, its eight ignores and the six-target graph are the audit leg the probe's build must keep green over the resolved graph (task item 7)
@answer n=2 entity=dependency-security#2.1 v=yes
why: the freeze reading in it (audit ok on the merged lock, no ignore added or dropped) is the one item 7 names; its six lock names are closed 2026-10-10 history the probe does not use
@answer n=3 entity=dependency-security#2.2 v=yes
why: the freeze reading (no other third-party package enters, leaves or moves; deny.toml unchanged) binds the probe's blitz-tests dev-dependencies, which must add no lock package
@answer n=4 entity=dependency-security#2.3 v=yes
why: Dioxus 0.7.3 keeps keyboard-types 0.7 while the lock holds 0.8.3, so a probe key handler must take the key event type Dioxus carries; the Dioxus pin and the no-lock-change freeze are item 7
@answer n=5 entity=dependency-security#2.4 v=yes
why: the freeze (Cargo.toml, Cargo.lock and deny.toml unchanged, no git dependency, Taffy registry 0.15) is the one item 7 names for this chunk
@answer n=6 entity=dependency-security#2.5 v=yes
why: the same freeze (no dependency enters, leaves or moves; deny.toml unchanged; no git dependency) that item 7 names
@answer n=7 entity=dependency-security#2.6 v=yes
why: the freeze plus the pins it records as staying (Dioxus 0.7.3, Stylo 0.22.0, Taffy 0.15) with no git or patch dependency, which item 7 names for Dioxus
@answer n=8 entity=dependency-security#2.7 v=yes
why: the freeze plus the pins that stay (Dioxus 0.7.3, winit =0.31.0-beta.3), which item 7 names for Dioxus
@answer n=9 entity=dependency-security#2.8 v=yes
why: the most recent record of the freeze and the pins that stay (Dioxus 0.7.3, winit =0.31.0-beta.3, no git dependency), which item 7 names
@answer n=10 entity=dependency-security#3.1 v=yes
why: the probe reads the driver's JSON answers through serde_json, which it must inherit from the root entry with no feature and no preserve_order (task item 9); the lock stays frozen
@answer n=11 entity=dependency-security#4.1 v=yes
why: png is a dev-dependency of blitz-tests, the crate the probe's tests join, so the no-lock-change and audit-green freeze binds; the shot encoder itself is not used
@answer n=12 entity=dependency-security#5.1 v=yes
why: blitz-test-harness is the stand harness the probe runs on (task item 8), so its dependency set and the no-lock-change freeze bind; the contrast reader's color source is not used
@answer n=13 entity=dependency-security#6.1 v=yes
why: the licence table is half of the cargo-deny audit leg that item 7 names as staying green over the resolved graph
@answer n=14 entity=dependency-security#6.2 v=no
why: the free-disk action of the Stand checks repeated CI job; the probe adds no CI job or step and the scope names no CI change
@answer n=15 entity=dependency-security#6.3 v=no
why: the matrix_test job holds no free-disk step and the probe adds no CI job, so the record reaches nothing the chunk changes
@answer n=16 entity=dependency-security#6.4 v=yes
why: the embolden leg is a --locked cargo leg on blitz-tests, the crate the probe adds tests to, so the probe's dependencies there must keep it green (item 7)
