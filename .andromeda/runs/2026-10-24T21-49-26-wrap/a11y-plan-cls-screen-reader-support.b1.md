# Classified — a11y-plan · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · no · its TextRun rule sits in blitz-dom's accessibility.rs, which the session did not change
screen-reader-support#1.2 · no · the aria-label and aria-labelledby rule holds, unchanged in accessibility.rs; accessibility_label's leg join is a §3 fact, not this rule's
screen-reader-support#1.3 · no · the label-binding rule holds, unchanged in accessibility.rs and document.rs; accessibility_label's leg join is a §3 fact, not this rule's
screen-reader-support#1.4 · no · the select node rule holds, unchanged in accessibility.rs and snapshot.rs; stand_act_select's leg join is a keyboard-harness fact, not this rule's
screen-reader-support#2.1 · no · the shell-tree equality holds and stand_accessibility_ids, already in the a11y leg before this chunk, re-asserts it green with no engine or stand change
screen-reader-support#3.1 · no · the snapshot readings hold, no snapshot source moved, and none of the checks it cites joined the a11y leg
screen-reader-support#4.1 · no · the actionable-key rule and its checks hold, the re-read after 0.2.0 moves no reading, and none of the cited checks joined the a11y leg
screen-reader-support#5.1 · yes · add-to · the pin accessibility_active_descendant_unread joined the a11y leg this chunk, so the pinned gap is now witnessed in CI
screen-reader-support#6.1 · no · element tag names are read at accessibility.rs:241, which the session did not change
screen-reader-support#6.2 · no · the Window root and the tree focus are read in accessibility.rs, which the session did not change
screen-reader-support#6.3 · no · the role-mapping test it cites, accessibility_roles, sat in the a11y leg before this chunk and is unedited
screen-reader-support#7.1 · no · accesskit_xplat's routing is unchanged and the session touched no platform-adapter code
screen-reader-support#8.1 · no · the adapter's creation in blitz-shell is unchanged and this entity states no stand-graph reading
screen-reader-support#8.2 · no · the accessibility feature's gating in blitz and the Dioxus crates is unchanged
screen-reader-support#9.1 · no · the lang and xml:lang handling is unchanged, and lang_attribute.rs joined no leg
screen-reader-support#9.2 · no · the dir selectors in blitz-dom's default.css are unchanged
screen-reader-support#9.3 · no · the fixture lang attributes sit in examples/assets, which the session did not change
screen-reader-support#9.4 · no · the lang="en" pages sit in examples/, which the session did not change
screen-reader-support#10.1 · no · the absence is a search over the s02 fixtures, which the session did not change
screen-reader-support#11.1 · no · the sr-only tooltip sits in examples/assets/github_profile_reduced2.html, which the session did not change
screen-reader-support#11.2 · no · the descriptive aria-labels sit in examples/assets/google.html, which the session did not change
screen-reader-support#11.3 · no · the aria-hidden and aria-atomic markup sits in examples/assets/google.html, which the session did not change
screen-reader-support#11.4 · no · the burger's aria-hidden spans sit in examples/assets/servo-new.html, which the session did not change
screen-reader-support#11.5 · no · the alt texts sit in examples/assets/servo.html, which the session did not change
screen-reader-support#11.6 · no · the missing alt sits in examples/assets/servo_header_reduced.html, which the session did not change
