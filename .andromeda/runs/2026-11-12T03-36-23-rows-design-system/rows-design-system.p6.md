@row_none n=1 rec=1 type=9
@row_none n=2 rec=2 type=9
@row_none n=3 rec=3 type=9
@row_add n=1 row=contract/native-config/wasm-font-context area=surface rec=4,22
boundary: dioxus-native configuration
item: font context on WASM
promise: On WASM a font context with bundled fonts must be provided; `build_single_font_ctx` serves where one font is bundled.
@row_none n=4 rec=5 type=5
@row_none n=5 rec=6 type=5
@row_none n=6 rec=7 type=5
@row_none n=7 rec=8 type=5
@row_none n=8 rec=9 type=5
@row_none n=9 rec=10 type=5
@row_none n=10 rec=11 type=5
@row_none n=11 rec=12 type=5
@row_none n=12 rec=13 type=5
@row_none n=13 rec=14 type=5
@row_none n=14 rec=15 type=5
@row_none n=15 rec=16 type=5
@row_none n=16 rec=17 type=5
@row_none n=17 rec=18 type=5
@row_none n=18 rec=19 type=5
@row_none n=19 rec=20 type=4
relied: WASM builds render sans-serif, serif, monospace and system-ui text with the bundled DejaVu Sans.
@row_none n=20 rec=21 type=S
@row_add n=2 row=contract/native-config/font-context area=typography rec=22
boundary: dioxus-native configuration
item: FontContext
promise: A custom `FontContext` can be set.
@row_none n=21 rec=23 type=6
@row_none n=22 rec=24 type=4
relied: List bullets can always draw from the bullet font, which the default font context always registers.
@row_add n=3 row=contract/driver-screenshot/fonts-unavailable area=typography rec=25
boundary: driver screenshot
item: fonts-unavailable
promise: A `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; a shot never paints with text skipped.
@row_add n=4 row=contract/script-runtime/document-fonts area=typography rec=26
boundary: script runtime DOM
item: document.fonts
promise: `document.fonts` is a stub FontFaceSet in which all fonts report as loaded.
@row_add n=5 row=decision/tests-built-with-system-fonts area=typography rec=27
name: Tests built with system fonts
rule: The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace.
why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
@row_none n=23 rec=28 type=4
relied: An element with no generic font family is laid out as sans-serif.
@row_none n=24 rec=29 type=4
@row_none n=25 rec=30 type=4
@row_none n=26 rec=31 type=4
relied: Layout treats `line-height: normal` as 1.2 times the font size.
@row_none n=27 rec=32 type=4
@row_none n=28 rec=33 type=4
@row_none n=29 rec=34 type=4
relied: The `lang` attribute decides the locale text is shaped and broken with.
@row_none n=30 rec=35 type=4
@row_none n=31 rec=36 type=4
@row_add n=6 row=decision/root-relative-units-track-root-font area=typography rec=37
name: Root-relative units track the root font
rule: rem and rlh units resolve against the root element's font size and line height, also after viewport and hidpi changes, with an initial root font-size of 16px.
why: The stylist device is seeded with the root's values so rem/rlh units do not fall back to the 16px default.
guard: rem_after_viewport_change
@row_none n=32 rec=38 type=4
@row_none n=33 rec=39 type=4
@row_none n=34 rec=40 type=4
relied: Every caller other than the select group header passes the default run paint and adds no synthesized bold.
@row_none n=35 rec=41 type=4
@row_none n=36 rec=42 type=5
@row_none n=37 rec=43 type=5
@row_none n=38 rec=44 type=5
@row_none n=39 rec=45 type=5
@row_none n=40 rec=46 type=5
@row_none n=41 rec=47 type=5
@row_none n=42 rec=48 type=7
@row_none n=43 rec=49 type=7
@row_none n=44 rec=50 type=7
@row_add n=7 row=absence/project-type-scale area=typography rec=51
capability: A project type scale or font tokens
