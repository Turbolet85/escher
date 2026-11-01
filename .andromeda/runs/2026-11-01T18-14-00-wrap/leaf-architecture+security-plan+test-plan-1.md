group: architecture+security-plan+test-plan-1
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
- leaf: .claude/docs/services/escher-driver.md:36   class: retired
  leaf-text: «An engine panic stands outside this rule, not fixed, its owner a CARRY on «Upstream sync ahead of the UI graph»: a key press panics in `position_in_containing_block` when the focused control's anonymous block was dropped because an ancestor gained `hidden`, on any control, a remote page able to reach it.»
  source: .andromeda/security-plan-detail/error-handling.md:54 «a key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone; focus stays and the key still acts»
  replace-with: «Fixed in the engine since 2026-11-01-upstream-sync-ui-graph: a key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone; focus stays and the key still acts (`BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still naming the block).»
- leaf: .claude/docs/services/escher-driver.md:17   class: omission
  leaf-text: «until the load is answered or `LOAD_WAIT` (5 s of wall time, re-exported at the crate root) has passed»
  source: .andromeda/architecture-detail/standard-contracts.md:241 «until the load is answered, a failed load answering it, or `LOAD_WAIT` has passed»
  replace-with: «until the load is answered, a failed load answering it, or `LOAD_WAIT` (5 s of wall time, re-exported at the crate root) has passed»
- leaf: .claude/docs/services/escher-driver.md:17   class: omission
  leaf-text: «`None` not measured»
  source: .andromeda/architecture-detail/standard-contracts.md:239 «The `contrast` reading reads a node past an element on the walk with a `mix-blend-mode` other than `normal` as not measured»
  replace-with: «`None` not measured, a node past an element on the walk with a `mix-blend-mode` other than `normal` among them»
