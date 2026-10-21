"""Mutation tier driver (collectors.md C1), units run sequentially.

usage: mutate.py {run_dir}
Per unit: skipped when {run_dir}/c-mutation-{unit}.json exists (resume by artifact presence);
its set's hand pass must read exit 0 in handpass.json (else baseline-test-failure). Then
cargo mutants on nextest's `mutants` profile with --baseline=skip, -j 2, --timeout 600,
CARGO_BUILD_JOBS=4, taskset -c 12-15,28-31, TMPDIR={repo}/target/mutants-tmp, output
{run_dir}/mutants-{unit}. Cap = max(1800, planned x per-mutant s x 1.5); a unit with no recorded
figure starts at the 1800 s floor and is re-sized once from its own live rate. Completion only by
outcomes.json `end_time` set AND `total_mutants` == len(mutants.json). Per unit the summary lands
in c-mutation-{unit}.json, with host-excluded missed mutants classed by sum_mutation.cover().
"""
import json, os, signal, subprocess, sys, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from sum_mutation import summarize  # noqa: E402

rd = sys.argv[1]
repo = os.getcwd()
DIFF = f'{rd}/window.diff'
UNITS = [
    # unit, scope, scope args, test set, planned, per-mutant s (baseline record's timing) or None
    ('dioxus-native-dom', 'files', ['packages/dioxus-native-dom/src/dioxus_document.rs',
                                     'packages/dioxus-native-dom/src/lib.rs',
                                     'packages/dioxus-native-dom/src/snapshot.rs',
                                     'packages/dioxus-native-dom/src/snapshot_text.rs'], 'broad', 83, 1623 / 39),
    ('blitz-traits', 'in-diff', None, 'broad', 9, None),
    ('blitz-vibey-script', 'in-diff', None, 'broad', 2, None),
    ('escher-driver', 'in-diff', None, 'driver', 207, 5268 / 119),
    ('blitz-dom', 'in-diff', None, 'broad', 180, 915 / 15),
]
FLOOR = 1800
sets = json.load(open(f'{rd}/testsets.json', encoding='utf-8'))
hand = json.load(open(f'{rd}/handpass.json', encoding='utf-8'))
env = dict(os.environ, TMPDIR=f'{repo}/target/mutants-tmp', CARGO_BUILD_JOBS='4', NEXTEST_PROFILE='mutants')
os.makedirs(f'{repo}/target/mutants-tmp', exist_ok=True)


def read_json(p):
    try:
        return json.load(open(p, encoding='utf-8'))
    except (OSError, ValueError):
        return None


for unit, scope, files, tset, planned, rate in UNITS:
    art = f'{rd}/c-mutation-{unit}.json'
    if os.path.exists(art):
        print(unit, 'already summarized, skipped', flush=True)
        continue
    s = sets[tset]
    scope_args = ([a for f in files for a in ('--file', f)] if scope == 'files' else ['--in-diff', DIFF])
    out = f'{rd}/mutants-{unit}'
    cmd = (['taskset', '-c', '12-15,28-31', 'cargo', 'mutants', '-p', unit] + scope_args
           + ['--test-tool', 'nextest', '--test-package=' + ','.join(s['pkgs']), '--cargo-test-arg=--lib']
           + [f'--cargo-test-arg=--test={t}' for t in s['tests']]
           + ['--baseline=skip', '-j', '2', '--timeout', '600', '--output', out])
    printed = ('TMPDIR=$PWD/target/mutants-tmp CARGO_BUILD_JOBS=4 NEXTEST_PROFILE=mutants ' + ' '.join(cmd))
    base = {'unit': unit, 'scope': scope, 'files': files, 'test_set': tset, 'planned': planned,
            'hand_pass': hand[tset], 'command': printed}
    if hand[tset]['exit'] != 0:
        json.dump(dict(base, state='baseline-test-failure'), open(art, 'w', encoding='utf-8'), indent=1)
        print(unit, 'baseline-test-failure (hand pass red)', flush=True)
        continue
    cap = max(FLOOR, planned * rate * 1.5) if rate else FLOOR
    resized = rate is not None
    t0 = time.time()
    with open(f'{rd}/mutants-{unit}.log', 'w', encoding='utf-8') as log:
        p = subprocess.Popen(cmd, env=env, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
        stopped = False
        while p.poll() is None:
            time.sleep(30)
            el = time.time() - t0
            if el <= cap:
                continue
            oc = read_json(f'{out}/mutants.out/outcomes.json') or {}
            tested = sum(1 for o in oc.get('outcomes', []) if o.get('scenario') != 'Baseline')
            if not resized:
                resized = True
                if tested:
                    cap = max(FLOOR, planned * (el / tested) * 1.5)
                    print(unit, f'cap re-sized from own rate: {round(cap)} s', flush=True)
                    continue
            os.killpg(p.pid, signal.SIGINT)
            try:
                p.wait(120)
            except subprocess.TimeoutExpired:
                os.killpg(p.pid, signal.SIGKILL)
                p.wait()
            stopped = True
    base.update(cap_s=round(cap), exit=p.returncode, elapsed_s=round(time.time() - t0))
    res = summarize(repo, f'{out}/mutants.out', base, stopped)
    json.dump(res, open(art, 'w', encoding='utf-8'), indent=1)
    print(unit, res['state'], res.get('counts'), res.get('score'), flush=True)
print('ALL DONE', flush=True)
