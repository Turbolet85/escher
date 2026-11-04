@answer n=1 entity=color-palette#1.1 v=no
why: Color type and sRGB conversion are paint values; the probe paints and styles nothing and reads no colour.
@answer n=2 entity=color-palette#1.2 v=no
why: system colour scheme feeds prefers-color-scheme; the probe reads no scheme and no layout mode touches it.
@answer n=3 entity=color-palette#1.3 v=no
why: recascade on scheme change; the probe's runs change no scheme and the chunk measures no cascade.
@answer n=4 entity=color-palette#1.4 v=no
why: iframe scheme copy; iframe sub-documents are not among the probe's reads of the stand's nodes, handlers and state.
@answer n=5 entity=color-palette#1.5 v=no
why: legacy bgcolor attribute maps to background-color; a paint mapping the probe does not read.
@answer n=6 entity=color-palette#1.6 v=no
why: frame clear colour base_color is unset by default; a paint value the probe neither sets nor reads, and the scope names launch but not base_color.
@answer n=7 entity=color-palette#1.7 v=no
why: canvas background is a paint rule; the probe reads layout and listeners, not the canvas.
@answer n=8 entity=color-palette#1.8 v=no
why: WPT white background fill; the WPT runner is not reached by the probe.
@answer n=9 entity=color-palette#2.1 v=no
why: browser chrome focus colour; apps/browser is not reached by the probe.
@answer n=10 entity=color-palette#2.2 v=no
why: browser about:history link colour; apps/browser pages are not reached by the probe.
@answer n=11 entity=color-palette#2.3 v=no
why: seven_guis accent colours are paint values held in layout-templates; the probe reads no colour and the Cells and flight booker tasks are outside its Counter, CRUD and served-task runs.
@answer n=12 entity=color-palette#2.4 v=no
why: blitz-dom default link, focus outline and button colours are default paint values the probe does not read.
@answer n=13 entity=color-palette#2.5 v=no
why: multiple-select chosen-row colours are paint values; select lists are not in the probe's runs.
@answer n=14 entity=color-palette#2.6 v=no
why: open select list highlighted-row constants are paint values; the select popup is not in the probe's scope.
@answer n=15 entity=color-palette#2.7 v=no
why: open select list disabled-row constant is a paint value; the select popup is not in the probe's scope.
@answer n=16 entity=color-palette#2.8 v=no
why: open select list border constant is a paint value; the select popup is not in the probe's scope.
@answer n=17 entity=color-palette#2.9 v=no
why: open select list plain-row pair is a measured paint value in select_popup_painted, a test the chunk does not touch or read.
@answer n=18 entity=color-palette#2.10 v=no
why: default text-selection highlight is a paint default the probe does not read.
@answer n=19 entity=color-palette#2.11 v=no
why: counter and transparent button colours are paint values in examples the probe does not run, and the probe reads no colour.
@answer n=20 entity=color-palette#3.1 v=no
why: browser tabstrip surface colours are paint values in apps/browser, which the probe does not reach.
@answer n=21 entity=color-palette#3.2 v=no
why: browser tooltip, FPS overlay and status bar colours are paint values in apps/browser, which the probe does not reach.
@answer n=22 entity=color-palette#3.3 v=no
why: browser about-page backgrounds are paint values in apps/browser, which the probe does not reach.
@answer n=23 entity=color-palette#3.4 v=no
why: rdme page background colours are paint values in apps/readme, which the probe does not reach.
@answer n=24 entity=color-palette#3.5 v=no
why: todomvc page background is a paint value; the web-spa surface is not needed and todomvc is not the stand.
@answer n=25 entity=color-palette#3.6 v=no
why: wasm_hello dark palette is a paint value on the web-spa surface, which is not needed.
