@answer n=1 entity=iconography#1.1 v=no
why: the cursor selection logic lives in packages/blitz-dom/src/document.rs, which the session did not change, and none of the reached files carries it
@answer n=2 entity=iconography#1.2 v=no
why: Cargo.lock moved only the pulldown-cmark lines 6505 and 6507, so the cursor-icon dependency this row cites still holds, and no cursor or shell code changed
@answer n=3 entity=iconography#1.3 v=no
why: favicon_url in packages/blitz-dom/src/document.rs is untouched, and no blitz-dom or apps/browser file changed
@answer n=4 entity=iconography#2.1 v=no
why: the cursor.html fixture is untouched, and the session changed no example asset or cursor value set
@answer n=5 entity=iconography#2.2 v=no
why: the octicon and google SVG fixtures are untouched, and no example asset changed
@answer n=6 entity=iconography#2.3 v=no
why: the Font Awesome fixture references are untouched, and no example asset changed
@answer n=7 entity=iconography#2.4 v=no
why: the graphite fixture is untouched, and no example asset changed
@answer n=8 entity=iconography#2.5 v=no
why: the svg, svg_native and preact example files are untouched, and no example changed
@answer n=9 entity=iconography#3.1 v=no
why: the reached files' changed lines (pulldown-cmark lock lines, deny.toml ignore entry) carry no icon match, and the new probe test reads listeners and state, not icon handling
@answer n=10 entity=iconography#3.2 v=no
why: the same reached-file changes carry no icon match, and the new probe test reads listeners and state, not icon sets
