@answer n=1 entity=iconography#1.1 v=no
why: cursor selection rule for the shell pointer; the probe reads components, handlers and state, and task item 5 names cursors as not needed
@answer n=2 entity=iconography#1.2 v=no
why: CSS cursor keyword to CursorIcon mapping and shell cursor set; no cursor is read or set by the probe
@answer n=3 entity=iconography#1.3 v=no
why: favicon_url reads the first icon link href; favicons are named as not needed by task item 5
@answer n=4 entity=iconography#2.1 v=no
why: a fixture enumerating 36 CSS cursor values; the probe reads no cursor and no fixture of this kind
@answer n=5 entity=iconography#2.2 v=no
why: inline SVG octicons and google paths in captured fixtures; the probe runs on seven_guis and paints no icon
@answer n=6 entity=iconography#2.3 v=no
why: Font Awesome classes and glyph content in fixtures; icon glyphs are outside the UI graph links the probe reads
@answer n=7 entity=iconography#2.4 v=no
why: graphite sprite atlas and CSS data-URI icons in a fixture; the chunk reads no atlas and paints no icon
@answer n=8 entity=iconography#2.5 v=no
why: inline SVG currentColor icons and a Preact destroy glyph; the probe paints nothing and reads no SVG fill
@answer n=9 entity=iconography#3.1 v=no
why: a search absence over s08 files for icon handling; the chunk's links do not include icon handling
@answer n=10 entity=iconography#3.2 v=no
why: a search absence over s11 files for icon sets; the chunk's links do not include icon sets
