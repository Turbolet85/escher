"""Mutation tier (C1) driver: per scoped unit, sequentially, the unmutated hand pass of the unit's test set
(read green first), then cargo-mutants on nextest's `mutants` profile with --baseline=skip, under the
founder's footprint (-j 2, taskset -c 12-15,28-31, --timeout 600). Each unit's test set is the one the
baseline record's `commands["mutation:{baseline_key}"]` names. A unit with mutants.out/outcomes.json
already complete is skipped (resume by artifact presence).

usage: mutate.py RUN_DIR LEDGER BASELINE_SHA
"""
import json, os, shlex, subprocess, sys, time

rd, ledger, base = sys.argv[1:4]
CPUS = '12-15,28-31'
UNITS = [  # (unit, baseline commands key, --file list, cap_s)
    ('seven_guis', 'mutation:seven_guis', ['examples/seven_guis/src/tasks/timer.rs'], 2308),
    ('escher-driver', 'mutation:escher-driver', [
        'packages/escher-driver/src/mcp.rs', 'packages/escher-driver/src/mcp/framing.rs',
        'packages/escher-driver/src/mcp/listing.rs', 'packages/escher-driver/src/mcp/result.rs',
        'packages/escher-driver/src/mcp_tests.rs', 'packages/escher-driver/src/schema.rs',
        'packages/escher-driver/src/schema/spec.rs', 'packages/escher-driver/src/schema_tests.rs'], 11905),
    ('blitz-dom', 'mutation:blitz-dom/range-keyboard', [
        'packages/blitz-dom/src/node/range.rs', 'packages/blitz-dom/src/events/keyboard.rs'], 18248),
]
recs = [json.loads(l) for l in open(ledger, encoding='utf-8') if l.strip()]
b = [r for r in recs if r['sha'] == base][-1]
log = open(f'{rd}/mutation-driver.log', 'a', encoding='utf-8')


def say(*a):
    log.write(time.strftime('%Y-%m-%dT%H:%M:%SZ ', time.gmtime()) + ' '.join(map(str, a)) + '\n'); log.flush()


def test_set(key):
    cmd = b['commands'][key].split(' · ')[1]  # the cargo mutants form
    toks = shlex.split(cmd)
    pk = next(t for t in toks if t.startswith('--test-package=')).split('=', 1)[1].split(',')
    args = [t.split('=', 1)[1] for t in toks if t.startswith('--cargo-test-arg=')]
    return pk, args


def complete(out):
    p = f'{out}/mutants.out/outcomes.json'
    if not os.path.isfile(p):
        return False
    o = json.load(open(p, encoding='utf-8'))
    return bool(o.get('end_time'))


env = dict(os.environ, TMPDIR=os.path.abspath('target/mutants-tmp'), CARGO_BUILD_JOBS='4', NEXTEST_PROFILE='mutants')
os.makedirs('target/mutants-tmp', exist_ok=True)
for unit, key, files, cap in UNITS:
    out = f'{rd}/mutants-{unit}'
    if complete(out):
        say(unit, 'already complete, skipped'); continue
    pk, args = test_set(key)
    hand = ['taskset', '-c', CPUS, 'cargo', 'nextest', 'run', '--locked'] + sum((['-p', p] for p in pk), []) + args
    say(unit, 'hand pass:', 'CARGO_BUILD_JOBS=4 NEXTEST_PROFILE=mutants ' + shlex.join(hand))
    with open(f'{rd}/hand-{unit}.log', 'w', encoding='utf-8') as hl:
        rc = subprocess.run(hand, env=env, stdout=hl, stderr=subprocess.STDOUT).returncode
    say(unit, 'hand pass exit', rc)
    if rc != 0:
        say(unit, 'baseline-test-failure: no mutant tried'); continue
    mut = ['timeout', '-s', 'INT', str(cap), 'taskset', '-c', CPUS, 'cargo', 'mutants', '-p', unit] + \
        sum((['--file', f] for f in files), []) + ['--test-tool', 'nextest', '--test-package=' + ','.join(pk)] + \
        ['--cargo-test-arg=' + a for a in args] + ['--baseline=skip', '--timeout', '600', '-j', '2', '--output', out]
    say(unit, 'mutants:', 'TMPDIR=$PWD/target/mutants-tmp CARGO_BUILD_JOBS=4 NEXTEST_PROFILE=mutants ' + shlex.join(mut))
    t0 = time.time()
    with open(f'{rd}/mutants-{unit}.log', 'w', encoding='utf-8') as ml:
        rc = subprocess.run(mut, env=env, stdout=ml, stderr=subprocess.STDOUT).returncode
    say(unit, 'mutants exit', rc, 'wall', round(time.time() - t0), 'complete', complete(out))
say('driver done')
