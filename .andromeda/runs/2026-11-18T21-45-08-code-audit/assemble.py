"""Assemble record.json (audit-pass.md §Schema). usage: python3 assemble.py {run_dir} {ts}"""
import json, sys
from datetime import datetime

rd, ts = sys.argv[1:3]
R = '.andromeda/runs/2026-11-18T21-45-08-code-audit'
HEAD = 'bc766124059bc619b94019fe200681196553acbc'
BASE = '0d93539f19d1662658302f59480fdeb334587eff'
J = lambda n: json.load(open(f'{rd}/{n}', encoding='utf-8'))
recs = [r['record'] for r in J('ledger.json')]
recs = [json.loads(r) if isinstance(r, str) else r for r in recs]
b = [r for r in recs if r['sha'] == BASE][-1]
sz, cx, dp, gr, dd, ch = (J(f'c-{k}.json') for k in ('sizes', 'complexity', 'duplication', 'graph', 'dead', 'churn'))
forms = J('forms.json')
m = {u: J(f'c-mutation-{u}.json') for u in ('seven_guis', 'browser', 'blitz-paint')}
SRC = "-- '*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'"


def wall(r):
    f = lambda s: datetime.fromisoformat(s[:26].rstrip('Z') if len(s) > 26 else s.rstrip('Z'))
    return round((f(r['end_time']) - f(r['start_time'])).total_seconds())


# escher-driver: the before-stop copy (89/96), never a score
ed = json.load(open(f'{rd}/mutants-escher-driver.before-stop-2026-11-19/mutants.out/outcomes.json', encoding='utf-8'))
edc = {'mutants': 0, 'caught': 0, 'missed': 0, 'not_measured': 0, 'timeout': 0, 'unviable': 0}
for x in ed['outcomes']:
    if x['scenario'] == 'Baseline':
        continue
    edc['mutants'] += 1
    edc[{'CaughtMutant': 'caught', 'MissedMutant': 'missed', 'Timeout': 'timeout', 'Unviable': 'unviable'}[x['summary']]] += 1
assert edc['missed'] == 0

caps = {u: json.load(open(f'{rd}/state-{u}.json'))['cap_s'] for u in ('seven_guis', 'browser', 'blitz-paint')}
timing = {u: {'scope': 'in-diff', 'base': BASE, 'planned': m[u]['planned'], 'tested': m[u]['tested'], 'wall_s': wall(m[u]),
              'jobs': 6, 'cap_s': caps[u]} for u in m}

BTF_NOTE = {
    'wgpu_texture': 'the unit holds no test: nextest exits 4 on `no tests to run` in the unmutated tree (cargo-mutants exit 4), so no mutant was tried; none tested',
    'dioxus-native': 'the unit holds no test: nextest exits 4 on `no tests to run` in the unmutated tree (cargo-mutants exit 4), so no mutant was tried; none tested',
    'wasm_hello': 'the unit holds no test: nextest exits 4 on `no tests to run` in the unmutated tree (cargo-mutants exit 4), so no mutant was tried; none tested',
    'wpt': 'NOT a no-test unit: 28 tests, 3 failed in the unmutated copy (nextest 100, cargo-mutants exit 4), none tested; the read cause an environment fault of this run\'s form: tempfile::tempdir() at wpt/runner/src/main.rs:827 panicked NotFound under the RELATIVE TMPDIR=target/mutants-tmp, resolved from the test\'s own cwd inside the copy; an absolute TMPDIR is the next run\'s form'}

skips = [{'metric': 'mutation:seven_guis', 'reason': 'unviable-dominant',
          'note': 'complete ' + json.dumps(m['seven_guis']['counts']) + '; unscored by the letter; every unviable a genuine compile error (4 E0277, 3 E0599: a Default::default() or Harness::new/from/from_iter replacement on a type that has none), not a host fault'}]
