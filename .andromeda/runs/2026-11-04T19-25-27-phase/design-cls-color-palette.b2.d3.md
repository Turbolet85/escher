@answer n=1 entity=color-palette#3.7 v=no
why: the main-background fill on the custom-widget, html and wgpu examples; the probe paints nothing and reads no background colour
@answer n=2 entity=color-palette#4.1 v=no
why: browser tooltip and status text colour; the browser app is not on the probe path
@answer n=3 entity=color-palette#4.2 v=no
why: browser about-page muted text colour; browser app pages are not on the probe path
@answer n=4 entity=color-palette#4.3 v=no
why: browser error page paragraph colour; the browser app is not on the probe path
@answer n=5 entity=color-palette#4.4 v=no
why: todomvc body and heading colours; the web-spa todomvc canvas is not needed (task item 9)
@answer n=6 entity=color-palette#4.5 v=no
why: wasm_hello heading and code colours; the web-spa surface is not needed (task item 9)
@answer n=7 entity=color-palette#5.1 v=no
why: disabled checkbox and radio accent; form-control paint, which the probe does not do
@answer n=8 entity=color-palette#6.1 v=no
why: WPT runner PASS terminal colour; CLI terminal output colours are not needed (task item 10)
@answer n=9 entity=color-palette#6.2 v=no
why: WPT runner FAIL-with-passing-subtests terminal colour; CLI output colours are not needed (task item 10)
@answer n=10 entity=color-palette#6.3 v=no
why: WPT runner FAIL terminal colour; CLI output colours are not needed (task item 10)
@answer n=11 entity=color-palette#6.4 v=no
why: WPT runner TIMEOUT terminal colour; CLI output colours are not needed (task item 10)
@answer n=12 entity=color-palette#6.5 v=no
why: WPT runner SKIP terminal colour; CLI output colours are not needed (task item 10)
@answer n=13 entity=color-palette#6.6 v=no
why: WPT runner CRASH terminal colour; CLI output colours are not needed (task item 10)
@answer n=14 entity=color-palette#7.1 v=no
why: browser urlbar border colour; browser chrome is not on the probe path
@answer n=15 entity=color-palette#7.2 v=no
why: browser chrome focus border colour; browser chrome paint is not read by the probe
@answer n=16 entity=color-palette#7.3 v=no
why: blitz-dom default input focus outline colour; the probe drives clicks and keys but reads no outline paint, and focus paint is not a link's reading
@answer n=17 entity=color-palette#8.1 v=no
why: default scrollbar thumb palette; the probe does no scrolling (task item 6) and the scrollbar tests are not in its scope
@answer n=18 entity=color-palette#8.2 v=no
why: scrollbar-color resolution and author thumb blending; scrollbar paint, which the probe does not touch
@answer n=19 entity=color-palette#8.3 v=no
why: unchecked radio ring and checkbox tick colours; form-control paint, which the probe does not do
@answer n=20 entity=color-palette#8.4 v=no
why: debug overlay box colours; devtools paint that the probe does not read
@answer n=21 entity=color-palette#8.5 v=no
why: devtools layout stroke colours; the devtools overlay is not needed (task item 7)
@answer n=22 entity=color-palette#8.6 v=no
why: SVG currentColor serialisation; serialize.rs is not on the probe path
@answer n=23 entity=color-palette#9.1 v=no
why: rdme GitHub markdown colour tokens; the rdme app is not on the probe path
@answer n=24 entity=color-palette#10.1 v=no
why: absence of colour tokens and the SelectListView colour constants; the probe reads components, handlers and state, not colour, and the task lists these constants as not needed (task item 1)
@answer n=25 entity=color-palette#11.1 v=no
why: clip-path fixture block palette; an example fixture that is not on the probe path
