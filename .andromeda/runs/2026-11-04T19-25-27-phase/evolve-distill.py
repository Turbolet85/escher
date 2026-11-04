import json

ts = "2026-11-04T20:06:17Z"
base = {"v": 1, "ts": ts, "version": "escher-0.3.0", "epoch": "Epoch 3 — The UI graph",
        "chunk": "2026-11-04-dioxus-runtime-probe", "skill": "andromeda-phase", "step": "distill"}
signals = ["agents-distill:7", "first-distill:7", "agents-task:6", "first-task:6", "agents-catread:51", "first-catread:50", "wall-stage1:337", "agents-classify:237", "first-classify:235", "wall-classify:1022", "agents-strong:24", "first-strong:21", "wall-strong:317", "agents-history:7", "first-history:7", "wall-history:186", "respawn-agents:5", "respawn-carried-from-picked:2", "respawn-every-entity:2", "respawn-own-file:1", "respawn-quotes-verbatim:2", "wall-respawn:277", "respawn-kept:78", "unchecked:0", "move-moved:1", "move-held:0", "entities:1352", "classify-yes:322", "one-draw-yes:77", "false-yes:180", "lost-classify:25", "lost-strong:16", "listed:16", "standing:74", "draws-dead:0", "batches-on-one:0", "draws:3", "sampled:1", "forced:1"]
consumed = [{"artifact": "spec:" + d, "quality": "ok"} for d in
            ["architecture", "security-plan", "design-system", "layout-templates", "test-plan", "obs-plan", "a11y-plan"]]
consumed += [{"artifact": "sidecar:" + d, "quality": "ok"} for d in
             ["architecture", "security-plan", "design-system", "layout-templates", "test-plan", "obs-plan", "a11y-plan"]]
consumed.append({"artifact": "scope", "quality": "ok"})
rs = [
    dict(base, kind="step", id=ts + "-a", outcome="ok", counts={"retries": 5, "halted": 0},
         consumed=consumed,
         produced=[{"artifact": "extracts", "signals": signals + ["binding-unilateral"],
                    "note": "0 @none; the test-plan history alone surfaced upstream 286874cc1's task_polled_during_render_can_access_document, the fix option no extract carried"}],
         problem=None),
    dict(base, kind="friction", id=ts + "-b", type="retry.distiller-respawn",
         what="category reader arch-cat-conventions failed [OWN_FILE] (three scope fct citations); re-spawned once, passed",
         impact={"retries": 1}, artifacts=["arch-cat-conventions"]),
    dict(base, kind="friction", id=ts + "-c", type="retry.distiller-respawn",
         what="classifiers tests-cls-test-strategy.b1 and .b1.d3 failed [EVERY_ENTITY] (missing test-strategy#2.2); re-spawned once, both passed, no dead draw",
         impact={"retries": 2}, artifacts=["tests-cls-test-strategy.b1", "tests-cls-test-strategy.b1.d3"]),
    dict(base, kind="friction", id=ts + "-d", type="retry.distiller-respawn",
         what="strong readers arch-cascade-g5 and a11y-cascade-g3 failed [CARRIED_FROM_PICKED, QUOTES_VERBATIM] (cited facts no picked entity holds); re-spawned once, both passed",
         impact={"retries": 2}, artifacts=["arch-cascade-g5", "a11y-cascade-g3"]),
    dict(base, kind="friction", id=ts + "-e", type="contract.binding-contradiction",
         what="check C tension, unilateral: design says stand readings need blitz-dom's system-fonts feature for blitz-tests, tests says stand checks use bundled DejaVu Sans with system fonts off; carried to P3/P4, no retry",
         impact={}, artifacts=["design", "tests"]),
    dict(base, kind="friction", id=ts + "-f", type=None, untyped=True,
         what="fanout.py render --batch classify-p1 --respawn refused at usage (exit 2): a part's re-spawn renders with --batch classify and the tool names it classify-p1-r1; re-fired",
         impact={"retries": 1}, artifacts=["fanout-contract.md §The batch 1a"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
