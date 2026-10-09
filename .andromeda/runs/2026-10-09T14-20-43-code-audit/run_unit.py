"""Tier C driver: one scoped unit at a time.

  run_unit.py hand <key>           the unmutated pass of the unit's test set, by hand (its exit recorded)
  run_unit.py mutants <key>        cargo mutants for the unit, polled under the cap rules (floor, one re-size)
  run_unit.py watch <key> <cap_s>  re-attach to a still-running invocation under an absolute cap
  run_unit.py cmd <key>            print the firing forms

The stop is the poll's own (no fixed timeout prefix): a cap is re-sized ONCE at the floor from the unit's
own rate; a re-sized cap over 2 h leaves the invocation running and returns `resize-over-2h` for the
operator's word.
"""
import json
import os
import shlex
import signal
import subprocess
import sys
import time

RUN = '.andromeda/runs/2026-10-09T14-20-43-code-audit'
FLOOR_S = 1800
# Footprint, on the operator's direction (2026-10-09; the host was contended by this run's first launch at
# -j 4 with unbounded cargo jobs: load 209 on 32 cores, IO stalled 465 s, 35 linkers at once): about a quarter
# of the cores in total, for every unit. 2 mutants in parallel, each cargo bounded to 4 build jobs, the tool's
# jobserver at 6 tasks: with the one implicit token each of the two cargos holds that is at most 8 compiler
# processes (measured at 8 tasks: 10 rustc at once), linkers being their children; 4 test threads per binary.
# The five units finished first ran at that setting with no priority prefix (their status files and twins carry
# it). Both blitz-dom units run at the same footprint - 2 mutants, each cargo bounded to 4 build jobs, the
# jobserver at 6 tasks (at most 8 compiler processes), 4 test threads - with the whole invocation at the lowest
# CPU and IO priority: the founder's word relayed by the operator (2026-10-09; the host is in interactive use
# meanwhile). A one-mutant economy launch between the two was stopped on his correction and discarded.
JOBS = 2
FOOTPRINT = {'CARGO_BUILD_JOBS': '4', 'RUST_TEST_THREADS': '4'}
JOBSERVER_TASKS = 6
# The pin, the operator's (2026-10-09T17:28Z): the running scrolling tree was pinned by him to CPUs 12-15,28-31
# with taskset (the linkers held the host near 80 % CPU), and the changed-set unit and its unmutated pass launch
# under the same pin.
PIN = '12-15,28-31'
PRIORITY = ['taskset', '-c', PIN, 'nice', '-n', '19', 'ionice', '-c', '3']
# Time is not the constraint and a timeout taken under contention would misread a mutant: the test timeout is
# far above the unmutated run (17 s of summed test time at the first hand pass).
TIMEOUT_S = 600
ENV_PREFIX = ' '.join(f'{k}={v}' for k, v in FOOTPRINT.items())

ACT = ['stand_act_diff', 'stand_act_disabled', 'stand_act_ids', 'stand_act_keys', 'stand_act_obstructed',
       'stand_act_range', 'stand_act_refused', 'stand_act_scroll', 'stand_act_spans', 'stand_act_timer']
SESSION = ['stand_session_fresh', 'stand_session_ids', 'stand_session_lifecycle', 'stand_session_quiet',
           'stand_session_state']
DND = 'packages/dioxus-native-dom/src/'

