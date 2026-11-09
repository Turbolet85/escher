@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT archive and dispatch workflow is untouched, and no workflow or leg list changed
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json comes from the untouched WPT workflow, which the session does not reach
@answer n=3 entity=ci-integration#1.3 v=no
why: ci-leg.sh and ci.yml are untouched, and the new tests add output without changing where the leg log is written or what it holds
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage leg and its lcov artifact are untouched, and the claim names no file the new source changes
@answer n=5 entity=ci-integration#1.5 v=no
why: agent-run.sh and its state are untouched, and the new stand target is picked up by prefix only
@answer n=6 entity=ci-integration#1.6 v=yes mode=stale
why: reached host_log.rs gains a coverage test and reached seven_guis common/mod.rs gains coverage_needles, so the cited lines 512 and 17-25 and the check-prefix list may have moved
@answer n=7 entity=ci-integration#1.7 v=no
why: recording.rs and session.rs are not reached, and the coverage role only reads kept checks and writes no file, so the recording rows hold
@answer n=8 entity=ci-integration#1.8 v=no
why: the cold-agent pipe, its script and the MCP surface are untouched
@answer n=9 entity=ci-integration#1.9 v=no
why: cold-agent.sh is not reached, no kept check of the live run is committed, and the two committed checks are byte-identical
@answer n=10 entity=ci-integration#1.10 v=no
why: the two committed checks are byte-identical and their readers, stand_replay and the freshness check, still hold
