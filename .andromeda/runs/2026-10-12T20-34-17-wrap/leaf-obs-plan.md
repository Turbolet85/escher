group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:12   class: stale
  leaf-text: «`stand_act_spans`'s two re-run children aside, which install the sink over an in-memory capture.»
  source: .andromeda/obs-plan.md:70 «since 2026-10-10-mcp-surface `stand_mcp_tools`, which installs a hand-written `tracing::Subscriber` (`Recorder`) with `tracing::subscriber::with_default` in process — not escher's sink»
  replace-with: «`stand_act_spans`'s two re-run children aside, which install the sink over an in-memory capture, and `stand_mcp_tools`, which installs a hand-written in-process recorder (not escher's sink) that prints nothing.»

- leaf: .claude/docs/obs-summary.md:43   class: omission
  leaf-text: «on the windowed stand it is unmeasured.»
  source: .andromeda/obs-plan.md:296 «A shot's image is screen content that leaves only in its answer: 0 occurrences of the first 64 base64 characters of a shot in the `escher-session` host's and the `mcp` role's stderr at `trace`»
  replace-with: «on the windowed stand it is unmeasured. A shot's image (since 2026-10-12-headless-screenshot) is screen content that leaves only in its answer: 0 occurrences of the first 64 base64 characters of a shot in the `escher-session` host's and the `mcp` role's stderr at `trace`, under both builds (`host_log`, `mcp_log`).»

- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «one of the eight names `escher_driver::Cause::name` returns, for a refused call»
  source: .andromeda/obs-plan.md:106 «one of the nine names `Cause::name` returns (`unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `time-unavailable` · `fonts-unavailable`»
  replace-with: «one of the nine names `escher_driver::Cause::name` returns, for a refused call — `fonts-unavailable` the ninth since 2026-10-12-headless-screenshot»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «(the lengths of the returned diff) — fixed words, bools and counts»
  source: .andromeda/obs-plan.md:106 «`snapshot` and, since 2026-10-12-headless-screenshot, an accepted `screenshot` — whole, or by `id` — record `verb` alone»
  replace-with: «(the lengths of the returned diff) — fixed words, bools and counts; `snapshot` and, since 2026-10-12-headless-screenshot, an accepted `screenshot` — whole, or by `id` — record `verb` alone, and a shot's image, its byte size and its `width` / `height` are never fielded»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «or `serve_mcp` outside a tool call»
  source: .andromeda/obs-plan.md:106 «since 2026-10-12-headless-screenshot `Harness::screenshot` / `screenshot_rect` in blitz-test-harness and the driver's private `image` encoder (`png`, `base64`), which print, panic and log nothing»
  replace-with: «or `serve_mcp` outside a tool call — nor, since 2026-10-12-headless-screenshot, `Harness::screenshot` / `screenshot_rect` and the driver's private `image` encoder, which print, panic and log nothing»

- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «the twelve `stand_act_*` checks»
  source: .andromeda/obs-plan.md:70 «the thirteenth, `stand_act_screenshot`, since 2026-10-12-headless-screenshot»
  replace-with: «the thirteen `stand_act_*` checks — `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot —»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «installs no subscriber and reads no env var; the builder»
  source: .andromeda/obs-plan.md:70 «for `stand_screenshot_repeat` since 2026-10-12-headless-screenshot, which installs no subscriber and re-runs its test binary twice on an ignored child that prints its shots to stdout»
  replace-with: «installs no subscriber and reads no env var, the engine check `password_painted_masked` installs none, and `stand_screenshot_repeat` (since 2026-10-12-headless-screenshot) installs none and re-runs its test binary twice on an ignored child that prints its shots to stdout, read by the parent into its assertions only; the builder»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «its refusal detection and its sixth verb `scroll` with it»
  source: .andromeda/obs-plan.md:70 «its sixth verb `scroll` and, since 2026-10-12-headless-screenshot, its seventh, `screenshot`»
  replace-with: «its refusal detection, its sixth verb `scroll` and, since 2026-10-12-headless-screenshot, its seventh, `screenshot`, with it»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the MCP `help` tool, never a log line —,»
  source: .andromeda/obs-plan.md:70 «the fourteenth module, `image`, the shot's PNG encoder and hand-written base64, which installs no subscriber, reads no env var and prints, panics and logs nothing»
  replace-with: «the MCP `help` tool, never a log line — and so, since 2026-10-12-headless-screenshot, does the `image` module, the shot's PNG encoder and hand-written base64, which prints, panics and logs nothing, a shot (`Outcome::Shot`) written by `to_json` as a command's answer and, over MCP, as an `image` block then the line without `png`, never logged or fielded —,»

- leaf: .claude/rules/observability.md:24   class: omission
  leaf-text: «and the settle loop `Harness::settle` and `Session::act` carry no span of their own.»
  source: .andromeda/obs-plan.md:106 «never fielded — `in_view`, `advanced_ms`, the screen text or its length, a shot's image, its byte size or its `width` / `height`»
  replace-with: «and the settle loop `Harness::settle`, `Session::act` and, since 2026-10-12-headless-screenshot, `Harness::screenshot` / `screenshot_rect` and the driver's private `image` encoder carry no span of their own; a `screenshot` call's command span records `verb` (and `cause` when refused) alone — never a shot's image, its byte size or its `width` / `height`.»

- leaf: .claude/rules/observability.md:32   class: omission
  leaf-text: «and unmeasured on the windowed stand, where no command runs)»
  source: .andromeda/obs-plan.md:296 «A shot's image is screen content that leaves only in its answer: 0 occurrences of the first 64 base64 characters of a shot in the `escher-session` host's and the `mcp` role's stderr at `trace`»
  replace-with: «and unmeasured on the windowed stand, where no command runs; a shot's image, since 2026-10-12-headless-screenshot, measured on the `escher-session` host and its `mcp` role — 0 occurrences of its first 64 base64 characters at `trace`, under both builds)»

- leaf: .claude/rules/observability.md:32   class: omission
  leaf-text: «ids, names, values and state readings by the ratified crossing»
  source: .andromeda/obs-plan.md:70 «`Outcome::Shot { width, height, png }` is written by `to_json` as a command's answer and, over MCP, as an `image` block then the line without `png` (`ToolResult::shot`), never logged or fielded»
  replace-with: «ids, names, values and state readings by the ratified crossing, and since 2026-10-12-headless-screenshot a shot's PNG as a command's answer»
