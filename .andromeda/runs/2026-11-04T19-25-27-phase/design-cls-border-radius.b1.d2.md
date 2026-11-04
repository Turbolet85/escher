@answer n=1 entity=border-radius#1.1 v=no
why: browser chrome radius values; the probe reads components, handlers and state and styles nothing, and the browser shell is not in its scope
@answer n=2 entity=border-radius#1.2 v=no
why: browser about-page radius values; the probe reads no radius and paints nothing
@answer n=3 entity=border-radius#1.3 v=no
why: a pointer that the stand's radii live in layout-templates, not design-system values; the probe styles nothing and reads no radius
@answer n=4 entity=border-radius#1.4 v=no
why: transparent example card radius; the probe does not touch the transparent example
@answer n=5 entity=border-radius#1.5 v=no
why: blitz-dom default button radius; the probe paints nothing, so the engine default is not read
@answer n=6 entity=border-radius#1.6 v=no
why: engine checkbox frame radius; the probe does not paint form controls
@answer n=7 entity=border-radius#1.7 v=no
why: engine scrollbar thumb radius; the probe does no scrolling and reads no scrollbar
@answer n=8 entity=border-radius#2.1 v=no
why: engine per-corner radius resolution at paint time; the probe reads no paint output and no computed radius
@answer n=9 entity=border-radius#2.2 v=no
why: inset() clip-path TODO on radius; clip-path and hit-testing are outside the probe, which does not read or change them
@answer n=10 entity=border-radius#2.3 v=no
why: img and image-input border attribute mapping; the probe reads no image or border attribute
@answer n=11 entity=border-radius#3.1 v=no
why: radius syntax examples in the examples assets; the probe does not read those fixtures
@answer n=12 entity=border-radius#3.2 v=no
why: google and graphite fixture radii; web fixtures outside the seven_guis probe
@answer n=13 entity=border-radius#3.3 v=no
why: inline-flex-transform fixture and transforms example radius; outside the probe's scope
@answer n=14 entity=border-radius#3.4 v=no
why: servo, shadow, preact and reference-page radii; outside the seven_guis probe and no radius is read
@answer n=15 entity=border-radius#4.1 v=no
why: absence record of a border-radius search over the s05 files; a search result that states no rule the probe must honour
@answer n=16 entity=border-radius#4.2 v=no
why: absence record of a border-radius and box-shadow search over the s06 files; the probe does not read shadows, z-index or hit-testing
@answer n=17 entity=border-radius#4.3 v=no
why: absence record of a radius search over the s07 slice files; no radius rule for the probe to honour
@answer n=18 entity=border-radius#4.4 v=no
why: absence record of a border-radius search over the s08 files with no match; no rule the probe must honour
@answer n=19 entity=border-radius#4.5 v=no
why: absence record of a radius search over the s10 slice files; no radius rule for the probe to honour
@answer n=20 entity=border-radius#4.6 v=no
why: absence record of a radius, shadow and elevation search over the s11 files with only a touch-point match; the probe does not read elevation or shadows
