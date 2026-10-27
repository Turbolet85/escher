# a11y · Screen Reader Support — category read

## Carried
- (.andromeda/a11y-plan.md:192) `lint.rs::findings` reads the held screen as a `Snapshot`, so any named helper or table walk that the factoring adds must take each node's role and name from that model alone and add no role or name mapping of its own, the same as every other reader of the model.
  > The snapshot model consumes the tree: `DioxusDocument::snapshot` takes each node's role, name, focus and `selected`, `pressed` and `invalid` from `accessibility_tree`, never from the element, and `enabled` (the presence of `disabled`) and `value` from the element, `checked` from a native checkbox's or radio's state, else a checkable role's `toggled`; its text form, parts, diff, MCP results, node-key help and `screenshot` read that model and nothing else — never a second role or name mapping.
- (.andromeda/a11y-plan-detail/screen-reader-support.md:39) The load page's boot check must stay green after the lint refactor in both layout modes. This check also asserts that the lint reads no finding at boot, so the factored `findings` must still return none there.
  > The check also returns nothing on the load page the stand serves beside the lean four (`stand_load_page::the_load_page_is_keyed_and_reads_no_finding_at_boot`, in both layout modes), so it reads empty on all six tasks `escher-session` serves

## Bindings
- **The lint walk as a model reader ↔ the category's snapshot-reader rule**: the body's broad line (.andromeda/a11y-plan.md:192) and the detail's list of the model's readers (.andromeda/a11y-plan-detail/screen-reader-support.md:24) both forbid a second role or name mapping. A refactored `findings` stays one reader of the `Snapshot` and holds no mapping of its own.
- **The load-page boot check ↔ the body's actionable-key rule**: the check pinned at (.andromeda/a11y-plan-detail/screen-reader-support.md:39) is the stand-wide "returns nothing on every stand task" reading of (.andromeda/a11y-plan.md:196). The same test also holds the lint's zero-finding reading, so the refactor must leave both readings unchanged.
