"""P4+P5: diff record.json vs the baseline record (re-found by baseline_sha), judge per audit-pass.md §Thresholds,
render proposals.md with programmatic row-count asserts.

Usage: render.py {run_dir}
"""
import json, re, sys

RD = sys.argv[1]
rec = json.load(open(f"{RD}/record.json", encoding="utf-8"))
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')
recs, bad = [], 0
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try: recs.append(json.loads(line))
    except ValueError: bad += 1
# the baseline: the LAST record BEFORE this run's own append whose sha == baseline_sha
own = [i for i, r in enumerate(recs) if r.get("ts") == rec["ts"] and r.get("sha") == rec["sha"]]
upto = own[-1] if own else len(recs)
base = [r for r in recs[:upto] if r["sha"] == rec["baseline_sha"]][-1]
n_records = len(recs)
UNIT = rec["mutation"]["scoped_units"][0]
mc = rec["mutation"]["counts"][UNIT]

# tool-version trend-breaks (leading token vs leading token)
tb = [k for k, v in rec["tool_versions"].items() if base["tool_versions"].get(k) is not None and
      str(base["tool_versions"][k]).split()[0] != str(v).split()[0]]
assert not tb, tb

d = lambda a, b: round(b - a, 3)
B, C = base, rec
prop, info, below = [], [], []

# --- thresholds (span = 1) ---
if C["graph"]["cycles"] > B["graph"]["cycles"]:
    prop.append("new-cycle")
dp_abs = C["duplication"]["pct"] - B["duplication"]["pct"]
if dp_abs >= 0.5 and C["duplication"]["pct"] >= B["duplication"]["pct"] * 1.15:
    prop.append("duplication-up")
else:
    below.append(f"duplication-up · `duplication.pct` {B['duplication']['pct']} → {C['duplication']['pct']} ({d(B['duplication']['pct'], C['duplication']['pct']):+} pt) — fell; fires at ≥ +0.5 pt AND ≥ +15 %")
oc_b, oc_c = B["complexity"]["over_ceiling"], C["complexity"]["over_ceiling"]
if oc_c - oc_b >= 3 and oc_c >= oc_b * 1.25:
    prop.append("complexity-creep")
else:
    below.append(f"complexity-creep · `complexity.over_ceiling` {oc_b} → {oc_c} (+{oc_c - oc_b}, {100 * (oc_c - oc_b) / oc_b:+.1f} %) — the absolute floor (+3) is met, the relative one (+25 %) is not. "
                 "The +3 equals the three functions over cognitive 15 in files CREATED this epoch: `walk` packages/dioxus-native-dom/src/element_id.rs:72 (26) · "
                 "`unkeyed_elements_read_their_component_path` tests/blitz-tests/tests/stand_element_ids.rs:171 (21) · `ids_hold_across_a_remount` tests/blitz-tests/tests/stand_id_persistence.rs:189 (17)")
dz_b, dz_c = B["dead"]["zero_ref_candidates"], C["dead"]["zero_ref_candidates"]
if dz_c - dz_b >= 5:
    prop.append("dead-growth")
else:
    below.append(f"dead-growth · `dead.zero_ref_candidates` {dz_b} → {dz_c} ({dz_c - dz_b:+}) — same recipe, verbatim (recipes.dead); raw zero-ref rows {B['dead']['zero_ref_rows']} → {C['dead']['zero_ref_rows']}, "
                 f"test-path exclusions {B['dead']['excluded_test_path']} → {C['dead']['excluded_test_path']}, trait-impl {B['dead']['fp_classes']['trait-impl (dispatch)']} → {C['dead']['fp_classes']['trait-impl (dispatch)']}; other FP classes unchanged")
cl_b, cl_c = B["coverage"]["line"], C["coverage"]["line"]
if cl_c <= cl_b - 2:
    prop.append("coverage-drop")
else:
    below.append(f"coverage-drop · `coverage.line` {cl_b} → {cl_c} ({d(cl_b, cl_c):+} pt) — a rise, never celebrated (trend-only; mutation is the honest half); branch null at both records (llvm-cov reports no branch data)")
below.append(f"mutation-drop · not evaluable: `{UNIT}` has no prior scored value (first scored this run); `escher-telemetry` (scored {B['mutation']['scores'].get('escher-telemetry')} at the baseline) was not touched this epoch and not scoped")
below.append(f"monotonic · not evaluable this run: it needs two diffs (three chained records) — the baseline record `{B['sha'][:8]}` has no `baseline_sha`, so the chain holds one diff. "
             "Tracked scalars this diff: `duplication.pct` " + f"{B['duplication']['pct']}→{C['duplication']['pct']} · `complexity.over_ceiling` {oc_b}→{oc_c} · `dead.zero_ref_candidates` {dz_b}→{dz_c} · "
             f"`sizes.file_max` {B['sizes']['file_max']}→{C['sizes']['file_max']} · `sizes.over_800` {B['sizes']['over_800']}→{C['sizes']['over_800']} · `coverage.line` {cl_b}→{cl_c}")
