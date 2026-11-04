@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT archive and the dispatch to blitz-wpt-results are upstream steps in wpt.yml; the probe is test or example code and changes no workflow.

@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed and published by wpt.yml upstream only, which the chunk does not touch.

@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg log written by ci-leg.sh is CI's concern; the probe prints nothing (task item 1) and adds no CI leg.

@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage lcov artifact and its keyboard_compat.rs exclusion belong to the coverage leg; the probe adds no metric or exclusion.

@answer n=5 entity=ci-integration#1.5 v=no
why: target/agent-run state is written by agent-run.sh, which the probe neither runs nor edits.

@answer n=6 entity=ci-integration#1.6 v=no
why: session state directories are created by the session host for its callers; the probe boots the stand in process (task item 3) and names no session directory.

@answer n=7 entity=ci-integration#1.7 v=no
why: recording directories hold recorded session calls; the probe records nothing and passes no --record or recording_into.

@answer n=8 entity=ci-integration#1.8 v=no
why: cold-agent pipe state is written by scripts/cold-agent.sh, which the chunk neither runs nor edits.

@answer n=9 entity=ci-integration#1.9 v=no
why: the cold-agent recording directory belongs to the pipe's live run, and the probe adds no recording.

@answer n=10 entity=ci-integration#1.10 v=no
why: the committed Counter checks are read by stand_replay and the seven_guis freshness check, which the probe neither edits nor re-pins, and its Counter run writes nothing into them.
