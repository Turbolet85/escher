# obs · standing conventions

- S1 · a new harness or script log line is a JSON line encoded by python's json module, carrying no user content or captured output · 3-observability-harness-contract
  decision/cold-agent-verdict-holds-no-content · Cold-agent verdict holds no content — Every field of the cold-agent verdict is a count, a boolean, a closed word or an object of closed words to counts, holding no transcript text, tool argument, tool answer, reply or check.
  decision/harness-json-via-python-json · Harness JSON lines encoded by python's json module — The agent-run and cold-agent harnesses write JSON lines encoded by python3's stdlib json module embedded in the script, never assembled in bash.
  decision/verdict-fields-closed · Cold-agent verdict fields closed — Each cold-agent verdict field is a count, a boolean, a closed word, an object of closed words to counts, the task, the client's tool names, version and model, or the transcript file's path, never transcript text, a tool argument, a tool answer, a reply or the check.
- S2 · a new check is silent on success and reports a failure by a kind and a count · 3-observability-harness-contract
  decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Telemetry and error-text checks install escher's sink, MCP tools and layout checks a recorder; the paint-tree bench reads an opt-in env var; a re-run child prints ids or digests; the order page logs.
  decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
- S3 · a new or changed check that asserts empty stderr unsets RUST_LOG · 3-observability-harness-contract
- S4 · a new or changed CI leg uploads its log on failure and uploads no local state · ci-integration · 9-ci-integration
- S5 · a new cleanup or sweep step touches only the state it owns · ci-integration · 9-ci-integration
