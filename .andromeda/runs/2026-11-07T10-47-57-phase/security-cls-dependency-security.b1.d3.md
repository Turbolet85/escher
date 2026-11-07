@answer n=1 entity=dependency-security#1.0 v=no standing=S2
why: the cargo-deny --locked audit tool and its deny.toml config are the S2 convention; the chunk adds no package and no ignore
@answer n=2 entity=dependency-security#2.1 v=yes
why: names bash ci-leg.sh audit as the local witness before a push, since the fast leg runs no audit, which the scope's audit-pass obligation (n=8) needs
@answer n=3 entity=dependency-security#3.1 v=no standing=S2
why: 2026-10-10 sync record of lock entries reviewed by hand and an audit reading, the S2 lock-review convention; the chunk enters no package
@answer n=4 entity=dependency-security#3.2 v=no standing=S2
why: 2026-10-13 sync record of the lock set and audit reading, the S2 convention; the chunk changes no package
@answer n=5 entity=dependency-security#3.3 v=no standing=S2
why: 2026-10-17 sync record of the lock set and audit reading, the S2 convention; its keyboard-types fact is not a rule over the chunk's render code
@answer n=6 entity=dependency-security#3.4 v=no standing=S2
why: 2026-10-18 cherry-pick record with its audit reading, the S2 convention; the Taffy state it records is not a rule over the chunk
@answer n=7 entity=dependency-security#3.5 v=no standing=S2
why: 2026-10-19 cherry-pick record with its audit reading, the S2 convention; the chunk changes no dependency
@answer n=8 entity=dependency-security#3.6 v=yes
why: states that Dioxus stays at 0.7.3 and that upstream's Dioxus 0.8.0-alpha.3 (286874cc1) is not taken, the pin scope n=8 names
@answer n=9 entity=dependency-security#3.7 v=yes
why: its pins line keeps Dioxus at 0.7.3, the pin scope n=8 names
@answer n=10 entity=dependency-security#3.8 v=yes
why: its pins line keeps Dioxus at 0.7.3, the pin scope n=8 names
@answer n=11 entity=dependency-security#4.1 v=no standing=S2
why: its audit-green and lock lines are the S2 convention; the serde_json readers sit in escher-driver, which the chunk does not touch
@answer n=12 entity=dependency-security#5.1 v=no standing=S2
why: its audit-green and lock lines are the S2 convention; the png dev-dependency record changes nothing the chunk adds
@answer n=13 entity=dependency-security#6.1 v=no standing=S2
why: its audit-reading line is the S2 convention; the color record concerns blitz-test-harness, which the chunk does not change
@answer n=14 entity=dependency-security#7.1 v=no standing=S2
why: the licence table the cargo-deny audit leg reads is the S2 audit leg; the chunk adds no package, so no licence reading changes
@answer n=15 entity=dependency-security#7.2 v=no
why: the CI free-disk action in ci.yml, and the chunk edits no workflow (not_needed n=10)
@answer n=16 entity=dependency-security#7.3 v=no
why: the matrix_test free-disk step and its workflow pin, and the chunk edits no workflow or CI job
@answer n=17 entity=dependency-security#7.4 v=no standing=S2
why: the embolden locked cargo leg is the --locked convention S2 names, and the chunk's new tests run under --locked
