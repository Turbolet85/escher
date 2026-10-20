group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:40   class: stale
  leaf-text: «given only the escher server's eleven tools»
  source: .andromeda/architecture.md:222 «only the escher server's twelve tools — the driver's own `escher-session mcp`»
  replace-with: «given only the escher server's twelve tools»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «`png` padded base64, status `0`) · `status` each run one command against it»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9 «is the whole command line, twelve verbs — the eight instance-level ones … `lint` the eighth, no argument, its accepted answer one line `{"findings":[…]}`, status `0`)»
  replace-with: «`png` padded base64, status `0`) · `lint` (the eighth instance verb, no argument; its accepted answer one line `{"findings":[…]}`, status `0`) · `status` each run one command against it»

- leaf: .claude/docs/commands.md:59   class: stale
  leaf-text: «its tools the eleven driver verbs (`screenshot` after `scroll` since 2026-10-12-headless-screenshot, an accepted shot answered as an `image` block (`image/png`, base64) then a text block)»
  source: .andromeda/architecture.md:196 «`tools/list` answers the twelve verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`), `help` last»
  replace-with: «its tools the twelve driver verbs (`screenshot` after `scroll` since 2026-10-12-headless-screenshot, an accepted shot answered as an `image` block (`image/png`, base64) then a text block; then `lint`, answering its line of findings)»

- leaf: .claude/docs/commands.md:61   class: retired
  leaf-text: «`stand_lint`, 5 tests, on the in-process `Session::lint` (in process only, no verb):»
  source: .andromeda/test-plan.md:27 «`stand_lint` 6, … on `Session::lint` in process and, its sixth, on the `lint` verb through `Session::run`, which answers what the reader reads — 7 findings on the planted page, 0 on each lean task at boot»
  replace-with: «`stand_lint`, 6 tests, on `Session::lint` in process and, its sixth, on the `lint` verb through `Session::run`, which answers what the reader reads — 7 findings on the planted page, 0 on each lean task at boot:»

- leaf: .claude/docs/services/blitz-test-harness.md:16   class: retired
  leaf-text: «whose in-process `Session::lint` (since 2026-10-19-visual-lint, in process only) reads its `contrast` defect»
  source: .andromeda/architecture.md:202 «The visual lint — `Session::lint` in process and the `lint` verb, the eighth instance verb, on every carrier (the socket's `call`, the command line, an MCP tool, `help`'s listing) and recorded»
  replace-with: «whose `Session::lint` (since 2026-10-19-visual-lint; in process and, as the `lint` verb, the eighth instance verb, on every carrier) reads its `contrast` defect»
