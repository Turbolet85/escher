@answer n=1 entity=border-radius#1.1 v=no
why: browser chrome radii in the browser app assets; the chunk touches no browser app surface and changes no box shape
@answer n=2 entity=border-radius#1.2 v=no
why: browser about-page radii in the browser app assets; the chunk touches no browser app surface
@answer n=3 entity=border-radius#1.3 v=no
why: the stand's radii are held in layout-templates, which the chunk does not edit; the chunk moves only the stand's signals onto the registry and changes no box shape
@answer n=4 entity=border-radius#1.4 v=no
why: transparent example card radius in examples/transparent, a file the chunk does not reach
@answer n=5 entity=border-radius#1.5 v=no
why: blitz-dom default button radius is an engine default the graph read does not paint; the chunk changes no box shape
@answer n=6 entity=border-radius#1.6 v=no
why: checkbox frame radius is engine paint the chunk does not call or read; the chunk paints nothing
@answer n=7 entity=border-radius#1.7 v=no
why: scrollbar thumb radius is engine paint the chunk does not call or read; the chunk paints nothing
@answer n=8 entity=border-radius#2.1 v=no
why: per-corner radius resolution in blitz-paint is engine paint the chunk does not call or read
@answer n=9 entity=border-radius#2.2 v=no
why: inset clip-path ignoring border-radius is clipping and hit-testing, which the graph read does not do
@answer n=10 entity=border-radius#2.3 v=no
why: the border attribute on img is image rendering the chunk does not touch
@answer n=11 entity=border-radius#3.1 v=no
why: radius syntax fixtures under examples/assets are example files the chunk does not reach
@answer n=12 entity=border-radius#3.2 v=no
why: google, graphite, hr and input fixture radii are example files the chunk does not reach
@answer n=13 entity=border-radius#3.3 v=no
why: inline-flex-transform and transforms example radii are example files the chunk does not reach
@answer n=14 entity=border-radius#3.4 v=no
why: servo, shadow and preact fixture radii are example files the chunk does not reach
@answer n=15 entity=border-radius#4.1 v=no
why: absence search for border-radius values over the s05 files; the chunk adds no radius code, so the absence stays true for it
@answer n=16 entity=border-radius#4.2 v=no
why: absence search over the s06 files for radius, shadow and z-index; the chunk adds no such code
@answer n=17 entity=border-radius#4.3 v=no
why: absence search over the s07 slice for radius; the chunk adds no radius code
@answer n=18 entity=border-radius#4.4 v=no
why: absence search over the s08 files for radius; the chunk adds no radius code
@answer n=19 entity=border-radius#4.5 v=no
why: absence search over the s10 slice files for radius definitions; the chunk adds no radius code
@answer n=20 entity=border-radius#4.6 v=no
why: absence search over the s11 files for radius, shadow, elevation and z-index; the chunk adds none of these
