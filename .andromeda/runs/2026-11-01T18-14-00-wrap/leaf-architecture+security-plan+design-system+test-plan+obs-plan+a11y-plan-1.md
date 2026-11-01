group: architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan-1
read: CLAUDE.md · 151 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «and three read-only readers, `text_extent`»
  source: .andromeda/architecture-detail/established-decisions.md:80 «A dropped anonymous block clearing its children's `layout_parent` stands beside those eight, a direct engine change for every document, ratified»
  replace-with: «a dropped anonymous block's children (`BaseDocument::deallocate_anonymous_block` sets `layout_parent` to `None` on each child still in the slab naming the block, so a key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone, focus staying and the key still acting — a direct engine change for every document, ratified), and three read-only readers, `text_extent`»
- leaf: CLAUDE.md:23   class: omission
  leaf-text: «shared types and provider traits (net, navigation, shell, events, `NodeId`)»
  source: .andromeda/architecture-detail/design-philosophy.md:26 «a wrapping handler must forward it or the failure stops at the wrapper»
  replace-with: «shared types and provider traits (net — `NetHandler::error` a default that does nothing, so a wrapping handler must forward it or the failure stops at the wrapper — navigation, shell, events, `NodeId`)»
- leaf: CLAUDE.md:26   class: omission
  leaf-text: «read from `SelectListView` and past any clip or `opacity` on its select's ancestors»
  source: .andromeda/architecture-detail/standard-contracts.md:130 «`render_element` pushes a viewport-wide blend layer and pops it last, so an open list, through the `paint_select_list` call above, paints last, past the blend layers»
  replace-with: «read from `SelectListView` and past any clip or `opacity` on its select's ancestors and the viewport-wide blend layer `render_element` pushes for `mix-blend-mode` and `isolation`, which paint in every document with no frame test»
- leaf: CLAUDE.md:35   class: stale
  leaf-text: «`None` past an `opacity` below 1, a filter or a background image;»
  source: .andromeda/a11y-plan.md:160 «`None` under opacity below 1, a filter, a background image or a `mix-blend-mode` other than `normal`»
  replace-with: «`None` past an `opacity` below 1, a filter, a background image or a `mix-blend-mode` other than `normal`;»
- leaf: CLAUDE.md:44   class: stale
  leaf-text: «The workspace binds no network port; beside one test-only bind (since 2026-10-14-quality-gates escher-driver's unix-only client unit test binds a `UnixListener` that never answers in `eab-{pid}` under the system temporary directory and removes it — served by no binary, reached by no caller, no crossing)»
  source: .andromeda/architecture-detail/occupied-resources.md:8 «three test-only binds: blitz-net's two below and this one: escher-driver's unix-only unit test»
  replace-with: «The workspace serves no network port; beside three test-only binds (blitz-net's two `TcpListener`s on `127.0.0.1:0` and, since 2026-10-14-quality-gates, escher-driver's unix-only client unit test, which binds a `UnixListener` that never answers in `eab-{pid}` under the system temporary directory and removes it — each served by no binary, reached by no caller, no crossing)»
- leaf: CLAUDE.md:45   class: omission
  leaf-text: «with no DOM `blur` or `focusout`, reading neither `disabled` nor `hidden`;»
  source: .andromeda/a11y-plan.md:107 «The fixup reads `is_inert()` alone, so a focused control that no longer shows, by `hidden`, `style="display:none"` or CSS alone, keeps focus and takes keys, with no panic»
  replace-with: «with no DOM `blur` or `focusout`, reading `is_inert()` alone, so a focused control that no longer shows, by `hidden`, `style="display:none"` or CSS alone, keeps focus and takes keys, with no panic;»
- leaf: CLAUDE.md:52   class: stale
  leaf-text: «(fmt · clippy `--locked -D warnings` · workspace tests · CI scripts — the same legs CI runs)»
  source: .andromeda/architecture-detail/standard-contracts.md:343 «`fast` (fmt → clippy → test → ci-scripts → embolden, stopping at the first red: the local pre-push gate)»
  replace-with: «(fmt · clippy `--locked -D warnings` · workspace tests · CI scripts · `embolden` — the legs CI runs, but for `embolden`, which has no CI job of its own)»
- leaf: CLAUDE.md:52   class: stale
  leaf-text: «neither builds `blitz-paint/font-embolden`, which CI's macOS leg alone exercises, so a paint change under that feature also takes its local witness `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`.»
  source: .andromeda/architecture-detail/standard-contracts.md:352 «`blitz-paint/font-embolden` is built locally by the `embolden` step of `fast`,»
  replace-with: «`fast`'s last step, `embolden`, builds `blitz-paint/font-embolden`, which macOS exercises in CI, and is the local witness for a paint change under that feature: `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`.»
- leaf: CLAUDE.md:96   class: stale
  leaf-text: «the local pre-push gate: fmt → clippy → workspace tests → CI scripts, as CI runs them»
  source: .andromeda/architecture-detail/stack-and-technologies.md:76 «`bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts → embolden, stopping at the first red) the local pre-push gate»
  replace-with: «the local pre-push gate: fmt → clippy → workspace tests → CI scripts → `embolden` (the `blitz-paint/font-embolden` witness, a leg with no CI job of its own), as CI runs them»
