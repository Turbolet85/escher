group: architecture-1
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «`status` each run one command against it, and `stop` ends it;»
  source: .andromeda/architecture.md:138 «each with `--session <dir>` exactly once but `help`, which needs none, admits at most one, reads it not, and answers the self-description's one line with status `0`, reaching no session»
  replace-with: «`status` each run one command against it, `stop` ends it, and `help` (since 2026-10-12-self-description) — which needs no `--session`, admits at most one and reads it not — answers the self-description's one line with status `0`, reaching no session;»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «`<dir>` must be short, since a socket address has about a hundred bytes for the whole path»
  source: .andromeda/architecture.md:133 «at most 94 bytes as given on Linux and 90 on every other target, and its remedy is to name a shorter path for the state directory, or a relative one, which counts only its own bytes»
  replace-with: «`<dir>` must be short — at most 94 bytes as given on Linux and 90 on every other target; past it the session error states the bound and its remedy: a shorter path, or a relative one, which counts only its own bytes»

- leaf: .claude/docs/commands.md:59   class: stale
  leaf-text: «its tools the nine driver verbs»
  source: .andromeda/architecture.md:133 «`tools/list` answers the ten verbs of `VERBS` in order, `help` last»
  replace-with: «its tools the ten driver verbs, `help` last (since 2026-10-12-self-description, answered by the server with no session run)»

- leaf: .claude/docs/services/dioxus-native-dom.md:19   class: stale
  leaf-text: «`added` and `changed` hold `DiffNode { id, parent, role, name, state, bounds }` — the node's reading AFTER the step, flat, `parent` its parent snapshot node's id (`None` for a root) — and `removed` holds ids.»
  source: .andromeda/architecture.md:136 «`DiffNode { id: String, parent: Option<String>, role: accesskit::Role, name: String, state: NodeState, bounds: BoundingRect, differs: Vec<Reading> }`»
  replace-with: «`added` and `changed` hold `DiffNode { id, parent, role, name, state, bounds, differs }` — the node's reading AFTER the step, flat, `parent` its parent snapshot node's id (`None` for a root), and, since 2026-10-12-self-description, `differs` the readings that differ from before the step in node-key order (`Reading::ALL`), empty for an added node, a node that only moved reading `[Bounds]` — and `removed` holds ids; `pub enum Reading` (eleven variants, `Parent` · `Role` · `Name` · `Enabled` · `Checked` · `Value` · `Selected` · `Pressed` · `Invalid` · `Focused` · `Bounds`, with `Reading::ALL`) is re-exported beside them.»
