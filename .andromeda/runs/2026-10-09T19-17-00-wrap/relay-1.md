# Operator directive — the adaptation wrap at escher's Epoch 4 boundary (2026-10-09)

**Provenance.** The rulings below are the founder's. He gave them by question dialog in the overseer session on
2026-10-09 (between 18:56Z and 19:13Z); the overseer relays them. His chosen option labels are quoted in Russian; the
English wording of each ruling is the overseer's rendering of the option he chose. The founder has no remote to this window.

**What this wrap is.** A 0-pending adaptation wrap. No source file changes in it. If the adaptation form cannot write one
of the items below, say which and why, and carry it to the corrective chunk's own wrap — do not stretch the form.

## 1. Route — one corrective entry FIRST in Epoch 5 («Тест + CI»)
Add one entry at the head of Epoch 5, ahead of "Upstream sync ahead of agent surfaces": a corrective chunk from the
Epoch 4 code audit (`.andromeda/runs/2026-10-09T14-20-43-code-audit/proposals.md`). Items, for your disposition:
- the one surviving mutant, `packages/dioxus-native-dom/src/snapshot.rs:187:9` (`delete match arm "textarea" in value`):
  killed by a check that types into a textarea and reads its value from the snapshot. No stand task holds a textarea, so
  it is a minimal fixture.
- CI's system-package step (`sudo apt-get update && … install libfontconfig1-dev`) gets a timeout and a retry. It hung
  in four CI runs on 2026-10-07, each cancelled and re-run by hand. Its owner today is "Quality gates" in Epoch 6: move
  that part up to the corrective entry and leave the rest of "Quality gates" as it is.

## 2. Ratified PROVISIONAL items — the marks come off
- **The log sink prints one line per closed span of an admitted target** («Утвердить как есть»): ratified as built.
- **The diff's three sizes may reach a log** («Числа допустимы»): his sentence that a diff leaves the process through the
  returned value only, with no log carrying it, stands; it gains the clarification that the diff's CONTENT never reaches a
  log, while its three sizes — the counts of added, removed and changed — may.
- **An in-process driver call that returns the snapshot text and the diff is not a boundary widening** («Не расширение»):
  it is the returned value his rule allows. Nothing is reworded for it beyond any mark that waits on this answer.

## 3. Playbook — one rule appended («Добавить»)
"A spec claim a chunk's measurement disproves, about code the chunk did not edit, is amended to the measured limit and its
fix pinned as a CARRY; routine."

## 4. `select` — a residual («Остаток до первой нужды»)
The missing `select` interaction model is a residual outside 0.1.0, built when a stand task or a fixture first needs one.
The old carry that named "no `select` or range interaction model" no longer owns `select`; range keeps its entry.

## 5. What this wrap also carries
- The two boundary run dirs, uncommitted until now: the diagnosis (`2026-10-09T14-03-28-evolve-diagnose`) and the code
  audit (`2026-10-09T14-20-43-code-audit`), with the audit's record in `.andromeda/code-metrics.ndjson`.
- escher's FIRST citation sweep: follow the wrap letter — read every row, write the read file, then ask the operator.
- NOT in this wrap: the review of deferred (Tier 3) learnings. It follows, with the founder, as its own step.
