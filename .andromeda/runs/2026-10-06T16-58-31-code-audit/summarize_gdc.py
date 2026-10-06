"""A4 graph, A5 dead, A6 coverage summaries -> c-graph.json, c-dead.json, c-coverage.json.

Usage: summarize_gdc.py {run_dir}
"""
import json, re, sys

RD = sys.argv[1]
g = json.load(open(f"{RD}/graph-raw.json", encoding="utf-8"))
PREFIX = re.compile(r"^rust-analyzer cargo (\S+) \S+ ")


def short(sym):  # "{crate} {descriptor}" — the baseline's fan_in_top form
    m = PREFIX.match(sym)
    return f"{m.group(1)} {sym[m.end():]}" if m else sym


graph = {"cycles": len(g["cycles"]), "cycle_paths": [r["path"] for r in g["cycles"]],
         "fan_in_top": [[short(r["callee"]), r["n"]] for r in g["fan_in"]],
         "fan_out": [[r["from_crate"], r["n"]] for r in g["fan_out"]],
         "cross_unit_edges": g["edges"][0]["n"]}
json.dump({"graph": graph}, open(f"{RD}/c-graph.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)

d = json.load(open(f"{RD}/dead-classed.json", encoding="utf-8"))
unused = []
cur = None
for line in open(f"{RD}/machete-raw.txt", encoding="utf-8"):
    m = re.match(r"^(\S+) -- \./.*Cargo\.toml:$", line.rstrip())
    if m:
        cur = m.group(1)
    elif cur and line.startswith("\t") and line.strip():
        unused.append([cur, line.strip()])
    elif not line.strip():
        cur = None
dead = {"unused_deps": unused, "zero_ref_candidates": d["zero_ref_candidates"],
        "top": [c[:2] for c in d["candidates"][:10]], "fp_classes": d["fp_classes"],
        "zero_ref_rows": d["zero_ref_rows"], "excluded_test_path": d["excluded_test_path"]}
json.dump({"dead": dead, "candidates_by_kind": d["candidates_by_kind"], "candidates_by_crate": d["candidates_by_crate"],
           "candidates": d["candidates"],
           "fp_class_note": "counts are candidates, never dead: entry points, runtime-invoked surfaces, test-only helpers, "
                            "trait-impl dispatch, derive/attr-invoked and format-string captures are classed out first"},
          open(f"{RD}/c-dead.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)

tot = [l for l in open(f"{RD}/coverage-raw.log", encoding="utf-8") if l.startswith("TOTAL")][-1].split()
# llvm-cov report columns: TOTAL regions missed cover% functions missed exec% lines missed cover% branches missed cover%
line_pct = float(tot[9].rstrip("%"))
branch = None if tot[12] == "-" else float(tot[12].rstrip("%"))
cov = {"line": line_pct, "branch": branch}
json.dump({"coverage": cov, "total_row": " ".join(tot)}, open(f"{RD}/c-coverage.json", "w", encoding="utf-8"), indent=1)
print(graph["cycles"], graph["cross_unit_edges"], len(unused), dead["zero_ref_candidates"], cov)
