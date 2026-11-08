@answer n=1 entity=dependency-security#1.0 v=no
why: cargo-deny audit configuration and advisory ignores; the chunk adds no dependency and reaches neither deny.toml nor ci-leg.sh
@answer n=2 entity=dependency-security#2.1 v=no
why: a CI audit going red on newly published advisories; no reached unit runs or names the audit leg
@answer n=3 entity=dependency-security#3.1 v=no
why: a past upstream sync's lock-set record; the chunk moves no manifest or lock entry and states no pin it must meet
@answer n=4 entity=dependency-security#3.2 v=no
why: a past upstream sync's lock-set record for taffy and the workspace version; the chunk moves no dependency
@answer n=5 entity=dependency-security#3.3 v=no
why: a past upstream sync's keyboard-types and test-that record; the chunk moves no dependency
@answer n=6 entity=dependency-security#3.4 v=no
why: a past cherry-pick record with Cargo.toml, Cargo.lock and deny.toml unchanged; the chunk takes no upstream text
@answer n=7 entity=dependency-security#3.5 v=no
why: a past cherry-pick record with Cargo.toml, Cargo.lock and deny.toml unchanged; the chunk takes no upstream text
@answer n=8 entity=dependency-security#3.6 v=no
why: a past upstream sync's lock and pin record (Stylo, Taffy, Dioxus, tokio); the chunk changes no pin or lock entry
@answer n=9 entity=dependency-security#3.7 v=no
why: a past upstream sync's lock and pin record (Stylo, Taffy, Dioxus, winit); the chunk changes no pin or lock entry
@answer n=10 entity=dependency-security#3.8 v=no
why: a past upstream sync's lock and pin record (Stylo, Taffy, Dioxus, winit); the chunk changes no pin or lock entry
@answer n=11 entity=dependency-security#4.1 v=yes
why: serde_json without preserve_order reads a kept check's bytes through Check::from_bytes and the driver's answer line the actions replay compares, which the scope names
@answer n=12 entity=dependency-security#5.1 v=no
why: the png encoder of a screenshot and blitz-paint's features; no screenshot unit is in the chunk's reach and the chunk calls no png
@answer n=13 entity=dependency-security#6.1 v=no
why: the color crate in blitz-test-harness's contrast reader; blitz-test-harness is not in the chunk's reach
@answer n=14 entity=dependency-security#7.1 v=no
why: the deny.toml licence table and the audit leg's licence reading; the chunk adds no dependency and reaches no deny.toml
@answer n=15 entity=dependency-security#7.2 v=no standing=S4
why: a CI action's uses: pinned to a commit SHA, the ci.yml convention every change of its kind owes
@answer n=16 entity=dependency-security#7.3 v=no
why: the matrix_test job's absent free-disk step and test_ci_workflows.py's pin; neither file is in the chunk's reach
@answer n=17 entity=dependency-security#7.4 v=no standing=S4
why: a locked cargo CI leg passing --locked, the CI cargo command convention every change of its kind owes
