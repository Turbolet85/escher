@answer n=1 entity=screen-reader-support#8.2 v=yes mode=stale
why: the session adds the crate-private mutation_buffer module to packages/dioxus-native-dom/src/lib.rs, a reached file, so the cited lines 7-10 may have moved
@answer n=2 entity=screen-reader-support#9.1 v=no
why: the session reaches no lang handling, lang_attribute test or blitz-dom file
@answer n=3 entity=screen-reader-support#9.2 v=no
why: the session does not touch blitz-dom default.css or its dir attribute selectors
@answer n=4 entity=screen-reader-support#9.3 v=no
why: the session changes no html fixture under examples/assets
@answer n=5 entity=screen-reader-support#9.4 v=no
why: the session does not touch the pages that declare lang="en"
@answer n=6 entity=screen-reader-support#10.1 v=no
why: the session adds no html lang attribute and changes no s02 fixture
@answer n=7 entity=screen-reader-support#11.1 v=no
why: the session does not touch the github fixture or its sr-only tooltip element
@answer n=8 entity=screen-reader-support#11.2 v=no
why: the session does not touch the google fixture's aria-labels
@answer n=9 entity=screen-reader-support#11.3 v=no
why: the session does not touch the google fixture's aria-hidden or aria-atomic markup
@answer n=10 entity=screen-reader-support#11.4 v=no
why: the session does not touch the servo-new fixture's aria-hidden spans
@answer n=11 entity=screen-reader-support#11.5 v=no
why: the session does not touch the servo.html alt text or its fixture images
@answer n=12 entity=screen-reader-support#11.6 v=no
why: the session does not touch the servo_header_reduced.html fixture image
@answer n=13 entity=screen-reader-support#12.1 v=no
why: the session leaves the focus and IME rules of blitz-dom node.rs unchanged and does not reach that file
@answer n=14 entity=screen-reader-support#12.2 v=yes mode=stale
why: the session rewrites DioxusDocument initial_build onto the buffered render in dioxus_document.rs, a reached file, so the cited pin lines 755-789 may have moved and the on-mount IME pin needs re-reading
@answer n=15 entity=screen-reader-support#12.3 v=no
why: the session does not touch the IME commit, preedit or DeleteSurrounding handling in blitz-dom text.rs
@answer n=16 entity=screen-reader-support#13.1 v=no
why: the session adds no accessibility, aria or screen-reader integration to the searched s06 files
