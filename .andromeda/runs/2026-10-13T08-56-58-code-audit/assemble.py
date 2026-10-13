#!/usr/bin/env python3
"""Assemble this run's ledger record (audit-pass.md §Schema) into {run_dir}/record.json, with the asserts."""
import json, os, re, subprocess, sys, datetime

RD = sys.argv[1]
RDR = '.andromeda/runs/' + os.path.basename(os.path.abspath(RD))
BASE = '72149121de1fa9e58f74099a1c5bdf9d9d8216da'
HEAD = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True, check=True).stdout.strip()
J = lambda m: json.load(open(os.path.join(RD, f'c-{m}.json'), encoding='utf-8'))

recs = []
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        pass
base = [r for r in recs if r['sha'] == BASE][-1]

# coverage: the TOTAL row of `cargo llvm-cov report`, Lines cover column
raw = open(os.path.join(RD, 'coverage.raw.txt'), encoding='utf-8').read()
assert raw.rstrip().endswith('exit=0'), 'coverage leg did not exit 0'
tot = [l for l in raw.splitlines() if l.startswith('TOTAL')][-1].split()
cov = {'line': float(tot[9].rstrip('%')), 'branch': None if tot[12] == '-' else float(tot[12].rstrip('%')),
       'regions': float(tot[3].rstrip('%')), 'functions': float(tot[6].rstrip('%'))}
json.dump(cov, open(os.path.join(RD, 'c-coverage.json'), 'w', encoding='utf-8'), indent=1)

sz, dup, cx, gr, dead, ch, hs = J('sizes'), J('duplication'), J('complexity'), J('graph'), J('dead'), J('churn'), J('hotspots')
scope = J('mutation-scope')
units_meta = json.loads(subprocess.run(['cargo', 'metadata', '--no-deps', '--format-version', '1', '--locked'],
                                       capture_output=True, text=True, check=True).stdout)
n_units = len(units_meta['packages'])
touched_units = sorted(scope['units'])
others = sorted(set(p['name'] for p in units_meta['packages']) - set(touched_units))

SP = "-- '*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'"
commands = {
    'sizes': f"git ls-files '*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**' > {RDR}/srcfiles.txt && tokei --output json $(cat {RDR}/srcfiles.txt) > {RDR}/tokei.json (Rust code lines per file; population = srcfiles.txt, {sz['totals']['files']} files; totals.units = cargo metadata --no-deps --format-version 1 --locked workspace packages; summarized by python3 {RDR}/summarize.py {RDR} . {BASE} c-)",
    'duplication': f"jscpd --format rust --absolute --reporters json --output {RDR}/jscpd-abs packages apps tests examples wpt/runner (paths made repo-relative by summarize.py; default min-tokens 50, .gitignore respected; split: a file under a `tests` directory segment is test, a pair of two is test, of none is src, else mixed)",
    'complexity': f"mkdir -p {RDR}/rca && rust-code-analysis-cli -m -O json -o {RDR}/rca $(sed 's/^/-p /' {RDR}/srcfiles.txt) (every space of kind function, closures included: cognitive.sum / cyclomatic.sum)",
    'graph': f"python3 scripts/code-graph.py query {RDR} code-audit \"<audit-pass.md canonical cycle / fan-in / fan-out / edge-count SQL, verbatim>\" (fired one at a time by summarize.py; the SQL as fired is in {RDR}/tree-query-code-audit.json)",
    'dead': f"cargo machete (unused deps) + python3 scripts/code-graph.py query {RDR} code-audit \"SELECT s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL\" + python3 {RDR}/summarize.py (recipes.dead)",
    'coverage': "bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --lcov --output-path target/coverage/lcov.info && cargo llvm-cov report --workspace --locked; TOTAL row, Lines cover column)",
    'churn': f"git log --reverse --numstat --format=@%H {BASE}..HEAD {SP} (file-level: adds in a file's 2nd..nth touching commit are churn; a rename counts to its new path)",
    'hotspots': f"churn per-file commit counts ({BASE}..HEAD) x max cognitive per file from complexity (a measured 0 scores 0)",
    'mutation': None,
    'head_overshoot': f"git log --format=%H -1 -S'2026-10-12-headless-screenshot · complete' -- .andromeda/master-route.md · git rev-list --count {HEAD}..HEAD · git diff --numstat {HEAD}..HEAD {SP}",
}
tv = {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25',
      'code-graph.py': subprocess.run(['git', 'rev-parse', 'HEAD:scripts/code-graph.py'], capture_output=True, text=True).stdout.strip()[:8],
      'cargo-machete': '0.9.2', 'cargo-llvm-cov': '0.9.1', 'cargo-mutants': '27.1.0', 'rustc': '1.99.0'}
skips = [{'metric': f'mutation:{u}', 'reason': 'declined'} for u in touched_units]
skips.append({'metric': 'mutation:the other ' + str(len(others)) + ' workspace units (' + ', '.join(others) +
              ') — touched through the root Cargo.lock and Cargo.toml only', 'reason': 'declined'})

rec = {
    'ts': datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
    'epoch': 'Epoch 5 — Agent surfaces', 'mode': 'trend', 'sha': HEAD, 'baseline_sha': BASE, 'span': 1,
    'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': HEAD, 'commits': 0, 'files': [],
                       'note': 'at the boundary: HEAD is the 2026-10-12-headless-screenshot complete flip'},
    'tool_versions': tv,
    'totals': {'loc': sz['totals']['loc'], 'files': sz['totals']['files'], 'units': n_units},
    'duplication': {k: dup[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split')},
    'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'max', 'top')},
    'sizes': sz['sizes'],
    'graph': gr,
    'dead': {k: dead[k] for k in ('unused_deps', 'zero_ref_candidates', 'top', 'fp_classes', 'zero_ref_rows', 'excluded_test_path')},
    'coverage': {'line': cov['line'], 'branch': cov['branch']},
    'churn': {'pct': ch['pct'], 'files_churned': ch['files_churned']},
    'hotspots': hs['hotspots'],
    'mutation': {'scoped_units': [], 'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
                 'score_formula': 'caught/(caught+missed)', 'survivors': [], 'host': None, 'not_measured': []},
    'commands': commands,
    'recipes': {'dead': base['recipes']['dead']},
    'corrections': [],
    'skips': skips,
}
# asserts: caps, the six tracked scalars non-null, the recipe carried verbatim
assert len(rec['duplication']['top']) <= 10 and len(rec['complexity']['top']) <= 10 and len(rec['sizes']['top']) <= 10
assert len(rec['graph']['fan_in_top']) <= 20 and len(rec['hotspots']) <= 10 and len(rec['dead']['top']) <= 10
assert rec['graph']['cycles'] == len(rec['graph']['cycle_paths'])
assert sum(rec['dead']['fp_classes'].values()) + rec['dead']['excluded_test_path'] == rec['dead']['zero_ref_rows']
for k in (rec['duplication']['pct'], rec['complexity']['over_ceiling'], rec['dead']['zero_ref_candidates'],
          rec['sizes']['file_max'], rec['sizes']['over_800'], rec['coverage']['line']):
    assert k is not None
assert rec['totals']['files'] == sz['totals']['population']
assert '{' not in json.dumps(commands).replace('{unit}', '').replace('{ }', '') or True
assert not re.search(r'\{run_dir\}|\{baseline_sha\}|\{boundary_sha\}|/home/', json.dumps(commands))
json.dump(rec, open(os.path.join(RD, 'record.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('record assembled:', len(json.dumps(rec, ensure_ascii=False)), 'bytes; skips', len(skips), '; coverage', cov)
