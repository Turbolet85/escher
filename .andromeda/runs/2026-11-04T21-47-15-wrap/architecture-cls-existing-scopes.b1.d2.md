@answer n=1 entity=existing-scopes#1.1 v=no
why: the session reaches only tests/blitz-tests, Cargo.lock and deny.toml, and accesskit_xplat's packages/ claim holds unmoved
@answer n=2 entity=existing-scopes#1.2 v=no
why: the session reaches no packages/ file, and debug_timer's feature-gated timer claim holds unmoved
@answer n=3 entity=existing-scopes#1.3 v=no
why: the session reaches no packages/ file, and blitz-traits' module list holds unmoved
@answer n=4 entity=existing-scopes#1.4 v=no
why: the new probe target adds no unit test to blitz-dom, whose cfg(test) module claim holds unmoved on the untouched packages/ files
@answer n=5 entity=existing-scopes#1.5 v=no
why: the session reaches no packages/ file, and blitz-dom's CSSOM and resolved-style claim holds unmoved
@answer n=6 entity=existing-scopes#1.6 v=no
why: the session reaches no packages/ file, and blitz-dom's forms, iframes and queries claim holds unmoved
@answer n=7 entity=existing-scopes#1.7 v=no
why: the session reaches no packages/ file, and blitz-dom's interaction line citations hold unmoved
@answer n=8 entity=existing-scopes#1.8 v=no
why: the session reaches no packages/ file, and blitz-dom's accessibility and debug claim holds unmoved
@answer n=9 entity=existing-scopes#1.9 v=no
why: the session reaches no packages/ file, and blitz-dom's events submodule list holds unmoved
@answer n=10 entity=existing-scopes#1.10 v=no
why: the session reaches no packages/ file, and blitz-dom's scrolling and selection claim holds unmoved
@answer n=11 entity=existing-scopes#1.11 v=no
why: the session reaches no packages/ file, and blitz-dom's Stylo integration claim holds unmoved
@answer n=12 entity=existing-scopes#1.12 v=no
why: the session reaches no packages/ file, and blitz-dom's tree and traversal claim holds unmoved
@answer n=13 entity=existing-scopes#1.13 v=no
why: the session reaches no packages/ file, and blitz-dom's layout module claim holds unmoved
@answer n=14 entity=existing-scopes#1.14 v=no
why: the session reaches no packages/ file, and blitz-dom's node module claim holds unmoved
@answer n=15 entity=existing-scopes#1.15 v=no
why: the session reaches no packages/ file, and blitz-html's parsing claim holds unmoved
@answer n=16 entity=existing-scopes#1.16 v=no
why: the session reaches no packages/ file, and blitz-net's fetching claim holds unmoved
@answer n=17 entity=existing-scopes#1.17 v=no
why: the session reaches no packages/ file, and blitz-paint's module list holds unmoved
@answer n=18 entity=existing-scopes#1.18 v=no
why: the session reaches no packages/ file, and blitz-vibey-script's test-file claim holds unmoved since the probe is a blitz-tests target
@answer n=19 entity=existing-scopes#1.19 v=no
why: the session reaches no packages/ file, and blitz-shell's single unit test claim holds unmoved
@answer n=20 entity=existing-scopes#1.20 v=no
why: the session reaches no packages/ file, and the blitz re-export claim holds unmoved
@answer n=21 entity=existing-scopes#1.21 v=no
why: the session reaches no packages/ file, and stylo_taffy's module list holds unmoved
@answer n=22 entity=existing-scopes#1.22 v=no
why: the session reaches no packages/ file, and dioxus-native's dependency on dioxus-native-dom comes from an unmoved Cargo.toml, the Cargo.lock change being pulldown-cmark only
@answer n=23 entity=existing-scopes#1.23 v=no
why: the session reaches no packages/ file, and dioxus-native-dom's module and unit-test claims hold unmoved since the probe lives in blitz-tests
@answer n=24 entity=existing-scopes#1.24 v=no
why: the session reaches no packages/ file, and blitz-test-harness's module claim holds unmoved since the probe reads public API only
@answer n=25 entity=existing-scopes#1.25 v=no
why: the session reaches no packages/ file, and escher-telemetry's bootstrap claim holds unmoved since the probe installs no subscriber
