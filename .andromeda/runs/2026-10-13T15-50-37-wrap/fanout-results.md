# Fan-out results — 2026-10-13-upstream-sync-polish-and-ship (resumed 2026-10-13)

Batch: one snapshot pair — before `tree e5568071e1b30944acedc0082cbbd3c0925f1a4a index d875636cc515b426a80ffe6c7056d4dce783385f head a6c24d24ef661ecc7fadaade79cfb56d703c5e96`, after `tree 0d86132419b8587cb142a0079f240adf465f202f`, same index and head; `git diff-tree` printed exactly the seven `A …/fanout-{doc}.md` lines. Every file split at its `- detector:` items, each count equal to its receipt; entity probe 0 in all seven; no fix, no twin. Detectors sent: 15 (arch 2 · security 3 · design 1 · layout 1 · tests 3 · obs 3 · a11y 2), the drift-base's whole set.

Rejected before the checks for a source the report does not carry: 0 (every coordinate is a report bullet's, a New-text row's or a span between two rows of one file; Cargo.toml:106 and :36 · :46-54 and blitz-vibey-script/Cargo.toml:12 are New-text added ranges; the test-plan per-file counts 10 and 2 are the listing's own test heads).

Inputs: the report (with its addendum, inputs#I4, the overseer's directive) and `citation-dispositions.md` (20 rows `claim false — raised at Validate`).

## architecture: 14 proposals · fanout-architecture.md
note: the Project Intent `Upstream sync:` line (:205) and the Front page item (:210) left to main; the 7832c177 mentions at :87 are dated history and stand; no port, socket, listener, env var or crate added; the crate-private arrivals not proposed.
1. apply — check 1 (Accurate this-chunk addition, routine; also the plan's Expected amendments name it — [Dependency pinning], Taffy registry 0.15, no git dependency, P5-approved inputs#I3) · §Established Decisions → [Dependency pinning]: taffy a registry "0.15" dependency (0.15.0), no git dependency left. Settles citation row arch:73.
2. apply — with 1 (dependent) · §Stack → Layout row: taffy "0.15" from crates.io, the eight features. Settles rows arch:48 (Cargo.toml:105-115).
3. apply — check 1 (Accurate; Expected amendments) · §Stack → Workspace package row: version and the nine in-repo `=` pins `0.3.0-beta.3`. Settles rows arch:45 ×2.
4. apply — with 1/3 (dependent) · §Inherited Defaults → Framework: engine pins `=0.3.0-beta.3`, taffy registry 0.15. Settles rows arch:219 ×2.
5. apply — check 1 + check 6 (report Spec claims disproved, arch:91; Expected amendments name it) · §Established Decisions → [DOM semantics]: the "mappings intentionally not cleared" decision retired; a removed node and its descendants are dropped and their mappings cleared (#1185).
6. apply — with 5 (dependent; Expected amendments name the bridge's "a removed node dropped") · §Standard Contracts → Dioxus DOM bridge: the drop-on-removal contract; also retires the line's "a node a re-render removes is DETACHED (parent `None`) and keeps resolving" (citation row arch:136@c7176).
7. apply — check 1 + check 6 (report Spec claims disproved, arch:98) · §Established Decisions → [Dioxus integration]: `DioxusDocument::new` adds no sheet; the default sheet via `DocumentConfig::default()`.
8. apply — with 7 (dependent; disproved arch:92) · §Established Decisions → [HTML parsing]: `HtmlDocument` uses the configured sheets as given.
9. apply, trimmed — check 1 (Accurate this-chunk) for the listed field's shape; the method list is Registry over-reach (routine reject of that part: the contract lists the `DocumentConfig` fields, never the `add_`/`remove_user_agent_stylesheet` methods, `git grep` of architecture.md 0) · §Standard Contracts → Document core: `DocumentConfig.ua_stylesheets` is `Vec<UserAgentStylesheet>`, defaulting to the default sheet by a hand-written `Default`; `UserAgentStylesheet` named as its element type.
10. apply — check 1 (the Events contract enumerates `UiEvent` variants at :123) · §Standard Contracts → Events: `UiEvent::PointerLeave` / `DomEventKind::PointerLeave`, `EventDriver::handle_pointer_leave`.
11. apply — check 1 + check 5 (Expected amendments: upstream's `is_bool_attr`, 28 names, our constant retired) · §Standard Contracts → Dioxus DOM bridge: the falsy clear reads `is_bool_attr`, 28 names, `inert` added. Settles rows arch:136@c1679 · @c1736.
12. apply — with 11 (dependent) · §Existing Scopes → blitz-tests row: the 28-name list.
13. reject — check 1, Registry over-reach (routine): the bridge contract names no re-exported free function (`synthetic_click_event` 0 in architecture.md); `synthetic_form_event` is a public symbol inside a registered crate, unused by escher.
14. apply — check 1 (Accurate this-chunk) · §Inherited Defaults → Publishability: blitz-vibey-script `publish = false`.
15. (raised by main) apply — check 5, Expected amendments #1, routine (report Cross-project; Counts, `sync-pin`) · §Project Intent → `Upstream sync:` line (:205): last merged `434e06615d5f47a3df92c0f94e916df088fdcf4d`, merge `a6c24d24`, 2026-10-13. The :87 mentions stay (dated refs of the flagged diffs).
16. (raised by main) apply — check 5, Expected amendments #2, routine (report Files `README.md:48`; Outcome "the eleven driver commands as tools" 1, the stale wording 0) · §Project Intent → Front page (:210): eleven tools, the screenshot no longer "still to come".
17. (raised by main) apply — check 5, citation row arch:64 `claim false`, routine (report Dependencies: blitz-tests gains dev-dependency `cursor-icon`) · §Stack → Testing row: `cursor-icon` among the blitz-tests dev-dependencies.
18. (raised by main) apply — directive inputs#I4 item 1, routine (a doubled word, no claim moves) · architecture.md:257: «the hand-written written forms» → «the hand-written forms».
19. (raised by main) apply — directive inputs#I4 item 2, no playbook rule, settled by the operator's recorded direction (the invocation: "carry its three items"); the rule proposed at the card · the founder's word respelled at 11 sites — the list below.

## security-plan: 3 proposals · fanout-security-plan.md
note: D-security-auth not triggered; blitz-vibey-script `publish = false` beside "the blitz-tests crate is not published" left to main (main: no change — :247 states the blitz-tests crate's own manifest; publishability is arch's Inherited Defaults, amendment architecture 14); the directive's items are main's.
1. apply — check 1 + check 5 (Expected amendments: §Input Validation, the 28-name predicate) · §Input Validation → Dioxus mutations falsy-boolean row: `is_bool_attr`, 28 names. Settles rows security-plan:142 ×2.
2. apply — check 1 + check 5 (Expected amendments: §Dependency Security Pinning, no git dependency) · §Dependency Security → Pinning: no git dependency; taffy registry 0.15.0. Settles row security-plan:237.
3. apply — check 1 (Accurate this-chunk; the section records the 2026-10-10 sync's delta) · §Dependency Security → Pinning: the 2026-10-13 sync's dependency delta, as measured at evidence/lock.md.
4. (raised by main) apply — directive inputs#I4 item 1, routine (a table row split by a bare pipe; no claim moves) · security-plan.md:101: the code span `` `font-face|font_face` `` re-spelled with no bare pipe, the row then counting 4 pipes like :100 and :102.
5. (raised by main) apply — directive inputs#I4 item 2, as architecture 19 · the founder's word respelled at 18 sites — the list below; :101 follows sp-amend :357.

## design-system: 0 proposals · fanout-design-system.md
note: D-design-tokens holds; :339 named as true by a changed mechanism; the nine `default.css:{N}` citations to the sweep.
1. (raised by main) apply — check 5 (citation row design-system:339 `claim false`) + check 6 (report disproved, design-system:339), routine · §Surface: desktop-native → Tokens (platform-specific), the UA-sheet line (:339): every document gets the default sheet through `DocumentConfig::default()` (config.rs:73-91), not a `DioxusDocument` call.

## layout-templates: 0 proposals · fanout-layout-templates.md
note: D-layout-surface holds; no surface added; citations to the sweep.

## test-plan: 10 proposals · fanout-test-plan.md
note: D-tests-framework and D-tests-obs-harness hold; §3 agent-run Proof re-read and `synthetic_form_event`'s absent test not proposed.
1. apply — check 1 (Accurate; Expected amendments §1 inventory) · §1 Coverage scope (dioxus-native-dom): 67 unit tests in nine files.
2. apply — with 1 (dependent) · §1 observed-absent row: mutation_writer.rs among the files holding tests; stylo_taffy still none.
3. apply — check 1 + check 6 (report disproved test-plan:99) · §3 Crate-local test helpers: a re-render drops the node; `remove_node_if_unparented` gone from the test. Settles row test-plan:99@c731.
4. apply — check 1 (Accurate) · §1 Coverage scope (tests/blitz-tests): the five upstream checks and `cursor-icon`.
5. apply — check 1 (Accurate; Expected falsy-attrs :26 · :330), the "not stated" clause applied as `not measured here` (Apply's epistemic form) · §1 Dioxus integration: the bridge clears by `is_bool_attr`, 28 names.
6. apply — check 1 (Expected amendments §2 Directory pattern) · §2 Directory pattern: 118 files, 117 targets, 128 executables.
7. apply — check 1 (Expected §9 / Counts, CI-scripts 79) · §4 CI workflows and leg script: 79 tests, test_wpt_area_changes.py 5.
8. apply — with 7 (dependent) · §1 CI Python scripts: 79 tests.
9. apply — with 7 (dependent) · §4 CI Python script: test_wpt_area_changes.py 5 tests. Settles row test-plan:144.
10. apply — check 1 (Expected §9) · §9 Local baseline: 179 · 893 · 0 · 12, executables 128.

## obs-plan: 1 proposal · fanout-obs-plan.md
note: feature wiring holds (16 / 10, evidence/features.md); obs-plan.md:148's `service.version=0.3.0-beta.2` is a dated verbatim capture (2026-10-07-driver-command-spans) and stands — main agrees: no change.
1. apply — check 1 + check 5 (Expected amendments: obs §3, the five targets joined to the no-subscriber census) · §3 Logging stack: the five upstream targets, 0 env · 0 print · 0 subscriber.

## a11y-plan: 1 proposal · fanout-a11y-plan.md
note: D-a11y-obs-schema holds; no new interactive element; adapter absence holds (`a11y-plan.md:14`).
1. apply — check 1 + check 5 (Expected amendments: a11y §5, the 28-name list) · §5 Script and framework exposure: `is_bool_attr`, 28 names, a falsy `inert` now cleared. Settles rows a11y-plan:183 ×2.

## Check 2 · 3 · 4 · 5 · 6
- Check 2, cross-contradiction: none — architecture 6 and 11 edit one contract in the same direction and land as one edit.
- Check 3, intent-consistency: the scope record is empty (`scope: clean — … recorded 0`); every proposal records the merge the entry and plan intend. No divergence.
- Check 4, absence: architecture 13's reject rests on `synthetic_click_event` 0 in architecture.md (read); architecture 9's trimmed part on `add_user_agent_stylesheet` 0 there.
- Check 5, Expected amendments: all twelve entries matched — `Upstream sync:` (architecture 15) · Front page (16) · pinning/stack/inherited (1–4) · DOM semantics (5) · bridge (6, 11) · security (1, 2) · a11y §5 (1) and §1 (no change: absence holds) · test-plan §1 §2 §9 (1, 4, 6, 7, 10) · obs §3 (1) · design-system (no value moved; :339 raised) · layout-templates (not carried, no figure moved) · citations (the sweep). The 20 citation rows `claim false`: arch:45 ×2 → 3 · arch:48 → 2 · arch:64 → 17 · arch:73 → 1 · arch:92 → 8 · arch:98 → 7 · arch:136 ×3 → 6, 11 · arch:219 ×2 → 4 · sp:142 ×2 → 1 · sp:237 → 2 · ds:339 → design-system 1 · tp:99 → 3 · tp:144 → 9 · a11y:183 ×2 → 1.
- Check 6, disproved claims: arch:91 → architecture 5 · arch:98, :92 → 7, 8 · arch:121's field list → 9 · design-system:339 → design-system 1 · test-plan:99 → 3 · the plan's gate entry 50 → routed: a plan defect, recorded by the report and by inputs#I4 item 3 with its `0` left uncorrected (overseer1 holds it as ES43; the `/tmp` controls path as ES44); no master states it.

## The founder's word, respelled (inputs#I4 item 2)
Master lines with «relayed by the overseer», read by `grep -c` before this pass: security-plan 14, architecture 7. The ten amendments (bf973d33): sp-amend :325 driver-cli socket «Сокет несёт команды» · :336 driver-cli CLI and schema «Бинарник приложения, путь» · «Именованные флаги» · «Четыре класса» · :357 driver-cli `@font-face` «Утвердить + проверка» · :385 snapshot-states adapter states «Ратифицировать» (confirm «Подтвердить всё») · :397 mcp-surface crossing «Ратифицировать как есть» (confirm «Подтвердить всё») · :406 mcp-surface `serde_json` «serde_json, только чтение», span «Span с tool и ending», trial «Записать по форку 3»; arch-amend :436 socket «Сокет несёт команды» · :534 adapter states «Ратифицировать» · :555 MCP crossing «Ратифицировать как есть» · :564 «serde_json, только чтение», «Span с tool и ending». Each changed site takes the form «relayed verbatim by the overseer as his chosen option label(s) «…»; the English wording is the overseer's».

Changed (29):
- security-plan.md:25 @c1187 — the socket widened to calls and answers → sp-amend :325
- security-plan.md:28 @c474 — the MCP result's content crossing → sp-amend :397
- security-plan.md:78 @c1763 — the answer carries what the screen reads → sp-amend :325
- security-plan.md:81 @c1009 — `start` · `status` · `stop`, a further widening → sp-amend :336
- security-plan.md:81 @c5868 — the schema an external-input surface "with the crossing", its MCP carrier «Ратифицировать как есть» → sp-amend :336 and :397
- security-plan.md:81 @c8643 — the `tool_call` span → sp-amend :406
- security-plan.md:82 @c4314 — the command line, a new external input surface → sp-amend :336
- security-plan.md:83 @c4074 — the MCP bounds and the echo → sp-amend :397
- security-plan.md:101 @c308 — the `@font-face` source → sp-amend :357
- security-plan.md:123 @c3435 — the adapter's third class, states → sp-amend :385
- security-plan.md:123 @c4785 — a hosted `snapshot`'s text over the socket → sp-amend :325
- security-plan.md:123 @c5238 — MCP results as the id's third exit → sp-amend :397
- security-plan.md:123 @c7306 — a diff over the socket → sp-amend :325
- security-plan.md:124 @c3501 — names over the socket → sp-amend :325
- security-plan.md:125 @c1726 — the adapter states widening → sp-amend :385
- security-plan.md:243 @c747 — `serde_json` a reader only → sp-amend :406
- security-plan.md:287 @c575 — the live trial recorded on «Записать по форку 3» → sp-amend :406
- security-plan.md:393 @c5831 — the `tool_call` span → sp-amend :406
- architecture.md:87 @c2846 — the accessibility builder writes the three states → arch-amend :534
- architecture.md:101 @c764 — the socket's crossing → arch-amend :436
- architecture.md:101 @c2557 — `serde_json` → arch-amend :564
- architecture.md:101 @c3405 — the `tool_call` span → arch-amend :564
- architecture.md:101 @c4516 — the MCP crossing and its bounds → arch-amend :555
- architecture.md:122 @c4819 — the adapter's states → arch-amend :534
- architecture.md:133 @c22837 — `serde_json` → arch-amend :564
- architecture.md:133 @c41006 — the MCP bounds → arch-amend :555
- architecture.md:133 @c43646 — the `tool_call` span → arch-amend :564
- architecture.md:146 @c1215 — the socket widened to calls → arch-amend :436
- architecture.md:146 @c1960 — the MCP crossing → arch-amend :555

Left (8), one reason each:
- security-plan.md:25 @c724 — the snapshot-states inside-the-crossing ruling; its amendment, sp-amend :375, keeps the short spelling and is not one of the ten.
- security-plan.md:78 @c3426 — the same inside-the-crossing ruling (sp-amend :375), not one of the ten.
- security-plan.md:226 @c1064 — the orphaned-host answer (the idle expiry): arch-amend :436 names it with no option label, so there is no label to name.
- architecture.md:87 @c1884 — the scrolling-box bounds-and-hit ruling (arch-amend :502), not one of the ten.
- architecture.md:101 @c3913 — the orphaned-host answer, as security-plan.md:226.
- architecture.md:124 @c4223 — the hit ruling (arch-amend :502), not one of the ten.
- architecture.md:133 @c31097 — `type` replacing, the founder's answer (arch-amend :479), not one of the ten.
- architecture.md:136 @c13146 — the scrolled-box bounds ruling (arch-amend :502), not one of the ten.

Folded at cascade step 2 (after the founder's answers, inputs#I5 — rule A appended, its class now routine): the sweep's `relayed-short` pattern reached four more sites outside the two masters the directive named, each copying one of the ten, each respelled to the amendment's form — obs-plan 2 (raised by main) `obs-plan.md:107` → arch-amend :564 / sp-amend :406 («Span с tool и ending») · a11y-plan 2 (raised by main) `a11y-plan.md:46` → arch-amend :534 / sp-amend :385 («Ратифицировать») · a11y-plan 3 (raised by main) `a11y-plan.md:275` → arch-amend :555 / sp-amend :397 («Ратифицировать как есть») · test-plan 11 (raised by main) `.andromeda/registries/contracts/test-plan/session-lifecycle.md:7` → arch-amend :436 / sp-amend :325 («Сокет несёт команды»).

After the pass the line counts read (to be measured at apply): security-plan short 14 → 3 lines (:25, :78, :226), architecture 7 → 5 lines (:87, :101, :124, :133, :136) — each left line still carrying a site listed above.
