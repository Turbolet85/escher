@answer n=1 entity=ci-integration#1.1 v=no
why: WPT report archive and dispatch run in wpt.yml under upstream-only guards; the chunk reaches no WPT path and adds no WPT figure.
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed and published by the upstream WPT workflow; the chunk computes and publishes no score.
@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg merged log is written and uploaded by ci-leg.sh and ci.yml, neither reached; the probe's own output stays bound by the silence, masking and failure-message rules (needs 1 and 2, S1, S4, S5), which it already honours.
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage lcov counts public source line by line; the probe adds test and example code only, no public source, and reaches no coverage floor.
@answer n=5 entity=ci-integration#1.5 v=no
why: target/agent-run state belongs to agent-run.sh and its contract; the probe runs through no agent-run.sh path and writes no state there.
@answer n=6 entity=ci-integration#1.6 v=no
why: the session state directory is created by host.rs for host sessions; the probe boots seven_guis::stand in process (needs 3), so it creates no session directory and writes nothing into one.
@answer n=7 entity=ci-integration#1.7 v=no
why: the recording directory holds checks written by the session recorder; the probe uses no recorder, writes no check and keeps none at rest.
@answer n=8 entity=ci-integration#1.8 v=no
why: cold-agent pipe state belongs to cold-agent.sh and its shim contract tests; the probe reaches none of it.
@answer n=9 entity=ci-integration#1.9 v=no
why: the cold-agent recording directory is written by the pipe's live run; the probe runs no cold-agent pipe and records nothing.
@answer n=10 entity=ci-integration#1.10 v=no
why: the committed Counter checks are read by stand_replay and seven_guis' freshness check; the probe commits no check, replays none and changes no Counter UI.
