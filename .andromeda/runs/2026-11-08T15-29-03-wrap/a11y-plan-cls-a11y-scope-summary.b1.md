@answer n=1 entity=a11y-scope-summary#1.1 v=no
why: the workspace accesskit dependency lines sit in Cargo.toml, which the session does not reach and whose dependencies it does not change
@answer n=2 entity=a11y-scope-summary#1.2 v=no
why: the accesskit_xplat package manifest is not reached and the session adds no platform adapter
@answer n=3 entity=a11y-scope-summary#1.3 v=no
why: the blitz-dom accessibility tree is not reached, since blitz-dom was not changed by the session
@answer n=4 entity=a11y-scope-summary#1.4 v=no
why: the select list lives in blitz-dom and its harness tests, none of which the session reaches
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: the custom widget hook lives in blitz-dom, which the session does not change
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: the blitz-html accessibility feature forward lives in its manifest, which the session does not reach
@answer n=7 entity=a11y-scope-summary#1.7 v=no
why: blitz-shell and its accessibility state are not reached, since the session does not change blitz-shell
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: the blitz accessibility feature lives in its manifest, which the session does not reach
@answer n=9 entity=a11y-scope-summary#1.9 v=no
why: NodeId::as_u64 in blitz-traits is not reached by the session
@answer n=10 entity=a11y-scope-summary#1.10 v=yes mode=stale
why: actionable_nodes and the shared Readings rule add a dioxus-native-dom actionable module that both binaries compile under the accessibility feature, a module this entity's list of compiled modules does not name
@answer n=11 entity=a11y-scope-summary#1.11 v=yes mode=stale
why: it cites tests/blitz-tests/tests/common/mod.rs lines 104-109, a file the session reaches whole, so those line numbers can have moved
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: the browser app manifest and its default feature set are not reached by the session
@answer n=13 entity=a11y-scope-summary#1.13 v=no
why: the blitz-tests manifest and its accessibility features are unchanged, since the session adds no dependency
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: the input fixture examples/assets/input.html is not reached by the session
@answer n=15 entity=a11y-scope-summary#1.15 v=yes mode=stale
why: docs/capabilities.md is reached whole by the session, so its Accessibility reach section cannot be shown unmoved by the actions verb and the actionable rule
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: the searched files are the top-level examples/*.rs files, which the session does not reach
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: the searched s06 files lie outside the session's reached set, so the change adds no accessibility-tree or ARIA code to them
@answer n=18 entity=a11y-scope-summary#2.3 v=no
why: the searched s07 files lie outside the session's reached set, so the change adds no accessibility word to them
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: the only hits are wpt runner paths and a quarantine reason, none of which the session reaches or extends with a keyboard or focus word
