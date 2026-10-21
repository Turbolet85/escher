"""The unmutated pass, by hand, per test set (collectors.md C1: tests outside the mutated package).

usage: handpass.py {run_dir}
Runs each set of {run_dir}/testsets.json through nextest on the `mutants` profile, under the
mutation footprint, and records {set: {command, exit, wall_s}} in {run_dir}/handpass.json.
Output goes to {run_dir}/handpass-{set}.log (deleted after summarization).
"""
import json, os, subprocess, sys, time

rd = sys.argv[1]
sets = json.load(open(f'{rd}/testsets.json', encoding='utf-8'))
env = dict(os.environ, CARGO_BUILD_JOBS='4', NEXTEST_PROFILE='mutants')
res = {}
for name, s in sets.items():
    cmd = (['taskset', '-c', '12-15,28-31', 'cargo', 'nextest', 'run', '--locked']
           + [a for p in s['pkgs'] for a in ('-p', p)] + ['--lib'] + [f'--test={t}' for t in s['tests']])
    t0 = time.time()
    with open(f'{rd}/handpass-{name}.log', 'w', encoding='utf-8') as log:
        rc = subprocess.run(cmd, env=env, stdout=log, stderr=subprocess.STDOUT).returncode
    res[name] = {'command': 'CARGO_BUILD_JOBS=4 NEXTEST_PROFILE=mutants ' + ' '.join(cmd),
                 'exit': rc, 'wall_s': round(time.time() - t0)}
    print(name, rc, res[name]['wall_s'], flush=True)
json.dump(res, open(f'{rd}/handpass.json', 'w', encoding='utf-8'), indent=1)
