@answer n=1 entity=border-radius#1.1 v=no
why: browser chrome radius values; the chunk does not touch the browser app
@answer n=2 entity=border-radius#1.2 v=no
why: browser about-page radius values; outside the driver read and the browser app
@answer n=3 entity=border-radius#1.3 v=no
why: the stand's card, button and input radii live in layout-templates; the read hit-tests no radius
@answer n=4 entity=border-radius#1.4 v=no
why: transparent example card radius; no example the chunk touches
@answer n=5 entity=border-radius#1.5 v=no
why: blitz-dom default button radius is painted; the read paints nothing
@answer n=6 entity=border-radius#1.6 v=no
why: checkbox frame radius is engine paint; the read paints nothing
@answer n=7 entity=border-radius#1.7 v=no
why: scrollbar thumb radius is engine paint; the read scrolls nothing
@answer n=8 entity=border-radius#2.1 v=no
why: per-corner elliptical radius resolution is paint; the depth strategy's clip rule names overflow and contain, not radius
@answer n=9 entity=border-radius#2.2 v=no
why: inset clip-path ignoring radius is paint; the covered rule names overflow and contain clipping, not clip-path
@answer n=10 entity=border-radius#2.3 v=no
why: img border attribute maps to pixel borders; the read classifies no image node
@answer n=11 entity=border-radius#3.1 v=no
why: example radius values list fixtures the chunk neither reads nor re-measures
@answer n=12 entity=border-radius#3.2 v=no
why: example fixture radii; the chunk's minimal fixture adds no radius and its record sits in the depth and motion lists
@answer n=13 entity=border-radius#3.3 v=no
why: inline-flex .button radius in fixtures; the chunk re-measures no radius
@answer n=14 entity=border-radius#3.4 v=no
why: servo, shadow and preact fixture radii; the chunk re-measures no radius
@answer n=15 entity=border-radius#4.1 v=no
why: absence of radius tokens over the s05 files; the chunk adds no radius, so the check keeps holding
@answer n=16 entity=border-radius#4.2 v=no
why: absence of radius handling and box-shadow over the s06 files; the chunk adds no radius or shadow
@answer n=17 entity=border-radius#4.3 v=no
why: absence of radius over the s07 slice; the chunk introduces no radius
@answer n=18 entity=border-radius#4.4 v=no
why: absence of radius over the s08 files; the chunk introduces no radius
@answer n=19 entity=border-radius#4.5 v=no
why: absence of radius definitions over the s10 files; the chunk introduces no radius
@answer n=20 entity=border-radius#4.6 v=no
why: absence of radius tokens over the s11 files; the chunk introduces no radius
