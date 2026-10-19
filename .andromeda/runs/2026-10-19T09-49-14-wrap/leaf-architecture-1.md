group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»

- leaf: .claude/docs/services/dioxus-native-dom.md:85   class: stale
  leaf-text: «no kept check holds it either — a recorded session keeps its calls and its answers' digests alone»
  source: .andromeda/architecture.md:193 «while a kept check holds it at rest — each kept answer whole, a `snapshot`'s text and an acting verb's diff included — in an owner-only file never logged»
  replace-with: «a kept check holds it at rest — each kept answer whole, a `snapshot`'s text and an acting verb's diff included — in an owner-only file never logged, the two committed checks under `tests/blitz-tests/checks/` the one exception to owner-only»

- leaf: .claude/docs/services/dioxus-native-dom.md:86   class: omission
  leaf-text: «in an acting MCP tool call's result (ratified by the founder, 2026-10-10) (`stand_diff`)»
  source: .andromeda/architecture.md:193 «no log or event carries a diff, and a file only as a kept check's kept answer, owner-only and never logged, the two committed checks the one exception»
  replace-with: «in an acting MCP tool call's result (ratified by the founder, 2026-10-10) (`stand_diff`); no log or event carries a diff, and a file only as a kept check's kept answer, owner-only and never logged, the two committed checks the one exception»

- leaf: .claude/docs/stack.md:137   class: omission
  leaf-text: «escher-driver's reader of MCP requests and of kept checks, writing nothing»
  source: .andromeda/architecture.md:57 «taken by escher-driver alone as its reader of MCP requests, of kept checks and of the live answer line a replay compares»
  replace-with: «escher-driver's reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing»

- leaf: .claude/docs/stack.md:155   class: stale
  leaf-text: «seven_guis' unix-gated test target `cli_flow` runs two POSIX `sh` scripts under plain `sh` — a shell is that one target's requirement»
  source: .andromeda/architecture.md:336 «`counter.sh` and `flight.sh`, which `cli_flow` runs, and `record_checks.sh`, which `recording` runs to record the two committed checks and compare them byte for byte»
  replace-with: «seven_guis' unix-gated test target `cli_flow` runs two POSIX `sh` scripts under plain `sh` and its `recording` target a third, `record_checks.sh`, under `sh` to record the two committed checks and compare them byte for byte — a shell is those two targets' requirement»

- leaf: .claude/docs/conventions.md:6   class: omission
  leaf-text: «and what the checks that hold a `Session` share in a second such module, `tests/blitz-tests/tests/session_common/mod.rs` (neither a test target).»
  source: .andromeda/architecture.md:245 «tests/blitz-tests/     checks/ — the two committed kept checks `stand_replay` replays»
  replace-with: «and what the checks that hold a `Session` share in a second such module, `tests/blitz-tests/tests/session_common/mod.rs` (neither a test target); the two committed kept checks `stand_replay` replays sit in `tests/blitz-tests/checks/`.»
