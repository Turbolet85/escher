# escher 0.1.0 — requirements

*Source: `escher-0.1.0/intent.md` §Findings. Ids are version-scoped (`v010-NN`) because the project has no prior
capability numbering. Every capability is verified headless, by an agent, on the 7GUIs stand, or on a minimal fixture
beside it where the tasks lack the case (intent §Principles).*

- v010-01 · Stable element ids — every element of the stand carries a stable id: the author's key where given, else its component path (per intent §1)
- v010-02 · Id persistence — an element's id is identical across a re-render, a remount and a fresh process (per intent §1)
- v010-03 · Id coherence — an element has the same id in the snapshot, the accessibility tree and the driver (per intent §1)
- v010-04 · Compact semantic snapshot — one command returns the screen as a tree of id · role · name · state · bounds, readable whole in one tool result (per intent §2)
- v010-05 · Snapshot state fidelity — snapshot state reflects enabled, checked, value and focused; a disabled control reads disabled, a typed value reads its value (per intent §2)
- v010-06 · Post-action diff — after an action the driver returns exactly the nodes that changed; an action that changes nothing returns an empty diff (per intent §3)
- v010-07 · Headless screenshot — the driver screenshots the whole screen or one element with no display (per intent §4)
- v010-08 · Screenshot determinism — a repeated screenshot on the same host yields the same pixels (per intent §4)
- v010-09 · Act by id — driver actions target an element by its stable id (per intent §5)
- v010-10 · Settled actions — acting returns only once the UI has settled; a stand flow with a delayed update passes with no sleep anywhere in the check (per intent §5)
- v010-11 · Explainable refusals — an action that cannot happen is refused with its cause named: not found · stale · disabled · covered by another element · off-screen (per intent §6)
- v010-12 · Command line — every driver command runs from a CLI with JSON output and an exit code meaning accepted or refused; a stand flow is scriptable in shell alone (per intent §7)
- v010-13 · MCP surface — the same commands are exposed as MCP tools, and an agent session completes a stand flow through them (per intent §8)
- v010-14 · Self-describing API — one consistent verb set, with help and schemas served by the tool itself, and every refusal naming its remedy (per intent §9)
- v010-15 · Cold-agent test — a fresh agent given only the tool completes a stand task and writes a passing check, with its wrong calls counted (per intent §9; covers v010-01…v010-14 end to end; the count is 0.1.0's baseline, a bar on it starts in 0.2.0)
- v010-16 · Id stability across code edits — an element's id stays the same when the app's code is edited around it, not only across a re-render, a remount and a fresh process (per intent §1)
- v010-17 · Snapshot selection, toggle and validity states — snapshot state also reflects selected, pressed and invalid; the active flight mode, the selected row and an invalid date each read so on the stand (per the founder's ruling 2026-10-10)
