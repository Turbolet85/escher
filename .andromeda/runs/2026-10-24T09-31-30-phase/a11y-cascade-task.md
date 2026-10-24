# Task — a11y · 2026-10-24-upstream-sync-polish-and-ship

## The chunk
Takes upstream/main (DioxusLabs/blitz, measured sha f9cbb2756) into the fork as a whole merge or a cherry-picked partial take, the pin shape decided at P4; no new escher feature is built.
Upstream's delta reaches blitz-dom engine code (packages/blitz-dom/src/accessibility.rs, node/node.rs hit testing, node/element.rs, document.rs, config.rs, mutator.rs, scrolling.rs, events/pointer.rs, the inline layout module) and may move Stylo to 0.23-pre, Taffy to a git rev and every Dioxus package to 0.8.0-alpha.3, re-syncing packages/dioxus-native-dom (dioxus_document.rs, events.rs, mutation_writer.rs, a new mutation_buffer.rs, keyboard_compat.rs deleted) and editing examples/seven_guis/src/tasks/cells.rs.
Fixes the red Dependency audit by a semver-compatible tokio update reached through accesskit_unix, accesskit_xplat and blitz-shell.
After the take every standing a11y proof must still pass unchanged in both layout modes; a moved layout, style or contrast number is re-measured and recorded, never loosened.

## Needs
- The tier-0 target SC 2.1.1, 1.4.3 and 2.4.3 as a re-proof: focus order, Enter and Space activation and contrast are re-read by their standing checks on the merged engine, not extended.
- The accessibility tree builder in packages/blitz-dom/src/accessibility.rs, which upstream's Stylo move touches: role derivation from the role attribute and the HTML element mapping, the four ARIA state reads (selected, pressed or checked by role, invalid, role switch), the merged name and description reads and expanded, disabled and level, re-proven by accessibility_roles, accessibility_states, accessibility_label, accessibility_description, accessibility_merged_states and accessibility_active_descendant_unread.
- Hidden-content exclusion (hidden, display none, visibility hidden, inert, aria-hidden kept marked) and the select option's exception from display none, re-proven by accessibility_hidden and inert_subtree, since style resolution may move under Stylo 0.23.
- Hit testing through inert and the clip stops, because upstream's dropped-node skip lands in hit_inner in node/node.rs; inert_subtree's pass-through reading must hold.
- The accessibility tree lifecycle and the engine's changed set: mutator.rs is a predicted merge conflict holding the changed-set insert, and the shell's refresh rebuilds the platform tree from a non-empty set; stand_accessibility_ids is the reading.
- Focusability and the disabled rule on form controls (presence on button, input, select, textarea alone; inert not focusable), reached through node/element.rs and document.rs, which upstream's class-atom and animation commits move; focusability_updates is the reading.
- Keyboard handling of the engine: Enter and Space activation, the range input's keys and pointer, the single-choice select's keys and typeahead on the document's clock, and the button and select click focus, in events/keyboard.rs, events/pointer.rs and node/select.rs, all inside the merge surface the chunk re-reads (CARRY 5 select model, CARRY 4 clock).
- The double-click window read on the document's clock, through the clock invariant re-read on config.rs, scrolling.rs and events/pointer.rs.
- The Dioxus bridge's falsy boolean-attribute removal (disabled, hidden, inert among them) on both attribute paths, which lives in the dioxus-native-dom attribute writing that the Dioxus 0.8 sync replaces upstream's side of.
- Keyboard event delivery into Dioxus documents, through upstream's re-synced events.rs and the deletion of keyboard_compat.rs where the take carries the Dioxus move.
- stand_keyboard's stated Tab orders (tab_order and ENABLED_TAB_ORDER), Enter and Space on each stand button and the CRUD list keyboard region crud-list named People, which must read unchanged on the merged engine.
- Stable element ids carried as AccessKit author_id through DioxusDocument's accessibility_tree override in dioxus_document.rs, and the snapshot model that takes role, name, focus and states from that tree; stand_element_ids, stand_id_persistence, stand_id_edits and the snapshot checks decide whether the re-sync moved an id.
- The actionable-key rule and unkeyed_actionable returning nothing on every stand task, reached through upstream's edit of the non-lean cells task; stand_actionable_keys and stand_non_lean_tasks are the readings.
- The password and file-input mask in the snapshot and in painted frames, which must survive the re-synced dioxus-native-dom and any blitz-paint move.
- Colour contrast: Harness::text_contrast, contrast_ratio and stand_contrast's control, TEXT_ROWS and STATE_ROWS tables with their exempt rule; a Stylo or Taffy move and the new text-indent and word-break keywords can move computed colours or layout, so each row is re-measured, never loosened; the visual lint's contrast reading follows the same rule.
- The stand's state cues stated by aria attributes alone (CRUD listbox options with aria-selected, flight booker mode buttons with aria-pressed, date inputs with aria-invalid and the error cue without colour), re-read on the merged stand.
- The accessibility feature wiring: the Dioxus crates taken with default features off and escher-driver naming accessibility unconditionally; a Dioxus 0.8 manifest move must keep that feature reaching the stand.
- The platform adapter path accesskit_xplat and accesskit_unix under blitz-shell, through the tokio advisory fix the audit needs; the update must not change the adapter's behaviour.
- The a11y CI leg's six files (accessibility_hidden, accessibility_roles, focusability_updates, stand_keyboard, stand_contrast, stand_accessibility_ids) and the stand-repeat leg's three stand files under the zero-failure budget, which must be green on the take's pushed commit.

## Not needed
- Focus-ring, target-size and typography tokens, motion and prefers-reduced-motion, colour scheme and zoom preferences: the chunk adds no stand styling and the plan records none of them as built.
- Language and direction, text alternatives in examples/assets fixtures and IME: the sync changes no fixture markup the plan tracks and adds no text input method.
- Cognitive orientation, timeouts, plain language and authentication: out of the tier-0 target and untouched by the take.
- Skip links, modal focus traps, route-change focus restoration and live regions: not built and not introduced by the sync.
- The windowed assistive-technology witness and the adapter build in a stand binary: owned by residuals for a later entry, not built here.
- HTML's focus fixup for a node made inert while focused, the native checkbox and radio toggled fix and aria-activedescendant support: residuals targeted elsewhere; the sync builds none of them.
- The A11y re-read entry's work: re-stating or extending the plan's criteria and tables belongs to a later Epoch 5 entry.
- New contrast rows, new keyboard tables or new a11y checks: the chunk re-proves existing ones only, adopting no new upstream API in escher code.
- The NOT YET MEASURED and observed-absent records (scope searches, assertion searches, ARIA searches): no measurement in this chunk resolves them.
- The a11y violation schema and the obs log format: the sink allowlist re-check is an observability reading, not an a11y one.
- Upstream-only surfaces (apps/browser, the WPT runner, mimalloc, flake.nix): outside escher's a11y surface.
