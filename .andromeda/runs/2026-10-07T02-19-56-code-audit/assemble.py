"""P3 — assemble record.json from the evidence twins in this run dir (audit-pass.md §Schema).

Usage: assemble.py <run_dir> <head sha> <mutation state: complete|budget-exhausted> <ISO-UTC ts>
"""
import json
import os
import sys

rd, head, mut_state, ts = sys.argv[1:5]
RD = rd.rstrip('/')
L = lambda n: json.load(open(os.path.join(rd, n), encoding='utf-8'))
norm = lambda s: __import__('re').sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')

recs, bad = [], 0
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        bad += 1
base = recs[-1]  # the baseline as read at Setup: the ledger's last record BEFORE this run's append
assert base['sha'] == '42b80ad94e1c96014c0325e2a3f4a7135ff1f9ca' and bad == 0
assert all(norm(r['epoch']) != norm('Epoch 3 — Observation model') for r in recs), 'epoch already recorded'

BASE = base['sha']
PATHSPEC = "'*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'"
UNIT = 'dioxus-native-dom'
TESTS = ['stand_element_ids', 'stand_id_persistence', 'stand_accessibility_ids', 'accessibility_names',
         'stand_id_edits', 'stand_actionable_keys', 'stand_snapshot']
MARKER = '2026-10-07-change-tracking-and-diff'

sizes, dup, cx, graph, dead, cov = (L('c-sizes.json'), L('c-duplication.json'), L('c-complexity.json'),
                                    L('c-graph.json'), L('c-dead.json'), L('c-coverage.json'))
churn, hot = L('c-churn.json'), L('c-hotspots.json')

skips = [
    {'metric': 'mutation:dioxus-native-dom (actionable.rs, lib.rs, mutation_writer.rs, snapshot.rs, snapshot_diff.rs, '
               'snapshot_text.rs — the unit\'s other touched files, 142 of its 199 touched-file mutants)', 'reason': 'declined'},
    {'metric': 'mutation:blitz-dom', 'reason': 'declined'},
    {'metric': 'mutation:blitz-shell', 'reason': 'declined'},
    {'metric': 'mutation:seven_guis', 'reason': 'declined'},
]
mutation = {
    'scoped_units': [UNIT], 'unit_states': {}, 'scores': {}, 'counts': {},
    'score_formula': 'caught/(caught+missed)', 'survivors': [], 'host': 'x86_64-unknown-linux-gnu', 'not_measured': [],
    'scope_note': 'file-scoped on the operator\'s word at this run\'s confirm to element_id.rs + dioxus_document.rs — the '
                  'two files the Epoch 2 score covers (57 of the unit\'s 199 touched-file mutants); -j 4 under a '
                  '15-minute cap, no re-run; tests = dioxus-native-dom lib + blitz-tests ' + ', '.join(TESTS) +
                  ' (the Epoch 2 set plus stand_id_edits, stand_actionable_keys and stand_snapshot, which this epoch '
                  'added); the unmutated pass of that set was run by hand first, exit 0 (mutation-hand-baseline.log)',
}
if mut_state == 'complete':
    m = L('c-mutation-%s.json' % UNIT)
    mutation['unit_states'][UNIT] = m['state']
    mutation['counts'][UNIT] = m['counts']
    mutation['survivors'] = m['survivors']
    mutation['not_measured'] = m['not_measured']
    mutation['host'] = m['host']
    if m['score'] is not None:
        mutation['scores'][UNIT] = m['score']
    else:
        skips.insert(0, {'metric': 'mutation:' + UNIT, 'reason': 'unviable-dominant'})
    c = m['counts']
    assert len(m['survivors']) == c['missed'] == len({tuple(s) for s in m['survivors']})
    assert len(m['not_measured']) == c['not_measured']
else:
    mutation['unit_states'][UNIT] = 'budget-exhausted'
    skips.insert(0, {'metric': 'mutation:' + UNIT, 'reason': 'budget-exhausted'})

