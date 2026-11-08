@answer n=1 entity=input-validation#1.26 v=no
why: the data URL net provider sits in blitz-shell net.rs, which the change does not reach
@answer n=2 entity=input-validation#1.27 v=no
why: the script fetch bytes check sits in blitz-vibey-script, which did not change
@answer n=3 entity=input-validation#1.28 v=no
why: the file URL script loader sits in blitz-vibey-script, which did not change
@answer n=4 entity=input-validation#1.29 v=no
why: the script type check sits in blitz-vibey-script document.rs, which did not change
@answer n=5 entity=input-validation#1.30 v=no
why: the scripting flag sits in blitz-html, which did not change
@answer n=6 entity=input-validation#1.31 v=no
why: the stylesheet UTF-8 and blocking rule sits in blitz-dom net.rs, which did not change and is not a reached file
@answer n=7 entity=input-validation#1.32 v=no
why: the stylesheet href resolution sits in blitz-dom mutator.rs, which did not change and is not a reached file
@answer n=8 entity=input-validation#1.33 v=no
why: the iframe HTML decode sits in blitz-dom net.rs, which did not change and is not a reached file
@answer n=9 entity=input-validation#1.34 v=no
why: the image format decode sits in blitz-dom net.rs, which did not change and is not a reached file
@answer n=10 entity=input-validation#1.35 v=no
why: the favicon check sits in apps/browser favicon.rs, and the browser app is untouched
@answer n=11 entity=input-validation#1.36 v=no
why: the font format sniff sits in blitz-dom net.rs, which did not change, and the session brings no font fact
@answer n=12 entity=input-validation#1.37 v=no
why: the font length and compression fallback sits in blitz-dom util.rs, which did not change and is not a reached file
@answer n=13 entity=input-validation#1.38 v=no
why: the claim that no driver verb takes a URL still holds because graph takes no argument and an argument is refused, and blitz-dom net.rs did not change
@answer n=14 entity=input-validation#1.39 v=no
why: the SVG DTD parse sits in blitz-dom svg.rs, which did not change and is not a reached file
@answer n=15 entity=input-validation#1.40 v=no
why: the SVGZ decompression sits in blitz-dom svg.rs, which did not change and is not a reached file
@answer n=16 entity=input-validation#1.41 v=no
why: the SVG viewBox check sits in blitz-dom svg.rs, which did not change and is not a reached file
@answer n=17 entity=input-validation#1.42 v=no
why: the inline SVG parse sits in blitz-dom layout construct.rs, which did not change and is not a reached file
@answer n=18 entity=input-validation#1.43 v=no
why: the CSS declaration validation sits in blitz-dom resolved_style.rs and cssom.rs, which did not change and are not reached files
@answer n=19 entity=input-validation#1.44 v=no
why: the mix-blend-mode and isolation paint rule sits in blitz-paint and blitz-dom node.rs, which did not change and are not reached files
@answer n=20 entity=input-validation#1.45 v=no
why: the set and remove style property rejection sits in blitz-dom element.rs, which did not change and is not a reached file
@answer n=21 entity=input-validation#1.46 v=no
why: the selectorText setter rule sits in blitz-dom cssom.rs, which did not change and is not a reached file
@answer n=22 entity=input-validation#1.47 v=no
why: the query string parse sits in blitz-dom query_selector.rs, which did not change and is not a reached file
@answer n=23 entity=input-validation#1.48 v=no
why: the style element entity decode sits in blitz-dom document.rs, which did not change, and the reached file is dioxus_document.rs, a different file
@answer n=24 entity=input-validation#1.49 v=no
why: the bgcolor parse sits in blitz-dom stylo.rs, which did not change and is not a reached file, and the markup fact concerns element ids, not legacy attributes
@answer n=25 entity=input-validation#1.50 v=no
why: the legacy dimension parse sits in blitz-dom stylo.rs, which did not change and is not a reached file
