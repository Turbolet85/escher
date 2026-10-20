group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:43   class: omission
  leaf-text: «A lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles.»
  source: .andromeda/obs-plan.md:196 «a part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text»
  replace-with: «A lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles. A part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text — `host_log` and `mcp_log` each drive one part read at `trace` with 0 needles.»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «an accepted `lint` records `verb` alone too,»
  source: .andromeda/obs-plan.md:122 «an accepted `lint` and a part read (`Outcome::Part`) record `verb` alone too»
  replace-with: «an accepted `lint` and a part read (`Outcome::Part`) record `verb` alone too,»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «and a lint's findings or their count, are never fielded;»
  source: .andromeda/obs-plan.md:122 «never fielded — `in_view`, `advanced_ms`, the screen text or its length, a part's number, the part count, a shot's image, its byte size or its `width` / `height`, a lint's findings or their count»
  replace-with: «a lint's findings or their count, and a part's number or the part count, are never fielded;»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «with blitz-dom's read-only `BaseDocument::text_extent` and `Harness::text_contrast`, the lint's contrast reader, which open no span, emit no event and record no field,»
  source: .andromeda/obs-plan.md:122 «and dioxus-native-dom's `Snapshot::text_parts`, the part read's cut, which open no span, emit no event and record no field, escher-driver keeping its 2 `info_span!` sites»
  replace-with: «with blitz-dom's read-only `BaseDocument::text_extent` and `Harness::text_contrast`, the lint's contrast reader, and dioxus-native-dom's `Snapshot::text_parts`, the part read's cut, which open no span, emit no event and record no field,»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «adds no log site, span, field, log field or env read and reads no wall clock, escher-driver keeping its 2 `info_span!` sites.»
  source: .andromeda/obs-plan.md:83 «The parts check is silent: `stand_snapshot_parts` (a `fixtures` and `session_common` reader, both layout modes, in process and through `serve_mcp`) installs no subscriber, reads no env var and prints»
  replace-with: «adds no log site, span, field, log field or env read and reads no wall clock, escher-driver keeping its 2 `info_span!` sites; the parts check `stand_snapshot_parts` (a `fixtures` and `session_common` reader, both layout modes, in process and through `serve_mcp`) likewise installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, and dioxus-native-dom's `Snapshot::text_parts` and `TextPart` hold no log, print or failure path, adding no log site, span, field, log field or env read.»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «and a lint's `Outcome::Linted { findings }` written by `to_json` alone as `{"findings":[…]}` — a command's answer, over MCP its JSON line, kept whole in a recorded check — never logged or fielded —,»
  source: .andromeda/obs-plan.md:83 «a part's answer (`Outcome::Part`) is written by `to_json` alone as `{"text":…,"part":n,"parts":m}` — a command's answer, over MCP its JSON line — never logged or fielded»
  replace-with: «and a lint's `Outcome::Linted { findings }` written by `to_json` alone as `{"findings":[…]}` — a command's answer, over MCP its JSON line, kept whole in a recorded check — never logged or fielded —, and a part's `Outcome::Part` written by `to_json` alone as `{"text":…,"part":n,"parts":m}` — a command's answer, over MCP its JSON line — never logged or fielded —,»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the load-wait check `stand_load_wait` (a `fixtures` and `session_common` reader, both layout modes)»
  source: .andromeda/obs-plan.md:79 «The third shared module of the stand checks, `tests/blitz-tests/tests/fixtures/mod.rs` — the fixture boot and hold, three pages and the held network stand-in `HeldNet`, […] — is silent»
  replace-with: «the stand checks' third shared module `tests/blitz-tests/tests/fixtures/mod.rs` (the fixture boot and hold, three pages and the held network stand-in `HeldNet`) holds no print, `tracing` call, subscriber, env read, file, socket, listener or `blitz_net`, its 24 readers adding no print or sink site; the load-wait check `stand_load_wait` (a `fixtures` and `session_common` reader, both layout modes)»

- leaf: .claude/rules/observability.md:24   class: omission
  leaf-text: «a `screenshot` or `lint` call's command span records `verb` (and `cause` when refused) alone — never a shot's image, its byte size or its `width` / `height`, nor a lint's findings or their count.»
  source: .andromeda/obs-plan.md:122 «an accepted `lint` and a part read (`Outcome::Part`) record `verb` alone too […] never fielded — […] a part's number, the part count, a shot's image, its byte size or its `width` / `height`»
  replace-with: «a `screenshot` or `lint` call's or a part read's (`Outcome::Part`) command span records `verb` (and `cause` when refused) alone — never a shot's image, its byte size or its `width` / `height`, nor a lint's findings or their count, nor a part's number or the part count.»

- leaf: .claude/rules/observability.md:24   class: omission
  leaf-text: «with blitz-dom's read-only `BaseDocument::text_extent`, which open no span,»
  source: .andromeda/obs-plan.md:122 «and dioxus-native-dom's `Snapshot::text_parts`, the part read's cut, which open no span, emit no event and record no field, escher-driver keeping its 2 `info_span!` sites»
  replace-with: «with blitz-dom's read-only `BaseDocument::text_extent`, and dioxus-native-dom's `Snapshot::text_parts`, the part read's cut, which open no span,»

- leaf: .claude/rules/observability.md:32   class: omission
  leaf-text: «a lint's `command` line holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles);»
  source: .andromeda/obs-plan.md:196 «a part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text»
  replace-with: «a lint's `command` line holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles; a part read's `command` line, `verb="snapshot"`, holds no part number, count, size or text — `host_log` and `mcp_log` each drive one part read at `trace` with 0 needles);»
