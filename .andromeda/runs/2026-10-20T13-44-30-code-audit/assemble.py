"""Assemble the ledger record (audit-pass.md §Schema) from this run's c-*.json twins.

usage: assemble.py {run_dir}  -> {run_dir}/record.json
"""
import json, os, subprocess, sys
from datetime import datetime, timezone

rd = sys.argv[1]
RDREL = '.andromeda/runs/2026-10-20T13-44-30-code-audit'
BASE = '7eff48f7aa9aad1d3d8dfa072999d1ed275a97db'
SRC = "'*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'"
J = lambda n: json.load(open(f'{rd}/{n}', encoding='utf-8'))
head = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True, check=True).stdout.strip()
recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8')]
b = [r for r in recs if r['sha'] == BASE][-1]

sz, cx, dup, g, dead, cov, ch, hot, hp = (J('c-sizes.json'), J('c-complexity.json'), J('c-duplication.json'),
                                         J('c-graph.json'), J('c-dead.json'), J('c-coverage.json'),
                                         J('c-churn.json'), J('c-hotspots.json'), J('handpass.json'))
UNITS = ['dioxus-native-dom', 'blitz-traits', 'blitz-vibey-script', 'escher-driver', 'blitz-dom']
mu = {u: J(f'c-mutation-{u}.json') for u in UNITS}

mutation = {'scoped_units': UNITS, 'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
            'score_formula': 'caught/(caught+missed)', 'survivors': [],
            'host': 'x86_64-unknown-linux-gnu', 'not_measured': []}
skips = [{'metric': 'mutation:seven_guis (only its tests changed in the window: examples/seven_guis/tests/*; '
                    'no src file touched)', 'reason': 'declined'}]
commands = {}
for u, m in mu.items():
    st = m['state']
    mutation['unit_states'][u] = st + (f" (total_mutants/mutants.json {m['unit_states_line']})" if 'unit_states_line' in m else '')
    tp = f'{rd}/mutants-{u}/mutants.out/timeout.txt'  # a timeout's site kept FULL: no slot leaves it unowned
    if os.path.exists(tp):
        mutation.setdefault('timeouts', []).extend(
            [l.split(': ', 1)[0], l.split(': ', 1)[1]] for l in open(tp, encoding='utf-8').read().splitlines() if l)
    if 'counts' in m:
        mutation['counts'][u] = m['counts']
        mutation['survivors'] += m['survivors']
        mutation['not_measured'] += m['not_measured']
    if 'score' in m and m['score'] is not None:
        mutation['scores'][u] = m['score']
    if st.startswith('complete') or st == 'budget-exhausted':
        t = {'scope': m['scope'], 'planned': m['planned'], 'tested': m['counts']['mutants'],
             'wall_s': m['wall_s'], 'jobs': 2, 'cap_s': m['cap_s']}
        if m['scope'] == 'files':
            t['files'] = sorted(m['files'])
        else:
            t['in_diff'] = f'{RDREL}/window.diff'
        mutation['timing'][u] = t
    if not st.startswith('complete'):
        reason = st.split(' ')[0]
        skips.append({'metric': f'mutation:{u}', 'reason': reason})
    commands[f'mutation:{u}'] = (m['hand_pass']['command'] + ' (the unmutated pass, by hand, exit '
                                 f"{m['hand_pass']['exit']}) · " + m['command'])
commands['mutation'] = (f'per scoped unit: its `mutation:{{unit}}` entry, driven sequentially by python3 {RDREL}/mutate.py '
                        f'{RDREL}; in-diff = git diff {BASE}..HEAD -- packages/ > {RDREL}/window.diff; test sets in '
                        f'{RDREL}/testsets.json = the baseline record\'s set per unit (broad for dioxus-native-dom, blitz-dom, '
                        'and here blitz-traits and blitz-vibey-script with -p blitz-vibey-script and its dom/preact/selection '
                        'targets added; escher-driver its own) + every test target added since the baseline (focus_redraw, '
                        'focus_within, select_interaction, stand_act_select, stand_lint, stand_load_wait, stand_snapshot_parts, '
                        f'text_extent); hand passes by python3 {RDREL}/handpass.py {RDREL}; planned from cargo mutants -p {{unit}} '
                        '{scope} --list --json; summarized by sum_mutation.py (pinned cover recipe)')
commands['sizes'] = (f"git ls-files {SRC} > {RDREL}/srcfiles.txt && tokei --output json $(cat {RDREL}/srcfiles.txt) > "
                     f"{RDREL}/tokei.json (Rust code lines per file; population = srcfiles.txt, {sz['totals']['files']} files; "
                     "totals.units = cargo metadata --no-deps --format-version 1 --locked workspace_members; summarized by "
                     f"python3 {RDREL}/sum_sizes_cx.py {RDREL}/tokei.json {RDREL}/rca {RDREL}/srcfiles.txt {RDREL}/c-; reproduced "
                     f"the baseline record's sizes, totals, complexity and top lists exactly on a git archive of {BASE})")
commands['complexity'] = (f"mkdir -p {RDREL}/rca && rust-code-analysis-cli -m -O json -o {RDREL}/rca $(sed 's/^/-p /' "
                          f"{RDREL}/srcfiles.txt) (every space of kind function, closures included: cognitive.sum / "
                          "cyclomatic.sum; summarized with sizes)")
