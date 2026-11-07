@answer n=1 entity=input-validation#1.26 v=no
why: the session changes only the dioxus-native-dom render path, and the data: URL net provider in blitz-shell net.rs is not in its reach
@answer n=2 entity=input-validation#1.27 v=no
why: the script-fetch bytes check in blitz-vibey-script fetch.rs is not in the session's changed files and the session adds no fetch path
@answer n=3 entity=input-validation#1.28 v=no
why: the file: URL document check in blitz-vibey-script is not in the session's changed files and the session adds no URL load
@answer n=4 entity=input-validation#1.29 v=no
why: the script type allow-list in blitz-vibey-script document.rs is not in the session's changed files
@answer n=5 entity=input-validation#1.30 v=no
why: the scripting-disabled HTML parse in blitz-html html_sink.rs is not in the session's changed files, and the session changes Dioxus rendering, not HTML parsing
@answer n=6 entity=input-validation#1.31 v=no
why: the stylesheet UTF-8 and failed-load rules in blitz-dom net.rs are not in the session's changed files and stylesheet loading is unchanged
@answer n=7 entity=input-validation#1.32 v=no
why: the stylesheet href resolution in blitz-dom mutator.rs and its test are not in the session's changed files
@answer n=8 entity=input-validation#1.33 v=no
why: the iframe HTML lossy decode in blitz-dom net.rs is not in the session's changed files
@answer n=9 entity=input-validation#1.34 v=no
why: the image format decode in blitz-dom net.rs is not in the session's changed files
@answer n=10 entity=input-validation#1.35 v=no
why: the favicon byte check in apps/browser favicon.rs is not in the session's changed files
@answer n=11 entity=input-validation#1.36 v=no
why: the font format sniff in blitz-dom net.rs and its test are not in the session's changed files
@answer n=12 entity=input-validation#1.37 v=no
why: the font length and WOFF fallback in blitz-dom util.rs are not in the session's changed files
@answer n=13 entity=input-validation#1.38 v=no
why: the @font-face URL skip in blitz-dom net.rs is not in the session's changed files, and the session adds no URL or net path, so the stand's one fixed load-page request and the no-agent-URL claim still hold
@answer n=14 entity=input-validation#1.39 v=no
why: the SVG encoding and DTD rule in blitz-dom node/svg.rs is not in the session's changed files
@answer n=15 entity=input-validation#1.40 v=no
why: the SVGZ magic-byte decompression in blitz-dom node/svg.rs is not in the session's changed files
@answer n=16 entity=input-validation#1.41 v=no
why: the viewBox number check in blitz-dom node/svg.rs is not in the session's changed files
@answer n=17 entity=input-validation#1.42 v=no
why: the inline SVG parse in blitz-dom layout/construct.rs is not in the session's changed files
@answer n=18 entity=input-validation#1.43 v=no
why: the CSS declaration validation in blitz-dom resolved_style.rs and cssom.rs is not in the session's changed files
@answer n=19 entity=input-validation#1.44 v=no
why: the mix-blend-mode and isolation paint rules in blitz-paint are not in the session's changed files and no paint path changed
@answer n=20 entity=input-validation#1.45 v=no
why: the set_style_property and remove_style_property rejection in blitz-dom node/element.rs is not in the session's changed files
@answer n=21 entity=input-validation#1.46 v=no
why: the selectorText setter in blitz-dom cssom.rs is not in the session's changed files
@answer n=22 entity=input-validation#1.47 v=no
why: the query-string parse in blitz-dom query_selector.rs is not in the session's changed files
@answer n=23 entity=input-validation#1.48 v=no
why: the style element entity decode in blitz-dom document.rs is not in the session's changed files
@answer n=24 entity=input-validation#1.49 v=no
why: the legacy bgcolor parse in blitz-dom stylo.rs is not in the session's changed files
@answer n=25 entity=input-validation#1.50 v=no
why: the legacy dimension parse in blitz-dom stylo.rs is not in the session's changed files
