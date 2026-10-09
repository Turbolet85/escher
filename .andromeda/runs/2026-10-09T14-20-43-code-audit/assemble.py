"""Assembles this run's ledger record as record.json from the evidence twins (Phase 3).

usage: assemble.py <ISO-UTC ts>
The mutation block is built from whatever c-mutation-{key}.json twins exist plus DECLINED below.
"""
import json
import os
import subprocess
import sys

RUN = '.andromeda/runs/2026-10-09T14-20-43-code-audit'
BASE = 'b7e3a43b8540343b8e655dfc700640511ef2b707'
PREV = '42b80ad94e1c96014c0325e2a3f4a7135ff1f9ca'
ts = sys.argv[1]
git = lambda *a: subprocess.run(['git', *a], capture_output=True, text=True, check=True).stdout.strip()
head = git('rev-parse', 'HEAD')
c = lambda n: json.load(open(os.path.join(RUN, f'c-{n}.json'), encoding='utf-8'))
PS = "'*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'"

recs = []
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        pass
base = [r for r in recs if r['sha'] == BASE][-1]
assert head not in [r['sha'] for r in recs], 'this HEAD is already recorded: re-audit rules apply'

sizes, dup, cplx, graph, dead = c('sizes'), c('duplication'), c('complexity'), c('graph'), c('dead')
cov, churn, hot = c('coverage'), c('churn'), c('hotspots')

# ---- mutation
KEYS = [('escher-driver', 'escher-driver'), ('escher-telemetry', 'escher-telemetry'),
        ('dioxus-native-dom', 'dioxus-native-dom'), ('blitz-test-harness', 'blitz-test-harness'),
        ('dioxus-native-dom-snapshot', 'dioxus-native-dom/snapshot'),
        ('blitz-dom-scrolling', 'blitz-dom/scrolling'), ('blitz-dom-changed-set', 'blitz-dom/changed-set')]
extra = json.load(open(os.path.join(RUN, 'mutation-decisions.json'), encoding='utf-8'))
mut = {'scoped_units': [], 'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
       'score_formula': 'caught/(caught+missed)', 'survivors': [], 'host': None, 'not_measured': [],
       'timeouts': [], 'riders': {}, 'scope_note': extra['scope_note']}
commands_mut = {}
skips = list(extra['skips'])
for key, name in KEYS:
    p = os.path.join(RUN, f'c-mutation-{key}.json')
    ex = os.path.join(RUN, f'mutation-exhausted-{key}.json')
    if os.path.isfile(p):
        m = json.load(open(p, encoding='utf-8'))
        mut['scoped_units'].append(name)
        mut['unit_states'][name] = m['unit_state']
        mut['counts'][name] = m['counts']
        if m['score'] is not None:
            mut['scores'][name] = m['score']
        else:
            skips.append({'metric': f'mutation:{name}', 'reason': 'unviable-dominant'})
        if m['unit_state'].startswith('complete'):
            mut['timing'][name] = m['timing']
        assert len(m['survivors']) == m['counts']['missed'], name
        assert len({tuple(s) for s in m['survivors']}) == len(m['survivors']), name
        assert len(m['not_measured']) == m['counts']['not_measured'], name
        mut['survivors'] += m['survivors']
        mut['not_measured'] += m['not_measured']
        mut['timeouts'] += m['timeouts']
        mut['host'] = m['host']
        if m.get('riders'):  # a filtered unit's field-deletion mutants, tallied apart from its counts and score
            r = m['riders']
            assert len(r['survivors']) == r['counts']['missed'] and len(r['sites']) == r['counts']['mutants'], name
            mut['riders'][name] = {k: r[k] for k in ('counts', 'sites', 'survivors', 'not_measured')}
        commands_mut[f'mutation:{name}'] = m['hand_pass']['command'] + ' (the unmutated pass, by hand, exit ' + \
            str(m['hand_pass']['exit']) + ') · ' + m['command']
        if m.get('pin'):  # a pin applied to the running tree from outside: not in the firing form itself
            commands_mut[f'mutation:{name}'] += (f" · pinned while running, from {m['pin']['at']}: taskset -c "
                                                 f"{m['pin']['cpus']} on the tool's process tree, by "
                                                 f"{m['pin']['by'].split(',')[0]} (at {m['pin']['marker']['live_tested']}"
                                                 f" of {m['timing']['planned']} tested)")
    elif os.path.isfile(ex):
        m = json.load(open(ex, encoding='utf-8'))
        mut['scoped_units'].append(name)
        mut['unit_states'][name] = 'budget-exhausted'
        mut['timing'][name] = m['timing']
        skips.append({'metric': f'mutation:{name}', 'reason': 'budget-exhausted'})
        commands_mut[f'mutation:{name}'] = m['command']
assert len({tuple(s) for s in mut['survivors']}) == len(mut['survivors'])
if mut['host'] is None:
    mut['host'] = 'x86_64-unknown-linux-gnu'

