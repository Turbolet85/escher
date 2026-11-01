# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT report is archived by upstream-only, repository-guarded workflows, which the chunk's Not needed list drops
ci-integration#1.2 · no · wptscores.json is computed and published by the upstream-only WPT workflow, which the Not needed list drops
ci-integration#1.3 · yes · the per-leg merged log in target/ci-logs/ written by ci-leg.sh (truncation at 90-91) and uploaded only on failure is the Needs item on raw logs in target/ci-logs/ for the per-OS tally read, and ci-leg.sh's fast, doc, a11y and audit legs are reached
ci-integration#1.4 · no · standing S4 · the coverage-report lcov artifact is one of the fork's two artifacts, and the coverage leg is not among the reached legs
ci-integration#1.5 · no · the agent-run harness state under target/agent-run/ is named in the Not needed list, and the chunk does not change scripts/agent-run.sh
ci-integration#1.6 · no · the session state directory is a session and recording directory named in the Not needed list, which no commit of the delta or predicted conflict reaches
ci-integration#1.7 · no · the recording directory is named in the Not needed list as recording and session directories, which no commit of the delta or predicted conflict reaches
ci-integration#1.8 · no · the cold-agent pipe state is named in the Not needed list, and the chunk runs no cold-agent bar and changes nothing in that pipe
ci-integration#1.9 · no · cold-agent recording is part of the cold-agent pipe named in the Not needed list, and the chunk changes nothing there
ci-integration#1.10 · no · the committed Counter checks are read by stand_replay and seven_guis' freshness check, neither of which is among the reached files, and the chunk adds no typed text or host path to them
