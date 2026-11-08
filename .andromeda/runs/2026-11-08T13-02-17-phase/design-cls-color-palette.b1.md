@answer n=1 entity=color-palette#1.1 v=no
why: the Color type's sRGB conversion is paint, and the read reports ids, verbs and causes, not paint
@answer n=2 entity=color-palette#1.2 v=no
why: the system colour scheme feeds prefers-color-scheme only, which the read does not reach
@answer n=3 entity=color-palette#1.3 v=no
why: a scheme change's full recascade is triggered by a colour-scheme change the read never makes
@answer n=4 entity=color-palette#1.4 v=no
why: the iframe sub-document copy of the parent's colour scheme is paint state the read does not reach
@answer n=5 entity=color-palette#1.5 v=no
why: the legacy bgcolor attribute mapping is paint in the stylo layer, which the chunk does not touch
@answer n=6 entity=color-palette#1.6 v=no
why: the frame clear colour is dioxus-native paint config, and the chunk touches no dioxus-native renderer file
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background is paint, and hit-testing and the covered cause do not read background colour
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT runner's white fill is in the wpt runner, which the chunk does not touch
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus colour belongs to the browser app, which the chunk does not touch
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour is browser app paint, which the chunk does not touch
@answer n=11 entity=color-palette#2.3 v=no
why: the stand's accent and select border are paint held in layout-templates, and neither the read nor the stand proof reads colour
@answer n=12 entity=color-palette#2.4 v=no
why: the default sheet's link colour, underline and input outline are paint, and the actionable criteria are not colour
@answer n=13 entity=color-palette#2.5 v=no
why: a multiple select's chosen-row paint pair is paint the read neither paints nor detects
@answer n=14 entity=color-palette#2.6 v=no
why: the select list's highlighted-row paint constants are paint, and rows are answered by owner and verb, not by highlight
@answer n=15 entity=color-palette#2.7 v=no
why: DISABLED_TEXT is how a disabled option is painted, not how it is detected, and the disabled cause comes from the target's state
@answer n=16 entity=color-palette#2.8 v=no
why: the select list's border constant is paint the read does not reach
@answer n=17 entity=color-palette#2.9 v=no
why: the select list's plain-row pair is paint measured by select_popup_painted, which the read does not re-measure
@answer n=18 entity=color-palette#2.10 v=no
why: the engine's default text-selection highlight is paint the read does not reach
@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent example button colours are example paint, and the chunk adds no example or touches those apps
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surface is browser app paint, which the chunk does not touch
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar surfaces are browser app paint, which the chunk does not touch
@answer n=22 entity=color-palette#3.3 v=no
why: the browser about-page backgrounds are browser app paint, which the chunk does not touch
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background per scheme is readme app paint, which the chunk does not touch
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background is example paint, and the chunk adds no todomvc change
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette is example paint, and the chunk does not touch that example
