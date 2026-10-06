"""A5 dead-code classing — the baseline record's recipe, implemented verbatim (pinned as this record's recipes.dead).

Usage: dead_class.py {run_dir}
Reads {run_dir}/graph-raw.json (zero_ref rows: canonical SQL without LIMIT) + the source tree at HEAD;
writes {run_dir}/dead-classed.json.
"""
import json, re, sys, collections

RD = sys.argv[1]
rows = json.load(open(f"{RD}/graph-raw.json", encoding="utf-8"))["zero_ref"]
PREFIX = re.compile(r"^rust-analyzer cargo \S+ \S+ ")
TRAIT = re.compile(r"impl#\[[^\]]*\]\[[^\]]*\]")
_src = {}


def lines(f):
    if f not in _src:
        try:
            _src[f] = open(f, encoding="utf-8").read().split("\n")
        except OSError:
            _src[f] = []
    return _src[f]


def text(f):
    return "\n".join(lines(f))


def like_tests(p):  # '/'||p LIKE '%/tests/%'
    return "/tests/" in "/" + (p or "")


def test_segment(stripped):  # a module path segment named test|tests|*_test|*_tests
    segs = stripped.split("/")[:-1]
    return any(s in ("test", "tests") or s.endswith("_test") or s.endswith("_tests") for s in segs)


def test_attr_above(f, line0):  # #[test] / #[cfg(test)] within the 4 lines above the def (def_line is 0-indexed)
    L = lines(f)
    win = "\n".join(L[max(0, line0 - 4):line0])
    return "#[test]" in win or "#[cfg(test)]" in win


excluded, trait, residual = 0, 0, []
for r in rows:
    s = PREFIX.sub("", r["symbol"])
    if like_tests(s) or like_tests(r["file"]):
        excluded += 1
    elif TRAIT.search(s):
        trait += 1
    else:
        residual.append((s, r))

per_line = collections.Counter((r["file"], r["def_line"]) for _, r in residual)
classes = collections.Counter({"trait-impl (dispatch)": trait})
cand = []
for s, r in residual:
    f, l0, name = r["file"], r["def_line"], r["name"]
    if s == "main()." or s == "crate/":
        c = "entry-point"
    elif test_segment(s) or test_attr_above(f, l0):
        c = "test-only"
    elif "__bitflags_flag_names/" in s or "Props#" in s or "StoreImplExt#" in s:
        c = "derive/attr-invoked"
    elif per_line[(f, l0)] > 1:
        c = "runtime-invoked (macro-generated binding)"
    elif re.search(r"\{" + re.escape(name) + r"[}:]", text(f)):
        c = "format-capture"
    else:
        c = "candidate"
        cand.append([r["crate"] + " " + s, f"{f}:{l0 + 1}", r["kind"], r["crate"]])
    classes[c] += 1

cand.sort(key=lambda x: (x[1].split(":")[0], int(x[1].split(":")[1]), x[0]))
out = {"zero_ref_rows": len(rows), "excluded_test_path": excluded, "fp_classes": dict(classes),
       "zero_ref_candidates": len(cand),
       "candidates_by_kind": dict(collections.Counter(c[2] for c in cand)),
       "candidates_by_crate": dict(collections.Counter(c[3] for c in cand).most_common()),
       "candidates": cand}
json.dump(out, open(f"{RD}/dead-classed.json", "w", encoding="utf-8"), ensure_ascii=False)
print(len(rows), excluded, dict(classes), "candidates", len(cand))
