@row_add n=1 row=overview/surface-web-spa area=surface-web-spa rec=1
text: The web-spa surface is WASM pages rendering into a canvas: on wasm32 the dioxus-native surface is a canvas appended to the page body and sized from host CSS, so a page such as todomvc fills the window with that canvas.
@row_none n=1 rec=2 type=S
@row_none n=2 rec=3 type=S
@row_none n=3 rec=4 type=S
@row_none n=4 rec=5 type=5
@row_none n=5 rec=6 type=5
@row_none n=6 rec=7 type=S
@row_none n=7 rec=8 type=5
@row_none n=8 rec=9 type=5
@row_none n=9 rec=10 type=5
@row_none n=10 rec=11 type=5
@row_none n=11 rec=12 type=4
@row_none n=12 rec=13 type=9
@row_none n=13 rec=14 type=4
@row_none n=14 rec=15 type=5
@row_none n=15 rec=16 type=5
@row_none n=16 rec=17 type=5
@row_none n=17 rec=18 type=5
@row_none n=18 rec=19 type=7
@row_none n=19 rec=20 type=7
@row_none n=20 rec=21 type=7
@row_none n=21 rec=22 type=7
@row_none n=22 rec=23 type=7
@row_none n=23 rec=24 type=7
@row_none n=24 rec=25 type=4
@row_add n=2 row=contract/viewport-color-scheme/color-scheme area=color-palette rec=26
boundary: The viewport's system color scheme
item: color scheme
promise: The system color scheme is Light by default or Dark, is set on the viewport, and feeds prefers-color-scheme.
@row_add n=3 row=decision/color-scheme-change-full-recascade area=color-palette rec=27
name: Full recascade on color-scheme change
rule: A change of color scheme triggers a full recascade of the document.
why: light-dark() and system colors resolve at cascade time.
@row_none n=25 rec=28 type=4
relied: Iframe sub-documents take the parent viewport's color scheme.
@row_none n=26 rec=29 type=4
@row_add n=4 row=contract/dioxus-native-config/base-color area=color-palette rec=30
boundary: dioxus-native configuration
item: base_color
promise: The frame clear color is an optional base_color, a peniko Color, unset by default.
@row_none n=27 rec=31 type=4
@row_none n=28 rec=32 type=4
@row_none n=29 rec=33 type=5
@row_none n=30 rec=34 type=5
@row_none n=31 rec=35 type=S
@row_none n=32 rec=36 type=5
@row_add n=5 row=decision/select-chosen-row-chromium-pair area=color-palette rec=37
name: Select chosen row takes Chromium's inactive selected pair
rule: A multiple select's chosen row is painted with Chromium's inactive selected pair, a #c8c8c8 background with #000000 text.
why: Follows Chromium's inactive selected pair; hardcoded as the engine holds no tokens.
guard: select_interaction
@row_add n=6 row=decision/engine-colours-hardcoded area=color-palette rec=37,38,39,40
name: Engine colours hardcoded
rule: The engine's own select colours (chosen row, open list highlighted row, disabled row and border) are fixed hardcoded values; the open select list's are constants of the select list view, not in the default sheet.
why: The engine holds no tokens.
@row_none n=33 rec=41 type=9
@row_none n=34 rec=42 type=5
@row_none n=35 rec=43 type=5
@row_none n=36 rec=44 type=5
@row_none n=37 rec=45 type=5
@row_none n=38 rec=46 type=5
@row_none n=39 rec=47 type=5
@row_none n=40 rec=48 type=5
@row_none n=41 rec=49 type=5
@row_none n=42 rec=50 type=5
@row_none n=43 rec=51 type=5
@row_none n=44 rec=52 type=5
@row_none n=45 rec=53 type=5
@row_none n=46 rec=54 type=5
@row_none n=47 rec=55 type=5
