# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · wpt.yml is not in the reached files and no CI step changed, so its upstream-only archive and dispatch claim still holds
ci-integration#1.2 · no · wptscores.json comes from the WPT run in wpt.yml, which the session did not reach, and no CI step changed
ci-integration#1.3 · no · ci-leg.sh and ci.yml are not reached and no CI step changed, and the new trace test's failures name only a kind and a count, so the no-user-data claim holds
ci-integration#1.4 · no · the coverage leg's script, its keyboard_compat.rs exclusion and its artifact upload are untouched, and the row states no count the session moved
ci-integration#1.5 · no · the agent-run state area and its events are untouched (only test counts moved, which the row does not state) and scripts/agent-run.sh is not reached
ci-integration#1.6 · yes · stale · the session moved its cited host_log.rs:303 (shifted by lines 134-278 of the new fourth test) and common/mod.rs:17-25 (shifted by lines 6-7 added), and its list of session-starting checks lacks the new cells and load-page sessions
ci-integration#1.7 · yes · add-to · F9 adds target/cold-agent/record/, where the live server child keeps its checks, local state that no CI leg uploads and cold-agent cleanup removes, so the row's location and never-removed clauses must name it
ci-integration#1.8 · yes · add-to · F9 adds target/cold-agent/record/, owner-only and removed by cleanup, to the state this row's list omits, and its cold-agent.sh citations sit in a reached file that grew by 127 lines
ci-integration#1.9 · yes · add-to · F9 adds that no kept check of the live run is committed, so the two committed Counter checks stay the only committed ones, which this row's list must carry
