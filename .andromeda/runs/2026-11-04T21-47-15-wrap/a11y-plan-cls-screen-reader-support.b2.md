@answer n=1 entity=screen-reader-support#9.2 v=no
why: the session changed only the probe test, Cargo.lock and deny.toml, and the blitz-dom default.css rule cited here sits under packages/, which the session did not change.
@answer n=2 entity=screen-reader-support#9.3 v=no
why: the session changed no examples/assets fixture, so the eleven html lang lines stand.
@answer n=3 entity=screen-reader-support#9.4 v=no
why: the three pages declaring lang="en" are under examples/, which the session did not change.
@answer n=4 entity=screen-reader-support#10.1 v=no
why: the absence search covers the 21 s02 fixtures under examples/, none of which the session changed.
@answer n=5 entity=screen-reader-support#11.1 v=no
why: the github fixture under examples/assets is unchanged, and the session reaches no examples/ file.
@answer n=6 entity=screen-reader-support#11.2 v=no
why: the google fixture aria-label lines are under examples/, which the session did not change.
@answer n=7 entity=screen-reader-support#11.3 v=no
why: the google fixture aria-hidden and aria-atomic lines are under examples/, which the session did not change.
@answer n=8 entity=screen-reader-support#11.4 v=no
why: the servo-new fixture hiding burger bars is under examples/, which the session did not change.
@answer n=9 entity=screen-reader-support#11.5 v=no
why: the servo.html fixture images are under examples/, which the session did not change.
@answer n=10 entity=screen-reader-support#11.6 v=no
why: the servo_header_reduced fixture is under examples/, which the session did not change, so its missing alt stands.
@answer n=11 entity=screen-reader-support#12.1 v=no
why: node.rs under packages/blitz-dom is unchanged, and the session touched no packages/ file.
@answer n=12 entity=screen-reader-support#12.2 v=no
why: construct.rs and dioxus_document.rs at the cited lines are unchanged, and the probe leaves the input_focused_on_mount_enables_ime pin standing.
@answer n=13 entity=screen-reader-support#12.3 v=no
why: text.rs IME event handling under packages/blitz-dom is unchanged, and the session touched no packages/ file.
@answer n=14 entity=screen-reader-support#13.1 v=no
why: no listed s06 file changed, the Cargo.lock move is pulldown-cmark alone and adds no accessibility crate, and the probe is a test reading public API rather than AT integration.
