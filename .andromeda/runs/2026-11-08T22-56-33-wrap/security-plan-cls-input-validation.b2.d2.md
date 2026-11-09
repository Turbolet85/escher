@answer n=1 entity=input-validation#1.26 v=no
why: its scheme gate lives in apps/browser/src/nav.rs and packages/dioxus-native/src/link_handler.rs, neither reached by the session
@answer n=2 entity=input-validation#1.27 v=no
why: href resolution lives in packages/blitz-dom/src/events/pointer.rs and url.rs, which the session does not reach
@answer n=3 entity=input-validation#1.28 v=no
why: DocumentUrl parsing in packages/blitz-dom/src/url.rs is outside the session's reached files
@answer n=4 entity=input-validation#1.29 v=no
why: fragment decoding in packages/blitz-dom/src/scrolling.rs is outside the session's reached files
@answer n=5 entity=input-validation#1.30 v=no
why: file: URL handling in packages/blitz-net/src/lib.rs is untouched, and the coverage role reads paths through File::open, not the net provider
@answer n=6 entity=input-validation#1.31 v=no
why: data: URL decoding in packages/blitz-net/src/lib.rs is outside the session's reached files
@answer n=7 entity=input-validation#1.32 v=no
why: DataUriNetProvider in packages/blitz-shell/src/net.rs is outside the session's reached files
@answer n=8 entity=input-validation#1.33 v=no
why: script fetch in packages/blitz-vibey-script/src/fetch.rs is outside the session's reached files
@answer n=9 entity=input-validation#1.34 v=no
why: the file: URL gate in packages/blitz-vibey-script is outside the session's reached files and the session adds no fetch path
@answer n=10 entity=input-validation#1.35 v=no
why: the script type check in packages/blitz-vibey-script/src/document.rs is outside the session's reached files
@answer n=11 entity=input-validation#1.36 v=no
why: HTML scripting flag in packages/blitz-html is outside the session's reached files
@answer n=12 entity=input-validation#1.37 v=no
why: stylesheet UTF-8 handling lives in packages/blitz-dom/src/net.rs and its render-blocking tests, none reached by the session
@answer n=13 entity=input-validation#1.38 v=no
why: stylesheet href resolution in packages/blitz-dom/src/mutator.rs is outside the session's reached files
@answer n=14 entity=input-validation#1.39 v=no
why: iframe HTML decoding in packages/blitz-dom/src/net.rs is outside the session's reached files
@answer n=15 entity=input-validation#1.40 v=no
why: image decoding in packages/blitz-dom/src/net.rs is outside the session's reached files
@answer n=16 entity=input-validation#1.41 v=no
why: favicon validation in apps/browser/src/favicon.rs is outside the session's reached files
@answer n=17 entity=input-validation#1.42 v=no
why: font format sniffing in packages/blitz-dom/src/net.rs and its font_face_sniffed_source test are outside the session's reached files
@answer n=18 entity=input-validation#1.43 v=no
why: font length and compression fallback in packages/blitz-dom/src/util.rs is outside the session's reached files
@answer n=19 entity=input-validation#1.44 v=yes mode=stale
why: its stand boot claim (stand::boot_load and the in-process LoadNet) sits in tests/blitz-tests/tests/common/mod.rs, which the session reaches whole, and the report does not show that boot unchanged
@answer n=20 entity=input-validation#1.45 v=no
why: SVG encoding and DTD handling in packages/blitz-dom/src/node/svg.rs is outside the session's reached files
@answer n=21 entity=input-validation#1.46 v=no
why: SVGZ detection in packages/blitz-dom/src/node/svg.rs is outside the session's reached files
@answer n=22 entity=input-validation#1.47 v=no
why: viewBox parsing in packages/blitz-dom/src/node/svg.rs is outside the session's reached files
@answer n=23 entity=input-validation#1.48 v=no
why: inline SVG parse handling in packages/blitz-dom/src/layout/construct.rs is outside the session's reached files
@answer n=24 entity=input-validation#1.49 v=no
why: CSS declaration validation in packages/blitz-dom/src/resolved_style.rs and cssom.rs is outside the session's reached files
@answer n=25 entity=input-validation#1.50 v=no
why: blend and isolation handling in packages/blitz-paint is outside the session's reached files
