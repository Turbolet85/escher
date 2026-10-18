
## 2026-10-17-upstream-sync-engine-debts — the bump crate leaves the workspace
**Section:** §Stack and Technologies (Language / runtime · Release tooling) · §Established Decisions ([Release versioning]) · §Standard Contracts (CLIs) · §Occupied Resources (Names) · §Infrastructure Patterns (Directory structure · Deployment model) · §Project Intent (Apps and demonstrators · Growth model) · §Inherited Defaults (Publishability) · §Existing Scopes
**Change:**
- The Release tooling row (bump: toml_edit 0.22, semver 1) and the Existing Scopes `bump` row are gone; `bump` leaves the edition-2024 list, the Names row, the directory tree's `apps/` line, the CLIs line (its target-and-semver arguments), the Deployment model (its toml_edit version rewrite) and Apps and demonstrators.
- [Release versioning], the Growth model and Publishability: was «bump versions blitz packages together and anyrender packages together» / «versioned together for release by bump»; now blitz packages are versioned together through the one workspace version and the in-repo `=` pins, with no in-repo release tool since upstream deleted the bump crate. The prepatch rule stands.
**Why:** upstream's `23b9b26a9` "Delete the bump crate", taken by the sync; `toml_datetime`, `toml_edit` and `toml_write` left the lock with it, and no port, listener, env read or member was added.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
