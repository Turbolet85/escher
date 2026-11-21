@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-har
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summa
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=14 class=stale
leaf-text: the registry growing for the document's life since a dropped boundary cannot be told.
source: ctr-ixzsilx6 «dropped boundaries are pruned before each render, so one dropped in a step reads till the next render.»
replace-with: dropped boundaries pruned before each render, so one dropped in a step reads till the next render.
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=31 class=omission
leaf-text: Cells still calls its handles synchronously, kept, the hazard not needing it.
source: dec-s4kltjgf «except: a Dioxus task that wakes its own waker while it waits livelocks inside the render, asserted by no check, so a task that waits for a later render leaves its waker to the producer»
replace-with: Cells still calls its handles synchronously, kept, the hazard not needing it; a Dioxus task that wakes its own waker while it waits livelocks inside the render, asserted by no check, so a task that waits for a later render leaves its waker to the producer.
