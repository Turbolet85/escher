# Apply — architecture · part 1 of 5

## Verdicts
- fanout-architecture--cascade-g2.md 4 · applied · .andromeda/architecture.md:60#1
- fanout-architecture.md 2 · folded into fanout-architecture--cascade-g2.md 4
- fanout-architecture--cascade-g2.md 10 · applied · .andromeda/architecture.md:62#1
- fanout-architecture.md 1 · folded into fanout-architecture--cascade-g2.md 10
- fanout-architecture--cascade-g2.md 15 · applied · .andromeda/architecture.md:74#1
- fanout-architecture.md 3 · folded into fanout-architecture--cascade-g2.md 15
- fanout-architecture--cascade-g2.md 22 · applied · .andromeda/architecture.md:72#1
- fanout-architecture--cascade-g4.md 4 · applied · .andromeda/architecture.md:135#1
- fanout-architecture.md 5 · folded into fanout-architecture--cascade-g4.md 4
- fanout-architecture--cascade-g4.md 10 · applied · .andromeda/architecture.md:137#1
- fanout-architecture--cascade-g4.md 11 · applied · .andromeda/architecture.md:139#1
- fanout-architecture.md 6 · folded into fanout-architecture--cascade-g4.md 11
- fanout-architecture--cascade-g4.md 15 · applied · .andromeda/architecture.md:141#1
- fanout-architecture.md 7 · folded into fanout-architecture--cascade-g4.md 15
- fanout-architecture--cascade-g4.md 21 · applied · .andromeda/architecture.md:145#1
- fanout-architecture.md 10 · folded into fanout-architecture--cascade-g4.md 21
- fanout-architecture.md 4 · applied · .andromeda/architecture.md:70#1
- fanout-architecture.md 8 · applied · .andromeda/architecture.md:141#2
- fanout-architecture.md 9 · applied · .andromeda/architecture.md:143#1
- fanout-architecture.md 11 · applied · .andromeda/architecture.md:151#1
- fanout-architecture.md 12 · applied · .andromeda/architecture.md:183#1
- fanout-architecture.md 14 · applied · .andromeda/architecture.md:247#1
- fanout-architecture--cascade-g1.md 1 · applied · .andromeda/architecture-detail/stack-and-technologies.md:63#1
- fanout-architecture--cascade-g1.md 2 · applied · .andromeda/architecture-detail/stack-and-technologies.md:73#1

## Edits
- line: .andromeda/architecture.md:60
  old: «and the select model of keys, typeahead and click, each flagged upstreamable with no upstream pull request open.»
  new: «and the select model of keys, typeahead and click, and eight more — `visible_region` through transforms, a non-invertible transform's no hit, the focus fixup under `inert`, the tree's focus falling back to the window, native `toggled`, the `multiple` select, `optgroup` and `change` on a checkbox and a radio — each flagged upstreamable with no upstream pull request open; the merge surface of those eight is a line owned by Epoch 3's sync, `hit_inner` a textual conflict with upstream `0980cce5f`.»
- line: .andromeda/architecture.md:62
  old: «every choice that changes the chosen option goes through the one path `apply_select_choice`, and a choice of the chosen option fires nothing; no popup is painted, an option is not clicked, and a `multiple` select or an `optgroup`'s options are not modelled.»
  new: «every choice that changes the chosen option or a `multiple` select's chosen set goes through the one path `apply_select_choice`, and a single select's choice of its chosen option fires nothing; a `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge; no popup is painted, Enter and Space open nothing on a single select, an option of a closed single select, under a group too, takes no click and stays refused `covered`, `size` is unread and Ctrl+A is not modelled.»
- line: .andromeda/architecture.md:74
  old: «A known limit: HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line.»
  new: «The focus fixup: a node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives it up at the mutator's flush through `BaseDocument::clear_focus`, with no DOM `blur` or `focusout`; the fixup reads neither `disabled` nor `hidden`.»
- line: .andromeda/architecture.md:72
  old: «a `select`'s choice is held as its options' `CHECKED` state, settled by HTML's selectedness algorithm, and a controlled app»
  new: «a `select`'s choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` state, a single select's settled by HTML's selectedness algorithm; its options include its `optgroup` children's; and a controlled app»
- line: .andromeda/architecture.md:135
  old: «`BaseDocument::select_value` reads a select's value, public and read-only, its writers crate-private.»
  new: «`BaseDocument::select_value` reads a select's value, a `multiple` select's first chosen option in tree order, `select_values` its chosen set and `select_of` an option's or group's select, each public and read-only; public `clear_select_typeahead` clears the typed text, its one non-test caller the driver's `focus_multiple_select`; the choice's writers are crate-private.»
- line: .andromeda/architecture.md:137
  old: «a bound `<label>` and option labels, and writing state attributes each read once;»
  new: «a bound `<label>` and option and group labels, and writing state attributes each read once and a native control's checked state on a checkable role;»
