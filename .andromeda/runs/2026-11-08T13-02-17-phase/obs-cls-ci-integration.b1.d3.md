@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT report archived to Pages and dispatched upstream is a WPT publishing rule; the chunk's read and checks touch no WPT output
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed and published upstream only; the chunk adds no score
@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg CI log is ci-leg.sh's merged leg output, a CI-file rule; the chunk adds no CI job or leg output, and the new stand checks print nothing
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage lcov upload is a CI artifact rule; the chunk adds no coverage floor or exclusion
@answer n=5 entity=ci-integration#1.5 v=no
why: the agent-run harness state is written by agent-run.sh, which the chunk does not call or change
@answer n=6 entity=ci-integration#1.6 v=yes
why: the session directory holds the socket file alone and no screen content; the new socket-carried answer must not be written there, and host_log (need 9) reads this directory
@answer n=7 entity=ci-integration#1.7 v=yes
why: recording.rs writes each answered call's accepted flag, digest and answer whole into check-{n}.json here, so the new answer's content and the recording bounds bind the chunk
@answer n=8 entity=ci-integration#1.8 v=no
why: the cold-agent pipe state and its cleanup belong to cold-agent.sh, which the chunk does not change; the tool_call span name it counts is unchanged (need 2, standing S3)
@answer n=9 entity=ci-integration#1.9 v=yes
why: the new answer is kept whole in recorded checks, so the rule that these kept checks are never printed or copied and that no live-run check is committed binds the chunk's answer writes and any new committed fixture
@answer n=10 entity=ci-integration#1.10 v=yes
why: stand_replay and seven_guis' freshness check read the two committed Counter checks through replay.rs, which the chunk reaches; the new answer must not change their replay and no further check may be committed
