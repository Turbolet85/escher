@carried n=1 label=architecture/established-decisions fact=fct-7rnkaga4
why: the exercised record must count a click handler reached by a driver's Enter or Space press, since the engine dispatches that Click to the app's handlers, so the record is taken where the bridge dispatches, never inferred from the driver verb.
quote: keyboard activation — a pressed `Enter` whose target is the focused node, after the text-input and range arms, dispatches a Click on it when it is no disabled form control ([DOM semantics]) and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`, and a released Space on the focused node dispatches the same Click, through the new `handle_key_up`, for a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is not disabled — a link takes no Space, and a text input and a range input take neither key there — the Click's data built from `Node::synthetic_click_event_data(modifiers)` and queued by the event driver, so the app's handlers run before the default action `handle_click`
@carried n=1 run=2 label=architecture fact=fct-gejxlyk5
quote: Those two fixes, the accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable with no upstream pull request open.
@carried n=2 label=architecture/established-decisions fact=fct-7rnkaga4
why: a range input's `input` handler is reached by a key or pointerdown that changes its value and not by one that changes nothing, so the coverage count must follow the events the engine actually dispatched.
quote: each change fires one `input` event and a move that changes nothing fires none
@carried n=3 label=architecture/established-decisions fact=fct-i7wbrsm5
why: a checkbox's or radio's `change` handler is reached by the engine's own `change` after a click, so the exercised record must count that (node, `change`) pair as reached.
quote: and the engine's `change` on a checkbox and a radio (§Standard Contracts → Events)
@carried n=3 run=2 label=architecture fact=fct-vmq4u633
quote: and `change` on a checkbox and a radio — each flagged upstreamable with no upstream pull request open
@carried n=4 label=architecture/established-decisions fact=fct-2fe67avo
why: a select's `input` and `change` handlers are reached only through `apply_select_choice`, and a choice of the chosen option reaches neither, so the coverage count must record exactly those dispatches.
quote: Every choice that changes the chosen option or a `multiple` select's chosen set goes through the one path `apply_select_choice`, which takes the chosen set — the `CHECKED` write, the changed set, `input` then `change`, each carrying `select_value`, and a redraw — and a choice of the chosen option fires nothing
@carried n=4 run=2 label=architecture/established-decisions fact=fct-t7ezjz4v
quote: Enter, Space and a click on a row choose through `apply_select_choice`, and a disabled row changes nothing and leaves the list open
@carried n=4 run=3 label=architecture fact=fct-hnlfyl4f
quote: every choice that changes the chosen option or a `multiple` select's chosen set goes through the one path `apply_select_choice`, and a single select's choice of its chosen option fires nothing
@carried n=5 label=architecture/established-decisions fact=fct-oraccejk
why: the coverage read and the exercised record sit in dioxus-native-dom beside the UI graph, which reads under `accessibility`, so they must build within that crate's default feature set and leave the set unchanged.
quote: `dioxus-native-dom`: accessibility, svg, system-fonts, text-transform-icu (packages/dioxus-native-dom/Cargo.toml:13)
@carried n=5 run=2 label=architecture fact=fct-33m5glp2
quote: each crate fixes its default feature set in its manifest
@binding n=1
**Established decisions (engine-dispatched events) ↔ Standard Contracts (Dioxus DOM bridge)**: the (element, event) handlers a run reached are those the engine dispatched — keyboard activation's Click, a range input's `input`, a checkbox's or radio's `change`, a select's `input` then `change` through `apply_select_choice` — so the exercised record is written where those events reach the vdom on the bridge's dispatch path, holding on every app, registry or not, and never inferred from the driver verb that caused them (architecture/established-decisions fct-7rnkaga4) (architecture/established-decisions fct-i7wbrsm5) (architecture/established-decisions fct-2fe67avo) (architecture fct-srb7slak)
@binding n=2
**Established decisions (direct engine changes) ↔ plan**: the coverage command and the owed replay test read those engine behaviours unchanged — no edit reaches blitz-dom's event, keyboard, pointer or select code, whose merge surfaces stand as lines owned by Upstream sync ahead of determinism and exploration (architecture fct-gejxlyk5) (architecture fct-vmq4u633) (architecture fct-dkrt7vg6)
@binding n=3
**Established decisions (default features) ↔ Cross-cutting Patterns (config management)**: the new record and coverage read build inside dioxus-native-dom's default feature set, the `accessibility` feature gating the snapshot modules they read beside, and the default list stays as its manifest fixes it (architecture/established-decisions fct-oraccejk) (architecture fct-nmnb5xux)
