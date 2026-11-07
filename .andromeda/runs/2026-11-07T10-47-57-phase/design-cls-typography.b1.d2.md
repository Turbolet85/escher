@answer n=1 entity=typography#1.1 v=no
why: browser chrome text sizes are app-owned styles; the scope's browser obligation is identical documents, element ids and mounted events, not paint
@answer n=2 entity=typography#1.2 v=no
why: browser suggestion section header is browser chrome styling the chunk does not touch or measure
@answer n=3 entity=typography#1.3 v=no
why: browser FPS overlay font is not reached by the render-path change or named by the scope
@answer n=4 entity=typography#1.4 v=no
why: about:history heading style is an app page the chunk neither changes nor measures
@answer n=5 entity=typography#1.5 v=no
why: readme body typography belongs to the readme app, which the chunk does not touch
@answer n=6 entity=typography#1.6 v=yes
why: the default system-ui, sans-serif family of inputs is the family the ported test's get_client_rect and the pins' focus assertions over text inputs are measured at (scope n=2, n=9)
@answer n=7 entity=typography#1.7 v=no
why: default heading sizes and weights do not sit under any node the chunk's tests measure or the scope names
@answer n=8 entity=typography#1.8 v=no
why: default code and pre family is not a text input and no code node is named by the chunk
@answer n=9 entity=typography#1.9 v=no
why: select multiple row line-height applies to select lists, which the chunk does not touch
@answer n=10 entity=typography#1.10 v=no
why: todomvc example styling is outside the chunk's code and tests
@answer n=11 entity=typography#1.11 v=no
why: wgpu_texture example styling is outside the chunk's code and tests
@answer n=12 entity=typography#1.12 v=yes
why: the 16.0 editor size is the size text inputs are measured at, which the ported test's get_client_rect and the focus assertions read (scope n=2, n=9)
@answer n=13 entity=typography#1.13 v=yes
why: the 16px base size for non-monospace generics is the size system-ui text inputs are measured at (scope n=2, n=9)
@answer n=14 entity=typography#1.14 v=no
why: list bullet font belongs to list markers, which no chunk test or scope item reaches
@answer n=15 entity=typography#2.1 v=yes
why: the wasm bundled DejaVu registration for sans-serif, serif, monospace and system-ui is what the web-spa surface paints with, and the scope re-asserts that paint is unchanged (scope n=5)
@answer n=16 entity=typography#2.2 v=no
why: the entity only points to where the stand's font registration is held in architecture and states no value; the chunk changes neither that registration nor font_ctx()
@answer n=17 entity=typography#2.3 v=yes
why: on WASM a context with bundled fonts must be provided, which is what the web-spa surface paints text through, and its paint is re-asserted unchanged (scope n=5)
@answer n=18 entity=typography#2.4 v=no
why: this lists dioxus-native's Cargo feature set, which the chunk does not change (Dioxus stays at 0.7); the system-fonts feature the chunk needs for blitz-tests is carried by typography#2.8
@answer n=19 entity=typography#2.5 v=no
why: the always-registered bullet font serves list markers, which no chunk test or scope item reaches
@answer n=20 entity=typography#2.6 v=no
why: the screenshot refusal fonts-unavailable path is named as not needed (scope not_needed n=3), and no screenshot path changes
@answer n=21 entity=typography#2.7 v=no
why: the document.fonts stub is in the blitz-vibey-script JavaScript bootstrap, outside the Dioxus bridge render path the chunk changes and not named by the scope
@answer n=22 entity=typography#2.8 v=yes
why: scope need n=9 names blitz-dom's system-fonts feature for blitz-tests text measurement, and the ported test's get_client_rect and the focus assertions depend on it; the scope names it, so it is not dropped as a standing convention
@answer n=23 entity=typography#3.1 v=yes
why: generic family mapping with None as sans-serif resolves the family the measured text inputs use, so their measured geometry reads it (scope n=2, n=9)
@answer n=24 entity=typography#3.2 v=no
why: weight, width and style conversion acts on weights the measured inputs do not state, and engine text mapping is named as not needed (scope not_needed n=3)
@answer n=25 entity=typography#3.3 v=no
why: OpenType feature tags apply only to ligature, caps, position, numeric and east-asian settings the measured inputs do not set, and engine text mapping is named as not needed (scope not_needed n=3)
