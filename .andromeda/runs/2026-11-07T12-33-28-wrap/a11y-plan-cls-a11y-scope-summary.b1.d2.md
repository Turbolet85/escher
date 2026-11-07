@answer n=1 entity=a11y-scope-summary#1.1 v=no
why: the workspace AccessKit dependency lines are in the root Cargo.toml, which the session does not reach and no dependency changed
@answer n=2 entity=a11y-scope-summary#1.2 v=no
why: the accesskit_xplat package manifest is not a file the session reaches
@answer n=3 entity=a11y-scope-summary#1.3 v=no
why: the blitz-dom accessibility tree builder is not in any file the session reaches
@answer n=4 entity=a11y-scope-summary#1.4 v=no
why: the select list lives in blitz-dom, which the session does not reach
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: the custom widget hook lives in blitz-dom node/custom_widget.rs, which the session does not reach
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: blitz-html's accessibility feature forwarding is in its manifest, which the session does not reach
@answer n=7 entity=a11y-scope-summary#1.7 v=no
why: blitz-shell's manifest and AccessibilityState modules are not reached, and the windowed launch's MountedData change does not alter what this entity says of them
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: blitz's accessibility feature forwarding is in its manifest, which the session does not reach
@answer n=9 entity=a11y-scope-summary#1.9 v=no
why: blitz-traits node_id.rs is not a file the session reaches
@answer n=10 entity=a11y-scope-summary#1.10 v=no
why: the claim rests on Dioxus crate feature lines and the dependency graph, which the session leaves unchanged since it edits source only in dioxus-native-dom with no feature or dependency change
@answer n=11 entity=a11y-scope-summary#1.11 v=no
why: the served pages' Tab sequences and keyboard rows sit in examples/seven_guis and stand tests the session does not reach, and the entity says nothing of a synchronous scroll_to or the borrow hazard
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: the browser app's accessibility feature default is in its manifest, which the session does not reach
@answer n=13 entity=a11y-scope-summary#1.13 v=no
why: the claim is about blitz-tests manifest feature lines and its accesskit dev-dependency, which the session leaves unchanged even though the probe source changes
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: examples/assets/input.html is not a file the session reaches
@answer n=15 entity=a11y-scope-summary#1.15 v=no
why: the capability statement's tier, criteria and named limits are unchanged, and the new render-time limit carries the keyboard-navigation slug that no entity of this batch holds
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: the absence is a search over examples/*.rs files, which the session does not reach
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: the session's reached files are dioxus-native-dom source and the blitz-tests probe, and add no aria, accesskit, role or screen-reader text to the s06 search slice
@answer n=18 entity=a11y-scope-summary#2.3 v=no
why: the session adds no accessibility handling text to the s07 search slice's files
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: the s13 search and its hits are wpt/runner files, which the session does not reach
