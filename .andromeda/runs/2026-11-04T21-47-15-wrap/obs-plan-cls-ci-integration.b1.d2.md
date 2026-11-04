@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT archive row is produced by wpt.yml, which the session does not change or reach
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed and published by wpt.yml, which the session does not change or reach
@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg log is written by ci-leg.sh and ci.yml, neither changed, and the new target only adds libtest lines to a leg's merged output
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage leg's lcov scope is the workspace's public source under packages and examples, which the session leaves unchanged
@answer n=5 entity=ci-integration#1.5 v=no
why: the agent-run status, events and run.log paths and their raw-output shape are unchanged, and the session adds no counts to the entity's claim
@answer n=6 entity=ci-integration#1.6 v=no
why: the session state directory is created by the escher-driver host for the listed checks, and the new target drives blitz_test_harness::Harness in process with no socket
@answer n=7 entity=ci-integration#1.7 v=no
why: the recording directory is written only by the escher-driver recorder for the listed checks, and the new target records nothing
@answer n=8 entity=ci-integration#1.8 v=no
why: the cold-agent pipe state paths and scripts/cold-agent.sh are unchanged, and the session runs no cold-agent check
@answer n=9 entity=ci-integration#1.9 v=no
why: the cold-agent recording directory is written by the pipe's live run, which the session neither runs nor changes
@answer n=10 entity=ci-integration#1.10 v=no
why: the two committed Counter checks and record_checks.sh are unchanged, and the stand run stays at 237 over 51 files, so stand_replay reads still hold
