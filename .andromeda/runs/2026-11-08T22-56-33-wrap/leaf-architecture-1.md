@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `.andromeda/architecture.md` · directory lay
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `.andromeda/architecture.md` §Est
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `.andromeda/architecture.md` · `.claude/docs/stack.md` · `
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=19 class=stale
leaf-text: App and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script (since 2026-10-13-upstream-sync-polish-and-ship) and wpt set `publish = false`;
source: fct-usjlsvek «app and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script and wpt set `publish = false`, rdme `publish = true`»
replace-with: App and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script (since 2026-10-13-upstream-sync-polish-and-ship) and wpt set `publish = false`, rdme `publish = true`;
@leaf_edit n=2 leaf=.claude/docs/services/blitz-traits.md line=17 class=omission
leaf-text: `Viewport` (`color_scheme`, `window_size`, `hidpi_scale`, `zoom`)
source: fct-3p4vmlvn «`DevtoolSettings` and `Viewport`, its `scale()` hidpi_scale × zoom.»
replace-with: `Viewport` (`color_scheme`, `window_size`, `hidpi_scale`, `zoom`; its `scale()` hidpi_scale × zoom)
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=3 class=tree
leaf-text: _Extracted from `.andromeda/architecture.md` §Conventions
source: fct-tajqmgcr «→ detail conventions»
replace-with: _Extracted from the architecture master `doc:architecture` (the records store) §Conventions
@leaf_edit n=4 leaf=.claude/docs/conventions.md line=61 class=tree
leaf-text: - Architecture: `.andromeda/architecture.md` · directory layout
source: fct-6a34ag4h «- **Directory structure** (Cargo.toml:2-30; Cargo.toml:31; Cargo.toml:252-263):»
replace-with: - Architecture: `doc:architecture` (the records store) · directory layout
@leaf_edit n=5 leaf=.claude/docs/gotchas.md line=3 class=tree
leaf-text: _Documented architectural traps from `.andromeda/architecture.md` and the specialist plans.
source: fct-rg3ojpkx «→ detail established-decisions»
replace-with: _Documented architectural traps from the architecture master `doc:architecture` (the records store) and the specialist plans.
@leaf_edit n=6 leaf=.claude/docs/gotchas.md line=68 class=tree
leaf-text: - Decisions and rationale: `.andromeda/architecture.md` §Established Decisions
source: fct-rg3ojpkx «→ detail established-decisions»
replace-with: - Decisions and rationale: `doc:architecture` (the records store) §Established Decisions
@leaf_edit n=7 leaf=.claude/docs/services/blitz-dom.md line=3 class=tree
leaf-text: _Crate notes. Primary source: `.andromeda/architecture.md` (§Standard Contracts, §Existing Scopes, §Infrastructure Patterns)._
source: fct-a6nd5xwu «→ detail standard-contracts»
replace-with: _Crate notes. Primary source: the architecture master `doc:architecture` (§Standard Contracts, §Existing Scopes, §Infrastructure Patterns)._
@leaf_edit n=8 leaf=.claude/docs/services/blitz-dom.md line=44 class=tree
leaf-text: - `.andromeda/architecture.md` · `.claude/docs/stack.md` · `.claude/docs/conventions.md`
source: fct-a6nd5xwu «→ detail standard-contracts»
replace-with: - `doc:architecture` (the records store) · `.claude/docs/stack.md` · `.claude/docs/conventions.md`
@leaf_edit n=9 leaf=.claude/docs/services/blitz-paint.md line=3 class=tree
leaf-text: _Crate notes. Primary source: `.andromeda/architecture.md` (§Design Philosophy, §Standard Contracts Paint and shell, [Painting])._
source: fct-a6nd5xwu «→ detail standard-contracts»
replace-with: _Crate notes. Primary source: the architecture master `doc:architecture` (§Design Philosophy, §Standard Contracts Paint and shell, [Painting])._
@leaf_edit n=10 leaf=.claude/docs/services/blitz-paint.md line=35 class=tree
leaf-text: - `.andromeda/architecture.md` · `.claude/docs/services/blitz-dom.md`
source: fct-a6nd5xwu «→ detail standard-contracts»
replace-with: - `doc:architecture` (the records store) · `.claude/docs/services/blitz-dom.md`
@leaf_edit n=11 leaf=.claude/docs/services/blitz-shell.md line=3 class=tree
leaf-text: _Crate notes. Primary source: `.andromeda/architecture.md` (§Standard Contracts Paint and shell, §Infrastructure Patterns Shell loop, [Shell and windowing])._
source: fct-a6nd5xwu «→ detail standard-contracts»
replace-with: _Crate notes. Primary source: the architecture master `doc:architecture` (§Standard Contracts Paint and shell, §Infrastructure Patterns Shell loop, [Shell and windowing])._
@leaf_edit n=12 leaf=.claude/docs/services/blitz-shell.md line=34 class=tree
leaf-text: - `.andromeda/architecture.md` · `.claude/docs/services/blitz-dom.md`
source: fct-a6nd5xwu «→ detail standard-contracts»
replace-with: - `doc:architecture` (the records store) · `.claude/docs/services/blitz-dom.md`
@leaf_edit n=13 leaf=.claude/docs/services/blitz-traits.md line=3 class=tree
leaf-text: _Crate notes. Primary source: `.andromeda/architecture.md` §Standard Contracts (Provider traits)._
source: fct-a6nd5xwu «→ detail standard-contracts»
replace-with: _Crate notes. Primary source: the architecture master `doc:architecture` §Standard Contracts (Provider traits)._
@leaf_edit n=14 leaf=.claude/docs/services/blitz-traits.md line=33 class=tree
leaf-text: - `.andromeda/architecture.md` · `.claude/docs/services/blitz-dom.md`
source: fct-a6nd5xwu «→ detail standard-contracts»
replace-with: - `doc:architecture` (the records store) · `.claude/docs/services/blitz-dom.md`
