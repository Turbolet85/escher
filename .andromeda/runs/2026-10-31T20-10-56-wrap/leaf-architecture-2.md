group: architecture-2
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»
- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: omission
  leaf-text: «`bounds` (`get_client_bounding_rect` — viewport-relative,»
  source: .andromeda/architecture-detail/standard-contracts.md:272 «`bounds` `BaseDocument::select_list_bounds(node)` first, a row's or a group's place in an open list»
  replace-with: «`bounds` (`BaseDocument::select_list_bounds(node)` first, a row's or a group's place in an open list, its select's client rect again once the list closes; else `get_client_bounding_rect` — viewport-relative,»
- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «escher-driver's `lint.rs`, over the public, re-exported `is_multiple_select`, also read by `execute.rs` `locate`, and `BaseDocument::select_of`»
  source: .andromeda/architecture-detail/standard-contracts.md:21 «`is_multiple_select`, called by `closed_select_of` alone within the workspace, still re-exported; and `is_select`, the driver's select test, called by `execute.rs` `locate` alone»
  replace-with: «escher-driver's `lint.rs`, answering `None` for a select whose list is open, over the public, re-exported `is_multiple_select`, read within the workspace by `closed_select_of` alone, and `BaseDocument::select_of`; `is_select`, public and re-exported, is the driver's select test, read by `execute.rs` `locate` alone»
- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: omission
  leaf-text: «seven fields; a struct literal without a spread names all seven»
  source: .andromeda/architecture-detail/standard-contracts.md:293 «The snapshot carries no `expanded` node key and `NodeState` holds no field for a list, `NODE_FIELDS` keeping the thirteen keys §Driver session names.»
  replace-with: «seven fields, none for a list, the snapshot carrying no `expanded` node key and `NODE_FIELDS` keeping its thirteen keys; a struct literal without a spread names all seven»
- leaf: .claude/docs/workflow.md:9   class: omission
  leaf-text: «the local pre-push gate, the same leg commands CI runs.»
  source: .andromeda/architecture-detail/standard-contracts.md:334 «No local gate builds `blitz-paint/font-embolden`, which CI's macOS leg alone exercises; `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` is the local witness»
  replace-with: «the local pre-push gate, the same leg commands CI runs; no local gate builds `blitz-paint/font-embolden`, which CI's macOS leg alone exercises, so a paint change under that feature takes `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` as its local witness.»
