@answer n=1 entity=a11y-scope-summary#1.1 v=no
why: the session touches no workspace manifest, so the accesskit workspace dependency and its member lines stand
@answer n=2 entity=a11y-scope-summary#1.2 v=no
why: the accesskit_xplat package manifest is not among the session's reached files and the adapter is unchanged
@answer n=3 entity=a11y-scope-summary#1.3 v=no
why: the blitz-dom accessibility tree builder is untouched per the task and no blitz-dom file is reached
@answer n=4 entity=a11y-scope-summary#1.4 v=no
why: the select list and its harness checks lie in files the session does not reach and the popup did not move
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: the custom-widget TODO lives in blitz-dom, which the session does not reach
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: blitz-html's feature forwarding sits in its manifest, which the session does not touch
@answer n=7 entity=a11y-scope-summary#1.7 v=no
why: blitz-shell's default feature and adapter wrapper sit in files the session does not reach
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: blitz's accessibility feature forwarding sits in its manifest, which the session does not touch
@answer n=9 entity=a11y-scope-summary#1.9 v=no
why: NodeId::as_u64 in blitz-traits is unchanged and no blitz-traits file is reached
@answer n=10 entity=a11y-scope-summary#1.10 v=yes mode=add-to
why: F4 the ungated reached record and element_listener_ids compile without the accessibility feature while element_facts stays gated, which extends this feature-reach account
@answer n=11 entity=a11y-scope-summary#1.11 v=yes mode=stale
why: the cited Tab-sequence constants at tests/blitz-tests/tests/common/mod.rs lines 104-109 sit in a reached shared module whose tables moved, so the line citation may have shifted
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: the browser app manifest is not reached and its default feature set is unchanged
@answer n=13 entity=a11y-scope-summary#1.13 v=no
why: the blitz-tests manifest lines are not reached and the new target builds with the same features
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: examples/assets/input.html is not reached and the session adds no fixture change
@answer n=15 entity=a11y-scope-summary#1.15 v=no
why: docs/capabilities.md is reached, but its accessibility-reach claims (tier 0, SC 2.1.1 1.4.3 2.4.3, the fourteen a11y-leg files, the named limits) still hold per the untouched items and the session adds no criterion
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: the searched examples/*.rs top-level files are not among the reached files, which sit under examples/seven_guis
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: the session adds no accesskit, ARIA or screen-reader code and no ARIA mapping moved, its new role wording is a command-line role
@answer n=18 entity=a11y-scope-summary#2.3 v=no
why: the session adds no accessibility handling, its new reader and record are ungated and element_facts stays gated
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: the only hits lie in wpt/runner files the session does not reach and the session adds no keyboard handling per untouched item 4
