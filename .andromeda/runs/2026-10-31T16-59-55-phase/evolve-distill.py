import json

ts = "2026-10-31T17:40:10Z"
base = {"v": 1, "ts": ts, "version": "escher-0.3.0", "epoch": "Epoch 2 — Engine debts",
        "chunk": "2026-10-31-select-popup", "skill": "andromeda-phase", "step": "distill"}
signals = ["agents-distill:7", "first-distill:7", "agents-task:6", "first-task:6", "agents-catread:51",
           "first-catread:51", "wall-stage1:523", "agents-classify:237", "first-classify:235", "wall-classify:877",
           "agents-strong:30", "first-strong:29", "wall-strong:465", "agents-history:7", "first-history:7",
           "wall-history:275", "respawn-agents:2", "respawn-every-entity:2", "wall-respawn:75", "unchecked:0",
           "move-moved:1", "move-held:1", "entities:1155", "classify-yes:451", "one-draw-yes:64", "false-yes:103",
           "lost-classify:30", "lost-strong:19", "listed:36", "standing:57", "draws:3", "sampled:1", "forced:1",
           "guard-refused:1"]
docs = ["architecture", "security-plan", "design-system", "layout-templates", "test-plan", "obs-plan", "a11y-plan"]
consumed = [{"artifact": "spec:" + d, "quality": "ok"} for d in docs]
consumed += [{"artifact": "sidecar:" + d, "quality": "ok"} for d in docs]
consumed.append({"artifact": "scope", "quality": "ok"})
step = dict(base, kind="step", id=ts + "-a", outcome="ok",
            counts={"retries": 2, "halted": 0}, consumed=consumed,
            produced=[{"artifact": "extracts", "signals": signals,
                       "note": "the guard refused one listing loop's stdout redirect, re-issued bare and ran; "
                               "0 of 7 no-coverage; history read whole 4, through the index 3; "
                               "classify one script of 491,317 B, no parts"}],
            problem=None)
frictions = [
    dict(base, kind="friction", id=ts + "-b", type="retry.distiller-respawn",
         what="classifier security-cls-input-validation.b5.d3 failed EVERY_ENTITY on a stray row 'yes: 2' under "
              "its answers; one re-spawn passed",
         impact={"retries": 1}, evidence=".andromeda/runs/2026-10-31T16-59-55-phase/"
                                         ".raw-security-cls-input-validation.b5.d3.md"),
    dict(base, kind="friction", id=ts + "-c", type="retry.distiller-respawn",
         what="classifier tests-cls-test-strategy.b1.d3 failed EVERY_ENTITY, an entity missing; one re-spawn passed",
         impact={"retries": 1}, evidence=".andromeda/runs/2026-10-31T16-59-55-phase/"
                                         ".raw-tests-cls-test-strategy.b1.d3.md"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in [step] + frictions))
