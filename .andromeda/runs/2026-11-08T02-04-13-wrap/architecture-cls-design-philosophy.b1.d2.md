@answer n=1 entity=design-philosophy#1.1 v=yes mode=stale
why: packages/dioxus-native-dom/src/lib.rs is reached by the new graph and signal_registry modules, so the cited line lib.rs:3 can have moved
@answer n=2 entity=design-philosophy#2.1 v=yes mode=stale
why: packages/dioxus-native-dom/src/dioxus_document.rs is reached by DioxusDocument::graph and the registry dispatch on the event path, so the cited lines :59 and :85-86 can have moved
@answer n=3 entity=design-philosophy#3.1 v=no
why: its cited blitz-dom, blitz-paint and blitz-net files are unreached and blitz-dom is untouched, so the browser-spec claims still hold
@answer n=4 entity=design-philosophy#4.1 v=no
why: its cited blitz-dom files are unreached and the new graph reader holds no NodeId, so the versioned-identity claim still holds
@answer n=5 entity=design-philosophy#5.1 v=no
why: its cited harness and benchmark files are unreached and stand_graph is one more headless test in the existing tests directory, so the headless claim still holds
