# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · no · dependency pinning is Not needed (the chunk adds no dependency), and the root Cargo.toml pins are unreached, so their coupling is only a re-check every change owes
established-decisions#2.1 · no · build profiles are Not needed (the chunk adds no profile), and the root Cargo.toml profile tables are unreached, so the dev inheritance is only a re-check every change owes
established-decisions#3.1 · no · release versioning is Not needed (the chunk cuts no release), and the workspace version and publish-browser.yml are unreached, so nothing here is more than a re-check every change owes
established-decisions#4.1 · yes · the Needs name the renderer-backend decision and cross-OS note: the harness screenshot renders through VelloCpuImageRenderer over opaque white, and a byte-for-byte cross-OS check holds no screenshot step
established-decisions#5.1 · yes · the Needs name the default-features decision: accessibility is default and gates the snapshot modules the witnesses read, and writing-mode is in no default list, so the vertical-text check must opt in to it
established-decisions#6.1 · yes · the Needs name the single incremental pipeline (default true, both modes checked) and the stand's pinned Light viewport, and the writing-mode pref binds the vertical-text check
established-decisions#7.1 · no · resource loading is Not needed (the stand and fixtures boot offline), and its render-blocking, depth-cap and image-cache rules live in unreached blitz-dom files
established-decisions#8.1 · no · CSS approximations are Not needed (no sweep reading rests on them), and their 1/64px snap, flex alignment, table and scrollbar mappings live in unreached blitz-dom and stylo_taffy files
established-decisions#9.1 · no · unsupported features are Not needed (no sweep reading rests on them), and their 3D, container-query and pseudo-class limits live in unreached blitz-dom and stylo_taffy files
established-decisions#10.1 · yes · the Needs name scroll_into_view through every scrolling box, the shared client rect, the hit stopping at a clipping box's padding box, and the range, select, keyboard-activation and click-focus models the named witnesses re-prove
established-decisions#11.1 · yes · the Needs name the inert and disabled-on-form-controls engine changes, the password click that places the caret by its bullets, and the client rect read where transforms move it, each an engine change for every document
established-decisions#11.2 · yes · the Needs name the document's clock through DocumentConfig.clock, witnessed by the virtual clock checks and by the double-click window read on that clock
established-decisions#11.3 · yes · the Needs name the select model (arrows, Home and End, typeahead, input then change through one apply path), witnessed by the select checks the record names
established-decisions#12.1 · yes · the Needs name RangeInputData (seeded, kept across re-construction, cleared off type=range) and a text input's Parley editor behind the password caret, while its table, replaced, damage and z-order clauses are not reached
established-decisions#13.1 · no · a replaced element's aspect-ratio height is not needed (no sweep reading rests on img or svg, and inline SVG is Not needed), and its rule lives in the unreached blitz-dom replaced-layout file
established-decisions#14.1 · yes · the Needs name ElementData::is_disabled and disabled read on form controls alone, and the Dioxus removal that clears a removed node's element-id mapping so it no longer resolves
established-decisions#15.1 · yes · the Needs name inert honoured on an element or an ancestor (not focusable, no hit, out of the tree and snapshot), an engine change for every document, witnessed by the inert_subtree checks
established-decisions#15.2 · no · the Needs name no focus-within reading or changed-set marking on a focus move, the keyboard witnesses rest on the focus move carried by 10.1 and 14.1, and these rules live in the unreached blitz-dom document file
established-decisions#15.3 · yes · the Needs name the select model, and this entry holds its choice as CHECKED, its option rules, the mutator's settling of recorded selects and the snapshot's selected reading of an option
established-decisions#16.1 · no · the Needs name no hit on a dropped node, the removal itself is carried by 14.1, and the rule lives in the unreached blitz-dom node, document and paint-tree files
established-decisions#17.1 · no · HTML parsing is Not needed (no sweep reading rests on the parse setting, doctype, XHTML or template handling), and its UA-sheet clause rides DocumentConfig::default(), which the chunk leaves alone
established-decisions#18.1 · no · shell and windowing are Not needed (the sweep is headless and its windowed witness is owned elsewhere), and the window, IME and accesskit_xplat rules live in unreached files
established-decisions#19.1 · yes · the Needs name the painting decision behind the password caret witness, a password input painted masked in every document, while its layer, culling, gradient and border clauses are not reached
