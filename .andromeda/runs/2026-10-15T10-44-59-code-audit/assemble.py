"""Phase 3: assemble record.json from the c-*.json twins and the mutation artifacts.

usage: python3 assemble.py {run_dir_rel}   (cwd = repo root)
"""
import json
import os
import sys
from datetime import datetime, timezone

RD = sys.argv[1]
SHA = 'af1a00ad5b346697ef8b20004e4f1f769ce772f2'
BASE = '495067f8ab5155eda6369530c16bee7043cee600'
EPOCH = 'Epoch 6 — Polish & ship'
SRC = "'*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'"
UNITS = ['escher-telemetry', 'blitz-test-harness', 'seven_guis', 'dioxus-native-dom', 'escher-driver',
         'blitz-dom/range-keyboard']
DECLINED = ['accesskit_xplat', 'blitz', 'blitz-examples', 'blitz-html', 'blitz-net', 'blitz-paint', 'blitz-shell',
            'blitz-tests', 'blitz-traits', 'blitz-vibey-script', 'browser', 'browser-persistence', 'bump', 'counter',
            'debug_timer', 'dioxus-native', 'rdme', 'stylo_taffy', 'todomvc', 'transparent', 'wasm_hello',
            'wgpu_texture', 'wpt']


def j(name):
    return json.load(open(os.path.join(RD, name), encoding='utf-8'))


def slug(k):
    return k.replace('/', '-')


recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8')]
base = [r for r in recs if r['sha'] == BASE][-1]
sizes, cx, dup, dead = j('c-sizes.json'), j('c-complexity.json'), j('c-duplication.json'), j('c-dead.json')
graph, cov, churn, hs = j('c-graph.json'), j('c-coverage.json'), j('c-churn.json'), j('c-hotspots.json')
meta = j('metadata.json')

mutation = {'scoped_units': list(UNITS), 'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
            'score_formula': 'caught/(caught+missed)', 'survivors': [], 'host': 'x86_64-unknown-linux-gnu',
            'not_measured': []}
skips, commands_mut = [], {}
for u in UNITS:
    p = os.path.join(RD, 'c-mutation-%s.json' % slug(u))
    sp = os.path.join(RD, 'state-%s.json' % slug(u))
    if os.path.isfile(p):
        d = json.load(open(p, encoding='utf-8'))
        mutation['unit_states'][u] = d['unit_state']
        mutation['counts'][u] = d['counts']
        mutation['timing'][u] = d['timing']
        mutation['survivors'] += d['survivors']
        mutation['not_measured'] += d['not_measured']
        if d['score'] is not None:
            mutation['scores'][u] = d['score']
        else:
            skips.append({'metric': 'mutation:%s' % u, 'reason': 'unviable-dominant'})
        commands_mut['mutation:%s' % u] = '%s (the unmutated pass, by hand, exit %d) · %s' % (
            d['hand']['command'], d['hand']['exit'], d['command'])
    else:
        st = json.load(open(sp, encoding='utf-8')) if os.path.isfile(sp) else {'state': 'not-run'}
        mutation['unit_states'][u] = st['state']
        if st.get('timing'):
            mutation['timing'][u] = st['timing']
        reason = st['state'] if st['state'] in ('budget-exhausted', 'baseline-test-failure') else st['state']
        skips.append({'metric': 'mutation:%s' % u, 'reason': reason})
        commands_mut['mutation:%s' % u] = st.get('command')
for u in DECLINED:
    skips.append({'metric': 'mutation:%s' % u, 'reason': 'declined'})
skips.append({'metric': 'mutation:blitz-dom (the whole unit beyond range.rs + keyboard.rs: 5477 listed; its 15 other '
                        'touched files 2657)', 'reason': 'declined'})

