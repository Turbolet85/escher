# Report — 2026-10-10-snapshot-states

**Chunk:** Snapshot states — a snapshot reads which control is selected, pressed or invalid, and the stand's tasks state all three
**Date:** 2026-10-10T17:50Z
**Commits:** `2fc32829` chore(2026-10-10-snapshot-states): operator pre-CI commit, for the run this chunk verdict reads — the one commit of the operator pass (`git log --format='%h %s' f0cb9dd7..HEAD`, 1 line). The chunk's base is its parent, `f0cb9dd7` (the previous wrap's commit).

Every coordinate of added text below is a range of this report's last section, `## New text, by line`. A count
carries the command that produced it. "Base" is `f0cb9dd7`.

## Changes (structured — detectors read this)

- **Files:** 13 source files — `gate.py scope`: `changed 13 · listed 13 · recorded 0`.
  - Engine: `packages/blitz-dom/src/accessibility.rs` (+31 −1, `git diff --numstat f0cb9dd7`).
  - Snapshot: `packages/dioxus-native-dom/src/snapshot.rs`, `snapshot_text.rs`, `snapshot_diff.rs` (the last: one unit test, no code line).
  - Driver: `packages/escher-driver/src/schema.rs`, `json.rs`, `lib.rs` (crate docs only).
  - Stand: `examples/seven_guis/src/tasks/flight_booker.rs` (+4 −0), `examples/seven_guis/src/tasks/crud.rs` (+3 −0) — added attribute lines only.
  - Checks: new `tests/blitz-tests/tests/accessibility_states.rs` (220 lines) and `tests/blitz-tests/tests/stand_act_states.rs` (158 lines); extended `tests/blitz-tests/tests/stand_snapshot_state.rs` and `tests/blitz-tests/tests/stand_act_diff.rs`.
  - Ledger and chunk folder: `escher-0.1.0/verification-matrix.json` (v010-17 `planned → implemented`); `chunks/2026-10-10-snapshot-states/evidence/{controls,readings,operator-pass}.md`. No `scope-record.md` exists.
