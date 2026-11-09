@answer n=1 entity=screen-reader-support#4.6 v=yes mode=stale
why: common/mod.rs is reached whole by the shared-module change, so the cited lines 93-102 for ACTIONABLE_AT_BOOT may have moved
@answer n=2 entity=screen-reader-support#4.7 v=no
why: the probe's listener-mark reading and the clickable field are unchanged, and dioxus_runtime_probe.rs is not in the change
@answer n=3 entity=screen-reader-support#4.8 v=no
why: clickable's stand readings are unchanged, no snapshot field was added, and the session does not touch the probe
@answer n=4 entity=screen-reader-support#4.9 v=yes mode=add-to
why: F3, escher_driver::handler_coverage is a new reader of the UI graph's handlers and joins the list of takers beside NodeState.clickable and the actionable rule, with element_id.rs reached by the ungated element_listener_ids
@answer n=5 entity=screen-reader-support#4.10 v=yes mode=add-to
why: the coverage reader applies the same mounted exclusion over graph nodes, so the statement of who reads the actionable rule's listener leg needs it beside unkeyed_actionable and actionable_nodes
@answer n=6 entity=screen-reader-support#4.11 v=no
why: a removed listener is absent from the current listeners the coverage reader reads, so it stays read by neither, and actionable.rs is not reached
@answer n=7 entity=screen-reader-support#4.12 v=no
why: cells-grid's aria label, its residual res-wvatzhpe and the windowed reading are untouched by the session
@answer n=8 entity=screen-reader-support#4.13 v=yes mode=add-to
why: stand_handler_coverage adds a keyboard-activation assertion that a control's click handler counts as exercised, to sit beside the Enter and Space assertion this entity lists on load-fetch, which still holds
@answer n=9 entity=screen-reader-support#5.1 v=no
why: the actionable-key check, the focused list's keys and the CRUD and flight booker states are unchanged and no Enter or Space assertion is listed here
@answer n=10 entity=screen-reader-support#6.1 v=no
why: blitz-dom's tag-name write is not in the change
@answer n=11 entity=screen-reader-support#6.2 v=no
why: the tree root and focus reads of blitz-dom's accessibility builder are not in the change
@answer n=12 entity=screen-reader-support#6.3 v=no
why: the role-mapping test file is not in the change and its stated goal is unchanged
@answer n=13 entity=screen-reader-support#7.1 v=no
why: accesskit_xplat's event routing is not in the change
@answer n=14 entity=screen-reader-support#8.1 v=no
why: blitz-shell's accessibility adapter creation is not in the change
@answer n=15 entity=screen-reader-support#8.2 v=no
why: the ungated reached record and element_listener_ids are not screen-reader exposure and element_facts stays accessibility-gated, so the accesskit-behind-feature statement holds on the reached lib.rs
@answer n=16 entity=screen-reader-support#9.1 v=no
why: lang and xml:lang handling is not in the change
@answer n=17 entity=screen-reader-support#9.2 v=no
why: blitz-dom's default stylesheet is not in the change
@answer n=18 entity=screen-reader-support#9.3 v=no
why: the fixture html lang attributes are not in the change
@answer n=19 entity=screen-reader-support#9.4 v=no
why: the page lang declarations are not in the change
@answer n=20 entity=screen-reader-support#10.1 v=no
why: the lang absence search over the s02 fixtures is not in the change
@answer n=21 entity=screen-reader-support#11.1 v=no
why: the GitHub fixture's sr-only tooltip element is not in the change
@answer n=22 entity=screen-reader-support#11.2 v=no
why: the google fixture's descriptive aria-labels are not in the change
@answer n=23 entity=screen-reader-support#11.3 v=no
why: the google fixture's aria-hidden and aria-atomic markup is not in the change
@answer n=24 entity=screen-reader-support#11.4 v=no
why: the servo-new fixture's aria-hidden burger spans are not in the change
@answer n=25 entity=screen-reader-support#11.5 v=no
why: the servo fixture's image alt text is not in the change
