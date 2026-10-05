# Curation — 2026-10-05-fork-ci-reached

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "bare `gh` reads upstream in this checkout — pass `-R Turbolet85/escher` on every read of the fork" (confidence 0.8)
  Proof: plan entry 15's literal `gh run view 37375560233 --json jobs …` exited 1, `HTTP 404` on `repos/DioxusLabs/blitz/actions/runs/37375560233`; `git remote -v` lists `upstream https://github.com/DioxusLabs/blitz.git`; `gh repo set-default --view` → "No default remote repository has been set"; the `-R Turbolet85/escher` form exited 0 (evidence/operator-pass.md §Entry 15). Signals: measured +0.4 · technical detail +0.2 · load-bearing +0.2.
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A chunk that moves cited source lines stales the masters' file:line citations" (0.8)
  Proof: 114 stale `Cargo.toml` / `ci.yml` / `wpt.yml` / `publish-browser.yml` citations across architecture, security-plan, design-system, test-plan, obs-plan after this chunk; 0 of 32 detector proposals named one; 112 re-pointed with a digit-stripped equality check per master (fanout-results.md §Orchestrator raises). Signals: measured +0.4 · detail +0.2 · no-other-home +0.2.
                                              + "Count from the listing you just read, never from the plan's forecast" (0.8)
  Proof: evidence/operator-pass.md first recorded "10 `v0-rust-*` + 1 apt = 11" (the plan's "6 linux + 4 matrix = 10"); `gh cache list` listed 11 + 1 and the usage API read `active_caches_count` 12; corrected at P1. Signals: measured (falsified a plan claim) +0.4 · detail +0.2 · no-other-home +0.2.
  Filters: 2 rejected — "`perl` matches inside hyperlink" (0.2: detail only; the rewrite it found is gone) · "the fork's Actions cache sits at ≈97 % of 10 GB" (home: amended into architecture §Occupied Resources this wrap) · 0 dup · 0 conflict · 0 deferred
  Load-bearing: "bare `gh` reads upstream — pass `-R`" → CI gate legs
  No-other-home: "a source-moving chunk stales file:line citations" · "count from the listing, not the forecast"
  CLAUDE.md size: 126/200 · T1 0.4 KB, 0 over 600 B (health.py check 1)
  Side note: the `gh -R` fact was first written into the generated `.claude/docs/commands.md` during the cascade; removed there (no master carries it, so a re-derivation would drop it) and curated here instead.
