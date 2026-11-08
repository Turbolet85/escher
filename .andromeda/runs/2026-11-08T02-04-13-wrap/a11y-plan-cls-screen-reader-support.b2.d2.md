@answer n=1 entity=screen-reader-support#8.2 v=yes mode=add-to
why: the session reaches dioxus-native-dom lib.rs and adds the accessibility-gated read DioxusDocument::graph (F4), which this exposure statement must carry beside the accesskit support
@answer n=2 entity=screen-reader-support#9.1 v=no
why: lang handling lives in blitz-dom and lang_attribute.rs, which the session does not reach
@answer n=3 entity=screen-reader-support#9.2 v=no
why: the dir attribute rule is in blitz-dom default.css, outside the files the session reaches
@answer n=4 entity=screen-reader-support#9.3 v=no
why: the 11 html lang fixtures under examples/assets are not touched by the session
@answer n=5 entity=screen-reader-support#9.4 v=no
why: the lang="en" pages (servo.html and the preact examples) are not touched by the session
@answer n=6 entity=screen-reader-support#10.1 v=no
why: the searched s02 fixtures are not touched by the session, so the absence of lang still holds
@answer n=7 entity=screen-reader-support#11.1 v=no
why: github_profile_reduced2.html is a fixture the session does not reach
@answer n=8 entity=screen-reader-support#11.2 v=no
why: the google.html aria-labels sit in a fixture the session does not reach
@answer n=9 entity=screen-reader-support#11.3 v=no
why: the google.html aria-hidden and aria-atomic lines sit in a fixture the session does not reach
@answer n=10 entity=screen-reader-support#11.4 v=no
why: the servo-new.html bar spans sit in a fixture the session does not reach
@answer n=11 entity=screen-reader-support#11.5 v=no
why: the servo.html image alt text sits in a fixture the session does not reach
@answer n=12 entity=screen-reader-support#11.6 v=no
why: servo_header_reduced.html is a fixture the session does not reach
@answer n=13 entity=screen-reader-support#12.1 v=no
why: IME enabling and the cursor area are handled in blitz-dom node.rs, which the session does not reach
@answer n=14 entity=screen-reader-support#12.2 v=yes mode=stale
why: dioxus_document.rs is a reached file, and the test pinned at its lines 775-809 can have moved with the new graph and event-dispatch code
@answer n=15 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and disable handling is in blitz-dom text.rs, which the session does not reach
@answer n=16 entity=screen-reader-support#13.1 v=yes mode=stale
why: dioxus_document.rs is a reached file now carrying the accessibility-gated graph, so the absence reading over the s06 files can have moved and no report shows it still holds
