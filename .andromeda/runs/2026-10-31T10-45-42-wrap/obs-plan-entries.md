## 2026-10-31-engine-debts — citations re-pointed past the chunk's added lines
**Section:** §9 CI Integration (ci-integration.md: Per-leg CI log) · §6 Log Coverage (log-coverage.md: the escher-driver `command` line, the blitz-dom groups, the `tests/dom.rs` absence row) · §4 Span / Trace Coverage (span-trace-coverage.md: the `command` span's absent list) · §2 Telemetry Strategy (telemetry-strategy.md: By invocation; body: the feature-gated log sites) · §1 Obs Scope Summary (body: blitz-dom events / util) · §3 Observability Harness Contract (body: the upstream focus checks, the visual lint) · §7 Error Capture & Reporting (body: the error classes, the labelled error sources) · §8 PII Scrubbing & Compliance (body: values logged as-is)
**Change:** each citation below points to the lines the cited code now stands at; claim text unchanged.
- §9: `.github/scripts/ci-leg.sh` and `.github/workflows/ci.yml`.
- §6 and §4: escher-driver `execute.rs`.
- §6, §2, §3, §7 and §8: blitz-dom `document.rs`.
- §6, §2 and §8: blitz-dom `mutator.rs`.
- §6, §1 and §8: blitz-dom `events/pointer.rs`.
- §6: blitz-vibey-script `tests/dom.rs`.
- §3: escher-driver `lint.rs`.
- §7: blitz-vibey-script `runtime/events.rs`.
**Why:** the chunk's edits moved the cited lines, among them `ci.yml`'s free-disk step and `runtime/events.rs` losing its `change` synthesis. The chunk adds no log site.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the chunk's surfaces in §6, no log site added, and the census's carve-out
**Section:** §6 Log Coverage (log-coverage.md: the examples emitter; blitz-dom (document / resolve / mutator / net); blitz-shell, blitz-paint)
**Change:**
- Examples: Cells' `cells-grid` Tab stop and keys and its `.cell.current` outline are instrumentation n/a.
- blitz-dom, one new `no site added` row: the transformed `visible_region` read, the non-invertible hit answer, the inert focus fixup, the tree's focus fallback, native `toggled`, the `multiple` select, `optgroup` and the engine's checkable `change` add no log site, print, env read, listener, wall-clock read or panic path on their added lines.
- blitz-dom, a second `no site added` row: the readers `BaseDocument::select_values`, `select_of` and `clear_select_typeahead`, and `dioxus_native_dom::closed_select_of` and `is_multiple_select`, are instrumentation n/a and add no log site or print.
- blitz-shell: `packages/blitz-shell/src/event.rs` holds the crate's unit test, an inline `#[cfg(test)]` module, held out of the added-lines census under `packages/`, so that census's 0 does not speak for it.
**Why:** the added-lines census reads 0 over the chunk's added lines, with `blitz-shell/src/event.rs` held out because its addition is an inline test, and the report reads each of these surfaces as instrumentation n/a or no log site added. The carve-out stands beside the crate's log sites so that no later reader takes the 0 to cover that test module.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `focussed node` re-read beside the focus fixup
**Section:** §6 Log Coverage (log-coverage.md: blitz-dom (document / resolve / mutator / net))
**Change:** the row reads: info, focussed node, once per focus move under the workspace build, where engine `tracing` is compiled in, holding a node number and no id or name; the focus fixup adds no log site, and neither `clear_focus` nor the flush prints (was `info: focussed node` alone).
**Why:** where the focused node reads inert, `DocumentMutator::flush` now calls `BaseDocument::clear_focus`, so the chunk re-read this line and reads its needles at 0. The fixup is an engine change on the founder's own word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the five new checks read silent
**Section:** §6 Log Coverage (log-coverage.md: absence · logs)
**Change:** one new `observed absent` row: a print, an env read, a subscriber or a listener in the five checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event`, searched by the silence census over those files, exit 1, reading 0.
**Why:** the chunk adds these five targets, and its silence census reads exit 1, last line 0.
**Kept:** §3's silence block gains no row for these checks; their silence stands in this §6 row, and the engine changes' added-lines reading in §6's `no site added` row.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — where a `multiple` select's set, an `optgroup`'s label and the engine's `change` leave
**Section:** §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: three new items)
**Change:**
- A `multiple` select's chosen set leaves only as form entries — on `input` and `change` one `(name, FormValue::Text(value))` per chosen option (`multiple_select_values`) — as the snapshot's `value`, which reads `select_value`, the first chosen option in tree order, and as the options' tree nodes, never as a log line or field.
- An `optgroup`'s non-empty `label` is its `Role::Group` name in the accessibility tree, inside the ratified name crossing, and it is not painted.
- The engine's `change` on a checkbox or radio, sent after `input` with the same value, a checkbox always and a radio only where it was not checked before the click, carries the value class `input` already carries; a page's script hears one `change`, from the engine, the script runtime synthesising none.
- Each item states that the added lines hold no log site, print or env read, the added-lines census reading 0.
**Why:** the chunk adds these values, and the report reads each one's exit and no log site on its added lines. The three are the founder's forks F6, F7 and F10, his word by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). The answer card read the same crossings in security-plan as lying inside those forks; the operator confirmed that by dialog at P2 Validate, 2026-10-31 13:13Z, as the operator's reading of the founder's fork texts, not a new word of the founder's (inputs#I16). An `optgroup` `label` as a name stands on the P5 lean, not on F7's text.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `stand_keyboard`'s stated reach in §3
**Section:** §3 Observability Harness Contract (body: the silent-checks passage)
**Change:** `stand_keyboard` drives the four lean stand tasks, the load page and Cells, and its own fixtures (was the four lean stand tasks and its own fixtures); the rest of the passage is unchanged.
**Why:** the chunk tests the `cells-grid` Tab stop and keys and the load page rows in `stand_keyboard`, its targets moving from 15 to 21, so the stated reach was stale.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the `fixtures` module's readers, 24 to 28
**Section:** §3 Observability Harness Contract (body: the third shared module's bullet)
**Change:** `tests/blitz-tests/tests/fixtures/mod.rs` has 28 readers, as measured at this chunk's census (was 24, as measured at 2026-10-20-large-screen-in-parts, per «2026-10-20-large-screen-in-parts — a part read's span and its silence»); the rest of the bullet stands.
**Why:** the chunk's census reads 28 readers.
**Kept:** §3's twenty-nine readers of `session_common` is left as written; the report reads 31 and unmoved by this chunk, so the older gap is not this chunk's move.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/
