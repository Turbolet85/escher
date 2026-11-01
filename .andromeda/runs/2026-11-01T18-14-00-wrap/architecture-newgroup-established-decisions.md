# New groups — architecture · established-decisions

## Verdicts
- fanout-architecture--cascade-g2.md 1 · distinct · line 39 lists the prefs set, and no group names `init_stylo_prefs`, its `Once`, its two callers or the 0.22 names kept over upstream's
- fanout-architecture--cascade-g2.md 2 · contradiction · lines 45–45 · «a `<link>` starts loading once it has both `rel=stylesheet` and an `href`» · the line has any `<link>` with both load, where the proposal has one with a bad `href` start no load; its failed, empty or non-CSS clause meets no standing line · its own line rewritten
- fanout-architecture--cascade-g2.md 3 · distinct · the unsupported list at line 57 does not name `background-clip: text`, and no group records `ac6f9f582`
- fanout-architecture--cascade-g2.md 8 · distinct · no group states `deallocate_anonymous_block` or the key-press panic's fix; line 123's `flush` reading `is_inert()` stands consistent with it
- fanout-architecture--cascade-g2.md 12 · distinct · line 88 counts six other upstream behaviour changes, and no group names these twelve commits or their ratification
- fanout-architecture--cascade-g2.md 13 · distinct · no group names `AGENTS.md` or `22f0cd000`
- fanout-architecture--cascade-g2.md 14 · distinct · no group states `<picture>` or a `picture` rule; line 129's `display: contents` is `optgroup`'s
- fanout-architecture--cascade-g2.md 15 · contradiction · lines 101–101 · «display/float/position/contain/visibility/font/text-shaping changes yield `ALL_DAMAGE`, an in-flow `order` change `RELAYOUT | REORDER_CHILDREN`, else `RELAYOUT`» · the line leaves a padding or border change at `RELAYOUT`, where the proposal has one on a table box take `ALL_DAMAGE` · its own line rewritten
- fanout-architecture--cascade-g3.md 1 · twin · fanout-architecture--cascade-g2.md 14 · the same removal of upstream's `picture` rule, proposed as a new row under [Layout] and not at the end of line 101
- fanout-architecture--cascade-g3.md 2 · contradiction · lines 101–101 · «display/float/position/contain/visibility/font/text-shaping changes yield `ALL_DAMAGE`, an in-flow `order` change `RELAYOUT | REORDER_CHILDREN`, else `RELAYOUT`» · the line leaves a padding or border change at `RELAYOUT`, where the proposal has one on a table box take `ALL_DAMAGE`; the same fact as fanout-architecture--cascade-g2.md 15, but this new row at line 109 leaves line 101 standing unedited
- fanout-architecture--cascade-g3.md 3 · distinct · line 161 culls non-finite transformed boxes, a different rule, and no group states `render_element`'s border box and culling on the scaled size
- cascade-compare-architecture.md 3 · twin · fanout-architecture--cascade-g2.md 8 · the same panic's fix through `deallocate_anonymous_block`, proposed at another place with its tests and upstream's own panic added
- cascade-compare-architecture.md 4 · distinct · line 167's registered extensions name `Url` alone, no group states `URLSearchParams` or its native helper's deletion, and fanout-architecture--cascade-g2.md 12 only calls it a realization
- cascade-compare-architecture.md 5 · distinct · no group states `flush_queued_mounted_events` or its guard; fanout-architecture--cascade-g2.md 12 names `6ef4b532d` only as a commit, with none of this fact
