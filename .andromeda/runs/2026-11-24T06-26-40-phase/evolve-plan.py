import json
ts = "2026-11-24T07:52:37Z"
base = {"v": 1, "ts": ts, "version": "escher-0.4.0", "epoch": "Epoch 4 — Inside the canvas",
        "chunk": "2026-11-24-upstream-sync-ahead-of-inside-the-canvas", "skill": "andromeda-phase", "step": "plan"}
rs = [
    dict(base, kind="step", id=ts + "-a", outcome="ok",
         counts={"dialogue_rounds": 0, "iterations": 1, "extra_reads": 3},
         consumed=[
             {"artifact": "rows", "quality": "ok", "note": "rows sum past 250 000 B, read by search; every handle the plan cites found in its rows file"},
             {"artifact": "research", "quality": "thin", "note": "the msrv and coverage legs, both CI jobs, were not measured at P3; measured at P4 on the trial"},
             {"artifact": "scope", "quality": "ok"}],
         produced=[{"artifact": "plan", "signals": ["designed-dialogue:left-out-recheck", "authority-resolved:pinned-scope row re-read yielded to the operator's resume-at-P4 word, surfaced on the card"],
                    "note": "20 gate entries, 11 criteria; the founder's L asked for a measurement, written as evidence/left-out-recheck.md"}],
         problem=[
             {"nature": "process", "solution": "workaround", "note": "T1 A added layout_reason.rs to a pinned scope that did not name it; the P2 re-run was not fired, leaned on the rows already picked and named on the P5 card for the operator to order"},
             {"nature": "environment", "solution": "workaround", "note": "the coverage leg on the trial build dir read a session test red on the 94-byte socket bound (109 bytes); re-run under the short build dir target/tic, green"}]),
    dict(base, kind="friction", id=ts + "-b", type="input.research-thin",
         what="research measured fast, audit, a11y and doc on the trial but not msrv or coverage, both CI jobs; a second and third trial run at P4 filled them",
         impact={"retries": 1, "extra_reads": 2}, artifacts=["research"],
         evidence=".andromeda/cache/chunks/2026-11-24-upstream-sync-ahead-of-inside-the-canvas/evidence/left-out-recheck.md"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
