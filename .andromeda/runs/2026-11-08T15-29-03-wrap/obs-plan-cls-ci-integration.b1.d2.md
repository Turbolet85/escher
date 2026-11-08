@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT archive and dispatch in wpt.yml is in no reached file and the session changes no CI workflow
@answer n=2 entity=ci-integration#1.2 v=no
why: the wptscores.json publication in wpt.yml is untouched and the session adds no CI step or artifact
@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg log written by ci-leg.sh and ci.yml is unchanged and the session adds no CI leg
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage leg, its upload and its exclusion are unchanged and the new source files only add to the public source the leg already counts
@answer n=5 entity=ci-integration#1.5 v=no
why: agent-run.sh and its target/agent-run state are in no reached file and the session changes no harness event or field
@answer n=6 entity=ci-integration#1.6 v=yes mode=stale
why: its line citations into session_common/mod.rs, seven_guis common/mod.rs and host_log.rs point at reached files whose lines the session can have moved
@answer n=7 entity=ci-integration#1.7 v=yes mode=add-to
why: the actions answer is kept whole in a recorded check, so its ids, owner ids, verb words and closed cause words join the at-rest content
@answer n=8 entity=ci-integration#1.8 v=yes mode=stale
why: scripts/cold-agent.sh is reached by the verb-list change, so its cited lines 17, 555-588 and 593-596 can have moved
@answer n=9 entity=ci-integration#1.9 v=yes mode=stale
why: scripts/cold-agent.sh is reached by the verb-list change, so its cited lines 257-271 and 570-577 can have moved
@answer n=10 entity=ci-integration#1.10 v=no
why: the two committed Counter checks are byte-identical by the session's own untouched item and no reached file originates them
