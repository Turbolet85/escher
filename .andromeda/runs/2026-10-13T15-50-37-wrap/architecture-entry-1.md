
## 2026-10-13-upstream-sync-polish-and-ship — the upstream sync: taffy from the registry, the beta.3 pins, the pin and the front page
**Section:** §Stack and Technologies → Workspace package row · Layout row · Testing row · §Established Decisions → [Dependency pinning] · §Inherited Defaults → Framework · Publishability · §Project Intent → `Upstream sync:` line · Front page
**Change:**
- [Dependency pinning]: was "taffy is a git dependency pinned by `rev`"; now taffy is a registry version, "0.15" (resolved 0.15.0, its eight features unchanged), and no git dependency is left in the manifest. The coupled pins, the winit exact beta and the commented-out patch blocks stand.
- Layout row: was taffy from git at a pinned rev; now taffy "0.15" from crates.io with the same eight features. Inherited Defaults → Framework: engine pins `=0.3.0-beta.3`, taffy "0.15" from the registry.
- Workspace package row: version and the nine in-repo `=` pins move `0.3.0-beta.2` → `0.3.0-beta.3`.
- Testing row: blitz-tests' dev-dependencies gain `cursor-icon` (the workspace's), for upstream's `pointer_leave` check's hand-written `ShellProvider`.
- Publishability: blitz-vibey-script sets `publish = false` and joins the exceptions to the published `packages/` crates.
- `Upstream sync:` was last merged 7832c177 (merge 9462a7e4, 2026-10-10); now 434e06615d5f47a3df92c0f94e916df088fdcf4d (merge a6c24d24, 2026-10-13), the next sync's merge base.
- Front page: the page says eleven driver commands as tools, and the screenshot is no longer named as still to come.
**Why:** the merge took upstream's pin set whole — the plan's lean, with no third-party package moved and the audit green; the [Dependency pinning] retirement was named in the plan's reviewed Expected amendments and confirmed by the founder at this wrap's card (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить (Рекомендую)»; the English wording is the overseer's). Trap for the next sync: the CLAUDE.md coupled-pins warning no longer names a taffy git rev — taffy now moves with the registry like parley.
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/
