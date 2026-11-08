@answer n=1 entity=iconography#1.1 v=yes
why: the scope names the engine's cursor reading (Pointer inside links, Default otherwise) as not a reacts-to-a-click signal, so the new field must not be read from it
@answer n=2 entity=iconography#1.2 v=no
why: the CSS cursor keyword to CursorIcon mapping and the shell set_cursor path are not named by the scope and the chunk's new code does not call or read them
@answer n=3 entity=iconography#1.3 v=no
why: favicon_url serves browser favicons, which the chunk does not touch (browser app icon not needed)
@answer n=4 entity=iconography#2.1 v=no
why: a fixture enumerating cursor values under examples/assets is not a file the chunk reaches and no rule it states is one the chunk's code or tests must honour
@answer n=5 entity=iconography#2.2 v=no
why: inline SVG octicons and google SVG paths in captured fixtures are not reached by the graph read and the chunk adds no icon
@answer n=6 entity=iconography#2.3 v=no
why: Font Awesome classes and pseudo-element glyphs in fixtures are not reached by the graph read and the chunk loads no icon font
@answer n=7 entity=iconography#2.4 v=no
why: graphite's sprite atlas and data-URI icons are fixture content the chunk does not reach
@answer n=8 entity=iconography#2.5 v=no
why: inline currentColor SVG icons and the Preact destroy button are fixture or example content the chunk does not reach and does not paint
@answer n=9 entity=iconography#3.1 v=no
why: an absence record over icon assets in s08 files states no rule the chunk's code or tests must honour, and the chunk touches no browser app icon
@answer n=10 entity=iconography#3.2 v=no
why: an absence record over icon sets in s11 files states no rule the chunk's code or tests must honour, and the chunk touches no browser app icon