- line: .andromeda/architecture.md:139
  old: «Two measured limits: the accessibility tree states no native checkbox's or radio's checked state, its fix owned by a `.andromeda/residuals.md` line, and the platform-tree refresh's windowed witness»
  new: «A native control under a role that is not checkable writes no checked state, and neither author attribute is read on one; a measured limit: the platform-tree refresh's windowed witness»
- line: .andromeda/architecture.md:141
  old: «the engine dispatches `change` only after `input` on a select's choice.»
  new: «the engine dispatches `change` after `input` on a select's choice and on a checkbox's or radio's click, a radio only where it was not checked before, and a page's script hears that one `change`; a text or range input's `input` brings none.»
- line: .andromeda/architecture.md:145
  old: «while `visible_region` reads a scrolling box under a transform where it stands untransformed (a measured limit);»
  new: «and `visible_region` reads each clipping box through its transforms, exact under a translate and a scale and under a rotation the rotated padding box's bounding box, so it holds corners the box does not show (a measured limit), a mapped box with a non-finite edge answering `None`;»
- line: .andromeda/architecture.md:70
  old: «`disabled` is read one way, by its presence, on a form control alone — focusability, the pointer and click gates and the Enter and Space activation read it;»
  new: «`disabled` is read by its presence on a form control through one reading — focusability, the pointer and click gates and the Enter and Space activation read it — and an option's through `option_disabled`, its own attribute or its group's;»
- line: .andromeda/architecture.md:141
  old: «a focus change as `blur`, `focusout`, `focus`, `focusin`;»
  new: «a focus change as `blur`, `focusout`, `focus`, `focusin`, but for the focus fixup under `inert`, which clears focus and dispatches none of them;»
- line: .andromeda/architecture.md:143
  old: «a hit stops at the padding box of a box paint clips and skips a node dropped since the last resolve;»
  new: «a hit stops at the padding box of a box paint clips, skips a node dropped since the last resolve, and answers nothing for the whole subtree of a transform whose determinant is 0 or not finite;»
- line: .andromeda/architecture.md:151
  old: «`ElementData::is_disabled` is the one disabled reading — a form control with a `disabled` attribute present;»
  new: «`ElementData::is_disabled` is the one disabled reading of a form control — a `disabled` attribute present; an option's is the one rule `option_disabled`, its own attribute or its group's;»
- line: .andromeda/architecture.md:183
  old: «a password or file `input`; a select's `input` and `change` reach the app as form data, and its options are its children in the snapshot.»
  new: «a password or file `input`; the public `closed_select_of` and `is_multiple_select` are the select tests the driver reads, naming the element and never the role; a select's `input` and `change` reach the app as form data, a `multiple` select's as one `(name, FormValue::Text(value))` per chosen option, so `evt.values()` reads the set and `value` stays `select_value`; its options are its children in the snapshot, a `multiple` select's rows and groups reading client rects of their own.»
- line: .andromeda/architecture.md:247
  old: «with `--locked` on every cargo leg but `examples/wasm_hello`»
  new: «with `--locked` on every cargo leg but `fmt`, `cargo fmt --all --check` (.github/scripts/ci-leg.sh:20), and `examples/wasm_hello`»
- line: .andromeda/architecture-detail/stack-and-technologies.md:63
  old: «pulldown-cmark 0.13,»
  new: «pulldown-cmark 0.13 (resolved 0.13.5, past RUSTSEC-2026-0414; `rdme` never sets `ENABLE_WIKILINKS`, so no reader is exposed; Cargo.lock:6505; apps/readme/src/markdown/pulldown_cmark.rs:7-12),»
- line: .andromeda/architecture-detail/stack-and-technologies.md:73
  old: «`dtolnay/rust-toolchain` (`# master`, each step naming its `toolchain`) and `taiki-e/install-action` (`# v2`) — every ci.yml»
  new: «`dtolnay/rust-toolchain` (`# master`, each step naming its `toolchain`), `taiki-e/install-action` (`# v2`) and `jlumbroso/free-disk-space`, the free-disk step of the `Stand checks repeated` job (.github/workflows/ci.yml:311-316) — every ci.yml»

## Patterns
[[pattern]]
id = "arch1-01"
regex = 'an option is not clicked, and a `multiple` select or an `optgroup`.s options are not modelled'

[[pattern]]
id = "arch1-02"
fixed = 'focus fixup is not built'

[[pattern]]
id = "arch1-03"
regex = 'choice is held as its options. `CHECKED` state, settled by'

[[pattern]]
id = "arch1-04"
fixed = 'public and read-only, its writers crate-private'

[[pattern]]
id = "arch1-05"
regex = 'tree states no native checkbox.s or radio.s checked state'

[[pattern]]
id = "arch1-06"
fixed = 'dispatches `change` only after `input`'

[[pattern]]
id = "arch1-07"
fixed = 'where it stands untransformed'

[[pattern]]
id = "arch1-08"
fixed = 'by its presence, on a form control alone'

[[pattern]]
id = "arch1-09"
fixed = 'is the one disabled reading — a form control'

[[pattern]]
id = "arch1-10"
regex = 'every cargo leg (`--locked` )?but `examples/wasm_hello`'
