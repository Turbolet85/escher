@answer n=1 entity=a11y-scope-summary#1.1 v=no
why: the workspace accesskit dependency sits in Cargo.toml, which the session did not reach.
@answer n=2 entity=a11y-scope-summary#1.2 v=no
why: accesskit_xplat's manifest is outside the files the session reached.
@answer n=3 entity=a11y-scope-summary#1.3 v=no
why: the blitz-dom accessibility tree builder is outside the reached files and no builder read moved.
@answer n=4 entity=a11y-scope-summary#1.4 v=no
why: the select list lives in blitz-dom, which the session did not reach, and stand_keyboard's list walks did not move.
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: the custom widget TODO lives in blitz-dom, which the session did not reach.
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: blitz-html's feature forwarding sits in its manifest, which the session did not reach.
@answer n=7 entity=a11y-scope-summary#1.7 v=no
why: blitz-shell's manifest and window code are outside the reached files.
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: blitz's feature forwarding sits in its manifest, which the session did not reach.
@answer n=9 entity=a11y-scope-summary#1.9 v=no
why: the blitz-traits node id code is outside the reached files.
@answer n=10 entity=a11y-scope-summary#1.10 v=yes mode=add-to
why: F4 says the ungated reached record and element_listener_ids compile without the accessibility feature while element_facts stays gated, so this list of what the feature gates in dioxus-native-dom gains them.
@answer n=11 entity=a11y-scope-summary#1.11 v=yes mode=stale
why: the served-page claim cites tests/blitz-tests/tests/common/mod.rs lines 104-109, a reached shared-module file whose lines the session can move.
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: apps/browser's manifest is outside the reached files.
@answer n=13 entity=a11y-scope-summary#1.13 v=no
why: tests/blitz-tests/Cargo.toml is outside the reached files and the new stand tests add no dependency.
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: examples/assets/input.html is outside the reached files.
@answer n=15 entity=a11y-scope-summary#1.15 v=yes mode=stale
why: docs/capabilities.md is a reached file, and its accessibility reach section can move with the session's stand assertions and the new coverage reader.
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: the session's reached files are not among the 18 top-level examples/*.rs files searched.
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: the reached files add no role, ARIA, accesskit or screen-reader code, since no ARIA mapping or tree builder read moved.
@answer n=18 entity=a11y-scope-summary#2.3 v=no
why: the reached files add no accessibility handling, since the new reader is ungated and element_facts stays gated, so the absence holds.
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: the only keyboard and focus hits sit in wpt/runner files, which the session did not reach.
