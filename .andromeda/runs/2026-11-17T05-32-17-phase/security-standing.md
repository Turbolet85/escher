# security · standing conventions

- S1 · each new test or fixture of an engine-side crate takes no tracing subscriber and no style dependency of its own · dependency-security
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
- S2 · each change to the cold-agent pipe keeps its events and logs to run metadata only, its recordings owner-only and private, and its record, transcripts and server output unprinted · logging-monitoring · data-protection
  contract/kept-check-file/lifecycle · Kept check files · lifecycle — A kept check is never logged and never removed by escher, but for its own check when a recording ends.
  contract/kept-check-file/medium · Kept check files · medium — Under `--record <dir>`, `Session::recording_into` or an exploration's record directory, one `check-{n}.json` with mode `0600` per recorded session, each explored path its own, is written in a `0700` directory the launcher named.
  decision/cold-agent-transcript-raw · Cold-agent raw transcript kept unscrubbed — The cold-agent session's raw transcript stays unscrubbed by design, is never printed and is gitignored; each committed copy has its host paths, the session dir and the home prefix, masked.
  decision/kept-check-owner-only · Kept checks are owner-only — A recorded check is written 0600 into a 0700 directory the launcher named, is never logged, and is never removed by escher but for its own check when a recording ends. · why: Ratified. · guard: recording::the_committed_checks_are_what_the_binary_records · except: Two scripted stand-Counter checks, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
  decision/kept-checks-owner-only · Kept checks owner-only and uncommitted — A kept check stays in an owner-only medium, is never logged and is never committed. · except: The two committed checks of the repository's own scripted stand-Counter sessions.
  decision/recording-dir-never-named · Recording directory is never exposed — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged. · why: Ratified.
  decision/recording-dir-owner-only · Recording directory is owner-only — The recording directory is created mode 0700 and a present one open to group or others is refused `record-dir-not-private` with status 3, nothing written and nothing spawned. · why: Ratified.
  decision/recording-directory-never-named · Recording directory never named or logged — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
  decision/recording-directory-owner-only · Owner-only recording directory — The recording directory is created 0700, a group- or other-open one is refused with nothing written or spawned, each check is created 0600 and never replaced, and recording is unix only.
  decision/recording-never-replaces-files · Recording never replaces or removes foreign files — A recorded check file is created 0600 with create-new under the smallest free number, rewritten whole through a temp file and rename; no file is replaced and a file the recorder did not create is never removed.
- S3 · each new refusal a test asserts is a typed value from a closed set with fixed text, holding nothing the caller supplied · error-handling
  contract/cli-roles/coverage · escher-session roles · coverage — coverage <app> <check>... with no --session or --record, else usage; each check read up to 1,048,576 bytes, and an unreadable, oversized, non-check or other-app file refused malformed, status 1, holding nothing of the path or file.
  contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error: the session-error object on stdout and the session error's fixed message on stderr.
  contract/mcp-errors/32600 · MCP protocol errors · -32600 — invalid request, with no data: not an object with jsonrpc "2.0" and a string method, or an id of another form.
  contract/mcp-errors/32601 · MCP protocol errors · -32601 — method not found, with no data: every method but initialize, ping, tools/list and tools/call.
  contract/mcp-errors/32602 · MCP protocol errors · -32602 — invalid params or unknown tool, with no data: params not an object with a string name and an optional object arguments, or a name outside the verb table.
  contract/mcp-errors/32700 · MCP protocol errors · -32700 — parse error, with no data: a non-UTF-8, non-JSON or overlong line.
  contract/mcp-protocol-errors/invalid-params · MCP protocol errors · invalid params — A fixed code and fixed message with no `data`; only the admitted request id is written back.
  contract/mcp-protocol-errors/invalid-request · MCP protocol errors · invalid request — A fixed code and fixed message with no `data`; only the admitted request id is written back.
  contract/mcp-protocol-errors/method-not-found · MCP protocol errors · method not found — A fixed code and fixed message with no `data`; only the admitted request id is written back.
  contract/mcp-protocol-errors/parse-error · MCP protocol errors · parse error — A fixed code and fixed message with no `data`; only the admitted request id is written back.
  contract/mcp-protocol-errors/unknown-tool · MCP protocol errors · unknown tool — A fixed code and fixed message with no `data`; no reply holds the tool name.
  contract/mcp-server/protocol-error · MCP server · protocol error — A protocol error is a fixed code and message with no data.
  contract/refusal-causes/disabled · Driver refusal causes · disabled — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
  contract/refusal-causes/fonts-unavailable · Driver refusal causes · fonts-unavailable — One of the eight closed causes, returned by the executor, with a fixed name, meaning and remedy holding no path.
  contract/refusal-causes/not-found · Driver refusal causes · not-found — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
  contract/refusal-causes/stale · Driver refusal causes · stale — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
  contract/refusal-causes/unknown-verb · Driver refusal causes · unknown-verb — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
  contract/refusal-json/cause · Refusal JSON · cause — The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.
  contract/refusal-json/meaning · Refusal JSON · meaning — The cause's fixed meaning text.
  contract/refusal-json/remedy · Refusal JSON · remedy — The cause's fixed remedy text.
  contract/session-error-json/kind · Session error JSON · kind — One fixed kebab-case word per session error variant, inside `{"error":{...}}`.
  contract/session-error-json/message · Session error JSON · message — The error's fixed meaning, holding no path.
  contract/session-error-json/remedy · Session error JSON · remedy — The error's fixed remedy text.
  decision/reach-error-closed-set · Reach errors as a closed set with fixed texts — A reach error is one of a closed set: unknown state, refused at a step with the step's index and its closed refusal cause, or not settled at a step with the busy reading; each reads a fixed text holding nothing a caller supplied. · guard: a_reach_error_reads_fixed_text_naming_no_name_tried
  decision/refusal-closed-cause-set · Command refusal as a closed set of causes — An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field. · why: So a refusal cannot hold anything a call supplied. · except: A Fault is carried only beside the malformed cause.
  decision/refusal-holds-no-call-input · A refusal holds nothing a call supplied — Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
  decision/session-error-fixed-table · Session errors from a fixed table — An escher-driver session error is written from a fixed table, holding nothing a call supplied.
  decision/session-errors-fixed-table · Session errors as one fixed table — Each session error's kind, meaning and remedy is one row of a fixed table served by `help`, every text a fixed string holding no path, carried on stdout and in MCP results as the program's own text. · why: Ratified as the program's own fixed text inside the ratified carriers.
  decision/session-label-only-caller-string · Session label is the one echoed caller string — The session label is never text read from the document and is the one caller string a reply carries.
  decision/state-directory-long-path-refused · Too-long state directory path refused — A state directory path too long for a socket address is refused with a fixed message holding no path.
