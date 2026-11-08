@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and sRGB conversion are paint plumbing, and the read reports ids, verbs and causes, not paint
@answer n=2 entity=color-palette#1.2 v=no
why: the system colour scheme feeds prefers-color-scheme, which the read does not evaluate
@answer n=3 entity=color-palette#1.3 v=no
why: the recascade on colour-scheme change is paint work, and the read runs no cascade or settle
@answer n=4 entity=color-palette#1.4 v=no
why: copying the colour scheme into iframe sub-documents is paint-only
@answer n=5 entity=color-palette#1.5 v=no
why: the legacy bgcolor attribute mapping to background-color is paint-only
@answer n=6 entity=color-palette#1.6 v=no
why: the frame clear base_color is a renderer setting the read never paints through
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background fallback is paint, and covered is decided by hit-testing, not background colour
@answer n=8 entity=color-palette#1.8 v=no
why: the white background fill for WPT ref-test pages is outside the driver's agent-read surface
@answer n=9 entity=color-palette#2.1 v=no
why: browser chrome focus and icon colours belong to the browser app, which the chunk does not touch
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour belongs to the browser app's stylesheets, which the chunk does not touch
@answer n=11 entity=color-palette#2.3 v=no
why: the seven_guis stand's accent, success-message and select-border colours are paint, and the stand proof reads ids and causes, not paint
@answer n=12 entity=color-palette#2.4 v=no
why: the default link, focus and button colours are paint, and the actionable set comes from handlers, owner-served nodes and unkeyed_actionable criteria, not from a[href] styling
@answer n=13 entity=color-palette#2.5 v=no
why: the chosen-row colour pair of a multiple select is paint, and the read does not paint rows
@answer n=14 entity=color-palette#2.6 v=no
why: SelectListView's highlight colour constants are paint, and the scope names only the disabled row value among the select list's constants
@answer n=15 entity=color-palette#2.7 v=yes
why: the scope names the DISABLED_TEXT row colour and says it is how a disabled option is painted, not detected, so the read's disabled cause for owner-served select rows must come from the target's disabled state and the stand proof must honour that
@answer n=16 entity=color-palette#2.8 v=no
why: the select list border colour is paint, and the scope does not name it
@answer n=17 entity=color-palette#2.9 v=no
why: the plain row pair is painted and measured by select_popup_painted, and the scope does not name its value
@answer n=18 entity=color-palette#2.10 v=no
why: the default text-selection highlight is an engine paint default the read never touches
@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent example button colours are paint in example apps the chunk does not touch
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surface scale is browser app paint
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar backgrounds are browser app paint
@answer n=22 entity=color-palette#3.3 v=no
why: the browser about-page backgrounds are browser app paint
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background is readme app paint
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background is example app paint
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette is example app paint