commands['duplication'] = (f"jscpd --format rust --absolute --reporters json --output {RDREL}/jscpd-abs packages apps tests "
                           f"examples wpt/runner (paths made repo-relative by python3 {RDREL}/sum_dup.py "
                           f"{RDREL}/jscpd-abs/jscpd-report.json . {RDREL}/c-duplication.json; default min-tokens 50, .gitignore "
                           "respected; top = 10 largest fragments, ties in report order; split by a `tests` directory segment; "
                           f"reproduced the baseline record exactly on a git archive of {BASE})")
commands['graph'] = (f"python3 {RDREL}/graph_queries.py {RDREL} (fires scripts/code-graph.py query {RDREL} code-audit "
                     "\"<audit-pass.md canonical cycle / fan-in / fan-out / edge-count SQL, verbatim>\" one at a time; db_state "
                     "regenerated on the first query (built.head was 191e66fdb), fresh after; the regenerating call printed no "
                     "rows on stdout, so the run was fired again whole on the fresh DB; fan-in symbols printed less their "
                     "'rust-analyzer cargo {crate} {version}' prefix as '{crate} {path}'; the SQL as fired is in "
                     f"{RDREL}/tree-query-code-audit.json, the zero-ref result there cut to its row count)")
commands['dead'] = (f"cargo machete > {RDREL}/machete.out + python3 scripts/code-graph.py query {RDREL} code-audit \"SELECT "
                    "s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = "
                    f"s.symbol WHERE r.callee IS NULL\" (fired by graph_queries.py) + python3 {RDREL}/sum_dead.py {RDREL} "
                    f"{RDREL}/machete.out (recipes.dead, verbatim from the baseline record; def_line is 0-based in the DB, "
                    "printed +1; reproduced the baseline's unused-deps list and top-10 exactly)")
commands['coverage'] = ("bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --ignore-filename-regex "
                        "'keyboard_compat\\.rs$' --lcov --output-path target/coverage/lcov.info && cargo llvm-cov report "
                        "--workspace --locked --ignore-filename-regex 'keyboard_compat\\.rs$' && coverage_floor.py; TOTAL row, "
                        "Lines Cover column; exit 0)")
commands['churn'] = (f"git log --reverse --numstat --format=@%H {BASE}..HEAD -- {SRC} (file-level: adds in a file's 2nd..nth "
                     "touching commit are churn; files_churned = files touched by 2+ commits; a rename counts to its new path; "
                     f"python3 {RDREL}/sum_git.py {RDREL} {BASE} '<the baseline record's hotspots>')")
commands['hotspots'] = f'churn per-file commit counts ({BASE}..HEAD) x max cognitive per file from complexity (a measured 0 scores 0)'
commands['head_overshoot'] = (f"git log --format=%H -1 -S'2026-10-20-large-screen-in-parts · complete' -- .andromeda/master-route.md"
                              f" · git rev-list --count {head}..HEAD · git diff --numstat {head}..HEAD -- {SRC}")

rec = {
    'ts': datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
    'epoch': 'Epoch 4 — Visual lint and real-app readiness', 'mode': 'trend', 'sha': head, 'baseline_sha': BASE,
    'span': 1, 'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': head, 'commits': 0, 'files': [],
                       'note': "at the boundary: HEAD is 2026-10-20-large-screen-in-parts' complete flip, the last of the "
                               "epoch's seven markers; 2026-10-19-visual-lint was gated and flipped complete earlier "
                               "(c1a2e5b85), so the newest flip is the last marker's"},
    'tool_versions': {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'code-graph.py': 'f22becbb',
                      'cargo-machete': '0.9.2', 'cargo-llvm-cov': '0.9.1', 'cargo-mutants': '27.1.0',
                      'cargo-nextest': '0.9.146', 'rustc': '1.99.0'},
    'totals': {'loc': sz['totals']['loc'], 'files': sz['totals']['files'], 'units': 28},
    'duplication': {'pct': round(dup['pct'], 2), 'duplicated_lines': dup['duplicated_lines'],
                    'total_lines': dup['total_lines'], 'clones': dup['clones'], 'top': dup['top'], 'split': dup['split']},
    'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90',
                                      'over_ceiling', 'max', 'top')},
    'sizes': {k: sz[k] for k in ('file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
    'graph': g,
    'dead': {'unused_deps': dead['unused_deps'], 'zero_ref_candidates': dead['zero_ref_candidates'], 'top': dead['top']},
    'coverage': {'line': cov['line'], 'branch': cov['branch']},
    'churn': {'pct': ch['pct'], 'files_churned': ch['files_churned']},
    'hotspots': hot['top'],
    'mutation': mutation,
    'commands': commands,
    'recipes': {'dead': b['recipes']['dead']},
    'corrections': [],
    'skips': skips,
}
for u in UNITS:  # the per-unit survivor assert (audit-pass.md §Schema's Caps)
    if u in mutation['counts']:
        rows = [s for s in mu[u]['survivors']]
        assert len({(s, t) for s, t in rows}) == len(rows) == mutation['counts'][u]['missed'], u
json.dump(rec, open(f'{rd}/record.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('record assembled', mutation['unit_states'], mutation['scores'], len(mutation['survivors']), 'survivors')
