@answer n=1 entity=screen-reader-support#8.2 v=yes mode=stale
why: lib.rs is a reached file whose cited lines 7-12 may have moved, and the report does not show the accesskit-only claim holds on the changed file
@answer n=2 entity=screen-reader-support#9.1 v=no
why: lang_attribute.rs and its lang handling are not among the session's reached files
@answer n=3 entity=screen-reader-support#9.2 v=no
why: blitz-dom default.css is not among the session's reached files
@answer n=4 entity=screen-reader-support#9.3 v=no
why: the examples/assets fixtures carrying an html lang attribute are not reached by the session
@answer n=5 entity=screen-reader-support#9.4 v=no
why: the pages declaring lang="en" are fixtures and preact examples the session does not reach
@answer n=6 entity=screen-reader-support#10.1 v=no
why: the s02 fixtures searched for lang= are not reached by the session
@answer n=7 entity=screen-reader-support#11.1 v=no
why: the github fixture's sr-only tooltip is in a fixture the session does not reach
@answer n=8 entity=screen-reader-support#11.2 v=no
why: the google fixture's descriptive aria-labels are in a fixture the session does not reach
@answer n=9 entity=screen-reader-support#11.3 v=no
why: the google fixture's aria-hidden and aria-atomic uses are in a fixture the session does not reach
@answer n=10 entity=screen-reader-support#11.4 v=no
why: the servo-new fixture's aria-hidden bar spans are in a fixture the session does not reach
@answer n=11 entity=screen-reader-support#11.5 v=no
why: the servo.html fixture alt texts are in a fixture the session does not reach
@answer n=12 entity=screen-reader-support#11.6 v=no
why: the servo_header_reduced fixture's missing alt attribute is in a fixture the session does not reach
@answer n=13 entity=screen-reader-support#12.1 v=no
why: the blitz-dom node.rs IME enable and cursor-area code is not among the session's reached files
@answer n=14 entity=screen-reader-support#12.2 v=yes mode=stale
why: dioxus_document.rs is a reached file whose cited lines 775-809 may have moved with the bridge dispatch change, and the report does not show the claim holds on it
@answer n=15 entity=screen-reader-support#12.3 v=no
why: the blitz-dom text.rs IME editing code is not among the session's reached files
@answer n=16 entity=screen-reader-support#13.1 v=no
why: the session's reached files add no aria, role, accesskit or screen-reader term, and the accessibility tree lifecycle and platform adapter stay untouched
