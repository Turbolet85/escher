"""P1 coverage summarizer + P3 snapshot assembly (ad-hoc: snapshot.json, no ledger append).

usage: assemble.py {run_dir} {coverage.raw.log}
"""
import json
import os
import re
import subprocess
import sys
from datetime import datetime, timezone

rd, covlog = sys.argv[1:3]
RD = '.andromeda/runs/' + os.path.basename(os.path.normpath(rd))
BASE = '143b6013229d785f4613895e16c1c82b1c1a5be2'
SRC = "'*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'"


def j(name):
    return json.load(open(os.path.join(rd, name), encoding='utf-8'))


# coverage: the llvm-cov report's TOTAL row, its Lines cover column (the 3rd `Cover`)
cov = {'line': None, 'branch': None}
txt = open(covlog, encoding='utf-8', errors='replace').read()
ok = re.search(r'^exit=0$', txt, re.M) is not None
hdr = [l for l in txt.splitlines() if l.startswith('Filename') and 'Cover' in l]
tot = [l for l in txt.splitlines() if l.startswith('TOTAL')]
if ok and hdr and tot:
    # header words split ("Missed Regions"), so read the TOTAL row by its columns:
    # TOTAL regions missed cover% functions missed executed% lines missed cover% branches missed cover
    t = tot[-1].split()
    cov['line'] = float(t[9].rstrip('%'))
    if len(t) > 12 and t[12] not in ('-', ''):
        cov['branch'] = float(t[12].rstrip('%'))
json.dump({**cov, 'leg_exit_0': ok, 'total_row': tot[-1] if tot else None},
          open(os.path.join(rd, 'c-coverage.json'), 'w', encoding='utf-8'), indent=1)

sz, cx, dup, gr, dead, ch, hot = (j('c-sizes.json'), j('c-complexity.json'), j('c-duplication.json'),
                                   j('c-graph.json'), j('c-dead.json'), j('c-churn.json'), j('c-hotspots.json'))
meta = json.loads(subprocess.run(['cargo', 'metadata', '--no-deps', '--format-version', '1', '--locked'],
                                 capture_output=True, text=True, check=True).stdout)
head = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True, check=True).stdout.strip()
recipes_dead = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8')
                if l.strip()][-1]['recipes']['dead']
TOUCHED = ['blitz-dom', 'blitz-examples', 'blitz-shell', 'blitz-test-harness', 'blitz-tests',
           'blitz-vibey-script', 'dioxus-native-dom', 'escher-driver']
rec = {
    'ts': datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
    'epoch': 'Epoch 2 — Engine debts', 'mode': 'ad-hoc', 'sha': head, 'baseline_sha': BASE, 'span': 1,
    'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': None, 'commits': None, 'files': [],
                       'note': 'epoch in progress: 2026-10-18-engine-debts reads gated in the master record, so no '
                               'complete flip exists (the pickaxe returns empty) and no boundary is defined; '
                               'ad-hoc, not appended'},
    'tool_versions': {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25',
                      'code-graph.py': 'f22becbb', 'cargo-machete': '0.9.2', 'cargo-llvm-cov': '0.9.1',
                      'cargo-mutants': '27.1.0', 'cargo-nextest': '0.9.146', 'rustc': '1.99.0'},
    'totals': {**sz['totals'], 'units': len(meta['packages'])},
    'duplication': {k: dup[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split')},
    'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90',
                                      'over_ceiling', 'max', 'top')},
    'sizes': sz['sizes'],
    'graph': gr,
    'dead': {k: dead[k] for k in ('unused_deps', 'zero_ref_candidates', 'top', 'fp_classes', 'zero_ref_rows',
                                  'excluded_test_path')},
    'coverage': cov,
    'churn': {'pct': ch['pct'], 'files_churned': ch['files_churned']},
    'hotspots': hot,
    'mutation': {'scoped_units': [], 'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
                 'score_formula': 'caught/(caught+missed)', 'survivors': [],
                 'host': 'x86_64-unknown-linux-gnu', 'not_measured': []},
    'commands': {
        'sizes': f"git ls-files {SRC} > {RD}/srcfiles.txt && tokei --output json $(cat {RD}/srcfiles.txt) > "
                 f"{RD}/tokei.json (Rust code lines per file; population = srcfiles.txt, {sz['totals']['files']} "
                 f"files; totals.units = cargo metadata --no-deps --format-version 1 --locked workspace packages; "
                 f"summarized by python3 {RD}/sum_sizes_cx.py {RD}/tokei.json {RD}/rca {RD}/srcfiles.txt {RD}/c-; "
                 f"the summarizer reproduced the baseline record's sizes, totals (loc, files) and complexity "
                 f"exactly on a git archive of {BASE}, 0 mismatches)",
        'duplication': f"jscpd --format rust --absolute --reporters json --output {RD}/jscpd-abs packages apps "
                       f"tests examples wpt/runner (paths made repo-relative by python3 {RD}/sum_dup.py "
                       f"{RD}/jscpd-abs/jscpd-report.json . {RD}/c-duplication.json; default min-tokens 50, "
                       f".gitignore respected; split: a file under a `tests` directory segment is test, a pair of "
                       f"two is test, of none is src, else mixed; reproduced the baseline record exactly on a git "
                       f"archive of {BASE})",
        'complexity': f"mkdir -p {RD}/rca && rust-code-analysis-cli -m -O json -o {RD}/rca $(sed 's/^/-p /' "
                      f"{RD}/srcfiles.txt) (every space of kind function, closures included: cognitive.sum / "
                      f"cyclomatic.sum; summarized with sizes)",
        'graph': f"python3 {RD}/graph_queries.py {RD} (fires scripts/code-graph.py query {RD} code-audit "
                 f"\"<audit-pass.md canonical cycle / fan-in / fan-out / edge-count SQL, verbatim>\" one at a time; "
                 f"the SQL as fired is in {RD}/tree-query-code-audit.json)",
        'dead': f"cargo machete (unused deps) + python3 scripts/code-graph.py query {RD} code-audit \"SELECT "
                f"s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee "
                f"= s.symbol WHERE r.callee IS NULL\" (fired by graph_queries.py) + python3 {RD}/sum_dead.py {RD} "
                f"{RD}/machete.out (recipes.dead; def_line is 0-based in the DB, printed +1)",
        'coverage': 'bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --lcov '
                    '--output-path target/coverage/lcov.info && cargo llvm-cov report --workspace --locked; TOTAL '
                    f'row, Lines cover column; read by python3 {RD}/assemble.py {RD} {RD}/coverage.raw.log)',
        'churn': f"git log --reverse --numstat --format=@%H {BASE}..HEAD -- {SRC} > {RD}/churn.numstat "
                 f"(file-level: adds in a file's 2nd..nth touching commit are churn; a rename counts to its new "
                 f"path; python3 {RD}/sum_git.py {RD} {RD}/churn.numstat)",
        'hotspots': f"churn per-file commit counts ({BASE}..HEAD) x max cognitive per file from complexity "
                    f"(a measured 0 scores 0)",
        'mutation': None,
        'head_overshoot': "git log --format=%H -1 -S'2026-10-18-engine-debts · complete' -- "
                          ".andromeda/master-route.md (empty: the epoch's last chunk reads gated)",
    },
    'recipes': {'dead': recipes_dead},
    'corrections': [],
    'skips': [{'metric': f'mutation:{u}', 'reason': 'declined'} for u in TOUCHED],
}
json.dump(rec, open(os.path.join(rd, 'snapshot.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('coverage', cov, 'leg ok', ok)
print('snapshot written', len(json.dumps(rec, ensure_ascii=False)), 'bytes')
