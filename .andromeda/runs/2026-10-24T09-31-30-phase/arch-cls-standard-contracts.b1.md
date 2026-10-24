# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the changed set that `take_changed_nodes` drains, fed by the mutator's insert (a predicted conflict), and `DocumentConfig` in config.rs, which upstream's Parallel traversal edits
standard-contracts#2.1 · yes · the document clock (`DocumentConfig.clock`, `Clock`, `WallClock`, `VirtualClock`), re-read on config.rs, document.rs, scrolling.rs and events/pointer.rs, all reached by upstream's delta
standard-contracts#2.2 · yes · `select_value` reads the select model's choice (`CHECKED`), one of the five carried merge surfaces the task names
standard-contracts#3.1 · yes · `doc.mutate()` and `build_accessibility_tree` with its state reads, which bind through the Stylo 0.23 move touching accessibility.rs
standard-contracts#4.1 · yes · the events contract (keyboard activation, a range input's `input`, a select's `input` then `change`), which binds through upstream's edits to events/pointer.rs and the files around it
standard-contracts#5.1 · yes · `scroll_into_view`, `visible_region`, the clipping-box hit, `Node::absolute_position` and `NodeTree` stale-id panics, which bind through the scrolling.rs conflict and upstream's node/node.rs hit-testing change
standard-contracts#6.1 · yes · `get_client_bounding_rect` and its readers, named in the scrolling-and-input need, which bind through the scrolling.rs conflict
standard-contracts#6.2 · yes · `text_extent` reads inline layout data (block offset, Parley size, inline-root test) that upstream's inline layout module split reaches, so its existing check must still pass
standard-contracts#7.1 · yes · `LayoutPassState` and its Taffy trait impls, named in the layout need, with the inline layout module split, the layout/construct.rs conflict and the Taffy-main commits behind stylo_taffy
standard-contracts#8.1 · yes · the `ElementData` accessors and the `SpecialElementData` variants, where upstream's class atoms land and the nodes-and-widgets need names
standard-contracts#9.1 · yes · `ElementData::is_disabled`, named in the DOM-semantics need as read on form controls alone
standard-contracts#9.2 · yes · the crate-private `node::select` rules module, named in the DOM-semantics need and part of the carried select model
standard-contracts#10.1 · no · the task's Not-needed list puts the provider traits outside the take (`NetProvider` named) and says upstream's delta does not reach them
standard-contracts#11.1 · no · the task's Not-needed list puts the HTML and networking crates outside the take (`HtmlDocument` and blitz-net's `Provider` named) and says upstream's delta does not reach them
standard-contracts#12.1 · yes · the password input's masked paint in blitz-paint's render.rs, the painting merge surface the task names
