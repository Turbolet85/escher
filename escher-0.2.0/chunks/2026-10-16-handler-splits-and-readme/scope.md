# Scope — 2026-10-16-handler-splits-and-readme

**Working entry (escher-0.2.0/working-route.md:11, verbatim):** Handler splits and README — range and activation handler arms as named functions, `parse_float` in steps; README states 0.1.0 done, 0.2.0 planned, «Escher» capitalized (v020-01)

**Epoch:** Epoch 1 — Foundation · **Version:** escher-0.2.0 · **Requirement named:** v020-01 (Codebase structure)

## What this chunk builds

### A. The handler splits (blitz-dom, the version-end audit's M1)
- The escher-authored per-element default-action arms of blitz-dom's three over-ceiling event handlers leave them as named functions: the range-input arms and the keyboard-activation / button-focus arms of `handle_click` (`packages/blitz-dom/src/events/pointer.rs:640`, cognitive 74), `handle_pointerdown` (`events/pointer.rs:383`, 40) and `handle_key_or_input_event` (`events/keyboard.rs:20`, 56) — the audit's figures, `.andromeda/runs/2026-10-15T10-44-59-code-audit/proposals.md` §M1.
- `parse_float` (`packages/blitz-dom/src/node/range.rs:234`, cognitive 19) is split into its steps (the audit names sign · digits · exponent).
- The splits are behaviour-preserving refactors: no engine behaviour changes for any document — every range rule, every activation gate (the closed element/type matches and the `disabled` presence check), every event's order and count stay as the three engine changes ratified 2026-10-13 left them. A behaviour change found necessary is a fork for the founder, not a step of this chunk. (Verified: arch §Established Decisions [Scrolling and input], security-plan §Input Validation's range, keyboard-activation and `disabled` rows.)
- [premise-corrected: the merge-base measure shows upstream's own bodies at 69 · 35 · 50] "No longer over the complexity ceiling" applies to escher's branches, not to the three parents. Measured with the audit's instrument (`rust-code-analysis-cli` 0.0.25, function spaces, `cognitive.sum`), HEAD reproduces the audit (`handle_click` 74, `handle_pointerdown` 40, `handle_key_or_input_event` 56, `parse_float` 19), and the same handlers at the upstream merge base `434e0661` read 69 · 35 · 50. escher's arms add 5 · 5 · 6, so no split of escher's arms takes a parent under 15. The chunk's reading: each extracted function and each `parse_float` step at or under 15; each parent below its HEAD figure, with its upstream figure as its floor; `parse_float` itself at or under 15, so it leaves the over-ceiling set (`research.md` §Patterns detected).
- The upstream-merged entrants the audit lists beside them — `dispatch_hover_change`, `refresh_anonymous_block_styles`, `set_attribute` / `clear_attribute`, `construct_entry_list`, `propagate_damage_flags`, `compute_inline_layout_inner` — are left to upstream, per the audit's proposal ("leave the upstream-merged entrants to upstream, or name them for an upstream issue"). Upstream's arms inside the three handlers are left too. (Verified: proposals.md §M1; arch §Project Intent's upstream-sync line.)
- The existing checks that pin this behaviour stay green unchanged: `range_input_interaction`, `stand_keyboard`, `focusability_updates`, `stand_accessibility_ids`, the `node/range.rs` inline unit tests, the stand act checks (`stand_act_range`, `stand_act_keys`), both layout modes (`incremental_oracle` invariant), and the CI legs `fast`, `doc` and `a11y`. (Verified: each file present under `tests/blitz-tests/tests/`.)

### B. The README (the founder's own word)
- `README.md` states the current state: 0.1.0 done and verified, 0.2.0 planned — the founder's word, relayed (inputs#I2 §The README; the operator's directive inputs#I1).
- The project's name is written «Escher», capital E, in the README's prose, its title and headings included; crate, binary and path names (`escher-driver`, `escher-session`, `seven_guis`, …) stay lowercase as written (inputs#I1, inputs#I2). (Verified: the founder's «чтоб эшер там с большой буквы писался» names the README; the prose names the project at README.md lines 1, 7, 9, 17, 59, 65, 69, 73.)
- "0.1.0 done and verified" is stated from the recorded facts, each re-read: verified 17/17 (`matrix.py coverage --dir escher-0.1.0`: `verified 17/17`, done-test YES); the cold-agent baseline of 43 tool calls with 0 wrong over the four lean tasks (`escher-0.1.0/chunks/2026-10-15-cold-agent-test/report.md:263-264`); the version-end audit's 0 surviving mutants over 1,558 (proposals.md:73-87, six units: 1,285 caught, 0 missed, 6 timeouts, the rest unviable). "0.2.0 planned" comes from the route's five epochs (`escher-0.2.0/working-route.md` headers) and its eleven requirements.
- The README's present-tense claims that the 0.1.0 description now gets wrong are rewritten to the state: "… are being built (see Plans)" (README.md:9), "Version 0.1.0 is built in six stages" (:52) and "Beyond 0.1.0: …" (:61), which names items 0.2.0 now holds (visual lint, virtual time, turning traces into tests). The Try-it commands, licence and author sections are kept, since no fact in them is wrong. (Verified: README.md read whole.)
- The capitalisation is the README's prose only. CLAUDE.md, `.andromeda/*` and crate docs are not re-spelled by this chunk. The repository's GitHub description reads "An agent-first UI framework built on Blitz" and names no project, so nothing outside the README needs the capital (verified: `gh repo view Turbolet85/escher --json description`).

## Boundaries
- No mutation run: no `cargo mutants` (or the audit's `mutate.py`) runs in this chunk without the founder's new word (inputs#I1). The six timeout-only mutants are the later entry "Fast-failing mutants".
- `document.rs`'s inline test modules and the `mcp.rs` / `schema.rs` splits are the next entry, "File splits" — not this chunk.
- v020-01's acceptance has legs in both this entry and "File splits" (working-route.md:13 names v020-01 too), so this chunk advances v020-01 without fully verifying it. (Verified: `matrix.py show --id v020-01` acceptance names the handler branches, `document.rs`'s test modules and the two driver files.)
- Every deferral this chunk makes gets a `.andromeda/residuals.md` line naming the founder's label (inputs#I1); his forks are raised as dialogs.
- No new crate, listener, port, env var, carrier, verb, field, log field or span.

## CI since the last wrap (Setup 5a, `ci.py conclusion`)
- `d236e9186e4f` green · checks 17/17 · wall 774 s · CI#38865273992
- `1a34eb2bbebd` green · checks 17/17 · wall 1548 s · CI#38863184197
- `af1a00ad5b34` green · checks 17/17 · wall 920 s · CI#38719947381

## Freight
- None: `route.py pins` reads 0 freight blocks on the markerless tail.
