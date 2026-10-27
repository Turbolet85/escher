# Classified — a11y · keyboard-navigation.b2

## Answers
keyboard-navigation#3.9 · yes · node/node.rs is a conflict surface reached whole and the rule (the primary mouse click built at the node's centre) lives there, so its conflict resolution must keep it
keyboard-navigation#3.10 · yes · the scope names keyboard copy and paste and the Enter and Space keys, and this conversion carries their Key and Modifiers to the window's dispatch; convert_events.rs is not reached, so the binding runs through those named keys
keyboard-navigation#3.11 · yes · the rule (every key press and release dispatched as KeyDown or KeyUp after shell shortcuts) lives in window.rs, a merge surface, at 689-697, and the keyboard activation and clipboard keys pass through it
keyboard-navigation#3.12 · yes · the rule lives in window.rs (a merge surface, 577-582) and blitz-traits events.rs (a merge surface, 73 and 158), so it binds through those merges
keyboard-navigation#3.13 · yes · the IME shell hooks live in window.rs (a merge surface, 637-640) and the forwarding of Preedit, Commit and DeleteSurrounding in blitz-traits events.rs (a merge surface, 744-789), the IME surface that 7fc9ee3af's enabling reaches
keyboard-navigation#4.1 · no · focus, blur and activeElement sit in blitz-vibey-script's dom/element.rs and dom/document.rs, neither reached (its reached file runtime.rs is not where these lines sit), and the scope names no script focus read
keyboard-navigation#4.2 · no · the autofocus reflection and the blitz-vibey-script feature sit in its dom/element.rs and Cargo.toml, neither reached, and the scope names no autofocus
keyboard-navigation#4.3 · no · the JS keyboard event fields sit in blitz-vibey-script's dom/event.rs, not reached, and the scope names no script keyboard event
keyboard-navigation#4.4 · yes · dioxus_document.rs is a merge surface and the forwarding of KeyDown, KeyUp and KeyPress as keyboard data lives at 371-375, which the merged bridge must keep
keyboard-navigation#4.5 · yes · dioxus_document.rs is a merge surface and the forwarding of Focus, Blur, FocusIn and FocusOut lives at 366-369, which the merged bridge must keep
keyboard-navigation#4.6 · no · set_focus and the unqueued focus events sit in dioxus-native-dom's events.rs, not reached, and the scope names no Dioxus-side focus call or queued event
keyboard-navigation#4.7 · no · the autofocus forwarding sits in the dioxus-native-dom and dioxus-native manifests, neither reached, and the scope names no autofocus feature
keyboard-navigation#4.8 · yes · the IME TODO at dioxus_document.rs 384-385 lives in a merge surface that 7fc9ee3af, the IME enabling for focused text inputs, also edits
keyboard-navigation#4.9 · yes · the scope names it (a falsy value of a bridge boolean attribute removes it, and the Dioxus 0.8 take reaches the bridge), and the re-read stand_keyboard and inert_subtree are the checks it names as witnesses
keyboard-navigation#5.1 · no · the urlbar's arrow, Escape and Enter handling sits in apps/browser/src/toolbar.rs, which the take does not reach, and the scope names no urlbar key
keyboard-navigation#5.2 · no · the browser tab shortcuts sit in apps/browser/src/tab_strip.rs, not reached, and the scope names no tab shortcut
keyboard-navigation#5.3 · no · the browser new-tab input's autofocus and Enter sit in apps/browser/src/about_pages.rs, not reached, and the scope names no such input
keyboard-navigation#5.4 · no · rdme's release shortcuts sit in apps/readme/src/readme_application.rs, not reached; the release dispatch beneath them is 3.11's, which binds, and the scope names none of these keys
keyboard-navigation#5.5 · no · use_back_button's BrowserBack handling sits in packages/dioxus-native/src/hooks.rs, not reached, and the scope names no BrowserBack key
keyboard-navigation#5.6 · no · the todomvc example's focus on mount and its Enter, Escape and Tab handling sit in examples/todomvc/src/app.rs, not reached, and the scope names no todomvc page
keyboard-navigation#5.7 · no · the Cells commit on Enter or blur sits in seven_guis's cells.rs, not reached, and Cells holds no keyboard-activation row (the pages served beside the lean four hold none)
keyboard-navigation#5.8 · yes · the Needs name the CRUD list's keyboard model and the unread aria-activedescendant, re-asserted by the re-read stand_keyboard and accessibility_active_descendant_unread, which the entity names as its witness
keyboard-navigation#5.9 · no · the Preact example's autofocus and Enter sit in examples/preact/index.html, not reached, and the scope names no Preact page
keyboard-navigation#5.10 · no · the Android activities' focusable view sits in Kotlin files under apps/browser and examples/todomvc, outside the Rust take and not reached
keyboard-navigation#6.1 · no · the docs.rs fixture sits in examples/assets, which the take edits none of and no re-proven check reads; the tabindex rules its markup exercises are the ones the node.rs and mutator.rs resolutions keep, which stand_keyboard's tables re-read
