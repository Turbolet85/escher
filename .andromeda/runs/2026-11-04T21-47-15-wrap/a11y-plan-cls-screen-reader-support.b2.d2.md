@answer n=1 entity=screen-reader-support#9.2 v=no
why: its claim lives in packages/blitz-dom/assets/default.css, which the session did not change (the session made no change under packages/)
@answer n=2 entity=screen-reader-support#9.3 v=no
why: its claim lists examples/assets fixtures' html lang attributes, which the session did not change (no change under examples/)
@answer n=3 entity=screen-reader-support#9.4 v=no
why: its claim lists the lang="en" pages under examples/assets and examples/preact, none of which the session changed
@answer n=4 entity=screen-reader-support#10.1 v=no
why: its absence claim is a search of the 21 s02 fixture files under examples/assets, none of which the session changed
@answer n=5 entity=screen-reader-support#11.1 v=no
why: its claim concerns the sr-only tooltip in examples/assets/github_profile_reduced2.html, which the session did not change
@answer n=6 entity=screen-reader-support#11.2 v=no
why: its claim concerns aria-labels in examples/assets/google.html, which the session did not change
@answer n=7 entity=screen-reader-support#11.3 v=no
why: its claim concerns aria-hidden and aria-atomic in examples/assets/google.html, which the session did not change
@answer n=8 entity=screen-reader-support#11.4 v=no
why: its claim concerns aria-hidden bar spans in examples/assets/servo-new.html, which the session did not change
@answer n=9 entity=screen-reader-support#11.5 v=no
why: its claim concerns alt text in examples/assets/servo.html, which the session did not change
@answer n=10 entity=screen-reader-support#11.6 v=no
why: its claim concerns the image without alt in examples/assets/servo_header_reduced.html, which the session did not change
@answer n=11 entity=screen-reader-support#12.1 v=no
why: its IME focus behaviour lives in packages/blitz-dom node.rs, which the session did not change, and the Cargo.lock move touches only pulldown-cmark, off this path
@answer n=12 entity=screen-reader-support#12.2 v=no
why: its claim lives in blitz-dom construct.rs and the dioxus-native-dom test at dioxus_document.rs, neither changed by the session, and the Cargo.lock move touches only pulldown-cmark, so the claim still holds
@answer n=13 entity=screen-reader-support#12.3 v=no
why: its IME event handling lives in packages/blitz-dom text.rs, which the session did not change
@answer n=14 entity=screen-reader-support#13.1 v=no
why: its absence claim is scoped to the 17 listed s06 files, none of which the session changed, and the new probe file is not among them
