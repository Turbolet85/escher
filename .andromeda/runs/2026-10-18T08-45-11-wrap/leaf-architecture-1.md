group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «a wrapper may override), `BaseDocument`, `DocumentMutator`,»
  source: .andromeda/architecture.md:139 «- The document's clock (PROVISIONAL): `DocumentConfig.clock: Option<Arc<dyn Clock>>`, `None` in `Default`; a document holds the config's clock, else a fresh `WallClock` — wall time, as before —»
  replace-with: «a wrapper may override), `BaseDocument`, the document's clock (PROVISIONAL: `DocumentConfig.clock: Option<Arc<dyn Clock>>`, `None` by default and the document then holding a fresh `WallClock` — wall time, as before — read for the overlay scrollbar fade, the double-click window, pan and the fling's generation, the smooth-scroll start and the fling and smooth frames, a sub-document's config carrying its parent's clock; the crate root re-exports `Clock` (`Send + Sync`, `now() -> Duration`), `WallClock` and `VirtualClock` (every clone reading one time, `advance(Duration)` adding with saturation), and no `Instant::now()` or `SystemTime::now()` stands outside `clock.rs` — an API addition flagged upstreamable), `DocumentMutator`,»

- leaf: .claude/docs/services/blitz-dom.md:30   class: omission
  leaf-text: «**Document core:** `src/document.rs`, `src/config.rs`, `src/resolve.rs`»
  source: .andromeda/architecture.md:278 «node, clock, config, cssom, debug, events, font_metrics, form, html, iframe, layout, mutator»
  replace-with: «**Document core:** `src/document.rs`, `src/config.rs`, `src/clock.rs`, `src/resolve.rs`»
