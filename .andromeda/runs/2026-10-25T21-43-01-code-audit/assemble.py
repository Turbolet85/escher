"""P3: assemble {run_dir}/record.json from the c-*.json twins (audit-pass.md §Schema).

usage: assemble.py RUN_DIR   (run from the repo root)
"""
import json, re, subprocess, sys
from datetime import datetime, timezone

rd = sys.argv[1]
J = lambda n: json.load(open(f'{rd}/{n}', encoding='utf-8'))
recs = []
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        pass
BASE = 'cbe90e22413191bb5d38504c62a6ebc93b4361f6'
base = [r for r in recs if r['sha'] == BASE][-1]
sha = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True, check=True).stdout.strip()
epoch = next(l for l in open('escher-0.2.0/working-route.md', encoding='utf-8') if l.startswith('### Epoch 5'))
epoch = re.sub(r'^###\s*', '', epoch).rstrip('\n')
RUN = rd
SRC = "'*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'"

sz, cx, dup = J('c-sizes.json'), J('c-complexity.json'), J('c-duplication.json')
g, dead, cov, ch = J('c-graph.json'), J('c-dead.json'), J('c-coverage.json'), J('c-churn.json')
scope = J('mutation-scope.json')['units']
UNITS = ['blitz-dom', 'seven_guis', 'wpt']
mut = {u: J(f'c-mutation-{u}.json') for u in UNITS}

mutation = {'scoped_units': UNITS, 'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
            'score_formula': 'caught/(caught+missed)', 'survivors': [], 'host': None, 'not_measured': []}
skips = [
    {'metric': 'mutation:dioxus-native (touched: src/lib.rs +2, upstream; no test target, the windowed launch reached by no headless test; the founder\'s «Не запускать (Рекомендую)»)', 'reason': 'declined'},
    {'metric': 'mutation:browser (touched: document_loader.rs, toolbar.rs +1 each and its manifest, upstream; no test on those files; the founder\'s «Не запускать (Рекомендую)»)', 'reason': 'declined'},
    {'metric': 'mutation:blitz-examples (touched: examples/multi_doc.rs, a root example; cargo mutants --list reads 0 mutants — no mutable target, a no-op)', 'reason': 'declined'},
    {'metric': 'mutation:blitz-tests (only its test files changed in the window: tests/blitz-tests/tests/*; no src)', 'reason': 'declined'},
]
commands = {}
for u in UNITS:
    m = mut[u]
    mutation['host'] = m['host']
    st = m['state']
    mutation['unit_states'][u] = (f"unviable-dominant {m['counts']['unviable']}/{m['tested']}"
                                  if st == 'unviable-dominant' else m.get('unit_state', st))
    if 'counts' in m:
        mutation['counts'][u] = m['counts']
        for s in m['survivors']:
            mutation['survivors'].append(s)
        for s in m['not_measured']:
            mutation['not_measured'].append(s)
        assert len(m['survivors']) == m['counts']['missed']
        assert len(m['not_measured']) == m['counts']['not_measured']
    if st == 'complete' and m.get('score') is not None:
        mutation['scores'][u] = m['score']
    if st in ('complete', 'budget-exhausted'):
        mutation['timing'][u] = {'scope': 'in-diff', 'in_diff': f'{RUN}/window.diff', 'planned': scope[u]['planned'],
                                 'tested': m['tested'], 'wall_s': m['wall_s'], 'jobs': 2, 'cap_s': m['run']['cap_s']}
    if st != 'complete':
        cn = m.get('counts') or {}
        skips.append({'metric': f'mutation:{u}' + (f" (complete {m['tested']}/{cn.get('mutants')}: caught {cn.get('caught')}, missed {cn.get('missed')}, timeout {cn.get('timeout')}, unviable {cn.get('unviable')} — the unviable are compile errors of the replacement (E0599 no `Harness::new`, E0277 no `Default` for `Element`), no disk or lock fault in any log; unscored by the letter)" if st == 'unviable-dominant' else ''), 'reason': st if st in ('budget-exhausted', 'baseline-test-failure',
                                                                        'unviable-dominant') else 'budget-exhausted'})
    hand = ''
    try:
        h = J(f'hand-{u}.json')
        hand = f"{h['command']} (the unmutated pass, by hand, exit {h['exit']}) · "
    except OSError:
        pass
    commands[f'mutation:{u}'] = hand + m['run'].get('command', '(not run)')
assert len({tuple(s) for s in mutation['survivors']}) == len(mutation['survivors'])

