# obs · standing conventions

- S1 · every new test or check is silent on success and shares its helpers through the common modules; its failure names a kind and a count, never content. · 3-observability-harness-contract
  decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Checks whose subject is telemetry install escher's sink, the MCP tools check its own recorder, the paint-tree bench reads one opt-in env var, and a re-run child prints ids or digests its parent reads.
  decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
- S2 · every new test that asserts an empty stderr unsets RUST_LOG for the child it runs. · 3-observability-harness-contract
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
- S3 · every kept check or fixture the chunk commits holds no secret or private data and no page content beyond the fixture's own. · 9-ci-integration · ci-integration
  decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.
  decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.
- S4 · every new log line or span carries no content-named value; content is redacted at the sink, never stored, and the span line keeps the closed span line's layout. · 8-pii-scrubbing-compliance · pii-scrubbing-compliance
  contract/escher-log-line/escaping · escher's sink log line · value escaping — Newlines and carriage returns in values are escaped as \n and \r, and string values are Debug-quoted.
  contract/escher-log-line/scrub · escher's sink log line · field scrub — Every field of an event line and every pair of a span line passes the sink's scrub.
  contract/escher-log-line/span-line · escher's sink log line · closed span line — A closed span's line is {time} {LEVEL} {target} service.name=… service.version=… span={the span's name}, then the span's own fields in the order recorded, then message, time.busy and time.idle.
  contract/escher-log-line/stream · escher's sink log line · output stream — One line per printed event and one per closed span, on stderr, with no ANSI.
  contract/span-line/field-order · The closed span's line · field order — `{time} {LEVEL} {target} service.name=… service.version=… span={the span's name}`, then the span's own fields in the order recorded, then `message`, `time.busy` and `time.idle`.
  contract/span-line/level-and-target · The closed span's line · level and target — The line's level and target are the span's own.
  contract/span-line/scrub · The closed span's line · scrub — Every pair of the line, `span` included, passes the scrub under the span's target.
  contract/span-line/span · The closed span's line · span — `span` is a reserved field name holding the span's name.
  contract/span-line/time-busy · The closed span's line · time.busy — The busy timing is tracing-subscriber's own; no escher code reads a clock for it.
  contract/span-line/time-idle · The closed span's line · time.idle — The idle timing is tracing-subscriber's own; no escher code reads a clock for it.
  contract/span-line/value-quoting · The closed span's line · value quoting — A string value is Debug-quoted (`span="command"`, `message="close"`); a number or a bool is written bare.
  contract/telemetry-scrub-set/error · escher-telemetry's scrub set · error — An `error` field is in the scrub set, so the sink prints its value as `[redacted]`; a failed fetch's error text reaches no carrier beyond its log line.
  contract/telemetry-scrub-set/url · escher-telemetry's scrub set · url — A `url` field is in the scrub set, so the sink prints its value as `[redacted]`.
  decision/closed-span-lines-judged-alike · Closed span lines judged by the same rule — A closed span's line is judged pair by pair by the target rule, and a span from a target outside the allowlist writes no byte at creation, record, enter, exit or close.
  decision/content-named-span-values-not-stored · Content-named span values never stored — The value of a content-named span field is never stored, its marker written at print time.
  decision/content-span-fields-never-stored · Content-named span fields never stored — The value of a content-named span field is never stored; its [redacted] marker is written at print time, and every other span field's value is held only in process memory until the span closes.
  decision/engine-target-safe-fields · Engine targets print safe fields only — Under an engine target (blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer, js_console) the sink prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line, every other field as {name}=[redacted].
  decision/escher-target-content-named-set · Escher targets redact the content-named set — Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
  decision/scrub-at-sink-not-call-sites · Scrubbing at the sink, not at call sites — Scrubbing is applied in escher's own sink, a closed span's line judged pair by pair by the same rule, and no engine call site is edited.
