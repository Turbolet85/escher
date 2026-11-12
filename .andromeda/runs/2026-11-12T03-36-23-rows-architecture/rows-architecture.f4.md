@row_add n=1 row=decision/constant-rationale-docs area=conventions rec=1,2
name: Rationale for constants
rule: Doc comments state the rationale for constants.
@row_add n=2 row=decision/do-not-reorder-marker area=conventions rec=3,4
name: Ordering-sensitive block marker
rule: An ordering-sensitive block is marked "WARNING: DO NOT REORDER".
@row_add n=3 row=decision/blitz-js-native-prefix area=conventions rec=5,6
name: Internal JS native prefix
rule: Internal JS natives are prefixed `__blitz_`.
@row_add n=4 row=decision/paint-module-per-concern area=conventions rec=5,6
name: Paint rendering one module per concern
rule: Paint rendering is split one module per concern.
@row_add n=5 row=decision/borrow-take-and-restore area=conventions rec=7,8
name: Take and restore for borrow conflicts
rule: Borrow conflicts are worked around by taking a child list or layout out and restoring it.
@row_add n=6 row=decision/let-chains area=conventions rec=7,8
name: Let-chains in if conditions
rule: `if` conditions use let-chains.
@row_add n=7 row=decision/integration-test-one-behavior area=conventions rec=9,10
name: One behavior per integration test file
rule: Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
@row_add n=8 row=decision/android-entry-point area=conventions rec=11,12
name: Android entry points
rule: Android entry points are `android_main` calling `set_android_app`.
@row_add n=9 row=decision/dioxus-example-shape area=conventions rec=11,12
name: Dioxus example shape
rule: Dioxus examples declare `dioxus_native::launch(app)` with `#[component]` components and signal and store hooks, their CSS in `const` raw strings.
@row_add n=10 row=decision/reduced-fixture-variants area=conventions rec=13,14
name: Reduced fixture variants
rule: Reduced variants of HTML fixtures sit beside the full pages.
@row_add n=11 row=contract/provider-traits/viewport-scale area=standard-contracts rec=15
boundary: blitz-traits provider traits
item: Viewport scale()
promise: A `Viewport`'s `scale()` is hidpi_scale × zoom.
@row_add n=12 row=contract/test-harness/screenshot area=standard-contracts rec=16,17
boundary: blitz-test-harness
item: screenshot and screenshot_rect
promise: `screenshot` and `screenshot_rect` resolve at the harness's own time without tick or pump, paint a raster of the viewport through blitz-paint, composite it over opaque white, and print, panic and log nothing.
@row_add n=13 row=contract/test-harness/text-contrast area=standard-contracts rec=16,17
boundary: blitz-test-harness
item: text_contrast
promise: `text_contrast` reads a WCAG contrast of an element's text over its composited background and changes nothing; it reads None when there is no such text or the background cannot be composited.
