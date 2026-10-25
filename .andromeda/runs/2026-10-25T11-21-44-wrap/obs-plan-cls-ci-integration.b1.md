# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT archive and dispatch run in wpt.yml, which the change does not reach, and the report leaves WPT score publishing untouched
ci-integration#1.2 · no · wptscores.json is computed and published by wpt.yml, which the change does not reach, and the report leaves WPT score publishing untouched
ci-integration#1.3 · no · the log path, upload-on-failure rule and retention in ci-leg.sh and ci.yml are unchanged, and the new pin adds no log site
ci-integration#1.4 · no · the coverage leg, its lcov output and the workspace's Rust source are unchanged, and the new Python pin sits outside the line counts
ci-integration#1.5 · no · scripts/agent-run.sh and its cleanup are unchanged, and the new pin writes nothing under target/
ci-integration#1.6 · no · no check or session-directory code changed, and the new pin writes no file
ci-integration#1.7 · no · no recording code, kept check or check file changed, so the recording directory's contents and at-rest rules hold
ci-integration#1.8 · no · the pipe's script, state files and shim contract tests are unchanged, and the new pin is not a cold-agent test, so CI still runs only the shim contract tests for the pipe
ci-integration#1.9 · no · the two Counter checks and record_checks.sh are unchanged, and the new committed files do not alter what the checks hold
