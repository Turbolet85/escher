# Cascade dispositions — 2026-11-13-local-files-for-local-documents

## Step 2 — the sweep
The landings' dry runs wrote no pattern file (each said `patterns: none: … changed prose alone — no file`: architecture 4 sets, security-plan 2, test-plan 2 + 1, obs-plan 1). Main's own patterns stand in `cascade-patterns.toml`. The search covered the amended claims' old wording and mechanism over all seven masters, their key files, the leaves, the curation homes and the bases:
- `diskread` — «`file:` URLs read from disk», the unconditional disk read;
- `scriptonly` — «only for / from a `file:` document», «loading document is itself a `file:` document», the rule stated for script loads alone;
- `pngdecode` — «decodes a PNG»;
- `fileurl` — the five amended rows' handles.

A first fire also carried `initiator` (the new tokens `initiator` · `stamped_request` · `FileForbidden` · `clippy.toml` · `disallowed-methods`). Its control never fired over the pre-pass masters, since these are new facts with no old site, so it was dropped as the tool directs.

Listing `sweep-pass-1.1.txt`, read whole: `total (4 patterns) · 20 rows over 7 files`; `per class · new 1/1 · standing 15/4 · leaf 4/3 · curation 0/0 · base 0/0`.

- `architecture.md:391 diskread new dec-ykas6pyc` — amended (TX 737), the new wording itself.
- `architecture.md:203 scriptonly/fileurl dec-mvu7nnu6` edited — amended (TX 737); its clause «so a remote page cannot read local files» landed on inputs#I5's placement.
- `architecture.md:551 ctr-xges6t4p` — no change: the re-exported `document_may_load` still lets a `file:` URL load only for a `file:` document. The predicate is unchanged, and its body moved to `blitz_traits::net`.
- `architecture.md:837 ctr-rje7fbpv` — no change: the default script fetcher's schemes, `file:` only for a `file:` document, hold unchanged.
- `architecture.md:1116 ctr-yctyehsj` — no change: the predicate's promise holds through the re-export. Only its `verified` evidence coordinate (`fetch.rs:44-45`, now `blitz-traits net.rs`) is stale, and that is no cell.
- `architecture.md:591 fileurl dec-p4r2r3dp` — amended (TX 737).
- `security-plan.md:10 dec-mc7yl2gi` — no change: the default script fetcher's local-only rule, about script fetches alone, holds.
- `security-plan.md:14 dec-gsml2o7x` · `:111 dec-ik2uwgla` — amended (TX 738).
- `test-plan.md:359 dec-bbvpdkrm` (mixed ×2: its name, a label, and its rule) — amended (TX 739, TX 741).
- `obs-plan.md:207 dec-5qmnwhpl` — amended (TX 740).
- leaf `.claude/rules/security.md:13` (diskread) · `:15` (scriptonly) · `.claude/docs/security-summary.md:14` (scriptonly) · `.claude/rules/testing.md:18` (pngdecode) — step 3, read by `cascade.py leaves` from this listing.

Curation homes and bases: 0 rows.

Lateral binds: test-plan §3 ↔ obs-plan §3/§6 and the a11y violation schema — no harness command, log field or violation field moved (report: no log site, no schema). Nothing to keep consistent.

## Leaves
`cascade.py leaves`, pass mode: `total · pass · 12 group(s) · 22 leaves · 592746 B · unmapped 1`. Unmapped: `.claude/rules/host-linux.md` (host), the host's own statements. No comparer reads it, and no step-3 edit touches it.

Apply call: `total · blocks 14 · applied 10 · held 4` (6 files). The 6 groups with no block: architecture-2, architecture-3, architecture+obs-plan, architecture+security-plan+test-plan-1, architecture+test-plan (0 findings each), and test-plan-1 (3 findings, all declined).

- architecture-1 — pass · findings 5 · applied 4 (conventions.md:12#1 builder exception, :22#1 stamping constructor; services/blitz-dom.md:21#1 stamping constructor; services/blitz-traits.md:14#1 `initiator`) · declined 1: blitz-dom.md:11#1 (`StylesheetHandler.document_url` — the compiler enforces the field at every construction; detail stays in the master).
- architecture+security-plan+test-plan+obs-plan — pass · findings 1 · applied 1 (CLAUDE.md:25#1 blitz-net's `file:` rule, in the generated modules block).
- obs-plan — pass · findings 1 · applied 1 (obs-summary.md:19#1).
- security-plan-1 — pass · findings 2 · applied 1 (security-summary.md:14#1) · by hand 1: security-summary.md:15#1, the comparer's replacement dropped «with no restriction», which still holds where blitz-net reads; drafted whole to `draft-.claude-docs-security-summary.md` and landed (`rec.py harness land`, rev 13, tx 751).
- security-plan-2 — pass · findings 3 · applied 2 (security.md:15#1, :13#2 the stamping constructor) · by hand 1: security.md:13#1, the replacement dropped «with no path restriction» for a read that is allowed; drafted whole to `draft-.claude-rules-security.md` and landed (rev 13, tx 750).
- test-plan-1 — pass · findings 3 · declined 3: tests-summary.md:18#1 (a permission to use the real provider in a stand-in; no reader acts against it), :21#1 (a listing of checks), :46#1 (the clippy leg fails by itself on a stray `Request::get`; the convention stands in conventions.md).
- test-plan-2 — pass · findings 2 · applied 1 (testing.md:18#1 decodes or encodes) · declined 1: testing.md:18#2 (the same permission as tests-summary.md:18#1).
