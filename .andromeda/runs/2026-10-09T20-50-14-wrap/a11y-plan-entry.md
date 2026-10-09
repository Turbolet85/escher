
## 2026-10-09-audit-corrections-agent-surfaces — the value reading is proven on a textarea
**Section:** §7 Screen Reader Support → Accessibility tree output (the snapshot-model bullet)
**Change:** the clause "`value` is the editor's text, else the `value` attribute" now says the reading holds for a textarea as for a text input: on the in-file fixture of `stand_snapshot_state` a textarea named by `aria-label` reads role `MultilineTextInput`, a non-empty name, its HTML `id` as its stable id and a box wider and taller than 0, reads `""` untyped and its typed text back as its `value`, unmasked, and after the click exactly it reads `focused` and is the tree's focus, in both layout modes; it is a test-only element of that fixture. Nothing was retired: §7 held no textarea value reading before.
**Why:** the snapshot's `value` reader already took a textarea, and no check read one — the one mutant the Epoch 4 code audit left alive. The textarea is named by `aria-label` because a `label for` binds an `input` element only.
**Kept:** the password and file-input mask clauses are unchanged; the textarea's text is synthetic and reads back unmasked by design.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/
