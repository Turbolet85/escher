@answer n=1 entity=input-validation#1.26 v=no
why: the rule lives in apps/browser nav.rs and link_handler.rs, which no task file reaches, and the chunk adds no navigation
@answer n=2 entity=input-validation#1.27 v=no
why: the link href rule lives in blitz-dom pointer.rs and url.rs, which no task file reaches, and the chunk resolves no href
@answer n=3 entity=input-validation#1.28 v=no
why: the DocumentUrl parse rule lives in blitz-dom url.rs, which no task file reaches, and the chunk parses no document URL
@answer n=4 entity=input-validation#1.29 v=no
why: the fragment decoding rule lives in blitz-dom scrolling.rs, which no task file reaches, and the chunk matches no fragment
@answer n=5 entity=input-validation#1.30 v=no
why: the file: URL rule lives in blitz-net, which no task file reaches, and the chunk adds no fetch
@answer n=6 entity=input-validation#1.31 v=no
why: the data: URL decoding rule lives in blitz-net, which no task file reaches, and the chunk adds no fetch
@answer n=7 entity=input-validation#1.32 v=no
why: the data: scheme rule lives in blitz-shell net.rs, which no task file reaches, and the chunk fetches nothing
@answer n=8 entity=input-validation#1.33 v=no
why: the script fetch bytes rule lives in blitz-vibey-script, which no task file reaches, and the chunk runs no document script
@answer n=9 entity=input-validation#1.34 v=no
why: the script file: URL rule lives in blitz-vibey-script, which no task file reaches, and the chunk loads no script
@answer n=10 entity=input-validation#1.35 v=no
why: the script type rule lives in blitz-vibey-script document.rs, which no task file reaches, and the chunk executes no script
@answer n=11 entity=input-validation#1.36 v=no
why: the scripting-flag rule lives in blitz-html html_sink.rs, which no task file reaches, and the chunk parses no HTML
@answer n=12 entity=input-validation#1.37 v=no
why: the stylesheet encoding rule lives in blitz-dom net.rs, which no task file reaches, and the chunk fetches no stylesheet
@answer n=13 entity=input-validation#1.38 v=no
why: the stylesheet href rule lives in blitz-dom mutator.rs and net.rs, which no task file reaches, and the chunk resolves no stylesheet href
@answer n=14 entity=input-validation#1.39 v=no
why: the iframe HTML decoding rule lives in blitz-dom net.rs, which no task file reaches, and the chunk fetches no iframe
@answer n=15 entity=input-validation#1.40 v=no
why: the image format rule lives in blitz-dom net.rs, which no task file reaches, and the chunk decodes no image
@answer n=16 entity=input-validation#1.41 v=no
why: the favicon rule lives in apps/browser favicon.rs, which no task file reaches, and the chunk touches no favicon
@answer n=17 entity=input-validation#1.42 v=no
why: the font sniffing rule lives in blitz-dom net.rs, which no task file reaches, and the chunk registers no font
@answer n=18 entity=input-validation#1.43 v=no
why: the font length and compression rule lives in blitz-dom util.rs, which no task file reaches, and the chunk decodes no font bytes
@answer n=19 entity=input-validation#1.44 v=no
why: the @font-face source rule lives in blitz-dom net.rs, which no task file reaches, and the chunk adds no @font-face source or URL path
@answer n=20 entity=input-validation#1.45 v=no
why: the SVG encoding and DTD rule lives in blitz-dom node/svg.rs, which no task file reaches, and the chunk parses no SVG
@answer n=21 entity=input-validation#1.46 v=no
why: the SVGZ decompression rule lives in blitz-dom node/svg.rs, which no task file reaches, and the chunk parses no SVG
@answer n=22 entity=input-validation#1.47 v=no
why: the viewBox rule lives in blitz-dom node/svg.rs, which no task file reaches, and the chunk reads no viewBox
@answer n=23 entity=input-validation#1.48 v=no
why: the inline SVG parse rule lives in blitz-dom layout/construct.rs, which no task file reaches, and the chunk adds no inline SVG
@answer n=24 entity=input-validation#1.49 v=no
why: the CSS declaration rule lives in blitz-dom resolved_style.rs and cssom.rs, which no task file reaches, and the chunk parses no declaration
@answer n=25 entity=input-validation#1.50 v=no
why: the mix-blend-mode and isolation rule lives in blitz-paint blend.rs and render.rs, which no task file reaches, and the chunk paints no blended element
