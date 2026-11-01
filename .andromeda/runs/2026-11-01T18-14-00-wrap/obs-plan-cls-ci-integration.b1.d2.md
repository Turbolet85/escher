# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · wpt.yml and its upstream-only publication are not reached, and the session names no WPT workflow change
ci-integration#1.2 · no · wptscores.json is computed in wpt.yml, which the session does not reach
ci-integration#1.3 · yes · stale · ci-leg.sh gained lines 8-9 and 28-29 and ci.yml's matrix_test free-disk step left at 392-398, moving the cited ci-leg.sh:92-93 and ci.yml:414-423 citations
ci-integration#1.4 · yes · stale · ci-leg.sh gained lines 8-9 and 28-29 above the coverage write, moving the cited ci-leg.sh:42-49 citation; ci.yml:286-291 sits above the removed step and stands
ci-integration#1.5 · no · scripts/agent-run.sh is unedited, and the claim that no CI leg uploads this state still holds
ci-integration#1.6 · no · the session state sources (host.rs, session_common, seven_guis common, host_log.rs) are not reached, and the embolden leg has no CI job so no leg uploads state
ci-integration#1.7 · no · the recording sources (recording.rs, session.rs) are not reached and the session adds no upload
ci-integration#1.8 · no · the cold-agent scripts are not reached, and embolden has no CI job so CI still runs only the shim contract tests
ci-integration#1.9 · no · scripts/cold-agent.sh is not reached and no CI leg uploads its recording
ci-integration#1.10 · no · the two committed check files are not reached, and the new tests add no check read by stand_replay or the freshness check