skips += [{'metric': f'mutation:{u}', 'reason': 'baseline-test-failure', 'note': BTF_NOTE[u]} for u in ('wpt', 'wgpu_texture', 'dioxus-native', 'wasm_hello')]
skips += [{'metric': 'mutation:blitz-dom', 'reason': 'postponed',
           'note': 'NOT MEASURED THIS BOUNDARY - postponed on the founder\'s word, not declined (relayed to overseer1 2026-11-19 05:14Z: «Да блиц дом оставляем на потом»); no mutant tested. Its in-diff listing is 1161; the founder\'s confirmed scope (2026-11-18, «61, два места исключены (Recommended)») is 61: a DEPARTURE FROM THE LETTER excluding the two FnValue sites --exclude-re \'replace (BaseDocument::scroll_state|node_scroll_range) -> \' (packages/blitz-dom/src/scrolling.rs:537 scroll_state x1000 and :12 node_scroll_range x100 combinatorial tuple returns). `postponed` is outside the letter\'s reason set (none fits a unit held for a later run): named here, never read as declined'},
          {'metric': 'mutation:rdme', 'reason': 'declined', 'note': 'out of the tier by architecture:mutation-tier-excludes-demo-shells'}]
DE = {'blitz-examples': 'touched: three root examples/*.rs (custom_widget, paint_bench, screenshot) and the root Cargo.toml/Cargo.lock; cargo mutants --in-diff --list reads 0',
      'blitz-tests': 'touched: 16 test files only; cargo mutants --in-diff --list reads 0',
      'counter': 'touched: its Cargo.toml only; cargo mutants --in-diff --list reads 0',
      'todomvc': 'touched: its Cargo.toml only; cargo mutants --in-diff --list reads 0',
      'transparent': 'touched: its Cargo.toml only; cargo mutants --in-diff --list reads 0'}
skips += [{'metric': f'mutation:{u}', 'reason': 'diff-empty', 'note': n} for u, n in DE.items()]

states = {'seven_guis': 'unviable-dominant 7/13', 'browser': 'complete 4/4', 'blitz-paint': 'complete 43/43',
          'escher-driver': 'exited-incomplete 89/96', 'blitz-dom': 'postponed 0/61 (1161 listed in-diff; not measured this boundary)',
          'wpt': 'baseline-test-failure', 'wgpu_texture': 'baseline-test-failure', 'dioxus-native': 'baseline-test-failure',
          'wasm_hello': 'baseline-test-failure', 'rdme': 'declined'}
states.update({u: 'diff-empty' for u in DE})

