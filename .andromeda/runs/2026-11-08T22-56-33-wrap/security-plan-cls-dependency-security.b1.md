@answer n=1 entity=dependency-security#1.0 v=no
why: the session adds no dependency and leaves Cargo.toml, Cargo.lock and deny.toml byte-identical, so the audit graph and its ignores stand
@answer n=2 entity=dependency-security#2.1 v=no
why: the session changes no dependency and no CI audit leg, so the advisory-after-green-audit reading still holds
@answer n=3 entity=dependency-security#3.1 v=no
why: a record of the 2026-10-10 upstream sync whose lock and manifest the session leaves unchanged
@answer n=4 entity=dependency-security#3.2 v=no
why: a record of the 2026-10-13 upstream sync whose workspace version and pins the session does not move
@answer n=5 entity=dependency-security#3.3 v=no
why: a record of the 2026-10-17 upstream sync whose lock set and zbus-family ignores the session does not change
@answer n=6 entity=dependency-security#3.4 v=no
why: a record of the 2026-10-18 cherry-pick whose Cargo.toml, Cargo.lock and deny.toml the session leaves byte-identical
@answer n=7 entity=dependency-security#3.5 v=no
why: a record of the 2026-10-19 cherry-pick whose dependency set and audit reading the session does not move
@answer n=8 entity=dependency-security#3.6 v=no
why: a record of the upstream sync at f9cbb2756 whose pins and lock package count the session leaves unchanged
@answer n=9 entity=dependency-security#3.7 v=no
why: a record of the upstream sync at ebacd68bd whose manifests and lock the session does not touch
@answer n=10 entity=dependency-security#3.8 v=no
why: a record of the upstream sync at e42ee1f25 whose pins and manifests the session leaves byte-identical
@answer n=11 entity=dependency-security#4.1 v=yes mode=add-to
why: the coverage role adds a second reader of argv-named kept-check files through Check::from_bytes, serde_json still writing nothing and no dependency added
@answer n=12 entity=dependency-security#5.1 v=no
why: the png dependency and its escher-driver and blitz-tests uses are unchanged, the session adding no png reader or manifest edit
@answer n=13 entity=dependency-security#6.1 v=no
why: blitz-test-harness and its color dependency are not in the session's reached files and blitz-tests gains no dependency
@answer n=14 entity=dependency-security#7.1 v=no
why: deny.toml and its licence table are unchanged by the session, so the licence audit reading holds
@answer n=15 entity=dependency-security#7.2 v=no
why: the CI free-disk action step in ci.yml is not in the session's reached files and ci.yml is unchanged
@answer n=16 entity=dependency-security#7.3 v=no
why: test_ci_workflows.py and the matrix_test job are untouched, so the pinned free-disk step reading holds
@answer n=17 entity=dependency-security#7.4 v=no
why: ci-leg.sh and the embolden leg it runs are not reached, the session's tests being other test binaries
