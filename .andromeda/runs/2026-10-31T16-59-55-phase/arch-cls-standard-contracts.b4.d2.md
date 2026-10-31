# Classified — arch · standard-contracts.b4

## Answers
standard-contracts#21.1 · yes · the two committed kept checks stand_replay replays are written under this recording rule (answers kept whole, the record shape, the bounds, strict Check::from_bytes), and the snapshot change alters what they hold
standard-contracts#21.2 · yes · stand_replay runs Check::replay over those two kept checks, which stops at the first step whose kept answer differs, so the closed select options' new bounds in snapshot answers bind
standard-contracts#21.3 · yes · the overlap reading skips a closed select's options by closed_select_of through the element, which the chunk restates for an open list's own boxes and edits in lint.rs; Session::lint reads and changes nothing
standard-contracts#21.4 · yes · the acting verbs (click on an option, the executor's settled step) wait on loads through settle_waiting, while snapshot and lint wait on nothing; the chunk edits execute, lint and snapshot
standard-contracts#22.1 · yes · the closed-select overlap skip, stand_lint's at_boot [0, 0, 0, 0] and the multiple rows' client rects; the chunk edits lint's overlaps exclusion and the stand_lint table
standard-contracts#22.2 · no · the load-page witness (stand_load_page.rs, load-fetch) is a check the chunk does not reach or touch; the chunk's acts settle through the same wait, which 21.4 carries
standard-contracts#23.1 · no · blitz-vibey-script's JS dispatch and JS preventDefault feedback concern script listeners; the chunk adds no script surface and its handlers are Rust or Dioxus
standard-contracts#24.1 · no · document_may_load governs JS fetch, imports and script loads; the chunk adds no script surface and loads nothing
standard-contracts#25.1 · no · CSSStyleDeclaration accessors are script-side; the chunk reads no CSS property from script and adds none
standard-contracts#26.1 · yes · the windowed path the scope names (the list shown in dioxus-native's window through the engine overlay) runs through launch and launch_cfg and the DioxusNativeWindowRenderer that paints it; the Config types stay unchanged
