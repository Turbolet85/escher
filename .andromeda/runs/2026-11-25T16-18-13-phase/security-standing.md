# security · standing conventions

- S1 · each new test or check reads no environment beyond its ratified variables, prints no temp or home path and puts no credential into evidence · secret-management
  decision/coverage-floor-input-validation · Coverage floor checker validates and prints no paths — coverage_floor.py, stdlib only, refuses a malformed or repeated floor or a crate with no record with exit 2, and prints only crate directories and counts, never a source or lcov path. · guard: CoverageFloorTest
  decision/coverage-floor-prints-no-paths · Coverage floor prints no paths — The coverage floor check prints only crate directories and counts, never a source or lcov path.
  decision/driver-reads-only-rust-log · Driver reads only RUST_LOG — The escher-driver library and the escher-session binary read no environment variable beyond RUST_LOG.
- S2 · each new log line or span holds no screen content — no accessible name, stable id or caller input — only cause names, verbs and counts · logging-monitoring
  decision/escher-target-content-fields-withheld · escher_ targets withhold content-named fields — In escher's telemetry sink an escher_ target keeps all fields but twelve content-named ones.
  decision/escher-target-content-scrub · Escher targets scrub content-named fields — An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which print as [redacted].
  decision/select-list-label-never-logged · Select list row labels never logged — The select list's public surface is read-only but for one closing writer, and a list row's label, being page text, is never logged.
- S3 · each new process check drives the binary through the closed verb schema and the one validation gate, never a back door · input-validation
  contract/mcp-tool-arguments/value-kinds · MCP tool arguments · value kinds — A string becomes text, an i64 a number, a bool a flag; any other value is refused malformed before anything runs, and a repeated key collapses to its last value.
  contract/recorded-check/arguments · Recorded check file · arguments — Argument names drawn from that verb's schema.
  contract/refusal-causes/malformed · Driver refusal causes · malformed — The one cause that carries a fault; a call breaking the schema, such as an `actions` call with an argument or a `graph` part outside 1 to 1000, is refused with it and nothing runs.
  decision/cli-closed-argv-reader · Closed argv reader — The session command line reads argv with a closed reader with fixed rules, and checks a call before the session address is asked for.
  decision/one-validation-path-for-every-carrier · One validation path for every carrier — The session socket, the command line and MCP each pass a call through the schema's validation before anything runs, and nothing public runs a command that did not come from that validation. · why: The crossing to external input surfaces was ratified with this check as the one they all pass through.
  decision/single-validation-gate · Single command validation gate — Every call, from the socket, the command line or MCP, is checked against a closed static table of verbs and bounded argument kinds, the first failure in a fixed order, and nothing public runs a command that did not come from that validation.
