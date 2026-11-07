@answer n=1 entity=ci-integration#1.1 v=no
why: WPT report archiving and dispatch are upstream-only CI artifact handling, which the chunk adds no job or artifact to (not_needed 3).
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed and published by the WPT workflow, outside the render-path change.
@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg CI log is CI artifact storage the chunk does not touch (not_needed 3); no check of the chunk reads it.
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage lcov artifact is CI artifact handling and the chunk adds no counter or artifact (not_needed 3, 4).
@answer n=5 entity=ci-integration#1.5 v=no
why: agent-run harness state belongs to scripts/agent-run.sh, which the chunk leaves untouched (not_needed 1).
@answer n=6 entity=ci-integration#1.6 v=yes
why: host_log.rs:419 is a source of this rule and host_log is re-asserted under both builds, so the state directory's no-log-or-event-content rule is witnessed by the chunk's work.
@answer n=7 entity=ci-integration#1.7 v=no
why: the recording directory belongs to escher-driver's recording module and the cold-agent pipe, both outside the chunk (not_needed 2, 9).
@answer n=8 entity=ci-integration#1.8 v=no
why: cold-agent pipe state is not touched by the chunk (not_needed 2).
@answer n=9 entity=ci-integration#1.9 v=no
why: cold-agent recording is the pipe's live-run state, which the chunk does not touch (not_needed 2).
@answer n=10 entity=ci-integration#1.10 v=yes
why: the committed counter-cli.json and counter-mcp.json hold the Counter values and snapshot text that stand_replay reads, and stand_replay is re-asserted by the chunk as its witness.
