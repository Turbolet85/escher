"""Mutation tier driver (C1), sequential per unit, the founder's footprint on every invocation.

usage: mutate.py RUN_DIR UNIT [UNIT ...]   (run from the repo root)
Per unit, from {run_dir}/testsets.json and {run_dir}/mutation-scope.json:
 - a unit whose tests live outside it: the same test set run by hand on the unmutated tree
   ({run_dir}/hand-{unit}.json: command, exit, wall_s); red -> baseline-test-failure, no mutants;
   green -> cargo mutants ... --baseline=skip
 - a unit with its own tests (no target list): the tool's own unmutated baseline
 - cap: a unit with a recorded figure keeps its cap_s; without one it starts at the 1800 s floor and,
   still incomplete there, is re-sized ONCE: planned x (elapsed / tested so far) x 1.5; none tested ->
   budget-exhausted. A cap stop is SIGINT to the tool's process group.
Writes {run_dir}/run-{unit}.json; the tool's output under {run_dir}/mutants-{unit}/mutants.out/.
"""
import json, os, signal, subprocess, sys, time

rd, units = sys.argv[1], sys.argv[2:]
repo = os.getcwd()
ts = json.load(open(f'{rd}/testsets.json', encoding='utf-8'))
scope = json.load(open(f'{rd}/mutation-scope.json', encoding='utf-8'))['units']
TASKSET = ['taskset', '-c', '12-15,28-31']
ENV_HAND = {'CARGO_BUILD_JOBS': '4', 'NEXTEST_PROFILE': 'mutants'}
tmp = os.path.join(repo, 'target', 'mutants-tmp')
os.makedirs(tmp, exist_ok=True)
ENV_MUT = dict(ENV_HAND, TMPDIR=tmp)
FLOOR = 1800


def envfmt(e, shown_tmp='target/mutants-tmp'):
    return ' '.join(f'{k}={shown_tmp if k == "TMPDIR" else v}' for k, v in e.items())


def read_outcomes(out):
    p = f'{out}/mutants.out/outcomes.json'
    try:
        return json.load(open(p, encoding='utf-8'))
    except (OSError, ValueError):
        return None


def log(msg):
    print(time.strftime('%H:%M:%S'), msg, flush=True)


for u in units:
    t, sc = ts[u], scope[u]
    rec = {'unit': u, 'scope': sc['scope'], 'planned': sc['planned']}
    test_args = []
    if t['targets']:
        hand = TASKSET + ['cargo', 'nextest', 'run', '--locked'] + sum((['-p', p] for p in t['pkgs']), []) + \
            ['--lib'] + [f'--test={x}' for x in t['targets']]
        log(f'{u}: hand pass')
        t0 = time.time()
        with open(f'{rd}/hand-{u}.log', 'w') as lf:
            rc = subprocess.run(hand, env=dict(os.environ, **ENV_HAND), stdout=lf, stderr=subprocess.STDOUT).returncode
        json.dump({'command': f'{envfmt(ENV_HAND)} ' + ' '.join(hand), 'exit': rc, 'wall_s': round(time.time() - t0)},
                  open(f'{rd}/hand-{u}.json', 'w'), indent=1)
        log(f'{u}: hand pass exit {rc}')
        if rc != 0:
            rec['state'] = 'baseline-test-failure'
            json.dump(rec, open(f'{rd}/run-{u}.json', 'w'), indent=1)
            continue
        test_args = [f'--test-package={",".join(t["pkgs"])}', '--cargo-test-arg=--lib'] + \
            [f'--cargo-test-arg=--test={x}' for x in t['targets']] + ['--baseline=skip']
    out = f'{rd}/mutants-{u}'
    cmd = TASKSET + ['cargo', 'mutants', '-p', u, '--in-diff', f'{rd}/window.diff', '--test-tool', 'nextest'] + \
        test_args + ['-j', '2', '--timeout', '600', '--output', out]
    rec['command'] = f'{envfmt(ENV_MUT)} ' + ' '.join(cmd)
    cap = sc['cap_s']
    resizable = 'floor' in sc['cap_basis']
    log(f'{u}: cargo mutants, cap {cap}s')
    start = time.time()
    with open(f'{rd}/mutants-{u}.log', 'w') as lf:
        proc = subprocess.Popen(cmd, env=dict(os.environ, **ENV_MUT), stdout=lf, stderr=subprocess.STDOUT,
                                start_new_session=True)
        stopped = None
        while proc.poll() is None:
            time.sleep(30)
            el = time.time() - start
            if el < cap or proc.poll() is not None:
                continue
            o = read_outcomes(out)
            tested = max(0, len((o or {}).get('outcomes', [])) - (0 if test_args else 1))
            if resizable:
                resizable = False
                if tested == 0:
                    stopped = 'budget-exhausted (none tested at the floor)'
                else:
                    cap = max(cap, round(sc['planned'] * el / tested * 1.5))
                    rec['cap_resized'] = {'at_s': round(el), 'tested': tested, 'cap_s': cap}
                    log(f'{u}: cap re-sized to {cap}s ({tested} tested at {round(el)}s)')
                    continue
            else:
                stopped = f'budget-exhausted at {round(el)}s, {tested} tested'
            log(f'{u}: {stopped}; SIGINT')
            os.killpg(proc.pid, signal.SIGINT)
            try:
                proc.wait(120)
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid, signal.SIGKILL)
                proc.wait()
            break
    rec.update({'exit': proc.returncode, 'wall_s': round(time.time() - start), 'cap_s': cap,
                'stopped': stopped})
    json.dump(rec, open(f'{rd}/run-{u}.json', 'w'), indent=1)
    log(f'{u}: exit {proc.returncode} after {rec["wall_s"]}s')
log('all units done')
