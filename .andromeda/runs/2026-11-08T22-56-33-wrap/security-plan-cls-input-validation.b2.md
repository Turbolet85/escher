@answer n=1 entity=input-validation#1.26 v=no
why: the navigation gate lives in apps/browser nav.rs and packages/dioxus-native link_handler.rs, neither reached by the session
@answer n=2 entity=input-validation#1.27 v=no
why: href resolution lives in blitz-dom pointer.rs and url.rs, which the session does not reach
@answer n=3 entity=input-validation#1.28 v=no
why: DocumentUrl parsing lives in blitz-dom url.rs, which the session does not reach
@answer n=4 entity=input-validation#1.29 v=no
why: fragment decoding lives in blitz-dom scrolling.rs, which the session does not reach
@answer n=5 entity=input-validation#1.30 v=no
why: file: URL handling lives in blitz-net lib.rs, which the session does not reach
@answer n=6 entity=input-validation#1.31 v=no
why: data: URL decoding lives in blitz-net lib.rs, which the session does not reach
@answer n=7 entity=input-validation#1.32 v=no
why: DataUriNetProvider lives in blitz-shell net.rs, which the session does not reach
@answer n=8 entity=input-validation#1.33 v=no
why: script fetch byte checks live in blitz-vibey-script fetch.rs, which the session does not reach
@answer n=9 entity=input-validation#1.34 v=no
why: the file: URL document gate lives in blitz-vibey-script files the session does not reach
@answer n=10 entity=input-validation#1.35 v=no
why: script type filtering lives in blitz-vibey-script document.rs, which the session does not reach
@answer n=11 entity=input-validation#1.36 v=no
why: the scripting flag lives in blitz-html html_sink.rs, which the session does not reach
@answer n=12 entity=input-validation#1.37 v=no
why: stylesheet UTF-8 handling lives in blitz-dom net.rs, which the session does not reach
@answer n=13 entity=input-validation#1.38 v=no
why: stylesheet href resolution lives in blitz-dom mutator.rs, which the session does not reach
@answer n=14 entity=input-validation#1.39 v=no
why: iframe HTML decoding lives in blitz-dom net.rs, which the session does not reach
@answer n=15 entity=input-validation#1.40 v=no
why: image decoding lives in blitz-dom net.rs, which the session does not reach
@answer n=16 entity=input-validation#1.41 v=no
why: favicon decoding lives in apps/browser favicon.rs, which the session does not reach
@answer n=17 entity=input-validation#1.42 v=no
why: font format sniffing lives in blitz-dom net.rs, which the session does not reach
@answer n=18 entity=input-validation#1.43 v=no
why: font length and WOFF fallback live in blitz-dom util.rs, which the session does not reach
@answer n=19 entity=input-validation#1.44 v=yes mode=stale
why: the session reaches tests/blitz-tests/tests/common/mod.rs, through which this row's stand-boot claim (stand::boot_load, LoadNet) reads, and no shown check holds that claim on the changed file
@answer n=20 entity=input-validation#1.45 v=no
why: SVG DTD parsing lives in blitz-dom svg.rs, which the session does not reach
@answer n=21 entity=input-validation#1.46 v=no
why: SVGZ decompression lives in blitz-dom svg.rs, which the session does not reach
@answer n=22 entity=input-validation#1.47 v=no
why: viewBox validation lives in blitz-dom svg.rs, which the session does not reach
@answer n=23 entity=input-validation#1.48 v=no
why: inline SVG parsing lives in blitz-dom layout construct.rs, which the session does not reach
@answer n=24 entity=input-validation#1.49 v=no
why: CSS declaration parsing lives in blitz-dom resolved_style.rs and cssom.rs, which the session does not reach
@answer n=25 entity=input-validation#1.50 v=no
why: blend mode handling lives in blitz-paint, which the session does not reach