below.append(f"new-cycle · `graph.cycles` {B['graph']['cycles']} → {C['graph']['cycles']}; `cross_unit_edges` {B['graph']['cross_unit_edges']} → {C['graph']['cross_unit_edges']}; `fan_out` per unit identical; `fan_in_top` same 20 symbols "
             f"(top: `{C['graph']['fan_in_top'][0][0]}` {B['graph']['fan_in_top'][0][1]} → {C['graph']['fan_in_top'][0][1]})")
below.append(f"sizes · `file_max` {B['sizes']['file_max']} → {C['sizes']['file_max']} (packages/blitz-dom/src/document.rs, −{B['sizes']['file_max'] - C['sizes']['file_max']}) · `over_800` {B['sizes']['over_800']} → {C['sizes']['over_800']} · "
             f"p50 {B['sizes']['file_p50']} → {C['sizes']['file_p50']} · p90 {B['sizes']['file_p90']} → {C['sizes']['file_p90']}")
below.append(f"complexity percentiles · cyclomatic p50/p90 {B['complexity']['cyclomatic_p50']}/{B['complexity']['cyclomatic_p90']} → {C['complexity']['cyclomatic_p50']}/{C['complexity']['cyclomatic_p90']} · "
             f"cognitive p50/p90 {B['complexity']['cognitive_p50']}/{B['complexity']['cognitive_p90']} → {C['complexity']['cognitive_p50']}/{C['complexity']['cognitive_p90']} · "
             f"`compute_inline_layout_inner` 140 → 129 (no longer the max; max now `synthesize_presentational_hints_for_legacy_attributes` 135, unchanged)")
ud_same = sorted(map(tuple, B["dead"]["unused_deps"])) == sorted(map(tuple, C["dead"]["unused_deps"]))
below.append(f"unused deps (cargo-machete) · {len(B['dead']['unused_deps'])} → {len(C['dead']['unused_deps'])} — {'the identical list' if ud_same else 'list changed'}")
below.append(f"populations (never movements) · `totals.loc` {B['totals']['loc']} → {C['totals']['loc']} · `totals.files` {B['totals']['files']} → {C['totals']['files']} · `duplication.total_lines` {B['duplication']['total_lines']} → {C['duplication']['total_lines']}")

# --- informational ---
db, dc = B["duplication"], C["duplication"]
pop_rel = 100 * (dc["total_lines"] - db["total_lines"]) / db["total_lines"]
sp = lambda k: f"{k} {db['split'][k]['pairs']}→{dc['split'][k]['pairs']} pairs / {db['split'][k]['lines']}→{dc['split'][k]['lines']} L"
info.append(f"count-under-ratio · `duplication.clones` {db['clones']} → {dc['clones']} ({dc['clones'] - db['clones']:+}) and `duplicated_lines` {db['duplicated_lines']} → {dc['duplicated_lines']} "
            f"({dc['duplicated_lines'] - db['duplicated_lines']:+}) while `pct` {db['pct']} → {dc['pct']} ({d(db['pct'], dc['pct']):+} pt); population `total_lines` {pop_rel:+.2f} % "
            f"(duplicated lines {100 * (dc['duplicated_lines'] - db['duplicated_lines']) / db['duplicated_lines']:+.2f} %) · split: {sp('src')} · {sp('test')} · {sp('mixed')} · "
            f"top standing pair `{dc['top'][0][0]}` ↔ `{dc['top'][0][1]}` {dc['top'][0][2]} L, standing since the Epoch 1 — Foundation baseline record")
bt = {r[0] for r in B["sizes"]["top"]}
ent = [r for r in C["sizes"]["top"] if r[0] not in bt]
info.append("top-N entrants · `sizes.top`: " + ", ".join(f"`{r[0]}` {r[1]}" for r in ent) +
            " (entered; packages/blitz-dom/src/node/element.rs 850 left the top 10, still > 800) · `duplication.top`, `complexity.top`, `dead.top`, `fan_in_top`: no entrants")
info.append(f"churn · {C['churn']['pct']} % of source adds landed in a file's 2nd..nth touching commit this epoch ({C['churn']['files_churned']} files churned of 28 touched) — "
            f"informational while the ledger held < 2 records at read (it held {n_records - 1} before this append)")
