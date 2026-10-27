# Classified — obs · ci-integration.b1

## Answers

ci-integration#1.1 · no · the WPT report is an upstream-only CI telemetry artifact the task's Not needed list excludes; no reached file or Need touches it
ci-integration#1.2 · no · wptscores.json is an upstream-only CI telemetry artifact the task's Not needed list excludes; no reached file or Need touches it
ci-integration#1.3 · no · a failed leg's merged CI log is in the task's Not needed list; ci-leg.sh and ci.yml are not reached
ci-integration#1.4 · no · the coverage report is in the task's Not needed list; no reached file names the coverage leg or its lcov artifact
ci-integration#1.5 · no · the agent-run harness log is in the task's Not needed list, and the chunk runs and changes no harness
ci-integration#1.6 · no · the session socket directory belongs to the session library and its checks, which the chunk does not reach, and no Need names its contents
ci-integration#1.7 · yes · the README's recorded-check line must agree with the plan that a kept check is an owner-only file at rest and never logged (no log or event line)
ci-integration#1.8 · yes · the chunk cites verdict counts (tool_calls, wrong_calls, screenshot_calls, lint_calls) from verdict.json and server.log, and the rule that these raw files are never printed, copied or committed binds that citing
ci-integration#1.9 · yes · the chunk cites recorded_sessions and recording_kept from record/, and the rule that no kept check of the live run is copied or committed binds that citing
ci-integration#1.10 · yes · the Need that a kept check is never logged covers the two committed Counter checks, which this entity records as no log or event line and as read by stand_replay
