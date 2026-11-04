@answer n=1 entity=keyboard-navigation#3.22 v=yes
why: the probe's CRUD row click (item 8) is carried by the harness's synthetic primary click at the node centre, where the probe must aim
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion is windowed-shell input, and the probe presses keys headlessly (item 10)
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: the shell shortcut check before KeyDown dispatch is on the windowed path, read not run (item 10)
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the probe presses keys only on visible Counter and CRUD controls, never on a control whose anonymous block was dropped
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS AppleStandardKeybinding forwarding is a macOS shell path the headless probe never reaches
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME forwarding is a shell path, and the probe drives no text composition
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus(), blur() and activeElement are blitz-vibey-script JS bindings, not the Dioxus path the probe uses
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the autofocus reflection is JS-side in blitz-vibey-script, and the probe names no autofocus control
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSSStyleDeclaration accessors are JS-side vibey-script, and the probe reads no style
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields belong to vibey-script, not to the Dioxus handlers the probe reads
@answer n=11 entity=keyboard-navigation#4.5 v=yes
why: the probe's key handlers read key data (press_with modifiers, the CRUD list's arrow keys) as Dioxus keyboard data
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: focus and blur forwarding is read by no link, and the snapshot reads focus from the accessibility tree (item 3)
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: set_focus queueing is a MountedData focus call the probe does not make, and the events.rs re-read covers only the RefCell check at line 162
@answer n=14 entity=keyboard-navigation#4.8 v=yes
why: the MountedData RefCell panic from a key-handler-spawned task is the hazard the chunk re-verifies at events.rs:162 and the dioxus_document.rs poll lines
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the autofocus feature wiring in Cargo.toml is not reached, since the probe uses the accessibility feature, not autofocus
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: IME handling is a TODO in dioxus-native-dom, and the probe drives no IME
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: falsy boolean attribute removal is a bridge attribute path the probe writes no part of (task item 8, not needed), and aria-selected sits outside the 28-name list
@answer n=18 entity=keyboard-navigation#5.1 v=no
why: browser urlbar keys belong to apps/browser, outside the stand the probe drives
@answer n=19 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts belong to apps/browser, outside the stand the probe drives
@answer n=20 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search autofocus belongs to apps/browser, outside the stand the probe drives
@answer n=21 entity=keyboard-navigation#5.4 v=no
why: rdme shortcuts belong to apps/readme, outside the stand the probe drives
@answer n=22 entity=keyboard-navigation#5.5 v=no
why: use_back_button handles BrowserBack presses, a hook the Counter or CRUD probe never uses
@answer n=23 entity=keyboard-navigation#5.6 v=no
why: the todomvc focus and Enter behaviour belongs to a different example app, not Counter or CRUD
@answer n=24 entity=keyboard-navigation#5.7 v=no
why: the Cells edit commit belongs to the Cells task, which the probe does not drive
@answer n=25 entity=keyboard-navigation#5.8 v=no
why: the Cells grid tab stop and synchronous scroll_to are Cells behaviour, which the probe does not drive
