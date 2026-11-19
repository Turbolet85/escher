@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dir
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/sta
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=40 class=retired
leaf-text: - Event-producing functions take a `dispatch_event` callback.
source: dec-iouqf4lq «Engine and user-agent events are queued on the document and dispatched by the event driver's flush of pending events, never through a callback passed to the producer; queued events for dropped nodes are discarded.»
replace-with: - Engine and user-agent events are queued on the document and dispatched by the event driver's flush of pending events, never through a callback passed to the producer; queued events for dropped nodes are discarded.
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=12 class=stale
leaf-text: Stylo 0.22, Taffy 0.15
source: dec-qy3gqtf5 «Servo Stylo 0.24, with selectors and cssparser, does styling and selector matching.»
replace-with: Stylo 0.24, Taffy 0.15
@leaf_edit n=3 leaf=.claude/docs/services/blitz-dom.md line=12 class=stale
leaf-text: Parley 0.12 (registry)
source: dec-qvdsgthp «Parley 0.13 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.»
replace-with: Parley 0.13 (registry)
@leaf_edit n=4 leaf=.claude/docs/services/blitz-dom.md line=21 class=omission
leaf-text: an engine change for every document. Every box-generating
source: dec-6beca3l4 «An inline-box placeholder carries a private engine fallback font family registered in every font context, so an inline-block inside text no font can shape is placed once its own font loads.»
replace-with: an engine change for every document. An inline-box placeholder carries a private engine fallback font family registered in every font context, so an inline-block inside text no font can shape is placed once its own font loads — a direct engine change for every document, flagged upstreamable. Every box-generating
@leaf_edit n=5 leaf=.claude/docs/services/blitz-shell.md line=11 class=stale
leaf-text: winit `=0.31.0-beta.3`
source: dec-odccjf3r «winit is pinned exactly to a beta version, 0.31.0-beta.4.»
replace-with: winit `=0.31.0-beta.4`
@leaf_edit n=6 leaf=.claude/docs/services/blitz-shell.md line=18 class=stale
leaf-text: On a poll that reported work `View::poll` takes the document's changed set (`take_changed_nodes`, outside the `accessibility` cfg — a shell built without the feature still drains it) and, under `accessibility`, rebuilds the platform tree when the set it took was non-empty,
source: dec-pzi6d5tp «On a redraw, after resolve, the shell hands the adapter a fresh tree built from the wrapping document when the document's changed set is non-empty, then clears the set; a poll refreshes nothing.»
replace-with: On a redraw, after resolve, the shell, under `accessibility`, rebuilds the platform tree from the wrapping document when the document's changed set is non-empty and then clears the set — a poll refreshes nothing —
