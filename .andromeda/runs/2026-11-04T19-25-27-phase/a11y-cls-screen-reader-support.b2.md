@answer n=1 entity=screen-reader-support#9.2 v=no
why: the blitz-dom default stylesheet's dir-attribute bidi selectors; the probe reads no dir attribute and changes no engine path
@answer n=2 entity=screen-reader-support#9.3 v=no
why: html lang on eleven fixtures; the probe reads no lang and runs only on the seven_guis stand
@answer n=3 entity=screen-reader-support#9.4 v=no
why: pages declaring lang="en"; no page or example on the probe's path
@answer n=4 entity=screen-reader-support#10.1 v=no
why: absent lang in the other s02 fixtures; a fixture observation outside the seven_guis stand
@answer n=5 entity=screen-reader-support#11.1 v=no
why: sr-only tooltip in the GitHub fixture; the probe runs on seven_guis, not a fixture
@answer n=6 entity=screen-reader-support#11.2 v=no
why: google fixture's descriptive aria-labels; a fixture observation, not a rule over the probe's name reads
@answer n=7 entity=screen-reader-support#11.3 v=no
why: google fixture's aria-hidden and aria-atomic; hidden content is not_needed and aria-atomic is live-region scope the plan defers
@answer n=8 entity=screen-reader-support#11.4 v=no
why: servo-new burger bars hidden with aria-hidden; hidden content is not_needed and no such element is on the stand
@answer n=9 entity=screen-reader-support#11.5 v=no
why: servo.html image alt text; a fixture observation with no image on the probe's path
@answer n=10 entity=screen-reader-support#11.6 v=no
why: servo_header_reduced image without alt; a fixture observation outside the seven_guis stand
@answer n=11 entity=screen-reader-support#12.1 v=yes
why: focusing a text input enables IME in blitz-dom node.rs; the probe reads focus from the tree and whether its CRUD walk focuses a text field is open, a fork the probe must account for
@answer n=12 entity=screen-reader-support#12.2 v=yes
why: an input already focused when its editor is built enables IME; whether a CRUD text input is focused at mount is open, a fork over the probe's focus reads; the pinning test sits in re-read dioxus_document.rs, but the entity binds through that fork, not the poll check
@answer n=13 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and DeleteSurrounding act on the text editor; the probe sends no IME events and names no text entry, only press and press_with keys with arrow, Enter and Space
@answer n=14 entity=screen-reader-support#13.1 v=no
why: absence of screen-reader or AT integration in the s06 files; the probe builds no platform adapter and reads AT only as the read-not-run residual, so the absence asks nothing of the chunk
