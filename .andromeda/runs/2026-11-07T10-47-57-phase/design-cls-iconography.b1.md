@answer n=1 entity=iconography#1.1 v=no
why: cursor selection from the CSS cursor keyword and hover targets lives in blitz-dom document.rs, which the chunk does not edit, and task not_needed item 8 excludes cursor logic
@answer n=2 entity=iconography#1.2 v=no
why: CSS cursor keyword mapping and ShellProvider::set_cursor in blitz-traits and blitz-shell are cursor logic the chunk does not change, and task not_needed item 8 excludes them
@answer n=3 entity=iconography#1.3 v=no
why: favicon_url reads the first icon link href, and task not_needed item 7 states favicons are not touched
@answer n=4 entity=iconography#2.1 v=no
why: a cursor-value enumeration in an examples fixture that no chunk file or probe test is stated to drive
@answer n=5 entity=iconography#2.2 v=no
why: inline SVG octicon and google paths in captured example fixtures, which the buffered render neither reads nor changes
@answer n=6 entity=iconography#2.3 v=no
why: Font Awesome class and content rules in example fixtures and a remote font load, none named by the scope or reached by the chunk's code or tests
@answer n=7 entity=iconography#2.4 v=no
why: graphite sprite atlas positioning in an example fixture, not reached by the chunk's code or tests
@answer n=8 entity=iconography#2.5 v=no
why: inline SVG currentColor fills, the svg_native example and the Preact destroy button are example content the scope does not name and the chunk does not reach
@answer n=9 entity=iconography#3.1 v=no
why: an absence record of a search for icon handling in listed files, stating no behaviour the chunk's code or tests must honour
@answer n=10 entity=iconography#3.2 v=no
why: an absence record of a search for icon sets in listed files, stating no behaviour the chunk's code or tests must honour
