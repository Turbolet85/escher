"""Write RUN_DIR/meta.json and RUN_DIR/c-mutation.json for assemble.py.

usage: build_meta.py RUN_DIR BASELINE_JSON
The mutation block folds each unit's c-mutation-{unit}.json; commands are the firing forms as run.
"""
import json, os, re, subprocess, sys, time

rd, base_p = sys.argv[1:3]
R = '.andromeda/runs/2026-10-18T22-58-09-code-audit'
assert rd.rstrip('/') == R
base = json.load(open(base_p, encoding='utf-8'))
sha = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True, check=True).stdout.strip()
BSHA = base['sha']
SRC = "'*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'"

units = ['seven_guis', 'blitz-test-harness', 'dioxus-native-dom', 'blitz-dom/clock', 'escher-driver',
         'blitz-dom/engine-debts']
FILES = {'seven_guis': ['examples/seven_guis/src/stand.rs'],
         'blitz-test-harness': ['packages/blitz-test-harness/src/harness.rs'],
         'dioxus-native-dom': ['packages/dioxus-native-dom/src/snapshot.rs'],
         'blitz-dom/clock': ['packages/blitz-dom/src/clock.rs']}
SETKEY = {'seven_guis': 'seven_guis', 'escher-driver': 'escher-driver'}
mut = {'scoped_units': [], 'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
       'score_formula': 'caught/(caught+missed)', 'survivors': [], 'host': 'x86_64-unknown-linux-gnu',
       'not_measured': []}
skips, commands = [], {}
for u in units:
    slug = u.replace('/', '_')
    p = f'{rd}/c-mutation-{slug}.json'
    if not os.path.isfile(p):
        skips.append({'metric': f'mutation:{u}', 'reason': 'budget-exhausted'})
        continue
    r = json.load(open(p))
    mut['scoped_units'].append(u)
    hp = json.load(open(f'{rd}/handpass-{SETKEY.get(u, "broad")}.json'))
    cmd = open(f'{rd}/cmd-{slug}.txt').read().strip()
    commands[f'mutation:{u}'] = (hp['command'] + f' (the unmutated pass, by hand, exit {hp["exit"]})'
                                 + ' · mkdir -p target/mutants-tmp && ' + cmd)
    st = r.get('state')
    t = r.get('timing')
    if st == 'complete':
        c = r['counts']
        if r['unviable_dominant']:
            mut['unit_states'][u] = f"unviable-dominant {c['unviable']}/{r['tested']}"
            skips.append({'metric': f'mutation:{u}', 'reason': 'unviable-dominant'})
        else:
            mut['unit_states'][u] = f"complete {r['tested']}/{r['listed']}"
            if r['score'] is not None:
                mut['scores'][u] = r['score']
        mut['counts'][u] = c
        mut['survivors'] += r['survivors']
        mut['not_measured'] += r['not_measured']
        # wall_s of a complete invocation: end_time - start_time
        from datetime import datetime
        f = lambda s: datetime.fromisoformat(s.replace('Z', '+00:00'))
        t['wall_s'] = round((f(r['end_time']) - f(r['start_time'])).total_seconds())
    elif st == 'budget-exhausted':
        mut['unit_states'][u] = 'budget-exhausted'
        skips.append({'metric': f'mutation:{u}', 'reason': 'budget-exhausted'})
    elif st == 'baseline-test-failure':
        mut['unit_states'][u] = 'baseline-test-failure'
        skips.append({'metric': f'mutation:{u}', 'reason': 'baseline-test-failure'})
        t = None
    if t:
        if u in FILES:
            t['files'] = FILES[u]
        if u == 'blitz-dom/engine-debts':
            t['files'] = sorted({m['file'] for m in json.load(open(f'{rd}/mutants-{slug}/mutants.out/mutants.json'))})
            t['in_diff'] = f'{R}/engine-debts-at-head.diff'
        mut['timing'][u] = t
skips.append({'metric': 'mutation:escher-driver (shard 1/2 of its touched-files listing: 96 of 193)',
              'reason': 'budget-exhausted'})
skips.append({'metric': 'mutation:blitz-dom (its engine-debts files beyond the in-diff lines: 2818 listed under --file; '
                        '5772 whole)', 'reason': 'declined'})
skips.append({'metric': 'mutation:blitz-shell · blitz-vibey-script · dioxus-native-dom events.rs/keyboard_compat.rs/lib.rs '
                        '(upstream-only changes: 33 · 637 · 708 listed)', 'reason': 'declined'})
skips.append({'metric': 'mutation:seven_guis · escher-driver · blitz-test-harness · dioxus-native-dom (the units beyond '
                        'their touched files: 241 · 525 · 228 · 934 listed whole)', 'reason': 'declined'})
json.dump(mut, open(f'{rd}/c-mutation.json', 'w', encoding='utf-8'), indent=1)