- **Symbols / APIs:**
  - **The accessibility builder reads three state attributes** — `BaseDocument::build_accessibility_node` (private; one caller, `build_accessibility_tree`, per research.md §Graph impact), `packages/blitz-dom/src/accessibility.rs:102-129`, the import at `:2-4`. On every element, whatever its role, it reads the attribute's value trimmed of ASCII whitespace and compared exactly, in lower case:
    - `aria-selected`: `true` → AccessKit `selected` true, `false` → false, anything else → nothing;
    - `aria-pressed`: `true` · `false` · `mixed` → `toggled` `True` · `False` · `Mixed`, anything else → nothing;
    - `aria-invalid`: `true` · `grammar` · `spelling` → `invalid` `True` · `Grammar` · `Spelling`; absent, empty (after trimming) or `false` → nothing; **any other value → `Invalid::True`** (WAI-ARIA 1.2's rule, quoted in the comment).
    - `aria-checked` is not read; nothing else in the file writes `toggled`. No role is consulted. The added lines hold no log site and no panic path.
    - What it reaches: the output of the public `BaseDocument::build_accessibility_tree` changes for every document — its three production callers and the three production readers of `accessibility_tree` are unchanged in code (research.md §Graph impact): the shell's `update_tree` and `handle_blitz_shell_event` (the platform adapter's feed), `unkeyed_actionable` (reads roles only) and `snapshot`.
  - **`dioxus_native_dom::NodeState` gains three public fields** — `selected: Option<bool>`, `pressed: Option<bool>`, `invalid: bool` (`packages/dioxus-native-dom/src/snapshot.rs:59-69`), read from the accessibility node at the one construction site (`:159-161`): `is_selected()`, `toggled()` with only `Toggled::True` reading `Some(true)` (`False` and `Mixed` read `Some(false)`), `invalid().is_some()`. Seven fields now, in the order `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused`. Still `Debug`, `Clone`, `PartialEq`, `Default`; no new public type; the crate root's re-export line is unedited. A struct literal without a spread no longer compiles: `NodeState {` has 18 sites, all inside the crate (`grep -rn -c 'NodeState {' --include=*.rs packages tests examples`: `snapshot.rs` 2, `snapshot_text.rs` 16); outside the crate the type is named by one file, `stand_act_states.rs`, as a reference type.
  - **`Snapshot::to_text` writes three more state tokens** — `selected` | `unselected`, `pressed` | `unpressed` (each only where the model holds a reading), `invalid` (only when true), after `value=` and before `focused` (`packages/dioxus-native-dom/src/snapshot_text.rs:71-75`, `:76-80`, `:81-83`; the rustdoc restated at `:26-31`, `:34`, `:38-39`). Still a pure function of the snapshot.
  - **`Snapshot::diff`** — no code line changed; a node whose `selected`, `pressed` or `invalid` alone moved is named `changed` by the existing whole-state comparison.
  - **`escher_driver::NODE_FIELDS`** — `[&str; 9]` → `[&str; 12]`: `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused` · `bounds` (`packages/escher-driver/src/schema.rs:32-37`). The constant's type changed with its length. Remaining readers: `json.rs`'s unit test and the crate root's re-export; no file under `tests/` or `examples/` names it (`grep -rn NODE_FIELDS`, 0 hits outside the crate).
  - **Two help texts of `VERBS` restated** — `snapshot`'s `text` (`schema.rs:232-236`) and the acting verbs' `changed` (`:203-205`) now say the state words and the two limits of the boolean model (a mixed toggle reads unpressed / `pressed` false; a grammar or spelling error reads invalid). Each matches the plan's wording exactly at both its table row and its pin (`evidence/controls.md` §3: 332 and 290 characters, 2 literals each).
  - **`Outcome::to_json`, a node's written form** — `selected` and `pressed` written only where present, `invalid` always written, in the table's order (`packages/escher-driver/src/json.rs:102-104`, `:129-135`, `:174-176`). Optional keys are now `parent`, `enabled`, `checked`, `value`, `selected`, `pressed`; always written: `id`, `role`, `name`, `invalid`, `focused`, `bounds`.
  - **Crate docs** — one sentence on the three states and the two limits (`packages/escher-driver/src/lib.rs:53-56`).
  - **Not added:** no verb, argument kind, field kind, refusal cause or `SessionError`; no socket, port, env var, crate or feature. The wire stays `v2` and `wire.rs`, `host.rs`, `client.rs`, `execute.rs`, `refusal.rs`, `session.rs`, `command.rs`, `cli.rs` are unedited (the plan's preservation entry, `git diff --quiet f0cb9dd7 -- …`, green). What a `v2` answer carries grew inside two existing carriers: a diff node up to three more keys, a snapshot's text up to five more words.
  - **Stand markup, attributes only** — `flight-one-way` and `flight-return` state `aria-pressed` `"true"` | `"false"` (`examples/seven_guis/src/tasks/flight_booker.rs:69`, `:76`); `flight-start` and `flight-return-date` state `aria-invalid` (`:85`, `:93`); `crud-list` takes `role="listbox"` (`examples/seven_guis/src/tasks/crud.rs:58`); each row takes `role="option"` and `aria-selected` (`:71-72`). No element, class, style, key, id or order changed; the three classes that style the states stay.
- **Crates / modules:** none added or removed. Changed: blitz-dom (one file), dioxus-native-dom (three), escher-driver (three), seven_guis (two task files), blitz-tests (two new test targets, auto-discovered; no manifest line).
- **Dependencies:** none added or bumped — `Cargo.toml`, `Cargo.lock`, `deny.toml` and the five crate manifests are unchanged (the same preservation entry). AccessKit's `Invalid` and `Toggled` are newly named by blitz-dom, and `Toggled` by dioxus-native-dom and `stand_snapshot_state`; the crate was already a dependency of each.
- **Schema / config:** the diff node's JSON shape (twelve keys, above) and the snapshot text's token grammar (above). No config key, no scrub or redaction shape: the password and file mask is untouched and its two checks pass unedited in what they assert of it.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**

  | What | Base → now | Basis | Master sites that state it |
  |---|---|---|---|
  | `NodeState` fields | 4 → 7 | the struct, above | architecture:136 (names the four) |
  | `NODE_FIELDS` keys · keys left out when absent | 9 → 12 · 4 → 6 | `schema.rs`, `json.rs`, above | architecture:133 (lists the nine; "`parent`, `enabled`, `checked` and `value` left out when absent") |
  | Text at boot, flight booker · CRUD, both layout modes | 1269 → 1287 · 2034 → 2049 B (counter 755 and timer 1259 unchanged) | `evidence/readings.md` §Sizes — read off `every_screen_fits_the_budget` under a restored one-shot mutation | architecture:136 (`755 · … 1269 · … 1259 · … 2034`); test-plan:26 ("the largest, Crud, reads 2034") |
  | Hosted `snapshot` line, four tasks | 832 · 1,375 · 1,368 · 2,187 → 832 · 1,393 · 1,368 · 2,202 B | `evidence/readings.md` §Sizes — the binary run per task | architecture:136; test-plan:200 |
  | `run stand` | 119 passed · 6 ignored · 31 files → 125 · 6 · 32, 0 failed | the gate entry's `run.end`, `evidence/readings.md` | test-plan:116 |
  | Workspace test leg | 165 result lines · 776 passed → 167 · 791; 0 failed, 11 ignored both | `target/ci-logs/test.log` of implement's run; re-read at the operator entry's `fast` leg | test-plan:329 |
  | dioxus-native-dom unit tests | 55 → 60 (`snapshot.rs` 14 → 17, `snapshot_text.rs` 7 → 8, `snapshot_diff.rs` 8 → 9) | `cargo test -p dioxus-native-dom --lib`: 60 passed; `grep -c '#\[test\]'` per file at base and now | test-plan:24 ("55 unit tests in eight files") |
  | escher-driver unit tests | 50 → 50 (two restated in `schema.rs`, one in `json.rs`) | `cargo test -p escher-driver --lib`: 50 passed | test-plan:21 — unchanged |
  | Driver-action checks `stand_act_*` | 11 → 12 files | `ls` · `git ls-tree f0cb9dd7` | architecture:115, :263; test-plan:66, :100, :101; obs-plan:70 (each says "eleven"); the test-plan key file `.andromeda/registries/contracts/test-plan/session-lifecycle.md:11` ("the eleven driver-action files", each listed with its test count) |
  | Readers of `mod session_common;` | 16 → 17 | `grep -l '^mod session_common;'` now · `git grep` at base | architecture:263 ("sixteen readers"); test-plan:66, :101 ("16") |
  | Readers of `mod common;` | 17 → 17 | the same two greps | architecture:263, test-plan:66, :100 — unchanged (see Spec claims disproved, item 3) |
  | `.rs` files directly under `tests/blitz-tests/tests/` | 106 → 108 (`all.rs` among them, never built) | `git ls-tree` at base · `ls` | test-plan:66 states an executable count ("no `all` executable among 107") — not re-measured here |
  | `stand_snapshot_state` tests · `stand_act_diff` tests | 8 → 11 · 6 → 6 | `grep -c '#\[test\]'` | — |
  | `a11y` CI leg | 6 + 6 + 3 over three files, unchanged | its log | — |
  | `accessibility.rs` against the local ref `upstream/main` (`7832c177`) | +26 → +57 added, −0 → −1 | `git diff --numstat upstream/main f0cb9dd7` and `git diff --numstat upstream/main`; the ref was not fetched | architecture:87 names the upstreamable flag for the two earlier engine changes |

- **Dev-tool versions:** none — no host tool was installed, upgraded or read changed.
- **Harness / gate surface:** none changed. `scripts/agent-run.sh` is unedited and picks `stand_act_states` up by its `stand_` prefix; the `a11y` CI leg stays three named files — `accessibility_states` rides the workspace test leg (scope.md decision 7); no CI file, script or status shape moved.
- **Cross-project / external claims:**
  - **Fork CI on the pushed sha** — `Turbolet85/escher`, CI#38072258834, on `2fc3282965443aac2b7ff4d70729031ff1b0f945`: `verdict: green · checks 16/16 · wall 767 s` (`evidence/operator-pass.md`, entry 20). It is the one witness of the MSRV build and the windows, macos, ios and android legs. The wrap's own commit will add to that tree.
  - **Upstream** — DioxusLabs/blitz `main` read as the local ref `upstream/main` = `7832c177`, not fetched. No upstream pull request was opened (CARRY 3).
  - **WAI-ARIA 1.2** — the three anchors the engine's comments and the new check's doc cite (`#aria-selected`, `#aria-pressed`, `#aria-invalid`) come from research's P3 download; neither implement nor this wrap fetched them. The page states no case rule for a state's value: lower-case exact match is a choice, so `TRUE` reads unrecognized — nothing for the first two attributes, invalid for the third. No browser was measured.
  - **AccessKit 0.25.1** — the setters and the two enums were read in the local cargo registry copy of the locked version.
  - **The founder's word on the two boundary forks** (`.andromeda/runs/2026-10-10T16-12-10-phase/p4-founder-forks.md`) — not in when this report was authored: no relay had reached that session, and both stood as the operator's reading (`inputs#I4`). **In at this wrap's resume** (`inputs#I6`): the founder answered both forks by question dialog in the overseer session on 2026-10-10, relayed by the overseer — fork 1 «Внутри ратифицированного» = option A, inside the ratified crossing; fork 2 «Ратифицировать» = option A, ratify: the builder writes the three states on the node. The labels are the overseer's rendering of the builder's option names; he added no words of his own. The relay carries fork 1's stated price as a rule of his from now on — a new field of a closed value set inside an already ratified carrier is not a widening and does not come to him — and fork 2's: what an assistive technology reads stays unmeasured until the windowed check.
  - **Inputs** (`inputs.py verify`, at this wrap's first firing: `5 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 4 · uncited 0 · unparsed 0`; at its resume, after the two snaps: `inputs: 7 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 5 · uncited 0 · unparsed 0`):
    - I1 · message: the operator, as the argument of /andromeda-phase · copy message · n/a — a message has no live source
    - I2 · `../additional/escher-overseer/relays/snapshot-states-route-adaptation.md` · copy no-repo · unchanged
    - I3 · message: the operator, phase P4 first round · copy message · n/a
    - I4 · message: the operator, phase P4 second round · copy message · n/a
    - I5 · message: the operator, phase P5 review · copy message · n/a
    - I6 · `../additional/escher-overseer/relays/snapshot-states-founder-answers.md` · copy no-repo · unchanged — snapped at this wrap's resume (`inputs#I6`): the founder's answers to the two boundary forks, as relayed by the overseer. The record of Expected amendments 5 and 6 rests on it; no other fact of the Changes above does.
    - I7 · message: the operator, as the arguments of /andromeda-wrap-session, 2026-10-10 · copy message · n/a — snapped at this wrap's resume (`inputs#I7`): the resume word, the halt owed at Validate, and the seven route items sent to the route-resolve.
    - At its first firing this wrap read no external input of its own and snapped none.
- **Reverted / negative API facts:** none — nothing was written and then removed. The 22 mutation controls and the one sizing mutation each changed a file and restored it by bytes, sha256 equal.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **plan.md Implementation notes / research.md §Sweep records** — "the eleven [`NodeState` literals] in `snapshot_text.rs` close with `..state()` or `..NodeState::default()` and compile unchanged". Measured: one of them, the last literal of `state_tokens_follow_the_model`, was a full literal and failed to compile on the new fields (`E0063: missing fields`). It sits in the test step 6 restates anyway; no cost, no master states the claim.
  2. **plan.md step 11 and the note above the gate block** — "Step 11's readings are … taken from the listed entries' logs". Measured: the two `host_log` entries' logs hold only the test's `ok` line, and no green entry prints a text size. The by-level figures came from the instrument the driver-cli chunk committed; the sizes from a restored mutation. A plan claim, no master's.
  3. **obs-plan:70** — the stand checks, "15 of which read the shared module `tests/blitz-tests/tests/common/mod.rs`". Measured: 17 `stand_*.rs` files declare `mod common;`, at the base and now (`git grep -l '^mod common;' f0cb9dd7 -- 'tests/blitz-tests/tests/stand_*.rs'`, and the same `grep` on the tree). This chunk added no reader of it; the count predates it. architecture:263 and test-plan:66 and :100 say 17.
  4. **research.md §Files inspected** — `snapshot.rs` "13 unit tests". Measured at the base: 14 `#[test]` lines. No master states a per-file count for it.
  - Held as predicted, not disproved: the two text sizes (1287, 2049), `run stand` at 32 files, the workspace at 167 result lines.
- **Expected amendments (from plan):** site searches ran over the seven masters and the 7 files under `.andromeda/registries/` (one script, a line count per pattern per file; the registries read again for the count words `eleven` · `sixteen` · `16 readers` · `installs no subscriber` · `31 files` · `165 result` · `776` · `55 unit`). One registry file holds a hit: the test-plan key file `contracts/test-plan/session-lifecycle.md`, named at item 9.
  1. architecture §Standard Contracts → Dioxus DOM bridge (`NodeState` at seven fields · the token order · the text sizes) — **carried**: Symbols / APIs, Counts. Sites: `NodeState` architecture 1 line (:136, three occurrences); `1,?269` architecture:136; `` `focused` ``|`state tokens` architecture :130, :133, :136.
  2. architecture §Standard Contracts → Driver session (`NODE_FIELDS` at twelve · `invalid` always written · the two help texts) — **carried**: Symbols / APIs, Counts. Sites: `NODE_FIELDS` architecture 2 lines (:133, :136); `\b832\b` architecture:136.
  3. architecture §Standard Contracts → Mutation, query and CSSOM (the builder reads three state attributes by value) — **carried**: Symbols / APIs. Sites: `aria-label` architecture:122 (the builder's name sources); `aria-(selected|pressed|invalid|checked)` 0 hits in the seven masters and the registries — the fact is new to every master.
  4. architecture §Established Decisions → Scrolling and input (a third direct engine change, flagged upstreamable against `7832c177`) — **carried**: Symbols / APIs, Counts (last row), Cross-project. Sites: `upstreamable` architecture:87; `7832c177` architecture:87, :205.
  5. security-plan §Input Validation, Driver session socket row (three state readings inside the snapshot's text and the diff; PROVISIONAL) — **carried**: Symbols / APIs ("Not added"), Coverage. Sites: the row at security-plan:74; `platform adapter` security-plan:118 (the id's exits). `PROVISIONAL` 0 hits in the seven masters; the playbook holds the two rules (`.andromeda/playbook.md:36`, `:40`).
  6. security-plan §Input Validation, Markup attributes rows (three attribute reads with their parse · the platform adapter crossing gains the class "states"; PROVISIONAL) — **carried**: Symbols / APIs, Coverage. Sites: `aria-label` security-plan:119 (the row); the adapter's content classes security-plan:118.
  7. a11y-plan §2 and §7 (the builder writes state on an element node · the snapshot takes three states from the tree · the CRUD rows and list carry roles · a listbox option no keyboard reaches) — **carried**: Symbols / APIs, Coverage. Sites: `blitz-dom/src/accessibility.rs` a11y-plan 17 lines; `aria-hidden` a11y-plan:106; `[Ll]ist[Bb]ox` a11y-plan:102, :117; `` `focused` `` a11y-plan :155, :205, :271, :315; `unfocusable|no keyboard reaches` 0 hits.
  8. a11y-plan §4 (the three attributes in the pattern catalog) — **carried**: Symbols / APIs. Sites: `aria-(selected|pressed|invalid|checked)` 0 hits; the catalog's neighbours by `aria-hidden` a11y-plan:114, :122, :124, :295, :296.
  9. test-plan §1, §2, §4, §9 (the new files, tests and re-counted totals) — **carried**: Counts. Sites: test-plan:21, :24, :26, :66, :100, :101, :116, :200, :329 (each named in the Counts table), and its key file `.andromeda/registries/contracts/test-plan/session-lifecycle.md:11` — "the eleven driver-action files" with a test count per file; the twelfth, `stand_act_states`, holds 3 tests, `Session::run` in each, both layout modes.
  10. obs-plan §3 (the two new check files in the no-subscriber census) — **carried**: Coverage, Counts ("eleven" driver-action checks). Sites: `installs no subscriber` obs-plan:69, :70, :106 — the census is :70.
  11. obs-plan §8 (the host-log figures, if they moved) — **not carried: they did not move.** Alone 0 lines unset and 742 B in 4 lines at `info`, `debug` and `trace`; workspace 860 B in 5; 0 of 15 id needles, 0 of 6 name needles, 0 sentinel occurrences, stdout 0 B in all eight runs (`evidence/readings.md`). Sites: `by-level|742 B|860 B` obs-plan:42, :294, :299.
  12. layout-templates §Surface: desktop-native → Primary screens (the roles and state attributes on the two tasks, attributes only) — **carried**: Symbols / APIs (Stand markup). Sites: `flight_booker\.rs` and `tasks/crud\.rs` layout-templates:10.
  13. design-system (coordinates only — the citations into `flight_booker.rs` and `json.rs`) — **not carried as an amendment: no design value moved**; the citations are the citation sweep's. Sites: `flight_booker\.rs` design-system:53, :54, :207; `escher-driver/src/json\.rs` design-system:396. The four lines added to `flight_booker.rs` all stand above its styles; the thirteen added to `json.rs` at `:102-176` stand above the cited `:196-235`.
- **Coverage of new surfaces:**
  - the builder's three attribute reads (markup from any document, remote HTML included) → validation closed-set match by value, no panic path✓ (a value of 5,500 characters builds the tree; `accessibility_states`) · instrumentation n/a (no log site, by design) · PII nothing written as typed✓ (the tree's `Debug` holds neither the unrecognized word nor the long value) · tests integ✓ (`accessibility_states`, 4 tests, both layout modes, both builds) · a11y ✓ (it is the a11y tree's state) · tokens n/a
  - `selected` / `toggled` / `invalid` on accessibility nodes handed to the platform adapter → validation n/a (output) · instrumentation n/a · PII closed enums and bools✓ · tests ✗ at the adapter (`unrunnable-here`: no stand binary builds one, and the dev host's AT-SPI bus is off) · a11y `unrunnable-here` · tokens n/a
  - `NodeState.selected` · `.pressed` · `.invalid` → validation n/a · instrumentation n/a · PII bools✓ · tests unit✓ (3) + integ✓ (`stand_snapshot_state`, 3) · a11y each reading asserted equal to the accessibility node's✓ · tokens n/a
  - the text tokens `selected` `unselected` `pressed` `unpressed` `invalid` → validation n/a · instrumentation n/a · PII a closed set of bare words✓ · tests unit✓ (`state_tokens_follow_the_model`, a fixture test) + integ✓ (`stand_snapshot_text` unedited, both screens under their ceilings) · a11y n/a · tokens n/a
  - the diff node's keys `selected` `pressed` `invalid`, crossing the session socket and the command line → validation n/a (output) · instrumentation the command span unchanged at eight fields, no field for a state✓ (`stand_act_spans`, `host_log` under both builds) · PII booleans✓ · tests unit✓ (`json.rs`) + integ✓ (`stand_act_states`, `stand_act_diff`, `cli_commands`, `cli_flow`) + by hand through the binary (`evidence/readings.md`) · a11y n/a · tokens n/a
  - the flight booker's mode buttons as toggle buttons (`aria-pressed`) → validation n/a · instrumentation n/a · PII n/a · tests integ✓ · a11y role Button unchanged, focus order unchanged✓ (`stand_accessibility_ids` unedited) · tokens n/a (no style line)
  - the two date inputs' `aria-invalid` → tests integ✓ · a11y the non-colour invalid cue check passes beside it✓ (`stand_flight_booker` unedited) · tokens n/a · the rest n/a
  - the CRUD list as `listbox` and its rows as `option` with `aria-selected` → tests integ✓ · a11y roles read `ListBox` / `ListBoxOption`✓, the actionable-key check still returns nothing✓ (`stand_actionable_keys` unedited), **keyboard ✗ — a row is not focusable, so the stand shows a listbox option no keyboard reaches** (decided at P4, `inputs#I3`; owner: the route entry "Stand keyboard harness"), **name ✗ — the list reads the accessible name `""`** (measured through the binary; owner: "Stand a11y assertions") · tokens n/a (no style line) · the rest n/a

## Deviations from intent

1. **The red reading of steps 2 and 3.** On the tree as built both test files fail to compile — they name fields the model gains at step 5 — so their "red before the code" is a compile error. Recorded as that, and an assertion-level red was taken after steps 4 to 8 with the stand still unchanged (`evidence/controls.md` §1). *Justification:* the plan's order was kept; the second reading is the one that shows the tests fail for the reason they exist.
2. **Five controls beyond step 10's list.** The step asks one mutation per new or extended test and lists seventeen, which reach no red for four tests: the diff unit test, the two restated schema tests and `aria_checked_sets_none_of_the_three`. Five controls were added and named as outside the list. *Justification:* the step's own rule; without them four tests had never been seen red.
3. **Rows added to step 1's tables.** `TRUE` for `aria-pressed` and `aria-invalid` (the plan's Implementation notes say step 1 pins it) and a whitespace-only `aria-invalid`, which reads not invalid — the consequence of trimming before the empty-value test.
4. **The order inside step 2's CRUD test.** Create and Delete run before the filter edit, so Delete is asserted with three rows remaining, not one. Every clause of v010-17's acceptance is asserted.
5. **`to_text`'s rustdoc example** gained two lines, not one, and its container's height figure was changed to hold them.
6. **Two of step 11's readings came from other sources than the plan names** (Spec claims disproved, item 2).
7. **The operator pass ran in the implement session**, on the operator's word given after implement's report, and this wrap ran Phase 1 only in its first session, on the operator's word given with its invocation ("run P1 only and stop" — the operator, 2026-10-10). When this report was authored Phases 2 to 7 had not run: no amendment was applied, the master record read `pending`, and v010-17 read `implemented`. **Resumed on the operator's word** (`inputs#I7`, 2026-10-10, a later session): "resume from report.md" — the report is reused as authored, with five lines amended at the resume (this one, the founder's-word line of Cross-project, the Inputs list — entries `I6` and `I7` — and, under Decisions & corrections, the operator-directions line and the provisional line); the fan-out, the gates, the flip and the commit follow in the same run dir.

scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 13 · listed 13 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 59`).

## Decisions & corrections

- **Operator directions this chunk** — the operator pass, in plan order, with the commit subject named and "Stop and tell me if any entry is red" (quoted whole in `evidence/operator-pass.md`); "run P1 only and stop" for this wrap's first session; at its resume (`inputs#I7`): "resume from report.md", the founder's relay read first, a halt at Validate on a card that cites his recorded word and lists the proposals it covers, and the seven route items sent to the route-resolve as listed. No correction of the work was given.
- **Decisions the plan carried and the code now holds** (all the operator's, `inputs#I3`, `inputs#I4`, `inputs#I5`): attributes read on every element by value; `aria-checked` unread; a boolean model with the two limits said in the served help; rows as `option` in a `listbox`, mode buttons as toggle buttons; tokens after `value=`; `invalid` always written in JSON; the `a11y` leg stays three files; Cells untouched.
- **Provisional when this report was authored, awaiting the founder:** the two boundary forks — three state readings inside the snapshot's text and the diff, and three state properties on nodes the platform adapter reads. At the resume his answers are in (`inputs#I6`): option A of each. Their record in the security plan is not this report's to make — it is Validate's, on a halt and the operator's one confirm (`inputs#I7`).
- **Route items the plan owes to route-resolve** (none done by this chunk): a CARRY on "Stand requirement sweep" for an ARIA checkbox's and switch's reading · a CARRY on "Self-description" for a help per node key · the clickable-row CARRY on "Stand requirement sweep" re-read — a row now reads `ListBoxOption` · a CARRY on "Stand keyboard harness" for the option no keyboard reaches · a CARRY on "Stand a11y assertions" for the unnamed listbox and for the windowed witness of the three states · the merge-surface CARRY on "Upstream sync ahead of polish and ship" re-measured for `accessibility.rs` (+57 −1 against `7832c177`) · whether Cells states its selected cell, on "Stand requirement sweep".
- **Found by the controls:** no stand screen shows a mixed toggle, so `snapshot-mixed-reads-pressed` leaves all four stand files green — the limit is held at the engine and by two unit tests only. `stand_snapshot_state`'s `stated` helper asserts all three readings of every node against the tree, so a wrong reading of one state also fails a test of another on a screen that states both.
- **A false alarm of implement's own:** the control batch's closing check compared `git status` text before and after; a ledger-tool trail written into the run dir meanwhile made the texts differ and the call exit 1. The sources were restored — the per-control sha256 lines are the reading.
- **Sweep hazards:**
  - `grep 'NodeState {'` cannot tell a literal closed by a spread from a full one; research's "11 no-change" was read off the hits, and one was full.
  - A green `cargo test` prints no figure a test only formats into a failure message: a size or a count "from the entry's log" needs an entry that prints it.
  - `\b107\b`, `\b106\b` and `\b119\b` hit 19, 27 and 17 lines of the masters and registries, and the hits read include line numbers inside citations (test-plan:57 `harness_test.rs:119-183`, test-plan:207 `screenshot.rs:107-143`, the key file's `client.rs:53-119`) — a count is located by its words, never by its digits. Not every one of those 63 lines was read.
  - The gate tool files an operator firing made with no run dir under `run-{stamp}`, not under the implement run's log directory.
  - The Bash guard refuses a heredoc with a file target even when the target is the session scratchpad; a script goes through the Write tool.

## Outcome

**Acceptance criteria, each re-asserted against the diff:**

- (matrix) **v010-17** — met. Every clause is asserted by a test in the diff: the pressed readings and their swap, with no other node reading a pressed (`stand_snapshot_state.rs:377-397`); the selected readings, the move, Delete and a filter edit, with no node outside the rows reading a selected (`:399-449`); the invalid readings on both date inputs, with no other node invalid (`:451-486`); each equal to the accessibility node's (`:80-107`); and through `Session::run` each step returning the node as it reads after it, each diff equal to the check's own and across the two modes (`stand_act_states.rs:79-103`, `:105-134`, `:136-158`; `stand_act_diff.rs`). Ledger: `implemented`, eight test ids.
- (a11y) the three attributes on a parsed page, every value of each grammar, a role that supports the attribute and one that does not, `aria-checked` setting nothing — met (`accessibility_states.rs`, row counts 9, 10 and 11 asserted).
- (a11y) roles, names, the actionable-key check and the focus sequences unchanged; the non-colour invalid cue — met (the standing-checks entry, unedited files).
- (security) a closed set of bare words in the text and booleans in JSON; a value outside the read set builds the tree and is written nowhere — met.
- (security) the password and file mask reads as before — met.
- (security, obs) the command span at eight fields; `host_log` at 0 id needles, 0 name needles, 0 typed text under both builds — met.
- (obs) no log site in the builder; no subscriber, env read or print in a new check — met (the engine-lines entry; 0 hits of eight patterns over the two new files).
- (arch) the change lands only in the eleven files and the two new ones; no manifest or lock line; no crate under `packages/` names seven_guis — met.
- (arch) every new key named in `NODE_FIELDS` and written in its order; nine verbs, six argument kinds, eight causes — met.
- (arch) `to_text` pure with one fixed token order; every screen under its ceiling and the budget; no id type in `NodeState` — met (1287 of 1536, 2049 of 2304).
- (arch, tests) the two help texts held word for word; each limit pinned at the engine, the snapshot and the text — met. On the stand itself neither limit is exercised (no mixed toggle, no grammar or spelling error) — stated, not a gap in the criterion.
- (layouts) the two task files differ by added attribute lines only; ids, row ids and the clipping list as they were — met.
- (layouts, design) an accepted command writes one line of uncoloured JSON and exits 0; the two shell flows match as written — met.
- (design) no colour value, transition or animation added — met.
- (tests) each new or extended test recorded red once; each new check in both layout modes and green under both builds — met (`evidence/controls.md`).
- (tests) `fast`, `doc` and `a11y` green; `boot` then `run stand` ends `passed`, failed 0 — met.
- (CI) the fork's run on the pushed sha reads green — met: CI#38072258834.
- No criterion rests on a drifted input.

**Gates** — implement's one full call of the block: `entries 20 · green 17 · red 0 · recorded 0 · timeout 0 · not-run 3`, no fix iteration, no deferral, no `defer` key in the block.

| Entry (by its `run`) | Verdict |
|---|---|
| `cargo test -p dioxus-native-dom --locked` | green · exit 0, `lacks FAILED` |
| `cargo test -p escher-driver --locked` | green · exit 0, `lacks FAILED` |
| `cargo test -p blitz-tests --locked --test accessibility_states --test stand_act_states` | green · exit 0, `lacks running 0 tests`, `lacks FAILED` |
| `cargo test -p blitz-tests --locked --test stand_snapshot_state --test stand_act_diff` | green · the same three atoms |
| `cargo test --workspace --locked --test accessibility_states --test stand_act_states --test stand_snapshot_state --test stand_act_diff` | green · the same three atoms |
| `cargo test -p blitz-tests --locked --test accessibility_hidden --test accessibility_roles … --test stand_session_state` (the fifteen standing files) | green · the same three atoms |
| `cargo test -p seven_guis --locked --test cli_commands --test cli_flow` | green · the same three atoms |
| `cargo test -p seven_guis --locked --test host_log` | green · the same three atoms |
| `cargo test --workspace --locked --test host_log` | green · the same three atoms |
| `git diff -U0 f0cb9dd7… -- examples/seven_guis/src/tasks/flight_booker.rs examples/seven_guis/src/tasks/crud.rs \| … grep -c -v …` (attributes only) | green · `exit 1`, `last line 0` |
| `git diff f0cb9dd7… -- packages/blitz-dom/src \| … grep -c -E 'println!\|…'` (no log site, no panic path) | green · `exit 1`, `last line 0` |
| `git diff --quiet f0cb9dd7… -- Cargo.toml Cargo.lock deny.toml …` (preservation) | green · exit 0 |
| `bash scripts/agent-run.sh boot` | green · exit 0 |
| `bash scripts/agent-run.sh run stand` | green · exit 0 — `run.end`: passed 125, failed 0, ignored 6, 32 files, `outcome` passed |
| `bash .github/scripts/ci-leg.sh fast` | green · exit 0 — 167 result lines, 791 passed, 0 failed, 11 ignored |
| `bash .github/scripts/ci-leg.sh doc` | green · exit 0 |
| `bash .github/scripts/ci-leg.sh a11y` | green · exit 0 — 6 + 6 + 3 |
| `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (`leg = 'operator'`) | green in the operator pass — `hygiene: clean`, no row, three readings |
| `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` (`leg = 'operator'`) | green in the operator pass — exit 0 · `history moved: refs/remotes/origin/build/escher-0.1.0 f0cb9dd7→2fc32829`; its `fast` leg on the committed tree: 167 result lines, 791 passed, 0 failed, 11 ignored |
| `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` (`leg = 'operator'`) | green in the operator pass — `verdict: green · checks 16/16`, CI#38072258834 |

**Smoke:** skipped — no boot path is edited, the plan lists no smoke or self-verify entry, and the project has no headful self-verify. The `escher-session` binary was driven by hand end to end for the readings (`evidence/readings.md`); the windowed stand was not booted.

**Watches:** none — the entry carried no `watch:`.

**Outcome basis:** the operator pass's final state — one commit, `2fc32829`, and the CI run on it, both recorded in `evidence/operator-pass.md`. Between implement's gate call and that commit no source file moved: the eight files the controls touched read the sha256 `evidence/controls.md` records, and the commit left the tree clean. Implement's report, given in this session's conversation, is the basis for what only it holds (the controls, the hand readings, the census). The one file changed since the commit is `evidence/operator-pass.md`, extended after the push. The wrap has run Setup and Phase 1 only.

**Process hygiene** (implement's census, re-measured at this report against the host's process list, 2026-10-10T17:50Z):

| Process | Started by | Final state |
|---|---|---|
| `cargo`, `rustc` and test binaries of the gate call, the controls and the readings | implement | terminated |
| `escher-session` hosts — 8 by the by-level instrument, 4 for the size readings, 2 for the end-to-end drive | implement | terminated — each ended by `stop`, no state directory left under `target/tmp/` |
| hosts started inside the `cli_*`, `host_*` and `stand_session_*` checks | the gate entries | terminated |
| the `fast` leg and the push of the operator entry; the CI poll | the operator pass | terminated |
| `python scripts/code-graph.py refresh` | this wrap's Setup | terminated — exit 0, `tree-refresh[rust]: 7373 nodes / 42124 edges` |

Not this chunk's and left alone: three `rust-analyzer` servers with their proc-macro children, each running for more than 21 hours, and a `hostwatch` script more than three days old.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: f0cb9dd7 (the parent of the oldest pre-CI commit 2fc32829) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### examples/seven_guis/src/tasks/crud.rs — added 3 line(s) in 2 range(s)
added: 58 · 71-72
### examples/seven_guis/src/tasks/flight_booker.rs — added 4 line(s) in 4 range(s)
added: 69 · 76 · 85 · 93
### packages/blitz-dom/src/accessibility.rs — added 31 line(s) in 2 range(s)
added: 2-4 · 102-129
- 2-4 «use accesskit::{»
### packages/dioxus-native-dom/src/snapshot.rs — added 151 line(s) in 6 range(s)
added: 6 · 34-35 · 59-69 · 94-95 · 159-161 · 607-738
  - 608-613 @609 «fn attribute(doc: &DioxusDocument, id: &str, name: blitz_dom::LocalName) -> Option<String> {»
  - 615-625 @617 «const SELECTED: [(Option<&str>, Option<bool>); 7] = [»
  - 627-648 @628 «fn an_aria_selected_value_reads_selected() {»
  - 650-660 @652 «const PRESSED: [(Option<&str>, Option<bool>); 7] = [»
  - 662-708 @663 «fn an_aria_pressed_value_reads_pressed_and_a_checked_one_does_not() {»
  - 710-720 @712 «const INVALID: [(Option<&str>, bool); 7] = [»
  - 722-738 @723 «fn an_aria_invalid_value_reads_invalid() {»
### packages/dioxus-native-dom/src/snapshot_diff.rs — added 24 line(s) in 1 range(s)
added: 256-279
  - 256-278 @257 «fn a_selected_pressed_or_invalid_reading_alone_names_that_node() {»
### packages/dioxus-native-dom/src/snapshot_text.rs — added 102 line(s) in 7 range(s)
added: 26-31 · 34 · 38-39 · 71-83 · 200-234 · 255-257 · 260-301
  - 71-75 «match node.state.selected {»
  - 76-80 «match node.state.pressed {»
  - 81-83 «if node.state.invalid {»
### packages/escher-driver/src/json.rs — added 28 line(s) in 8 range(s)
added: 102-104 · 129-135 · 174-176 · 435-437 · 443 · 455-457 · 463-467 · 479-481
  - 129-131 «if let Some(selected) = parts.selected {»
  - 132-134 «if let Some(pressed) = parts.pressed {»
### packages/escher-driver/src/lib.rs — added 4 line(s) in 1 range(s)
added: 53-56
### packages/escher-driver/src/schema.rs — added 25 line(s) in 7 range(s)
added: 32-37 · 203-205 · 232-236 · 430-434 · 439-441 · 585 · 589-590
### tests/blitz-tests/tests/accessibility_states.rs — new file · 220 line(s)
- 45-57 «fn page(html: &str, incremental: bool) -> HtmlDocument {»
  - 46-53 «let mut doc = HtmlDocument::from_html(»
- 59-74 @61 «fn carriers(attribute: &str, supporting: Supporting, values: &[Option<&str>]) -> String {»
  - 63-72 «let rows: String = values»
- 76-92 @78 «fn read(doc: &HtmlDocument, tree: &TreeUpdate, id: &str) -> (Role, States) {»
  - 79-81 «let node_id = doc»
  - 82-87 «let node = tree»
  - 88-91 «(»
- 94-126 @98 «fn assert_rows(attribute: &str, supporting: Supporting, rows: &[(Option<&str>, States)]) {»
  - 101-125 «for incremental in [false, true] {»
- 128-145 @129 «fn aria_selected_reads_true_and_false_and_nothing_else() {»
  - 132-142 «let rows = [»
- 147-165 @148 «fn aria_pressed_reads_its_three_values_and_nothing_else() {»
  - 151-162 «let rows = [»
- 167-186 @168 «fn aria_invalid_reads_its_values_and_an_unrecognized_one_as_true() {»
  - 171-183 «let rows = [»
- 188-220 @189 «fn aria_checked_sets_none_of_the_three() {»
  - 192-219 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_diff.rs — added 29 line(s) in 7 range(s)
added: 4-7 · 125-130 · 137-142 · 144 · 151-157 · 216 · 222-225
### tests/blitz-tests/tests/stand_act_states.rs — new file · 158 line(s)
- 18-23 @19 «struct Held {»
- 25-56 «impl Held {»
  - 26-32 «fn task(task: LeanTask, incremental: bool) -> Self {»
  - 34-55 @37 «fn step(&mut self, call: &Call) -> Acted {»
- 58-66 @59 «fn changed<'d>(acted: &'d Acted, id: &str) -> Option<&'d NodeState> {»
  - 60-65 «acted»
- 68-77 @70 «fn assert_modes_agree(runs: &[Vec<SnapshotDiff>]) {»
  - 73-76 «assert!(»
- 79-103 @80 «fn a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading() {»
  - 82-101 «for incremental in [false, true] {»
- 105-134 @106 «fn a_row_click_returns_the_row_selected_and_the_row_it_leaves_not_selected() {»
  - 108-132 «for incremental in [false, true] {»
- 136-158 @137 «fn a_typed_start_date_returns_the_field_invalid_or_not() {»
  - 139-156 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_snapshot_state.rs — added 187 line(s) in 6 range(s)
added: 4-7 · 12 · 16 · 21 · 80-148 · 377-487
- 80-107 @84 «fn stated(harness: &Harness<DioxusDocument>) -> Snapshot {»
  - 87-105 «for read in snapshot.nodes() {»
- 109-116 @111 «fn pressed(harness: &Harness<DioxusDocument>) -> Vec<(String, bool)> {»
  - 112-115 «stated(harness)»
- 118-129 @120 «fn selected(harness: &Harness<DioxusDocument>) -> Vec<(String, bool)> {»
  - 121-128 «stated(harness)»
- 131-139 @133 «fn invalid(harness: &Harness<DioxusDocument>) -> Vec<String> {»
  - 134-138 «stated(harness)»
- 141-147 @142 «fn readings(stated: &[(&str, bool)]) -> Vec<(String, bool)> {»
  - 143-146 «stated»
- 377-397 @378 «fn exactly_the_active_flight_mode_reads_pressed() {»
  - 379-396 «for incremental in [false, true] {»
- 399-449 @400 «fn exactly_the_selected_row_reads_selected() {»
  - 401-448 «for incremental in [false, true] {»
- 451-486 @452 «fn a_date_that_is_no_date_reads_invalid() {»
  - 453-485 «for incremental in [false, true] {»