TEST_ARGS = lambda u: ' '.join(f'--cargo-test-arg=--test={t}' for t in forms[u]['targets'])
cmds = {
    'sizes': f"git ls-files '*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**' > {R}/srcfiles.txt && tokei --output json $(cat {R}/srcfiles.txt) > {R}/tokei.json (Rust code lines per file, stats.code; population = srcfiles.txt, 510 files; totals.units = cargo metadata --no-deps --format-version 1 --locked workspace_members; summarized by python3 {R}/sum_sizes_cx.py {R}/tokei.json {R}/rca {R}/srcfiles.txt 28 {R}/c- ; the summarizer reproduced the baseline record's sizes, totals and complexity scalars exactly on a git archive of {BASE})",
    'complexity': f"mkdir -p {R}/rca && rust-code-analysis-cli -m -O json -o {R}/rca $(sed 's/^/-p /' {R}/srcfiles.txt) (every space of kind function: cognitive.sum / cyclomatic.sum; percentiles nearest-rank; over_ceiling cognitive > 15; top by cognitive desc, then file, line; summarized with sizes)",
    'duplication': f"mise exec node@26.8.2 -- jscpd --format rust --absolute --reporters json --output {R}/jscpd-abs packages apps tests examples wpt/runner (jscpd 5.4.0, the node 26.8.2 install: the active node carries no jscpd; paths made repo-relative by python3 {R}/sum_dup.py {R}/jscpd-abs/jscpd-report.json . {R}/c-duplication.json; pct rounded to 2 places; default min-tokens 50, .gitignore respected; top = 10 largest clones, ties in report order; split by a `tests` directory segment; reproduced the baseline record's scalars and top exactly on a git archive of {BASE})",
    'graph': f"python3 {R}/graph_dead.py {R} {R}/machete.out . (fires python3 scripts/code-graph.py query {R} code-audit \"<audit-pass.md canonical cycle / fan-in / fan-out / edge-count SQL, verbatim>\" one at a time; every query read db_state fresh; fan-in symbols printed less their 'rust-analyzer cargo {{crate}} {{version}}' prefix as '{{crate}} {{path}}')",
    'dead': f"cargo machete > {R}/machete.out + python3 scripts/code-graph.py query {R} code-audit \"SELECT s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL\" (fired by graph_dead.py) + recipes.dead, verbatim from the baseline record (def_line 0-based in the DB, printed +1; top ordered by file, def line, symbol; the top-10 equals the baseline record's)",
    'coverage': "bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --ignore-filename-regex 'keyboard_compat\\.rs$' --lcov --output-path target/coverage/lcov.info && cargo llvm-cov report ... && coverage_floor.py; TOTAL row, Lines Cover column 73.91 %, rounded to one place; exit 0, 3 of 3 crates at or above their floors)",
    'churn': f"git log --reverse --numstat --format=@%H {BASE}..HEAD {SRC} (file-level: adds in a file's 2nd..nth touching commit are churn; files_churned = files touched by 2+ commits; a rename counts to its new path; python3 {R}/sum_git.py {R} {BASE} HEAD {R}/c-perfile-cx.json; 6 commits, 64 files, 4283 adds, 26 churned)",
    'hotspots': f"churn per-file commit counts ({BASE}..HEAD) x max cognitive per file from complexity (a measured 0 scores 0)",
    'head_overshoot': f"git log --format=%H -1 -S'2026-11-18-cold-agent-live-run · complete' -- .andromeda/master-route.md · git rev-list --count {HEAD}..HEAD · git diff --numstat {HEAD}..HEAD {SRC}",
    'mutation_peak': f"cargo clean --target-dir target/mutants-peak && systemd-run --user --scope -q --collect -p MemoryAccounting=yes sh {R}/peak.sh {R} env NEXTEST_PROFILE=mutants cargo nextest run --locked -p browser --target-dir target/mutants-peak (rc=0 anon_peak=4253446144, 44 s, 43 passed; browser the scoped unit with the largest closure, 715 lines of cargo tree)",
    'mutation': f"per run unit its `mutation:{{unit}}` entry, driven sequentially by python3 {R}/mutate.py {R} seven_guis browser wpt wgpu_texture dioxus-native wasm_hello blitz-paint escher-driver blitz-dom (jobs 6, the host fit, no core pin; forms built by python3 {R}/forms.py {R} from each unit's recorded form, new test targets of the epoch NOT added to the recorded lists); in-diff = git diff {BASE}..{HEAD} {SRC} > {R}/epoch.diff; planned from cargo mutants -p {{unit}} --in-diff {R}/epoch.diff [the form's flags] --list --json; caps per collectors.md §The mutation cap (no unit's baseline timing at jobs 6: floor 1800 s, re-sized once from the unit's own live rate); --baseline=skip units' hand pass recorded in {R}/handpass-{{unit}}.txt (all exit 0); summarized by python3 {R}/sum_mutation.py {R} . {{units}} (pinned cover recipe). History: fired 2026-11-18 21:5xZ and stopped 36 s in (hand pass only, no mutant) on the founder's word, POSTPONED ({R}/tier-postponed.md); re-fired detached 2026-11-18 22:28Z on his «Ярус по diff, сейчас (Recommended)» · «Запустить и не опрашивать (Recommended)»; blitz-dom's state file written by the escher overseer 05:15Z (postponed); driver killed 2026-11-19 05:42:54Z by overseer1 on the founder's word during escher-driver ({R}/stopped-by-overseer1.txt)",
}
for u in ('seven_guis', 'browser', 'blitz-paint', 'wpt', 'wgpu_texture', 'dioxus-native', 'wasm_hello'):
    cmds[f'mutation:{u}'] = forms[u]['form']
