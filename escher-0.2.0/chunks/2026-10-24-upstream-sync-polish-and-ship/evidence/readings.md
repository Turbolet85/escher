# Readings — CARRY 1–5, per row (step 5)

Read on the take's staged tree against the base `a356f9086` (`git diff --cached a356f9086 -- {file}`), 2026-10-24T11:41Z. Nothing is sent upstream: no upstream pull request is opened (the founder's standing hold, 2026-10-07/2026-10-10).

## CARRY 1 — the Taffy-main commits
They wait again, now six in all: the four `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`, plus `b8f4ba6fd` (#1242) and `9babd796a` (#1253), which move upstream's git Taffy to `802e497b…`. Each puts a git Taffy in the root manifest, and the founder kept the pins («Десять коммитов (Рекомендую)», inputs#I4). The root manifest still reads `taffy = "0.15"` from the registry (the preservation guard and the no-git-dependency entry). The CARRY moves to the next sync's entry.

## CARRY 2 — `keyboard_compat.rs`
Upstream retired it at `286874cc1` (#1260): `git ls-tree -r --name-only 286874cc1 -- packages/dioxus-native-dom/src/keyboard_compat.rs` reads 0 lines (a gate entry). This take leaves `286874cc1` out, so the file stays on our tree and both `cargo llvm-cov` lines in `.github/scripts/ci-leg.sh` keep `--ignore-filename-regex 'keyboard_compat\.rs$'` (a gate entry, 2 hits). The CARRY moves on with the reading "retired upstream at `286874cc1`, kept here, both exclusions standing".

## CARRY 3 — the engine-debts merge surface, per F-row
The take reaches only `node/node.rs` (`46d7cf993`, `f102663b8`) and `node/element.rs` (`e4ca5b9b5`) among the rows' files. `events/keyboard.rs`, `document.rs`, `accessibility.rs` and `events/pointer.rs` are not reached (`git diff --cached --stat` lists none of them).
- **`inert`** — `hit_inner`'s `inert` stop is untouched. `f102663b8`'s hunks sit below it, in the positive-z hoisted loop (`self.with(id)` → `self.tree().get(id)` with `continue`), the paint-children loop, the negative-z hoisted loop and the text-cluster lookup. `inert_subtree` is the reading.
- **the toggle per role, `role="switch"`** — `accessibility.rs` is not reached; untouched.
- **`Node::absolute_position`** — untouched; no hunk in `absolute_position` or `position_in_containing_block`. `scrolled_box_absolute_position` is the reading.
- **the hit's stop at `contain: paint`** — the clip stop at a box paint clips is untouched (no hunk between the visibility and `pointer-events` rules and the child loops). `hit_clipped_at_scrolling_box` is the reading.
- **the password click caret** — `document.rs` and `events/pointer.rs` are not reached; untouched. `password_caret_click` is the reading.
- **`disabled` on form controls** — `ElementData::is_disabled` is untouched. `e4ca5b9b5` adds the `classes` field, its `parse_classes` and its `new`/`clone` lines beside it.
- **the transformed client rect** — `document.rs` is not reached; untouched. `transformed_client_rect` is the reading.
Beside the rows: `46d7cf993` adds `Node::out_of_flow` (an `AtomicBool`), `record_out_of_flow` and the free `style_is_out_of_flow`. `Node::is_out_of_flow` keeps its signature and its style reading, now through `style_is_out_of_flow` (`Absolute | Fixed` and not `display: none`).

## CARRY 4 — the one-virtual-clock merge surface, per file
- `config.rs` — reached by `fb297a38c`'s doc comments alone (`StyleThreading`'s doc, `Parallel`'s and `Sequential`'s variant docs, the `style_threading` field doc now `Sequential`). The `clock` field is untouched.
- `resolve.rs` — also reached, by `28f848d3b`'s import list alone: `build_inline_layout_into` is now imported from `inline` in place of `construct` (+2 −2). No time read is touched. The plan's step 5 named `config.rs` alone; this file is the reading's correction.
- `clock.rs`, `document.rs`, `iframe.rs`, `scrolling.rs`, `events/pointer.rs` — not reached.
- Wall-clock reads in blitz-dom outside `clock.rs`: 0 (a gate entry).

## CARRY 5 — the select-by-id merge surface, per file
- `mutator.rs` — reached by `e4ca5b9b5`'s two class arms: `set_attribute`'s `class` arm after the `id` arm, ahead of the `value` arm our range re-seed reads, and `remove_attribute`'s `classes.clear()` after the `id` arm. The range and select arms and the changed-set inserts are untouched.
- `node/element.rs` — reached by the `classes` field and its three sync points (above). The select's `CHECKED` choice and `select_typeahead` are untouched.
- `node/select.rs`, `document.rs`, `events/{keyboard,pointer,mod}.rs`, `accessibility.rs`, `form.rs`, `assets/default.css`, blitz-traits `events.rs`, blitz-vibey-script `runtime.rs` — not reached.
- `select_interaction` and `stand_act_select` are the readings (the integration entry).