commands.update({
    'sizes': f"git ls-files {SRC} > {RUN}/srcfiles.txt && tokei --output json $(cat {RUN}/srcfiles.txt) > {RUN}/tokei.json (Rust code lines per file, tokei's Rust report stats.code; population = srcfiles.txt, {sz['totals']['files']} files; totals.units = cargo metadata --no-deps --format-version 1 --locked workspace_members; summarized by python3 {RUN}/sum_sizes_cx.py . {RUN}/tokei.json {RUN}/rca {RUN}/srcfiles.txt {RUN}/c-; the summarizer reproduced the baseline record's sizes, totals and complexity exactly on a git archive of {BASE})",
    'complexity': f"mkdir -p {RUN}/rca && rust-code-analysis-cli -m -O json -o {RUN}/rca $(sed 's/^/-p /' {RUN}/srcfiles.txt) (every space of kind function, closures included: cognitive.sum / cyclomatic.sum; percentiles nearest-rank; over_ceiling cognitive > 15; summarized with sizes)",
    'duplication': f"jscpd --format rust --absolute --reporters json --output {RUN}/jscpd-abs packages apps tests examples wpt/runner (paths made repo-relative by python3 {RUN}/sum_dup.py {RUN}/jscpd-abs/jscpd-report.json . {RUN}/c-duplication.json; default min-tokens 50, .gitignore respected; top = 10 largest fragments, ties in report order; split by a `tests` directory segment; reproduced the baseline record exactly on a git archive of {BASE})",
    'graph': f"python3 {RUN}/graph_queries.py {RUN} (fires python3 scripts/code-graph.py query {RUN} code-audit \"<audit-pass.md canonical cycle / fan-in / fan-out / edge-count SQL, verbatim>\" one at a time; db_state fresh (built.head 55bf34259, no .rs change to HEAD); fan-in symbols printed less their 'rust-analyzer cargo {{crate}} {{version}}' prefix as '{{crate}} {{path}}'; summarized by python3 {RUN}/sum_graph_dead.py {RUN} {RUN}/machete.out .)",
    'dead': f"cargo machete > {RUN}/machete.out + python3 scripts/code-graph.py query {RUN} code-audit \"SELECT s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL\" (fired by graph_queries.py) + python3 {RUN}/sum_graph_dead.py {RUN} {RUN}/machete.out . (recipes.dead, verbatim from the baseline record; def_line 0-based in the DB, printed +1)",
    'coverage': cov['command'],
    'churn': f"git log --reverse --numstat --format=@%H {BASE}..HEAD -- {SRC} (file-level: adds in a file's 2nd..nth touching commit are churn; files_churned = files touched by 2+ commits; a rename counts to its new path; python3 {RUN}/sum_git.py {RUN} {BASE} '<the baseline record's hotspots>')",
    'hotspots': f"churn per-file commit counts ({BASE}..HEAD) x max cognitive per file from complexity (a measured 0 scores 0)",
    'head_overshoot': f"git log --format=%H -1 -S'2026-10-25-cold-agent-bar · complete' -- .andromeda/master-route.md · git rev-list --count {sha}..HEAD · git diff --numstat {sha}..HEAD -- {SRC}",
    'mutation': f"per scoped unit: its `mutation:{{unit}}` entry, driven sequentially by python3 {RUN}/mutate.py {RUN} blitz-dom seven_guis wpt; in-diff = git diff {BASE}..HEAD -- packages/ examples/ apps/ wpt/ > {RUN}/window.diff; test sets in {RUN}/testsets.json = blitz-dom: the baseline record's blitz-dom set + hit_test_dropped_nodes, stand_cells_served, stand_load_page; seven_guis: the baseline record's escher-driver set under -p seven_guis -p blitz-tests -p escher-driver + stand_cells_served, stand_load_page; wpt: its own tests under the tool's own baseline; planned from cargo mutants -p {{unit}} --in-diff {RUN}/window.diff --list --json; summarized by python3 {RUN}/sum_mutation.py {RUN} blitz-dom seven_guis wpt (pinned cover recipe)",
})

rec = {
    'ts': datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'), 'epoch': epoch, 'mode': 'trend', 'sha': sha,
    'baseline_sha': BASE, 'span': 1, 'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': sha, 'commits': 0, 'files': [],
                       'note': "at the boundary: HEAD is 2026-10-25-cold-agent-bar's complete flip, the last of the epoch's five markers; no gated record in the epoch"},
    'tool_versions': {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'code-graph.py': 'f22becbb',
                      'cargo-machete': '0.9.2', 'cargo-llvm-cov': '0.9.1', 'cargo-mutants': '27.1.0',
                      'cargo-nextest': '0.9.146', 'rustc': '1.99.0'},
    'totals': dict(sz['totals'], units=28),
    'duplication': dup['duplication'], 'complexity': cx['complexity'], 'sizes': sz['sizes'],
    'graph': g['graph'], 'dead': dead['dead'], 'coverage': cov['coverage'], 'churn': ch['churn'],
    'hotspots': ch['hotspots'], 'mutation': mutation, 'commands': commands,
    'recipes': {'dead': base['recipes']['dead']}, 'corrections': [], 'skips': skips,
}
json.dump(rec, open(f'{rd}/record.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('record ok', rec['epoch'], rec['sha'][:10], json.dumps(mutation['unit_states']), json.dumps(mutation['scores']),
      'survivors', len(mutation['survivors']), 'nm', len(mutation['not_measured']), 'skips', len(skips))
