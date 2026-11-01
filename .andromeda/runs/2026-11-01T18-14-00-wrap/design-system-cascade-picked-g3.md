# design-system · picked accounting · group 3 of 4

## Picked
depth-strategy#1.6 · not touched · the report has `default.css` only losing lines, upstream's `picture` rule alone among rules named, so the dialog backdrop's value still holds
depth-strategy#2.2 · proposed · the report changes this line, `mix-blend-mode` other than `normal` joining the stacking roots through `Node`'s arm (F1); the atomic list stands, stacking being the measured half
depth-strategy#3.1 · not touched · the report adds no line to `render.rs` between :634 and :1293 or to `node/node.rs` past :1384, and `select_popup.rs` only at :513, so paint order and the hit walk hold, a blended root reading under the same rules
depth-strategy#3.2 · not touched · no added line of `document.rs` falls in its hit-first spans and `stand_act_popup` is not reached, so the list still answers first
depth-strategy#3.3 · not touched · `node/node.rs`'s additions stop at :531-538 and :1382-1384, the stacking arm, and its check of a non-invertible transform is not among them, so the rule holds
depth-strategy#4.1 · not touched · `paint_order.rs` is not reached and the report moves no `z-index: auto` level, its paint-order test green under `run all`
depth-strategy#4.2 · proposed · the report re-reads the list as still last, now past the blend layers, which this line does not yet say
depth-strategy#4.3 · not touched · `box_shadow.rs` is no merge surface and its one added line, :41, sits outside the clip and radius spans, so the outset rule holds; the inline-box skip is F4, beside #4.4
depth-strategy#4.4 · proposed · its inset rule holds outside `box_shadow.rs:41`, and F4's inline box painting no shadow of its own goes beside the shadow rows here
depth-strategy#4.5 · proposed · F2 and F3 go beside it; its own opacity, filter and backdrop-filter clause is unedited, the report's blend layer being the viewport-wide one at `render.rs:485-506`
border-radius#1.5 · not touched · the report has `default.css` only losing lines, the `picture` rule named and no button rule, so the 1px default holds
border-radius#1.6 · not touched · `form_controls.rs` is not among the reached files and the report moves no checkbox frame
border-radius#1.7 · not touched · the report adds no line to `render.rs` between :634 and :1293, where the thumb's radius stands, so it holds
border-radius#2.1 · not touched · no line of `render.rs` is added past :1293, so the radii's resolve holds, the take's scaled-size move being `render_element`'s border box and culling
border-radius#2.3 · not touched · `stylo.rs` gained only :231-238 and :771-776, neither the `border` attribute's mapping, so it holds
motion#1.3 · not touched · no added line of `document.rs` falls in its wheel span and `select_popup.rs` moved only at :513, so the wheel and key rules hold
motion#1.8 · not touched · no added line of `document.rs` or `render.rs` falls in the fade's spans and `scrollbar.rs` and `resolve.rs` are not reached, so the timings hold
motion#2.1 · not touched · `stylo.rs` gained only :231-238 and :771-776, the resolve context's scope and `borrow_data`'s filter, so the animation phases hold
motion#2.2 · not touched · `settle.rs` gained the counter's forward and its test and the settle rule is unedited, `custom_widget.rs` gained only :83-84 and `document.rs` nothing in the animating span, so the set holds
motion#4.1 · not touched · `escher-driver` is untouched and the clock claim holds, «at most `LOAD_WAIT`» staying true when a failed load ends the wait sooner, a sentence the report owes to the harness and driver contracts
iconography#1.1 · not touched · no added line of `document.rs` falls in the cursor's spans, so the keyword order holds
iconography#1.3 · not touched · no added line of `document.rs` falls in `favicon_url`'s span, so it holds
