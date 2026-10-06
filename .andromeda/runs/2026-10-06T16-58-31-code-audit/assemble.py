"""P3: assemble {run_dir}/record.json from the c-*.json twins (audit-pass.md §Schema) and assert the caps/keys.

Usage: assemble.py {run_dir}
"""
import json, subprocess, sys, datetime

RD = sys.argv[1]
RDN = RD.rstrip("/")
J = lambda n: json.load(open(f"{RD}/c-{n}.json", encoding="utf-8"))
git = lambda *a: subprocess.run(["git", *a], capture_output=True, text=True, check=True).stdout.strip()

HEAD = git("rev-parse", "HEAD")
BASE = "d4113768df08d45f0c6494f91134031b17ac9023"
BOUNDARY = "47bbf38ff98d9f017743d0c5438a0a2d6fc4b1f4"
EPOCH = "Epoch 2 — Element identity"
UNIT = "dioxus-native-dom"

sz, du, cx, gr, de, cv, ch, hs = (J(n) for n in ("sizes", "duplication", "complexity", "graph", "dead", "coverage", "churn", "hotspots"))
mu = J(f"mutation-{UNIT}")

mutation = {"scoped_units": [UNIT], "unit_states": {UNIT: mu["unit_state"]},
            "scores": {UNIT: mu["score"]} if mu["score"] is not None else {},
            "counts": {UNIT: mu["counts"]}, "score_formula": mu["score_formula"],
            "survivors": mu["survivors"], "host": mu["host"], "not_measured": mu["not_measured"],
            "scope_note": "file-scoped to the epoch's touched files of the unit (element_id.rs + dioxus_document.rs, "
                          "57 of the crate's 315 mutants); tests = dioxus-native-dom lib + blitz-tests stand_element_ids, "
                          "stand_id_persistence, stand_accessibility_ids, accessibility_names"}

srcfiles = f"{RDN}/srcfiles.txt"
commands = {
    "sizes": f"git ls-files '*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**' > {srcfiles} && "
             f"tokei --output json $(cat {srcfiles}) > {RDN}/tokei.json (Rust code lines per file; population = srcfiles.txt, "
             f"{sz['population_files']} files)",
    "duplication": f"jscpd --format rust --absolute --reporters json --output {RDN}/jscpd-abs packages apps tests examples wpt/runner "
                   "(paths made repo-relative by summarize.py; default min-tokens 50, .gitignore respected)",
    "complexity": f"mkdir -p {RDN}/rca && rust-code-analysis-cli -m -O json -o {RDN}/rca $(sed 's/^/-p /' {srcfiles}) "
                  "(per function space: cognitive.sum / cyclomatic.sum)",
    "graph": f"python3 scripts/code-graph.py query {RDN} code-audit \"<audit-pass.md canonical cycle / fan-in / fan-out / edge-count SQL>\" "
             f"(exact SQL in {RDN}/graph_q.py and tree-query-code-audit.json)",
    "dead": f"cargo machete (unused deps) + python3 scripts/code-graph.py query {RDN} code-audit \"SELECT s.symbol, s.crate, s.name, s.kind, "
            "s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL\" + "
            f"{RDN}/dead_class.py classing (recipes.dead)",
    "coverage": "bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --lcov --output-path target/coverage/lcov.info "
                "&& cargo llvm-cov report --workspace --locked; TOTAL row, Lines cover column)",
    "churn": f"git log --reverse --numstat --format=@%H {BASE}..HEAD -- '*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' "
             "':!:docs/**' ':!:refs/**' (file-level: adds in a file's 2nd..nth touching commit are churn)",
    "hotspots": f"churn per-file commit counts ({BASE}..HEAD) x max cognitive per file from complexity",
    "mutation": "mkdir -p target/mutants-tmp && TMPDIR=$PWD/target/mutants-tmp cargo mutants --workspace --file packages/{unit}/src/element_id.rs --file packages/{unit}/src/dioxus_document.rs "
                "--test-package={unit},blitz-tests --cargo-test-arg=--lib --cargo-test-arg=--test=stand_element_ids "
                "--cargo-test-arg=--test=stand_id_persistence --cargo-test-arg=--test=stand_accessibility_ids "
                "--cargo-test-arg=--test=accessibility_names --baseline=skip --timeout 120 -j 2 "
                f"--output {RDN}/mutants-{{unit}}",
    "head_overshoot": f"git log --format=%H -1 -S'2026-10-06-accessibility-tree-identity · complete' -- .andromeda/master-route.md · "
                      f"git rev-list --count {BOUNDARY}..HEAD · git diff --numstat {BOUNDARY}..HEAD -- '*.rs' ':!:.andromeda/**' "
                      "':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'",
}
recipe_dead = ("zero-ref rows = canonical audit-pass SQL without LIMIT (symbol LEFT JOIN refs, refs.callee NULL; rows of `symbol`, "
               "not distinct strings). 1) exclude test: '/'||stripped-symbol-path OR '/'||file LIKE '%/tests/%' (stripped = "
               "regexp_replace(symbol,'^rust-analyzer cargo \\S+ \\S+ ','')). 2) trait-impl: stripped path matches "
               "'impl#\\[[^\\]]*\\]\\[[^\\]]*\\]'. 3) residual classed first-match: entry-point (fn main() at crate root | module "
               "crate/) · test-only (path segment test|tests|*_test|*_tests, or #[test]/#[cfg(test)] within the 4 lines above the "
               "def) · derive/attr-invoked (__bitflags_flag_names/ | Props# | StoreImplExt#) · runtime-invoked (>1 residual symbol "
               "on one def line = macro-generated binding) · format-capture ({name} or {name: in the def file) · else candidate. "
               "Candidates are 'candidates', never 'dead'.")

