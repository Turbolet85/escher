group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «nor the in-process `Session::lint` — no verb, never handed to `Session::run` — with blitz-dom's»
  source: .andromeda/obs-plan.md:121 «and `Session::lint` itself, which the `lint` verb reaches inside its call's `command` span, opening none of its own, with blitz-dom's read-only `BaseDocument::text_extent` and `Harness::text_contrast`»
  replace-with: «nor `Session::lint` itself, which the `lint` verb reaches inside its call's `command` span, opening none of its own, with blitz-dom's»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «record `verb` alone, an accepted `advance`»
  source: .andromeda/obs-plan.md:121 «record `verb` alone (an element shot that crops to no pixel is refused `off-screen`), an accepted `lint` records `verb` alone too, and a refused call records `verb` (when known) and `cause`»
  replace-with: «record `verb` alone, an accepted `lint` records `verb` alone too, an accepted `advance`»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «and a shot's image, its byte size and its `width` / `height` are never fielded»
  source: .andromeda/obs-plan.md:121 «never fielded — `in_view`, `advanced_ms`, the screen text or its length, a shot's image, its byte size or its `width` / `height`, a lint's findings or their count, the session's label»
  replace-with: «and a shot's image, its byte size and its `width` / `height`, and a lint's findings or their count, are never fielded»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «its seventh, `screenshot`, with it —»
  source: .andromeda/obs-plan.md:70 «its sixth verb `scroll` and, since 2026-10-12-headless-screenshot, its seventh, `screenshot`, its eighth, `lint`, and `Session::with_time` with them»
  replace-with: «its seventh, `screenshot`, and its eighth, `lint`, with it —»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «as an `image` block then the line without `png`, never logged or fielded —,»
  source: .andromeda/obs-plan.md:70 «and `Outcome::Linted { findings }` is written by `to_json` alone as `{"findings":[…]}` — a command's answer, over MCP its JSON line, kept whole in a recorded check — never logged or fielded»
  replace-with: «as an `image` block then the line without `png`, never logged or fielded, and a lint's `Outcome::Linted { findings }` written by `to_json` alone as `{"findings":[…]}` — a command's answer, over MCP its JSON line, kept whole in a recorded check — never logged or fielded —,»

- leaf: .claude/docs/obs-summary.md:43   class: omission
  leaf-text: «under both builds (`host_log`, `mcp_log`). A recording host»
  source: .andromeda/obs-plan.md:195 «a lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles»
  replace-with: «under both builds (`host_log`, `mcp_log`). A lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles. A recording host»

- leaf: .claude/rules/observability.md:24   class: stale
  leaf-text: «nor the in-process `Session::lint` (no verb, never handed to `Session::run`) with blitz-dom's»
  source: .andromeda/obs-plan.md:121 «and `Session::lint` itself, which the `lint` verb reaches inside its call's `command` span, opening none of its own, with blitz-dom's read-only `BaseDocument::text_extent` and `Harness::text_contrast`»
  replace-with: «nor `Session::lint` itself, which the `lint` verb reaches inside its call's `command` span, opening none of its own, with blitz-dom's»

- leaf: .claude/rules/observability.md:24   class: omission
  leaf-text: «a `screenshot` call's command span records `verb` (and `cause` when refused) alone — never a shot's image, its byte size or its `width` / `height`.»
  source: .andromeda/obs-plan.md:121 «an accepted `lint` records `verb` alone too, and a refused call records `verb` (when known) and `cause` and none of the other six; … never fielded — … a lint's findings or their count»
  replace-with: «a `screenshot` or `lint` call's command span records `verb` (and `cause` when refused) alone — never a shot's image, its byte size or its `width` / `height`, nor a lint's findings or their count.»

- leaf: .claude/rules/observability.md:32   class: omission
  leaf-text: «0 occurrences of its first 64 base64 characters at `trace`, under both builds);»
  source: .andromeda/obs-plan.md:195 «a lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles»
  replace-with: «0 occurrences of its first 64 base64 characters at `trace`, under both builds; a lint's `command` line holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles);»
