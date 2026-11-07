@answer n=1 entity=color-palette#1.1 v=no
why: Color type and sRGB conversion are colour math the chunk never touches
@answer n=2 entity=color-palette#1.2 v=no
why: system colour scheme enum on the viewport; the chunk neither sets nor reads the scheme
@answer n=3 entity=color-palette#1.3 v=no
why: scheme-change recascade is triggered by scheme changes, which the chunk's render sites do not make
@answer n=4 entity=color-palette#1.4 v=no
why: iframe colour-scheme copy; no iframe sub-document is touched
@answer n=5 entity=color-palette#1.5 v=no
why: legacy bgcolor mapping in stylo.rs is not reached; the buffer records attribute names raw and replays them through the unchanged MutationWriter
@answer n=6 entity=color-palette#1.6 v=no
why: frame clear base_color lives in dioxus-native config and renderer, neither reached; the chunk changes when mutations apply, not the clear colour
@answer n=7 entity=color-palette#1.7 v=no
why: canvas background fallback is blitz-paint render code the chunk does not reach
@answer n=8 entity=color-palette#1.8 v=no
why: WPT white background fill is in the wpt runner, which the chunk does not reach
@answer n=9 entity=color-palette#2.1 v=no
why: browser chrome focus colour and icon hover states; the chunk touches no browser UI component
@answer n=10 entity=color-palette#2.2 v=no
why: about:history link colour; no about page is touched
@answer n=11 entity=color-palette#2.3 v=no
why: seven_guis accent and focus outline are paint values held in layout-templates; the chunk's focus assertions read focus state, not outline paint
@answer n=12 entity=color-palette#2.4 v=no
why: blitz-dom default stylesheet link, input focus outline and button colours; focus assertions read focus state, not paint, and default.css is not reached
@answer n=13 entity=color-palette#2.5 v=no
why: multiple-select chosen row pair; no select element is touched
@answer n=14 entity=color-palette#2.6 v=no
why: open select list highlight constants; no select list is touched, as the task's not_needed item 2 states
@answer n=15 entity=color-palette#2.7 v=no
why: open select list disabled-row text constant; no select list is touched
@answer n=16 entity=color-palette#2.8 v=no
why: open select list border constant; no select list is touched
@answer n=17 entity=color-palette#2.9 v=no
why: open select list plain-row pair measured by select_popup_painted; no select list is touched
@answer n=18 entity=color-palette#2.10 v=no
why: default text-selection highlight is a blitz-paint engine default the chunk does not reach
@answer n=19 entity=color-palette#2.11 v=no
why: counter and transparent example button colours; neither example is touched or named by the scope
@answer n=20 entity=color-palette#3.1 v=no
why: browser tabstrip surface values; browser chrome is not touched
@answer n=21 entity=color-palette#3.2 v=no
why: browser tooltip, FPS overlay and status bar values; browser chrome is not touched
@answer n=22 entity=color-palette#3.3 v=no
why: browser about-page backgrounds; no about page is touched
@answer n=23 entity=color-palette#3.4 v=no
why: rdme page background; the readme app is not reached
@answer n=24 entity=color-palette#3.5 v=no
why: todomvc page background; the todomvc example is not reached
@answer n=25 entity=color-palette#3.6 v=no
why: wasm_hello dark palette; the example is not reached, and the web-spa unchanged-paint scope is carried by the buffer replay, not by these values