rec = {
    "ts": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
    "epoch": EPOCH, "mode": "trend", "sha": HEAD, "baseline_sha": BASE, "span": 1, "ancestry_broken": False,
    "head_overshoot": {"boundary_sha": BOUNDARY, "commits": 1, "files": [],
                       "note": "1 commit past the boundary (42b80ad9 chore(session): the founder's PROVISIONAL batch) — "
                               "bookkeeping only (.andromeda/, .claude/, escher-0.1.0/*.md); 0 source files in the delta"},
    "tool_versions": {"jscpd": "5.4.0", "tokei": "14.0.0", "rust-code-analysis": "0.0.25", "code-graph.py": "d0425fb1",
                      "cargo-machete": "0.9.2", "cargo-llvm-cov": "0.9.1", "cargo-mutants": "27.1.0", "rustc": "1.99.0"},
    "totals": {**sz["totals"], "units": 28},
    "duplication": du["duplication"],
    "complexity": cx["complexity"],
    "sizes": sz["sizes"],
    "graph": gr["graph"],
    "dead": {k: de["dead"][k] for k in ("unused_deps", "zero_ref_candidates", "top", "fp_classes", "zero_ref_rows", "excluded_test_path")},
    "coverage": cv["coverage"],
    "churn": {"pct": ch["churn"]["pct"], "files_churned": ch["churn"]["files_churned"]},
    "hotspots": hs["hotspots"],
    "mutation": mutation,
    "commands": commands,
    "recipes": {"dead": recipe_dead},
    "corrections": [
        {"target_sha": BASE, "field": "commands.complexity",
         "was": "rust-code-analysis-cli -m -O json -o .andromeda/runs/2026-10-06T05-17-25-code-audit/rca $(sed s/^/-p\\ / …srcfiles.txt) …",
         "now": "mkdir -p {run_dir}/rca && rust-code-analysis-cli -m -O json -o {run_dir}/rca $(sed 's/^/-p /' {run_dir}/srcfiles.txt)",
         "note": "replay aborts as recorded: rust-code-analysis-cli 0.0.25 refuses a non-existent -o ('The output parameter must be a "
                 "directory'); the recorded form omitted the mkdir its run must have had (measured at this run)"},
        {"target_sha": BASE, "field": "recipes.dead", "was": None, "now": recipe_dead,
         "note": "schema-gap fill: the baseline's commands.dead pointed to its run dir's c-dead.json for the recipe; the recipe text is "
                 "now carried in the ledger (read from that file's recipe field only, operator-authorized 2026-10-06)"},
    ],
    "skips": [
        {"metric": f"mutation:{u}", "reason": "declined"} for u in
        ("blitz-dom", "blitz-paint", "blitz-shell", "blitz-vibey-script", "stylo_taffy", "seven_guis", "wpt", "dioxus-native")
    ],
}
if mu["unit_state"].startswith("unviable-dominant"):
    rec["skips"].append({"metric": f"mutation:{UNIT}", "reason": "unviable-dominant"})
if cv["coverage"]["branch"] is None:
    pass  # branch null: llvm-cov reports no branch data without -Zcoverage-options=branch (same as baseline)

# asserts (caps + per-unit survivor identity)
assert len(rec["duplication"]["top"]) <= 10 and len(rec["complexity"]["top"]) <= 10 and len(rec["sizes"]["top"]) <= 10
assert len(rec["hotspots"]) <= 10 and len(rec["graph"]["fan_in_top"]) <= 20 and len(rec["dead"]["top"]) <= 10
assert len(rec["graph"]["cycle_paths"]) == rec["graph"]["cycles"]
c = rec["mutation"]["counts"][UNIT]
assert len({tuple(s) for s in rec["mutation"]["survivors"]}) == len(rec["mutation"]["survivors"]) == c["missed"]
assert len(rec["mutation"]["not_measured"]) == c["not_measured"]
assert all(v is not None for k, v in commands.items())
json.dump(rec, open(f"{RD}/record.json", "w", encoding="utf-8"), ensure_ascii=False)
print("record ok", len(json.dumps(rec, ensure_ascii=False)), "bytes")
