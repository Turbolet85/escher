# P5 review — 2026-11-14-remote-pages-local-files · «Remote pages and local files in the browser»

Plan: `doc:2026-11-14-remote-pages-local-files/plan` (rev 3) — open its render
`.andromeda/cache/docs/2026-11-14-remote-pages-local-files/plan.md` in an editor. No plan stands in a chunk's folder;
writing in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py
diff --root . 2026-11-14-remote-pages-local-files` diffs the chunk's records.

## Goal
In the reference browser app (`apps/browser`), a remote page can no longer make the browser read a local file:
- **Favicon.** A remote page's favicon candidate under `file:` is never chosen, so the probe and the shared favicon
  cache are never reached.
- **Navigation.** A link, keyboard-activated link or form submission that a remote page starts toward a `file:` URL
  loads nothing. The refusal is silent, as in Chromium and Firefox.

A `file:` page keeps both its `file:` favicon and its `file:` links. The user's own loads are unchanged.

How:
- Both gates call the engine's own `document_may_load` predicate as it stands.
- No engine crate changes, and no log line, dependency or carrier is added.
- This is a narrowing, not a boundary widening.

The chunk also folds the CARRY: `docs/capabilities.md` drops its two «provisional» clauses.

## Touchpoints
- `apps/browser/src/favicon.rs` — `favicon_candidate` is filtered by `document_may_load`. Three new tests.
- `apps/browser/src/history.rs` — `BrowserNavProvider` gains `document_url` and `for_document`, and `navigate_to`
  returns early when the predicate refuses. A new test module with three tests.
- `apps/browser/src/document_loader.rs` — `make_doc_config` builds the provider through `for_document`.
- `docs/capabilities.md` — the CARRY's two clauses are deleted.

## Top acceptance criteria
- Six new browser unit tests are each `ok` under `cargo test -p browser --locked`:
  - three for the favicon: a remote page's `file:` candidate is none; a `file:` page's candidate is kept; a remote
    page's default `/favicon.ico` is unchanged.
  - three for navigation: remote → `file:` loads nothing; `file:` → `file:` loads; remote → `https:` loads.
- `gate.py control` reads C1–C5 each `holds`. That is one mutation per test (a gate removed, or a gate that refuses
  everything), as test-plan `decision/mutation-controlled-tests` and your directive (inputs#I1) require.
- `grep -c "pending the founder" docs/capabilities.md` prints 0.
- The `fast` leg is green, which includes the capability-statement pin.
- No added log, print or env read in `apps/browser`, and `Cargo.lock` is unchanged. Both silence probes were
  controlled at P5: `p5-controls.md` in this run dir.
- The operator pass's CI run is green on linux, windows and macOS.

## Test commands (9 entries)
| # | run | baseline |
|---|---|---|
| 1 | `cargo test -p browser --locked` (unit; six name atoms) | red: 29 pass, the new names absent |
| 2 | `cargo test -p browser --locked -- --list` | red: the new names absent |
| 3 | `grep -c "pending the founder" docs/capabilities.md` | red: 2 |
| 4 | `git diff e73c5ff8c -- apps/browser \| grep -cE …` (silence) | green: 0, controlled in a scratch clone |
| 5 | `git diff --quiet e73c5ff8c -- Cargo.lock` | green, controlled in a scratch clone |
| 6 | `bash .github/scripts/ci-leg.sh fast` | seen green at the last wrap |
| 7–9 | operator: hygiene · push · `ci.py --wait 1800` | leg, exempt |

## Concretization preview
No capability is claimed. The four planned 0.3.0 caps (v030-10, -11, -12, -15) each belong to a later markerless
entry, so no acceptance is concretized and none is weakened.

## Leans taken without a fork
- **A refused navigation is silent.** This follows Chromium and Firefox, per arch `decision/cite-the-spec`. Showing
  the error page was rejected.
- **The favicon is gated at `favicon_candidate`.** An initiator on the fetch was rejected because the shared cache
  would still serve a `file:` icon that a `file:` page had cached.
- **No founder fork arose**, so no `founder-forks.md` was written. Every option is a narrowing, and none matches a
  `rul-izni57qv` (Boundary widening) clause.

## Validation
- **Mechanical checks:** all pass. planlint reports 0 hits, the fence parses (9 entries), research's lists parse
  (4 items), and the snapshot citations verify (I1, n/a).
- **Check 4 (6):** the obs criterion named a probe that had no entry. Entries 4 and 5 were added and controlled, and
  the whole check set was re-run.
- **Validation-1:** the plan is aligned with `doc:…/scope` rev 3. P3's premise closure corrected 5 bullets, and one
  of them added `document_loader.rs`, so the row read ran again whole.
- **Stated limit:** the one-line wiring in `make_doc_config` has no test reaching it (it runs only inside a launched
  Dioxus runtime). The report will name it.
- **For the wrap:**
  - The Epoch 4 boundary batch: `decision/no-seed-on-carriers`, the `CLAUDE.md` seed clause and the entry's CONTEXT
    mark, on the founder's word in I6.
  - The obs and design PROVISIONAL marks from 2026-11-07-ui-graph and 2026-11-08-handler-coverage have no founder
    word yet. They stay marked and will be named in the handoff.

## Beyond the plan
none
