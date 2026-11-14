# tests · standing conventions

- S1 · each new integration test file is its own test target, one behaviour per target, and shared helpers live in a shared module that is no target · test-strategy · 2-test-strategy
  decision/one-test-target-per-file · One test target per file — blitz-tests keeps one auto-discovered test target per file under its tests directory, upstream's single all target held unbuilt at test = false. · why: A ratified difference from upstream, whose layout is autotests = false with all the crate's one test target. · guard: BlitzTestsTargetsTest
- S2 · each new stand or layout check runs in both layout modes on the bundled font, never skipping · integration-test-strategy · 5-integration-test-strategy
- S3 · each new check reads no wall time and never sleeps; time moves only through the virtual clock · integration-test-strategy · 5-integration-test-strategy
- S4 · each new session check owns its host process, sends child streams to null and is unix-gated where it uses the socket · session-lifecycle
  decision/check-child-streams-to-null · Check child streams sent to null — A check's child process streams are sent to null unless the check reads them. · why: so the child's libtest lines never reach the agent run script's count
  decision/check-host-state-dir-removed · Check host state directory removed — A check's host state directory is removed at stop.
  decision/check-owns-its-host-process · A check owns its host process — A check that needs a session process starts and stops its own host, every wait bounded, with a guard that kills and reaps a host a failing check still holds and its state directory removed at stop.
  decision/child-streams-to-null · Child streams sent to null — A host child's streams are sent to null unless the check reads them. · why: So the child's own libtest lines never reach the parent's output, where the agent run script would count them as tests.
  decision/failed-check-host-reaped · Host of a failing check reaped — A guard kills and reaps a host that a failing check still holds.
  decision/session-check-owns-its-host · A session check owns its host — A check that needs a session process starts and stops its own host, with every wait bounded.
  decision/session-checks-own-their-host · Session checks own their host — A check that needs a session process starts and stops its own host, bounds every wait, holds the host behind a guard that kills and reaps it, and sends the child's streams to null unless it reads them.
  decision/socket-checks-unix-gated · Socket session checks are unix-gated — A check holding a session over the socket is gated to unix. · except: The MCP checks speak to the mcp child over piped stdin and are not unix-gated, bar the MCP server's command-line comparisons, so the windows CI job runs them.
  decision/socket-session-checks-unix-gated · Socket session checks unix-gated — A check holding a session over the socket is `cfg(unix)`-gated; a check driving the `mcp` stdio child bounds every read, kills and reaps the child in its guard's drop and is not unix-gated, so the windows job runs it. · except: The `mcp_server` command-line comparison is unix-gated.
  decision/socket-session-checks-unix-only · Socket session checks gated to unix — A check holding a session over the socket is gated to unix.
- S5 · each new test is controlled red-before-green and its failures print no argument value; table-driven tests assert their row count · unit-test-strategy · 4-unit-test-strategy
- S6 · each new fixture is stated in code, shared fixture pages stated once, and the stand boots offline · test-data-fixtures · 7-test-data-fixtures
- S7 · each new stand check joins the stand repeat rounds under the zero-failure budget · ci-integration · 9-ci-integration
