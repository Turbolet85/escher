@answer n=1 entity=iconography#1.1 v=no
why: the cursor resolution in blitz-dom's document.rs is not touched by the buffered render, and the scope names no cursor behaviour
@answer n=2 entity=iconography#1.2 v=no
why: CursorIcon mapping and ShellProvider::set_cursor are untouched by the chunk, which changes only how DioxusDocument applies mutations
@answer n=3 entity=iconography#1.3 v=no
why: favicon_url reads the link rel icon, which the chunk neither changes nor reads
@answer n=4 entity=iconography#2.1 v=no
why: the cursor.html fixture is not named by the scope and is not an input the chunk's tests re-assert
@answer n=5 entity=iconography#2.2 v=no
why: the inline SVG octicon and google fixtures are not named by the scope and are not read by the chunk's tests
@answer n=6 entity=iconography#2.3 v=no
why: the Font Awesome class and pseudo-content fixtures are not named by the scope and are not read by the chunk's tests
@answer n=7 entity=iconography#2.4 v=no
why: the graphite sprite-atlas fixture is not named by the scope and is not read by the chunk's tests
@answer n=8 entity=iconography#2.5 v=no
why: the svg.html, svg_native and Preact destroy-button examples are not named by the scope and are not touched by the chunk
@answer n=9 entity=iconography#3.1 v=no
why: the absence of icon handling in the s08 files is a fact the chunk's mutation-application change does not alter
@answer n=10 entity=iconography#3.2 v=no
why: the absence of icon sets in the s11 files is a fact the chunk's mutation-application change does not alter
