
## 2026-10-10-upstream-sync-agent-surfaces — the second upstream sync: the merge base, Parley's pin form and the merged dependency set
**Section:** §Project Intent → the `Upstream sync:` line · §Stack and Technologies → Text and fonts, Accessibility, Markdown app (rdme), CI/CD · §Established Decisions → [Dependency pinning] · §Inherited Defaults → Framework · §Conventions → Licensing exceptions
**Change:**
- Upstream sync line: was merge base `23354585`, merge commit `f00b0216`, 2026-10-06; now DioxusLabs/blitz `main` at `7832c177`, merge commit `9462a7e4`, 2026-10-10 — the next sync's merge base.
- Parley: was "from git at a pinned rev" (Stack), "taffy and parley are git dependencies pinned by `rev`" ([Dependency pinning]) and "taffy and parley at pinned git revs" (Inherited Defaults); now parley is a registry version, "0.12", and taffy the one git dependency pinned by `rev`.
- Text and fonts gains ICU4X: icu_casemap, icu_locale_core, icu_properties and icu_segmenter "2.3" with writeable "0.6" — icu_casemap and writeable optional behind blitz-dom's default `text-transform-icu` feature, the other three unconditional dependencies of blitz-dom.
- Accessibility: accesskit_unix was 0.23.0, now 0.24.0; accesskit_android was 0.8.0, now 0.9.0. Markdown app: comrak was 0.55, now 0.56.
- CI/CD: the Python helper scripts now include upstream's `wpt_area_changes.py` with its test file and escher's `test_blitz_tests_targets.py`, which uses the stdlib `tomllib` — the `ci-scripts` leg needs Python 3.11 or later, no package added; the runner's Python version was not read.
- Licensing exceptions: `wasm_hello` was "has no license field"; now it takes its licence from the workspace.
**Why:** the merge of upstream's 61 commits took upstream's dependency set whole, with no `cargo update`. The sync line is what the next sync measures its merge base from. The coupling rule of [Dependency pinning] is unchanged: parley, skrifa and vello still move together, now through a registry version.
**Kept:** the Taffy rev is spelled in no master, so its bump amends nothing; accesskit "0.25" is still the manifest's string, the lock's patch bump stated nowhere.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/