UNITS = {
    'escher-driver': {
        'unit': 'escher-driver', 'scope': 'unit', 'select': ['-p', 'escher-driver'],
        'test_packages': ['escher-driver', 'blitz-tests', 'seven_guis'],
        'tests': ACT + SESSION + ['stand_settle', 'telemetry_scrub', 'host_binary', 'host_log'],
    },
    'escher-telemetry': {
        'unit': 'escher-telemetry', 'scope': 'unit', 'select': ['-p', 'escher-telemetry'],
        'test_packages': ['escher-telemetry', 'blitz-tests', 'seven_guis'],
        'tests': ['stand_act_spans', 'telemetry_drop', 'telemetry_init_idempotent', 'telemetry_panic_hook',
                  'telemetry_scrub', 'telemetry_stdout_silent', 'host_binary', 'host_log'],
    },
    'dioxus-native-dom': {
        'unit': 'dioxus-native-dom', 'scope': 'files',
        'select': ['--workspace', '--file', DND + 'element_id.rs', '--file', DND + 'dioxus_document.rs'],
        'test_packages': ['dioxus-native-dom', 'blitz-tests'],
        'tests': ['stand_element_ids', 'stand_id_persistence', 'stand_accessibility_ids', 'accessibility_names',
                  'stand_id_edits', 'stand_actionable_keys', 'stand_snapshot'],
    },
    'blitz-test-harness': {
        'unit': 'blitz-test-harness', 'scope': 'files',
        'select': ['--workspace', '--file', 'packages/blitz-test-harness/src/settle.rs'],
        'test_packages': ['blitz-test-harness', 'blitz-tests', 'escher-driver'],
        'tests': ACT + SESSION + ['stand_settle'],
    },
    'dioxus-native-dom-snapshot': {
        'unit': 'dioxus-native-dom', 'scope': 'files',
        'select': ['--workspace', '--file', DND + 'snapshot.rs', '--file', DND + 'snapshot_text.rs',
                   '--file', DND + 'snapshot_diff.rs', '--file', DND + 'actionable.rs'],
        'test_packages': ['dioxus-native-dom', 'blitz-tests'],
        'tests': ['stand_snapshot', 'stand_snapshot_state', 'stand_snapshot_text', 'stand_diff',
                  'stand_actionable_keys', 'stand_act_diff'],
    },
}
BD = 'packages/blitz-dom/src/'
# blitz-dom sits under every crate: its test set is every test target of the engine crate, the integration
# package and the Dioxus bridge (`--tests`: lib and integration tests, no doctests), never a hand-picked list.
BD_TESTS = {'test_packages': ['blitz-dom', 'blitz-tests', 'dioxus-native-dom'], 'tests': [], 'test_args': ['--tests']}
# Both blitz-dom units are function-filtered on the operator's word (2026-10-09, at the blitz-dom confirm): the
# four scrolling functions Epoch 4 changed, and the changed-set functions. Each filter is built by build_filter.py
# from the tool's own listing and proven against it; the field-deletion mutants no name filter reaches ride along.
for _key, _which in (('blitz-dom-scrolling', 'scrolling-touched'), ('blitz-dom-changed-set', 'changed-set')):
    try:
        _f = json.load(open(f'{RUN}/mutation-filter-blitz-dom-{_which}.json', encoding='utf-8'))
    except OSError:
        continue
    UNITS[_key] = dict(BD_TESTS, unit='blitz-dom', scope='filter',
                       select=['--workspace'] + [a for f in _f['files'] for a in ('--file', f)]
                       + [a for r in _f['regexes'] for a in ('-F', r)])


def test_args(cfg):
    return cfg.get('test_args') or ['--lib'] + [f'--test={t}' for t in cfg['tests']]


def hand_cmd(cfg):
    c = ['cargo', 'test', '--locked']
    for p in cfg['test_packages']:
        c += ['-p', p]
    return PRIORITY + c + test_args(cfg)


def mutants_cmd(key, cfg, timeout_s=TIMEOUT_S, jobs=JOBS):
    c = PRIORITY + ['cargo', 'mutants'] + cfg['select'] + ['--test-package=' + ','.join(cfg['test_packages'])]
    c += [f'--cargo-test-arg={a}' for a in test_args(cfg)]
    return c + ['--baseline=skip', '--timeout', str(timeout_s), '-j', str(jobs), '--jobserver-tasks',
                str(JOBSERVER_TASKS), '--output', f'{RUN}/mutants-{key}']


def status_path(key):
    return f'{RUN}/mutation-status-{key}.json'


def write_status(key, obj):
    with open(status_path(key), 'w', encoding='utf-8', newline='') as f:
        json.dump(obj, f, indent=1)
        f.write('\n')


def live(key):
    """(live total_mutants, end_time) from the tool's incrementally rewritten outcomes.json; a rate, never a tally."""
    try:
        o = json.load(open(f'{RUN}/mutants-{key}/mutants.out/outcomes.json', encoding='utf-8'))
        return o.get('total_mutants', 0), o.get('end_time')
    except (OSError, ValueError):
        return 0, None


