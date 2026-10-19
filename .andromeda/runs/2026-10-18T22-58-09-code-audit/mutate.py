"""Mutation tier driver (C1) — sequential, resumable by artifact presence (c-mutation-{unit}.json).

usage: mutate.py RUN_DIR
Per unit: the unmutated hand pass of its test set (once per set), then cargo mutants on nextest's
`mutants` profile with --baseline=skip under the confirmed footprint (jobs 2, CARGO_BUILD_JOBS=4,
taskset -c 12-15,28-31), polled every 20 s. The cap is enforced by the poll (SIGINT to the process group);
a unit with no recorded rate starts at the 1800 s floor and is re-sized ONCE there from its own rate
(elapsed / tested so far x planned x 1.5). A tally is read only from a COMPLETE invocation.
"""
import json, os, signal, subprocess, sys, time

rd = sys.argv[1]
ts = json.load(open(f'{rd}/testsets.json', encoding='utf-8'))
FOOT = ['taskset', '-c', '12-15,28-31']
ENV = dict(os.environ, CARGO_BUILD_JOBS='4', NEXTEST_PROFILE='mutants',
           TMPDIR=os.path.join(os.getcwd(), 'target', 'mutants-tmp'))
DRV = ['packages/escher-driver/src/' + f for f in
       ('describe.rs', 'execute.rs', 'json.rs', 'lib.rs', 'refusal.rs', 'schema.rs', 'schema_tests.rs', 'session.rs')]
UNITS = [  # name, package, scope flags, scope label, planned, test set, cap_s, rate known
    ('seven_guis', 'seven_guis', ['--file', 'examples/seven_guis/src/stand.rs'], 'files', 22, 'seven_guis', 1800, True),
    ('blitz-test-harness', 'blitz-test-harness', ['--file', 'packages/blitz-test-harness/src/harness.rs',
                                                  '--file', 'packages/blitz-test-harness/src/lib.rs'],
     'files', 32, 'broad', 1800, False),
    ('dioxus-native-dom', 'dioxus-native-dom', ['--file', 'packages/dioxus-native-dom/src/snapshot.rs'],
     'files', 34, 'broad', 1800, False),
    ('blitz-dom/clock', 'blitz-dom', ['--file', 'packages/blitz-dom/src/clock.rs'], 'files', 3, 'broad', 1800, True),
    ('escher-driver', 'escher-driver', sum((['--file', f] for f in DRV), []) + ['--shard', '0/2'],
     'shard 0/2', 97, 'escher-driver', 5350, True),
    ('blitz-dom/engine-debts', 'blitz-dom', ['--in-diff', f'{rd}/engine-debts-at-head.diff'],
     'in-diff', 153, 'broad', 8824, True),
]


def log(*a):
    with open(f'{rd}/mutate.log', 'a', encoding='utf-8') as fh:
        fh.write(time.strftime('%Y-%m-%dT%H:%M:%SZ ', time.gmtime()) + ' '.join(map(str, a)) + '\n')


def hand_pass(key):
    p = f'{rd}/handpass-{key}.json'
    if os.path.isfile(p):
        return json.load(open(p))['exit'] == 0
    s = ts[key]
    cmd = FOOT + ['cargo', 'nextest', 'run', '--locked'] + sum((['-p', x] for x in s['packages']), []) + \
        ['--lib'] + [f'--test={t}' for t in s['tests']]
    t0 = time.time()
    r = subprocess.run(cmd, env=ENV, capture_output=True, text=True)
    tail = [l for l in (r.stdout + r.stderr).splitlines() if 'Summary' in l or 'FAIL' in l][-5:]
    json.dump({'exit': r.returncode, 'wall_s': round(time.time() - t0), 'summary': tail,
               'command': 'CARGO_BUILD_JOBS=4 NEXTEST_PROFILE=mutants ' + ' '.join(cmd)},
              open(p, 'w'), indent=1)
    log('hand pass', key, 'exit', r.returncode, tail)
    return r.returncode == 0


def tested_so_far(out):
    try:
        oc = json.load(open(f'{out}/mutants.out/outcomes.json'))
        return sum(1 for o in oc['outcomes'] if isinstance(o['scenario'], dict)), oc
    except (OSError, ValueError, KeyError):
        return 0, None


os.makedirs(ENV['TMPDIR'], exist_ok=True)
for name, pkg, scope, label, planned, key, cap, known in UNITS:
    slug = name.replace('/', '_')
    art = f'{rd}/c-mutation-{slug}.json'
    if os.path.isfile(art):
        log('skip (artifact present)', name)
        continue
    if not hand_pass(key):
        json.dump({'unit': name, 'state': 'baseline-test-failure'}, open(art, 'w'), indent=1)
        continue
    s = ts[key]
    out = f'{rd}/mutants-{slug}'
    cmd = FOOT + ['cargo', 'mutants', '-p', pkg] + scope + ['--test-tool', 'nextest',
                                                          '--test-package=' + ','.join(s['packages']),
                                                          '--cargo-test-arg=--lib'] + \
        [f'--cargo-test-arg=--test={t}' for t in s['tests']] + \
        ['--baseline=skip', '--timeout', '600', '-j', '2', '--output', out]
    with open(f'{rd}/cmd-{slug}.txt', 'w') as fh:
        fh.write('TMPDIR=$PWD/target/mutants-tmp CARGO_BUILD_JOBS=4 NEXTEST_PROFILE=mutants ' + ' '.join(cmd) + '\n')
    t0 = time.time()
    proc = subprocess.Popen(cmd, env=ENV, stdout=open(f'{rd}/mutants-{slug}.stdout', 'w'),
                            stderr=subprocess.STDOUT, start_new_session=True)
    resized, stopped = known, False
    log('start', name, 'planned', planned, 'cap', cap)
    while proc.poll() is None:
        time.sleep(20)
        el = time.time() - t0
        if el > cap:
            n, _ = tested_so_far(out)
            if not resized and n > 0:
                new = max(cap, int(el / n * planned * 1.5))
                log('re-size', name, 'tested', n, 'elapsed', int(el), 'cap', cap, '->', new)
                cap, resized = new, True
                continue
            log('cap stop', name, 'tested', n, 'elapsed', int(el))
            os.killpg(proc.pid, signal.SIGINT)
            try:
                proc.wait(timeout=300)
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid, signal.SIGKILL)
                proc.wait()
            stopped = True
    wall = round(time.time() - t0)
    n, oc = tested_so_far(out)
    complete = bool(oc and oc.get('end_time')) and os.path.isfile(f'{out}/mutants.out/mutants.json') and \
        oc.get('total_mutants') == len(json.load(open(f'{out}/mutants.out/mutants.json')))
    timing = {'scope': label, 'planned': planned, 'tested': n, 'wall_s': wall, 'jobs': 2, 'cap_s': cap}
    log('end', name, 'exit', proc.returncode, 'complete', complete, 'stopped', stopped, json.dumps(timing))
    if complete:
        subprocess.run([sys.executable, f'{rd}/sum_mutation.py', out, name, art, 'complete'], check=True)
        res = json.load(open(art))
        res['timing'] = timing
        res['exit'] = proc.returncode
        json.dump(res, open(art, 'w'), indent=1)
    elif stopped:
        json.dump({'unit': name, 'state': 'budget-exhausted', 'tested': n, 'timing': timing}, open(art, 'w'), indent=1)
    else:
        log('INCOMPLETE EXIT — surfaced, no artifact', name, proc.returncode)
log('driver done')
