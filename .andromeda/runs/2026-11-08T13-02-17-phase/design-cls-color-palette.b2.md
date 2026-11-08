@answer n=1 entity=color-palette#3.7 v=no
why: example background colours for the custom-widget, static-HTML and wgpu examples; the chunk does not touch those examples and the read paints nothing.
@answer n=2 entity=color-palette#4.1 v=no
why: browser tooltip and status bar text colour; the browser app is not reached by the chunk.
@answer n=3 entity=color-palette#4.2 v=no
why: browser about-page muted text colours; the browser app is not reached by the chunk.
@answer n=4 entity=color-palette#4.3 v=no
why: browser error page paragraph colour; the browser app is not reached by the chunk.
@answer n=5 entity=color-palette#4.4 v=no
why: todomvc body and heading colours; the example is not reached by the chunk and the read paints nothing.
@answer n=6 entity=color-palette#4.5 v=no
why: wasm_hello heading and code colours; the example is not reached by the chunk and the read paints nothing.
@answer n=7 entity=color-palette#5.1 v=no
why: disabled checkbox and radio accent paint in blitz-paint form controls; the read detects disabled by state, not by paint colour.
@answer n=8 entity=color-palette#6.1 v=no
why: WPT runner PASS terminal colour; the wpt runner is not the driver's agent-read surface and the chunk does not reach it.
@answer n=9 entity=color-palette#6.2 v=no
why: WPT runner FAIL-with-passing-subtests terminal colour; the wpt runner is not the driver's agent-read surface.
@answer n=10 entity=color-palette#6.3 v=no
why: WPT runner FAIL terminal colour; the wpt runner is not the driver's agent-read surface.
@answer n=11 entity=color-palette#6.4 v=no
why: WPT runner TIMEOUT terminal colour; the wpt runner is not the driver's agent-read surface.
@answer n=12 entity=color-palette#6.5 v=no
why: WPT runner SKIP terminal colour; the wpt runner is not the driver's agent-read surface.
@answer n=13 entity=color-palette#6.6 v=no
why: WPT runner CRASH terminal colour; the wpt runner is not the driver's agent-read surface.
@answer n=14 entity=color-palette#7.1 v=no
why: browser urlbar border colour; the browser chrome is not reached by the chunk.
@answer n=15 entity=color-palette#7.2 v=no
why: browser chrome focus border colour; a paint value the read does not produce.
@answer n=16 entity=color-palette#7.3 v=no
why: blitz-dom default input focus outline colour; a paint value the read does not produce.
@answer n=17 entity=color-palette#8.1 v=no
why: default scrollbar thumb palette; scrollbar paint the chunk does not reach, and the read scrolls nothing.
@answer n=18 entity=color-palette#8.2 v=no
why: scrollbar-color resolution and thumb blending; scrollbar paint the chunk does not reach, and the into-view scroll it leaves alone is not a colour rule.
@answer n=19 entity=color-palette#8.3 v=no
why: unchecked radio ring and checkbox tick colours; form-control paint the chunk does not reach.
@answer n=20 entity=color-palette#8.4 v=no
why: debug overlay box colours; paint the chunk does not reach.
@answer n=21 entity=color-palette#8.5 v=no
why: devtools layout stroke colours; paint the chunk does not reach.
@answer n=22 entity=color-palette#8.6 v=no
why: SVG currentColor replacement on serialization; paint and serialization the chunk does not reach.
@answer n=23 entity=color-palette#9.1 v=no
why: rdme GitHub markdown colour tokens; apps/readme is not reached by the chunk.
@answer n=24 entity=color-palette#10.1 v=no
why: recorded absence of colour tokens and custom properties; the chunk adds no token, and the select list's fixed colour constants are paint, not a select-list behaviour the scope names.
@answer n=25 entity=color-palette#11.1 v=no
why: 25-colour palette of the existing clip-path example asset; the chunk adds no fixture of this kind and the read paints nothing.
