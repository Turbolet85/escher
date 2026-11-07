@answer n=1 entity=color-palette#1.1 v=no
why: the session changed only the dioxus-native-dom mutation path, and blitz-dom's sRGB Color conversion in util.rs is not a reached file
@answer n=2 entity=color-palette#1.2 v=no
why: the system colour scheme enum and its prefers-color-scheme feed in blitz-traits, blitz-dom and the device tests are not reached by the session
@answer n=3 entity=color-palette#1.3 v=no
why: the colour-scheme recascade in blitz-dom document.rs is a different file from the reached dioxus_document.rs, and the scheme behaviour is unchanged
@answer n=4 entity=color-palette#1.4 v=no
why: the parent-to-iframe colour-scheme copy in blitz-dom resolve.rs is not a reached file and its behaviour is unchanged
@answer n=5 entity=color-palette#1.5 v=no
why: the legacy bgcolor-to-background-color mapping in blitz-dom stylo.rs is not a reached file and no colour rule changed
@answer n=6 entity=color-palette#1.6 v=no
why: the optional base_color lives in dioxus-native config.rs and dioxus_renderer.rs, neither a reached file, and the frame clear colour is untouched
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background fallback in blitz-paint render.rs is not a reached file and paint is unchanged
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white background fill in ref_test.rs is not a reached file and no colour changed
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus colour in browser.css is not a reached file, and the session's focus change is MountedData set_focus behaviour, not a colour
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour in the browser's about stylesheets is not a reached file and no colour changed
@answer n=11 entity=color-palette#2.3 v=no
why: the stand's accent and Cells focus outline in cells.rs are not a reached file, the colours are unchanged, and Cells keeps its synchronous scroll_to handles
@answer n=12 entity=color-palette#2.4 v=no
why: the blitz-dom default stylesheet link, focus outline and button colours are not a reached file, and no CSS default changed
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple-select chosen-row pair in the default stylesheet is not a reached file and no CSS default changed
@answer n=14 entity=color-palette#2.6 v=no
why: the SelectListView highlight constants in select_list.rs are not a reached file and the open select list's colour constants are unchanged
@answer n=15 entity=color-palette#2.7 v=no
why: the SelectListView DISABLED_TEXT constant in select_list.rs is not a reached file and its value is unchanged
@answer n=16 entity=color-palette#2.8 v=no
why: the SelectListView BORDER constant in select_list.rs is not a reached file and its value is unchanged
@answer n=17 entity=color-palette#2.9 v=no
why: the plain select-row pair measured by select_popup_painted is not a reached file and no colour changed
@answer n=18 entity=color-palette#2.10 v=no
why: the engine default text-selection highlight in blitz-paint lib.rs is not a reached file and paint is unchanged
@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent example button colours are not a reached file and no colour changed
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surface values in browser.css are not a reached file and no colour changed
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar colours in browser.css are not a reached file and no colour changed
@answer n=22 entity=color-palette#3.3 v=no
why: the about-page background colour in the browser's about stylesheets is not a reached file and no colour changed
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background in blitz-markdown-overrides.css is not a reached file and no colour changed
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background in todomvc.css is not a reached file and no colour changed
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette in its example lib.rs is not a reached file and no colour changed
