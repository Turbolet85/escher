"""Mutation tier (C1) driver — sequential, resumable by artifact presence.

usage: mutate.py RUN_DIR
Per test set: the unmutated pass by hand (nextest, the `mutants` profile, same footprint), its
command and exit recorded in RUN_DIR/handpass-{set}.json. Per unit: skipped when
RUN_DIR/c-mutation-{unit}.json exists; else cargo mutants --baseline=skip under the confirmed
footprint (-j 2, CARGO_BUILD_JOBS=4, taskset -c 12-15,28-31, --timeout 600), its own process
group, polled every 30 s against the unit's wall-clock cap (SIGINT to the group at the cap ->
budget-exhausted). Completion is read by the tool's own markers only (sum_mutation.py).
"""
import json
import os
import signal
import subprocess
import sys
import time

rd = sys.argv[1]
repo = os.getcwd()
tmp = os.path.join(repo, 'target', 'mutants-tmp')
os.makedirs(tmp, exist_ok=True)

BASE_SETS = json.load(open(os.path.join(rd, 'testsets.json'), encoding='utf-8'))
ENV = dict(os.environ, CARGO_BUILD_JOBS='4', NEXTEST_PROFILE='mutants', TMPDIR=tmp)
TASKSET = ['taskset', '-c', '12-15,28-31']

UNITS = [
    {'unit': 'dioxus-native-dom', 'set': 'broad', 'pkg': 'dioxus-native-dom',
     'scope_args': ['--file', 'packages/dioxus-native-dom/src/lib.rs',
                    '--file', 'packages/dioxus-native-dom/src/snapshot.rs'], 'cap_s': 2374},
    {'unit': 'blitz-dom', 'set': 'broad', 'pkg': 'blitz-dom',
     'scope_args': ['--file', 'packages/blitz-dom/src/iframe.rs'], 'cap_s': 1800},
    {'unit': 'escher-driver', 'set': 'escher-driver', 'pkg': 'escher-driver',
     'scope_args': [a for f in ['cli', 'client', 'command', 'describe', 'error', 'execute', 'host',
                                'json', 'lib', 'mcp', 'mcp/result', 'mcp_tests', 'recording',
                                'replay', 'session', 'wire']
                    for a in ('--file', f'packages/escher-driver/src/{f}.rs')]
     + ['--shard', '3/5'], 'cap_s': 6419},
]


def log(msg):
    with open(os.path.join(rd, 'mutate.log'), 'a', encoding='utf-8') as f:
        f.write(time.strftime('%Y-%m-%dT%H:%M:%SZ ', time.gmtime()) + msg + '\n')


def hand_pass(name):
    out = os.path.join(rd, f'handpass-{name}.json')
    if os.path.isfile(out):
        return json.load(open(out, encoding='utf-8'))['exit']
    s = BASE_SETS[name]
    cmd = TASKSET + ['cargo', 'nextest', 'run', '--locked'] + [a for p in s['packages'] for a in ('-p', p)] \
        + ['--lib'] + [f'--test={t}' for t in s['tests']]
    t0 = time.time()
    with open(os.path.join(rd, f'handpass-{name}.log'), 'w', encoding='utf-8') as lf:
        rc = subprocess.run(cmd, env=ENV, stdout=lf, stderr=subprocess.STDOUT).returncode
    json.dump({'set': name, 'command': 'CARGO_BUILD_JOBS=4 NEXTEST_PROFILE=mutants ' + ' '.join(cmd),
               'exit': rc, 'wall_s': round(time.time() - t0)}, open(out, 'w', encoding='utf-8'), indent=1)
    log(f'hand pass {name}: exit {rc}')
    return rc


for u in UNITS:
    name = u['unit']
    if os.path.isfile(os.path.join(rd, f'c-mutation-{name}.json')):
        log(f'{name}: artifact present, skipped')
        continue
    rc = hand_pass(u['set'])
    s = BASE_SETS[u['set']]
    outdir = os.path.join(rd, f'mutants-{name}')
    cmd = TASKSET + ['cargo', 'mutants', '-p', u['pkg']] + u['scope_args'] + [
        '--test-tool', 'nextest', '--test-package=' + ','.join(s['packages']),
        '--cargo-test-arg=--lib'] + [f'--cargo-test-arg=--test={t}' for t in s['tests']] + [
        '--baseline=skip', '--timeout', '600', '-j', '2', '--output', os.path.relpath(outdir, repo)]
    json.dump({'unit': name, 'command': 'TMPDIR=$PWD/target/mutants-tmp CARGO_BUILD_JOBS=4 '
               'NEXTEST_PROFILE=mutants ' + ' '.join(cmd), 'cap_s': u['cap_s'], 'handpass_exit': rc},
              open(os.path.join(rd, f'cmd-mutation-{name}.json'), 'w', encoding='utf-8'), indent=1)
    if rc != 0:
        log(f'{name}: hand pass red ({rc}) -> baseline-test-failure, not run')
        subprocess.run([sys.executable, os.path.join(rd, 'sum_mutation.py'), rd, name, 'baseline-test-failure'])
        continue
    log(f'{name}: start, cap {u["cap_s"]} s')
    t0 = time.time()
    with open(os.path.join(rd, f'mutants-{name}.log'), 'w', encoding='utf-8') as lf:
        p = subprocess.Popen(cmd, env=ENV, stdout=lf, stderr=subprocess.STDOUT, start_new_session=True)
        state = None
        while p.poll() is None:
            if time.time() - t0 > u['cap_s']:
                os.killpg(p.pid, signal.SIGINT)
                try:
                    p.wait(timeout=120)
                except subprocess.TimeoutExpired:
                    os.killpg(p.pid, signal.SIGKILL)
                    p.wait()
                state = 'budget-exhausted'
                break
            time.sleep(30)
    wall = round(time.time() - t0)
    log(f'{name}: exit {p.returncode} after {wall} s{" (cap)" if state else ""}')
    args = [sys.executable, os.path.join(rd, 'sum_mutation.py'), rd, name, state or 'read', str(wall), str(u['cap_s'])]
    subprocess.run(args)
log('done')
