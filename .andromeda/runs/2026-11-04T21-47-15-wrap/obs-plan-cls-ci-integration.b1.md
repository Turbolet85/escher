@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT archive is produced by wpt.yml, which the session left unchanged, and the reached Cargo.lock and deny.toml feed no WPT report or Pages publication
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed by the WPT run in wpt.yml, which the session did not touch
@answer n=3 entity=ci-integration#1.3 v=no
why: ci-leg.sh and ci.yml are unchanged, so the per-leg log path and its upload only on failure still hold, and the audit leg's log behaves as before
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage leg's lcov covers the public source only, and the new test target is test code under tests/, so the lcov file and its keyboard_compat.rs exclusion stand
@answer n=5 entity=ci-integration#1.5 v=no
why: agent-run.sh and its target/agent-run paths are unchanged, and the new target only adds result lines to the existing events without changing what this entity names
@answer n=6 entity=ci-integration#1.6 v=no
why: the probe drives blitz_test_harness::Harness in process and starts no session host, so it creates no target/tmp session directory and joins none of the listed checks
@answer n=7 entity=ci-integration#1.7 v=no
why: the probe records no session, so it creates no recording directory and writes no check-{n}.json file
@answer n=8 entity=ci-integration#1.8 v=no
why: scripts/cold-agent.sh and its target/cold-agent state are untouched, and a blitz-tests target that never runs the cold-agent pipe writes none of it
@answer n=9 entity=ci-integration#1.9 v=no
why: the cold-agent recording directory and its live run are unchanged, and the probe writes no check into them
@answer n=10 entity=ci-integration#1.10 v=no
why: the two committed Counter checks under tests/blitz-tests/checks are untouched, and the probe commits no new check, so the only-committed-ones claim still holds
