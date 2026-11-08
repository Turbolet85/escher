@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT report archive is set by wpt.yml, which the session does not reach
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed and published by wpt.yml, which the session does not reach
@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg CI log is written by ci-leg.sh and ci.yml, neither reached, and no CI script changed
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage lcov is written by ci-leg.sh and ci.yml, neither reached, and the session adds no metric or exclusion
@answer n=5 entity=ci-integration#1.5 v=no
why: the agent-run state is written by scripts/agent-run.sh, which the session does not reach, and only a run's tally moved, not its events or fields
@answer n=6 entity=ci-integration#1.6 v=yes mode=stale
why: its cited lines into session_common, the seven_guis common module and host_log.rs are in reached files that gained lines, so the citations can be off
@answer n=7 entity=ci-integration#1.7 v=yes mode=add-to
why: a recorded check now keeps graph answers whole, carrying node ids, component and state names and handler event names, which the at-rest answer list does not name
@answer n=8 entity=ci-integration#1.8 v=yes mode=stale
why: scripts/cold-agent.sh is reached (VERBS gains graph, fake server answers graph), so its cited line ranges can move, and the report does not show them holding
@answer n=9 entity=ci-integration#1.9 v=yes mode=stale
why: its citations into scripts/cold-agent.sh, a reached file whose VERBS and fake server changed, can be off, and the report does not show them holding
@answer n=10 entity=ci-integration#1.10 v=yes mode=stale
why: counter-cli.json and counter-mcp.json were re-recorded, so the line citations 1-8 and 1-11 into them are stale
