# Fan-out results — 2026-10-19-shared-fixtures (resumed wrap, P2)

Snapshot before: `tree d6facf8256e750d8edc65a5ba9548d93211f6c47 index 98bcc2b47cee018b68fa43cb5217aecc04ac65f2 head 0fbdda02242a07bf583c0ef82998c098fc162394`
Snapshot after: `tree 2d4b88c703ab2d36df6bad402a242d04debdf793 index 98bcc2b47cee018b68fa43cb5217aecc04ac65f2 head 0fbdda02242a07bf583c0ef82998c098fc162394`
Path check: `git diff-tree` printed exactly the seven `A {run_dir}/fanout-{doc}.md` lines; index and head unchanged.
Entity probe: 0 in every file. No fix owed, no twin. Every receipt count equals its file's.
Detectors sent: arch 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2 = 15, the drift-base's `doc:` count.

architecture: 2 proposals · fanout-architecture.md
  note: D-arch-decisions clean; no Retired item; the `cites19` re-points are the citation sweep's; the «in-file fixture(s)» wording at `:350` stays true.
  1. apply — check 1 «Accurate this-chunk addition» (the module, its contents and its 19 readers are the report's Symbols / APIs; Expected amendment «architecture §Conventions → Tests — the third shared module and the reader counts»). Re-derived: written as one bullet inside the standing `values · scopes` group below the §Existing Scopes table (a table cell takes no accretion, classification writing rule 1), a pointer to test-plan §3 for what the module states (writing rule 3), counts re-measured at this wrap (`common` 21 · `session_common` 25 · `fixtures` 19, by `grep -l '^mod {m};'`).
  2. apply — check 1 «Accurate this-chunk addition», dependent-of 1. Re-derived: one bullet inside the standing `values · conv-tests` group below §Conventions **Tests** — the third such module beside `common` and `session_common`; no chunk stamp (writing rule 7).
security-plan: 0 proposals · fanout-security-plan.md
  note: no detector fires; the six citations into moved test files are the citation sweep's.
design-system: 0 proposals · fanout-design-system.md
  note: D-design-tokens holds; the §Motion citation is the citation sweep's.
layout-templates: 0 proposals · fanout-layout-templates.md
  note: D-layout-surface holds; the two citations into moved test files are the citation sweep's.
test-plan: 10 proposals · fanout-test-plan.md
  note: D-tests-framework and D-tests-obs-harness otherwise hold; the three §1 `:27` in-file-fixture occurrences were found by reading; the other `:27` «in-file fixture» passages still hold.
  1. apply — check 1 «Accurate this-chunk addition» (Counts «shared modules 2 → 3», `:27` stale; Expected amendment test-plan §1). Citation `tests/blitz-tests/tests/fixtures/mod.rs:17-107` is a span of the new-text listing (first row 17, last row 107).
  2. apply — check 1 (Counts «shared modules» / «Readers 0 → 19», `:68` stale); `tests/all.rs` «a third file» → «a fourth file» follows from the count.
  3. apply — check 1 (`:102`/`:103` stale for the missing entry; Expected amendment §3 Crate-local test helpers). Re-derived from Symbols / APIs and Deviation 1; coordinates from the new-text listing.
  4. apply — check 1 (Counts «bootstrap» NOT YET MEASURED → measured; Expected amendment §3). Re-derived from the report's reading: the proposal's `cargo test -p blitz-tests --locked --no-run` is not the report's spelling (`cargo test --no-run`), so the report's is written. The section's marker is dropped: the reading measures it whole.
  5. apply — check 1 (Symbols / APIs, the three pages; Expected amendment §7).
  6. apply — check 1 (Counts «held network stand-ins», `:278` stale; citation-dispositions `changed test-plan.md:278` claim false settled here; Expected amendment §8). Upstream's `RecordingNetProvider` row unchanged.
  7. apply — check 1, dependent-of 1 (Counts heldnet, `:12` stale the name).
  8. apply — check 1, dependent-of 1 (Symbols / APIs: `password_fixture` and `boot_fixture` removed from `stand_snapshot_text`; Deviation 4). Read: `stand_snapshot_text.rs:349` boots `fixtures::password_labelled`, `:380` its in-file `file_fixture` (new-text listing rows 349 · 380).
  9. apply — check 1, dependent-of 1 (Symbols / APIs: `password_fixture` removed from `stand_diff`; `Screen::fixture` kept). Read: `stand_diff.rs:689` takes `fixtures::password_labelled`.
  10. apply — check 1, dependent-of 1 (Symbols / APIs: `covered_fixture` removed from `stand_mcp_tools`; Deviation 1). Read: `stand_mcp_tools.rs:515` boots `fixtures::covered` (new-text listing range 514-515).
obs-plan: 1 proposal · fanout-obs-plan.md
  note: D-obs-instrumentation and D-obs-pii not violated; the keyed contract untouched; `cites19` re-points are the sweep's.
  1. apply — check 1 «Accurate this-chunk addition» (Counts «shared modules» `:70` @c585 stale, «silence census», «declare none»; Expected amendment obs-plan §3 Logging stack). Re-derived: `:70`'s «their shared modules» left as written, since it already covers every shared module. The census lands as one bullet inside the standing `absence · logs` group after `:78`, with no chunk stamp (writing rule 7).
a11y-plan: 0 proposals · fanout-a11y-plan.md
  note: D-a11y-surface and D-a11y-obs-schema hold; the citations are the sweep's.
  1. (raised by main — check 5, citation-dispositions `changed .andromeda/a11y-plan.md:285@c11036`, claim false) apply — routine, check 1 «Accurate this-chunk addition». §… (the `stand_mcp_masked` MCP masked-value row) — «the input a test-only element of `stand_mcp_masked`'s fixture» → the shared page `fixtures::password_named`, citation `tests/blitz-tests/tests/stand_mcp_masked.rs:23-30` → `tests/blitz-tests/tests/fixtures/mod.rs:37-44` (new-text listing row 37-44).

## Expected amendments reconciliation (check 5)
- test-plan §1 → test-plan 1, 7–10. · test-plan §3 → test-plan 3, 4. · test-plan §7 / §8 → test-plan 5, 6.
- architecture §Conventions → Tests → architecture 2 (and 1 for §Existing Scopes).
- obs-plan §3 Logging stack → obs-plan 1.
- security-plan citations → no edit owed: the citing lines `security-plan.md:101` and `:103` cite `font_face_sniffed_source.rs:50-99` / `:50-73` / `:75-99`, which read `unmoved` (the file's tests stand at 50-73 and 75-99, read), and «every request held by a hand provider» stays true of `HeldNet`. The other four rows (`:121`, `:125`, `:133`, `:362`) were re-pointed by the sweep.
- design-system §Motion → the sweep re-pointed `stand_settle.rs:279-307` to `247-275` (`design-system.md:290`); `:291` unmoved.

## Checks 2–6
- Cross-contradiction: none — test-plan 1/2/3 state the same three modules and counts; architecture 1/2 point to test-plan §3.
- Intent-consistency: scope record none (`gate.py scope` clean, recorded 0); Deviations 1–5 justified in the report and inside the plan's goal.
- Absence needs evidence: obs-plan 1's 0-site census rests on the report's gate entries (`grep -c … fixtures/mod.rs` exit 1 · 0; the nineteen + module → 4).
- Disproved claims: the report lists none.
- Boundary widening: none — the stand-in is in process, adds no crossing (plan Provenance; report «No port, socket, listener, env var, IPC method, endpoint or export added»).

Rejected for a source the report does not carry: 0. Escalations: 0.
