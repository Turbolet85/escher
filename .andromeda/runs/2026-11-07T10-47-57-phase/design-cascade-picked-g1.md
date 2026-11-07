@account n=1 entity=color-palette#1.6 v=not_in_scope
why: the frame clear colour lives in dioxus-native's config and renderer, which the chunk only uses and whose clear colour no scope line names
@account n=2 entity=color-palette#1.7 v=not_in_scope
why: blitz-paint's canvas background is painted by code the chunk leaves alone, and no re-asserted check reads a colour
@account n=3 entity=color-palette#8.6 v=not_in_scope
why: SVG serialization's currentColor rule is blitz-dom's own, untouched and unnamed by the scope
@account n=4 entity=typography#1.6 v=not_in_scope
why: default.css form-control fonts are untouched, and the ported test pins no geometry value measured at that family
@account n=5 entity=typography#1.12 v=not_in_scope
why: the text editor's base size is blitz-dom's own, and no check the chunk re-pins measures an input's size
@account n=6 entity=typography#1.13 v=not_in_scope
why: the engine generic base sizes are untouched and read by no value the chunk re-pins
@account n=7 entity=typography#2.1 v=not_in_scope
why: the wasm example's bundled fonts are untouched; the buffered render changes no font registration
@account n=8 entity=typography#2.3 v=not_in_scope
why: the FontContext config of dioxus-native is only used, and no scope line names it
@account n=9 entity=typography#2.4 v=not_in_scope
why: dioxus-native's font features are unchanged and no dependency moves
@account n=10 entity=typography#2.8 v=not_in_scope standing=S1
why: a test in the blitz-tests crate builds blitz-dom with the system-fonts feature, so text measures and font-dependent geometry assertions do not pass vacuously
@account n=11 entity=typography#3.1 v=not_in_scope
why: the generic-family mapping is stylo_to_parley's own, untouched and unnamed by the scope
@account n=12 entity=typography#3.4 v=not_in_scope
why: the line-height mapping is untouched and no check the chunk re-pins measures it
@account n=13 entity=spacing#2.5 v=not_in_scope
why: default.css body margin is untouched and no re-pinned value reads it
@account n=14 entity=depth-strategy#3.2 v=not_in_scope
why: the open select list's hit order is blitz-dom's own, and no select list is reached
@account n=15 entity=depth-strategy#3.4 v=not_in_scope
why: visible_region is untouched and no scope line names it
@account n=16 entity=border-radius#2.3 v=not_in_scope
why: the img border attribute mapping is stylo's own and untouched
@account n=17 entity=motion#1.2 v=carried
why: the scope names scroll_to and scroll as MountedData calls now reachable during render, and a Smooth one runs this scroll animation
@account n=18 entity=motion#1.4 v=carried
why: it states the Smooth and Instant behaviours of dioxus-native-dom's programmatic scrolling, the scroll_to and scroll calls the scope names
@account n=19 entity=motion#1.5 v=carried
why: the stand's Cells grid calls MountedData::scroll_to, a call the scope names, and the stand_* checks the chunk re-asserts witness it
@account n=20 entity=motion#1.6 v=carried
why: the into-view scroll rule is what scroll_to reaches, and the task requires it to stay as it is once reachable during render
@account n=21 entity=motion#1.8 v=not_in_scope
why: the overlay scrollbar fade timings are blitz-dom's own, untouched and unnamed by the scope
@account n=22 entity=motion#1.9 v=not_in_scope
why: the scrollbar's band hit after a scroll is blitz-dom's own, and the re-asserted behaviour names no scrollbar
@account n=23 entity=motion#2.2 v=not_in_scope
why: the harness settle's answer over the animating set is a component the chunk only drives, its answer unnamed by the scope
@account n=24 entity=motion#3.1 v=not_in_scope standing=S2
why: a harness-driven test reads animation time only through the VirtualClock its documents read; settle never advances it, only advance moves it
