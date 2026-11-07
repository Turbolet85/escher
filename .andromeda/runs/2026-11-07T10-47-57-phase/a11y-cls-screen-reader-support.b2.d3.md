@answer n=1 entity=screen-reader-support#8.2 v=yes
why: the lib.rs:7-10 accesskit gate sits on the module list the chunk edits, and the windowed launch's tree refresh the scope names runs behind it
@answer n=2 entity=screen-reader-support#9.1 v=no
why: lang and xml:lang element-language inheritance is blitz-dom's own behaviour, which the chunk neither names nor changes
@answer n=3 entity=screen-reader-support#9.2 v=no
why: the dir attribute's default-stylesheet bidi rule is style, which the chunk does not change
@answer n=4 entity=screen-reader-support#9.3 v=no
why: fixture html lang attributes are fixture content the chunk does not read or edit
@answer n=5 entity=screen-reader-support#9.4 v=no
why: the pages' lang="en" declarations are fixture content the chunk does not read or edit
@answer n=6 entity=screen-reader-support#10.1 v=no
why: the absence of html lang in the s02 fixtures is fixture content the chunk does not read or edit
@answer n=7 entity=screen-reader-support#11.1 v=no
why: the sr-only tooltip in the GitHub fixture is fixture markup the chunk does not touch
@answer n=8 entity=screen-reader-support#11.2 v=no
why: the google fixture's aria-label names are name reads the chunk leaves untouched
@answer n=9 entity=screen-reader-support#11.3 v=no
why: aria-hidden decoration and aria-atomic in the google fixture are visibility and live-region markup the chunk does not change
@answer n=10 entity=screen-reader-support#11.4 v=no
why: servo-new's aria-hidden burger spans are fixture markup, and hidden-content rules are untouched
@answer n=11 entity=screen-reader-support#11.5 v=no
why: servo.html image alt text is fixture content the chunk does not read or edit
@answer n=12 entity=screen-reader-support#11.6 v=no
why: the missing alt on servo_header_reduced's image is fixture content the chunk does not read or edit
@answer n=13 entity=screen-reader-support#12.1 v=yes
why: focusing a text input through the set_focus call the scope names enables IME and sets the cursor area, a side effect the render-time focus carries
@answer n=14 entity=screen-reader-support#12.2 v=yes
why: the render sites' buffer replay and render-time set_focus change the order of focus against text-editor creation, which decides whether IME is on for an input focused before its editor exists
@answer n=15 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and disable events are text-editor input paths the chunk does not touch
@answer n=16 entity=screen-reader-support#13.1 v=no
why: the absence of accessibility API in the 17 s06 files says nothing about the dioxus-native-dom files the chunk edits
