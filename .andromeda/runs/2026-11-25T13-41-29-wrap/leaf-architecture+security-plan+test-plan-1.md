@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=6 class=stale
leaf-text: resolving its id as `Session::layout` does, refusing `fonts-unavailable`
source: ctr-elrfiwmi «A read answering a CanvasReading of one canvas in process, resolving its id as the run path does with a virtual id read not-found, refusing fonts-unavailable where the document cannot paint text»
replace-with: resolving its id as the run path does with a virtual id read `not-found`, refusing `fonts-unavailable`
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=18 class=stale
leaf-text: `click` is the one verb taking a virtual node id `{owner}///{key}`, acting on the canvas at the virtual region and refused in the one order `disabled`, `off-screen`, `covered`, `not-found`, `stale` with nothing run, while `type`, `scroll`, `screenshot` with an id, `layout` and `canvas` read a virtual id `not-found`.
source: ctr-7vwjtnb2 «click takes a virtual node id, as layout does: it acts on the canvas at the virtual region and is refused in the one order disabled, off-screen, covered, not-found, stale with nothing run; type, scroll, screenshot with an id and canvas read a virtual id not-found.»
replace-with: `click` takes a virtual node id `{owner}///{key}`, as `layout` does, acting on the canvas at the virtual region and refused in the one order `disabled`, `off-screen`, `covered`, `not-found`, `stale` with nothing run, while `type`, `scroll`, `screenshot` with an id and `canvas` read a virtual id `not-found`.
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=18 class=omission
leaf-text: takes one required `id`, resolved as the run path does (`stale` or `not-found`), and answers where the element sits and why at its size
source: ctr-mxl62qim «Layout on a virtual node id answers it placed by app in its canvas, at the region's offset from the canvas's border box, with the snapshot's bounds, the canvas's shifts and both size causes layout; the engine is never asked for a region's reason.»
replace-with: takes one required `id`, resolved as the run path does (`stale` or `not-found`), and answers where the element sits and why at its size — a virtual node id answered placed by `app` in its canvas, at the region's offset from the canvas's border box, with the snapshot's bounds, the canvas's shifts and both size causes `layout`, the engine never asked for a region's reason
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=18 class=omission
leaf-text: `layout` read with no cause and `canvas` with none on a document that can paint text)
source: ctr-mfokuw36 «A listed virtual node's verbs are read against its canvas, each cause the one its call returns: a verb that takes no virtual id reads not-found, and click and layout meet the checks their call meets.»
replace-with: `layout` read with no cause and `canvas` with none on a document that can paint text; a listed virtual node's verbs read against its canvas, each cause the one its call returns — `click` and `layout` meeting their call's checks, every other verb `not-found`)
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=16 class=omission
leaf-text: handlers inside a canvas are not counted (deferred to 0.4.0)
source: ctr-rq3jq44s «Handlers inside a canvas are not counted: a virtual node a canvas declares carries none, and a click on one counts its canvas's own listener under the canvas's id.»
replace-with: handlers inside a canvas are not counted — a virtual node a canvas declares carries none, and a click on one counts its canvas's own listener under the canvas's id
@leaf_edit n=6 leaf=.claude/docs/services/escher-driver.md line=32 class=stale
leaf-text: while the graph, actions answer, lint, canvas answer and exploration hold none: a ratified boundary widening.
source: dec-eg7d4fr7 «the graph, actions and layout answers carry the id, never the name; the lint and canvas answer hold none.»
replace-with: while the graph, actions and layout answers carry the id, never the name, and the lint and canvas answer hold none: a ratified boundary widening.
@leaf_edit n=7 leaf=.claude/docs/services/escher-driver.md line=62 class=stale
leaf-text: no id of the UI graph, the actions answer or the lint answer holding `///`
source: dec-b5ugo5lk «No id of the lint answer holds the reserved `{owner}///{key}` form, nor of the stand's UI graph or actions answer while the stand declares no virtual node.»
replace-with: no id of the lint answer holding `///`, nor of the stand's UI graph or actions answer while the stand declares no virtual node
@leaf_stands n=1 leaf=.claude/docs/services/escher-driver.md line=19
because: The line says only that a `layout` call answers the `LayoutExplanation` `Session::layout` reads, which the canvas row's change (now resolving its id as the run path does) leaves true.
@leaf_stands n=2 leaf=.claude/docs/services/escher-driver.md line=37
because: The line says a `Session::layout` read records the screen's ids in the session's record, which the canvas row's change does not touch.
@leaf_stands n=3 leaf=.claude/docs/services/escher-driver.md line=49
because: The line says `Session::layout` opens no span, and the canvas row still says the canvas read opens no span of its own; only its id resolution changed, which this line does not state.