cmds['mutation:escher-driver'] = (forms['escher-driver']['form'] + f" [STOPPED mid-invocation 2026-11-19 05:42:54Z on the operator's word (overseer1, the founder's word); started 00:56:21Z, wall to the stop 17193 s, cap 21600 s re-sized once at 12/96; byte copy {R}/mutants-escher-driver.before-stop-2026-11-19 taken before the stop, read: 89/96 tested (88 caught, 1 unviable, 0 missed), no end_time; it is an exact prefix of the live dir's outcomes, which adds 5 Failure rows = builds interrupted by the stop, not results. "
    f"RESUME (collectors.md §A unit stopped mid-invocation): mkdir -p target/mutants-tmp; a further byte copy of the before-stop copy at a new --output, checked equal; the form + --iterate --list --json must list the 7 untested mutants (96 less the copy's 88 caught and 1 unviable); then the form + --iterate at that output under cap 21600-17193 = 4407 s; join to one mutants.out and score over the joined 96]")
cmds['mutation:blitz-dom'] = forms['blitz-dom']['form'] + ' [NOT FIRED: postponed on the founder\'s word; to run: delete its state file, then mutate.py with the one unit blitz-dom; mkdir -p target/mutants-tmp first]'
cmds['mutation:rdme'] = None
for u in DE:
    cmds[f'mutation:{u}'] = None

rec = {
    'ts': ts, 'epoch': 'Epoch 6 — Polish & ship', 'version': 'escher-0.3.0', 'mode': 'trend', 'sha': HEAD, 'baseline_sha': BASE,
    'span': 1, 'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': HEAD, 'commits': 0, 'files': [],
                       'note': "at the boundary: HEAD is 2026-11-18-cold-agent-live-run's complete flip, the last of the epoch's eight markers; no marker of the epoch was ever gated"},
    'tool_versions': {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'code-graph.py': 'f22becbb',
                      'code-graph-views.sql': 'acb08de4', 'rust-analyzer': '1.99.0', 'cargo-machete': '0.9.2',
                      'cargo-llvm-cov': '0.9.1', 'cargo-mutants': '27.1.0', 'cargo-nextest': '0.9.146', 'rustc': '1.99.0'},
    'totals': sz['totals'],
    'duplication': dp,
    'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'max', 'top')},
    'sizes': {k: sz[k] for k in ('file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
    'graph': {k: gr[k] for k in ('cycles', 'cycle_paths', 'fan_in_top', 'fan_out', 'cross_unit_edges')},
    'dead': {k: dd[k] for k in ('unused_deps', 'zero_ref_candidates', 'top')},
    'coverage': {'line': 73.9, 'branch': None},
    'churn': ch['churn'], 'hotspots': ch['hotspots'],
    'mutation': {
        'scoped_units': sorted(states), 'unit_states': states,
        'scores': {u: m[u]['score'] for u in ('browser', 'blitz-paint')},
        'counts': {**{u: m[u]['counts'] for u in m}, 'escher-driver': edc},
        'timing': timing, 'score_formula': 'caught/(caught+missed)',
        'survivors': m['browser']['survivors'] + m['blitz-paint']['survivors'] + m['seven_guis']['survivors'],
        'host': 'x86_64-unknown-linux-gnu',
        'host_fit': {'cores': 32, 'load': 0.5, 'mem_available': 36934236 * 1024, 'peak': 4253446144, 'unit': 'browser', 'jobs': 6,
                     'note': 'read 2026-11-18 22:27Z before the detached fire: CPU bound 7 at load 0.50, memory bound 6 = 0.75 x MemAvailable / peak; the first read 21:5xZ (load 6.02, MemAvailable 36640000 kB) gave 6 too. counts[escher-driver] is the before-stop copy\'s partial tally, no score'},
        'not_measured': []},
    'commands': cmds,
    'recipes': b['recipes'],
    'corrections': [],
    'skips': skips,
}
for u in rec['mutation']['scores']:
    c = rec['mutation']['counts'][u]
    assert len([s for s in rec['mutation']['survivors'] if s[0] == u]) == c['missed']
json.dump(rec, open(f'{rd}/record.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('ok', len(json.dumps(rec)))
