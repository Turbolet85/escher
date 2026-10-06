"""Pinned summarizers (collectors.md A1/A2/A3/B1/B2) -> {run_dir}/c-{metric}.json.

Usage: summarize.py {run_dir} {baseline_sha}
Inputs: {run_dir}/tokei.json, {run_dir}/jscpd-abs/jscpd-report.json, {run_dir}/rca/**.json, {run_dir}/srcfiles.txt,
        git log over the source-path definition. Percentiles: nearest-rank.
"""
import json, math, os, re, subprocess, sys, collections

RD, BASE = sys.argv[1], sys.argv[2]
ROOT = os.getcwd() + "/"
SRC = [l.strip() for l in open(f"{RD}/srcfiles.txt", encoding="utf-8") if l.strip()]
SRCSET = set(SRC)
PATHSPEC = ["*.rs", ":!:.andromeda/**", ":!:.claude/**", ":!:scripts/**", ":!:docs/**", ":!:refs/**"]


def nr(vals, p):  # nearest-rank percentile on sorted values
    v = sorted(vals)
    return v[max(0, math.ceil(p / 100 * len(v)) - 1)] if v else None


def dump(name, obj):
    json.dump(obj, open(f"{RD}/c-{name}.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)


def rel(p):
    return p[len(ROOT):] if p.startswith(ROOT) else p


def is_test(p):
    return p.startswith("tests/") or "/tests/" in p


# ---- A3 sizes (population = srcfiles.txt) ----
tk = json.load(open(f"{RD}/tokei.json", encoding="utf-8"))
per_file = {}
for lang, d in tk.items():
    if lang == "Total":
        continue
    for rep in d.get("reports", []):
        per_file[rel(rep["name"])] = rep["stats"]["code"]
missing = sorted(SRCSET - set(per_file))
vals = [per_file[f] for f in SRC if f in per_file]
top = sorted(((f, per_file[f]) for f in SRC if f in per_file), key=lambda x: (-x[1], x[0]))[:10]
sizes = {"file_p50": nr(vals, 50), "file_p90": nr(vals, 90), "file_max": max(vals), "over_800": sum(v > 800 for v in vals),
         "top": [list(t) for t in top]}
totals = {"loc": sum(vals), "files": len(vals)}
dump("sizes", {"sizes": sizes, "totals": totals, "population": "srcfiles.txt", "population_files": len(SRC),
               "not_in_tokei": missing})

# ---- A1 duplication ----
jr = json.load(open(f"{RD}/jscpd-abs/jscpd-report.json", encoding="utf-8"))
st = jr["statistics"]["total"]
pairs = []
split = {k: {"pairs": 0, "lines": 0} for k in ("src", "test", "mixed")}
for c in jr["duplicates"]:
    a, b = rel(c["firstFile"]["name"]), rel(c["secondFile"]["name"])
    a_, b_ = sorted((a, b))
    pairs.append([a_, b_, c["lines"]])
    k = "test" if is_test(a) and is_test(b) else "src" if not is_test(a) and not is_test(b) else "mixed"
    split[k]["pairs"] += 1
    split[k]["lines"] += c["lines"]
pairs.sort(key=lambda x: (-x[2], x[0], x[1]))
dup = {"pct": round(st["percentage"], 3), "duplicated_lines": st["duplicatedLines"], "total_lines": st["lines"],
       "clones": st["clones"], "top": pairs[:10], "split": split}
dump("duplication", {"duplication": dup, "all_pairs_count": len(pairs)})

# ---- A2 complexity (per function space: cognitive.sum / cyclomatic.sum) ----
fns = []


def walk(space, file):
    for s in space.get("spaces", []):
        if s["kind"] == "function":
            m = s["metrics"]
            fns.append({"fn": s["name"], "file": file, "line": s["start_line"],
                        "cog": m["cognitive"]["sum"], "cyc": m["cyclomatic"]["sum"]})
        walk(s, file)


for dp, _, fs in os.walk(f"{RD}/rca"):
    for fn in fs:
        if fn.endswith(".json"):
            d = json.load(open(os.path.join(dp, fn), encoding="utf-8"))
            f = d["name"]
            f = rel(os.path.normpath(os.path.join(ROOT, f)) if not f.startswith("/") else f)
            if f in SRCSET:
                walk(d, f)
cog = [x["cog"] for x in fns]
cyc = [x["cyc"] for x in fns]
off = sorted(fns, key=lambda x: (-x["cog"], x["file"], x["line"]))
cx = {"cyclomatic_p50": nr(cyc, 50), "cyclomatic_p90": nr(cyc, 90), "cognitive_p50": nr(cog, 50), "cognitive_p90": nr(cog, 90),
      "over_ceiling": sum(c > 15 for c in cog), "max": {"fn": off[0]["fn"], "file": off[0]["file"], "val": off[0]["cog"]},
      "top": [[x["fn"], f"{x['file']}:{x['line']}", x["cog"]] for x in off[:10]]}
dump("complexity", {"complexity": cx, "functions": len(fns), "files_with_functions": len({x["file"] for x in fns})})
maxcog = collections.defaultdict(float)
for x in fns:
    maxcog[x["file"]] = max(maxcog[x["file"]], x["cog"])

# ---- B1 churn / B2 hotspots ----
log = subprocess.run(["git", "log", "--reverse", "--numstat", "--format=@%H", f"{BASE}..HEAD", "--", *PATHSPEC],
                     capture_output=True, text=True, check=True).stdout
touch = collections.defaultdict(int)
adds_all = churn_adds = 0
churned_files = set()
for line in log.splitlines():
    if not line or line.startswith("@"):
        continue
    a, d, p = line.split("\t", 2)
    if a == "-":
        continue  # binary
    if "=>" in p:  # rename: count to the new path
        p = re.sub(r"\{[^{}]*=> ([^{}]*)\}", r"\1", p) if "{" in p else p.split(" => ")[1]
        p = p.replace("//", "/")
    touch[p] += 1
    adds_all += int(a)
    if touch[p] >= 2:
        churn_adds += int(a)
        churned_files.add(p)
churn = {"pct": round(100 * churn_adds / adds_all, 2) if adds_all else 0.0, "files_churned": len(churned_files),
         "adds_all": adds_all, "churned_adds": churn_adds, "files_touched": len(touch)}
hs = sorted(((f, int(n * maxcog.get(f, 0))) for f, n in touch.items() if f in SRCSET), key=lambda x: (-x[1], x[0]))
dump("churn", {"churn": churn, "per_file_commits": dict(sorted(touch.items()))})
dump("hotspots", {"hotspots": [list(h) for h in hs[:10]], "score": "commits(baseline..HEAD) x max cognitive in file"})
print("sizes", totals, sizes["file_p50"], sizes["file_p90"], sizes["file_max"], sizes["over_800"], "missing", missing)
print("dup", dup["pct"], dup["duplicated_lines"], dup["total_lines"], dup["clones"], split)
print("cx", {k: v for k, v in cx.items() if k != "top"}, len(fns))
print("churn", churn)
print("hotspots", hs[:10])
