
## 2026-10-16-fast-failing-mutants — the mutation tier's host instruments; `.config/nextest.toml`; the bridge's seven tests
**Section:** §Stack and Technologies (Code quality) · §Infrastructure Patterns (Directory structure) · §Existing Scopes (dioxus-native-dom)
**Change:**
- §Stack and Technologies, Code quality: a mutation tier run on the dev host alone, never in CI — cargo-mutants 27.1.0 (the tier's tool) with `--test-tool nextest` and `--baseline=skip`, cargo-nextest 0.9.146 (its test tool) under the `mutants` profile of the root `.config/nextest.toml`, each unit's mutant test command read green on the unmutated tree first; neither tool lockfile-resolved; `NEXTEST_PROFILE=mutants` a host tool's variable no workspace source reads; the role gains «a host-run mutation tier».
- §Infrastructure Patterns: the directory tree gains `.config/nextest.toml` — one profile, `mutants`, the default untouched.
- §Existing Scopes, dioxus-native-dom: was «six unit tests of the bridge»; now seven, the seventh a created text node's mapping and stack read.
**Why:** this chunk's own additions; `NEXTEST_PROFILE` takes no §Occupied Resources line, which registers what workspace source and CI scripts read.
**Ref:** .andromeda/runs/2026-10-17T07-59-25-wrap/
