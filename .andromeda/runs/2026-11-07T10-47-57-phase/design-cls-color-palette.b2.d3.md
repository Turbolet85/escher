@answer n=1 entity=color-palette#3.7 v=no
why: the example main background colour is paint styling the buffered render does not read or change.
@answer n=2 entity=color-palette#4.1 v=no
why: browser tooltip and status text colours are CSS values; the chunk changes mutation application, not colour.
@answer n=3 entity=color-palette#4.2 v=no
why: browser about-page muted text colours are CSS values the chunk neither sets nor reads.
@answer n=4 entity=color-palette#4.3 v=no
why: browser error page paragraph colour is a CSS value outside the chunk's render path.
@answer n=5 entity=color-palette#4.4 v=no
why: todomvc body and heading colours are example styling the chunk does not touch.
@answer n=6 entity=color-palette#4.5 v=no
why: wasm_hello heading and code colours are the example's own styling, not a value the buffered render reads or the probe re-asserts.
@answer n=7 entity=color-palette#5.1 v=no
why: disabled checkbox and radio accent colour sits in form controls the chunk does not touch.
@answer n=8 entity=color-palette#6.1 v=no
why: WPT runner terminal result colour is in the wpt runner, which the chunk does not touch.
@answer n=9 entity=color-palette#6.2 v=no
why: WPT runner terminal colour for partial-pass results is in the wpt runner, which the chunk does not touch.
@answer n=10 entity=color-palette#6.3 v=no
why: WPT runner terminal colour for FAIL is in the wpt runner, which the chunk does not touch.
@answer n=11 entity=color-palette#6.4 v=no
why: WPT runner terminal colour for TIMEOUT is in the wpt runner, which the chunk does not touch.
@answer n=12 entity=color-palette#6.5 v=no
why: WPT runner terminal colour for SKIP is in the wpt runner, which the chunk does not touch.
@answer n=13 entity=color-palette#6.6 v=no
why: WPT runner terminal colour for CRASH is in the wpt runner, which the chunk does not touch.
@answer n=14 entity=color-palette#7.1 v=no
why: browser urlbar border colour is CSS styling the chunk does not set or read.
@answer n=15 entity=color-palette#7.2 v=no
why: browser chrome focus border colour is paint styling; the chunk's focus assertions read focused-node identity, not outline colour.
@answer n=16 entity=color-palette#7.3 v=no
why: blitz-dom default input focus outline colour is paint styling; the focus assertions read which node holds focus, not its colour.
@answer n=17 entity=color-palette#8.1 v=no
why: default scrollbar thumb and stroke colours are scrollbar paint; the programmatic scroll rule concerns scroll writes, not colour.
@answer n=18 entity=color-palette#8.2 v=no
why: scrollbar-color resolution is scrollbar paint and style, which the scroll-through-MountedData work does not read.
@answer n=19 entity=color-palette#8.3 v=no
why: radio ring and checkbox tick colours are form control paint the chunk does not touch.
@answer n=20 entity=color-palette#8.4 v=no
why: debug overlay box colours are a paint debug aid the chunk does not touch.
@answer n=21 entity=color-palette#8.5 v=no
why: devtools layout stroke colours are a devtools paint path the chunk does not touch.
@answer n=22 entity=color-palette#8.6 v=no
why: SVG currentColor serialization is a serializer rule; the chunk's snapshot re-assertion reads the document graph and diff, and the scope names no SVG fixture.
@answer n=23 entity=color-palette#9.1 v=no
why: rdme GitHub markdown colour tokens are an app stylesheet the chunk does not touch.
@answer n=24 entity=color-palette#10.1 v=no
why: the observed absence of colour tokens concerns colour, which the chunk does not change; no token or custom property is added or read.
@answer n=25 entity=color-palette#11.1 v=no
why: the clip-path fixture palette is an example asset the chunk does not render through or touch.
