# Session Handoff

**Last Updated:** 2026-10-18T02:01:00Z
**Branch:** build/escher-0.2.0 · 0 ahead of origin/build/escher-0.2.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-17-upstream-sync-engine-debts — upstream/main merged at `23b9b26a9`, the ARIA exits ratified, the docs reconciled

## Position
- Done: 2026-10-17-upstream-sync-engine-debts, the first Epoch 2 entry. Upstream/main is merged at `23b9b26a9` (merge `3eda07bc`), and the final HEAD `ae8c8daf` read CI#39056637407 green, 17/17.
- Next: `/andromeda-phase` to promote and plan «Engine debts» (v020-03), the last Epoch 2 entry. It carries a CARRY for the untested merged `aria-expanded` · `aria-disabled` · `aria-level` reads.

## Work done
- A three-window wrap. P1 ran in the first window. The second resumed on the operator's directive (inputs#I4) and stopped at P2's answer card. This window resumed on the founder's late answer (inputs#I7): P2 apply through P7.
- Reconcile: 56 amendments across all seven masters, in 15 sidecar entries (49 proposed, 7 raised by main). 1 escalation was resolved: the answer card's ten ARIA proposals, confirmed by the founder's «Подтвердить все 10 (Рекомендую)». No proposal was rejected.
- Citations: 266 re-pointed, 31 changed, 105 stretched.
- Leaves: pass mode. 11 comparers returned 35 findings: 33 applied by the tool, 2 written by hand (a regex backslash the tool refuses).
- Curation (directive e): the two auto-memory entries are homed in Tier 1, one new and one extension, and drained from the memory. Tier 1 also gained a rule for the pre-CI commit subject.

## Drift resolved
- **The bump crate leaves.** Its rows and citations are gone from architecture, security-plan, design-system, layout-templates and test-plan.
- **Upstream's ARIA reads.**
  - `aria-labelledby` is now a fourth name source, outranking `<label>`.
  - Descriptions (`aria-description` text, `aria-describedby` targets) are a fourth content class, to the platform adapter only. This is a boundary widening by upstream merge, ratified as upstream's behaviour by the founder.
  - The three closed values sit inside the states class.
- **keyboard-types 0.8.**
  - The harness re-exports `NamedKey`.
  - The macOS action modifier is now `META`.
  - `keyboard_compat` is excluded from coverage by filename.
- **Versions and pin.** The workspace is at 0.3.0-beta.4, test-that at 0.6.0, and the merge base at `23b9b26a9`.
- **cargo-deny.** The ignores go 5 → 8 (the zbus family, External decay).
- **The (security) criterion** «audit … with `deny.toml` unchanged» is UNMET as written (directive a). Its letter is unchanged and nothing was reverted; it rests on scope-record.md's `deny.toml` widening and the External decay cause.
- **Two facts the report lacked.**
  - util.rs's `SUPER` → `META`.
  - The `todo!()` arms for physical key codes are gone.
- **The report's coverage line was false.** It credited tests for the three closed ARIA reads, and none exist. test-plan §1 now records them observed absent.
- **test-plan §2's count** is now 127 files / 126 targets (directive c). The earlier count was one short.

## Notes
- The P7.1 treatment of entry 11 (the plan's preservation guard, red on `deny.toml` alone) is `--skip 11` on the operator's directive (b).
- Last failed command: none outstanding.

## Deferred learnings
- recurrence-despite-learning: host-linux «Paths & the working directory». A bare `cd` into a run dir moved this window's cwd, and the guard refused three such `cd`s at the earlier windows.
- A Tier 3 entry cites `host.rs:350` (`.claude/docs/session-learnings.md:135`), past the end of the file it named at the base. Which file it meant is unmeasured; it was left as it stands.
- 1 carried from earlier handoffs, not this session's to curate: read a CI job's verdict with `ci.py`, not a log pattern (confidence 0.7).
