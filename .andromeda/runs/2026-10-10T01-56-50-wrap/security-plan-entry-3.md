
## 2026-10-10-upstream-sync-agent-surfaces — dependency security after the sync: Parley's pin form, the entering crates, an ungated licence table, the apt action gone upstream, a second `unsafe`
**Section:** §Dependency Security → Audit tool, Pinning, Unsafe code, Supply chain integrity
**Change:**
- Pinning: was "Git dependencies are pinned by commit rev", citing taffy and parley; now the one git dependency, taffy, is pinned by commit rev and parley is a registry version, "0.12".
- Pinning, new bullet: the sync took upstream's dependency set whole, no `cargo update`; the root manifest gains icu_casemap, icu_locale_core, icu_properties and icu_segmenter "2.3" and writeable "0.6"; six names enter the lock (core_detect, multiversion_no_op, icu_casemap, icu_casemap_data, memchr-n, fearless_simd_macros), each from crates.io with a checksum and reviewed by hand for licence, repository, build script, proc-macro flag and direct dependent, its source beyond the manifest not read; jetscii and tinyvec_macros leave; `advisories ok`, no ignore added or dropped.
- CI tooling: was "the upstream-only publish-browser and wpt workflows still reference [the apt-cache action] at `@latest`"; now they no longer use it — each installs with a bare `apt-get` line behind its repository guard.
- Unsafe code: was "one `unsafe` block" in stylo_taffy; now two — the second in its public `resolve_calc_value`, which reads a calc value back through a raw pointer.
- Audit tool: `deny.toml` also holds a `[licenses]` table the audit leg does not read.
- Supply chain integrity: the NOT YET MEASURED note narrows to SBOM generation and base image scanning; a new Licence compliance bullet — upstream's table (13 allowed expressions) byte-identical in `deny.toml`, no CI leg runs it, one reading `licenses ok` over the six-target graph with two unmatched allowances.
**Why:** all of it arrived by the merge. Upstream's `licenses` CI job was taken by the merge and reverted — the operator's decision of 2026-10-10 at the plan: no licence gate yet, one reading into evidence. Whether the fork gates on licences is left open on purpose: the operator, 2026-10-10, as the arguments of this wrap, pinned the question for the founder on the route entry "Quality gates".
**Kept:** `[graph]` and `[advisories]` are the chunk start's; the audit's reach is still cargo-deny's resolved graph.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/
