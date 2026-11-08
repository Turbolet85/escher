@answer n=1 entity=iconography#1.1 v=no
why: the cursor rule lives in blitz-dom document.rs, which the session did not reach, and the session states cursor keywords are unchanged
@answer n=2 entity=iconography#1.2 v=no
why: the keyword-to-CursorIcon mapping in blitz-dom, blitz-traits and blitz-shell is outside the reached files, and the session states cursor keywords are unchanged
@answer n=3 entity=iconography#1.3 v=no
why: favicon_url in blitz-dom document.rs is outside the reached files, and the session states favicons are unchanged
@answer n=4 entity=iconography#2.1 v=no
why: the cursor fixture under examples/assets is not a reached file, and the session leaves cursor keywords unchanged
@answer n=5 entity=iconography#2.2 v=no
why: the octicon and google SVG fixtures are not reached files, and the session leaves icons unchanged
@answer n=6 entity=iconography#2.3 v=no
why: the Font Awesome fixtures are not reached files, and the session leaves icons unchanged
@answer n=7 entity=iconography#2.4 v=no
why: the graphite sprite and data-URI fixture is not a reached file, and the session leaves icons unchanged
@answer n=8 entity=iconography#2.5 v=no
why: the svg fixtures, the svg_native example and the preact index are not reached files, and the session leaves icons unchanged
@answer n=9 entity=iconography#3.1 v=no
why: the session reached no file that adds icon text, and its untouched statement keeps icon handling unchanged
@answer n=10 entity=iconography#3.2 v=no
why: the session adds no icon or icon-set text, and its untouched statement keeps icons unchanged
