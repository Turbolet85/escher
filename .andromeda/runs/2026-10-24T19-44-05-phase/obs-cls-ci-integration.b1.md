# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT archive is in wpt.yml, which the chunk does not reach, and the task's Not needed names the WPT workflows
ci-integration#1.2 · no · wptscores.json comes from the WPT run in wpt.yml, which the chunk does not reach; WPT scores are Not needed
ci-integration#1.3 · yes · the a11y leg's merged log and its 7-day failure artifact, written by ci-leg.sh, which the widened leg's test list changes; the widened checks must stay silent so the artifact holds no user data
ci-integration#1.4 · yes · the coverage leg's lcov rule (its path, the keyboard_compat.rs exclusion, public-source counts only) sits in ci-leg.sh, a reached file the widening edits, so it must still hold on the changed file
ci-integration#1.5 · yes · the re-read runs scripts/agent-run.sh run stand (task Needs, indirect), so its test events carry file and test name only and run.log holds the checks' merged output, which must stay silent
ci-integration#1.6 · no · the chunk spawns no escher-session host and makes no state directory (Not needed); session_common is reached for its silence only, a re-check every change owes
ci-integration#1.7 · no · session recording and its kept-check directory are untouched (Not needed)
ci-integration#1.8 · no · the cold-agent pipe is not run or changed (Not needed), and CI runs only its shim contract tests, which the leg widening does not touch
ci-integration#1.9 · no · the committed Counter checks and stand_replay are untouched (Not needed)
