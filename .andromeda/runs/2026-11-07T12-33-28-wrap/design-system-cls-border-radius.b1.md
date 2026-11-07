@answer n=1 entity=border-radius#1.1 v=no
why: the session reaches only the dioxus-native-dom bridge files and the probe test, not the browser chrome CSS, and border radius values are untouched
@answer n=2 entity=border-radius#1.2 v=no
why: the cited about-history and about-newtab CSS lies outside the reached bridge files, and border radius values are untouched
@answer n=3 entity=border-radius#1.3 v=no
why: the stand's radii are held in layout-templates, which the session does not reach, and radius values are untouched
@answer n=4 entity=border-radius#1.4 v=no
why: the transparent example card at app.rs:121 lies outside the reached bridge files, and border radius values are untouched
@answer n=5 entity=border-radius#1.5 v=no
why: the blitz-dom default button radius at default.css:108 lies outside the reached bridge files, and border radius values are untouched
@answer n=6 entity=border-radius#1.6 v=no
why: the engine checkbox frame in blitz-paint form_controls.rs is not a reached file, and border radius values are untouched
@answer n=7 entity=border-radius#1.7 v=no
why: the engine scrollbar thumb radius in blitz-paint render.rs is not a reached file, and paint and hit-testing are unchanged
@answer n=8 entity=border-radius#2.1 v=no
why: the per-corner radius resolution at blitz-paint render.rs:1438-1455 is not a reached file, and paint is unchanged
@answer n=9 entity=border-radius#2.2 v=no
why: the inset clip-path TODO at blitz-paint clip_path.rs:167-168 is not a reached file, and paint is unchanged
@answer n=10 entity=border-radius#2.3 v=no
why: the img and object border attribute mapping at blitz-dom stylo.rs:1148-1169 is not a reached file, and border radius values are untouched
@answer n=11 entity=border-radius#3.1 v=no
why: the radius example fixtures under examples/assets are not reached files, and border radius values are untouched
@answer n=12 entity=border-radius#3.2 v=no
why: the google, graphite, hr and input fixture radii are not reached files, and border radius values are untouched
@answer n=13 entity=border-radius#3.3 v=no
why: the inline-flex-transform fixture and transforms example radii are not reached files, and border radius values are untouched
@answer n=14 entity=border-radius#3.4 v=no
why: the servo.css, shadow, and Preact example radii are not reached files, and border radius values are untouched
@answer n=15 entity=border-radius#4.1 v=no
why: the task marks border radius absence records untouched, so the border-radius token search still holds on the changed files
@answer n=16 entity=border-radius#4.2 v=no
why: the task marks border radius absence records untouched, so the border-radius and box-shadow search still holds on the changed files
@answer n=17 entity=border-radius#4.3 v=no
why: the task marks border radius absence records untouched, so the radius search over the s07 slice still holds on the changed files
@answer n=18 entity=border-radius#4.4 v=no
why: the task marks border radius absence records untouched, so the no-match radius search over the s08 files still holds on the changed files
@answer n=19 entity=border-radius#4.5 v=no
why: the task marks border radius absence records untouched, so the radius definitions search over the s10 files still holds on the changed files
@answer n=20 entity=border-radius#4.6 v=no
why: the task marks border radius absence records untouched, so the radius search over the s11 files still holds on the changed files
