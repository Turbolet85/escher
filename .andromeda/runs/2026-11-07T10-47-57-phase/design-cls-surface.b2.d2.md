@answer n=1 entity=surface#6.1 v=no
why: the macOS titlebar is browser window chrome in main.rs, which the chunk's render path does not touch
@answer n=2 entity=surface#6.2 v=no
why: the transparent example's window size and close behaviour are window config, not document rendering
@answer n=3 entity=surface#6.3 v=no
why: window compositing alpha is launch config the chunk neither sets nor reads, and the scope names only unchanged window behaviour
@answer n=4 entity=surface#7.1 v=yes
why: the scope names the web-spa canvas on the page, which must stay unchanged in what it paints
@answer n=5 entity=surface#7.2 v=no
why: the scrollbars feature forwards to blitz-paint thumbs, and the scope names neither scrollbar paint nor the feature
@answer n=6 entity=surface#7.3 v=yes
why: the rule says the scroll call the scope names via MountedData moves a select list's rows, a nested scrolling box the programmatic scroll rule covers
@answer n=7 entity=surface#8.1 v=no
why: the mobile urlbar's 16px font and padding are host CSS values, not document content the buffered replay carries
@answer n=8 entity=surface#9.1 v=yes
why: the mobile frame class is a document attribute the buffer must replay unchanged for the browser app's documents to stay identical
@answer n=9 entity=surface#10.1 v=no
why: the Android back button is browser navigation, which the chunk changes nothing in (not_needed 9)
@answer n=10 entity=surface#11.1 v=no
why: mobile screenshot save naming is capture behaviour the chunk does not touch
@answer n=11 entity=surface#12.1 v=yes
why: the scope's obligation that mobile-native documents come out identical rests on code CI never tests, so the rule on where mobile code is verified binds
@answer n=12 entity=surface#13.1 v=no
why: wpt runner result colours belong to a separate CLI the chunk neither drives nor prints into
@answer n=13 entity=surface#13.2 v=no
why: wpt runner heading and marker colours belong to a separate CLI the chunk does not reach
@answer n=14 entity=surface#13.3 v=no
why: wpt runner OSC 8 hyperlinks belong to a separate CLI the chunk does not reach
@answer n=15 entity=surface#14.1 v=no
why: paint_bench's printed lines belong to an example the chunk does not drive or change
@answer n=16 entity=surface#14.2 v=no
why: the screenshot example's printed lines belong to an example the chunk neither drives nor changes
@answer n=17 entity=surface#15.1 v=yes
why: the escher-session output contract (one JSON line, no ESC byte, exit classes) is the driver output that need 10 requires the buffer never change
@answer n=18 entity=surface#15.2 v=no
why: the session host's task list and usage line are not touched by the render-path change
@answer n=19 entity=surface#16.1 v=yes
why: the engine states a wasm font context must be provided, a precondition of what the web-spa surface paints, which the scope keeps unchanged
@answer n=20 entity=surface#17.1 v=no
why: wasm_hello's host-page canvas size, radius and tabindex are example page setup outside the document the chunk renders
