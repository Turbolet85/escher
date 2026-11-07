@answer n=1 entity=ci-integration#1.1 v=no
why: WPT report archiving and dispatch run only on upstream through wpt.yml, a CI job the chunk does not touch, and the WPT runner is task not_needed item 6.

@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed and published by the upstream-only wpt.yml job, which the chunk neither changes nor reads.

@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg CI log is CI artifact handling (ci-leg.sh and ci.yml upload), which task not_needed item 3 leaves untouched; the probe failure-message rule that keeps this log free of page text is carried by task item 7, not by this row.

@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage lcov file and its artifact upload are CI artifact handling the chunk adds no artifact to, and the row states no coverage floor the new render code must meet.

@answer n=5 entity=ci-integration#1.5 v=no
why: the agent-run state under target/agent-run is written by scripts/agent-run.sh, which the chunk leaves untouched (task not_needed item 1).

@answer n=6 entity=ci-integration#1.6 v=no
why: the session state directory belongs to the session library and its named checks (ss-*, hb-*, hl-*, cc-*, cf-*, ht-timer), none of which the chunk's probe or stand_* witnesses name, and the chunk changes no driver module.

@answer n=7 entity=ci-integration#1.7 v=no
why: the recording directory is written by the escher-driver recorder, which the chunk does not change, and no test in the scope names a --record launch.

@answer n=8 entity=ci-integration#1.8 v=no
why: the cold-agent pipe state is written by scripts/cold-agent.sh, which the chunk does not touch (task not_needed item 2).

@answer n=9 entity=ci-integration#1.9 v=no
why: the cold-agent recording is the live pipe's own escher-session mcp --record child, which the chunk does not touch (task not_needed item 2).

@answer n=10 entity=ci-integration#1.10 v=yes
why: the committed checks counter-cli.json and counter-mcp.json hold the Counter's ids, values and snapshot text, and stand_replay reads them; the chunk re-asserts the stand_* tests through this render path as its witness, so the checks are what the witness reads.
