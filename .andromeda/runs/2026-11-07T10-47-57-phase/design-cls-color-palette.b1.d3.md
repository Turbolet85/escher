@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and sRGB conversion are in blitz-dom util.rs, which the chunk does not reach
@answer n=2 entity=color-palette#1.2 v=no
why: the colour-scheme enum and prefers-color-scheme feed are not reached by the buffered render
@answer n=3 entity=color-palette#1.3 v=no
why: the recascade on scheme change is in blitz-dom document.rs, not reached and not triggered by mutation replay
@answer n=4 entity=color-palette#1.4 v=no
why: the iframe scheme copy in resolve.rs is outside the reached files and the named checks
@answer n=5 entity=color-palette#1.5 v=no
why: the bgcolor mapping is a stylo pass that the replayed attribute values do not touch
@answer n=6 entity=color-palette#1.6 v=no
why: the frame clear colour is set in config.rs and dioxus_renderer.rs, neither reached nor named
@answer n=7 entity=color-palette#1.7 v=no
why: canvas background resolution is in blitz-paint, which the chunk paints through unchanged, and no named check re-measures it
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white fill is in the wpt runner, which the chunk does not touch
@answer n=9 entity=color-palette#2.1 v=no
why: browser chrome focus colour is app CSS in apps/browser, not reached
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour is app CSS in apps/browser, not reached
@answer n=11 entity=color-palette#2.3 v=no
why: seven_guis accent and outline colours are layout-templates values, and the stand_* checks do not re-measure colour
@answer n=12 entity=color-palette#2.4 v=no
why: blitz-dom default link, outline and button colours are not read by the ported test or the pins, which assert focus position
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple-select chosen-row pair is a default.css value, and no select list is touched
@answer n=14 entity=color-palette#2.6 v=no
why: the select list highlight constants are not touched, as no select list is touched
@answer n=15 entity=color-palette#2.7 v=no
why: the select list disabled-row constant is not touched, as no select list is touched
@answer n=16 entity=color-palette#2.8 v=no
why: the select list border constant is not touched, as no select list is touched
@answer n=17 entity=color-palette#2.9 v=no
why: the select list plain-row pair is not touched, as no select list is touched
@answer n=18 entity=color-palette#2.10 v=no
why: the default text-selection highlight is a blitz-paint default that no selection path touches
@answer n=19 entity=color-palette#2.11 v=no
why: counter and transparent example button colours are example app code, not reached
@answer n=20 entity=color-palette#3.1 v=no
why: browser tabstrip surface values are app CSS in apps/browser, not reached
@answer n=21 entity=color-palette#3.2 v=no
why: browser tooltip, FPS overlay and status bar values are app CSS, not reached
@answer n=22 entity=color-palette#3.3 v=no
why: browser about-page backgrounds are app CSS, not reached
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background is app CSS in apps/readme, not reached
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background is example CSS that no named check reads
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello palette is example app code that the web-spa replay does not alter
