# Counts — 2026-10-27-readme-and-records (implement P2)

One firing of the block, run dir `.andromeda/runs/2026-10-27T05-19-41-implement/` (trail
`gate-2026-10-27-readme-and-records.json`), `gate v1.22 · 6739ca7e`, on the working tree over base `316c6d25b59a`.

Summary line, verbatim:

```text
entries 17 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 4
```

| # | Entry | Reading |
|---|---|---|
| 1 | the capability-statement pin | green · exit 0 · `Ran 4 tests` · `OK` |
| 2 | the three status headings | green · last line 3 |
| 3 | `0.2.0 — planned` gone | green · exit 1 · last line 0 |
| 4 | lowercase escher in README prose | green · exit 1 · last line 0 |
| 5 | the 2026-10-15 run sentence and the stale part-read line | green · exit 1 · last line 0 |
| 6 | the restated run paragraph | green · last line 1 |
| 7 | the hygiene search over the two documents and their pin | green · exit 1 · last line 0 |
| 8 | `git check-ignore -v escher-0.3.0-incubator/intent.md` | green · `.gitignore:39:escher-0.3.0-incubator/` |
| 9 | incubator lines in `.git/info/exclude` | green · exit 1 · last line 0 |
| 10 | the three current names as `fn` lines and in `records.md` | green · `names: 3` |
| 11 | the preservation guard | green · `guard: allowed 2 — README.md, docs/capabilities.md · companions excluded 0 · changed 2 · excluded 0` · `guard: clean` |
| 12 | `ci-leg.sh fast` | green · exit 0 · 108.42 s · artifact fresh (`target/ci-logs/test.log`) · CI scripts `Ran 108 tests` `OK` |
| 13 | `ci-leg.sh doc` | green · exit 0 · 0.45 s (no source moved; `Generated ./target/doc/accesskit_xplat/index.html and 28 other files`) |
| 14 | the front page through `gh api` | not run — defer (gated on the founder's word on F3); not voided |
| 15–17 | hygiene · push · CI read | not run — leg operator (the operator pass's) |