def planned(key):
    try:
        return len(json.load(open(f'{RUN}/mutants-{key}/mutants.out/mutants.json', encoding='utf-8')))
    except (OSError, ValueError):
        return None


def alive(pid):
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return False
    try:  # a zombie child still answers signal 0
        return open(f'/proc/{pid}/stat').read().rsplit(')', 1)[1].split()[0] != 'Z'
    except OSError:
        return False


def stop(pid):
    try:
        os.killpg(pid, signal.SIGINT)
    except ProcessLookupError:
        return
    for _ in range(60):
        if not alive(pid):
            return
        time.sleep(1)
    try:
        os.killpg(pid, signal.SIGKILL)
    except ProcessLookupError:
        pass


def poll(key, st, proc=None):
    pid, start = st['pid'], st['start_epoch']
    while True:
        if proc is not None:
            rc = proc.poll()
            done = rc is not None
        else:
            rc, done = None, not alive(pid)
        elapsed = time.time() - start
        tested, end = live(key)
        if done:
            st.update(state='exited', returncode=rc, elapsed_s=round(elapsed), live_tested=tested, end_time=end,
                      planned=planned(key))
            break
        if elapsed >= st['cap_s']:
            if tested == 0:
                stop(pid)
                st.update(state='budget-exhausted', elapsed_s=round(elapsed), live_tested=0, planned=planned(key),
                          note='none tested at the cap')
                break
            if not st['resized']:
                n = planned(key) or st.get('planned_listing')
                rate = elapsed / tested
                new_cap = max(FLOOR_S, round(n * rate * 1.5))
                st.update(resized=True, rate_s_per_mutant=round(rate, 2), rate_at={'elapsed_s': round(elapsed),
                          'live_tested': tested}, planned=n, resized_cap_s=new_cap)
                if new_cap > 7200:
                    st['over_2h'] = ('whole under its computed cap - the operator, 2026-10-09, with the footprint '
                                     'direction: time is not the constraint')
                st['cap_s'] = new_cap
                write_status(key, dict(st, state='running'))
                continue
            stop(pid)
            tested, end = live(key)
            st.update(state='budget-exhausted', elapsed_s=round(time.time() - start), live_tested=tested,
                      planned=planned(key))
            break
        time.sleep(15)
    write_status(key, st)
    print(json.dumps(st, indent=1))


mode, key = sys.argv[1], sys.argv[2]
cfg = UNITS[key]
if mode == 'cmd':
    print(ENV_PREFIX + ' ' + shlex.join(hand_cmd(cfg)))
    print(f'mkdir -p target/mutants-tmp && TMPDIR=$PWD/target/mutants-tmp {ENV_PREFIX} '
          + shlex.join(mutants_cmd(key, cfg)))
elif mode == 'hand':
    t0 = time.time()
    with open(f'{RUN}/mutation-hand-{key}.log', 'w', encoding='utf-8') as f:
        rc = subprocess.run(hand_cmd(cfg), stdout=f, stderr=subprocess.STDOUT,
                            env=dict(os.environ, **FOOTPRINT)).returncode
    wall = round(time.time() - t0, 1)
    text = open(f'{RUN}/mutation-hand-{key}.log', encoding='utf-8', errors='replace').read()
    results = [l for l in text.splitlines() if l.startswith('test result:')]
    passed = sum(int(l.split(' passed')[0].rsplit(' ', 1)[1]) for l in results)
    failed = sum(int(l.split(' failed')[0].rsplit(' ', 1)[1]) for l in results)
    ignored = sum(int(l.split(' ignored')[0].rsplit(' ', 1)[1]) for l in results)
    test_s = round(sum(float(l.rsplit('finished in ', 1)[1].rstrip('s')) for l in results if 'finished in' in l), 2)
    obj = {'command': ENV_PREFIX + ' ' + shlex.join(hand_cmd(cfg)), 'exit': rc, 'wall_s': wall,
           'result_lines': len(results),
           'passed': passed, 'failed': failed, 'ignored': ignored, 'summed_test_time_s': test_s}
    with open(f'{RUN}/mutation-hand-{key}.json', 'w', encoding='utf-8', newline='') as f:
        json.dump(obj, f, indent=1)
        f.write('\n')
    print(json.dumps(obj, indent=1))
    if rc != 0:
        print(text[-3000:])
