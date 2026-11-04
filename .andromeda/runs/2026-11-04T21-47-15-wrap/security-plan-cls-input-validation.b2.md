@answer n=1 entity=input-validation#1.26 v=no
why: the session changed no file under packages/blitz-shell, and its lock move (pulldown-cmark 0.13.6 only) does not reach the data URL net provider
@answer n=2 entity=input-validation#1.27 v=no
why: no file under packages/blitz-vibey-script changed, and the lock move does not touch the script fetch decoding at fetch.rs:60-67
@answer n=3 entity=input-validation#1.28 v=no
why: packages/blitz-vibey-script is unchanged and the new probe reads only public Dioxus API, so the file: URL gate stands
@answer n=4 entity=input-validation#1.29 v=no
why: no file under packages/blitz-vibey-script changed, so the script type allow-list at document.rs:307-316 stands
@answer n=5 entity=input-validation#1.30 v=no
why: packages/blitz-html is unchanged and the lock move does not reach the HTML sink's scripting flag
@answer n=6 entity=input-validation#1.31 v=no
why: no file under packages/blitz-dom changed, and the new probe pins no stylesheet loading path
@answer n=7 entity=input-validation#1.32 v=no
why: packages/blitz-dom/src/mutator.rs and the render_blocking_stylesheet pins are unchanged, so the unresolvable href rule stands
@answer n=8 entity=input-validation#1.33 v=no
why: packages/blitz-dom/src/net.rs is unchanged, so the lossy UTF-8 decode of fetched iframe HTML at :552-555 stands
@answer n=9 entity=input-validation#1.34 v=no
why: packages/blitz-dom/src/net.rs:575-610 is unchanged and the lock moves only pulldown-cmark, so the image decode path is not reached
@answer n=10 entity=input-validation#1.35 v=no
why: apps/browser/src/favicon.rs and its tests are unchanged, and no file under apps changed
@answer n=11 entity=input-validation#1.36 v=no
why: packages/blitz-dom/src/net.rs:314-335 is unchanged, and the font_face_sniffed_source witness is not in the session's change
@answer n=12 entity=input-validation#1.37 v=no
why: packages/blitz-dom/src/util.rs is unchanged and no font dependency moved in Cargo.lock
@answer n=13 entity=input-validation#1.38 v=no
why: packages/blitz-dom/src/net.rs:498-504 is unchanged, so the unresolvable font-face source skip and its witness stand
@answer n=14 entity=input-validation#1.39 v=no
why: packages/blitz-dom/src/node/svg.rs is unchanged, and the lock move does not reach the SVG XML parser
@answer n=15 entity=input-validation#1.40 v=no
why: packages/blitz-dom/src/node/svg.rs:93-100 is unchanged, so the SVGZ magic byte detection stands
@answer n=16 entity=input-validation#1.41 v=no
why: packages/blitz-dom/src/node/svg.rs:39-53 is unchanged, so the viewBox number checks stand
@answer n=17 entity=input-validation#1.42 v=no
why: packages/blitz-dom/src/layout/construct.rs:456-476 is unchanged, so the inline SVG parse fallback stands
@answer n=18 entity=input-validation#1.43 v=no
why: no file under packages/blitz-dom changed, so the CSS declaration validation at resolved_style.rs and cssom.rs stands
@answer n=19 entity=input-validation#1.44 v=no
why: packages/blitz-paint and the blend layer code are unchanged, and the new probe paints no blended element
@answer n=20 entity=input-validation#1.45 v=no
why: packages/blitz-dom/src/node/element.rs:807-877 is unchanged, so set_style_property and remove_style_property still reject by returning false
@answer n=21 entity=input-validation#1.46 v=no
why: packages/blitz-dom/src/cssom.rs:858-915 is unchanged, so an invalid selectorText leaves the rule unchanged
@answer n=22 entity=input-validation#1.47 v=no
why: packages/blitz-dom/src/query_selector.rs:204-210 is unchanged, so query string parse errors are still returned
@answer n=23 entity=input-validation#1.48 v=no
why: packages/blitz-dom/src/document.rs:1685-1690 is unchanged, so the style element entity decoding stands
@answer n=24 entity=input-validation#1.49 v=no
why: packages/blitz-dom/src/stylo.rs:906-927 and :1210-1217 are unchanged, so the bgcolor hex rule stands
@answer n=25 entity=input-validation#1.50 v=no
why: packages/blitz-dom/src/stylo.rs:929-958 and :1068-1074 are unchanged, so the legacy dimension parsing stands
