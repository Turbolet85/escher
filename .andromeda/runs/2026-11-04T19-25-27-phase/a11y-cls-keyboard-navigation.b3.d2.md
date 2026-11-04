@answer n=1 entity=keyboard-navigation#3.22 v=yes
why: the CRUD row click the probe drives is synthesised here as a primary click at the node's centre, which the handler-to-state link relies on.
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion is the windowed shell path, which the headless press never reaches and the chunk only reads, not runs.
@answer n=3 entity=keyboard-navigation#3.24 v=yes
why: the key handler the probe reaches through press is reached via the KeyDown and KeyUp dispatch this line names.
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the probe presses keys on controls that keep their box, and the no-panic on a dropped anonymous block is neither named nor acted on by the chunk.
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS AppleStandardKeybinding forwarding is a shell path the headless probe does not reach.
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME events are outside the probe's key presses, and the chunk reads no preedit or commit.
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus() and document.activeElement are script-side, while the probe reads focus from the accessibility tree.
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the autofocus reflection is a script-side attribute write, and the probe writes no attribute path.
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSS property accessors are script-side style writes, which the probe does not perform.
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields belong to the script runtime, which the probe does not run.
@answer n=11 entity=keyboard-navigation#4.5 v=yes
why: the probe's key handler receives the key, code and modifier data that press and press_with carry, so the reached handler depends on this mapping.
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: focus and blur forwarding is not read by any of the probe's links, which take focus from the tree and clicks from the listener map.
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: set_focus and the unqueued focus events are not named by the chunk, which drives focus through keys, and the hazard's MountedData call is carried by 4.8.
@answer n=14 entity=keyboard-navigation#4.8 v=yes
why: the task re-verifies this MountedData hazard, a task spawned from a key handler, and its events.rs:162 panic.
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the autofocus Cargo feature forwarding is a build feature the chunk neither enables nor reads.
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: IME is unhandled in the bridge, and the probe uses no IME path.
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: falsy boolean-attribute removal is not_needed by the task, since the probe writes no attribute path and disabled-ness and inert are outside its scope.
@answer n=18 entity=keyboard-navigation#5.1 v=no
why: browser urlbar keys live in apps/browser, outside the seven_guis stand.
@answer n=19 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts live in apps/browser, outside the seven_guis stand.
@answer n=20 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search autofocus lives in apps/browser, outside the seven_guis stand.
@answer n=21 entity=keyboard-navigation#5.4 v=no
why: rdme shortcuts live in apps/readme, outside the seven_guis stand.
@answer n=22 entity=keyboard-navigation#5.5 v=no
why: use_back_button handles BrowserBack in the native hooks, which the Counter or CRUD stand never uses.
@answer n=23 entity=keyboard-navigation#5.6 v=no
why: todomvc key handling lives in examples/todomvc, not the seven_guis stand the probe drives.
@answer n=24 entity=keyboard-navigation#5.7 v=no
why: Cells edit commits belong to the Cells task, which the Counter or CRUD stand does not use.
@answer n=25 entity=keyboard-navigation#5.8 v=no
why: the Cells grid keyboard model is not_needed by the task.
