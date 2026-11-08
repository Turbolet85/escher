@answer n=1 entity=input-validation#1.26 v=no
why: the fragment percent-decoding in blitz-dom scrolling.rs is untouched, and the session's changes sit in escher-driver and dioxus-native-dom
@answer n=2 entity=input-validation#1.27 v=no
why: the blitz-net file-URL path is untouched, and the new actions verb takes no URL argument
@answer n=3 entity=input-validation#1.28 v=no
why: blitz-net's data-URL decoding is untouched and no reached file holds it
@answer n=4 entity=input-validation#1.29 v=no
why: blitz-shell's DataUriNetProvider is untouched by the session
@answer n=5 entity=input-validation#1.30 v=no
why: the blitz-vibey-script script fetch is untouched and no reached file holds it
@answer n=6 entity=input-validation#1.31 v=no
why: document_may_load and the script fetchers are untouched, and no reached file holds them
@answer n=7 entity=input-validation#1.32 v=no
why: the blitz-vibey-script script type check is untouched
@answer n=8 entity=input-validation#1.33 v=no
why: the blitz-html parser sink is untouched and the session adds no parse path
@answer n=9 entity=input-validation#1.34 v=no
why: blitz-dom stylesheet loading and its UTF-8 check are untouched, and the session adds no stylesheet load
@answer n=10 entity=input-validation#1.35 v=no
why: blitz-dom stylesheet href resolution and render blocking are untouched
@answer n=11 entity=input-validation#1.36 v=no
why: blitz-dom iframe HTML decoding is untouched
@answer n=12 entity=input-validation#1.37 v=no
why: blitz-dom image decoding is untouched and no reached file holds it
@answer n=13 entity=input-validation#1.38 v=no
why: the apps/browser favicon decoding is untouched by the session
@answer n=14 entity=input-validation#1.39 v=no
why: blitz-dom font format sniffing is untouched and the session adds no font source
@answer n=15 entity=input-validation#1.40 v=no
why: blitz-dom font byte length and decompression handling are untouched
@answer n=16 entity=input-validation#1.41 v=no
why: the font-face skip lives in blitz-dom net.rs, which the session does not reach, and no driver verb takes a URL since actions takes no argument, so the claim still holds
@answer n=17 entity=input-validation#1.42 v=no
why: blitz-dom SVG parsing and its DTD setting are untouched
@answer n=18 entity=input-validation#1.43 v=no
why: blitz-dom SVGZ detection and decompression are untouched
@answer n=19 entity=input-validation#1.44 v=no
why: blitz-dom SVG viewBox parsing is untouched
@answer n=20 entity=input-validation#1.45 v=no
why: the inline SVG parse in blitz-dom layout/construct.rs is untouched
@answer n=21 entity=input-validation#1.46 v=no
why: blitz-dom CSS declaration validation is untouched
@answer n=22 entity=input-validation#1.47 v=no
why: the blitz-paint blend code and its layers are untouched, and the session adds no paint path
@answer n=23 entity=input-validation#1.48 v=no
why: blitz-dom element style setters are untouched
@answer n=24 entity=input-validation#1.49 v=no
why: the blitz-dom cssom selectorText setter is untouched
@answer n=25 entity=input-validation#1.50 v=no
why: blitz-dom query_selector parsing is untouched and the session adds no selector parse
