@answer n=1 entity=keyboard-navigation#3.22 v=no
why: the change reaches no blitz-dom node file, so the synthetic mouse click builder is unchanged
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: the change reaches no blitz-shell convert_events file, so the winit key conversion holds as written
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: the change reaches no blitz-shell window file, so key dispatch after shell shortcuts is unchanged
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the change reaches neither the blitz-dom document file nor the cited key-press test file, so the anonymous block behaviour is unchanged
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: the change reaches no blitz-shell application or window file, so the macOS binding forwarding is unchanged
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: the change reaches no blitz-shell or blitz-traits IME file, so the IME forwarding is unchanged
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: the change reaches no blitz-vibey-script file, so the element focus and blur surface is unchanged
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the change reaches no blitz-vibey-script form control file or Cargo feature, so autofocus reflection is unchanged
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: the change reaches no blitz-vibey-script css runtime file, so the style property accessors are unchanged
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: the change reaches no blitz-vibey-script event file, so the JS keyboard event fields are unchanged
@answer n=11 entity=keyboard-navigation#4.5 v=yes mode=stale
why: dioxus_document.rs is reached whole by the new ungated reached record and its handle_event write, so the cited lines 415-419 may have moved
@answer n=12 entity=keyboard-navigation#4.6 v=yes mode=add-to
why: the bridge's handle_event gains the reached record F1, written before runtime.handle_event beside the registry dispatch it describes
@answer n=13 entity=keyboard-navigation#4.7 v=yes mode=stale
why: dioxus_document.rs is reached whole by the session's new DioxusDocument methods, so the cited focus-data lines 410-413 may have moved
@answer n=14 entity=keyboard-navigation#4.8 v=no
why: the change reaches no dioxus-native-dom events file, so set_focus and the unqueued focus events are unchanged
@answer n=15 entity=keyboard-navigation#4.9 v=yes mode=stale
why: dioxus_document.rs is reached whole, so the cited initial_build and poll buffering lines 309-311 may have moved
@answer n=16 entity=keyboard-navigation#4.10 v=yes mode=stale
why: the render-time buffering limit lives in dioxus_document.rs, which the change reaches whole, and no report shows the limit still holds on the changed file
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: the change adds no Cargo feature or dependency, so the autofocus forwarding manifests are not reached
@answer n=18 entity=keyboard-navigation#4.12 v=yes mode=stale
why: dioxus_document.rs is reached whole, so the cited IME TODO lines 428-429 may have moved
@answer n=19 entity=keyboard-navigation#4.13 v=no
why: the change reaches no mutation_writer or falsy-attribute test file, so the 28-name boolean attribute predicate is unchanged
@answer n=20 entity=keyboard-navigation#5.1 v=no
why: the change reaches no browser toolbar file, so the urlbar keys are unchanged
@answer n=21 entity=keyboard-navigation#5.2 v=no
why: the change reaches no browser tab strip file, so the tab shortcuts are unchanged
@answer n=22 entity=keyboard-navigation#5.3 v=no
why: the change reaches no browser about-pages file, so the new-tab search autofocus is unchanged
@answer n=23 entity=keyboard-navigation#5.4 v=no
why: the change reaches no rdme readme application file, so its key bindings are unchanged
@answer n=24 entity=keyboard-navigation#5.5 v=no
why: the change reaches no dioxus-native hooks file, so use_back_button is unchanged
@answer n=25 entity=keyboard-navigation#5.6 v=no
why: the change reaches no todomvc app file, so the new-todo focus and Enter and Escape handling are unchanged
