# Classified — a11y · keyboard-navigation.b2

## Answers
keyboard-navigation#3.9 · yes · the centre-free primary Click that keyboard activation dispatches is built here, and node.rs is where the take's hit-testing merge lands
keyboard-navigation#3.10 · no · winit key conversion in blitz-shell is outside the take's named delta, and the standing keyboard checks inject keys through the harness, not winit
keyboard-navigation#3.11 · no · the windowed KeyDown and KeyUp dispatch in blitz-shell's window.rs is outside the take's named delta, and the standing keyboard checks run headless
keyboard-navigation#3.12 · no · macOS AppleStandardKeybinding forwarding is a macOS shell path outside the take's delta, and no standing check reaches it
keyboard-navigation#3.13 · no · IME forwarding is Not needed, since the sync adds no text input method
keyboard-navigation#4.1 · no · focus(), blur() and document.activeElement are blitz-vibey-script's JS binding, outside the take's named delta and the Needs
keyboard-navigation#4.2 · no · autofocus reflection is blitz-vibey-script's JS binding, outside the take's named delta and the Needs
keyboard-navigation#4.3 · no · JS keyboard event fields are blitz-vibey-script's binding, outside the take's named delta and the Needs, the Dioxus-side keyboard data being #4.4
keyboard-navigation#4.4 · yes · KeyDown, KeyUp and KeyPress reach Dioxus as keyboard data through dioxus_document.rs and events.rs, the keyboard delivery the Needs name for re-sync
keyboard-navigation#4.5 · yes · Focus, Blur, FocusIn and FocusOut forwarding sits in the same re-synced dispatch as the keyboard data in dioxus_document.rs, lines 366-369 beside 371-375
keyboard-navigation#4.6 · yes · set_focus and its focus and blur events, which are not queued, live in events.rs, the re-synced file the Needs name for keyboard delivery
keyboard-navigation#4.7 · no · the autofocus forward is not named in the Needs, which keep only the accessibility feature wiring on the Dioxus manifests
keyboard-navigation#4.8 · no · IME is Not needed, since the sync adds no text input method, so the TODO at dioxus_document.rs:384-385 binds nothing here
keyboard-navigation#4.9 · yes · the falsy boolean-attribute removal for disabled, hidden and inert on both attribute paths is the bridge attribute writing that the Dioxus 0.8 sync replaces, named in the Needs
keyboard-navigation#5.1 · no · the urlbar shortcuts sit in apps/browser, an upstream-only surface outside escher's a11y surface
keyboard-navigation#5.2 · no · the browser tab shortcuts sit in apps/browser, an upstream-only surface outside escher's a11y surface
keyboard-navigation#5.3 · no · the browser new-tab search input sits in apps/browser, an upstream-only surface outside escher's a11y surface
keyboard-navigation#5.4 · no · rdme's release shortcuts sit in apps/readme, outside the take's named delta and the Needs, and no standing check reaches them
keyboard-navigation#5.5 · no · the BrowserBack hook in dioxus-native's hooks.rs is a windowed convenience that no Needs item or standing check reaches, its repeat flag being the delivery #4.4 binds
keyboard-navigation#5.6 · no · todomvc's mount focus and its Enter and Escape keys belong to an example app outside the take's named edits and the stand's proofs
keyboard-navigation#5.7 · yes · cells.rs is the non-lean cells task the take edits, so its commit on Enter or blur must hold
keyboard-navigation#5.8 · yes · the crud-list keyboard region, its People name, its aria-selected options and its third Tab stop are the keyboard and state readings the Needs name
keyboard-navigation#5.9 · no · the Preact TodoMVC page in examples/preact is outside the take's delta and the stand's proofs
keyboard-navigation#5.10 · no · Android activity focus in apps/browser and examples/todomvc is outside the take's delta and the Needs
keyboard-navigation#6.1 · no · the sync changes no fixture markup the plan tracks, and the docs.rs fixture's tabindex=-1 is reached by no Needs item
