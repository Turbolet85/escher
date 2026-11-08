@answer n=1 entity=input-validation#1.26 v=no
why: external-browser navigation method and scheme in apps/browser nav.rs; the chunk adds no navigation
@answer n=2 entity=input-validation#1.27 v=no
why: link href resolution in blitz-dom pointer dispatch; the chunk adds no navigation and the scope names no href step
@answer n=3 entity=input-validation#1.28 v=no
why: DocumentUrl parsing in blitz-dom url.rs; the chunk parses no document URL
@answer n=4 entity=input-validation#1.29 v=no
why: URL fragment matching in blitz-dom scrolling.rs; the chunk reads no fragment
@answer n=5 entity=input-validation#1.30 v=no
why: file: URL handling in the blitz-net provider; the chunk fetches nothing and names no file path
@answer n=6 entity=input-validation#1.31 v=no
why: data: URL decoding in the blitz-net provider; the chunk fetches nothing
@answer n=7 entity=input-validation#1.32 v=no
why: data: URL handling in the blitz-shell DataUriNetProvider; the chunk adds no fetch
@answer n=8 entity=input-validation#1.33 v=no
why: script byte fetch in blitz-vibey-script; the chunk runs no document script
@answer n=9 entity=input-validation#1.34 v=no
why: file: URL script loading in blitz-vibey-script; the chunk runs no document script
@answer n=10 entity=input-validation#1.35 v=no
why: script type attribute gate in blitz-vibey-script; the chunk runs no script
@answer n=11 entity=input-validation#1.36 v=no
why: HTML scripting flag in blitz-html parsing; the scope names no parse behaviour and the chunk reads no parsed script
@answer n=12 entity=input-validation#1.37 v=no
why: stylesheet UTF-8 and render-blocking in blitz-dom net; the scope names no stylesheet load and the chunk reads no render-block state
@answer n=13 entity=input-validation#1.38 v=no
why: stylesheet href resolution in blitz-dom mutator; the chunk loads no stylesheet
@answer n=14 entity=input-validation#1.39 v=no
why: iframe HTML decoding in blitz-dom net; the chunk fetches no iframe
@answer n=15 entity=input-validation#1.40 v=no
why: image decoding in blitz-dom net; the chunk fetches no image
@answer n=16 entity=input-validation#1.41 v=no
why: favicon decoding in apps/browser; the chunk has no favicon
@answer n=17 entity=input-validation#1.42 v=no
why: font format sniffing in blitz-dom net; the chunk fetches no font
@answer n=18 entity=input-validation#1.43 v=no
why: font length and WOFF fallback in blitz-dom util; the chunk reads no font bytes
@answer n=19 entity=input-validation#1.44 v=no
why: @font-face source URL in blitz-dom net; the chunk adds no font source and its new verb takes no URL, so the stand's offline load page is not the rule's subject
@answer n=20 entity=input-validation#1.45 v=no
why: SVG DTD parsing in blitz-dom svg; the chunk parses no SVG
@answer n=21 entity=input-validation#1.46 v=no
why: SVGZ decompression in blitz-dom svg; the chunk parses no SVG
@answer n=22 entity=input-validation#1.47 v=no
why: SVG viewBox numbers in blitz-dom svg; the chunk reads no viewBox
@answer n=23 entity=input-validation#1.48 v=no
why: inline SVG parse in blitz-dom layout construct; the chunk changes no layout code
@answer n=24 entity=input-validation#1.49 v=no
why: CSS declaration validation in blitz-dom resolved_style; the chunk reads no style value and the UI graph carries no style
@answer n=25 entity=input-validation#1.50 v=no
why: mix-blend-mode and isolation painting in blitz-paint; the chunk changes no paint code
