@answer n=1 entity=color-palette#3.7 v=no
why: main background of the custom-widget and html examples; the probe paints nothing and reads no example background
@answer n=2 entity=color-palette#4.1 v=no
why: browser tooltip and status bar text colour; browser chrome is not on the seven_guis probe path
@answer n=3 entity=color-palette#4.2 v=no
why: browser about-page muted text colours; not read by the probe
@answer n=4 entity=color-palette#4.3 v=no
why: browser error page paragraph colour; not on the probe path
@answer n=5 entity=color-palette#4.4 v=no
why: todomvc body and heading colours; the probe runs seven_guis, not todomvc, and paints nothing
@answer n=6 entity=color-palette#4.5 v=no
why: wasm_hello heading and code colours; the web-spa surface is outside the chunk
@answer n=7 entity=color-palette#5.1 v=no
why: disabled checkbox and radio accent; the probe drives no checkbox or radio and paints nothing
@answer n=8 entity=color-palette#6.1 v=no
why: WPT runner PASS terminal colour; CLI terminal colours are not_needed item 10
@answer n=9 entity=color-palette#6.2 v=no
why: WPT runner FAIL-with-passing-subtests terminal colour; CLI terminal colours are not_needed item 10
@answer n=10 entity=color-palette#6.3 v=no
why: WPT runner FAIL terminal colour; CLI terminal colours are not_needed item 10
@answer n=11 entity=color-palette#6.4 v=no
why: WPT runner TIMEOUT terminal colour; CLI terminal colours are not_needed item 10
@answer n=12 entity=color-palette#6.5 v=no
why: WPT runner SKIP terminal colour; CLI terminal colours are not_needed item 10
@answer n=13 entity=color-palette#6.6 v=no
why: WPT runner CRASH terminal colour; CLI terminal colours are not_needed item 10
@answer n=14 entity=color-palette#7.1 v=no
why: browser urlbar border colour; browser chrome is outside the probe's seven_guis path
@answer n=15 entity=color-palette#7.2 v=no
why: browser chrome focus colour; the probe paints nothing and reads no focus style
@answer n=16 entity=color-palette#7.3 v=no
why: blitz-dom default input focus outline colour; a painted value the probe does not read and it styles nothing
@answer n=17 entity=color-palette#8.1 v=no
why: default scrollbar thumb and stroke values; the probe does not read or drive scrollbars
@answer n=18 entity=color-palette#8.2 v=no
why: scrollbar-color resolution and author thumb blending; the scope does not name scrollbars
@answer n=19 entity=color-palette#8.3 v=no
why: unchecked radio ring and checkbox tick colours; no radio or checkbox is on the probe path
@answer n=20 entity=color-palette#8.4 v=no
why: debug overlay box colours; the probe does not use the debug overlay
@answer n=21 entity=color-palette#8.5 v=no
why: devtools layout stroke colours; the probe does not use devtools show_layout
@answer n=22 entity=color-palette#8.6 v=no
why: SVG currentColor serialization; the probe does not serialize SVG
@answer n=23 entity=color-palette#9.1 v=no
why: rdme GitHub markdown colour tokens; apps/readme is outside the probe
@answer n=24 entity=color-palette#10.1 v=no
why: absence of engine colour tokens; SelectListView colours are not read by the probe
@answer n=25 entity=color-palette#11.1 v=no
why: clip-path fixture block palette; an example asset the probe does not read
