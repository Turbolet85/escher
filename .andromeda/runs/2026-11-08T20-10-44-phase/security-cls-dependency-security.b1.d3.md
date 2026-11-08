@answer n=1 entity=dependency-security#1.0 v=no
why: cargo-deny advisory and licence audit over the lock and deny.toml; the chunk edits no manifest, lock or deny.toml and adds no dependency, so the audit leg re-reads it on every push.
@answer n=2 entity=dependency-security#2.1 v=no
why: a CI audit going red on newly published advisories and the local ci-leg.sh audit run before a push, a process rule every change owes; the chunk adds no dependency.
@answer n=3 entity=dependency-security#3.1 v=no
why: record of the 2026-10-10 upstream sync's dependency set; history only, the chunk moves no lock or manifest entry.
@answer n=4 entity=dependency-security#3.2 v=no
why: record of the 2026-10-13 upstream sync (taffy 0.15, version pins); history only, the chunk moves no pin.
@answer n=5 entity=dependency-security#3.3 v=no
why: record of the 2026-10-17 upstream sync and its zbus-family ignores; history only, the chunk moves no lock entry or ignore.
@answer n=6 entity=dependency-security#3.4 v=no
why: record of the 2026-10-18 cherry-pick, no dependency moved; history only, the chunk takes no upstream text.
@answer n=7 entity=dependency-security#3.5 v=no
why: record of the 2026-10-19 cherry-pick, no dependency moved; history only, the chunk takes no upstream text.
@answer n=8 entity=dependency-security#3.6 v=no
why: record of the 2026-10-24 sync (tokio 1.54.1, mimalloc, pins held); the chunk moves no lock and its reached files use neither tokio nor mimalloc.
@answer n=9 entity=dependency-security#3.7 v=no
why: record of the 2026-10-27 sync with the Stylo, Taffy, Dioxus and winit pins held; the chunk changes no manifest, lock or pin.
@answer n=10 entity=dependency-security#3.8 v=no
why: record of the 2026-11-01 sync with the same pins held; the chunk changes no manifest, lock or pin.
@answer n=11 entity=dependency-security#4.1 v=yes
why: Check::from_bytes reads a kept check through serde_json and the new actions replay test asserts answer equality through it, so the no-preserve_order reader that writes nothing binds the chunk's code (task item 15).
@answer n=12 entity=dependency-security#5.1 v=no
why: the png encoder of a screenshot and the hand-written base64 of a shot; the chunk renders no screenshot and touches no png path.
@answer n=13 entity=dependency-security#6.1 v=no
why: the color luminance source of blitz-test-harness's contrast reader; that crate is not reached and the chunk's tests take no contrast reader.
@answer n=14 entity=dependency-security#7.1 v=no
why: the deny.toml licence table and the licence gate re-read on every push; the chunk adds no dependency and edits no deny.toml.
@answer n=15 entity=dependency-security#7.2 v=no
why: the free-disk-space step of the Stand checks repeated CI job; the chunk changes no ci.yml job or step.
@answer n=16 entity=dependency-security#7.3 v=no
why: the matrix_test job holds no free-disk step and test_ci_workflows.py pins it; the chunk touches neither ci.yml nor that test.
@answer n=17 entity=dependency-security#7.4 v=no standing=S4
why: a locked cargo leg in ci-leg.sh, the --locked CI cargo convention the standing names; the chunk adds no CI cargo command.
