@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and sRGB conversion of Stylo colours; the probe reads no colour value
@answer n=2 entity=color-palette#1.2 v=no
why: the system colour scheme enum feeding prefers-color-scheme; the probe sets no scheme and reads no style
@answer n=3 entity=color-palette#1.3 v=no
why: a colour-scheme change triggers a recascade; the probe triggers no scheme change
@answer n=4 entity=color-palette#1.4 v=no
why: parent colour scheme copied to iframe sub-documents; the probe uses no iframe
@answer n=5 entity=color-palette#1.5 v=no
why: legacy bgcolor attribute maps to background-color; the probe reads no paint
@answer n=6 entity=color-palette#1.6 v=no
why: the optional frame clear base_color, unset by default; the probe paints nothing and passes no base_color at launch
@answer n=7 entity=color-palette#1.7 v=no
why: canvas background from the root element; the probe paints nothing
@answer n=8 entity=color-palette#1.8 v=no
why: WPT-rendered pages get a white fill; the WPT runner is outside the probe
@answer n=9 entity=color-palette#2.1 v=no
why: browser chrome focus colour; the browser app is outside the probe
@answer n=10 entity=color-palette#2.2 v=no
why: browser about:history link colour; the browser app is outside the probe
@answer n=11 entity=color-palette#2.3 v=no
why: seven_guis accent colours held in layout-templates; the scope names the stand but not its colours, and the probe reads no colour
@answer n=12 entity=color-palette#2.4 v=no
why: blitz-dom default link, focus and button colours; the probe reads no paint
@answer n=13 entity=color-palette#2.5 v=no
why: multiple-select chosen-row colours; the probe drives no select
@answer n=14 entity=color-palette#2.6 v=no
why: select list highlighted-row constants; the probe opens no select popup
@answer n=15 entity=color-palette#2.7 v=no
why: select list disabled-row constant; the probe opens no select popup
@answer n=16 entity=color-palette#2.8 v=no
why: select list border constant; the probe opens no select popup
@answer n=17 entity=color-palette#2.9 v=no
why: select list plain-row pair; the probe opens no select popup
@answer n=18 entity=color-palette#2.10 v=no
why: default text-selection highlight; the probe makes no selection
@answer n=19 entity=color-palette#2.11 v=no
why: counter and transparent example button colours; the probe reads no button paint
@answer n=20 entity=color-palette#3.1 v=no
why: browser tabstrip surface levels; the browser app is outside the probe
@answer n=21 entity=color-palette#3.2 v=no
why: browser tooltip, FPS overlay and status bar colours; the browser app is outside the probe
@answer n=22 entity=color-palette#3.3 v=no
why: browser about-page backgrounds; the browser app is outside the probe
@answer n=23 entity=color-palette#3.4 v=no
why: rdme page background per scheme; the readme app is outside the probe
@answer n=24 entity=color-palette#3.5 v=no
why: todomvc page background; the todomvc example is outside the probe
@answer n=25 entity=color-palette#3.6 v=no
why: wasm_hello dark palette; the WASM example is outside the probe
