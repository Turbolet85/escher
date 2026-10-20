# Step 3 — the `:checked` mechanism, measured before steps 4-16

Run 2026-10-20 by /andromeda-implement (run dir `.andromeda/runs/2026-10-20T04-14-31-implement/`), after steps 2 and
3 only (`option` seeds `CHECKED` from `selected` at creation; `select { display: inline-block }` with `input`'s
border, padding and background; `select:not([multiple]) > option:checked { display: inline }`), before any other
step. A throwaway blitz-tests target `zz_checked_probe.rs`, removed after the run, booted an `HtmlDocument` through
`Harness::from_html_with` holding the step-16 fixture's selects (`fx-pick` Apple / Banana `selected` / Cherry,
`fx-named` Red / Green `selected`, `fx-off` `disabled` X `selected`), read each select's and option's computed
display and `get_client_bounding_rect`, then moved `CHECKED` from `fx-banana` to `fx-cherry` through
`BaseDocument::snapshot_node_and` (each option's element state swapped, `mark_ancestors_dirty`, as `toggle_radio`
writes it), pumped and read again. `cargo test -p blitz-tests --locked --test zz_checked_probe -- --nocapture`:
exit 0, `1 passed; 0 failed`.

Readings, identical in both layout modes (`incremental` false and true):

| reading | at boot | after `CHECKED` moved to `fx-cherry` |
|---|---|---|
| `fx-pick` display · rect | `Display(259)` (inline-block) · 109.125, 8, **62 × 24** | `Display(259)` · 109.125, 8, **55 × 24** |
| `fx-apple` | `Display(0)` (none) · 0 × 0 | `Display(0)` · 0 × 0 |
| `fx-banana` | `Display(258)` (inline) · 112.125, 11, **55.171875 × 18** | `Display(0)` · 0 × 0 |
| `fx-cherry` | `Display(0)` · 0 × 0 | `Display(258)` · 112.125, 11, **48.015625 × 18** |
| `fx-named` | `Display(259)` · 175.125, 8, 51 × 24 | `Display(259)` · 168.125, 8, 51 × 24 |
| `fx-off` | `Display(259)` · 230.125, 8, 17 × 24 | `Display(259)` · 223.125, 8, 17 × 24 |

- A select reads a client rect at least as wide as its chosen label: 62 ≥ 55.17 (Banana), then 55 ≥ 48.02 (Cherry);
  its chosen option lies inside it, inset by border 1 + padding 2 (x 112.125 = 109.125 + 3, y 11 = 8 + 3).
- Moving `CHECKED` restyles: the old option goes back to `display: none`, the new one lays out, the select shrinks to
  the new label and the selects after it move left by the 7 px difference — in both modes.

The restyle follows the state, so the plan's premise holds and steps 4-16 build on it. Research's reading of the
untouched tree (`fx-pick` `Display(258)`, 0×0, rect `None`) is the before.
