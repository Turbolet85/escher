@answer n=1 entity=existing-scopes#1.1 v=no
why: accesskit_xplat is outside the session's changed files and its adapter is unchanged
@answer n=2 entity=existing-scopes#1.2 v=no
why: debug_timer is outside the session's changed files and no timer code moved
@answer n=3 entity=existing-scopes#1.3 v=no
why: blitz-traits is outside the session's changed files and its modules stand
@answer n=4 entity=existing-scopes#1.4 v=no
why: the blitz-dom engine is unedited and the blitz-dom git diff gate is green
@answer n=5 entity=existing-scopes#1.5 v=no
why: blitz-dom CSSOM and styles are engine code the session did not move
@answer n=6 entity=existing-scopes#1.6 v=no
why: blitz-dom forms, iframes and queries are engine code the session did not move
@answer n=7 entity=existing-scopes#1.7 v=no
why: blitz-dom interaction, hit testing and select are engine code the session did not move
@answer n=8 entity=existing-scopes#1.8 v=no
why: the blitz-dom accessibility tree builder is engine code the session did not move
@answer n=9 entity=existing-scopes#1.9 v=no
why: the blitz-dom events submodules are engine code the session did not move
@answer n=10 entity=existing-scopes#1.10 v=no
why: blitz-dom scrolling and selection are engine code the session did not move
@answer n=11 entity=existing-scopes#1.11 v=no
why: the blitz-dom Stylo integration is engine code the session did not move
@answer n=12 entity=existing-scopes#1.12 v=no
why: blitz-dom tree, traversal and url are engine code the session did not move
@answer n=13 entity=existing-scopes#1.13 v=no
why: blitz-dom layout is engine code the session did not move
@answer n=14 entity=existing-scopes#1.14 v=no
why: the blitz-dom node module is engine code the session did not move
@answer n=15 entity=existing-scopes#1.15 v=no
why: blitz-html is outside the session's changed files and its TreeSink is unchanged
@answer n=16 entity=existing-scopes#1.16 v=no
why: blitz-net is outside the session's changed files
@answer n=17 entity=existing-scopes#1.17 v=no
why: blitz-paint is outside the session's changed files and the session adds no paint code
@answer n=18 entity=existing-scopes#1.18 v=no
why: blitz-vibey-script is the JavaScript runtime, which the session left untouched
@answer n=19 entity=existing-scopes#1.19 v=no
why: blitz-shell is outside the session's changed files
@answer n=20 entity=existing-scopes#1.20 v=no
why: the blitz crate's re-export list is unchanged and blitz is outside the session's changed files
@answer n=21 entity=existing-scopes#1.21 v=no
why: stylo_taffy is outside the session's changed files and no dependency moved
@answer n=22 entity=existing-scopes#1.22 v=no
why: dioxus-native is unedited, so its module list and its dependency on dioxus-native-dom still hold, and the new names arrive through dioxus-native-dom's glob re-export
@answer n=23 entity=existing-scopes#1.23 v=yes mode=add-to
why: dioxus-native-dom gains the accessibility-gated graph module and the ungated signal_registry module, and the snapshot model's scope gains the graph and the signal registry
@answer n=24 entity=existing-scopes#1.24 v=no
why: blitz-test-harness is outside the session's changed files and the stand tests live in tests/blitz-tests
@answer n=25 entity=existing-scopes#1.25 v=no
why: escher-telemetry is untouched and its bootstrap and sink rules are unchanged
