@answer n=1 entity=screen-reader-support#6.1 v=no
why: the session does not reach blitz-dom's element tag-name mark at accessibility.rs:268, which it left unchanged
@answer n=2 entity=screen-reader-support#6.2 v=no
why: the session does not reach blitz-dom's tree root and focus rule in accessibility.rs, which it left unchanged
@answer n=3 entity=screen-reader-support#6.3 v=no
why: the session does not reach the role-mapping test accessibility_roles.rs, which it did not touch
@answer n=4 entity=screen-reader-support#7.1 v=no
why: the session does not reach accesskit_xplat's event handler, which it did not touch
@answer n=5 entity=screen-reader-support#8.1 v=no
why: the session does not reach blitz-shell's AccessKit adapter, which it did not touch
@answer n=6 entity=screen-reader-support#8.2 v=yes mode=stale
why: the change reaches dioxus-native-dom's crate root re-exports in lib.rs, so the cited lines 7-13 there may have moved
@answer n=7 entity=screen-reader-support#9.1 v=no
why: the session does not reach the lang attribute handling tested in lang_attribute.rs, which it did not touch
@answer n=8 entity=screen-reader-support#9.2 v=no
why: the session does not reach blitz-dom's default stylesheet dir rules in default.css, which it did not touch
@answer n=9 entity=screen-reader-support#9.3 v=no
why: the session does not reach the eleven fixtures that set an html lang attribute, none of which it touched
@answer n=10 entity=screen-reader-support#9.4 v=no
why: the session does not reach the three pages that declare lang="en", none of which it touched
@answer n=11 entity=screen-reader-support#10.1 v=no
why: the session does not reach the s02 fixtures searched for lang absence, and it adds no lang attribute to any of them
@answer n=12 entity=screen-reader-support#11.1 v=no
why: the session does not reach the GitHub fixture's sr-only tooltip element, which it did not touch
@answer n=13 entity=screen-reader-support#11.2 v=no
why: the session does not reach the google fixture's descriptive aria-labels, which it did not touch
@answer n=14 entity=screen-reader-support#11.3 v=no
why: the session does not reach the google fixture's aria-hidden and aria-atomic markup, which it did not touch
@answer n=15 entity=screen-reader-support#11.4 v=no
why: the session does not reach the servo-new fixture's aria-hidden burger spans, which it did not touch
@answer n=16 entity=screen-reader-support#11.5 v=no
why: the session does not reach the servo.html fixture's image alt text, which it did not touch
@answer n=17 entity=screen-reader-support#11.6 v=no
why: the session does not reach the servo_header_reduced.html fixture's image, which it did not touch
@answer n=18 entity=screen-reader-support#12.1 v=no
why: the session does not reach blitz-dom's text input focus and IME handling in node.rs, which it left unchanged
@answer n=19 entity=screen-reader-support#12.2 v=no
why: the session does not reach dioxus_document.rs or construct.rs, where the pre-focused input IME claim and its test live, and it did not change them
@answer n=20 entity=screen-reader-support#12.3 v=no
why: the session does not reach blitz-dom's IME commit and preedit handling in text.rs, which it left unchanged
@answer n=21 entity=screen-reader-support#13.1 v=no
why: the session adds no screen-reader or accessibility API integration, and the listed s06 files are not among the files it reached