elif mode == 'mutants':
    out = f'{RUN}/mutants-{key}'
    if os.path.exists(out):
        sys.exit(f'{out} exists: a pre-existing output dir is stale by construction - nothing launched')
    hand = json.load(open(f'{RUN}/mutation-hand-{key}.json', encoding='utf-8'))
    if hand['exit'] != 0:
        sys.exit('the hand pass is not green: baseline-test-failure, nothing launched')
    timeout_s = int(sys.argv[3]) if len(sys.argv) > 3 else TIMEOUT_S
    tmp = os.path.join(os.getcwd(), 'target', 'mutants-tmp')
    os.makedirs(tmp, exist_ok=True)
    cmd = mutants_cmd(key, cfg, timeout_s)
    log = open(f'{RUN}/mutants-{key}.stdout.log', 'w', encoding='utf-8')
    proc = subprocess.Popen(cmd, stdout=log, stderr=subprocess.STDOUT,
                            env=dict(os.environ, TMPDIR=tmp, **FOOTPRINT), start_new_session=True)
    st = {'key': key, 'unit': cfg['unit'], 'pid': proc.pid, 'start_epoch': time.time(), 'cap_s': FLOOR_S,
          'resized': False, 'jobs': JOBS, 'timeout_s': timeout_s,
          'footprint': dict(FOOTPRINT, jobs=JOBS, jobserver_tasks=JOBSERVER_TASKS, priority=' '.join(PRIORITY)),
          'command': f'mkdir -p target/mutants-tmp && TMPDIR=$PWD/target/mutants-tmp {ENV_PREFIX} ' + shlex.join(cmd)}
    write_status(key, dict(st, state='running'))
    poll(key, st, proc)
elif mode == 'watchrate':
    # re-attach to a running invocation whose rate changed under it (the pin): the one re-size at the floor reads
    # the LATER rate - mutants tested per second since this marker - never the blended one (the operator's word)
    st = json.load(open(status_path(key), encoding='utf-8'))
    pid, start = st['pid'], st['start_epoch']
    n1, _ = live(key)
    e1 = time.time() - start
    st['pin'] = {'cpus': PIN, 'by': 'the operator, taskset on the running tree (root pid %d, descendants inherit)' % pid,
                 'at': '2026-10-09T17:28Z', 'marker': {'elapsed_s': round(e1), 'live_tested': n1}}
    write_status(key, dict(st, state='running'))
    while True:
        elapsed = time.time() - start
        tested, end = live(key)
        if not alive(pid):
            st.update(state='exited', returncode=None, elapsed_s=round(elapsed), live_tested=tested, end_time=end,
                      planned=planned(key))
            break
        if not st['resized'] and elapsed >= max(FLOOR_S, e1 + 600):
            if tested <= n1:
                stop(pid)
                st.update(state='budget-exhausted', elapsed_s=round(elapsed), live_tested=tested,
                          planned=planned(key), note='none tested since the pin marker at the floor')
                break
            rate = (elapsed - e1) / (tested - n1)
            new_cap = max(FLOOR_S, round(planned(key) * rate * 1.5))
            st.update(resized=True, rate_s_per_mutant=round(rate, 2), planned=planned(key), resized_cap_s=new_cap,
                      rate_at={'elapsed_s': round(elapsed), 'live_tested': tested, 'since_elapsed_s': round(e1),
                               'since_live_tested': n1}, cap_s=new_cap)
            if new_cap > 7200:
                st['over_2h'] = 'whole under its computed cap - the operator, 2026-10-09: time is not the constraint'
            write_status(key, dict(st, state='running'))
        elif st['resized'] and elapsed >= st['cap_s']:
            stop(pid)
            tested, end = live(key)
            st.update(state='budget-exhausted', elapsed_s=round(time.time() - start), live_tested=tested,
                      planned=planned(key))
            break
        time.sleep(15)
    write_status(key, st)
    print(json.dumps(st, indent=1))
elif mode == 'watch':
    st = json.load(open(status_path(key), encoding='utf-8'))
    st['cap_s'] = int(sys.argv[3])
    poll(key, st)
