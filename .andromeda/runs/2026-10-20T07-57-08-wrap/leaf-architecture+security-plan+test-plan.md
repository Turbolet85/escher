group: architecture+security-plan+test-plan
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:6   class: stale
  leaf-text: «the **MCP server** that serves the eleven verbs as MCP tools»
  source: .andromeda/architecture.md:196 «`tools/list` answers the twelve verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`), `help` last»
  replace-with: «the **MCP server** that serves the twelve verbs as MCP tools»

- leaf: .claude/docs/services/escher-driver.md:6   class: stale
  leaf-text: «and since 2026-10-19-visual-lint the in-process **visual lint** (`Session::lint`, in process only — its `lint` verb, on every carrier and recorded, owed to»
  source: .andromeda/architecture.md:202 «The visual lint — `Session::lint` in process and the `lint` verb, the eighth instance verb, on every carrier (the socket's `call`, the command line, an MCP tool, `help`'s listing) and recorded»
  replace-with: «and since 2026-10-19-visual-lint the **visual lint** (`Session::lint` in process and the `lint` verb, the eighth instance verb, on every carrier (the socket's `call`, the command line, an MCP tool, `help`'s listing) and recorded — built by»

- leaf: .claude/docs/services/escher-driver.md:6   class: stale
  leaf-text: «returns its layout and contrast defects as typed findings, crossing nothing.»
  source: .andromeda/architecture.md:118 «a lint's findings cross on those carriers and rest in a kept check as the `lint` verb's answer — stable ids, the closed defect words and numbers, no page text, colour, control value or `NodeId`»
  replace-with: «returns its layout and contrast defects as typed findings, which cross on the ratified carriers and rest in a kept check as the `lint` verb's answer — stable ids, the closed defect words and numbers, no page text, colour, control value or `NodeId` (a ratified boundary widening).»

- leaf: .claude/docs/services/escher-driver.md:16   class: stale
  leaf-text: «a call no record can spell (a verb outside the seven instance verbs, an argument»
  source: .andromeda/architecture.md:200 «a call no record can spell (a verb outside the eight instance verbs, a `lint` step kept like every other with its answer whole, an argument its schema does not name or names twice) is not kept»
  replace-with: «a call no record can spell (a verb outside the eight instance verbs, a `lint` step kept like every other with its answer whole, an argument»

- leaf: .claude/docs/services/escher-driver.md:17   class: stale
  leaf-text: «`Session::lint(&mut self) -> Result<Vec<Finding>, Refusal>` (since 2026-10-19-visual-lint; in process only — its `lint` verb, on every carrier and recorded, owed to»
  source: .andromeda/architecture.md:202 «The visual lint — `Session::lint` in process and the `lint` verb, the eighth instance verb, on every carrier (the socket's `call`, the command line, an MCP tool, `help`'s listing) and recorded»
  replace-with: «`Session::lint(&mut self) -> Result<Vec<Finding>, Refusal>` (since 2026-10-19-visual-lint; in process and, as the `lint` verb, the eighth instance verb, on every carrier (the socket's `call`, the command line, an MCP tool, `help`'s listing) and recorded — built by»

- leaf: .claude/docs/services/escher-driver.md:17   class: stale
  leaf-text: «No verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step, span or field carries it, and `VERBS` stays eleven.»
  source: .andromeda/architecture.md:202 «The `lint` verb carries it — `Command::Lint`, `Outcome::Linted`, the `{"findings":[…]}` written form, an MCP tool and a recorded step, its answer kept whole»
  replace-with: «The `lint` verb carries it — `Command::Lint`, `Outcome::Linted`, the `{"findings":[…]}` written form, an MCP tool and a recorded step, its answer kept whole — its `command` span recording `verb` and, refused, `cause` alone, no field added, and `VERBS` holds twelve.»

- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «a shot changes nothing (`has_changes` stays false, focus and the next snapshot unchanged).»
  source: .andromeda/architecture.md:196 «the `lint` arm calling `Session::lint` (packages/escher-driver/src/execute.rs:261-263), so a lint dispatches nothing, runs no step and settles nothing»
  replace-with: «a shot changes nothing (`has_changes` stays false, focus and the next snapshot unchanged); `lint` (since 2026-10-19-visual-lint) takes no argument and its arm calls `Session::lint`, so a lint dispatches nothing, runs no step and settles nothing, answering its findings.»

- leaf: .claude/docs/services/escher-driver.md:18   class: stale
  leaf-text: «while `snapshot` and `screenshot`, which run no step, wait on nothing.»
  source: .andromeda/architecture.md:203 «and `snapshot`, `screenshot` and `lint`, which run no step, wait on nothing»
  replace-with: «while `snapshot`, `screenshot` and `lint`, which run no step, wait on nothing.»

- leaf: .claude/docs/services/escher-driver.md:19   class: omission
  leaf-text: «`Shot { width, height, png }` (the last since 2026-10-12-headless-screenshot)»
  source: .andromeda/architecture.md:196 «`Linted { findings: Vec<Finding> }` (packages/escher-driver/src/execute.rs:75-79)»
  replace-with: «`Shot { width, height, png }` (since 2026-10-12-headless-screenshot) · `Linted { findings: Vec<Finding> }` (since 2026-10-19-visual-lint)»

- leaf: .claude/docs/services/escher-driver.md:20   class: omission
  leaf-text: «no base64 crate taken; `start` answers»
  source: .andromeda/architecture.md:196 «a `lint`'s outcome `{"findings":[…]}`, each finding an object keyed `id` · `defect` · `bounds` (`[x, y, width, height]`, as a node's bounds), then its reason's keys»
  replace-with: «no base64 crate taken; a `lint`'s outcome `{"findings":[…]}`, each finding keyed `id` · `defect` · `bounds` (`[x, y, width, height]`, as a node's bounds), then its reason's keys — `other` and `area` for an overlap, `visible` for a clip, `text` then `visible` (left out when no part can be seen) for a truncation, `ratio` (its shortest decimal, `null` when not finite) and `required` for contrast, none for off-screen and zero-size; `start` answers»

- leaf: .claude/docs/services/escher-driver.md:24   class: stale
  leaf-text: «`VERBS` — the verb table, eleven verbs in three levels»
  source: .andromeda/architecture.md:196 «`VERBS` lists twelve verbs in three levels»
  replace-with: «`VERBS` — the verb table, twelve verbs in three levels»

- leaf: .claude/docs/services/escher-driver.md:24   class: omission
  leaf-text: «then `start` (`app`), `status` and `stop` (session-level;»
  source: .andromeda/architecture.md:196 «`lint` (no argument, one named refused `malformed` with `Fault::Unnamed`; result `findings`, `FieldKind::Entries`, always present), whose help states the six defect words»
  replace-with: «`lint` (the eighth, since 2026-10-19-visual-lint: no argument, one named refused `malformed` with `Fault::Unnamed`; result `findings`, `FieldKind::Entries`, always present; its help stating the six defect words, each reason's keys and its limits — geometry only, no hit, right-to-left and vertical text read as left-to-right lines), then `start` (`app`), `status` and `stop` (session-level;»

- leaf: .claude/docs/services/escher-driver.md:24   class: stale
  leaf-text: «11 verbs, 13 node keys, 12 key names, 3 busy classes, 9 causes»
  source: .andromeda/architecture.md:196 «12 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes, 4 exit statuses, 15 session errors»
  replace-with: «12 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes»

- leaf: .claude/docs/services/escher-driver.md:25   class: omission
  leaf-text: «`Screenshot { id: Option<String> }` (the last since 2026-10-12-headless-screenshot)»
  source: .andromeda/architecture.md:196 «· `Lint` (validated from `("lint", [])`; packages/escher-driver/src/command.rs:175-176)»
  replace-with: «`Screenshot { id: Option<String> }` (since 2026-10-12-headless-screenshot) · `Lint` (validated from `("lint", [])`, since 2026-10-19-visual-lint)»

- leaf: .claude/docs/services/escher-driver.md:26   class: stale
  leaf-text: «`fonts-unavailable` from `screenshot` alone among the verbs, checked before anything is painted, and, since 2026-10-19-visual-lint, from the in-process `Session::lint` before anything is read»
  source: .andromeda/architecture.md:196 «the last from `screenshot` and `lint` among the verbs — a shot checked before anything is painted (below), a lint, and `Session::lint` in process, before anything is read (below)»
  replace-with: «`fonts-unavailable` from `screenshot` and `lint` among the verbs — a shot checked before anything is painted, a lint, and `Session::lint` in process, before anything is read»

- leaf: .claude/docs/services/escher-driver.md:27   class: stale
  leaf-text: «`tools/list` → the eleven verbs of `VERBS` in order (`screenshot` after `scroll` since 2026-10-12-headless-screenshot)»
  source: .andromeda/architecture.md:196 «`tools/list` answers the twelve verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`), `help` last»
  replace-with: «`tools/list` → the twelve verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`)»

- leaf: .claude/docs/services/escher-driver.md:32   class: omission
  leaf-text: «a bigger image ending `oversize` / `AnswerTooLarge`: a boundary widening ratified by the founder (2026-10-12).»
  source: .andromeda/architecture.md:118 «a lint's findings cross on those carriers and rest in a kept check as the `lint` verb's answer — stable ids, the closed defect words and numbers, no page text, colour, control value or `NodeId`»
  replace-with: «a bigger image ending `oversize` / `AnswerTooLarge`: a boundary widening ratified by the founder (2026-10-12). Since 2026-10-19-visual-lint a lint's findings cross on those carriers and rest in a kept check as the `lint` verb's answer — stable ids, the closed defect words and numbers, no page text, colour, control value or `NodeId` — with no new carrier, exit, path argument, cause, session error, span field or dependency: a ratified boundary widening.»

- leaf: .claude/docs/services/escher-driver.md:35   class: stale
  leaf-text: «three of the tool's own texts say measured limits —»
  source: .andromeda/architecture.md:196 «whose help states the six defect words, each reason's keys and its limits — geometry only, no hit, right-to-left and vertical text read as left-to-right lines»
  replace-with: «four of the tool's own texts say limits — `lint`'s help, its limits: geometry only, no hit, right-to-left and vertical text read as left-to-right lines;»

- leaf: .claude/docs/services/escher-driver.md:37   class: stale
  leaf-text: «since 2026-10-19-visual-lint, the one the in-process `Session::lint` reads, which takes no argument»
  source: .andromeda/architecture.md:196 «The record is fed from the snapshot `Session::lint` reads — in process or through the `lint` verb — which takes no argument»
  replace-with: «since 2026-10-19-visual-lint, the one `Session::lint` reads — in process or through the `lint` verb — which takes no argument»

- leaf: .claude/docs/services/escher-driver.md:38   class: stale
  leaf-text: «and `snapshot` and `screenshot` wait on none»
  source: .andromeda/architecture.md:203 «and `snapshot`, `screenshot` and `lint`, which run no step, wait on nothing»
  replace-with: «and `snapshot`, `screenshot` and `lint` wait on none»

- leaf: .claude/docs/services/escher-driver.md:49   class: stale
  leaf-text: «an accepted `screenshot` record `verb` alone;»
  source: .andromeda/architecture.md:196 «an accepted `screenshot` record `verb` alone, as does an accepted `lint`»
  replace-with: «an accepted `screenshot` record `verb` alone, as does an accepted `lint`;»

- leaf: .claude/docs/services/escher-driver.md:49   class: omission
  leaf-text: «a shot's image, its size or its dimensions, the label»
  source: .andromeda/architecture.md:196 «a shot's image, its size or its dimensions, a lint's findings or their count, the session's label»
  replace-with: «a shot's image, its size or its dimensions, a lint's findings or their count, the label»

- leaf: .claude/docs/services/escher-driver.md:62   class: stale
  leaf-text: «`stand_lint` (5, declaring `mod fixtures;` and `mod session_common;`, both layout modes, on the in-process `Session::lint`:»
  source: .andromeda/test-plan.md:27 «`stand_lint` 6, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on `Session::lint` in process and, its sixth, on the `lint` verb through `Session::run`»
  replace-with: «`stand_lint` (6, declaring `mod fixtures;` and `mod session_common;`, both layout modes, on `Session::lint` in process and, its sixth, on the `lint` verb through `Session::run`, which answers what the reader reads — 7 findings on the planted page, 0 on each lean task at boot (`the_lint_verb_answers_what_the_reader_reads`):»

- leaf: .claude/docs/services/escher-driver.md:63   class: omission
  leaf-text: «in process `stand_mcp_tools` (blitz-tests; the flow through `serve_mcp` in both layout modes, one `tool_call` span the parent of each `command`;»
  source: .andromeda/test-plan.md:205 «`stand_mcp_tools` (6, its sixth a `lint` tool call answering the reader's findings and parenting its `command` span»
  replace-with: «in process `stand_mcp_tools` (blitz-tests; the flow through `serve_mcp` in both layout modes, one `tool_call` span the parent of each `command`; since 2026-10-19-visual-lint a `lint` tool call answering the reader's findings and parenting its `command` span;»

- leaf: .claude/docs/services/escher-driver.md:63   class: omission
  leaf-text: «`mcp_server` (the `mcp` role completes the Counter flow by tool calls alone;»
  source: .andromeda/test-plan.md:205 «a `lint` tool call answering its line of findings (examples/seven_guis/tests/mcp_server.rs:340-358)»
  replace-with: «`mcp_server` (the `mcp` role completes the Counter flow by tool calls alone; a `lint` tool call answering its line of findings;»
