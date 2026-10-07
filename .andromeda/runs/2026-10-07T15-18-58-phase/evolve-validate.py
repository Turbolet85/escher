import json

TS = "2026-10-07T18:49:37Z"
ROUTE = "escher-0.1.0/working-route.md"
epoch = None
for line in open(ROUTE, encoding="utf-8").read().split("\n"):
    if line.startswith("### Epoch 4"):
        epoch = line[4:]
        break
assert epoch

base = {
    "v": 1,
    "version": "escher-0.1.0",
    "epoch": epoch,
    "chunk": "2026-10-07-refusal-detection",
    "skill": "andromeda-phase",
    "step": "validate",
}
RUN = ".andromeda/runs/2026-10-07T15-18-58-phase"

step = dict(base, kind="step", ts=TS, id=TS + "-a")
step.update({
    "outcome": "ok",
    "counts": {"dialogue_rounds": 2, "iterations": 1, "retries": 1},
    "consumed": [
        {"artifact": "plan", "quality": "ok",
         "note": "the check set was run whole three times in this window (before the first card, after one edit, after the re-plan); the last reading: planlint 0 hits, 18 gate entries parsed, 5 new entries baselined"},
        {"artifact": "scope", "quality": "ok",
         "note": "validation-1 read aligned before the first card; the review then showed the scope incomplete, and it read aligned again after the amendment"},
        {"artifact": "matrix", "quality": "ok"},
    ],
    "produced": [
        {"artifact": "matrix", "signals": ["acceptance-concretized", "claims-refined-0", "declined-0"],
         "note": "v010-11 claimed, one concretization previewed on both cards, method integration kept; audit read it cited at plan.md:395; no matrix.py call carried --run-dir before the word"},
        {"artifact": "scope", "signals": ["amended-at-review"],
         "note": "the intent-incomplete arm, on the operator's directive snapped as inputs#I3"},
    ],
    "problem": [
        {"nature": "process", "solution": "workaround",
         "note": "inputs.py snap takes --step from phase:P1, phase:P3 and implement only; the approving word, given at P5, was first sent with phase:P5 (exit 2) and then snapped as phase:P3, like the review directive before it"},
        {"nature": "process", "solution": "workaround",
         "note": "the approving word carried an instruction for the chunk's report (flag the engine fix as upstreamable); plan.md was not edited after the word, so the instruction is held verbatim in inputs#I4 and stated in this run's report"},
    ],
})

f1 = dict(base, kind="friction", ts=TS, id=TS + "-b")
f1.update({
    "type": "contract.mechanical-check",
    "what": "the first plan passed every mechanical check and validation-1 while its off-screen remedy could not be followed on the stand: scroll moved the viewport only and the stand's scrollers are nested; the operator's review caught it, no predicate attempts it, and the run returned to P4",
    "impact": {"iterations": 1},
    "artifacts": ["plan.md", "scope.md"],
    "evidence": RUN + "/relay-3.md",
})

f2 = dict(base, kind="friction", ts=TS, id=TS + "-c")
f2.update({
    "type": "ambiguity.review-cycles",
    "what": "the review prompt was issued as text three times over three context windows: once with no word taken (context alarm), once answered by a directive, once answered yes; the yes rested on the founder's word for an engine change for every document, given in another session and relayed",
    "impact": {"dialogue_rounds": 2},
    "artifacts": ["plan.md"],
    "evidence": RUN + "/relay-4.md",
})

print("\n".join(json.dumps(r, ensure_ascii=False) for r in (step, f1, f2)))
