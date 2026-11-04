@answer n=1 entity=a11y-scope-summary#1.1 v=no
why: the workspace manifest it cites (Cargo.toml) is not reached, and the Cargo.lock move names only pulldown-cmark, so accesskit "0.25" and the workspace members still hold
@answer n=2 entity=a11y-scope-summary#1.2 v=no
why: the accesskit_xplat manifest it cites (packages/accesskit_xplat/Cargo.toml) is not reached and no package changed
@answer n=3 entity=a11y-scope-summary#1.3 v=no
why: the blitz-dom manifest and accessibility module it cites are not reached, and the lock move touches only pulldown-cmark
@answer n=4 entity=a11y-scope-summary#1.4 v=no
why: the select list lives in blitz-dom source and the harness files it cites, none of which the session changed
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: the custom widget hook lives in blitz-dom source that the session did not change
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: the blitz-html manifest it cites is not reached and its feature forwarding is unchanged
@answer n=7 entity=a11y-scope-summary#1.7 v=no
why: the blitz-shell manifest it cites is not reached and its default features are unchanged
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: the blitz manifest it cites is not reached and its accessibility feature forwarding is unchanged
@answer n=9 entity=a11y-scope-summary#1.9 v=no
why: the blitz-traits node_id source it cites is not reached and the session changed no id code
@answer n=10 entity=a11y-scope-summary#1.10 v=yes mode=stale
why: Cargo.lock now resolves pulldown-cmark 0.13.6 where this row's re-read names the 0.13.5 move, and the lock is reached whole, so the stand-graph reading is re-read on the changed file
@answer n=11 entity=a11y-scope-summary#1.11 v=no
why: the served-page sources and stand tests it cites (examples/seven_guis, tests/blitz-tests/tests/stand_*.rs) are not changed by the session
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: apps/browser/Cargo.toml is not reached and its default feature set is unchanged
@answer n=13 entity=a11y-scope-summary#1.13 v=no
why: tests/blitz-tests/Cargo.toml is not reached, the new probe target adds no dependency, so the feature and accesskit dev-dependency it names still stand
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: examples/assets/input.html is not reached by the session
@answer n=15 entity=a11y-scope-summary#1.15 v=no
why: docs/capabilities.md is not reached, and the probe extends no criterion, leg or file list
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: the 18 examples/*.rs files searched are not reached, since the session changes nothing under examples/
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: the probe file is new and outside the 17 listed s06 files, and no listed file changed
@answer n=18 entity=a11y-scope-summary#2.3 v=no
why: the 8 s07 slice files searched are not reached by the session
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: the 12 listed s13 files searched (wpt/runner) are not reached by the session
