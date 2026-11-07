@answer n=1 entity=established-decisions#1.1 v=no
why: pins of markup5ever, taffy, parley, winit and the patch blocks; the chunk moves no dependency and keeps Dioxus at 0.7.3
@answer n=2 entity=established-decisions#2.1 v=no
why: build profiles; the chunk adds no profile and no config
@answer n=3 entity=established-decisions#3.1 v=no
why: release versioning; no version moves in the chunk
@answer n=4 entity=established-decisions#4.1 v=no
why: renderer backends and paint_bench; the chunk paints nothing and selects no renderer
@answer n=5 entity=established-decisions#5.1 v=yes
why: dioxus-native-dom's default features, accessibility included, are the set the stand_* snapshot checks and the new buffer module build and run under
@answer n=6 entity=established-decisions#6.1 v=yes
why: incremental layout defaults to true, and the scenarios run both layout modes, so the default-built bridge document is one of the modes re-measured
@answer n=7 entity=established-decisions#7.1 v=no
why: render-blocking stylesheets and resource loading; the replay makes the same mutator calls as today, and resource loading is not needed by this chunk
@answer n=8 entity=established-decisions#8.1 v=no
why: CSS approximations, the CSSOM 1/64px snap and alignment rules; the chunk reads no style or CSSOM geometry
@answer n=9 entity=established-decisions#9.1 v=no
why: unsupported features and pseudo-class matching; the chunk's focus checks read the focused node, not style matching
@answer n=10 entity=established-decisions#10.1 v=yes
why: the scope names MountedData get_client_rect, scroll_to, scroll, get_scroll_offset and get_scroll_size, whose results the shared bounds reader and scroll clamp rule carry, and the accessibility builder's selected, toggled and invalid write is what the snapshot checks read
@answer n=11 entity=established-decisions#11.1 v=yes
why: the client rect read where an element's transforms move it is the rule carried by the get_client_rect call the scope names
@answer n=12 entity=established-decisions#11.2 v=yes
why: the focus fixup and the tree's focus falling back to the window are the focus rules and removed-node interaction state the replayed buffer must keep
@answer n=13 entity=established-decisions#11.3 v=no
why: a dropped anonymous block's layout_parent for hidden, display none and CSS-driven drops; the chunk's removal is the bridge's own and the chunk runs no key press
@answer n=14 entity=established-decisions#11.4 v=no
why: the document clock for engine time reads; the chunk adds no time read and its scroll calls are checked for no panic and focus, not timing
@answer n=15 entity=established-decisions#11.5 v=no
why: the single select's keys, typeahead and apply_select_choice; the chunk drives no keys or selects
@answer n=16 entity=established-decisions#11.6 v=no
why: the select list's opening by click, Enter or Space; not named in the chunk's scope
@answer n=17 entity=established-decisions#11.7 v=yes
why: an open select list closes on removal of its select, and the chunk's replayed removal must clear interaction state on removed nodes
@answer n=18 entity=established-decisions#11.8 v=no
why: Tab, Enter and row choice on an open list; key handling the chunk does not touch
@answer n=19 entity=established-decisions#11.9 v=no
why: rejected and unbuilt select list alternatives; states no behaviour the chunk must honour
@answer n=20 entity=established-decisions#11.10 v=no
why: the select list's standing limits recorded in the docs; no code the chunk touches
@answer n=21 entity=established-decisions#11.11 v=no
why: the merge surface is sixteen blitz-dom files, none of which is among the chunk's reached files
@answer n=22 entity=established-decisions#11.12 v=no
why: multiple select keys; key handling the chunk does not touch
@answer n=23 entity=established-decisions#11.13 v=no
why: multiple select pointer handling; not named in the chunk's scope
@answer n=24 entity=established-decisions#11.14 v=no
why: upstream behaviour changes for JavaScript, IME, the windowed redraw route and accessors; the chunk's redraw stays DocumentMutator's, and the windowed path is read, not run
@answer n=25 entity=established-decisions#11.15 v=no
why: twelve upstream commits already taken before this chunk; the chunk changes none of the files they reach
