# Escalations — 2026-10-10-upstream-sync-agent-surfaces wrap, Phase 2

Two proposals were staged to escalate at Validate (`fanout-results.md`) and resolved with the operator before any amendment was applied. Both answers were given through this wrap's question dialog.

## 1 · security-plan §Input Validation → `@font-face` source (security-plan proposal 2)

- **Why it escalated:** a boundary widening, the playbook's never-routine class. Upstream #1109, merged in this chunk, makes the engine fetch and decode a document-supplied font source it used to skip: an `@font-face` source with no format hint whose URL has no extension (a `data:` URL).
- **Recommended:** ratify and record.
- **The answer:** word: "Record as PROVISIONAL" — the operator, 2026-10-10, through the question dialog of this wrap.
- **The operator's note, verbatim:** "A boundary widening is the founder own word, also when it arrives by an upstream merge. State in the row what widened, that no test of ours or of the delta covers it, and that escher routes no agent- or user-supplied URL through this path today."
- **Applied as:** the row states the widened case under a PROVISIONAL mark awaiting the founder's ruling, with the three facts the note names. The two facts the note adds were read before they were written: `git diff 23354585 7832c177` holds 0 added lines naming `font-face` or `font_face`, and `packages/escher-driver/src` holds 0 occurrences of `url`.
- **Stands until:** the founder's own word, in the batch at the Epoch 5 boundary (playbook, Provisional discharge). The handoff names it.

## 2 · architecture §Established Decisions → [Unsupported features], the `text-indent` sentence (architecture proposal 10)

- **Why it escalated:** graded `escalate` by the detector, and no playbook rule governs: the plan's expected-amendment entry leaves the judgment to the wrap, and the report measured nothing of the sentence's cause. The sentence cited an upstream code comment that #1155 removed; the code still hands both flags to Parley; stylo is unmoved at 0.22.0.
- **Recommended:** keep the sentence, marked not established.
- **The answer:** word: "Keep, marked not established (Recommended)" — the operator, 2026-10-10, through the question dialog of this wrap.
- **Applied as:** the sentence is re-cited to the lines that pass the flags and its cause is labelled `recorded, not established`.

No playbook rule is proposed: the first is the class the playbook never makes routine, and the second is one occurrence.