rd_rel = RD.rstrip('/')
commands = {
    'sizes': "git ls-files %s > %s/srcfiles.txt && tokei --output json $(cat %s/srcfiles.txt) > %s/tokei.json "
             "(Rust code lines per file; population = srcfiles.txt, %d files; totals.units = cargo metadata --no-deps "
             "--format-version 1 --locked workspace packages; summarized by python3 %s/sum_sizes_cx.py %s/tokei.json "
             "%s/rca %s/srcfiles.txt %s/c-; the summarizer reproduced the baseline record's sizes, totals and "
             "complexity exactly on a git archive of %s)" % (SRC, rd_rel, rd_rel, rd_rel, sizes['totals']['files'],
                                                             rd_rel, rd_rel, rd_rel, rd_rel, rd_rel, BASE),
    'duplication': 'jscpd --format rust --absolute --reporters json --output %s/jscpd-abs packages apps tests examples '
                   'wpt/runner (paths made repo-relative by python3 %s/sum_dup.py; default min-tokens 50, .gitignore '
                   'respected; split: a file under a `tests` directory segment is test, a pair of two is test, of none '
                   'is src, else mixed; reproduced the baseline record exactly on a git archive of %s)'
                   % (rd_rel, rd_rel, BASE),
    'complexity': 'mkdir -p %s/rca && rust-code-analysis-cli -m -O json -o %s/rca $(sed \'s/^/-p /\' %s/srcfiles.txt) '
                  '(every space of kind function, closures included: cognitive.sum / cyclomatic.sum)'
                  % (rd_rel, rd_rel, rd_rel),
    'graph': 'python3 scripts/code-graph.py query %s code-audit "<audit-pass.md canonical cycle / fan-in / fan-out / '
             'edge-count SQL, verbatim>" (fired one at a time; the SQL as fired is in %s/tree-query-code-audit.json)'
             % (rd_rel, rd_rel),
    'dead': 'cargo machete (unused deps) + python3 scripts/code-graph.py query %s code-audit "SELECT s.symbol, s.crate, '
            's.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee '
            'IS NULL" + python3 %s/sum_dead.py (recipes.dead)' % (rd_rel, rd_rel),
    'coverage': 'bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --lcov --output-path '
                'target/coverage/lcov.info && cargo llvm-cov report --workspace --locked; TOTAL row, Lines cover column)',
    'churn': 'git log --reverse --numstat --format=@%%H %s..HEAD -- %s (file-level: adds in a file\'s 2nd..nth '
             'touching commit are churn; a rename counts to its new path)' % (BASE, SRC),
    'hotspots': 'churn per-file commit counts (%s..HEAD) x max cognitive per file from complexity (a measured 0 '
                'scores 0)' % BASE,
    'mutation': 'per scoped unit: its `mutation:{unit}` entry (the unmutated pass, by hand, then cargo mutants with '
                '--baseline=skip, driven sequentially by python3 %s/mutate.py; test set per unit from python3 '
                '%s/targets.py %s/metadata.json {unit})' % (rd_rel, rd_rel, rd_rel),
    'head_overshoot': "git log --format=%%H -1 -S'2026-10-15-cold-agent-test · complete' -- .andromeda/master-route.md"
                      " · git rev-list --count %s..HEAD · git diff --numstat %s..HEAD -- %s" % (SHA, SHA, SRC),
}
commands.update(commands_mut)

rec = {
    'ts': datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'), 'epoch': EPOCH, 'mode': 'trend',
    'sha': SHA, 'baseline_sha': BASE, 'span': 1, 'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': SHA, 'commits': 0, 'files': [],
                       'note': 'at the boundary: HEAD is the 2026-10-15-cold-agent-test complete flip'},
    'tool_versions': {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'code-graph.py': 'f22becbb',
                      'cargo-machete': '0.9.2', 'cargo-llvm-cov': '0.9.1', 'cargo-mutants': '27.1.0',
                      'rustc': '1.99.0'},
    'totals': {'loc': sizes['totals']['loc'], 'files': sizes['totals']['files'], 'units': len(meta['packages'])},
    'duplication': {k: dup[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split')},
    'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90',
                                      'over_ceiling', 'max', 'top')},
    'sizes': {k: sizes[k] for k in ('file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
    'graph': graph,
    'dead': {'unused_deps': dead['unused_deps'], 'zero_ref_candidates': dead['zero_ref_candidates'],
             'top': dead['top'][:10],
             'fp_classes': {k: v for k, v in dead['fp_classes'].items() if k != 'excluded_test_path'},
             'zero_ref_rows': dead['zero_ref_rows'], 'excluded_test_path': dead['excluded_test_path']},
    'coverage': {'line': cov['line'], 'branch': cov['branch']},
    'churn': {'pct': churn['pct'], 'files_churned': churn['files_churned']},
    'hotspots': hs,
    'mutation': mutation,
    'commands': commands,
    'recipes': {'dead': base['recipes']['dead']},
    'corrections': [],
    'skips': skips,
}
# per-unit survivor asserts (audit-pass.md §Schema's Caps)
for u in UNITS:
    p = os.path.join(RD, 'c-mutation-%s.json' % slug(u))
    if os.path.isfile(p):
        d = json.load(open(p, encoding='utf-8'))
        assert len({tuple(r) for r in d['survivors']}) == len(d['survivors']) == d['counts']['missed'], u
        assert len(d['not_measured']) == d['counts']['not_measured'], u
assert len({tuple(r) for r in mutation['survivors']}) == len(mutation['survivors'])
assert len(rec['complexity']['top']) <= 10 and len(rec['sizes']['top']) <= 10 and len(rec['hotspots']) <= 10
assert len(rec['graph']['fan_in_top']) <= 20 and len(rec['graph']['cycle_paths']) == rec['graph']['cycles']
json.dump(rec, open(os.path.join(RD, 'record.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('record.json written:', len(json.dumps(rec, ensure_ascii=False)), 'bytes;', 'skips', len(skips),
      '; scores', mutation['scores'])
