group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/gotchas.md:11   class: stale
  leaf-text: «two documents resolving concurrently on Stylo's global pool panic (`already mutably borrowed`, blitz issue 430). The `Default` derive says `Sequential` while the field doc says `Parallel`.»
  source: .andromeda/architecture.md:44 «`Parallel` traversals are serialised process-wide by the static `PARALLEL_TRAVERSAL_LOCK`, its poison recovered with no panic path, so two documents resolving `Parallel` concurrently on Stylo's global pool no»
  replace-with: «nothing panics any more: `Parallel` traversals are serialised process-wide by the static `PARALLEL_TRAVERSAL_LOCK` (its poison recovered with no panic path), so two documents resolving `Parallel` concurrently on Stylo's global pool no longer panic (blitz issue 430); the `Default` derive and the field doc both name `Sequential` the default.»

- leaf: .claude/docs/gotchas.md:12   class: retired
  leaf-text: «a driver holding several documents (or a test running several in one process) uses `Sequential`.»
  source: .andromeda/architecture.md:44 «the windowed `dioxus_native::launch` and the browser's document loader and toolbar build their documents `Parallel`, and no headless document — harness, stand or driver — sets `style_threading`»
  replace-with: «no choice is needed for safety — the windowed `dioxus_native::launch` and the browser build their documents `Parallel`, serialised by the lock, and no headless document (harness, stand or driver) sets `style_threading`, so it resolves `Sequential`, which bypasses Stylo's global pool.»

- leaf: .claude/docs/services/blitz-dom.md:24   class: stale
  leaf-text: «`StyleThreading::Parallel` panics with two concurrent documents; `LAYOUT_CTX` is thread-local.»
  source: .andromeda/architecture.md:44 «`Parallel` traversals are serialised process-wide by the static `PARALLEL_TRAVERSAL_LOCK`, its poison recovered with no panic path, so two documents resolving `Parallel` concurrently on Stylo's global pool no»
  replace-with: «`StyleThreading::Parallel` traversals are serialised process-wide by the static `PARALLEL_TRAVERSAL_LOCK`, so two concurrent documents no longer panic (blitz issue 430); `Sequential` is the default, which no headless document overrides; `LAYOUT_CTX` is thread-local with a FIXME for multi-document use.»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «the root element excepted; a box clipped on one axis»
  source: .andromeda/architecture.md:143 «a hit stops at the padding box of a box paint clips and skips a node dropped since the last resolve;»
  replace-with: «the root element excepted — and skips a node dropped since the last resolve; a box clipped on one axis»

- leaf: .claude/docs/services/blitz-dom.md:33   class: tree
  leaf-text: «src/layout/{mod,construct,damage,inline,paint_tree,table,replaced}.rs»
  source: .andromeda/architecture.md:276 «viewport scale applied to Parley measurements and divided back out (packages/blitz-dom/src/layout/inline/layout.rs:236; packages/blitz-dom/src/layout/inline/layout.rs:877-880)»
  replace-with: «src/layout/{mod,construct,damage,paint_tree,table,replaced}.rs`, `src/layout/inline/{mod,build,layout}.rs»
