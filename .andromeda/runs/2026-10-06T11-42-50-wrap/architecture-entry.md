
## 2026-10-06-project-readme — README is escher's front page; stylo_taffy triple licence recorded
**Section:** §Conventions → Licensing exceptions · §Project Intent → Front page
**Change:**
- Licensing exceptions now also lists `stylo_taffy`, which declares `license = "MIT OR Apache-2.0 OR MPL-2.0"`: the workspace's dual licence plus MPL 2.0.
- Project Intent gains a **Front page** bullet. The root `README.md` is escher's own page, not upstream Blitz's. It covers what escher is, what is built and what is planned (stable element ids and the headless 7GUIs stand are built; snapshot, driver, CLI and MCP are planned), its Blitz lineage, the 0.1.0 epochs, the licence and an Author / Contact section. Its one image is the fork's `ci.yml` badge. No build, doc or test step reads it; `rdme` resolves a README at runtime only. At an Upstream sync, an upstream change to `README.md` resolves to escher's version.
**Why:** the chunk replaced upstream's front page wholesale, so later Upstream syncs will meet README conflicts at some syncs; the rule records which side wins. The README states stylo_taffy's MPL 2.0, which no master recorded. Trap: Product type's homepage/repository still name dioxuslabs/blitz; the release-metadata repoint is a separate decision pinned on the route, not this bullet.
**Ref:** .andromeda/runs/2026-10-06T11-42-50-wrap/
