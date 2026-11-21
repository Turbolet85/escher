# security · standing conventions

- S1 · every new log line or span field holds no screen content, label or caller input; only closed names and counts · logging-monitoring
  contract/command-span/cause · The command span line · cause — Recorded when the call is refused, as a closed cause word such as fonts-unavailable.
  contract/command-span/verb · The command span line · verb — Records the instance-level verb; a graph call, a graph part's included, an actions or layout call records the verb alone, plus cause when refused, never the part asked or the part count.
  contract/tool-call-span/ending · The tool_call span line · ending — One of the closed words accepted, refused or error.
  contract/tool-call-span/line · The tool_call span line · line — One closed line per MCP tools/call, the parent of that call's command span, holding tool and ending only.
  contract/tool-call-span/tool · The tool_call span line · tool — The tool table's word for the call, a closed word outside the content-named fields; absent for an unknown tool.
  decision/agent-run-events-metadata-only · Agent-run events hold harness metadata only — The agent-run test contract's JSON-line events carry no content-named field, only harness metadata, and never captured test output or panic text.
  decision/closed-span-lines · One sink line per closed span — The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored. · why: A boundary widening of the sink, ratified as built.
  decision/cold-agent-events-metadata-only · Cold-agent events hold run metadata only — The cold-agent pipe's events and verdict carry counts, booleans and closed words: refusals counted per closed cause word, a word outside the set not counted, and caused items per closed kind word, only the kind read; never transcript text, a tool argument or answer, a reply or the check.
  decision/cold-agent-logs-metadata-only · Cold-agent pipe logs run metadata only — The cold-agent pipe's events and verdict carry counts, booleans and closed words only: no content-named field and no transcript text, tool argument, tool answer, reply or check. · guard: The scrub-set key check and four planted markers, under the shim
  decision/command-span-content-free · Command span holds no content — One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
  decision/diff-counts-only-in-logs · Only diff sizes reach a log — A diff's content never reaches a log or event, while its three sizes (counts of added, removed and changed nodes) are recorded on the driver's command span, printed at `info` or below. · why: Ratified: a diff's content never reaches a log while its three sizes may.
  decision/one-command-span-per-call · One command span per call — Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
- S2 · new engine and driver code stays above its crate's line coverage floor · input-validation
  decision/cargo-llvm-cov-coverage-tool · cargo-llvm-cov as the coverage tool — cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.
  decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90. · guard: coverage leg
- S3 · a new test file under the test crate takes no tracing and no style dependency and is never published · dependency-security
  decision/blitz-tests-not-published · blitz-tests not published — The blitz-tests crate is never published (publish = false).
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
