@answer n=1 entity=screen-reader-support#4.6 v=no
why: ACTIONABLE_AT_BOOT's boot pins still hold on the changed common/mod.rs, since the actionable-key rule and unkeyed_actionable's result are unchanged
@answer n=2 entity=screen-reader-support#4.7 v=no
why: the probe's listener-mark reading and NodeState.clickable are not moved, and no snapshot field was added
@answer n=3 entity=screen-reader-support#4.8 v=no
why: clickable's stand values come from the snapshot model, which the session does not change
@answer n=4 entity=screen-reader-support#4.9 v=yes mode=stale
why: the vdom walk's readers and the element_id.rs:321-328 citation move with the new ungated element_listener_ids beside element_facts in the reached element_id.rs
@answer n=5 entity=screen-reader-support#4.10 v=yes mode=add-to
why: F3, the new handler_coverage reader applies the same mounted exclusion over graph nodes and joins the list of who reads the actionable rule
@answer n=6 entity=screen-reader-support#4.11 v=no
why: the two named readers still read neither removed listener, and the coverage reader is not one of them
@answer n=7 entity=screen-reader-support#4.12 v=no
why: cells-grid's aria_label, its missing role and residual res-wvatzhpe are untouched by the session
@answer n=8 entity=screen-reader-support#4.13 v=no
why: the stand_keyboard Enter and Space walk on the load page is unchanged, and the new keyboard-activation assertion is a separate handler-coverage check
@answer n=9 entity=screen-reader-support#5.1 v=no
why: the stand's control states (listbox, option, aria-invalid, selected) are not moved and the snapshot model is unchanged
@answer n=10 entity=screen-reader-support#6.1 v=no
why: the tag-name write at blitz-dom accessibility.rs:268 sits in no changed file
@answer n=11 entity=screen-reader-support#6.2 v=no
why: the tree root and focus rule are blitz-dom code the session does not change
@answer n=12 entity=screen-reader-support#6.3 v=no
why: accessibility_roles.rs is not reached and its goal statement is unaffected
@answer n=13 entity=screen-reader-support#7.1 v=no
why: accesskit_xplat's event routing is in no changed file
@answer n=14 entity=screen-reader-support#8.1 v=no
why: blitz-shell's adapter wiring is in no changed file
@answer n=15 entity=screen-reader-support#8.2 v=no
why: reached lib.rs gains only the ungated record and reader, which are not screen-reader exposure, and element_facts stays accessibility-gated, so the exposure-behind-feature statement still holds
@answer n=16 entity=screen-reader-support#9.1 v=no
why: lang inheritance in blitz-dom is not touched by the session
@answer n=17 entity=screen-reader-support#9.2 v=no
why: default.css's dir attribute selectors are in no changed file
@answer n=18 entity=screen-reader-support#9.3 v=no
why: the fixture lang attributes are under examples/assets, which the session does not change
@answer n=19 entity=screen-reader-support#9.4 v=no
why: the page lang declarations are in example pages the session does not change
@answer n=20 entity=screen-reader-support#10.1 v=no
why: the absence of html lang in the s02 fixtures is under examples/assets, which the session does not change
@answer n=21 entity=screen-reader-support#11.1 v=no
why: the github fixture's sr-only element is under examples/assets, which the session does not change
@answer n=22 entity=screen-reader-support#11.2 v=no
why: the google fixture's aria-labels are under examples/assets, which the session does not change
@answer n=23 entity=screen-reader-support#11.3 v=no
why: the google fixture's aria-hidden and aria-atomic are under examples/assets, which the session does not change
@answer n=24 entity=screen-reader-support#11.4 v=no
why: the servo-new burger markup is under examples/assets, which the session does not change
@answer n=25 entity=screen-reader-support#11.5 v=no
why: the servo.html alt texts are under examples/assets, which the session does not change
