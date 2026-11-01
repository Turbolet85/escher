group: architecture-2
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
- leaf: .claude/docs/services/dioxus-native-dom.md:25   class: omission
  leaf-text: «`mounted` listeners fire after `initial_build` and each `poll`; event kinds with zero handlers are skipped.»
  source: .andromeda/architecture-detail/standard-contracts.md:311 «`flush_queued_mounted_events` skips an element removed since its event was queued (packages/dioxus-native-dom/src/dioxus_document.rs:235-239)»
  replace-with: «`mounted` listeners fire after `initial_build` and each `poll`, `flush_queued_mounted_events` skipping an element removed since its event was queued, which it reads through `try_element_to_node_id` (upstream's test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0); event kinds with zero handlers are skipped.»
- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «its screen 47,949 bytes in 740 lines»
  source: .andromeda/architecture-detail/existing-scopes.md:137 «Cells booted through `stand::boot_cells` and held as a session, its screen 47,089 bytes in 740 lines walked in 5 parts of `SNAPSHOT_TEXT_BUDGET` through the driver»
  replace-with: «its screen 47,089 bytes in 740 lines»
- leaf: .claude/docs/stack.md:58   class: stale
  leaf-text: «fmt, clippy, test, CI scripts»
  source: .andromeda/architecture-detail/stack-and-technologies.md:76 «with `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts → embolden, stopping at the first red) the local pre-push gate»
  replace-with: «fmt, clippy, test, CI scripts, embolden»
