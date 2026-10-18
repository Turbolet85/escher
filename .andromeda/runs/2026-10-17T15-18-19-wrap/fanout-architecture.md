proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Names
    change: 'Drop `bump` and its citation `apps/bump/Cargo.toml:2` from the crates list; the crate leaves the workspace and its `members` with upstream''s `23b9b26a9` "Delete the bump crate", and nothing is added to the registry.'
    sidecar: 'Occupied Resources Names: the `bump` crate removed (deleted upstream at 23b9b26a9, merged by 3eda07bc).'
    rationale: 'Report Changes, Crates / modules: "`apps/bump` (crate `bump`) leaves the workspace and its `members` (entries 6 · 7)... No crate is added"; Files: `apps/bump/Cargo.toml` (D), `apps/bump/src/main.rs` (D); Counts `bump` lists arch `:154` ×2 changed.'
    basis: .andromeda/architecture.md:154
  - detector: D-arch-resources
    severity: warning
    section: §Stack and Technologies → Language / runtime
    change: 'Drop `bump` and `apps/bump/Cargo.toml:4` from the edition-2024 crate list.'
    sidecar: 'Stack Language/runtime: bump leaves the edition-2024 list (crate deleted upstream).'
    rationale: 'Report Counts `bump`: "Changed in architecture: `:44` ×2 (the edition-2024 list)"; Crates / modules: bump leaves the workspace.'
    basis: .andromeda/architecture.md:44
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Stack and Technologies → Release tooling
    change: 'Remove the Release tooling row (bump: toml_edit 0.22, semver 1): bump left the workspace, and toml_datetime 0.6.11, toml_edit 0.22.27 and toml_write 0.1.2 left the lock with it.'
    sidecar: 'Stack: the Release tooling (bump) row removed; its toml crates left the lock with it.'
    rationale: 'Report Dependencies: "Leaving with bump: `toml_datetime` 0.6.11, `toml_edit` 0.22.27, `toml_write` 0.1.2"; Counts `bump` arch `:63` ×3 and `tomlc` arch `:63` changed; Expected amendments: "the Release tooling (bump) row leaves".'
    basis: .andromeda/architecture.md:63
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Established Decisions → [Release versioning]
    change: 'Keep the prepatch rule (publish-browser.yml:21-26); restate "blitz packages versioned together" as the one workspace version and the in-repo `=` pins (Cargo.toml:35; Cargo.toml:45-53), dropping "bump versions … anyrender packages together" and its `apps/bump/src/main.rs` citations — no in-repo release tool remains since upstream deleted bump.'
    sidecar: '[Release versioning]: the bump-tool clause and its apps/bump citations removed; versioning together rests on the workspace version and in-repo pins.'
    rationale: 'Report Counts `bump`: "`:76` ×4 ([Release versioning]: \"bump versions blitz packages together\", citing `apps/bump`)" changed; Dependencies: the workspace version moved with "its nine in-repo `=` pins" (New text Cargo.toml 35 · 45-53).'
    basis: .andromeda/architecture.md:76
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → CLIs
    change: 'Drop the clause "bump: `blitz` or `anyrender` then a semver version (apps/bump/src/main.rs:67-91)".'
    sidecar: 'Standard Contracts CLIs: bump''s argument contract removed with the crate.'
    rationale: 'Report Counts `bump`: "`:138` ×2 (bump''s arguments)" changed; Files: `apps/bump/src/main.rs` (D).'
    basis: .andromeda/architecture.md:138
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → directory tree
    change: 'Drop `bump/` from the `apps/` line of the tree (browser/, browser/persistence/, readme/ remain).'
    sidecar: 'Infrastructure Patterns tree: apps/bump/ removed.'
    rationale: 'Report Counts `bump`: "`:171` (the directory tree''s `bump/`)" changed; Expected amendments: "the tree''s `apps/bump/` ... leave".'
    basis: .andromeda/architecture.md:171
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Deployment model
    change: 'Drop "bump rewrites package, workspace and workspace-dependency versions with toml_edit (apps/bump/src/main.rs:33-62)".'
    sidecar: 'Infrastructure Patterns Deployment model: the bump/toml_edit version rewrite removed.'
    rationale: 'Report Counts `bump` arch `:180` ×2 and `tomlc` arch `:180` changed; Dependencies: toml_edit leaves with bump.'
    basis: .andromeda/architecture.md:180
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Project Intent → Apps and demonstrators
    change: 'Drop "bump \"Utility to aid publishing blitz\" (apps/bump/Cargo.toml:5)".'
    sidecar: 'Project Intent Apps and demonstrators: bump removed.'
    rationale: 'Report Counts `bump`: arch `:213` ×2 changed ("each describes or cites `apps/bump`").'
    basis: .andromeda/architecture.md:213
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Project Intent → Growth model and how functionality is added
    change: 'Restate "blitz packages versioned together" citing the workspace version and its in-repo `=` pins (Cargo.toml:35; Cargo.toml:45-53) in place of `apps/bump/src/main.rs:5-23; :95-110`.'
    sidecar: 'Project Intent Growth model: versioning-together citation moved off the deleted bump crate.'
    rationale: 'Report Counts `bump`: arch `:216` ×2 changed; Files: `apps/bump/src/main.rs` (D).'
    basis: .andromeda/architecture.md:216
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Inherited Defaults → Publishability
    change: 'Replace "versioned together for release by bump, \"Utility to aid publishing blitz\" (apps/bump/src/main.rs:33-62; apps/bump/Cargo.toml:5)" with: versioned together through the workspace version and the in-repo `=` pins (Cargo.toml:45-53), no in-repo release tool since upstream deleted bump.'
    sidecar: 'Inherited Defaults Publishability: release versioning no longer attributed to bump.'
    rationale: 'Report Counts `bump`: arch `:225` ×3 changed; Crates / modules: bump leaves the workspace.'
    basis: .andromeda/architecture.md:225
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes
    change: 'Delete the `| bump | apps/bump | Release version bump tool (apps/bump/Cargo.toml:2-5) |` row.'
    sidecar: 'Existing Scopes: the bump row removed.'
    rationale: 'Report Counts `bump`: "`:262` ×4 (the crate table''s bump row)" changed; Expected amendments: "the crate table''s bump row leave".'
    basis: .andromeda/architecture.md:262
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Mutation, query and CSSOM
    change: '`build_accessibility_tree` (signature and three callers unchanged) names a node from four sources: after the `<label>` → input loop, `aria-labelledby` clears `labelled_by`, pushes the referenced nodes (unknown ids ignored) and sets the computed label (`set_label`, cycle-guarded through `write_text_alternative`), outranking a `<label>` (accname 2B over 2D); the builder also reads, as upstream wrote them, `aria-describedby` (referenced nodes as `described_by`), `aria-description` (its text into AccessKit''s description), `aria-expanded` (boolean), `aria-disabled` (`"true"` → `set_disabled`) and `aria-level` with tag and role defaults, then `set_html_tag`; our trimmed `aria-label` and closed-set `aria-selected` · `aria-pressed` · `aria-invalid` reads are kept and upstream''s own `aria-selected` (`parse::<bool>`) and unfiltered `aria-label` reads dropped, so one read per attribute (packages/blitz-dom/src/accessibility.rs:118-125; :244-252; :254-264; :267-295; :362-374; :376-383).'
    sidecar: 'Standard Contracts Mutation, query and CSSOM: the accessibility builder gains upstream''s aria-labelledby (fourth name source, outranks <label>), aria-describedby, aria-description, aria-expanded, aria-disabled, aria-level; one read per attribute kept.'
    rationale: 'Report Symbols / APIs, `packages/blitz-dom/src/accessibility.rs` (the six reads and helpers with their ranges); Counts `aria6`: arch `:122` ×2 changed ("naming a node from three sources" — `aria-labelledby` is a fourth, and it outranks a `<label>`), and "0 rows state `aria-describedby`, `aria-description`, `aria-level` or `aria-disabled` as builder reads, so those four are new text in arch §Standard Contracts"; Reverted / negative API facts.'
    basis: .andromeda/architecture.md:122
  - detector: D-arch-resources
    severity: escalate
    section: §Standard Contracts → Mutation, query and CSSOM
    change: 'Where the clause lists what the platform accessibility adapter receives (ids as `author_id`, accessible names, and the third class, states), add: `aria-labelledby` text reaches it inside the names class, and a fourth class, description — `aria-description`''s own page text (free, unbounded) and `aria-describedby`''s referenced nodes as `described_by` — to the adapter only (the snapshot reads no description), a widening by upstream merge ratified as upstream''s behaviour on the founder''s «Утвердить оба (Рекомендую)» (2026-10-17, inputs#I2); `aria-expanded` · `aria-disabled` · `aria-level` are closed values inside the states class.'
    sidecar: 'Standard Contracts: the adapter crossing gains the description content class (aria-description / aria-describedby), ratified as upstream''s behaviour by the founder 2026-10-17.'
    rationale: 'Report Coverage of new surfaces: "`aria-describedby` · `aria-description` → AccessKit description / `described_by`, to the platform adapter only ... PII: page text to the adapter, ratified as upstream''s behaviour (inputs#I2), the founder''s confirm due on P2''s card"; Decisions & corrections: the founder''s word; the arch line states the adapter''s content classes as ids, names and states. Escalate: a boundary widening is confirmed on the founder''s card before apply.'
    basis: .andromeda/architecture.md:122
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Dioxus DOM bridge
    change: 'At the snapshot node''s `name` (its non-empty `label`, else its `labelled_by` names), add that `label` now carries the computed `aria-labelledby` text, which outranks a `<label>`; the snapshot reads no description, and `enabled` still reads the `disabled` attribute, so `aria-disabled` moves no snapshot reading (packages/dioxus-native-dom/src/snapshot.rs:51-52; :154).'
    sidecar: 'Dioxus DOM bridge: snapshot name gains the aria-labelledby source; aria-disabled and the description leave the snapshot unchanged.'
    rationale: 'Report Counts `aria6`: "`architecture.md:136` @c12639 (a snapshot node''s `name` ...). The letter holds, but the computed `aria-labelledby` label now reaches `label`, so a name source is added"; Coverage of new surfaces (snapshot.rs:51-52,154).'
    basis: .andromeda/architecture.md:136
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Test harness
    change: 'blitz-test-harness exports `Key`, `Modifiers` and `NamedKey` — re-exports of keyboard-types 0.8''s `Key`, `Modifiers` and `NamedKey`, the three types a key press is named with (`pub use keyboard_types::{Key, Modifiers, NamedKey};`, packages/blitz-test-harness/src/lib.rs:32), so a crate depending on the harness names a key without a dependency of its own.'
    sidecar: 'Standard Contracts Test harness: the keyboard re-export gains NamedKey (keyboard-types 0.8).'
    rationale: 'Report Symbols / APIs: "`packages/blitz-test-harness/src/lib.rs:32` — the re-export is now `pub use keyboard_types::{Key, Modifiers, NamedKey};`"; Counts `kbt` and `reexp`: arch `:130` ×2 changed ("the two types `press_with` takes" — now three).'
    basis: .andromeda/architecture.md:130
  - detector: D-arch-resources
    severity: warning
    section: §Established Decisions → [Driver session]
    change: 'The key types `press` needs come through the harness''s re-exports `Key`, `Modifiers` and `NamedKey`, not a dependency of its own — `keyboard_key` maps each of the 11 named keys to `Key::Named(NamedKey::…)`, Space staying `Character(" ")` (packages/escher-driver/src/execute.rs:490-494; :496-501).'
    sidecar: '[Driver session]: the driver''s key types through the harness re-exports now include NamedKey.'
    rationale: 'Report Counts `reexp`: "`architecture.md:101` @c3292 (the driver''s key types come through the harness''s re-exports)" changed; Symbols / APIs, `execute.rs` — `keyboard_key` added 490-494 · 496-501, no `keyboard-types` dependency of its own (entry 11).'
    basis: .andromeda/architecture.md:101
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → dioxus-native-dom
    change: 'Add the crate-private module `keyboard_compat` (upstream''s; `mod keyboard_compat;`, packages/dioxus-native-dom/src/lib.rs:20; packages/dioxus-native-dom/src/keyboard_compat.rs:1-564), converting keyboard-types 0.8 into Dioxus 0.7''s types for `events.rs` (`convert_modifiers`, `convert_location`, `convert_key`, `convert_code` `pub(crate)`, `convert_named_key` private), excluded from the coverage leg by filename until upstream retires it.'
    sidecar: 'Existing Scopes dioxus-native-dom: the crate-private keyboard_compat module (upstream''s keyboard-types 0.7/0.8 bridge) added.'
    rationale: 'Report Crates / modules: "dioxus-native-dom gains the private module `keyboard_compat` (upstream''s)"; Symbols / APIs (keyboard_compat.rs 1-564, lib.rs:20); Counts `kbt`: "0 rows name `keyboard_compat`, so its text is new: ... the module in arch §Existing Scopes'' dioxus-native-dom row".'
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → CI contracts
    change: 'The `coverage` leg''s two `cargo llvm-cov` lines (the `--lcov` export and the `report`) take `--ignore-filename-regex ''keyboard_compat\.rs$''`, excluding upstream''s temporary keyboard-types 0.7/0.8 bridge until upstream retires it (.github/scripts/ci-leg.sh:41-42; .github/scripts/ci-leg.sh:44-45), pinned by `test_coverage_leg_reaches_the_floor_on_each_crate` (.github/scripts/test_ci_workflows.py:327); `coverage_floor.py` and the three floors unchanged.'
    sidecar: 'CI contracts: the coverage leg excludes keyboard_compat.rs by filename; floors unchanged.'
    rationale: 'Report Schema / config: ci-leg.sh added 41-42 · 44-45 and test_ci_workflows.py:327; Counts `cov`: "`architecture.md:141` @c2609 (the coverage leg''s command)" changed.'
    basis: .andromeda/architecture.md:141
  - detector: D-arch-decisions
    severity: warning
    section: §Stack and Technologies → Windowing and input
    change: 'keyboard-types "0.8" (Cargo.toml:145; lock 0.8.3), upstream''s move; Dioxus 0.7.3 still carries its own keyboard-types 0.7, so two versions stand in the graph and a module that glob-imports `dioxus::prelude::*` spells the workspace crate `::keyboard_types`.'
    sidecar: 'Stack: keyboard-types 0.7 → 0.8 (upstream); Dioxus keeps 0.7, two versions in the graph.'
    rationale: 'Report Dependencies: "keyboard-types `\"0.7\"` → `\"0.8\"` (lock 0.8.3) — upstream''s; Dioxus 0.7.3 still carries its own keyboard-types 0.7, so two versions now stand in the graph"; Counts `kbt`: arch `:52` changed; Deviations: the `::keyboard_types` spelling.'
    basis: .andromeda/architecture.md:52
  - detector: D-arch-decisions
    severity: warning
    section: §Stack and Technologies → Parallelism and misc
    change: 'test-that "0.6.0" (Cargo.toml:198), reviewed by hand: Apache-2.0, the same repository, no build script, proc-macro flag unchanged.'
    sidecar: 'Stack: test-that 0.5.2 → 0.6.0 (blitz-tests dev-dependency, reviewed).'
    rationale: 'Report Dependencies: "test-that and test-that-macro 0.5.2 → 0.6.0 (dev-dependencies of blitz-tests). Reviewed by hand"; Counts `testthat`: arch `:59` changed.'
    basis: .andromeda/architecture.md:59
  - detector: D-arch-decisions
    severity: warning
    section: §Stack and Technologies → Workspace package
    change: 'Version "0.3.0-beta.4" (Cargo.toml:35); in-repo crates pinned `=0.3.0-beta.4` (Cargo.toml:45-53).'
    sidecar: 'Stack: workspace version and in-repo pins 0.3.0-beta.3 → 0.3.0-beta.4 (upstream''s release bump).'
    rationale: 'Report Dependencies: "The workspace version 0.3.0-beta.3 → 0.3.0-beta.4, with its nine in-repo `=` pins"; Counts `wsver`: arch `:45` ×2 changed.'
    basis: .andromeda/architecture.md:45
  - detector: D-arch-decisions
    severity: warning
    section: §Inherited Defaults → Framework
    change: 'Engine crates pinned `=0.3.0-beta.4` (Cargo.toml:45-53).'
    sidecar: 'Inherited Defaults Framework: engine crate pins at 0.3.0-beta.4.'
    rationale: 'Report Counts `wsver`: "`architecture.md:220` (the engine crates pinned)" changed.'
    basis: .andromeda/architecture.md:220
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Project Intent → Upstream sync
    change: 'DioxusLabs/blitz `main` last merged at `23b9b26a9` (merge commit `3eda07bc`, parents `143b6013` · `23b9b26a9`, 2026-10-17) — the next sync''s merge base; upstream `main` read at `1e5479ae` at the take-up, 14 commits past `434e0661`; the four Taffy-main commits past the pin not taken (`6032d0b9d` inside it reverted by upstream''s own `85340347f`), so Taffy stays the registry "0.15".'
    sidecar: 'Project Intent Upstream sync: merge base 434e0661 → 23b9b26a9 (merge 3eda07bc, 2026-10-17).'
    rationale: 'Report Counts `pin`: "`434e0661` → `23b9b26a9`, merge `3eda07bc` ... 2 changed — `architecture.md:206` ×3 (§Project Intent, the `Upstream sync:` line)"; Cross-project claims (I1: `1e5479ae`, 14 commits); Reverted / negative API facts (Taffy-main commits); Dependencies (Taffy registry "0.15").'
    basis: .andromeda/architecture.md:206
note: 'Not proposed (needs a source read the report does not give): architecture.md:87 states the engine''s platform action modifier as `SUPER` on macOS citing packages/blitz-dom/src/util.rs:7-10, and the merge added util.rs line 8 (report New text) — the report''s `macmod` control matched no master, so whether the engine''s modifier also moved to `META` is unmeasured here; the orchestrator should read util.rs and amend §Established Decisions if it did. Report-listed no-change rows left as they stand: arch :87 ×4 and :136 (dated 434e0661 measurements), :211, :64, :66, :67, :226, :115, :141 @c7251,7320, :264.'
