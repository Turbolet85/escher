"""Build this run's firing form per unit from its recorded form (collectors.md §The mutation scope).
usage: python3 forms.py {run_dir}  -> {run_dir}/forms.json, prints a check per unit.
Recorded form = newest ledger record whose unit_states reads the unit `complete`: its `mutation:{unit}` entry (first
invocation only). This run re-resolves: --in-diff, --output, -j, --timeout kept, TMPDIR. Scope flags are never carried.
Every --test= target is checked against cargo metadata's test targets of the named --test-package set.
"""
import json, re, sys

rd = sys.argv[1]
JOBS = 6
recs = [r['record'] for r in json.load(open(f'{rd}/ledger.json', encoding='utf-8'))]
recs = [json.loads(r) if isinstance(r, str) else r for r in recs]
meta = json.load(open(f'{rd}/meta.json', encoding='utf-8'))
tests = {}
for p in meta['packages']:
    tests[p['name']] = {t['name'] for t in p['targets'] if 'test' in t['kind']}
UNITS = ['escher-driver', 'blitz-paint', 'wpt', 'seven_guis', 'browser', 'wgpu_texture', 'dioxus-native', 'wasm_hello', 'blitz-dom']
BLITZ_DOM_EXCLUDE = r'replace (BaseDocument::scroll_state|node_scroll_range) -> '
out = {}
for u in UNITS:
    src = None
    for r in reversed(recs):
        if (r.get('mutation') or {}).get('unit_states', {}).get(u, '').startswith('complete'):
            src = r
            break
    if src and u != 'seven_guis':
        form = src['commands']['mutation:' + u].split(' [')[0].split(' ; ')[0]
        origin = f"recorded form of {src['epoch']} ({src.get('version') or src['ts']})"
    elif u == 'seven_guis':
        # newest `complete` record (0.2.0 Epoch 1) holds a hand pass + a --file scoped run at -j 2 under a core pin;
        # its mutation invocation is the recorded form, its scope (--file) and pin not carried.
        f = src['commands']['mutation:seven_guis'].split(' · ')[1]
        f = f.split('&& ', 1)[1]
        form = f
        origin = f"recorded form of {src['epoch']} ({src['ts']}), the mutation invocation after its hand pass"
    else:
        form = f'NEXTEST_PROFILE=mutants TMPDIR=target/mutants-tmp cargo mutants -p {u} --in-diff X --test-tool nextest -j 6 --timeout 600 --output X'
        origin = 'no record holds the unit complete: form built here'
    # strip carried scope / run-specific flags, then re-resolve
    form = re.sub(r'TMPDIR=\S+', 'TMPDIR=target/mutants-tmp', form)
    form = re.sub(r'CARGO_BUILD_JOBS=\S+ ', '', form)
    form = re.sub(r'timeout -s INT \d+ ', '', form)
    form = re.sub(r'taskset -c \S+ ', '', form)
    form = re.sub(r' --file \S+', '', form)
    form = re.sub(r' --in-diff \S+', '', form)
    form = re.sub(r' --output \S+', '', form)
    form = re.sub(r' -j \d+', '', form)
    form = re.sub(r' --iterate', '', form)
    if 'TMPDIR=' not in form:
        form = 'TMPDIR=target/mutants-tmp ' + form
    form = form.replace(f'cargo mutants -p {u}', f'cargo mutants -p {u} --in-diff {rd}/epoch.diff', 1)
    if u == 'blitz-dom':
        form += f" --exclude-re '{BLITZ_DOM_EXCLUDE}'"
    form += f' -j {JOBS} --output {rd}/mutants-{u}'
    if '--timeout' not in form:
        form += ' --timeout 600'
    pk = re.search(r'--test-package=(\S+)', form)
    pkgs = pk.group(1).split(',') if pk else [u]
    want = re.findall(r'--cargo-test-arg=--test=(\S+)', form)
    have = set().union(*(tests.get(p, set()) for p in pkgs))
    missing = [t for t in want if t not in have]
    new = sorted(have - set(want)) if want else []
    out[u] = {'form': form, 'origin': origin, 'packages': pkgs, 'targets': want, 'missing_targets': missing,
              'unlisted_targets_of_packages': new, 'baseline_skip': '--baseline=skip' in form}
    print(u, '|', origin, '| targets', len(want), 'missing', missing, '| unlisted', len(new), '| baseline=skip', '--baseline=skip' in form)
json.dump(out, open(f'{rd}/forms.json', 'w', encoding='utf-8'), indent=1)
