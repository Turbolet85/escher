# Founder forks — 2026-11-15-upstream-sync-errors-and-explanation (phase run 2026-11-15T05-31-22-phase)

Written in the away form (`inputs-contract.md` §snap): the founder is away. The operator's directive at this invocation (`inputs#I1`) sets no dialog and the most conservative provisional answer. The plan holds no step that waits on either answer: each question's PROVISIONAL answer is what the plan does. His answer is appended below, snapped, and acted on at the wrap or a later chunk.

## Q1 — Lift "the workspace takes no git dependency" for the git Taffy?

**What his 2026-11-14 22:09Z word asked** (`inputs#I2`, «По таффи посмотрите, если можно будет сделать делайте»): measure the take of upstream's four Taffy-main commits (`1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`) and take them if every gate stays green and no master rule only he can lift is broken.

**The numbers** (research §The Taffy carry, measured 2026-11-15 on this host):

| | Git Taffy (the four commits as upstream wrote them) | Registry taffy 0.15.1 (crates.io) |
|---|---|---|
| Picks onto HEAD `ba44c3559` | 4 of 4 clean, 0 conflicts | — (no commit picked) |
| Edits | 3 files, +3 −3: `Cargo.toml`, `Cargo.lock`, one line in `blitz-dom/src/layout/inline/layout.rs` | 2 lines of `Cargo.lock`, `Cargo.toml` unchanged |
| `fast` leg | exit 0, 1461 passed · 0 failed · 13 ignored | exit 0, 1461 passed · 0 failed · 13 ignored |
| `audit` leg | exit 0, `advisories ok, licenses ok` | exit 0, `advisories ok, licenses ok` |
| `msrv` leg (Rust 1.91) | exit 0 | exit 0 |
| Taffy commits carried (from `v0.15.0` to `ea07c4c3`) | 14 of 14 | 13 of 14: all but #1303, the caller-defined float id |
| Master rule broken | `decision/no-git-dependencies` (architecture), `decision/no-git-dependency` (security-plan), CLAUDE.md's Critical Warning | none |

- No gate refuses the git take mechanically. The audit leg checks advisories and licences only, `deny.toml` has no `[sources]` table, and no CI script checks for a git dependency.
- Upstream itself has moved past these four commits. Its manifest at `ea07ee716` pins git Taffy `a3997d55…`, 49 commits past `v0.15.1`'s fork point. Nine more left-out upstream commits move it there: `b8f4ba6fd`, `9babd796a`, `d3ed4e2c7`, `ccfe15332`, `35cdfc859`, `930448918`, `c51dd246c`, `752ed2e49` and `b4952e15f`.
- Taffy's own `v0.15.1` commit `93db25f7` reads "Changelog: drop the entry for #1303, which is not on 0.15.x". So #1303 reaches the registry only with a 0.16 release.

**Options:**
1. **Keep the rule; take the registry carrier (PROVISIONAL — what this plan does) (Recommended).** The take moves the lock to taffy 0.15.1 and carries 13 of the 14 Taffy changes with no rule broken. #1303 and the later git chain wait for a registry release (0.16 or later), re-read at each sync. Your word is met as far as it can be without a rule only you can lift; the gap is one Taffy commit and the one `inline.rs` line that uses it. Its trade-off: escher's Taffy stays behind upstream's by #1303 and the later main commits, so later upstream float and layout commits that need them stay left out.
2. **Lift the rule for Taffy alone.** The take carries the four commits, or upstream's present `a3997d55` pin with the nine commits after them, as a git dependency. The two no-git-dependency rows, CLAUDE.md's warning and `.claude/rules/security.md` §Dependencies change by your word. A further chunk would be needed: the four commits measured green here, but the move to `a3997d55` and its nine commits was not measured.
3. **Lift the rule for every upstream pin** (Taffy, AnyRender, …). This is the widest option. It opens the pin-mover chains left out since 2026-10-17, which hold the AnyRender git pins and Dioxus 0.8.

**PROVISIONAL answer applied:** 1 — the most conservative, per `inputs#I1`.

## Q2 — Ratify the take's two behaviour changes for every document?

Under architecture `decision/upstream-behaviour-changes-ratified-per-take`, an upstream behaviour change for every document is taken only ratified for its take. This take brings two, on every document, remote pages included:
- `f5ea1caf1` — an `@font-face` rule's `font-stretch` descriptor is passed to fontique's font matching. It is a numeric range Stylo already computes, carrying nothing out, with no log site, print, env read or new request.
- taffy 0.15.0 → 0.15.1 — Taffy's flex, grid, float and baseline fixes (#1304, #1305, #1320, #1322, #1323, #1327, #1328, #1332, #1333, #1335, #1336, #1337, #1343, #1351 and #1358, by `v0.15.0...v0.15.1`). The workspace tests read no moved number on this host.

**Options:**
1. **Ratify both as upstream's behaviour (Recommended)** — the same as the 2026-10-27 and 2026-11-13 takes' upstream changes.
2. Ratify `f5ea1caf1` alone and hold Taffy at 0.15.0.
3. Hold both, and take only the two WPT-only commits.

**PROVISIONAL answer applied:** both taken, each marked PROVISIONAL until your word, per the precedent of the 2026-10-19 take. The plan still takes them because Q1's option 1 rests on the 0.15.1 move, and his 22:09Z word authorizes a take that breaks no rule only he can lift.

## Answers
_(the founder's word appended here, verbatim, then snapped)_
