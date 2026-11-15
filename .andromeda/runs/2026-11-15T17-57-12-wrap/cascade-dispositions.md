# Cascade dispositions — 2026-11-15-browser-corrective

## Step 2 — the sweep
Search: `cascade.py sweep --patterns-file cascade-patterns-architecture-1.toml` (the one landing's dry-run file; pattern `arch1-1`, fixed `clippy::expect_used`, control `decision/justified-expect-unwrap` r2 — fired). No pattern of main's own: the one amended claim is the `except`'s favicon exception, whose distinctive token the tool's pattern carries; a hand read of the leaves for `favicon` (git grep over `.claude` and CLAUDE.md) found no leaf stating the favicon exception.
Listing: `sweep-pass-1.1.txt` — total (1 pattern) · 4 rows over 3 files · per class · new 0/0 · standing 2/1 · leaf 2/2 · curation 0/0 · base 0/0.

- `.andromeda/architecture.md:502` dec-nc3nzjgd `decision/justified-expect-unwrap` — amended (tx 1162, the `except` left out); the `edited` row's match is the rule's own `clippy::expect_used`, no duplicate of the retired exception.
- `.andromeda/architecture.md:547` dec-vlchwzsx `decision/unwrap-expect-justified` — no change: states the rule alone, true.
- `CLAUDE.md:62` (leaf) — no change: states the rule alone, true; no exception named.
- `.claude/docs/conventions.md:32` (leaf) — no change: states the rule alone, true; no exception named.

## Leaves
Set: `cascade.py leaves --add .claude/docs/tests-summary.md` — pass mode (epoch 5 open), baseline 73a168d5; total · pass · 7 group(s) · 16 leaves · 435871 B · unmapped 1. The `--add` was the wrap's own: `tests-summary.md:31` lists the browser crate's tested areas (about pages, history, favicon, suggestions, persistence, tab).
Apply dry run: `total · blocks 0` — every comparer file read whole, 0 findings.
- architecture-1 … architecture-6 · pass · findings 0 · applied 0 · by hand 0 · declined 0.
- added-1 (`tests-summary.md`) · pass · findings 0 · applied 0 · by hand 0 · declined 1 — the wrap's own read: line 31 does not name the document loader's new inline tests; no master row lists the browser's tested files (the pre-pass `browser-tested` pattern's control refusal), so the line restates no source and a reader acts against nothing — the detail stays in the code.
- unmapped `.claude/rules/host-linux.md` (host) — the host's own statements, no comparer's and no step-3 edit.
