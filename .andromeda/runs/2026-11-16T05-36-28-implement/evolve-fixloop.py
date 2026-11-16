import json

ts = "2026-11-16T07:09:07Z"
base = {"v": 1, "ts": ts, "version": "escher-0.3.0", "epoch": "Epoch 5 — Errors and explanation",
        "chunk": "2026-11-16-errors-with-the-action", "skill": "andromeda-implement", "step": "fix-loop"}
rs = [
    dict(base, kind="step", id=ts + "-a", outcome="ok",
         counts={"iterations": 2, "retries": 1, "soft_exit": 0, "deferred": 0},
         consumed=[
             {"artifact": "plan", "quality": "ok", "note": "the ten local Test Commands ran as written"},
             {"artifact": "research", "quality": "ok"},
             {"artifact": "tests", "quality": "ok"},
             {"artifact": "source", "quality": "ok"}],
         produced=[
             {"artifact": "source", "signals": ["controls-held:4"], "note": "the four plan-named one-shot controls each read holds (control-1.txt)"},
             {"artifact": "matrix", "signals": []},
             {"artifact": "implement-outcome", "signals": ["green"]}],
         problem=None),
    dict(base, kind="friction", id=ts + "-b", type="tooling.result-not-run-stable",
         what="escher-telemetry's capture unit test read green alone and red inside the fast leg's workspace run: tracing-core's one-dispatcher mode cached a callsite's interest `never` from a thread with no subscriber; fixed by keeping two capture dispatchers registered for the process",
         impact={"iterations": 1, "retries": 1},
         artifacts=["packages/escher-telemetry/src/capture.rs"],
         evidence="$TMPDIR/andromeda-gate/2026-11-16-errors-with-the-action/implement-2026-11-16T05-36-28/8.log"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
