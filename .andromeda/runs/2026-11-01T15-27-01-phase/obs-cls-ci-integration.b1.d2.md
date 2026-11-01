# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT report and its Pages archive are upstream-only and in the task's Not needed list; wpt.yml is not reached
ci-integration#1.2 · no · wptscores.json is upstream-only and in the Not needed list; wpt.yml is not reached
ci-integration#1.3 · yes · the per-leg raw log in target/ci-logs/, unscrubbed and uploaded only on failure, is a Needs item for the per-OS tally read from the take's green run logs
ci-integration#1.4 · no · the coverage leg and its lcov artifact are neither reached nor named by the scope; ci-leg.sh is re-read only for the fast, doc, a11y and audit legs
ci-integration#1.5 · no · agent-run state is in the Not needed list; scripts/agent-run.sh is not changed by the chunk
ci-integration#1.6 · no · the session directory's 0700 mode and socket-only content are the session library's rule, which the take does not change and the capture checks do not read
ci-integration#1.7 · no · the recording and session directories are in the Not needed list
ci-integration#1.8 · no · cold-agent pipe state is in the Not needed list; the chunk runs no cold-agent bar
ci-integration#1.9 · no · cold-agent recording is in the Not needed list, and no kept check of that run is committed
ci-integration#1.10 · yes · the committed Counter checks are the input the stand re-run reads through stand_replay and the freshness check, so their ids, snapshot text and diffs must still hold after the take's layout moves
