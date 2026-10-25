# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT report and its Pages and dispatch archive belong to the upstream-only wpt.yml job, which the chunk does not reach
ci-integration#1.2 · no · wptscores.json is computed and published by the upstream-only WPT job, which the chunk does not reach
ci-integration#1.3 · yes · the stand-repeat leg runs the re-pinned stand checks and the ci-scripts leg runs test_cold_agent.py, both logging to this per-leg file uploaded on failure, so the checks must stay silent and cold-agent state must stay out of it
ci-integration#1.4 · no · the coverage leg's lcov report and its success-only upload are not reached, and the chunk changes no coverage exclusion, path or upload
ci-integration#1.5 · no · agent-run.sh's state and its cleanup are not reached, and its log is Not needed for the chunk
ci-integration#1.6 · no · the session state directory is made by host.rs and the session checks' helpers, none reached, and the chunk writes no log, event or screen content into one
ci-integration#1.7 · yes · the cold-agent recording-and-replay goal writes a kept check into a recording directory, so its 0700 directory, 0600 check at rest, no log line and no CI upload bind the goal
ci-integration#1.8 · yes · the chunk edits scripts/cold-agent.sh (TASKS, prompt, goals) and test_cold_agent.py, which write the cold-agent state files and per-run directory this entity fixes
ci-integration#1.9 · yes · the Counter checks are replayed by stand_replay and the freshness check, and the chunk edits stand.rs boot and options, which boot the Counter screen they record
