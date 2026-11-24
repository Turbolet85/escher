import json
ts = "2026-11-24T09:31:30Z"
base = {"v": 1, "ts": ts, "version": "escher-0.4.0", "epoch": "Epoch 4 — Inside the canvas",
        "chunk": "2026-11-24-upstream-sync-ahead-of-inside-the-canvas", "skill": "andromeda-phase", "step": "validate"}
rs = [
    dict(base, kind="step", id=ts + "-a", outcome="ok",
         counts={"dialogue_rounds": 2, "iterations": 1, "halted": 0},
         consumed=[
             {"artifact": "plan", "quality": "thin", "note": "rev 5 passed every mechanical check but carried two defects the operator's review caught: a waived row re-run and trial deletes"},
             {"artifact": "scope", "quality": "ok", "note": "amended at rev 4 with the founder's answers and the two adaptations before the row read re-ran"},
             {"artifact": "matrix", "quality": "ok"}],
         produced=[
             {"artifact": "matrix", "signals": ["claims-declined:1"], "note": "v040-02 noted, not claimed: two later sync entries carry it"},
             {"artifact": "scope", "signals": ["val1-intent-incomplete-amended"], "note": "the take-up's stop-at-forks clause closed by inputs#I2"}],
         problem=[
             {"nature": "process", "solution": "overridden", "note": "the operator ordered the P2 row read re-run whole for the scope that gained layout_reason.rs, where the plan had leaned past it; it re-ran (7 tasks, 117 pickers, union, 7 histories) and its rows were folded"},
             {"nature": "process", "solution": "overridden", "note": "the operator struck plan step 1's removal of the trial worktree and its build directories; they stay in place"},
             {"nature": "resources", "solution": "deferred", "note": "the operator's quiet window for another session stopped the leg baselines mid-fast (exit -15); fast and the rest re-ran after the go, all green"}]),
    dict(base, kind="friction", id=ts + "-b", type="contract.mechanical-check",
         what="the plan waived a P2 re-run the letter orders for a scope gaining a file, on a lean; no mechanical check reads a corrected scope against the pin, the operator's review caught it",
         impact={"dialogue_rounds": 1, "iterations": 1}, artifacts=["plan", "scope"],
         evidence=".andromeda/runs/2026-11-24T06-26-40-phase/p5-card-1.md"),
    dict(base, kind="friction", id=ts + "-c", type="contract.mechanical-check",
         what="plan step 1 deleted the trial worktree and two build directories the operator keeps; no check reads a step's deletes, the operator's review caught it",
         impact={"dialogue_rounds": 1}, artifacts=["plan"],
         evidence=".andromeda/runs/2026-11-24T06-26-40-phase/p5-card-2.md"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
