@answer n=1 entity=input-validation#1.26 v=no
why: external-browser navigation is named not needed (not_needed 1), the chunk adds no navigation and reaches no nav.rs or link_handler.rs
@answer n=2 entity=input-validation#1.27 v=no
why: link href resolution is named not needed (not_needed 1), the chunk resolves no href and reaches no pointer.rs default action
@answer n=3 entity=input-validation#1.28 v=no
why: DocumentUrl parsing is a blitz-dom behaviour the chunk only uses, the scope names no document URL and the chunk passes none
@answer n=4 entity=input-validation#1.29 v=no
why: URL fragment matching is a blitz-dom scrolling behaviour the chunk only uses, the scope names no fragment
@answer n=5 entity=input-validation#1.30 v=no
why: net provider file: URL handling is named not needed (not_needed 1), the chunk adds no fetch and reaches no blitz-net path
@answer n=6 entity=input-validation#1.31 v=no
why: net provider data: URL decoding is named not needed (not_needed 1), the chunk fetches no data URL
@answer n=7 entity=input-validation#1.32 v=no
why: the shell DataUriNetProvider is data: URL handling named not needed (not_needed 1), the chunk fetches no data URL
@answer n=8 entity=input-validation#1.33 v=no
why: script fetch bytes are named not needed (not_needed 2), the chunk runs no document script and reaches no ScriptFetcher
@answer n=9 entity=input-validation#1.34 v=no
why: script fetch of file: URLs is named not needed (not_needed 2), the chunk runs no document script
@answer n=10 entity=input-validation#1.35 v=no
why: the script type attribute gate is named not needed (not_needed 2), the chunk runs no document script
@answer n=11 entity=input-validation#1.36 v=no
why: HTML parsing with scripting disabled is a blitz-html behaviour the chunk only uses, the scope names no scripting flag and the chunk changes no parsing
@answer n=12 entity=input-validation#1.37 v=no
why: stylesheet UTF-8 handling is a blitz-dom loading behaviour the chunk only uses, the scope names no stylesheet load and the check holds with no edit of the chunk
@answer n=13 entity=input-validation#1.38 v=no
why: an unresolvable stylesheet href is a blitz-dom loading rule the chunk only uses, the scope names no href and the chunk starts no load
@answer n=14 entity=input-validation#1.39 v=no
why: iframe HTML decoding is a blitz-dom behaviour the chunk does not use, the scope names no iframe and the chunk fetches none
@answer n=15 entity=input-validation#1.40 v=no
why: fetched image format decoding is a blitz-dom behaviour the chunk does not use, the chunk fetches and decodes no image
@answer n=16 entity=input-validation#1.41 v=no
why: favicon acceptance is an apps/browser rule the chunk does not reach, the chunk stores and fetches no favicon
@answer n=17 entity=input-validation#1.42 v=no
why: font format sniffing is a blitz-dom behaviour the chunk does not reach, the stand boots on its bundled font and the scope names no font source
@answer n=18 entity=input-validation#1.43 v=no
why: font byte length and WOFF fallback are a blitz-dom behaviour the chunk does not reach, no font bytes are read by the chunk
@answer n=19 entity=input-validation#1.44 v=no
why: the font-face source widening is a blitz-dom behaviour the chunk does not use, and the new verb names checks rather than URLs so no URL reaches the font path, a re-check every change owes
@answer n=20 entity=input-validation#1.45 v=no
why: SVG encoding and DTD handling is a blitz-dom behaviour the chunk does not reach, the chunk parses no SVG
@answer n=21 entity=input-validation#1.46 v=no
why: SVGZ decompression is a blitz-dom behaviour the chunk does not reach, the chunk parses no SVG
@answer n=22 entity=input-validation#1.47 v=no
why: SVG viewBox number checks are a blitz-dom behaviour the chunk does not reach, the chunk parses no SVG
@answer n=23 entity=input-validation#1.48 v=no
why: inline SVG parsing in blitz-dom layout is a behaviour the chunk does not use and the chunk changes no layout code
@answer n=24 entity=input-validation#1.49 v=no
why: CSS declaration parsing is a blitz-dom and Stylo behaviour the chunk only uses, the scope names no style value and the chunk reads none
@answer n=25 entity=input-validation#1.50 v=no
why: mix-blend-mode painting is a blitz-paint rule the chunk does not reach, the chunk changes no paint code
