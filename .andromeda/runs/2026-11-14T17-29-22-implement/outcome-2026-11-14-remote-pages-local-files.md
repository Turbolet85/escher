# Implement outcome — 2026-11-14-remote-pages-local-files

**Outcome: green** (smoke skipped: the plan lists no smoke entry). Next: the operator pass, entries 7 (hygiene), 8 (push) and 9 (CI read), then `/andromeda-wrap-session`.

## Files changed
- `apps/browser/src/favicon.rs` — `favicon_candidate` now filters its candidate through `document_may_load(Some(&base), …)`. It cites the Fetch spec, and its `///` line names Chromium's behaviour. Three tests were added to the existing module.
- `apps/browser/src/history.rs` — `BrowserNavProvider` gains a private `document_url` and the constructor `for_document(history, base_url)`, which parses with `.ok()`. `navigate_to` returns early, silently, where `document_may_load` refuses. A new test module holds three tests.
- `apps/browser/src/document_loader.rs` — `make_doc_config` builds the provider through `for_document(history, base_url.as_deref())`.
- `docs/capabilities.md` — the CARRY: both «provisional, pending the founder's choice» clauses are dropped and the bullet re-flowed.

## Gates (trail `.andromeda/runs/2026-11-14T17-29-22-implement/gate-2026-11-14-remote-pages-local-files.json`)
Two full runs, both green: `entries 9 · green 6 · red 0 · recorded 0 · timeout 0 · not-run 3`. The second run is on the final tree, after the justification comment below. Entries 7–9 are `leg operator` and were not fired.

## Controls (step 7) — `.andromeda/cache/chunks/2026-11-14-remote-pages-local-files/evidence/control-1.txt`
`control: 5 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`. C1–C5 were fired against entry 1 in a clone at HEAD e73c5ff8. Each control's cases are the test names alone.

## Deviations from plan, with justifications
1. **`make_doc_config` binds the provider before the struct literal** instead of building it in place at line 56. `base_url` moves into the literal's `base_url` field, so building the provider inside the literal failed with E0382. The behaviour is the same.
2. **The history tests assert outside the component.** Step 5 modelled them on `tab.rs`, whose assertions sit inside the component. Here the component writes the store's current URL into a thread-local and the test asserts after `rebuild_in_place`. Each test still asserts the schemes of its document and target first.
3. **A one-line justification sits beside the new `#[allow(clippy::expect_used)]` in `history.rs`.** The row is architecture `decision/justified-expect-unwrap` (`dec-nc3nzjgd`). The plan does not cite it, and it binds this work: its exception covers «the browser app's test module» (favicon's existing one) and not a new one. Research named it among the patterns. No scope-record line is needed because it is a listed file.
4. Scope: `scope: clean — changed 4 · listed 4 · recorded 0`. No out-of-list edit was made.

## Stated limits
- The wiring call in `make_doc_config` is the one changed line no test reaches: it runs only inside a launched Dioxus runtime (`consume_context`). C4 controls the parse in `for_document`, not this call site. A mutant passing `None` there would survive the unit entry.

## Master rows binding this work that the plan does not cite
- architecture `decision/justified-expect-unwrap` (`dec-nc3nzjgd`) — see deviation 3. Its `except` names one browser test module, and this chunk adds a second one that carries a justification. The wrap may want the except's wording checked.

## Ledger
There is no `coverage` row for this chunk (0 rows; the scope says to expect no claim), so no `matrix.py implement` call was made.

## Surfaced / stuck
None. No founder fork arose, and `founder-forks.md` was not written.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc (gate entries 1, 2, 6; control clone builds) | this run | terminated — `gate.py census --comm blitz --comm cargo --comm rustc --args andromeda-control`: `census: none running` |
| control clone `~/.cache/andromeda-control/a790173f` | this run (gate.py control) | removed (`clone removed`) |

## Decisions & corrections
- Guard refusal: bash-guard refused a `cat` heredoc with a file target for the edit script. I re-issued it once through the Write tool and ran it by path.
- Operator directives were applied: in the controls file each case is the test name alone; a fork would go to `founder-forks.md` (none arose); the run stops after this report.
