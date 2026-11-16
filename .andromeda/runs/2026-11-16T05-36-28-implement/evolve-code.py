import json

ts = "2026-11-16T06:11:54Z"
base = {"v": 1, "ts": ts, "version": "escher-0.3.0", "epoch": "Epoch 5 — Errors and explanation",
        "chunk": "2026-11-16-errors-with-the-action", "skill": "andromeda-implement", "step": "code"}
rs = [
    dict(base, kind="step", id=ts + "-a", outcome="halted-resolved",
         counts={"extra_reads": 6, "soft_exit": 0, "dialogue_rounds": 2, "halted": 2},
         consumed=[
             {"artifact": "plan", "quality": "thin", "note": "two steps contradicted reality: every-level capture vs the count click reading no caused (framework trace records), and the Err case's count-after and no-text claims vs the root boundary replacing the app"},
             {"artifact": "research", "quality": "ok", "note": "every listed path existed or was new; two companions outside the lists (session_common/mod.rs, stand_act_spans.rs)"},
             {"artifact": "scope", "quality": "ok"},
             {"artifact": "source", "quality": "ok"}],
         produced=[{"artifact": "source", "signals": ["style-inferred", "guard-refused:2"],
                    "note": "the bash guard refused a cd into the cargo registry and a cat heredoc to a scratch file; each re-issued once (absolute paths; Edit tool)"}],
         problem=[
             {"nature": "product-logic", "solution": "overridden", "note": "the plan counts logs at every level from every outside target; dioxus framework crates trace on every re-render, so the operator ruled framework targets count at warn+ only (inputs#I5); the dioxuslabs warnings crate joined that set when the check read its trace records"},
             {"nature": "product-logic", "solution": "overridden", "note": "the root default error boundary replaces the whole app and renders the Err text; the operator ruled the Err case runs on its own session with only caused read for sentinels (inputs#I6)"}]),
    dict(base, kind="friction", id=ts + "-b", type="input.plan-step-ambiguous",
         what="plan step 1 (every level, every outside target) and step 8 (the count click answers no caused; Counter checks unmoved) were jointly unsatisfiable: dioxus-core/signals/generational-box/warnings trace on every re-render",
         impact={"dialogue_rounds": 1, "halted": 1}, artifacts=["plan"],
         evidence=".andromeda/runs/2026-11-16T05-36-28-implement/relay-1.md"),
    dict(base, kind="friction", id=ts + "-c", type="input.plan-step-ambiguous",
         what="plan step 8's Err case (count after it; no Err byte on the answer line) contradicted dioxus-core 0.7.10's root ErrorBoundary, which replaces the app with the error's Display",
         impact={"dialogue_rounds": 1, "halted": 1}, artifacts=["plan"],
         evidence=".andromeda/runs/2026-11-16T05-36-28-implement/relay-2.md"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
