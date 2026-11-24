# obs · standing conventions

- S1 · each new driver command answer or finding word is a closed value set carrying no page content, and is never logged · 3-observability-harness-contract · 8-pii-scrubbing-compliance
  contract/actions-answer/content · the actions answer (actions) · content — Carries ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text, mask or NodeId; a boundary widening, ratified.
  contract/actions-answer/logging · the actions answer (actions) · logging — The answer is kept whole in a recorded check and is never logged or fielded.
  contract/command-span/actions · The driver's command span · actions — An actions line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer.
  contract/command-span/graph · The driver's command span · graph — A graph's line, a part read's included, is `span="command" verb="graph"` (with `cause` when refused, an argument or a `part` outside 1 to 1000 refused `malformed`) and holds no node, state, id, name, part number or count.
  contract/command-span/layout · The driver's command span · layout — A layout's line is `span="command" verb="layout"` (with `cause` when refused) and holds nothing of its answer; the layout read the call reaches opens no span of its own.
  contract/command-span/snapshot-part · The driver's command span · snapshot part read — A part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text.
  contract/coverage-line/content · the coverage line (escher-session coverage) · content — Carries stable ids, owner ids, handler event names and two counts only, never a value or typed text; a boundary widening, ratified.
  contract/coverage-line/logging · the coverage line (escher-session coverage) · logging — The line is a command answer, never logged or fielded.
  contract/graph-answer/content · the UI graph answer (graph) · content — Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, booleans, and for a part its part number and part count only, never a state's value; a boundary widening, ratified.
  contract/layout-answer/content · the layout answer (layout) · content — Carries ids, closed words and numbers only, never a value or typed text; the sink at trace carries none of its keys (laid_out, placed, shifts).
  contract/layout-answer/logging · the layout answer (layout) · logging — The answer leaves on the session socket, the command line and MCP, crossing to the model provider in the transcript on a live cold-agent run, and is kept whole in a recorded check; it is never logged or fielded.
  decision/coverage-line-never-logged · Coverage line never logged — The handler coverage line is a command line answer only, never logged or fielded; each replayed step of a kept check writes its own command span line. · guard: a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace
  decision/graph-answer-never-logged · Graph answer never logged — The graph answer is written as a command answer alone, never logged or fielded.
- S2 · each new check or fixture is silent, runs with no sink in the headless stand, and unsets RUST_LOG where it reads stderr empty · 3-observability-harness-contract
- S3 · each committed kept check or fixture holds no private data and no local state · ci-integration
