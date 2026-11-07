@answer n=1 entity=test-data-fixtures#3.24 v=no
why: browser entry helpers in apps/browser; the chunk's Dioxus render change does not reach that crate
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: the .test TLD for browser test URLs in apps/browser; not reached by the chunk
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the in-test favicon PNG in apps/browser; not reached by the chunk
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: fetch body of blitz-vibey-script tests; ScriptDocument is not a Dioxus document the chunk renders
@answer n=5 entity=test-data-fixtures#3.28 v=yes
why: the stand LoadNet answer (fixed URL, LOADED_CSS, 200 ms delay) is the value stand_load_page and the other stand_* checks re-assert through the buffered poll
@answer n=6 entity=test-data-fixtures#3.29 v=no
why: render_blocking_stylesheet answers; that check is not named in the scope as re-asserted and the chunk's edits do not reach its stylesheet answers
@answer n=7 entity=test-data-fixtures#3.30 v=no standing=S1
why: S1 convention, a unit test in a #[cfg(test)] mod tests block of the file under test; these blitz-net loopback listeners are blitz-net's own and the chunk does not reach blitz-net
@answer n=8 entity=test-data-fixtures#4.1 v=no
why: the absence of fixture files or loaders in slice s08; the chunk adds no fixture or loader and the absence sets no rule the chunk's code or tests must meet
