@answer n=1 entity=border-radius#1.1 v=no
why: browser chrome radii live in the browser app's CSS; the chunk edits no browser asset and draws no radius
@answer n=2 entity=border-radius#1.2 v=no
why: about-page radii sit in browser CSS the chunk does not touch
@answer n=3 entity=border-radius#1.3 v=no
why: the stand's radii are held in layout-templates and are read by neither the coverage count nor the owed test
@answer n=4 entity=border-radius#1.4 v=no
why: the transparent example card radius; no example is built or replayed by the chunk
@answer n=5 entity=border-radius#1.5 v=no
why: the blitz-dom default button radius is painted, while the coverage read dispatches and paints nothing
@answer n=6 entity=border-radius#1.6 v=no
why: the checkbox frame radius is an engine paint value and the chunk paints nothing
@answer n=7 entity=border-radius#1.7 v=no
why: the scrollbar thumb radius is paint-only and the chunk adds no scrolling
@answer n=8 entity=border-radius#2.1 v=no
why: per-corner radius resolution in blitz-paint is paint-only and the chunk paints nothing
@answer n=9 entity=border-radius#2.2 v=no
why: the inset clip-path radius TODO concerns hit-testing, which the read-only coverage read does not dispatch
@answer n=10 entity=border-radius#2.3 v=no
why: the img border attribute mapping is DOM styling the chunk neither reads nor changes
@answer n=11 entity=border-radius#3.1 v=no
why: radius forms in example fixtures; no fixture is edited or replayed by the chunk
@answer n=12 entity=border-radius#3.2 v=no
why: google, graphite, hr and input fixture radii are outside the chunk's files and scope
@answer n=13 entity=border-radius#3.3 v=no
why: the inline-flex-transform fixture and transforms example radius is outside the chunk's files and scope
@answer n=14 entity=border-radius#3.4 v=no
why: servo.css, shadow and preact example radii are outside the chunk's files and scope
@answer n=15 entity=border-radius#4.1 v=no
why: the absence of radius values over the s05 files is not moved, since the chunk's Rust, test and doc text adds no radius value
@answer n=16 entity=border-radius#4.2 v=no
why: the absence of radius, shadow and z-index terms over the s06 files is not moved, since the chunk adds none of them
@answer n=17 entity=border-radius#4.3 v=no
why: the absence of radius terms over the s07 slice is not moved, since the chunk adds no radius term
@answer n=18 entity=border-radius#4.4 v=no
why: the absence of border-radius terms over the s08 files is not moved, since the chunk adds no radius term
@answer n=19 entity=border-radius#4.5 v=no
why: the absence of radius definitions over the s10 files is not moved, since the chunk defines no radius
@answer n=20 entity=border-radius#4.6 v=no
why: the absence of radius, shadow and z-index terms over the s11 files is not moved, since the chunk adds none of them
