proposals:
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → Scrolling and input"
    change: "The select model: every choice that changes the chosen set goes through the one path `apply_select_choice`, and a choice that changes nothing fires nothing; a `multiple` select is modelled, its rows taking keys and a click (plain chooses the row alone, Ctrl or Meta flips it, Shift spans, a disabled row never chosen), and an `optgroup`'s options are in its select's list, its non-empty `label` named as `Role::Group` in the tree and unpainted; no popup is painted, Enter and Space open nothing on a single select, an option of a closed single select takes no click and stays refused `covered`, under a group too, `size` is unread and Ctrl+A is not modelled."
    sidecar: "Established Decisions, the select model: `multiple` and `optgroup` modelled, the no-click limit narrowed to a closed select's option, no popup still painted (2026-10-31-engine-debts F6, F7, F8)."
    rationale: "Report Changes F6, F7, F8 and the Counts bullet: patterns `multiple-select`, `optgroup` and `option-click` each name `architecture.md:62` as changed, and `popup` restates no-popup beside its moved limit."
    basis: "packages/blitz-dom/src/events/pointer.rs:1013-1054"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → Scrolling and input"
    change: "Add after the seven-changes sentence: Beside those, eight more direct engine changes for every document, each ratified by the founder's own word on 2026-10-31 and flagged upstreamable with nothing sent — `visible_region` through transforms, a non-invertible transform taking no hit, the focus fixup under `inert`, the tree's focus falling back to the window, native `toggled`, the `multiple` select, `optgroup`, and `change` on a checkbox and a radio; their merge surface is a line owned by Epoch 3's sync, `hit_inner` a textual conflict with upstream `0980cce5f`."
    sidecar: "Established Decisions: the eight engine changes ratified on 2026-10-31 recorded as direct, upstreamable changes with their merge surface (2026-10-31-engine-debts)."
    rationale: "Report Expected amendments, architecture §Established Decisions: the eight engine changes ratified by the founder, flagged upstreamable, their merge surface a line owned by Epoch 3's sync; Cross-project claims name `0980cce5f`'s `hit_inner` conflict. The body counts the earlier direct engine changes and holds none of these."
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → DOM semantics"
    change: "HTML's focus fixup is built for `inert`: where the focused node reads `is_inert()`, the mutator's flush clears focus through `BaseDocument::clear_focus`, a subtree moved under an inert box included; a known limit, it fires no DOM `blur` or `focusout`, and `disabled` and `hidden` read as before."
    sidecar: "Established Decisions, DOM semantics: the focus-fixup limit retired, the fixup built with no `blur` (2026-10-31-engine-debts F3)."
    rationale: "Report Changes F3 and the Counts bullet: pattern `focus-fixup` names `architecture.md:74` as changed; the founder's word is to clear focus without `blur`."
    basis: "packages/blitz-dom/src/mutator.rs:1051-1063"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → DOM semantics"
    change: "`disabled` is read by its presence on a form control through one reading — focusability, the pointer and click gates and the Enter and Space activation read it — and an option's through `option_disabled`, its own attribute or its group's."
    sidecar: "Established Decisions, DOM semantics: an option's and its group's `disabled` named beside the form-control reading (2026-10-31-engine-debts F7)."
    rationale: "Report Changes F6 and F7: `option_disabled` is the one rule the select's `readings` and `form.rs:281` share, and disabled rows are never chosen, while the body says `disabled` is read on a form control alone; the same claim as the Nodes and widgets proposal."
    basis: "packages/blitz-dom/src/node/select.rs:175-190"
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Document core (blitz-dom)"
    change: "`BaseDocument::select_value` reads a select's value, a `multiple` select's first chosen option in tree order, public and read-only; public `select_values` reads the chosen set, public read-only `select_of` answers the select of an option or group, and public `clear_select_typeahead` clears the typed text, its one non-test caller the driver's `focus_multiple_select`; a select's value writers stay crate-private."
    sidecar: "Standard Contracts, Document core: `select_values`, `select_of` and `clear_select_typeahead` registered, `select_value` on a `multiple` select stated (2026-10-31-engine-debts F6, F7)."
    rationale: "Report Changes F6, F7, Sole-caller facts and Deviations 4 land three new public `BaseDocument` items; pattern `multiple-select` names `architecture.md:135` as changed, and the body says all of a select's writers are crate-private with none of the three registered."
    basis: "packages/blitz-dom/src/document.rs:736-755"
  - detector: D-arch-decisions
    severity: warning
    section: "§Standard Contracts → Mutation, query and CSSOM"
    change: "The tree writes `Toggled` from `checkbox_input_checked()` on a native checkbox or radio under a checkable role and nothing under any other role, reading neither author attribute on a native control; one measured limit stands: the platform-tree refresh's windowed witness is still owed, the refresh held by the document-level tests and both feature builds of blitz-shell."
    sidecar: "Standard Contracts, Mutation: the native-checked-state limit and its residual pointer retired, the tree's `toggled` write stated (2026-10-31-engine-debts F5)."
    rationale: "Report Changes F5 and the `native-toggled` pattern (9 of 10 hits changed): the body still says the tree states no native checkbox's or radio's checked state, its fix owned by a residual line."
    basis: "packages/blitz-dom/src/accessibility.rs:199-224"
  - detector: D-arch-decisions
    severity: warning
    section: "§Standard Contracts → Events"
    change: "The engine dispatches `change` after `input` on a select's choice, on a checkbox's click always and on a radio's click only where it was not checked before, and a page's script hears that one `change`, from the engine; a text or range input's `input` brings no `change`."
    sidecar: "Standard Contracts, Events: `change` extended to the checkbox and radio, the script runtime's synthesis retired (2026-10-31-engine-debts F10)."
    rationale: "Report Changes F10 and Retired: `handle_click` dispatches `change` on a checkbox and a radio, and `runtime/events.rs` loses its `change` synthesis, so the body's `change` only after `input` on a select's choice is false; `text_range_no_change_event` holds the text and range half. The `change-event` pattern does not reach this wording, so this is this sweep's own hit."
    basis: "packages/blitz-dom/src/events/pointer.rs:793-799"
  - detector: D-arch-decisions
    severity: warning
    section: "§Standard Contracts → Events"
    change: "A focus change dispatches `blur`, `focusout`, `focus`, `focusin`, but for the focus fixup under `inert`, which clears focus and dispatches none of them."
    sidecar: "Standard Contracts, Events: the focus fixup named as the one focus change with no events (2026-10-31-engine-debts F3)."
    rationale: "Report Changes F3: no DOM `blur` or `focusout` fires at the fixup, so the body's unqualified focus-change sequence restates what the DOM semantics proposal's change now excepts."
    basis: "packages/blitz-dom/src/mutator.rs:1051-1063"
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Scrolling, selection, tree"
    change: "A hit stops at the padding box of a box paint clips, skips a node dropped since the last resolve, and answers nothing for the whole subtree of a transform whose determinant is 0 or not finite."
    sidecar: "Standard Contracts, Scrolling: a non-invertible transform's no-hit answer stated, a first statement of it in the masters (2026-10-31-engine-debts F2)."
    rationale: "Report Changes F2, and Counts: `non-invertible`'s control is unfired, so a master holds no statement of F2; Expected amendments name this contract. `architecture.md:143`'s `visible_region` reader is unmoved."
    basis: "packages/blitz-dom/src/node/node.rs:1472-1477"
  - detector: D-arch-decisions
    severity: warning
    section: "§Standard Contracts → Scrolling, selection, tree"
    change: "`get_client_bounding_rect` reads an element where its transforms move it, and `visible_region` reads each clipping box through its transforms — exact under a translate and a scale, under a rotation the bounding box of the rotated padding box, so it holds corners the box does not show (a measured limit), and a mapped box with a non-finite edge answers `None`; `BaseDocument::text_extent` is a read-only reader that mutates nothing."
    sidecar: "Standard Contracts, Scrolling: the untransformed `visible_region` limit retired, the rotation limit stated in its place (2026-10-31-engine-debts F1)."
    rationale: "Report Changes F1 and Retired (`off-screen`'s transformed-box clause, the word `untransformed`): pattern `visible-region` names `architecture.md:145` as changed."
    basis: "packages/blitz-dom/src/scrolling.rs:790-812"
  - detector: D-arch-decisions
    severity: warning
    section: "§Standard Contracts → Nodes and widgets"
    change: "`ElementData::is_disabled` is the one disabled reading of a form control — a `disabled` attribute present; an option's is the one rule `option_disabled`, its own attribute or its group's; the select's rules live in the crate-private `node::select`, with no public type."
    sidecar: "Standard Contracts, Nodes and widgets: `option_disabled` stated beside `is_disabled` (2026-10-31-engine-debts F7)."
    rationale: "Report Changes F7 names `option_disabled` the one rule `readings` and `form.rs:281` share, and the new text in `docs/capabilities.md` has a disabled group's rows skipped, so `is_disabled` is no longer the one disabled reading as the body words it."
    basis: "packages/blitz-dom/src/node/select.rs:175-190"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: "`is_masked_input` is the one masked-control test, a password or file `input`; the public read-only `closed_select_of` and `is_multiple_select` are the driver's select tests, naming the element and never the role; a select's `input` and `change` reach the app as form data, a `multiple` select's as one `(name, FormValue::Text(value))` per chosen option, so `evt.values()` reads the set and `value` stays `select_value`; its options are its children in the snapshot, a `multiple` select's rows and groups reading client rects of their own."
    sidecar: "Standard Contracts, Dioxus DOM bridge: `closed_select_of` and `is_multiple_select` registered, a `multiple` select's form data and rows stated (2026-10-31-engine-debts F6, F9)."
    rationale: "Report Changes, Sole-caller facts and Deviations 4: two public re-exports of dioxus-native-dom unregistered in the body, and F6's `multiple_select_values` and `snapshot.rs` give a `multiple` select a form-data and row shape the body does not state."
    basis: "packages/dioxus-native-dom/src/snapshot.rs:242-252"
  - detector: D-new-detail
    severity: warning
    section: "§Infrastructure Patterns → CI/CD"
    file: new
    near: "- **CI/CD:** GitHub Actions on pull_request and push to `main` ... stand checks repeated (`stand-repeat`, since 2026-10-14-quality-gates: `ubuntu-latest`, needing the fast four"
    change: "- **CI/CD:** `stand-repeat` runs `jlumbroso/free-disk-space` pinned at `54081f138730dfa15788a46383842cd2f914a1be`, after the toolchain and before `Swatinem/rust-cache`, with `android: true`, `large-packages: false` and `swap-storage: false` (.github/workflows/ci.yml:311-316), since 2026-10-31-engine-debts; its first witness in this workflow ran `completed · success`, and it is not shown to be the cure of the out-of-disk red."
    sidecar: "Infrastructure Patterns, CI/CD: the free-disk step in `stand-repeat` stated, its cure unproven (2026-10-31-engine-debts)."
    rationale: "Report Harness / gate surface and Dependencies: a CI action site is added, and `free-disk`'s control is unfired, so no master states the step; §Infrastructure Patterns has no category file. Insufficient fixes holds the unproven cure; the older skipped step is P5's and is not stated here."
    basis: ".github/workflows/ci.yml:311-316"
note: "No new port, listener, env var, crate or library in the report (`pulldown-cmark` is a lock-only patch the body does not name); F13's unmet clause is the report's P2 escalation and touches no architecture text."
