@answer n=1 entity=border-radius#1.1 v=no
why: browser chrome radii in browser.css; the chunk changes how mutations are applied, not CSS, and the scope names documents, ids and events, not paint values
@answer n=2 entity=border-radius#1.2 v=no
why: browser about-page radii in about-*.css; those stylesheets are not reached and no scoped check reads these radii
@answer n=3 entity=border-radius#1.3 v=no
why: a pointer that the stand's radii live in layout-templates; it states no radius value that the stand_* witnesses re-assert
@answer n=4 entity=border-radius#1.4 v=no
why: transparent example card radius in examples/transparent; that example is not reached by the chunk
@answer n=5 entity=border-radius#1.5 v=no
why: blitz-dom default button radius in default.css; the engine default stylesheet is untouched and no probe check reads it
@answer n=6 entity=border-radius#1.6 v=no
why: checkbox frame radius in blitz-paint; the chunk paints and measures no form control
@answer n=7 entity=border-radius#1.7 v=no
why: scrollbar thumb radius in blitz-paint; the item 3 scroll rule concerns the programmatic into-view write, not thumb paint
@answer n=8 entity=border-radius#2.1 v=no
why: per-corner radius resolution in blitz-paint is a paint path the chunk uses unchanged; the chunk's code and tests neither pass nor measure these values
@answer n=9 entity=border-radius#2.2 v=no
why: inset() clip-path ignoring border-radius; clip_path.rs is untouched and the chunk adds no clip-path
@answer n=10 entity=border-radius#2.3 v=no
why: img border attribute mapping in blitz-dom stylo.rs; the buffer passes attribute names through the existing writer unchanged, so the mapping is not re-checked
@answer n=11 entity=border-radius#3.1 v=no
why: radius examples in examples/assets; fixtures the chunk does not reach
@answer n=12 entity=border-radius#3.2 v=no
why: google and graphite radius fixtures in examples/assets; not reached by the chunk
@answer n=13 entity=border-radius#3.3 v=no
why: inline-flex-transform button radius fixture and transforms example; not reached by the chunk
@answer n=14 entity=border-radius#3.4 v=no
why: servo, shadow and preact radius fixtures in examples; not reached by the chunk
@answer n=15 entity=border-radius#4.1 v=no
why: absence of radius tokens over the s05 files; the chunk introduces no radius or token, so the absence is not moved
@answer n=16 entity=border-radius#4.2 v=no
why: absence of radius, box-shadow and z-index handling over the s06 files; the chunk adds no styling
@answer n=17 entity=border-radius#4.3 v=no
why: absence of radius values over the s07 files; the chunk adds no radius
@answer n=18 entity=border-radius#4.4 v=no
why: absence of border-radius over the s08 files; the chunk adds no radius
@answer n=19 entity=border-radius#4.5 v=no
why: absence of radius definitions over the s10 files; the chunk adds no radius
@answer n=20 entity=border-radius#4.6 v=no
why: absence of radius, shadow, elevation and z-index over the s11 files; the chunk adds no styling, so the absence is not moved
