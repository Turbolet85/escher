# Classified — a11y · keyboard-navigation.b3

## Answers
keyboard-navigation#3.24 · no · KeyDown/KeyUp dispatch after shell shortcuts lives in blitz-shell, upstream-owned and not reached; the chunk's key checks drive the document directly
keyboard-navigation#3.25 · no · macOS AppleStandardKeybinding forwarding lives in blitz-shell and blitz-traits, neither reached nor named by the scope
keyboard-navigation#3.26 · no · IME forwarding from the shell lives in blitz-shell and blitz-traits, not reached; the document-side IME gap is #4.10
keyboard-navigation#4.1 · yes · script focus(), blur() and activeElement exposure is the script focus surface the events.rs and bootstrap.rs add/add resolution must keep
keyboard-navigation#4.2 · yes · the script autofocus reflection writing "true", which blitz-dom's autofocus takes, is script focus exposure under the same events.rs and bootstrap.rs resolution and sets the initial focus the tier-0 focus order reads
keyboard-navigation#4.3 · no · CSSStyleDeclaration accessors live in css.rs and style.js, neither reached; not keyboard or event exposure, and the hidden-content checks read CSS, not script style writes
keyboard-navigation#4.4 · yes · JS keyboard event fields are the script keyboard and event exposure the events.rs and bootstrap.rs resolution keeps
keyboard-navigation#4.5 · yes · KeyDown, KeyUp and KeyPress keyboard data reaching Dioxus is in dioxus_document.rs, a conflict surface that the stand_keyboard checks read
keyboard-navigation#4.6 · yes · Focus, Blur, FocusIn and FocusOut forwarding is in dioxus_document.rs, a conflict surface that the tier-0 focus-order checks read
keyboard-navigation#4.7 · yes · set_focus on mounted elements is the mounted-handle path Cells' editor focus return acts through, which the cells.rs resolution keeps
keyboard-navigation#4.8 · yes · Cells calls its MountedData handles synchronously, the rule the cells.rs conflict must keep so the RefCell panic under a spawned task is avoided
keyboard-navigation#4.9 · no · the autofocus feature forwarding lives in the dioxus-native and dioxus-native-dom manifests, neither reached; the root Cargo.toml's feature set is not named by the scope and no check needs a value passed
keyboard-navigation#4.10 · yes · the IME TODO lives in dioxus_document.rs, a conflict surface; its resolution must not silently add or drop IME handling
keyboard-navigation#4.11 · yes · the falsy-value removal for the bridge's boolean attributes (is_bool_attr; disabled, hidden, inert) is named by the Needs, reached through a26c6ddeb and 286874cc1, and the stand_keyboard and inert_subtree checks read it
keyboard-navigation#5.1 · no · browser urlbar keys live in apps/browser, not reached and not named by the scope
keyboard-navigation#5.2 · no · browser tab shortcuts live in apps/browser, not reached and not named by the scope
keyboard-navigation#5.3 · no · browser new-tab search autofocus lives in apps/browser, not reached and not named by the scope
keyboard-navigation#5.4 · no · rdme shortcuts live in apps/readme, not reached and not named by the scope
keyboard-navigation#5.5 · no · use_back_button lives in dioxus-native hooks.rs, not reached; no scope item names the BrowserBack key
keyboard-navigation#5.6 · no · the todomvc example lives in examples/todomvc, not reached and not named by the scope
keyboard-navigation#5.7 · yes · Cells' editor commit on Enter or blur is the cells.rs editor commit the scope names
keyboard-navigation#5.8 · yes · cells-grid as one Tab stop with its arrows and synchronous scroll_to is the Cells keyboard behaviour the scope names and stand_keyboard re-reads
keyboard-navigation#5.9 · yes · Cells' editor open, Enter commit and focus return are named by the scope through the cells.rs conflict
keyboard-navigation#5.10 · yes · the CRUD list walks (Tab stop, ArrowDown, ArrowUp, Home, End) are the list walks stand_keyboard re-reads, and the unread aria-activedescendant is pinned by accessibility_active_descendant_unread
keyboard-navigation#5.11 · no · the TodoMVC Preact example lives in examples/preact, not reached and not named by the scope
