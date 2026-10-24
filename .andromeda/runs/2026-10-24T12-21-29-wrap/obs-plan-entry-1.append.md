
## 2026-10-24-upstream-sync-polish-and-ship — the ten-commit take read silent, and `multi_doc`'s printed timings
**Section:** §1 Obs Scope Summary (body: examples) · §3 Observability Harness Contract (body: the silence census) · §5 Metric Coverage (body; metric-coverage.md) · §6 Log Coverage (log-coverage.md: examples) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md)
**Change:**
- §1 and §5: the root example `multi_doc` prints its timings — the run's and each resolve's, read with `Instant::now()` — in one `println!` on stdout; a metric row and the examples' scope name it; §6 cites it among the examples' `println!` output.
- §3 census: the upstream take read at `f9cbb2756` is silent — `hit_test_dropped_nodes` installs no subscriber, reads no env var and prints nothing; the taken commits add no tracing site over `packages/` (one commented `dbg!`) and no wall-clock read in blitz-dom outside `clock.rs`; `PARALLEL_TRAVERSAL_LOCK`, `Node::out_of_flow`, `ElementData::classes`, the dropped-node hit skip and the replaced element's `aspect-ratio` height add no log site, span, field, log field or env read; `packages/escher-telemetry/` unchanged; `host_log` and `mcp_log` pass under both builds.
- §8: the sink allowlist re-check holds — no scrub set, redaction shape or violation schema moves; `multi_doc`'s line is example output outside escher's sink, holding no page or user content.
**Why:** each upstream sync is read for log sites and the sink re-checked; the take is the founder's own word «Десять коммитов (Рекомендую)», relayed verbatim by the overseer (2026-10-24, chunk inputs#I4). No record class, field or carrier is added.
**Ref:** .andromeda/runs/2026-10-24T12-21-29-wrap/
