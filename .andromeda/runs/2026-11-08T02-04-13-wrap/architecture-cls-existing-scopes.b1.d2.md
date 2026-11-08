@answer n=1 entity=existing-scopes#1.1 v=no
why: the session reaches no accesskit_xplat file
@answer n=2 entity=existing-scopes#1.2 v=no
why: the session reaches no debug_timer file
@answer n=3 entity=existing-scopes#1.3 v=no
why: the session reaches no blitz-traits file and adds no trait
@answer n=4 entity=existing-scopes#1.4 v=no
why: no blitz-dom engine file moved and the blitz-dom diff gate is green, so the module list still holds
@answer n=5 entity=existing-scopes#1.5 v=no
why: no blitz-dom engine file moved, so the CSSOM and resolved-style claims hold
@answer n=6 entity=existing-scopes#1.6 v=no
why: no blitz-dom engine file moved, so the form, iframe and query claims hold
@answer n=7 entity=existing-scopes#1.7 v=no
why: no blitz-dom interaction file moved, so the hit testing, focus and select claims hold
@answer n=8 entity=existing-scopes#1.8 v=no
why: the accessibility tree builder and debug printing are untouched by the session
@answer n=9 entity=existing-scopes#1.9 v=no
why: the blitz-dom events module is untouched, the new dispatch sitting in dioxus-native-dom's bridge
@answer n=10 entity=existing-scopes#1.10 v=no
why: no blitz-dom scrolling or selection file moved
@answer n=11 entity=existing-scopes#1.11 v=no
why: no Stylo integration file moved and no dependency moved
@answer n=12 entity=existing-scopes#1.12 v=no
why: no blitz-dom tree or traversal file moved
@answer n=13 entity=existing-scopes#1.13 v=no
why: no blitz-dom layout file moved, so the layout and damage claims hold
@answer n=14 entity=existing-scopes#1.14 v=no
why: no blitz-dom node file moved and ElementData::is_disabled is only read by the new snapshot field
@answer n=15 entity=existing-scopes#1.15 v=no
why: the session reaches no blitz-html file
@answer n=16 entity=existing-scopes#1.16 v=no
why: the session reaches no blitz-net file and opens no outbound host
@answer n=17 entity=existing-scopes#1.17 v=no
why: the session reaches no blitz-paint file and changes no paint
@answer n=18 entity=existing-scopes#1.18 v=no
why: the JavaScript runtime is untouched by the chunk
@answer n=19 entity=existing-scopes#1.19 v=no
why: the session reaches no blitz-shell file
@answer n=20 entity=existing-scopes#1.20 v=no
why: the session reaches no blitz file and the crates it re-exports are unchanged
@answer n=21 entity=existing-scopes#1.21 v=no
why: the session reaches no stylo_taffy file and no dependency moved
@answer n=22 entity=existing-scopes#1.22 v=no
why: the dioxus-native crate is unedited and its module list and dependency on dioxus-native-dom hold, the new re-exports arriving through the existing pub use glob
@answer n=23 entity=existing-scopes#1.23 v=yes mode=add-to
why: the new graph and signal_registry modules, the graph and signal registry in the snapshot model's scope, the re-exported VIRTUAL_KEY_SEPARATOR and virtual_node_id beside element_id, and the bridge's registry dispatch join this crate's listing
@answer n=24 entity=existing-scopes#1.24 v=no
why: the session reaches no blitz-test-harness file and its six modules are unchanged
@answer n=25 entity=existing-scopes#1.25 v=no
why: the telemetry bootstrap, SAFE_FIELDS and CONTENT_FIELDS are unchanged and the graph adds no span field beyond verb
