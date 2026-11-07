@answer n=1 entity=iconography#1.1 v=no
why: the cursor-selection rule (CSS keyword, else Text, Pointer, Default) is not reached; the chunk changes how mutations are applied, not cursor selection in blitz-dom
@answer n=2 entity=iconography#1.2 v=no
why: the CSS cursor keyword to CursorIcon mapping and ShellProvider::set_cursor are untouched; the chunk's new code calls no cursor path
@answer n=3 entity=iconography#1.3 v=no
why: favicon_url reads the first link rel=icon, which the chunk neither calls nor changes; favicons are not touched
@answer n=4 entity=iconography#2.1 v=no
why: a fixture enumerating cursor values under examples/assets, which the scope does not name
@answer n=5 entity=iconography#2.2 v=no
why: inline SVG octicon and google paths in example fixtures, which the scope does not name; the re-asserted tests are not these assets
@answer n=6 entity=iconography#2.3 v=no
why: Font Awesome classes and a CDN-loaded icon font in example fixtures, which the scope does not name
@answer n=7 entity=iconography#2.4 v=no
why: graphite's sprite atlas and data-URI SVG icons in an example fixture, which the scope does not name
@answer n=8 entity=iconography#2.5 v=no
why: inline currentColor SVG in example fixtures and the svg_native example, none named by the scope; the svg_native example is not touched
@answer n=9 entity=iconography#3.1 v=no
why: an absence claim about icon handling in the listed s08 files; the chunk touches no icon code, so no rule over it binds
@answer n=10 entity=iconography#3.2 v=no
why: an absence claim about icon sets in the listed s11 files; the chunk touches no icon code, so no rule over it binds
