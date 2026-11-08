@answer n=1 entity=keyboard-navigation#3.22 v=yes
why: the scope's click verb is refused as disabled before this synthetic primary click at the node's centre is dispatched, so the refusal guards this call
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion sits in the shell's input path, which the chunk's coverage read and actions replay do not touch
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: shell-level key dispatch and shortcut ordering are upstream of the chunk's replayed checks and the handler coverage count
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: a key press on a control whose anonymous block was dropped is not named by the scope, which covers disabled refusal only
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard key bindings forwarded as UI events are shell behaviour the chunk does not name
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding is not named by the scope and the chunk adds no text composition
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus() and blur() and document.activeElement belong to the JS script runtime, which the chunk does not call
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: autofocus reflection is a form-control attribute path the chunk neither writes nor reads
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSS style property accessors reach the script runtime and the visibility exclusion, neither of which the chunk changes
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields are a script-side shape the chunk's Rust coverage and replay do not read
@answer n=11 entity=keyboard-navigation#4.5 v=yes
why: the scope's type verb carries KeyDown, KeyUp and KeyPress to Dioxus as keyboard data in dioxus_document.rs, the path that decides which keydown listeners a replayed check exercises
@answer n=12 entity=keyboard-navigation#4.6 v=yes
why: the signal registry's dispatch set around handle_event, its chain resolution to a listener of the event and the event-named-without-on form are what the coverage's exercised-handler record and graph names rely on
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: focus, blur, focusin and focusout forwarding is not carried by any verb the scope names, and the chunk changes no focus
@answer n=14 entity=keyboard-navigation#4.8 v=no
why: set_focus on mounted elements and the unqueued focus events are not named by the scope, which changes no focus
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the buffered render path and the probe pins are existing behaviour the chunk's read-only coverage does not change or call into
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: set_focus_to returning false for a node mounted in the same render is a limit the chunk's scope does not reach
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: the autofocus cargo feature forwarding is a build wiring the chunk does not change
@answer n=18 entity=keyboard-navigation#4.12 v=no
why: IME handling is a TODO in dioxus-native-dom that the chunk does not implement
@answer n=19 entity=keyboard-navigation#4.13 v=yes
why: a false disabled value removes the attribute, so an enabled control reads enabled and the driver's refusal where enabled is Some(false) depends on it
@answer n=20 entity=keyboard-navigation#5.1 v=no
why: the browser urlbar's arrow and Escape handling is app code outside the chunk's scope
@answer n=21 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts are app code outside the chunk's scope
@answer n=22 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search autofocus is app code outside the chunk's scope
@answer n=23 entity=keyboard-navigation#5.4 v=no
why: rdme's Ctrl or Cmd shortcuts are app code outside the chunk's scope
@answer n=24 entity=keyboard-navigation#5.5 v=no
why: use_back_button's BrowserBack handling is a hook in dioxus-native that the chunk does not call
@answer n=25 entity=keyboard-navigation#5.6 v=no
why: todomvc's focus-on-mount and Enter and Escape handling is example app code the chunk does not touch
