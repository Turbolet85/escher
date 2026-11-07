@answer n=1 entity=border-radius#1.1 v=no
why: browser chrome radii are untokenized CSS values in browser.css that the chunk neither edits nor re-asserts; its checks cover document build, ids, diff and events, not painted radius
@answer n=2 entity=border-radius#1.2 v=no
why: browser about-page radii are CSS values the chunk does not edit or re-measure
@answer n=3 entity=border-radius#1.3 v=no
why: the stand's radii are held in layout-templates, not design-system values, and the stand_* checks read the document, not radii
@answer n=4 entity=border-radius#1.4 v=no
why: the transparent example card is an example app the chunk does not touch
@answer n=5 entity=border-radius#1.5 v=no
why: the blitz-dom default button radius is an engine paint default; the chunk does not change blitz-dom and its re-asserted snapshot and diff read document structure, not painted radius
@answer n=6 entity=border-radius#1.6 v=no
why: the checkbox frame radius is engine paint in form_controls; no form control is touched and no paint is re-measured
@answer n=7 entity=border-radius#1.7 v=no
why: the scrollbar thumb radius is engine paint; the scroll calls reached through MountedData do not change thumb rendering
@answer n=8 entity=border-radius#2.1 v=no
why: per-corner radius resolution is blitz-paint's pipeline; the chunk changes how mutations are applied, not paint, and states no call or value it must act on
@answer n=9 entity=border-radius#2.2 v=no
why: the inset() clip-path TODO is a paint-path fact the chunk does not reach
@answer n=10 entity=border-radius#2.3 v=yes
why: the buffer records SetAttribute names and values for replay, and this entity's mapping keys on the `border` attribute name on img, so the replay must pass that attribute through unaltered
@answer n=11 entity=border-radius#3.1 v=no
why: radius examples in examples/assets are fixtures the chunk does not touch
@answer n=12 entity=border-radius#3.2 v=no
why: site-clone fixture radii in examples/assets are not touched by the chunk
@answer n=13 entity=border-radius#3.3 v=no
why: the inline-flex-transform fixture and transforms example radii are not touched by the chunk
@answer n=14 entity=border-radius#3.4 v=no
why: the servo.css, shadow fixture and preact example radii are not touched by the chunk
@answer n=15 entity=border-radius#4.1 v=no
why: an absence of radius values in the s05 files; the chunk adds no radius, so the absence cannot be broken by it
@answer n=16 entity=border-radius#4.2 v=no
why: an absence of radius, box-shadow and z-index handling in the s06 files; the chunk adds no such handling
@answer n=17 entity=border-radius#4.3 v=no
why: an absence of radius values in the s07 slice; the chunk adds no radius
@answer n=18 entity=border-radius#4.4 v=no
why: an absence of border-radius in the s08 files; the chunk adds no border-radius
@answer n=19 entity=border-radius#4.5 v=no
why: an absence of radius definitions in the s10 files; the chunk adds no radius definition
@answer n=20 entity=border-radius#4.6 v=no
why: an absence of radius, shadow, elevation and z-index in the s11 files; the chunk adds none of them