record = {
    'ts': ts, 'epoch': 'Epoch 3 — Observation model', 'mode': 'trend', 'sha': head, 'baseline_sha': BASE, 'span': 1,
    'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': head, 'commits': 0, 'files': [],
                       'note': 'at the boundary: HEAD is the %s complete flip' % MARKER},
    'tool_versions': {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'code-graph.py': 'd0425fb1',
                      'cargo-machete': '0.9.2', 'cargo-llvm-cov': '0.9.1', 'cargo-mutants': '27.1.0', 'rustc': '1.99.0'},
    'totals': {'loc': sizes['totals']['loc'], 'files': sizes['totals']['files'], 'units': 28},
    'duplication': {k: dup[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split')},
    'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90',
                                      'over_ceiling', 'max', 'top')},
    'sizes': {k: sizes[k] for k in ('file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
    'graph': {k: graph[k] for k in ('cycles', 'cycle_paths', 'fan_in_top', 'fan_out', 'cross_unit_edges')},
    'dead': {k: dead[k] for k in ('unused_deps', 'zero_ref_candidates', 'top', 'fp_classes', 'zero_ref_rows',
                                  'excluded_test_path')},
    'coverage': {'line': cov['line'], 'branch': cov['branch']},
    'churn': {'pct': churn['pct'], 'files_churned': churn['files_churned']},
    'hotspots': hot['top'],
    'mutation': mutation,
    'commands': {
        'sizes': "git ls-files %s > %s/srcfiles.txt && tokei --output json $(cat %s/srcfiles.txt) > %s/tokei.json "
                 "(Rust code lines per file; population = srcfiles.txt, %d files)" % (PATHSPEC, RD, RD, RD, sizes['totals']['files']),
        'duplication': "jscpd --format rust --absolute --reporters json --output %s/jscpd-abs packages apps tests examples "
                       "wpt/runner (paths made repo-relative by summarize.py; default min-tokens 50, .gitignore respected)" % RD,
        'complexity': "mkdir -p %s/rca && rust-code-analysis-cli -m -O json -o %s/rca $(sed 's/^/-p /' %s/srcfiles.txt) "
                      "(every space of kind function, closures included: cognitive.sum / cyclomatic.sum)" % (RD, RD, RD),
        'graph': "python3 scripts/code-graph.py query %s code-audit \"<audit-pass.md canonical cycle / fan-in / fan-out / "
                 "edge-count SQL, verbatim>\" (the SQL as fired is in %s/tree-query-code-audit.json)" % (RD, RD),
        'dead': "cargo machete (unused deps) + python3 scripts/code-graph.py query %s code-audit \"SELECT s.symbol, s.crate, "
                "s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS "
                "NULL\" + python %s/dead_class.py (recipes.dead)" % (RD, RD),
        'coverage': "bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --lcov --output-path "
                    "target/coverage/lcov.info && cargo llvm-cov report --workspace --locked; TOTAL row, Lines cover column)",
        'churn': "git log --reverse --numstat --format=@%%H %s..HEAD -- %s (file-level: adds in a file's 2nd..nth touching "
                 "commit are churn)" % (BASE, PATHSPEC),
        'hotspots': "churn per-file commit counts (%s..HEAD) x max cognitive per file from complexity (a measured 0 scores 0)" % BASE,
        'mutation': "cargo test --locked -p {unit} -p blitz-tests --lib %s (the unmutated pass, by hand, exit 0) · "
                    "mkdir -p target/mutants-tmp && TMPDIR=$PWD/target/mutants-tmp timeout -s INT -k 60 900 cargo mutants "
                    "--workspace --file packages/{unit}/src/element_id.rs --file packages/{unit}/src/dioxus_document.rs "
                    "--test-package={unit},blitz-tests --cargo-test-arg=--lib %s --baseline=skip --timeout 120 -j 4 "
                    "--output %s/mutants-{unit}" % (' '.join('--test=' + t for t in TESTS),
                                                    ' '.join('--cargo-test-arg=--test=' + t for t in TESTS), RD),
        'head_overshoot': "git log --format=%%H -1 -S'%s · complete' -- .andromeda/master-route.md · git rev-list --count "
                          "%s..HEAD · git diff --numstat %s..HEAD -- %s" % (MARKER, head, head, PATHSPEC),
    },
    'recipes': {'dead': base['recipes']['dead']},
    'corrections': [
        {'target_sha': 'd4113768df08d45f0c6494f91134031b17ac9023', 'field': 'commands.complexity',
         'was': "mkdir -p {run_dir}/rca && rust-code-analysis-cli -m -O json -o {run_dir}/rca $(sed 's/^/-p /' {run_dir}/srcfiles.txt)",
         'now': "mkdir -p .andromeda/runs/2026-10-06T05-17-25-code-audit/rca && rust-code-analysis-cli -m -O json -o "
                ".andromeda/runs/2026-10-06T05-17-25-code-audit/rca $(sed 's/^/-p /' "
                ".andromeda/runs/2026-10-06T05-17-25-code-audit/srcfiles.txt)",
         'note': "path-grammar fill: the Epoch 2 record's correction of this field left `{run_dir}` unresolved, so the "
                 "command is not replayable as written; the run dir is the one that correction's own `was` text names "
                 "(read from the ledger only)"},
    ],
    'skips': skips,
}
assert record['recipes']['dead']
json.dump(record, open(os.path.join(rd, 'record.json'), 'w', encoding='utf-8'), ensure_ascii=False)
line = json.dumps(record, ensure_ascii=False)
assert '\n' not in line
print('record.json', len(line.encode('utf-8')), 'bytes;', 'ledger records before append:', len(recs))