record = {
    'ts': ts, 'epoch': 'Epoch 4 — Driver core', 'mode': 'trend', 'sha': head, 'baseline_sha': BASE, 'span': 1,
    'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': head, 'commits': 0, 'files': [],
                       'note': 'at the boundary: HEAD is the 2026-10-07-driver-command-spans complete flip'},
    'tool_versions': {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25',
                      'code-graph.py': 'd0425fb1', 'cargo-machete': '0.9.2', 'cargo-llvm-cov': '0.9.1',
                      'cargo-mutants': '27.1.0', 'rustc': '1.99.0'},
    'totals': {'loc': sizes['totals']['loc'], 'files': sizes['totals']['files'], 'units': graph['units_in_graph']},
    'duplication': {k: dup[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split')},
    'complexity': {k: cplx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90',
                                         'over_ceiling', 'max', 'top')},
    'sizes': {k: sizes[k] for k in ('file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
    'graph': {k: graph[k] for k in ('cycles', 'cycle_paths', 'fan_in_top', 'fan_out', 'cross_unit_edges')},
    'dead': {k: dead[k] for k in ('unused_deps', 'zero_ref_candidates', 'top', 'fp_classes', 'zero_ref_rows',
                                   'excluded_test_path')},
    'coverage': {'line': cov['line'], 'branch': cov['branch']},
    'churn': {'pct': churn['pct'], 'files_churned': churn['files_churned']},
    'hotspots': hot['top'],
    'mutation': mut,
    'commands': {
        'sizes': f"git ls-files {PS} > {RUN}/srcfiles.txt && tokei --output json $(cat {RUN}/srcfiles.txt) > "
                 f"{RUN}/tokei.json (Rust code lines per file; population = srcfiles.txt, {sizes['totals']['files']} "
                 "files; totals.units = cargo metadata --no-deps --format-version 1 --locked workspace packages, "
                 "equal to the graph's distinct crates)",
        'duplication': f"jscpd --format rust --absolute --reporters json --output {RUN}/jscpd-abs packages apps tests "
                       "examples wpt/runner (paths made repo-relative by summarize.py; default min-tokens 50, "
                       ".gitignore respected; split: a file under a `tests` directory segment is test, a pair of two "
                       "is test, of none is src, else mixed)",
        'complexity': f"mkdir -p {RUN}/rca && rust-code-analysis-cli -m -O json -o {RUN}/rca $(sed 's/^/-p /' "
                      f"{RUN}/srcfiles.txt) (every space of kind function, closures included: cognitive.sum / "
                      "cyclomatic.sum)",
        'graph': f"python3 scripts/code-graph.py query {RUN} code-audit \"<audit-pass.md canonical cycle / fan-in / "
                 "fan-out / edge-count SQL, verbatim>\" (fired one at a time by graph_queries.py; the SQL as fired is "
                 f"in {RUN}/tree-query-code-audit.json)",
        'dead': f"cargo machete (unused deps) + python3 scripts/code-graph.py query {RUN} code-audit \"SELECT "
                "s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = "
                f"s.symbol WHERE r.callee IS NULL\" + python3 {RUN}/summarize.py (recipes.dead)",
        'coverage': "bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --lcov --output-path "
                    "target/coverage/lcov.info && cargo llvm-cov report --workspace --locked; TOTAL row, Lines cover "
                    "column)",
        'churn': f"git log --reverse --numstat --format=@%H {BASE}..HEAD -- {PS} (file-level: adds in a file's "
                 "2nd..nth touching commit are churn)",
        'hotspots': f"churn per-file commit counts ({BASE}..HEAD) x max cognitive per file from complexity (a "
                    "measured 0 scores 0)",
        'mutation': "per scoped unit: its `mutation:{unit}` entry (the unmutated pass by hand, then cargo mutants "
                    "with --baseline=skip)",
        **commands_mut,
        'head_overshoot': "git log --format=%H -1 -S'2026-10-07-driver-command-spans · complete' -- "
                          f".andromeda/master-route.md · git rev-list --count {head}..HEAD · git diff --numstat "
                          f"{head}..HEAD -- {PS}",
    },
    'recipes': {'dead': base['recipes']['dead']},
    'corrections': extra['corrections'],
    'skips': skips,
}
with open(os.path.join(RUN, 'record.json'), 'w', encoding='utf-8', newline='') as f:
    json.dump(record, f, ensure_ascii=False, indent=1)
    f.write('\n')
line = json.dumps(record, ensure_ascii=False)
print('record.json written |', len(line.encode('utf-8')), 'bytes as one line | scoped', mut['scoped_units'])
print('scores', mut['scores'], '| survivors', len(mut['survivors']), '| not_measured', len(mut['not_measured']))
print('skips', skips)
print('unresolved placeholders:', [k for k, v in record['commands'].items()
                                   if v and '{' in v.replace('{unit}', '')])
