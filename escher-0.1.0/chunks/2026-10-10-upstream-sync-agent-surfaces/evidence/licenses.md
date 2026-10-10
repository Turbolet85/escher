# Licences — one reading of upstream's table over our graph

Read 2026-10-10T01:28Z on the resolved, uncommitted merge: `cargo deny --locked check licenses` → exit 0, last line `licenses ok`.

- The configuration read is the resolved `deny.toml`: upstream's `[licenses]` table verbatim (13 allowed expressions, `include-dev`, `include-build`, `confidence-threshold = 0.8`) over OUR `[graph]` — the six targets with `all-features = true`. Upstream reads the same table over a graph with no target filter; that reading was not taken here.
- Verdict: green. No crate of the six-target graph carries a licence outside the allow list.
- Two warnings, `license-not-encountered`, each an allowance the graph never meets: `NCSA` and `Unicode-DFS-2016`. Neither fails the check.
- At the chunk start the same command read `licenses FAILED` at exit 4 (the plan's baseline): `deny.toml` held no `[licenses]` table, so every licence was refused.

This is a report-only reading by the operator's decision of 2026-10-10: no leg and no CI job runs the check. Whether it becomes a gate is the question carried to the route's "Quality gates" entry.
