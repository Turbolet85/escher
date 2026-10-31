# Classified — test-plan · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · yes · stale · cli_commands moves from 14 to 15 tests, and this procedure states the count 14
e2e-test-strategy#1.2 · yes · stale · mcp_log moves up one test, so the count (1) this procedure states is stale
e2e-test-strategy#2.1 · no · Cells' MountedData handle calls and events.rs:162 lie in no reached file and no touch names them
e2e-test-strategy#3.1 · yes · stale · reached example test files (cli_commands, host_log, mcp_log, common/mod.rs) gain tests and a helper, so the assertion search over slice s02 can read differently
e2e-test-strategy#3.2 · yes · add-to · F11: cli_commands gains a hosted test that opens a list, shows it and chooses a row through the binary, joining the list of UI-driven tests
