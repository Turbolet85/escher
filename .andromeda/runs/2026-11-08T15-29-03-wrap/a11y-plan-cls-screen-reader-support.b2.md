@answer n=1 entity=screen-reader-support#6.1 v=no
why: the cited blitz-dom tag-name line is in a file the session did not reach, and blitz-dom was not changed
@answer n=2 entity=screen-reader-support#6.2 v=no
why: the tree-root and focus claims cite only blitz-dom accessibility.rs, which the session did not reach
@answer n=3 entity=screen-reader-support#6.3 v=no
why: the role-mapping test file cited for the tree goal is not among the files the session reached
@answer n=4 entity=screen-reader-support#7.1 v=no
why: the accesskit_xplat event routing is untouched, since the session changed no platform adapter code
@answer n=5 entity=screen-reader-support#8.1 v=no
why: the blitz-shell AccessKit adapter construction is untouched by the session
@answer n=6 entity=screen-reader-support#8.2 v=yes mode=stale
why: the session reached dioxus-native-dom src/lib.rs and moved its crate root re-exports, so the cited lines 7-13 may have moved while the claim itself still holds
@answer n=7 entity=screen-reader-support#9.1 v=no
why: lang and xml:lang handling in blitz-dom and its lang test is untouched by the session
@answer n=8 entity=screen-reader-support#9.2 v=no
why: the default stylesheet dir mapping in blitz-dom assets is untouched by the session
@answer n=9 entity=screen-reader-support#9.3 v=no
why: the fixture html lang attributes under examples/assets are not among the files the session reached
@answer n=10 entity=screen-reader-support#9.4 v=no
why: the pages declaring lang="en" are not among the files the session reached
@answer n=11 entity=screen-reader-support#10.1 v=no
why: the absence of html lang in the other s02 fixtures is unaffected, since the session added no fixture and changed no fixture file
@answer n=12 entity=screen-reader-support#11.1 v=no
why: the GitHub fixture sr-only tooltip is an examples/assets file the session did not reach
@answer n=13 entity=screen-reader-support#11.2 v=no
why: the google fixture aria-labels are in examples/assets, which the session did not reach
@answer n=14 entity=screen-reader-support#11.3 v=no
why: the google fixture aria-hidden and aria-atomic uses are in examples/assets, which the session did not reach
@answer n=15 entity=screen-reader-support#11.4 v=no
why: the servo-new burger aria-hidden spans are in examples/assets, which the session did not reach
@answer n=16 entity=screen-reader-support#11.5 v=no
why: the servo.html image alt texts are in examples/assets, which the session did not reach
@answer n=17 entity=screen-reader-support#11.6 v=no
why: the servo_header_reduced.html image without alt is in examples/assets, which the session did not reach
@answer n=18 entity=screen-reader-support#12.1 v=no
why: IME enabling and blurring in blitz-dom node.rs is untouched, and blitz-dom was not changed
@answer n=19 entity=screen-reader-support#12.2 v=no
why: the IME-on-existing-focus behaviour in blitz-dom is unchanged, and dioxus_document.rs with its pinning test is not among the files the session reached
@answer n=20 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and disable handling in blitz-dom text.rs is untouched by the session
@answer n=21 entity=screen-reader-support#13.1 v=no
why: the absence search covers the 17 listed s06 files, none of which the session reached, and the new crate-private roles() read is not an AccessKit or screen-reader integration
