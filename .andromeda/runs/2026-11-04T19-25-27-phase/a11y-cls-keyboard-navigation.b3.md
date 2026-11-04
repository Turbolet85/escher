@answer n=1 entity=keyboard-navigation#3.22 v=yes
why: the synthetic primary click at the node centre is the Click that Enter and Space dispatch to a focused button, which the probe's link (2) click answer reads, and the scope leaves open whether that Click is this synthetic one
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion is the windowed shell path; the probe's press and press_with inject key data headlessly and never pass winit, which is read, not run
@answer n=3 entity=keyboard-navigation#3.24 v=yes
why: the press verb the probe drives produces the KeyDown and KeyUp dispatch this entity states, so a key handler's receipt of those events binds
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the probe presses only the visible Counter or CRUD controls, and a press on a control whose anonymous block was dropped by hidden, display none or CSS is a case the scope does not name
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS AppleStandardKeybinding forwarding is a windowed shell path the headless probe does not reach
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME forwarding is a windowed shell path, and the probe sends no IME events
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus(), blur() and document.activeElement are the JavaScript API of blitz-vibey-script, and the probe is Dioxus and reads focus only off the accessibility tree
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the autofocus reflection is a JavaScript attribute write, and the probe names no autofocus control
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSSStyleDeclaration accessors are the JavaScript script-to-DOM crossing, and the probe writes no style property
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JavaScript keyboard event fields belong to blitz-vibey-script, and the probe's keyboard data arrives through Dioxus
@answer n=11 entity=keyboard-navigation#4.5 v=yes
why: KeyDown, KeyUp and KeyPress reach Dioxus handlers as keyboard data with key, code and repeat, and the probe's key handler reads that data when press drives it
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: Focus, Blur, FocusIn and FocusOut focus data is not read by any link of the probe, which reads focus off the accessibility tree
@answer n=13 entity=keyboard-navigation#4.7 v=yes
why: the hazard's MountedData call is unnamed in the scope, so set_focus is an open fork over it, and focus and blur events not being queued is what a probe focusing a control through set_focus would have to honour
@answer n=14 entity=keyboard-navigation#4.8 v=yes
why: the MountedData RefCell panic from a key-handler task first polled inside render_immediate is the hazard the scope's item 5 names and the probe re-verifies
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the autofocus Cargo feature forwarding is not a feature the probe enables; the scope names only the accessibility feature, and the stand names no autofocus
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: IME handling is a TODO in dioxus-native-dom, and the probe sends no IME events
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: the falsy boolean attribute removal is the bridge's attribute write path, which the scope names not needed because the probe writes no attribute path
@answer n=18 entity=keyboard-navigation#5.1 v=no
why: the Browser urlbar keys belong to apps/browser, which the probe does not reach
@answer n=19 entity=keyboard-navigation#5.2 v=no
why: the Browser Cmd+T and Cmd+W tab shortcuts belong to apps/browser, which the probe does not reach
@answer n=20 entity=keyboard-navigation#5.3 v=no
why: the Browser new-tab search input autofocus belongs to apps/browser, which the probe does not reach
@answer n=21 entity=keyboard-navigation#5.4 v=no
why: the rdme shortcuts on key release belong to apps/readme, a different app from the seven_guis stand
@answer n=22 entity=keyboard-navigation#5.5 v=no
why: the use_back_button hook is not on the probe's path, and the probe presses no BrowserBack key
@answer n=23 entity=keyboard-navigation#5.6 v=no
why: the todomvc focus, Enter and Escape behaviour belongs to examples/todomvc, not the Counter or CRUD stand
@answer n=24 entity=keyboard-navigation#5.7 v=no
why: Cells commits an edit on Enter or blur, and Cells is not the probe's task
@answer n=25 entity=keyboard-navigation#5.8 v=no
why: the Cells grid Tab stop and its synchronous scroll_to belong to the Cells grid model, which is not_needed and not used by the probe
