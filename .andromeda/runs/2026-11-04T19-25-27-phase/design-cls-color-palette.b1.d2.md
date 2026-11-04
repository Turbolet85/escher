@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and sRGB conversion are a paint path, and the probe paints and styles nothing

@answer n=2 entity=color-palette#1.2 v=no
why: the colour scheme feeds prefers-color-scheme, which no probe link reads

@answer n=3 entity=color-palette#1.3 v=no
why: a recascade on scheme change is styling, and the probe changes no scheme and reads no cascade output

@answer n=4 entity=color-palette#1.4 v=no
why: iframe scheme copying is a sub-document styling rule, and the probe runs no iframe

@answer n=5 entity=color-palette#1.5 v=no
why: the legacy bgcolor attribute maps to a paint colour, and the probe paints nothing

@answer n=6 entity=color-palette#1.6 v=no
why: the optional frame clear colour is a paint default, unset by default, and the probe paints nothing

@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background is a paint fallback the probe does not read

@answer n=8 entity=color-palette#1.8 v=no
why: the white fill belongs to the WPT runner, which the chunk does not reach

@answer n=9 entity=color-palette#2.1 v=no
why: browser chrome focus colours belong to the browser app, which the chunk does not reach

@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour belongs to the browser app's about pages, which the chunk does not reach

@answer n=11 entity=color-palette#2.3 v=no
why: the seven_guis accent colours are held in layout-templates, and the probe reads no paint of the stand

@answer n=12 entity=color-palette#2.4 v=no
why: the blitz-dom default link, focus and button colours are paint defaults the probe does not read

@answer n=13 entity=color-palette#2.5 v=no
why: the multiple-select chosen-row pair is a paint value, and the colour palette is not needed for the probe

@answer n=14 entity=color-palette#2.6 v=no
why: the SelectListView highlight constants are a listed not-needed item, and the probe paints nothing

@answer n=15 entity=color-palette#2.7 v=no
why: the SelectListView disabled-row constant is a listed not-needed item, and the probe paints nothing

@answer n=16 entity=color-palette#2.8 v=no
why: the SelectListView border constant is a listed not-needed item, and the probe paints nothing

@answer n=17 entity=color-palette#2.9 v=no
why: the select list plain pair is a paint measurement the probe does not take

@answer n=18 entity=color-palette#2.10 v=no
why: the default text-selection highlight is a paint default the probe does not read

@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent button colours are example paint values the probe does not read

@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surfaces belong to the browser app, which the chunk does not reach

@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar belong to the browser app, which the chunk does not reach

@answer n=22 entity=color-palette#3.3 v=no
why: the about-page backgrounds belong to the browser app, which the chunk does not reach

@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background belongs to the readme app, which the chunk does not reach

@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background belongs to todomvc, which the chunk does not reach

@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette belongs to the WASM example, which the chunk does not reach
