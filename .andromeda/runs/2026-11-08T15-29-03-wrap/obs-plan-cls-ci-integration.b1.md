@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT archive and its repository guards live in wpt.yml, which the session did not touch and CI keeps its shape
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed by the wpt workflow, which the session did not touch
@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg CI log written by ci-leg.sh and ci.yml is unchanged, as CI keeps its steps and artifacts
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage leg's lcov file and its filename exclusion are unchanged, as CI keeps its steps and artifacts
@answer n=5 entity=ci-integration#1.5 v=no
why: the agent-run harness events, fields and state area are unchanged, and the stand tally moving is not an event-shape change
@answer n=6 entity=ci-integration#1.6 v=yes mode=stale
why: its cited lines in session_common/mod.rs, seven_guis common/mod.rs and host_log.rs sit in files the session reached, so the citations may have moved with the new reader, needles and hosted actions call
@answer n=7 entity=ci-integration#1.7 v=yes mode=add-to
why: a kept recorded check now holds the actions answer whole, with its ids, owner ids, verb words, closed cause words and booleans, which the at-rest list does not yet name
@answer n=8 entity=ci-integration#1.8 v=yes mode=stale
why: the cold-agent pipe's verb list joins actions in scripts/cold-agent.sh, a reached file whose cited lines 17, 555-588 and 593-596 may have moved
@answer n=9 entity=ci-integration#1.9 v=yes mode=stale
why: scripts/cold-agent.sh is a reached file and the cited lines 257-271 and 570-577 may have moved with the verb-list edit
@answer n=10 entity=ci-integration#1.10 v=no
why: the two committed Counter checks stay byte-identical and their readers are not changed by the session