info.append("hotspots (first values — the baseline skipped them, `no-baseline`) · " + " · ".join(f"`{f}` {s}" for f, s in C["hotspots"]))
st = rec["mutation"]["unit_states"][UNIT]
score = rec["mutation"]["scores"].get(UNIT)
info.append(f"mutation · `{UNIT}` first scored this run: {score} % ({mc['caught']} caught / {mc['caught'] + mc['missed']}; {mc['unviable']} unviable · {mc['timeout']} timeout · "
            f"{mc['not_measured']} not measured on this host; `{st}`) — scope: the epoch's touched files of the unit (57 of the crate's 315 mutants); no prior score, so mutation-drop has no subject")
info.append("corrections carried in this record (to the baseline `d4113768`): `commands.complexity` (the recorded form omits the `mkdir -p` its replay needs) · `recipes.dead` (schema-gap fill — the recipe text now rides the ledger)")
info.append("C1 invocation note · attempts 1–3 of the mutation tier aborted before testing any mutant (`c1-attempts.txt`): cargo-mutants 27.1.0 runs its UNMUTATED baseline "
            "over the mutated package only, ignoring `--test-package` / `--test-workspace` (src/lab.rs `run_baseline`), so the `--test=stand_*` args named targets absent from "
            "`dioxus-native-dom`; per-mutant runs honour `--test-package`. Run 4 used `--baseline=skip` (the baseline record's own form) after the unmutated tree was proven green "
            "by hand with the same test set — a runner-portability fact, not a test failure. Run 4 was STOPPED at 4 caught / 17 unviable: the unviable builds failed "
            "`Disk quota exceeded (os error 122)` on the /tmp tmpfs holding the two build copies — an environment fault presenting as unviable; its partial tally is "
            "never read. Run 5 moved the copies to the gitignored `target/mutants-tmp` via TMPDIR and hit the 15-min "
            "wall-clock cap at 40/57 (partial, never scored — `budget-exhausted` by letter); the operator chose a re-run under a 30-min cap: run 6 is the scored invocation. "
            "Attempt evidence: `c1-attempts.txt`")

# survivors rendered in full; n from counts (independent source)
surv = rec["mutation"]["survivors"]
assert len(surv) == mc["missed"] and len({tuple(s) for s in surv}) == len(surv)
nm = rec["mutation"]["not_measured"]
assert len(nm) == mc["not_measured"]

L = []
L.append(f"# Code Audit — escher · {rec['epoch']} · {rec['ts']}")
L.append(f"mode {rec['mode']} · HEAD {rec['sha']} · baseline {rec['baseline_sha']} · span {rec['span']}")
ho = rec["head_overshoot"]
L.append(f"{ho['commits']} commit past the boundary `{ho['boundary_sha'][:8]}` (`42b80ad9` chore(session): the founder's PROVISIONAL batch) · source delta files: none — every metric is the boundary state; "
         f"the baseline record's overshoot: `commits: 0` (at its boundary)")
L.append("no ancestry break · no trend-break (all eight tool tokens equal the baseline's)")
L.append("")
L.append("## Proposals")
if prop:
    raise SystemExit(f"proposals fired — render them by hand: {prop}")
L.append("None. No threshold fired at span 1.")
L.append("")
L.append("## Informational")
for x in info:
    L.append(f"- {x}")
L.append("")
L.append(f"### Survivors — `{UNIT}` ({mc['missed']})")
if surv:
    L.append("| site | mutation |")
    L.append("|---|---|")
    for s, m in surv:
        L.append(f"| `{s}` | {m} |")
else:
    L.append("none")
L.append(f"\nnot measured on this host ({rec['mutation']['host']}): {len(nm)}" + ("" if not nm else ": " + "; ".join(f"`{a}` {b} — {c}" for a, b, c in nm)))
L.append("no project union verdict")
L.append("")
L.append("## Below threshold — no action")
for x in below:
    L.append(f"- {x}")
L.append("")
L.append("## Skips")
for s in rec["skips"]:
    L.append(f"- {s['metric']} — {s['reason']}" + (" (touched this epoch; scope chosen at the attended confirm: the dioxus-native-dom epoch files only)" if s["reason"] == "declined" else ""))
open(f"{RD}/proposals.md", "w", encoding="utf-8").write("\n".join(L) + "\n")
# row-count assert on the rendered survivor table
rows = [l for l in L if l.startswith("| `")]
assert len(rows) == mc["missed"], (len(rows), mc["missed"])
print(f"proposals {len(prop)} · informational {len(info)} · below {len(below)} · skips {len(rec['skips'])} · ledger records {n_records}")
