# Report — 2026-10-06-stable-element-ids

**Chunk:** Stable element ids — author key else component path, on every stand element (v010-01)
**Date:** 2026-10-06T09:37Z
**Commits:** since last_wrap 2026-10-06T08:48:32Z — `405a562b chore(2026-10-06-stable-element-ids): operator pre-CI commit, for the run this chunk's verdict reads` (basis `git log --format='%h %s' 5371292e..HEAD`; 5371292e = the parent of the oldest pre-CI commit)

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-status 5371292e` + the untracked tree; source/test set = `gate.py scope`: changed 8 · listed 8)
  - new: `packages/dioxus-native-dom/src/element_id.rs` (the derivation + 6 inline unit tests) · `tests/blitz-tests/tests/stand_element_ids.rs` (4 stand checks)
  - modified: `packages/dioxus-native-dom/src/dioxus_document.rs` (+25 lines inserted after old line 182: two pub methods) · `packages/dioxus-native-dom/src/lib.rs` (+1 line after old 12: `mod element_id;`) · `examples/seven_guis/src/tasks/{counter,flight_booker,timer,crud}.rs` (HTML `id:` attributes; one Dioxus `key:` on the CRUD row)
  - ledger / audit: `escher-0.1.0/verification-matrix.json` (v010-01 planned → implemented), the phase / implement / wrap run dirs, `chunks/2026-10-06-stable-element-ids/{scope,research,plan,report}.md` + `evidence/operator-{15-hygiene,16-push,17-ci}.txt`
  - **line maps (old → new) for citation re-pointing** (basis `git diff -U0 5371292e -- {file}`):
    - `dioxus_document.rs`: ≤182 unchanged · ≥183 → +25
    - `lib.rs`: ≤12 unchanged · ≥13 → +1
    - `tasks/counter.rs`: ≤12 unchanged (11 edited in place) · ≥13 → +1
    - `tasks/flight_booker.rs`: ≤67 unchanged · 68–72 → +1 · 73–78 → +2 · 79–83 → +3 · 84–89 → +4 · ≥90 → +5 (103 → 108 edited in place)
    - `tasks/timer.rs`: ≤79 unchanged (74, 76 edited in place) · 80–94 → +1 (92 → 93 edited in place) · ≥95 → +2
    - `tasks/crud.rs`: ≤40 unchanged · 41–61 → +1 (51 → 52 edited in place) · 62–80 → +2 · 81–85 → +3 · 86–93 → +4 · 94–99 → +5 · 100–112 → +6 · ≥113 → +7
  - citations into these six files across the masters (basis `grep -rnoE '(dioxus_document|dioxus-native-dom/src/lib|tasks/(counter|flight_booker|timer|crud))\.rs:[0-9]+(-[0-9]+)?' .andromeda/*.md .andromeda/registries CLAUDE.md .claude/rules .claude/docs`, sidecars excluded): architecture 23 · design-system 17 · test-plan 10 · a11y-plan 8 · layout-templates 3 · security-plan 3 · obs-plan 0 · registries 0 · CLAUDE.md/rules/docs 0 — each re-pointed by the map, or confirmed below the first shifted line
- **Symbols / APIs:**
  - `DioxusDocument::element_id(&self, node: NodeId) -> Option<String>` — pub, new; `None` for a non-element, stale or detached node; never panics
  - `DioxusDocument::element_ids(&self) -> Vec<(NodeId, String)>` — pub, new; every element reachable from the document root, document pre-order, ids pairwise distinct
  - crate-private `element_id` module: `element_ids(dom, state, doc)`, `element_id(dom, state, doc, node)`, `VdomWalk`, `place_children`, `component_name`, `is_framework_scope`
  - read-only dioxus-core 0.7.10 public API used: `VirtualDom::base_scope`, `ScopeState::{try_root_node, id}`, `VNode::mounted_root`, `VComponent::{name, mounted_scope}`, `VNodeInner::{key, template, dynamic_nodes}`, `ScopeId::{ROOT, ROOT_SUSPENSE_BOUNDARY, ROOT_ERROR_BOUNDARY, APP}`
  - `DioxusState::try_element_to_node_id` gains a 4th read-only caller (the walk); the other 3 callers (`DioxusState::element_to_node_id`, `MutationWriter::assign_node_id`, `assert_no_stale_mappings`) are unchanged
  - **the id grammar as built** (the contract): (1) **author key** — the HTML `id`, verbatim, when non-empty, `/`-free and the first element in document pre-order to read that value as a key; (2) **component path** — `/`-joined component segments from below the app root to the owning component (dioxus-core's four framework scopes ROOT · ROOT_SUSPENSE_BOUNDARY · ROOT_ERROR_BOUNDARY · APP contribute none; a component's segment is the LAST `::` segment of `VComponent.name`, generics stripped; an owner's 2nd+ instance of a same-named component reads `{name}:{k}`), then one element segment per DOM level: `{tag}[{key}]` for a template root of a Dioxus-keyed VNode (non-empty, `/`-free, first among same-parent same-owner segments), else `{tag}:{n}` with `n` the index among same-parent, same-owner-instance, same-tag element siblings; a template root whose owner equals its DOM parent's owner (a list row, an `if` branch) APPENDS to the parent's path; one whose owner differs RESTARTS at the owner's component chain; an element owned by the app root itself reads `/{tag}:{n}…` (empty chain); (3) **document path** — an element no component renders: `/` + `{tag}:{n}` per DOM level from the document root (`/html:0`, `/html:0/head:0`, `/html:0/body:0`; `main#main` reads its author key `main`)
  - measured stand ids (basis: a dump of `element_ids()` on each `stand::boot(task, stand::options(true))`, this session): chrome `main` · `TaskShell/style:0` · `task-shell` · `task-header` · `back-btn` · `task-title` · `task-header-spacer` · `task-body` on every task; Counter `TaskShell/Counter/div:0` · `TaskShell/Counter/div:0/style:0` · `TaskShell/Counter/div:0/div:0` · `counter-value` · `counter-increment`; CRUD rows `TaskShell/Crud/div:0/div:1/div:0/div[0]` … `div[2]`
- **Crates / modules:** module `element_id` added to `dioxus-native-dom` (private); no crate added or removed
- **Dependencies:** none (gate `git diff --quiet 5371292e… -- Cargo.lock Cargo.toml …` exit 0)
- **Schema / config:** none — no config key, env var, log field or scrub set; the id is computed on demand and written to neither the DOM nor the vdom
- **Spec-master edits:** none (implement wrote no master)
- **Counts / qualifiers moved:**
  - workspace tests 431 · 0 · 4 → **441 passed · 0 failed · 4 ignored**, result lines 120 → **121** (+6 `dioxus-native-dom` unit tests in the existing lib binary, +4 `stand_element_ids` integration tests in a new binary) — basis `grep -E '^test result:' target/ci-logs/test.log` summed, after entry 16's `ci-leg.sh fast` on 405a562b; stated in test-plan §9 (line 314)
  - stand checks 13 → **17** files-tests (stand_boot 6 · stand_counter 1 · stand_flight_booker 1 · stand_timer 3 · stand_crud 2 · stand_element_ids 4); `run stand` ok stand events 13 → **17** (gate entry `bash scripts/agent-run.sh logs | python3 …` last line 17); stated in test-plan §3 line 111 ("`run stand` 13 `ok` stand events" — a dated measurement of the stand-test-contract chunk)
  - `stand_*.rs` files 5 → 6
- **Dev-tool versions:** none — no host tool installed or changed
- **Harness / gate surface:** none changed — `scripts/agent-run.sh` picks `stand_element_ids.rs` up by its `stand_` prefix with no script change
- **Cross-project / external claims:**
  - dioxus-core 0.7.10 (locked, crates.io registry source, read at `~/.cargo/registry/src/…/dioxus-core-0.7.10/src/`): `VirtualDom::new_with_props` names the user root `"root"` and `new_with_component` mounts it under `RootScopeWrapper` (ScopeId 0) → `SuspenseBoundary` (1) → `ErrorBoundary` (2) → root (3 = `ScopeId::APP`) (`root_wrapper.rs`, `virtual_dom.rs:286-325`, `scopes.rs:46-61`); `VComponent.name` is the component's full type path (measured: a test component read `dioxus_native_dom::element_id::tests::…::Item`); keyed-sibling uniqueness is a `debug_assert` on DIFF only (`diff/iterator.rs:95-111`), not on create
  - CI: run **CI#37443002592** on sha **405a562b506e** (the operator pre-CI commit) — `verdict: green · checks 16/16 · wall 425 s` (evidence/operator-17-ci.txt); this wrap's commit adds bookkeeping + specs on top
  - inputs: `inputs: absent — no external input snapshotted` (`inputs.py verify`)
- **Reverted / negative API facts:** none
- **Insufficient fixes (written, kept, not the remedy):** none
- **Spec claims disproved by measurement:**
  1. plan §The id grammar rule 2 + step 1a: "excluding the base scope itself, which dioxus-core names `root` (virtual_dom.rs:293)" — false: the base scope is `RootScopeWrapper`; `"root"` names ScopeId::APP, three scopes below it. Built: all four framework scopes contribute no segment (the plan's pinned `TaskShell/…` literals hold). (stated: chunk plan.md + scope.md "The root component is named `root` by dioxus-core"; matrix v010-01 acceptance says "the component names below the root" — consistent with the build)
  2. plan step 1a: "push `c.name`" yields `Counter` — false: `VComponent.name` is the full type path. Built: last `::` segment, generics stripped. (plan.md only)
  3. plan §Uniqueness "holds by construction … by the per-parent, per-owner, per-tag index" — false for the grammar as written: two instances of one component read the same path (same or different DOM parent), and restarting segments at EVERY template root collides a same-owner nested root (a list row, an `if` branch) with a top-level root of the same tag. Built: `{name}:{k}` for repeated instances; a same-owner template root appends to its parent's path (the plan's own pinned CRUD literal `…/div:1/div:0/div[0]` already required append). (plan.md only)
  4. plan step 1d(5): "an element removed by a re-render (`mark_dirty` + `poll`), its old `NodeId` no longer resolving" — false: dioxus-native-dom's removal only DETACHES the node (parent `None`); its slot is dropped only when its ElementId is reassigned (`MutationWriter::assign_node_id`'s `remove_node_if_unparented_with`, mutation_writer.rs:118-137). Built: the test asserts the detached read (`None`), then drops the node through `DocumentMutator::remove_node_if_unparented` and asserts the stale read (`None`).
  5. plan step 5(d): "Select the first row, press Delete … the deleted row's old `NodeId` reads `None`" — the node that leaves the DOM is the LAST row's: with index keys `key: "{i}"`, the selected row's keyed VNode diffs in place against the new row 0 (same key "0"). Built: the test asserts exactly one row node left the list and that it reads `None`.
  - none of the seven masters states any of these (each lives in the chunk plan / scope only) — dispositions route to the architecture Dioxus DOM bridge amendment (the grammar as built) and to the report
- **Expected amendments (from plan):**
  - architecture §Standard Contracts → Dioxus DOM bridge — add `element_id` / `element_ids` + the grammar → **carried** (Symbols / APIs bullet; the grammar as BUILT, not as planned). Site: `grep -n 'Dioxus DOM bridge' .andromeda/architecture.md` → 1 hit (line 134); registries 0 hits (`grep -rn 'Dioxus DOM bridge' .andromeda/registries`)
  - test-plan §3 Inspection / stand checks and §9 Local baseline → **carried** (Counts / qualifiers moved). Sites: `grep -n '431' .andromeda/test-plan.md` → 1 hit (line 314, §9 baseline); `grep -n 'run stand. 13' .andromeda/test-plan.md` → 1 hit (line 111, a dated stand-test-contract measurement — append-only re-count, not rewrite); registries (`.andromeda/registries/test-plan-contracts.toml`, `contracts/test-plan/`) → 0 hits for `431` / `stand checks`
  - security-plan §Input Validation "Markup attributes" — the HTML `id` gains a reader → **carried** (Symbols / APIs: author-key rule; Coverage line below). Site: `grep -n 'Markup attributes' .andromeda/security-plan.md` → 4 hits (lines 103-106, a table; the `id` row is to be added)
  - layout-templates §Surface: desktop-native §Primary screens — the lean tasks' controls carry author ids → **carried** (Files: the four task files; Symbols / APIs: measured stand ids). Site: `grep -n 'Primary screens' .andromeda/layout-templates.md` → 3 hits (lines 5, 67, 89 — one per surface; the desktop-native one is the target)
  - `file:line` citations into `dioxus_document.rs` and the four task files → **carried** (Files: line maps + per-master citation counts)
- **Coverage of new surfaces:**
  - `DioxusDocument::element_id` / `element_ids` (read API over author-supplied HTML `id`) → validation {author-key rule: empty / `/`-bearing / later-duplicate → no key, path instead; non-element / stale / detached → `None` — unit cases 2, 3, 5, 6 + stand (d)✓} · instrumentation {n/a — on-demand pure read, no hot path; obs-plan §8 forbids an id/key/path log field, census gate `last line 0`} · PII {n/a — no log or event field} · tests {unit 6 + integration 4} · a11y {n/a — `id` feeds no role / hidden / focus derivation (`grep -rn 'local_name!("id")' packages/blitz-dom/src` → node.rs, mutator.rs, document.rs only, none in accessibility.rs or focus code); `ci-leg.sh a11y` green} · tokens {n/a — no style change}
  - lean-task markup (`id:` ×20 across the four tasks, `key:` ×1) → validation n/a · instrumentation n/a · PII n/a · tests {stand_element_ids (b), existing 13 stand checks green} · a11y {roles, hidden state and Tab order untouched — markup-only gate} · tokens {n/a — no class/style/text change: `markup-only` gate}

## Deviations from intent
- the id grammar departs from the plan's text in four places (Spec claims disproved 1–3), each because dioxus-core 0.7.10 contradicts the plan's premise or the plan's grammar is not unique; every pinned literal of plan step 5c measured exactly as predicted, and the matrix v010-01 acceptance text is satisfied as written
- unit case 5 and stand check (d) assert the measured removal behaviour (Spec claims disproved 4–5), not the plan's premise
- unit case 4 carries an extra assertion (two `Item` component instances under two `section`s read `Item/b:0` and `Item:1/b:0`) — the repeated-instance rule's witness, folded in to keep the plan's 6-test count
- known edge, not tested: the app root's own elements read `/{tag}:{n}…`; a root component rendering a top-level `html` element would read `/html:0`, equal to the skeleton's document path — the stand never does
- scope record: none — `gate.py scope` clean, changed 8 · listed 8 · recorded 0

## Decisions & corrections
- the operator directed the operator pass to the agent: entry 15 (hygiene), the pre-CI commit, entry 16 (fast · clean-tree guard · push), entry 17 (`ci.py conclusion --wait`), results recorded in `evidence/`, then stop for the CI verdict (operator, 2026-10-06)
- decision (implement): dioxus-core framework scopes are excluded by `ScopeId` constant, not by name; the component segment is the type path's last segment
- hazard: dioxus-core's `VComponent.name` is NOT the identifier written in rsx — it is the full type path (`crate::module::Component`, generics included); a consumer printing or matching it must strip the path
- hazard: a removed Dioxus node stays resolvable (detached, parent `None`) until its ElementId is reassigned — "removed" ≠ "stale NodeId" on a Dioxus document
- hazard: edits applied through a python script in Bash bypass the PostToolUse rustfmt hook — the fast leg's fmt check went red once; run rustfmt after any scripted `.rs` edit
- hazard (recurring, carried learning): the project Bash guard refuses `cat > file <<EOF`; a probe test file went through the Write tool instead
- the rsx macro refuses a static Dioxus key (`key: "same"` → "Key must not be a static string"); use `key: "{value}"`

## Outcome
- acceptance (tests · v010-01): **met** — `stand_element_ids` 4 passed on each lean task, both layout modes: one id per element in DOM pre-order, pairwise distinct, `element_id` ≡ `element_ids`, keyed elements read their key (chrome + every step-4 key; `flight-booked` after a Book press), unkeyed read a path (`/`-bearing), pinned literals hold, both modes equal
- acceptance (security): **met** — stale, detached, text and document-root nodes read `None` without panic; duplicate / empty / `/`-bearing `id` reads a path (unit cases 2, 3, 5, 6; stand (d))
- acceptance (arch): **met** — additive: two documented pub methods; no crate, dependency, env var or port; `mutation_writer.rs`, blitz-dom, the harness, `app.rs`, `stand.rs` unchanged (`git diff --quiet 5371292e…` exit 0); no `NodeId` / `ElementId` / `ScopeId` / pointer in an id
- acceptance (design · layouts · a11y): **met** — `markup-only` gate; 13 existing stand checks green; `ci-leg.sh a11y` green
- acceptance (obs): **met** — census `last line 0`; content-key probe `last line 0`
- acceptance (tests): **met** — `ci-leg.sh fast` and `doc` exit 0; workspace 441 · 0 · 4; `run stand` 17 ok
- acceptance (CI): **met** — `ci.py conclusion` on 405a562b: verdict green, CI#37443002592, 16/16
- gates (implement's final full-block run, `.andromeda/runs/2026-10-06T09-11-52-implement`; the operator pass on its final state):
  - `cargo test -p dioxus-native-dom --lib --locked element_id` — green (exit 0 · contains 6 passed)
  - `cargo test -p blitz-tests --locked --test stand_element_ids` — green (exit 0 · contains 4 passed)
  - `cargo test -p blitz-tests --locked --test stand_boot --test stand_counter --test stand_flight_booker --test stand_timer --test stand_crud` — green (exit 0 · lacks FAILED)
  - `for f in counter flight_booker timer crud; do git show 5371292e…` (markup-only) — green (exit 0 · last line markup-only)
  - `git diff --quiet 5371292e… -- Cargo.lock …` — green (exit 0)
  - `test -f …element_id.rs && test -f …stand_element_ids.rs && cat … | grep -c -E 'println!|…'` — green (exit 1 · last line 0)
  - `bash scripts/agent-run.sh boot` — green (exit 0 · contains "ready")
  - `bash scripts/agent-run.sh run stand` — green (exit 0 · contains "run.end"; artifact events.jsonl fresh)
  - `bash scripts/agent-run.sh logs | python3 … stand_ ok count` — green (last line 17)
  - `bash scripts/agent-run.sh logs | python3 … content-named key count` — green (last line 0)
  - `bash scripts/agent-run.sh cleanup` — green (exit 0)
  - `bash .github/scripts/ci-leg.sh fast` — red once (fmt diff in element_id.rs from a scripted edit) → rustfmt → green on `--only` and on the full re-run
  - `bash .github/scripts/ci-leg.sh doc` — green · `bash .github/scripts/ci-leg.sh a11y` — green
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (leg operator) — `hygiene: clean` (evidence/operator-15-hygiene.txt)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` (leg operator) — exit 0, pushed 5371292e..405a562b (evidence/operator-16-push.txt)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` (leg operator) — exit 0 · verdict: green (evidence/operator-17-ci.txt)
  - smoke: skipped — no boot-path touchpoint; markup-only UI change with no headful self-verify (the stand boot ran under all 17 stand checks)
- watches: none folded
- outcome basis: the operator pass's final state — commit 405a562b (the only commit since 5371292e) and its CI run CI#37443002592 recorded in `evidence/`; implement's P4 report (this conversation) for the deviations and gate history
- process hygiene: the agent-run stand session (started by gate entry 7) terminated by entry 11 (`status` reads `booted: false`); cargo / test binaries terminated — re-measured at this wrap: `ps` shows no agent-run, cargo or blitz-tests process
