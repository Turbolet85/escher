"""Sequential mutation-tier driver. usage: python3 mutate.py {run_dir} {unit}...
Per unit: a form with --baseline=skip first gets its hand pass on the unmutated tree (the same packages and test set by
cargo nextest run under the mutants profile; command and exit written to {run_dir}/handpass-{unit}.txt; red -> the unit
is baseline-test-failure and no mutant runs). Then the firing form from forms.json, in its own process group.
Cap: floor 1800 s; still incomplete there, re-sized ONCE from its own rate (elapsed / tested so far x planned x 1.5);
none tested by then -> stopped at the floor, budget-exhausted. A cap stop is SIGINT to the group.
Completion is the tool's own markers: outcomes.json end_time set and total_mutants == len(mutants.json).
Writes {run_dir}/state-{unit}.json (start, end, cap_s, stop reason, exit) and appends to {run_dir}/mutate.log.
"""
import json, os, re, signal, subprocess, sys, time

rd = sys.argv[1]
units = sys.argv[2:]
forms = json.load(open(f'{rd}/forms.json', encoding='utf-8'))
FLOOR = 1800


def log(msg):
    with open(f'{rd}/mutate.log', 'a', encoding='utf-8') as fh:
        fh.write(time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()) + ' ' + msg + '\n')


def outcomes(u):
    p = f'{rd}/mutants-{u}/mutants.out/outcomes.json'
    try:
        return json.load(open(p, encoding='utf-8'))
    except (OSError, ValueError):
        return None


def planned_n(u):
    try:
        return len(json.load(open(f'{rd}/mutants-{u}/mutants.out/mutants.json', encoding='utf-8')))
    except (OSError, ValueError):
        return None


for u in units:
    v = forms[u]
    st = {'unit': u}
    if os.path.exists(f'{rd}/state-{u}.json') and json.load(open(f'{rd}/state-{u}.json')).get('done'):
        log(f'{u}: state present, skipped (resume by artifact presence)')
        continue
    if v['baseline_skip']:
        env = 'NEXTEST_PROFILE=mutants '
        pk = ' '.join(f'-p {p}' for p in v['packages'])
        tg = ' '.join(f'--test={t}' for t in v['targets'])
        hp = f'{env}cargo nextest run --locked {pk} --lib {tg}'
        t0 = time.time()
        r = subprocess.run(hp + f' > {rd}/handpass-{u}.out 2>&1', shell=True)
        open(f'{rd}/handpass-{u}.txt', 'w').write(f'{hp}\nexit={r.returncode} wall_s={round(time.time() - t0)}\n')
        log(f'{u}: hand pass exit {r.returncode}')
        if r.returncode != 0:
            st.update(done=True, state='baseline-test-failure', handpass_exit=r.returncode)
            json.dump(st, open(f'{rd}/state-{u}.json', 'w'), indent=1)
            continue
    start = time.time()
    p = subprocess.Popen(v['form'] + f' > {rd}/mutants-{u}.log 2>&1', shell=True, start_new_session=True)
    cap, resized, reason = FLOOR, False, None
    log(f'{u}: started pid {p.pid}')
    while p.poll() is None:
        time.sleep(20)
        el = time.time() - start
        if el >= cap:
            o = outcomes(u)
            tested = sum(1 for x in (o or {}).get('outcomes', []) if x.get('scenario') != 'Baseline')
            if not resized and tested > 0:
                pl = planned_n(u) or 0
                cap = max(FLOOR, round(pl * (el / tested) * 1.5))
                resized = True
                log(f'{u}: floor reached at {tested} tested of {pl}; cap re-sized once to {cap} s')
                if el < cap:
                    continue
            reason = 'cap'
            log(f'{u}: cap {cap} s spent, SIGINT to group')
            os.killpg(p.pid, signal.SIGINT)
            try:
                p.wait(timeout=300)
            except subprocess.TimeoutExpired:
                os.killpg(p.pid, signal.SIGKILL)
                p.wait()
            break
    end = time.time()
    st.update(done=True, start=round(start), end=round(end), cap_s=cap, resized=resized, stop=reason, exit=p.returncode)
    json.dump(st, open(f'{rd}/state-{u}.json', 'w'), indent=1)
    log(f'{u}: ended exit {p.returncode} wall {round(end - start)} s stop {reason}')
log('tier done')
