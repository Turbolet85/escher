@history_item n=1 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — (amd-tb5qjhhp) added the Motion rule that `BaseDocument::scroll_into_view` writes every scrolling box holding its target at once, innermost first, with the requested behaviour (Smooth included) applying to the viewport alone, and that the driver's `scroll` is instant inside one settled step. The founder ratified it on 2026-10-07. It is the source of decision/into-view-scroll-nested-boxes-instant, decision/into-view-scroll-instant-in-nested-boxes and decision/driver-scroll-instant. This chunk's seam edit makes `scroll_into_view` read `scroll_state` as `(current, min, max)` and skip a box whose min and max are both zero. Negative scroll ranges must not break the at-once, innermost-first chain.

@history_item n=2 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — (amd-3zi7x754) added the hit walk's edge rule: the walk stops at the padding box of a node that clips by `overflow` (the root excepted), so a point outside reaches none of its hoisted or paint children or inline content, while the node itself still answers inside its border box. Until then a hit reached scrolled-out content. This is the origin of decision/hit-test-stops-at-paint-clip and of the "hit stop" and own-scroll readings the scope keeps against upstream's `hit_inner` `matches_self` and `physical_unrounded_geometry` changes.

@history_item n=3 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — (amd-dkesqwkm) the hit-testing bullet now records the measured answers at an element's own box (img, iframe srcdoc, filled text input), and that a box both transformed and scrolled is hit where its rows show, not through its scrolled-out content, in both layout modes and both builds. These are readings the reversed-scroll-origin take must leave unchanged, because the take moves the scrolled-box geometry those hits read.

@history_item n=4 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (amd-ziwifmdv) widened the hit stop to a box paint clips by `overflow` or by a computed `contain` that includes `paint`. This is the current wording of decision/hit-test-stops-at-paint-clip and decision/hit-testing-stops-at-paint-clip. A later entry under the same marker (amd-mgqkxn5f, Epoch 2 close) records the founder's ratification on 2026-10-18. The take's `hit_inner` conflict resolution has to keep both clip causes and the root exception.

@history_item n=5 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (amd-oyq4dzvu) the harness drives the overlay scrollbar fade, smooth scrolling and fling from the documents' `VirtualClock`, and after a driver `scroll` `animating` reads true until 700 ms pass. The founder ratified it on 2026-10-18. This chunk touches `node/scrollbar.rs` and `scrolling.rs` under these readings, which back decision/shown-scrollbar-takes-thumb-drag and decision/smooth-scroll-behavior-and-wheel-cancel.

@history_item n=6 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (amd-b4kbnljy) an upstream engine change for every document, ratified by the founder: the hit walk skips a node dropped since the last resolve, as the paint walk does, and the `node.rs` hit citations were re-pointed. This is the precedent for this chunk's `ebacd68bd`, which hit-tests nodes that have no computed styles yet. It is also the shape this chunk's master amendment will likely take: an upstream hit-walk change recorded in §Depth Strategy, with the `node.rs` cites re-pointed.

@history_item n=7 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (amd-3v5nd3wr) the Motion item says that right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag, so a driver click there needs an `advance` past the fade first. The Cli item lists the six served tasks. These are the sources of decision/shown-scrollbar-takes-thumb-drag and contract/escher-session-tasks/counter. The take edits `node/scrollbar.rs` and `events/pointer.rs`, so the band and the thumb geometry must read the same under the new scroll origin.

@history_item n=8 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — Four entries under this marker bear on this chunk:
- amd-qfsnm3cq: `Node::hit_inner` answers no hit for a subtree under a non-invertible transform.
- amd-dyuawyah: `BaseDocument::visible_region` reads each clipping box through its transforms.
- amd-7kcbfemw: the Cells grid brings its current cell into view with `MountedData::scroll_to(Instant)`, and the standing into-view lines are kept.
- amd-kio4ik2d: the precedent for re-pointing citations into `node/node.rs`, `scrolling.rs` and `document.rs` when only line numbers moved.

The take edits `hit_inner` and the scroll and clip geometry these lines read.

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — The select-list entries under this marker are the source of the select rows this chunk is bound by:
- amd-5gwesam7: the open list answers hits first in `hit_with_scrollbar`, before the page's walk, and a box clipping the select clips no row (decision/open-select-list-hit-first and the except clause of decision/hit-testing-stops-at-paint-clip).
- amd-k65qa2rh: into view on a list row writes only the list's `scroll_y`, moving no other box and no viewport. This is the except clause of decision/into-view-scroll-instant-in-nested-boxes.
- amd-o2g5q66t: wheel and key on the list, and the list has no motion.
- amd-soij6nn6: `place` keeps the list inside the viewport and every edge finite.
- amd-aheturkj: `visible_region` answers a list row with the list's scrollport.
- amd-4hyhz7fd and amd-t52bkxyn: the list as an engine overlay; its open, choose and close events, including closing when its select is hidden or made inert.
- amd-turucjlk: a list row's `bounds` in the snapshot, and a screenshot of a row.
- amd-bbqwdmq6: the list's thumb takes no hit.

The take's panic in `a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes`, and its `is_scroll_container` fix in `writing_mode.rs`, sit on these readings. The same marker's amd-66qpcjm5 re-pointed citations into `scrolling.rs`, `document.rs` and `node.rs`.

@history_item n=10 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — Three entries under this marker bear on this chunk:
- amd-qddboe2v: a focused select whose anonymous block was dropped when its container lost its box keeps focus and takes a key with no panic. This is decision/boxless-focused-select-keeps-focus, the very path that upstream's new overflow read panicked on («layout_style() called on a node without computed styles»), which the private `is_scroll_container` guard now covers.
- amd-hj32qs3s: the stacking-root list behind decision/stacking-context-groups-paint-and-hit.
- amd-5ng7rtix: the precedent for an upstream take's citation-only re-points across `node/node.rs`, `document.rs` and `select_popup.rs`.

@history_item n=11 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (amd-btcjq4vl) on every Dioxus document, `MountedData`'s `scroll_to`, `scroll`, `get_scroll_offset`, `get_scroll_size` and `get_client_rect` can be called during a render. These are scroll-offset and size readings that a scroll range able to run negative could change, so they should be checked against the take.

@history_item n=12 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — (amd-ruexogdi) turned the master into rows: 137 rows, each verified against the code at edd0cfaea7f0. Every picked row's wording, including the duplicated twin hit-test and into-view rows across depth-strategy, motion and motion-calibrated, comes from this conversion. The chunk's amendments to the engine's scroll and hit readings will be row edits (`~ handle`).
