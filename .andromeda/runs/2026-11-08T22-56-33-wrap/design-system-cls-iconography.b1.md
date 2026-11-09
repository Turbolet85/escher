@answer n=1 entity=iconography#1.1 v=no
why: the cursor selection rule lives in packages/blitz-dom/src/document.rs, which no session file reaches, and the task marks cursors untouched
@answer n=2 entity=iconography#1.2 v=no
why: the CSS cursor mapping and shell cursor calls live in blitz-dom, blitz-traits and blitz-shell, none reached by the session, and cursors are marked untouched
@answer n=3 entity=iconography#1.3 v=no
why: favicon_url lives in packages/blitz-dom/src/document.rs, which the session does not reach, and favicons are marked untouched
@answer n=4 entity=iconography#2.1 v=no
why: the cursor fixture examples/assets/cursor.html is not a reached file and no cursor value changed
@answer n=5 entity=iconography#2.2 v=no
why: the octicon and google inline SVG fixtures are not reached files and the session adds no icon
@answer n=6 entity=iconography#2.3 v=no
why: the Font Awesome fixtures are not reached files and the session adds no icon or font
@answer n=7 entity=iconography#2.4 v=no
why: the graphite sprite and data-URI fixture is not a reached file and the session adds no icon
@answer n=8 entity=iconography#2.5 v=no
why: the currentColor SVG fixtures, examples/svg_native.rs and the Preact fixture are not reached files and the session adds no colour or icon
@answer n=9 entity=iconography#3.1 v=no
why: no reached file is an icon source or in the icon-search list, the session adds no icon text, and icons are marked untouched
@answer n=10 entity=iconography#3.2 v=no
why: no reached file is an icon source or in the icon-search list, the session adds no icon text, and icons are marked untouched
