@answer n=1 entity=cross-cutting-patterns#1.1 v=no
why: the new mutation_buffer module sits outside every accessibility gate, so the gated module list this row names still holds and no manifest moved
@answer n=2 entity=cross-cutting-patterns#2.1 v=no
why: the session adds no log site, print or span, and the replay re-fires the writer's own trace calls
@answer n=3 entity=cross-cutting-patterns#3.1 v=yes mode=add-to
why: the bridge still shares its document as Rc<RefCell<BaseDocument>> but now records mutations and replays them under one borrow, a new borrow-workaround shape beside the child-list and layout take-outs
@answer n=4 entity=cross-cutting-patterns#4.1 v=no
why: the session does not reach blitz-dom's text editor construction or the windowed redraw route, and the redraw rules are unchanged
@answer n=5 entity=cross-cutting-patterns#4.2 v=no
why: the stylesheet redraw-skip path lives in blitz-dom net.rs, which the session does not touch
@answer n=6 entity=cross-cutting-patterns#5.1 v=yes mode=stale
why: the citations into dioxus_document.rs for the mounted listeners and the zero-handler skip moved under the 52 added lines, while the rules themselves still hold
@answer n=7 entity=cross-cutting-patterns#6.1 v=yes mode=stale
why: flush_queued_mounted_events, cited at dioxus_document.rs:242-246, sits after the added lines so its citation moved, while the skip rule still holds
@answer n=8 entity=cross-cutting-patterns#7.1 v=no
why: its cited blitz-dom changed-set and invalidation sites are untouched, and the writer's calls reach them as before
@answer n=9 entity=cross-cutting-patterns#8.1 v=no
why: its restyle sites live in blitz-dom mutator.rs, which the session does not touch, and the writer stays byte-identical
