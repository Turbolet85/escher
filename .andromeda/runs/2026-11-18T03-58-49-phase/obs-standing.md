# obs · standing conventions

- S1 · a check the chunk adds or widens in a CI script runs silent on success, shares its modules, and names a failure by its kind and a count · 3-observability-harness-contract
  decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Telemetry and error-text checks install escher's sink, MCP tools and layout checks their own recorder, the paint-tree bench reads an opt-in env var, a re-run child prints ids or digests to its parent.
  decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
