group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «names from text children, a trimmed-non-empty `aria-label` and `<label>` association;»
  source: .andromeda/architecture.md:121 «naming a node from four sources — its direct text children (`labelled_by` its `TextRun`s), an `aria-label` non-empty after trimming (`label`, accname-1.2 §2C), a `<label>` bound»
  replace-with: «names from four sources — text children, a trimmed-non-empty `aria-label`, `<label>` association and, since the 2026-10-17 upstream sync, `aria-labelledby`, whose targets' text is set as the computed `label` and outranks a `<label>` and an `aria-label` (the `aria-label` the fallback when the targets hold no text); the same sync's builder also reads `aria-describedby` (pushed as `described_by`), `aria-description` (AccessKit's description field), `aria-expanded` (a boolean), `aria-disabled` (`true` sets AccessKit disabled) and `aria-level` (a number, with defaults by tag and role);»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: omission
  leaf-text: «`name` (the node's `label`, else its `labelled_by` targets' names, trimmed)»
  source: .andromeda/architecture.md:135 «since the 2026-10-17 upstream sync the `label` is also the computed text of an element's `aria-labelledby` targets, which outranks a `<label>` (§Standard Contracts → Mutation, query and CSSOM)»
  replace-with: «`name` (the node's `label`, else its `labelled_by` targets' names, trimmed — since the 2026-10-17 upstream sync the `label` is also the computed text of an element's `aria-labelledby` targets, which outranks a `<label>`, while the snapshot reads no description and `aria-disabled` moves no snapshot reading, `enabled` still reading the `disabled` attribute)»

- leaf: .claude/docs/services/dioxus-native-dom.md:38   class: omission
  leaf-text: «`src/{actionable,dioxus_document,element_id,mutation_writer,events,snapshot,snapshot_diff,snapshot_text,write_once_attr}.rs`»
  source: .andromeda/architecture.md:254 «and, since the 2026-10-17 upstream sync, keyboard_compat (crate-private, upstream's: converts keyboard-types 0.8 into Dioxus 0.7's types for `events.rs`»
  replace-with: «`src/{actionable,dioxus_document,element_id,mutation_writer,events,snapshot,snapshot_diff,snapshot_text,write_once_attr,keyboard_compat}.rs` (`keyboard_compat`, crate-private and upstream's since the 2026-10-17 upstream sync, converts keyboard-types 0.8 into Dioxus 0.7's types for `events.rs`, excluded from the coverage leg by filename until upstream retires it)»

- leaf: .claude/docs/services/blitz-traits.md:11   class: omission
  leaf-text: «keyboard-types, cursor-icon, http, url, bytes.»
  source: .andromeda/architecture.md:52 «keyboard-types "0.8" (lock 0.8.3) since the 2026-10-17 upstream sync, while Dioxus 0.7.3 carries its own keyboard-types 0.7, so two versions stand in the graph»
  replace-with: «keyboard-types (0.8 since the 2026-10-17 upstream sync, while Dioxus 0.7.3 carries its own keyboard-types 0.7, so two versions stand in the graph and a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`), cursor-icon, http, url, bytes.»
