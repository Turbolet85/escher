"""Assemble {run_dir}/record.json (audit-pass.md §Schema) from the c-*.json twins. Run from the repo root."""
import json, datetime

RD = '.andromeda/runs/2026-10-06T05-17-25-code-audit'
SHA = 'd4113768df08d45f0c6494f91134031b17ac9023'
L = lambda n: json.load(open(f'{RD}/c-{n}.json', encoding='utf-8'))
dup, cx, sz, gr, dd, cov = L('duplication'), L('complexity'), L('sizes'), L('graph'), L('dead'), L('coverage')
mu = L('mutation-escher-telemetry')
unit = mu['unit']
complete = mu['unit_state'].startswith('complete')
unviable_dominant = mu['counts']['unviable'] > (mu['counts']['caught'] + mu['counts']['missed'] + mu['counts']['not_measured'] + mu['counts']['timeout'])

skips = [{'metric': 'churn', 'reason': 'no-baseline'}, {'metric': 'hotspots', 'reason': 'no-baseline'},
         {'metric': 'mutation:seven_guis', 'reason': 'declined'}]
if unviable_dominant:
    skips.append({'metric': f'mutation:{unit}', 'reason': 'unviable-dominant'})

rec = {
    'ts': datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
    'epoch': 'Epoch 1 — Foundation', 'mode': 'baseline', 'sha': SHA, 'baseline_sha': None, 'span': None,
    'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': SHA, 'commits': 0, 'files': [], 'note': 'at the boundary: HEAD is the 2026-10-06-cold-agent-run-pipe complete flip'},
    'tool_versions': {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'code-graph.py': 'd0425fb1',
                      'cargo-machete': '0.9.2', 'cargo-llvm-cov': '0.9.1', 'cargo-mutants': '27.1.0', 'rustc': '1.99.0'},
    'totals': {'loc': sz['totals']['loc'], 'files': sz['totals']['files'], 'units': 28},
    'duplication': {k: dup[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split')},
    'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'max')}
                  | {'top': [[t['fn'], t['file'], t['value']] for t in cx['top']]},
    'sizes': {k: sz[k] for k in ('file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
    'graph': {k: gr[k] for k in ('cycles', 'cycle_paths', 'fan_in_top', 'fan_out', 'cross_unit_edges')},
    'dead': {'unused_deps': dd['unused_deps'], 'zero_ref_candidates': dd['zero_ref_candidates'], 'top': dd['top'][:10],
             'fp_classes': dd['fp_classes'], 'zero_ref_rows': dd['zero_ref_rows'], 'excluded_test_path': dd['excluded_test_path']},
    'coverage': {'line': cov['line'], 'branch': cov['branch']},
    'churn': {'pct': None, 'files_churned': None},
    'hotspots': [],
    'mutation': {
        'scoped_units': [unit],
        'unit_states': {unit: (f"unviable-dominant {mu['counts']['unviable']}/{mu['counts']['mutants']}" if unviable_dominant else mu['unit_state'])},
        'scores': {} if unviable_dominant or mu['score'] is None else {unit: mu['score']},
        'counts': {unit: mu['counts']},
        'score_formula': mu['score_formula'],
        'survivors': mu['survivors'],
        'host': mu['host'],
        'not_measured': mu['not_measured'],
    },
    'commands': {
        'sizes': "git ls-files '*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**' > .andromeda/runs/2026-10-06T05-17-25-code-audit/srcfiles.txt && tokei --output json $(cat .andromeda/runs/2026-10-06T05-17-25-code-audit/srcfiles.txt) (Rust code lines per file; population = srcfiles.txt, 286 files)",
        'duplication': 'jscpd --format rust --absolute --reporters json --output .andromeda/runs/2026-10-06T05-17-25-code-audit/jscpd-abs packages apps tests examples wpt/runner (paths made repo-relative by summarize.py; default min-tokens 50, .gitignore respected)',
        'complexity': 'rust-code-analysis-cli -m -O json -o .andromeda/runs/2026-10-06T05-17-25-code-audit/rca $(sed s/^/-p\\ / .andromeda/runs/2026-10-06T05-17-25-code-audit/srcfiles.txt) (per function space: cognitive.sum / cyclomatic.sum)',
        'graph': 'python3 scripts/code-graph.py query .andromeda/runs/2026-10-06T05-17-25-code-audit code-audit "<audit-pass.md canonical cycle / fan-in / fan-out / edge-count SQL>" (exact SQL in tree-query-code-audit.json)',
        'dead': 'cargo machete (unused deps) + python3 scripts/code-graph.py query .andromeda/runs/2026-10-06T05-17-25-code-audit code-audit "<zero-ref SQL, no LIMIT, test/trait-impl classes in SQL>" + summarize.py classing (recipe in c-dead.json)',
        'coverage': 'bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --lcov --output-path target/coverage/lcov.info && cargo llvm-cov report --workspace --locked; TOTAL row)',
        'churn': None, 'hotspots': None,
        'mutation': "cargo mutants --workspace --file 'packages/escher-telemetry/src/*.rs' --test-package=escher-telemetry,blitz-tests --cargo-test-arg=--lib --cargo-test-arg=--test=telemetry_panic_hook --cargo-test-arg=--test=telemetry_init_idempotent --cargo-test-arg=--test=telemetry_stdout_silent --cargo-test-arg=--test=telemetry_scrub --baseline=skip --timeout 120 -j 1 --output .andromeda/runs/2026-10-06T05-17-25-code-audit/mutants-escher-telemetry",
        'head_overshoot': "git log --format=%H -1 -S'2026-10-06-cold-agent-run-pipe · complete' -- .andromeda/master-route.md · git rev-list --count d4113768df08d45f0c6494f91134031b17ac9023..HEAD · git diff --numstat d4113768df08d45f0c6494f91134031b17ac9023..HEAD filtered to srcfiles.txt's definition",
    },
    'corrections': [],
    'skips': skips,
}
# per-unit survivor asserts (audit-pass.md Caps)
assert len({tuple(s) for s in rec['mutation']['survivors']}) == len(rec['mutation']['survivors']) == mu['counts']['missed']
assert len(rec['mutation']['not_measured']) == mu['counts']['not_measured']
json.dump(rec, open(f'{RD}/record.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('record.json written', len(json.dumps(rec)), 'bytes', rec['mutation']['unit_states'], rec['mutation']['scores'])
