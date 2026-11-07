@answer n=1 entity=screen-reader-support#8.2 v=yes mode=stale
why: lib.rs gains the mutation_buffer module and is reached whole, so the cited lines 7-10 may have moved, and the claim is not shown to hold on the changed file
@answer n=2 entity=screen-reader-support#9.1 v=no
why: lang_attribute.rs and the lang handling are not among the files the session reaches
@answer n=3 entity=screen-reader-support#9.2 v=no
why: blitz-dom default.css is not among the files the session reaches
@answer n=4 entity=screen-reader-support#9.3 v=no
why: the html lang fixtures listed are not among the files the session reaches
@answer n=5 entity=screen-reader-support#9.4 v=no
why: the pages declaring lang="en" are not among the files the session reaches
@answer n=6 entity=screen-reader-support#10.1 v=no
why: the s02 fixture set is not among the files the session reaches
@answer n=7 entity=screen-reader-support#11.1 v=no
why: the github fixture's sr-only tooltip is not among the files the session reaches
@answer n=8 entity=screen-reader-support#11.2 v=no
why: the google fixture's aria-labels are not among the files the session reaches
@answer n=9 entity=screen-reader-support#11.3 v=no
why: the google fixture's aria-hidden and aria-atomic uses are not among the files the session reaches
@answer n=10 entity=screen-reader-support#11.4 v=no
why: the servo-new fixture's aria-hidden spans are not among the files the session reaches
@answer n=11 entity=screen-reader-support#11.5 v=no
why: the servo.html alt texts are not among the files the session reaches
@answer n=12 entity=screen-reader-support#11.6 v=no
why: the servo_header_reduced.html image is not among the files the session reaches
@answer n=13 entity=screen-reader-support#12.1 v=no
why: blitz-dom node.rs IME and focus code is not reached, and the focus and key rules are unchanged
@answer n=14 entity=screen-reader-support#12.2 v=yes mode=stale
why: dioxus_document.rs is reached whole, so the cited test lines 755-789 may have moved under the buffered initial_build, while the IME behaviour holds as the replay order is unchanged
@answer n=15 entity=screen-reader-support#12.3 v=no
why: blitz-dom text.rs IME and text-editor event code is not reached by the session
@answer n=16 entity=screen-reader-support#13.1 v=yes mode=stale
why: the reached probe now asserts accessibility-tree focus, so the absence observed over the listed s06 files may no longer hold