commands.update({
    'sizes': f"git ls-files {SRC} > {R}/srcfiles.txt && tokei --output json $(cat {R}/srcfiles.txt) > {R}/tokei.json "
             f"(Rust code lines per file, reports[].stats.code; population = srcfiles.txt, 403 files; totals.units = "
             f"cargo metadata --no-deps --format-version 1 --locked workspace_members; summarized by python3 "
             f"{R}/sum_sizes_cx.py {R}/tokei.json {R}/rca {R}/srcfiles.txt {R}/c-; the summarizer reproduced the "
             f"baseline record's sizes, totals, complexity and complexity top exactly on a git archive of {BSHA})",
    'duplication': f"jscpd --format rust --absolute --reporters json --output {R}/jscpd-abs packages apps tests examples "
                   f"wpt/runner (paths made repo-relative by python3 {R}/sum_dup.py {R}/jscpd-abs/jscpd-report.json . "
                   f"{R}/c-duplication.json; default min-tokens 50, .gitignore respected; top = 10 largest fragments, ties "
                   f"in report order; split: a file under a `tests` directory segment is test, a pair of two is test, of "
                   f"none is src, else mixed; reproduced the baseline record exactly on a git archive of {BSHA})",
    'complexity': f"mkdir -p {R}/rca && rust-code-analysis-cli -m -O json -o {R}/rca $(sed 's/^/-p /' {R}/srcfiles.txt) "
                  f"(every space of kind function, closures included: cognitive.sum / cyclomatic.sum; summarized with sizes)",
    'graph': f"python3 {R}/graph_queries.py {R} (fires scripts/code-graph.py query {R} code-audit \"<audit-pass.md "
             f"canonical cycle / fan-in / fan-out / edge-count SQL, verbatim>\" one at a time; db_state fresh (built.head "
             f"1c9bc58b, no .rs/.toml/.lock change to HEAD); the SQL as fired is in {R}/tree-query-code-audit.json)",
    'dead': f"cargo machete > {R}/machete.out + python3 scripts/code-graph.py query {R} code-audit \"SELECT s.symbol, "
            f"s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE "
            f"r.callee IS NULL\" (fired by graph_queries.py) + python3 {R}/sum_dead.py {R} {R}/machete.out (recipes.dead, "
            f"verbatim from the baseline record; def_line is 0-based in the DB, printed +1)",
    'coverage': "bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --lcov --output-path "
                "target/coverage/lcov.info && cargo llvm-cov report --workspace --locked; TOTAL row, Lines cover column; "
                "exit 0)",
    'churn': f"git log --reverse --numstat --format=@%H {BSHA}..HEAD -- {SRC} (file-level: adds in a file's 2nd..nth "
             f"touching commit are churn; a rename counts to its new path; python3 {R}/sum_git.py {R} {BSHA}; the recipe "
             f"reproduced the baseline record's churn and hotspots on its own window)",
    'hotspots': f"churn per-file commit counts ({BSHA}..HEAD) x max cognitive per file from complexity (a measured 0 "
                f"scores 0)",
    'mutation': f"per scoped unit: its `mutation:{{unit}}` entry (the unmutated pass, by hand, then cargo mutants on "
                f"nextest's `mutants` profile with --baseline=skip under the confirmed footprint, driven sequentially by "
                f"python3 {R}/mutate.py {R}; test sets in {R}/testsets.json = the baseline record's set for the unit "
                f"(seven_guis · escher-driver; blitz-dom/range-keyboard's as `broad` for the rest) + every test target "
                f"added since {BSHA}; the in-diff input built by python3 {R}/blame_diff.py "
                f"48680dfb98f3018dc511347f71da7909c9fa783a {R}/engine-debts-at-head.diff $(git diff --name-only "
                f"34bcf71c678c6b40960ee629e19af6c27b6986cb 48680dfb98f3018dc511347f71da7909c9fa783a -- "
                f"packages/blitz-dom/src); summarized by python3 {R}/sum_mutation.py)",
    'head_overshoot': f"git log --format=%H -1 -S'2026-10-18-engine-debts · complete' -- .andromeda/master-route.md "
                      f"(the epoch's gated chunk, flipped last) · git rev-list --count {sha}..HEAD · git diff --numstat "
                      f"{sha}..HEAD -- {SRC}",
})
meta = {
    'ts': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
    'epoch': 'Epoch 2 — Engine debts', 'mode': 'trend', 'sha': sha, 'baseline_sha': BSHA, 'span': 1,
    'head_overshoot': {'boundary_sha': sha, 'commits': 0, 'files': [],
                       'note': ("at the boundary: HEAD is 2026-10-18-engine-debts' gated complete flip. That chunk "
                                "waited gated while Epoch 3's first two chunks (2026-10-18-upstream-sync-determinism-"
                                "and-traces, 2026-10-18-one-virtual-clock) were committed on top of its PROVISIONAL "
                                "commit, so this boundary state holds their source too (275950dc0..1c9bc58b5: the "
                                "cursor cherry-pick, the document clock) — attributed to this record's epoch; the "
                                "Epoch 3 record's window starts at this sha")},
    'tool_versions': {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25', 'code-graph.py': 'f22becbb',
                      'cargo-machete': '0.9.2', 'cargo-llvm-cov': '0.9.1', 'cargo-mutants': '27.1.0',
                      'cargo-nextest': '0.9.146', 'rustc': '1.99.0'},
    'units': 28, 'commands': commands, 'recipes': base['recipes'], 'corrections': [], 'skips': skips,
}
json.dump(meta, open(f'{rd}/meta.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('meta.json + c-mutation.json written;', mut['unit_states'], mut['scores'])
