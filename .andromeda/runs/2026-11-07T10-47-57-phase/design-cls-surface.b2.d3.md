@answer n=1 entity=surface#6.1 v=no
why: the macOS titlebar style is set by the browser app's window attributes, which the buffered render does not touch
@answer n=2 entity=surface#6.2 v=no
why: the transparent example's window size and decorations are window config outside the render path the chunk changes
@answer n=3 entity=surface#6.3 v=no
why: compositing alpha mode is launch config and renderer selection, not the document rendering the buffer changes
@answer n=4 entity=surface#7.1 v=yes
why: the web-spa need names the wasm surface rendering into a canvas on the page, and this entity states that canvas surface
@answer n=5 entity=surface#7.2 v=no
why: the scrollbars feature forwards to blitz-paint for painting only, which the chunk does not touch
@answer n=6 entity=surface#7.3 v=no
why: select-list thumb and row scrolling are select-list behaviour the chunk does not touch, and no chunk test scrolls a select list
@answer n=7 entity=surface#8.1 v=no
why: the mobile urlbar font and padding are browser CSS, outside document identity and the render path
@answer n=8 entity=surface#9.1 v=no
why: the mobile class drives urlbar CSS sizing, a browser UI component the chunk does not change
@answer n=9 entity=surface#10.1 v=no
why: the Android back button is browser navigation logic the chunk does not touch
@answer n=10 entity=surface#11.1 v=no
why: mobile screenshot file naming is capture behaviour the chunk does not touch
@answer n=11 entity=surface#12.1 v=yes
why: need 6 names mobile-native and the shared buffer code must still build for iOS and Android, which CI builds but never tests
@answer n=12 entity=surface#13.1 v=no
why: wpt runner result colours are CLI styling outside the reached files, and buffer content is never printed
@answer n=13 entity=surface#13.2 v=no
why: wpt runner heading and flag colours are CLI styling outside the reached files
@answer n=14 entity=surface#13.3 v=no
why: OSC 8 hyperlinks in wpt runner output are CLI output the chunk does not print to
@answer n=15 entity=surface#14.1 v=no
why: the paint_bench output line is CLI output the buffer never reaches, since buffer content is never printed
@answer n=16 entity=surface#14.2 v=no
why: the screenshot command's printed lines are CLI output the buffer never reaches
@answer n=17 entity=surface#15.1 v=no
why: the escher-session JSON answer contract is driver output the buffer never reaches
@answer n=18 entity=surface#15.2 v=no
why: the seven_guis task list is a CLI usage line the chunk does not touch
@answer n=19 entity=surface#16.1 v=yes
why: the web-spa need names what the wasm surface paints, and this entity states the font context on WASM that such paint depends on
@answer n=20 entity=surface#17.1 v=no
why: the wasm_hello host page canvas size, radius and outline are example page styling the scope does not name
