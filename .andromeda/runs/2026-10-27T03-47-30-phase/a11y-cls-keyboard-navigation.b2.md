# Classified — a11y · keyboard-navigation.b2

## Answers
keyboard-navigation#3.9 · no · the engine's synthetic primary-click builder; the README names the click verb's id addressing, not where a click lands, and no chunk code or test touches it
keyboard-navigation#3.10 · no · winit key conversion in the windowed shell; the chunk names no key conversion and changes no shell code
keyboard-navigation#3.11 · no · shell KeyDown and KeyUp dispatch after shell shortcuts; the headless press runs through the harness, the README names no shell shortcut, and no shell code changes
keyboard-navigation#3.12 · no · macOS standard key bindings forwarded as AppleStandardKeybinding events; a platform path the chunk names nowhere and does not change
keyboard-navigation#3.13 · no · IME event forwarding; IME is Not needed (no page or fixture change) and the chunk's claims name no IME
keyboard-navigation#4.1 · no · script-side focus, blur and document.activeElement in the Boa bindings; the README names no script focus API and the chunk changes no binding
keyboard-navigation#4.2 · no · autofocus reflection in the Boa bindings; no re-read check covers boot focus and the README claims no autofocus
keyboard-navigation#4.3 · no · script keyboard event fields in the Boa bindings; the chunk names no script key event and changes no binding
keyboard-navigation#4.4 · no · KeyDown, KeyUp and KeyPress reaching Dioxus as keyboard data; the press's activation is blitz-dom's own Click path and no re-read check covers a stand key handler
keyboard-navigation#4.5 · no · Focus, Blur, FocusIn and FocusOut forwarded as Dioxus focus data; the chunk names no focus event and changes no bridge code
keyboard-navigation#4.6 · no · Dioxus set_focus with its unqueued focus-event TODO; a framework gap none of the four debts the Later line names, and the chunk changes no code that calls it
keyboard-navigation#4.7 · no · the autofocus feature wiring in the Cargo manifests; a build feature the chunk does not change and no claim names
keyboard-navigation#4.8 · no · IME events unhandled in the Dioxus bridge; IME is Not needed (no page or fixture change) and the chunk's claims name no IME
keyboard-navigation#4.9 · yes · the re-pointed v010-17 witness's exactly-one selected reading and the restated control rows rest on falsy selected and disabled being removed (two of the bridge's 28 names), a re-pinned row's input, so the Not-needed bridge line does not drop it
keyboard-navigation#5.1 · no · the browser's urlbar arrow, Escape and Enter keys; the reference browser is outside the six apps and the README names no browser key
keyboard-navigation#5.2 · no · the browser's Cmd or Ctrl T and W tab shortcuts; the chunk names no browser shortcut and changes no browser code
keyboard-navigation#5.3 · no · the browser's new-tab search autofocus and Enter submit; outside the six apps and the chunk's claims
keyboard-navigation#5.4 · no · rdme's Ctrl or Cmd R, T and B shortcuts; the markdown viewer is outside the chunk's files and claims
keyboard-navigation#5.5 · no · use_back_button's BrowserBack hook; the chunk names no back control and changes no hook
keyboard-navigation#5.6 · no · TodoMVC's new-todo focus, Enter and Escape keys; TodoMVC is not one of the six apps and the chunk names none of its keys
keyboard-navigation#5.7 · yes · Cells is one of the six apps and the six-app line must claim no keyboard coverage of it; this entry records Cells' own Enter and blur commit, an app key handler rather than the engine's keyboard activation
keyboard-navigation#5.8 · no · the CRUD list's arrow keys and unread aria-activedescendant are the task's Not-needed CRUD line (no stand change), and the chunk's re-read checks name no CRUD keyboard behaviour
keyboard-navigation#5.9 · no · the Preact TodoMVC input's autofocus and Enter; an example page outside the chunk's files and the six apps
keyboard-navigation#5.10 · no · Android Activity focusability in touch mode; the Android apps are outside the chunk's files and claims
keyboard-navigation#6.1 · no · the docs.rs fixture's tabindex -1 toolbar containers; a fixture page outside the stand and the chunk's files
