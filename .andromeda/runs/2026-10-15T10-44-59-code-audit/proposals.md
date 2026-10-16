# Code Audit — escher · Epoch 6 — Polish & ship · 2026-10-16T04:32:16Z
mode trend · HEAD af1a00ad5 · baseline 495067f8a (Epoch 5 — Agent surfaces) · span 1

Overshoot: this run 0 commits (HEAD is the boundary, the 2026-10-15-cold-agent-test complete flip) · the baseline's 0 commits (at the boundary: HEAD is the 2026-10-12-headless-screenshot complete flip). No ancestry break; no trend-break (every tool token equals the baseline's).

Scope of the mutation tier (the founder's answers, relayed verbatim by the overseer, record `~/dev/projects/additional/escher-overseer/relays/v010-end-founder-word.md`): escher's five units whole («Давай полностью, можно в 4 потока я не за компом»), blitz-dom on range.rs + keyboard.rs («range.rs + keyboard.rs (Рекомендую)»), the upstream-only units not run («Не запускать (Рекомендую)»), `-j 4`.

## Proposals

### M1 — monotonic · `complexity.over_ceiling` — worse at both of the last two diffs
**Movement:** 74 → 80 → 83 (Epoch 4 — Driver core → Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship)
**Evidence:** functions over cognitive 15 at the baseline 80, at HEAD 83. Entered (4):

| file | function | line · cognitive |
|---|---|---|
| packages/blitz-dom/src/events/driver.rs | `dispatch_hover_change` | 76 · 16 |
| packages/blitz-dom/src/layout/damage.rs | `refresh_anonymous_block_styles` | 198 · 17 |
| packages/blitz-dom/src/node/range.rs | `parse_float` | 234 · 19 |
| tests/blitz-tests/tests/stand_accessibility_ids.rs | `every_interactive_role_element_reads_a_name` | 256 · 16 |

Left (1): `handle_pointer_move` packages/blitz-dom/src/events/driver.rs:47 (17). Standing over-ceiling functions whose value moved (8):

| file | function | cognitive baseline → HEAD |
|---|---|---|
| packages/blitz-dom/src/events/keyboard.rs | `handle_key_or_input_event` | 50 → 56 |
| packages/blitz-dom/src/events/pointer.rs | `handle_click` | 69 → 74 |
| packages/blitz-dom/src/events/pointer.rs | `handle_pointerdown` | 35 → 40 |
| packages/blitz-dom/src/form.rs | `construct_entry_list` | 37 → 38 |
| packages/blitz-dom/src/layout/damage.rs | `propagate_damage_flags` | 19 → 21 |
| packages/blitz-dom/src/layout/inline.rs | `compute_inline_layout_inner` | 129 → 131 |
| packages/blitz-dom/src/mutator.rs | `clear_attribute` | 25 → 28 |
| packages/blitz-dom/src/mutator.rs | `set_attribute` | 36 → 38 |

**Suspected shape:** the growth sits in blitz-dom's event handlers — `handle_click` 69 → 74, `handle_pointerdown` 35 → 40, `handle_key_or_input_event` 50 → 56 carry this epoch's keyboard-activation, button-focus and range default actions (direct engine changes ratified 2026-10-13); the range model's `parse_float` (19) is new — `events/pointer.rs`, `events/keyboard.rs` and `node/range.rs` were touched by escher's chunks alone this epoch (2026-10-13-range-input-interaction, 2026-10-13-stand-keyboard-harness); `dispatch_hover_change` and `refresh_anonymous_block_styles` came with the upstream merge (upstream #1176, #1174), and `set_attribute` / `clear_attribute` moved with both (upstream #1187 and the range re-seed). The series' 74 → 80 step at Epoch 5 is the earlier half.
**Proposal:** for the escher-authored handlers, extract the per-element default-action arms out of `handle_click` / `handle_pointerdown` / `handle_key_or_input_event` into named functions (the range and activation arms are self-contained), and split `parse_float` into its sign/digits/exponent steps; leave the upstream-merged entrants to upstream, or name them for an upstream issue.

### M2 — monotonic · `sizes.file_max` — worse at both of the last two diffs
**Movement:** 2659 → 2756 → 2793 (Epoch 4 — Driver core → Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship)
**Evidence:** the max file is `packages/blitz-dom/src/document.rs` at all three records — 2659 (Epoch 4) → 2756 → 2793 code lines (tokei). Of its 2793, 688 are its five inline `#[cfg(test)]` modules from line 2897 (`zoom_tests` · `hover_state_tests` · `hover_invalidation_tests` · `font_face_override_tests` · `changed_set_tests`; the first 2896 lines read 2105). This epoch's +104/−58 raw lines in it (`git diff --numstat 495067f8a..HEAD`): the upstream merge +88/−41 (upstream #1189 shared UA stylesheets, #1177 sub-document cursor) and escher's 2026-10-14-quality-gates edit to `hover_state_tests` +16/−17.
**Suspected shape:** the file carries upstream's `BaseDocument` body and, inline, a test volume a quarter its size, two of the five modules escher's (`changed_set_tests`, the `hover_state_tests` edits); this epoch's growth there is upstream's, not an escher feature.
**Proposal:** move escher's inline test modules out of `document.rs` into a sibling test file (`document/tests.rs` or a `blitz-tests` target) — the escher-side lever on this metric that keeps upstream's body untouched, so later upstream syncs stay mostly additive; whether upstream's own body is split is upstream's call.

### M3 — monotonic · `sizes.over_800` — worse at both of the last two diffs
**Movement:** 12 → 15 → 16 (Epoch 4 — Driver core → Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship)
**Evidence:** files over 800 code lines: 12 (Epoch 4) → 15 → 16. Every row at HEAD with its baseline value:

| file | baseline | HEAD |
|---|---|---|
| packages/blitz-dom/src/document.rs | 2756 | 2793 |
| packages/blitz-vibey-script/src/runtime.rs | 2209 | 2209 |
| packages/blitz-dom/src/node/node.rs | 1537 | 1561 |
| packages/blitz-dom/src/mutator.rs | 1500 | 1517 |
| packages/blitz-vibey-script/src/dom/element.rs | 1263 | 1263 |
| packages/escher-driver/src/mcp.rs | 1140 | 1140 |
| packages/blitz-dom/src/stylo.rs | 1126 | 1126 |
| packages/blitz-paint/src/render.rs | 1072 | 1125 |
| packages/blitz-dom/src/layout/construct.rs | 1057 | 1072 |
| packages/stylo_taffy/src/convert.rs | 909 | 917 |
| packages/blitz-dom/src/layout/inline.rs | 910 | 916 |
| packages/blitz-dom/src/node/element.rs | 854 | 910 |
| packages/blitz-dom/src/layout/text_transform.rs | 845 | 845 |
| packages/escher-driver/src/schema.rs | 840 | 840 |
| packages/blitz-dom/src/cssom.rs | 827 | 827 |
| packages/stylo_taffy/src/wrapper.rs | — (entered) | 819 |

**Suspected shape:** the one entrant, `packages/stylo_taffy/src/wrapper.rs` (819), grew with the upstream merge, upstream #1175 (UA flex centering, +32/−5); of escher's own files only `packages/escher-driver/src/mcp.rs` (1140) and `schema.rs` (840) stand over the line, both unchanged this epoch.
**Proposal:** split `escher-driver/src/mcp.rs` along its existing seams (protocol framing and bounds · tool listing · tool-result writing) and `schema.rs`'s verb table from its help texts; the upstream-owned rows are upstream's to split.

## Informational

### Mutation tier (host x86_64-unknown-linux-gnu; score = caught/(caught+missed))

| unit | scope | state | caught | missed | not measured | timeout | unviable | score | wall (-j 4) |
|---|---|---|---|---|---|---|---|---|---|
| `escher-telemetry` | unit | complete 33/33 | 28 | 0 | 0 | 0 | 5 | 100 | 1391 s |
| `blitz-test-harness` | unit | complete 228/228 | 200 | 0 | 0 | 0 | 28 | 100 | 9130 s |
| `seven_guis` | unit | complete 239/239 | 207 | 0 | 0 | 3 | 29 | 100 | 9676 s |
| `dioxus-native-dom` | unit | complete 403/403 | 282 | 0 | 0 | 1 | 120 | 100 | 13131 s |
| `escher-driver` | unit | complete 525/525 | 444 | 0 | 0 | 2 | 79 | 100 | 20424 s |
| `blitz-dom/range-keyboard` | files (packages/blitz-dom/src/events/keyboard.rs, packages/blitz-dom/src/node/range.rs) | complete 130/130 | 124 | 0 | 0 | 0 | 6 | 100 | 5857 s |

- `blitz-test-harness` not comparable — 100.00 (scope unit, Epoch 6 — Polish & ship) vs 100.00 (scope files packages/blitz-test-harness/src/settle.rs, Epoch 4 — Driver core); no delta
- `seven_guis` 100.00 (scope unit, Epoch 6 — Polish & ship) — no prior score: the first scored value
- `dioxus-native-dom` not comparable — 100.00 (scope unit, Epoch 6 — Polish & ship) vs 100.00 (scope files packages/dioxus-native-dom/src/dioxus_document.rs, packages/dioxus-native-dom/src/element_id.rs, Epoch 4 — Driver core); no delta
- `blitz-dom/range-keyboard` 100.00 (scope files, Epoch 6 — Polish & ship) — no prior score: the first scored value

Survivors — 0, complete (each a mutant that built and passed every test in its unit's set):

| site | mutation |
|---|---|

Not measured on this host (x86_64-unknown-linux-gnu) — 0.

Timeouts — 6 (scored neither caught nor missed; each a mutant whose test run exceeded the 300 s per-mutant timeout):

| unit | site | mutation |
|---|---|---|
| `dioxus-native-dom` | `packages/dioxus-native-dom/src/mutation_writer.rs:172:9` | `replace <impl WriteMutations for MutationWriter<'_>>::create_text_node with ()` |
| `escher-driver` | `packages/escher-driver/src/mcp.rs:154:53` | `replace + with * in read_request` |
| `escher-driver` | `packages/escher-driver/src/mcp.rs:154:53` | `replace + with - in read_request` |
| `seven_guis` | `examples/seven_guis/src/tasks/timer.rs:29:9` | `replace TimerTicks::next with ()` |
| `seven_guis` | `examples/seven_guis/src/tasks/timer.rs:32:31` | `replace -= with += in TimerTicks::next` |
| `seven_guis` | `examples/seven_guis/src/tasks/timer.rs:32:31` | `replace -= with /= in TimerTicks::next` |

Union verdict: no project union verdict (no `.andromeda/test-plan.md` line names cargo-mutants).

### Other movements

- count-under-ratio (not fired: `pct` rose with the counts) — clones 340 → 371 (+31), duplicated lines 3870 → 4167 (+297), `pct` 3.646 → 3.686, population `total_lines` 106157 → 113058 (+6.5 %); split src 178/1847 L · test 185/2219 L · mixed 8/101 L (baseline src 168/1761 · test 163/1979 · mixed 9/130); the top standing pair `tests/blitz-tests/tests/device_coalescing.rs` ↔ `tests/blitz-tests/tests/resize_restyle.rs` (43 L) stands since before this epoch.
- complexity top-10 entrant: handle_key_or_input_event · packages/blitz-dom/src/events/keyboard.rs:20 · 56
- sizes top-10 entrant: packages/stylo_taffy/src/convert.rs · 917
- duplication top-10 entrant: examples/seven_guis/tests/host_log.rs · examples/seven_guis/tests/mcp_log.rs · 29
- hotspots top-10 entrant: packages/blitz-dom/src/events/pointer.rs · 154
- hotspots top-10 entrant: packages/blitz-dom/src/events/keyboard.rs · 112
- hotspots top-10 entrant: packages/blitz-paint/src/render.rs · 60
- hotspots top-10 entrant: packages/dioxus-native-dom/src/mutation_writer.rs · 57
- fan-in top-20 entrant: blitz-dom node/node/impl#[Node]element_data(). · 128
- fan-in top-20 entrant: blitz-dom config/impl#[DocumentConfig][Default]default(). · 127
- unused-dependency entrant (cargo-machete): blitz-paint → usvg

## Below threshold — no action

- `duplication.pct` 3.724 → 3.646 → 3.686 (Epoch 4 — Driver core → Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship): not worse at both diffs
- `dead.zero_ref_candidates` 105 → 122 → 119 (Epoch 4 — Driver core → Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship): not worse at both diffs
- `coverage.line` 59.21 → 65.08 → 67.07 (Epoch 4 — Driver core → Epoch 5 — Agent surfaces → Epoch 6 — Polish & ship): not worse at both diffs
- duplication-up — duplication.pct 3.646 → 3.686 (+0.040 pt, +1.1 %)
- complexity-creep — complexity.over_ceiling 80 → 83 (+3, +3.8 %; fires at +3 AND +25 %)
- dead-growth — dead.zero_ref_candidates 122 → 119 (-3)
- coverage-drop — coverage.line 65.08 → 67.07 (+1.99 pt; a rise is never celebrated)
- new-cycle — graph.cycles 0 → 0
- mutation-drop — `escher-telemetry` 100.00 → 100.00 (scope unit both; Epoch 4 — Driver core → Epoch 6 — Polish & ship; fires at −10 pt)
- mutation-drop — `escher-driver` 100.00 → 100.00 (scope unit both; Epoch 4 — Driver core → Epoch 6 — Polish & ship; fires at −10 pt)
- sizes file_p50 131 → 137 · file_p90 593 → 605
- complexity cyclomatic p50/p90 1/5 → 1/5 · cognitive p50/p90 0/3 → 0/3 · max `synthesize_presentational_hints_for_legacy_attributes` 135 → 135
- graph cross-unit edges 96 → 97 (population, not a movement) · cycles 0 at both ends
- churn 23.21 % → 22.59 % · files churned 43 → 29
- dead FP classes at HEAD: candidate 119, derive/attr-invoked 96, entry-point 52, format-capture 4, runtime-invoked (macro-generated binding) 20, test-only 61, trait-impl (dispatch) 561 (rows 1988, test-path excluded 1075)
- totals (population) loc 86414 → 92380 · files 371 → 385 · units 29 → 29

## Skips

- mutation:accesskit_xplat — declined
- mutation:blitz — declined
- mutation:blitz-examples — declined
- mutation:blitz-html — declined
- mutation:blitz-net — declined
- mutation:blitz-paint — declined
- mutation:blitz-shell — declined
- mutation:blitz-tests — declined
- mutation:blitz-traits — declined
- mutation:blitz-vibey-script — declined
- mutation:browser — declined
- mutation:browser-persistence — declined
- mutation:bump — declined
- mutation:counter — declined
- mutation:debug_timer — declined
- mutation:dioxus-native — declined
- mutation:rdme — declined
- mutation:stylo_taffy — declined
- mutation:todomvc — declined
- mutation:transparent — declined
- mutation:wasm_hello — declined
- mutation:wgpu_texture — declined
- mutation:wpt — declined
- mutation:blitz-dom (the whole unit beyond range.rs + keyboard.rs: 5477 listed; its 15 other touched files 2657) — declined
