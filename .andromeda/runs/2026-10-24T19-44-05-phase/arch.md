# arch extract

## Relevance
partial — the chunk is a test-and-CI re-read in `tests/blitz-tests/` and `.github/scripts/`, with no new crate, surface or engine code. Arch binds it through the `a11y` leg's CI contract, the test conventions, and the engine semantics it re-asserts (activation, `disabled`/`inert`, the select model).

## Constraints
- The `a11y` leg is one leg of the one leg runner `.github/scripts/ci-leg.sh {leg}`, run the same way in CI and on the dev host. Its named-file list and its pin `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log` move together. Its last widening added no new job, no required status check and no `ci.yml` change (per architecture §Standard Contracts → CI contracts; detail `standard-contracts.md` contract-ci-contracts).
- The `a11y` job restores the test job's `workspace-test` cache with `save-if: false` and never saves. A widened leg keeps that cache relation and the fork's 10 GB cache budget (per architecture §Occupied Resources → CI infrastructure).
- blitz-tests keeps one test target per file, each file covering one behaviour and opening with a `//!` doc. Scenarios run in both layout modes. Stand checks share helpers (`tests/common/` tables such as `tab_order` / `ENABLED_TAB_ORDER`) through modules that are no test target. A test first asserts that its fixture produces the condition. Only upstream's `tests/all.rs` is declared, at `test = false` (per architecture §Conventions → Tests; detail `conventions.md` conv-tests).
- Incremental and non-incremental layout must produce identical layouts, so every re-asserted Tab, activation or contrast reading holds in both modes (per architecture §Design Philosophy, "One incremental pipeline, safe identities").
- Activation semantics to re-assert: Enter and a released Space dispatch a Click that the driver queues, so the app's handlers run before the default action. On a select's choice the engine dispatches `change` only after `input` (per architecture §Standard Contracts → Events).
- `disabled` is read one way: by its presence, on a form control alone (`ElementData::is_disabled`), for focusability, the pointer and click gates, and Enter/Space activation. `inert` on an element or an ancestor makes it unfocusable and unhit, and takes it out of the accessibility tree and the snapshot. HTML's focus fixup is a known limit owned by a residuals line, not this chunk's (per architecture §Established Decisions → DOM semantics; and §Standard Contracts → Nodes and widgets for `is_disabled` being the one disabled reading).
- The keyboard activation and focus rules, the select model (keys, typeahead, click, all through `apply_select_choice`, no popup, `multiple`/`optgroup` not modelled) and the document's clock are direct engine changes for every document. A re-read that needs any of them changed is not a test edit (per architecture §Established Decisions → Scrolling and input).

## Patterns to follow
- Contrast is read through `Harness::text_contrast`, which reads computed styles and changes nothing. Time-dependent readings (the select's 1 s typeahead window) move only through the harness's shared `VirtualClock` (`tick` / `advance_time`). `screenshot`, `text_contrast` and `settle` never move it (per architecture §Standard Contracts → Test harness).
- The stand is booted by `seven_guis::stand`: the four lean tasks, offline, in a pinned 800x600 Light viewport with one bundled font, no env read and no telemetry. Re-asserted rows read that boot (per architecture §Standard Contracts → Headless stand).
- The accessibility tree is read through `build_accessibility_tree`, which writes each state attribute once. Its stated measured limit is that no native checkbox's or radio's checked state is in the tree; that limit is owned by a residuals line and is not re-baselined here (per architecture §Standard Contracts → Mutation, query and CSSOM).
- The current a11y witnesses outside the leg are already listed per file, with the targets that run them (workspace `test` leg, per package, `stand-repeat`). Whether each candidate (`accessibility_states`, `inert_subtree`, `select_interaction`, `stand_act_select` …) still exists and runs as its own target is research's question (per architecture §Existing Scopes; detail `existing-scopes.md` scopes).

## Anti-patterns to avoid
- Adding a crate, binary, env var, listener or shared module that is a test target is a §Occupied Resources registration, never a silent add. The chunk adds none (per architecture §Occupied Resources → Names; §Occupied Resources → Environment variables).
- Folding an engine behaviour change for every document into the re-read, such as select key handling, `disabled`/`inert` gates or activation keys. Each is a ratified direct engine change, and a new one needs a ruling (per architecture §Established Decisions → Scrolling and input).

## Contract bindings
- arch §Standard Contracts → CI contracts (the `a11y` leg in `ci-leg.sh`, its `test_ci_workflows.py` pin) ↔ a11y-plan §9 (the a11y harness and leg) ↔ tests: the test-plan §9 → Legs list of `stand-repeat`'s named targets. A file added to `a11y` may already be one of `stand-repeat`'s ten named targets. Whether the requirement-witness check reading `ci-leg.sh`'s text (`stand_requirement_sweep`) is affected by an `a11y` line change is research's question.
- arch §Occupied Resources → CI infrastructure (the `workspace-test` cache restore) ↔ tests: the test job's cache key, which the `a11y` job only restores.
- arch §Conventions → Tests ↔ tests: `test_blitz_tests_targets.py` (`BlitzTestsTargetsTest`) pins the one-target-per-file manifest, so any newly named `--test` must be an auto-discovered file target.

## Acceptance criteria contributions
- (arch) `ci-leg.sh`'s `a11y` leg names each added file as a literal `--test` word that resolves to an existing blitz-tests file target, and `LegScriptTest`'s a11y tuple matches it word for word. `ci-scripts` is green (per architecture §Standard Contracts → CI contracts).
- (arch) The `ci.yml` "Accessibility (a11y) tests" job keeps its tier and dependencies. It gains no required status check and still restores `workspace-test` with `save-if: false` (per architecture §Occupied Resources → CI infrastructure).
- (arch) Every new or re-recorded row, such as a select keyboard row in `stand_keyboard`, runs in both layout modes in a file that keeps one behaviour per target. Shared tables stay in the non-target `tests/common/mod.rs` (per architecture §Conventions → Tests).
- (arch) The diff adds no crate, binary, env var, listener or engine source change under `packages/` (per architecture §Occupied Resources → Names; §Established Decisions → Scrolling and input).
