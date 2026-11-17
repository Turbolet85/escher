@task_prose part=session
Took upstream Blitz main up to e5305a4c1: five upstream picks merged in, ten upstream commits read and five left out, 97 carried left out rather than 112.
Moved the paint stack to AnyRender 0.15 as one coupled dependency set (vello 0.12, usvg 0.50, anyrender_vello_gpu in place of anyrender_vello_hybrid, skrifa 0.49 and read-fonts 0.46 beside 0.44 and 0.41) and zip to 9.6.1, dropping four advisory ignores.
Renamed the GPU renderer cargo feature vello-hybrid and hybrid to vello-gpu across dioxus-native, the apps, the examples, the justfile recipe and the wasm CI leg; the accessibility features themselves did not move.
Upstream engine behaviour taken: SVG image and feImage href now fetched as a stamped sub-resource (not use), and scroll offsets clamped to the scroll range at layout, so a shrunken scroller such as the CRUD list after a delete reads offset 0.
The CI default-features test job now frees disk before the test leg and reads df -h before and after it; the free-disk guard renamed from only-stand-repeat to test-and-stand-repeat.
Every gate green, the a11y leg green, the stand and all runs passed, CI green on attempt 2 after two jobs no runner acquired; no capability claimed.
@task_file n=1 path=Cargo.toml
what: the workspace manifest — the shared dependency pins, the AnyRender renderer set and the renamed GPU renderer key; edited
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lockfile — the resolved dependency set, AnyRender 0.15, zip 9.6.1, two skrifa lines; edited
@task_file n=3 path=deny.toml
what: the cargo-deny audit config — the advisory ignore list; edited, four ignores dropped
@task_file n=4 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document — its image fetch moved here as one stamped request builder, beside the document's accessibility tree method; edited
@task_file n=5 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the document mutator — image loading for img and SVG image and feImage href, removal of a subtree with its scroll state reset; edited
@task_file n=6 path=packages/blitz-dom/src/node/element.rs mark=merge
what: element data — the SVG href read (href else xlink:href) added; edited
@task_file n=7 path=packages/blitz-dom/src/util.rs mark=merge
what: engine utilities — SVG parsing now given the document's image cache for inline svg; edited
@task_file n=8 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: engine scrolling — a node's scroll range, scroll offset clamp, viewport clamp and scroll animation clamp; edited
@task_file n=9 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: engine layout — final layout now clamps each scroller's offset to its range; edited
@task_file n=10 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: layout tree construction for elements; edited by the upstream pick
@task_file n=11 path=packages/blitz-dom/src/resolve.rs mark=merge
what: the style and layout resolve pass — viewport and scroll animation clamped after a device change; edited
@task_file n=12 path=packages/blitz-dom/src/net.rs mark=merge
what: engine network sub-resource handlers; edited by the upstream pick
@task_file n=13 path=packages/blitz-dom/Cargo.toml mark=merge
what: the engine crate manifest — its svg feature gains png; edited
@task_file n=14 path=packages/blitz-paint/src/render.rs
what: the engine painter — element paint, scrollbars and their thumbs, ported to AnyRender 0.15 with anti-aliasing stated on every fill and stroke; edited
@task_file n=15 path=packages/blitz-paint/src/render/form_controls.rs
what: form control paint — checkbox, radio and range input thumb; ported to AnyRender 0.15; edited
@task_file n=16 path=packages/blitz-paint/src/render/select_list.rs
what: the open select list's paint — its rows, highlighted row and engine colours; ported to AnyRender 0.15; edited
@task_file n=17 path=packages/blitz-paint/src/text.rs mark=merge
what: text and glyph paint, including the text caret and selection; ported to AnyRender 0.15; edited
@task_file n=18 path=packages/blitz-paint/src/render/border.rs
what: border and outline paint, the focus outline among them; ported to AnyRender 0.15; edited
@task_file n=19 path=packages/blitz-paint/src/render/background.rs
what: background colour and image paint; ported to AnyRender 0.15; edited
@task_file n=20 path=packages/blitz-paint/src/render/box_shadow.rs
what: box shadow paint; called in AnyRender 0.15's form; edited
@task_file n=21 path=packages/blitz-paint/src/filters.rs
what: CSS filter conversion for paint; now given the element bounds; edited
@task_file n=22 path=packages/blitz-paint/src/debug_overlay.rs
what: the layout debug overlay paint; ported to AnyRender 0.15; edited
@task_file n=23 path=packages/dioxus-native/Cargo.toml
what: the Dioxus native renderer crate manifest — its cargo features, the accessibility feature beside the GPU renderer feature renamed vello-gpu; edited
@task_file n=24 path=packages/dioxus-native/src/lib.rs
what: the Dioxus native launch entry — renderer choice by feature, now vello-gpu; edited
@task_file n=25 path=packages/dioxus-native/src/dioxus_renderer.rs
what: the Dioxus native window renderer aliases — now the Vello GPU window renderer; edited
@task_file n=26 path=apps/browser/Cargo.toml
what: the browser app manifest — its features, accessibility off by default, the hybrid feature renamed vello-gpu; edited
@task_file n=27 path=apps/browser/src/capture.rs
what: the browser app's screenshot capture; ported to AnyRender 0.15; edited
@task_file n=28 path=examples/seven_guis/Cargo.toml
what: the seven_guis stand app manifest — its features, the hybrid feature renamed vello-gpu; edited
@task_file n=29 path=examples/todomvc/Cargo.toml
what: the todomvc example manifest — its features renamed vello-gpu; edited
@task_file n=30 path=apps/readme/Cargo.toml
what: the readme app manifest — its features renamed vello-gpu; edited
@task_file n=31 path=.github/workflows/ci.yml
what: the CI workflow — the default-features test job now frees disk and reads df -h before and after the test leg; the a11y leg and the stand-repeat job beside it; edited
@task_file n=32 path=.github/scripts/ci-leg.sh
what: the CI leg runner — its wasm leg now builds with vello-gpu; edited
@task_file n=33 path=.github/scripts/test_ci_workflows.py
what: the CI-script checks — the free-disk guard renamed to test-and-stand-repeat and a disk-reading guard added; edited
@task_file n=34 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand's scroll-into-view check over the CRUD list — re-pinned to the clamped offset after a delete; edited
@task_file n=35 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: the local-file refusal and request initiator check — now names the SVG image load; edited
@task_file n=36 path=tests/blitz-tests/tests/fragment_navigation.rs mark=merge
what: fragment navigation and scroller checks — upstream's percentage-height scroller clamp test added; edited
@task_file n=37 path=justfile
what: the developer recipes — the hybrid recipe renamed vello-gpu; edited
@task_file n=38 path=wpt/runner/src/test_runners/ref_test.rs mark=merge
what: the WPT reftest runner; ported to AnyRender 0.15; edited
@task_file n=39 path=wpt/WPT_COMMIT mark=merge
what: the WPT pin; edited
@task_file n=40 path=docs/parley.md mark=merge
what: the parley text notes; edited by the upstream pick
@task_item n=1 part=touches
The rules on accessibility as a cargo feature (default-on, taken by name in Dioxus, off by default in the browser, always on in the session library, no platform adapter on the stand binaries): every feature list beside them was edited, vello-hybrid and hybrid renamed vello-gpu; the accessibility features themselves did not move, so check each row's stated feature names.
@task_item n=2 part=touches
The a11y CI leg, the accessibility test leg in the slow tier and accessibility checks gating CI: the CI workflow was edited (default-features test job gains free-disk and two df -h steps); the a11y leg stays and read green.
@task_item n=3 part=touches
Stand checks repeated three rounds: the free-disk step was on the stand-repeat job alone and now stands on the default-features test job too; its guard renamed from only-stand-repeat to test-and-stand-repeat; three rounds read 1449 passed, 0 failed, 30 ignored.
@task_item n=4 part=touches
The CRUD list contracts (arrow, Home, End keys and its Tab stop): after a delete shrinks the list its scroll offset is now clamped to 0 at layout, so a row the scroll-into-view check pinned as refused off-screen is no longer refused; the keys themselves unchanged.
@task_item n=5 part=touches
Driver click refusals (covered, off-screen) and driver click in a shown scrollbar band: scrolling now clamps offsets to the scroll range at layout and on removal of a subtree, which can change what a click reads as off-screen or scrolled.
@task_item n=6 part=touches
Open select list paint rules (engine-constant colours, rows of one height, no motion, the multiple select chosen-row pair): the select list painter was ported to AnyRender 0.15; pixels differ slightly (anti-aliasing), colours and heights not meant to move.
@task_item n=7 part=touches
Scrollbar thumb contrast stroke and the near-white author thumb visible state change: the element and form control painters were ported to AnyRender 0.15.
@task_item n=8 part=touches
Stand contrast measured from computed styles, the lint reading no finding at boot, contrast unread past non-colour paint: the paint stack moved under them; the a11y leg green, no token moved.
@task_item n=9 part=touches
The driver's screenshot contracts (element shot, row shot) and a screenshot changes nothing: the screenshot path rides on the new renderer; the six stand tasks differ by 427 to 2618 pixels, max channel delta 23 to 30.
@task_item n=10 part=touches
Hidpi or zoom change reshapes text and password caret by bullets: text and glyph paint ported, two skrifa and read-fonts versions now in the lock beside parley's.
@task_item n=11 part=touches
The focus outline of the Cells grid and :focus paint: border and outline paint ported to AnyRender 0.15.
@task_item n=12 part=touches
Removing the focused node drops focus with blur steps, and the changed set marks: subtree removal in the mutator now also resets the removed nodes' scroll state.
@task_item n=13 part=touches
Empty-alt image read as decorative and the accessibility tree built through the document's own method: the document's image fetch moved into the document and now also loads SVG image and feImage href.
@task_item n=14 part=touches
The renderer names: the GPU renderer feature name vello-hybrid freed and vello-gpu taken; any row naming the hybrid renderer or its feature goes stale.
@task_fact n=1 slug=keyboard-navigation
fact: A scroller's offset is clamped to its scroll range at layout, so after a delete shrinks the CRUD list its offset reads 0 and its first row lies inside the list's box, in both layout modes.
@task_item n=1 part=untouched
ARIA role mapping, ARIA states and property reads, accessible name precedence and the label rules: no role, state or name reading moved.
@task_item n=2 part=untouched
The snapshot contracts, the snapshot text form, the snapshot diff and the UI graph: no snapshot field, token or node key moved.
@task_item n=3 part=untouched
Keyboard activation, Tab order, range input keys and the single and multiple select key and event contracts: no key handling or event order moved.
@task_item n=4 part=untouched
Hidden and inert content kept out of the tree and inert hit-testing: no visibility or inertness rule moved.
@task_item n=5 part=untouched
The WCAG contrast ratio helper, the required ratio per SC 1.4.3 and the lint contrast finding fields: no formula, threshold or finding field moved.
@task_item n=6 part=untouched
Cognitive accessibility, the invalid-state rules and the shared tab-order table: nothing reached them.
@task_item n=7 part=untouched
The AccessKit adapter, its event handler and the actions verb: no adapter, request or action moved.
