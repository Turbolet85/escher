import json, re, sys, collections

LEDGER = '.andromeda/friction-log.ndjson'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')


def norm_epoch(s):
    return re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')


def norm_skill(s):
    return re.sub(r'^andromeda-', '', s or '')


def load():
    recs, skips, bad_ts = [], 0, []
    for ln, line in enumerate(open(LEDGER, encoding='utf-8'), 1):
        if not line.strip():
            continue
        try:
            r = json.loads(line)
        except ValueError:
            skips += 1
            continue
        r['_line'] = ln
        r['skill'] = norm_skill(r.get('skill'))
        r['_epoch'] = norm_epoch(r.get('epoch'))
        p = r.get('problem')
        r['problem'] = [] if p is None else (p if isinstance(p, list) else [p])
        if not ISO.fullmatch(str(r.get('ts'))):
            bad_ts.append([r.get('chunk'), r.get('skill'), r.get('step'), r.get('kind'), str(r.get('ts'))[:40]])
        recs.append(r)
    return recs, skips, bad_ts


if __name__ == '__main__':
    recs, _, _ = load()
    mode = sys.argv[1]
    if mode == 'steps':
        for r in recs:
            if r['kind'] == 'step':
                print(r['_line'], r['chunk'], r['skill'], r['step'], r.get('outcome'),
                      'probs=%d' % len(r['problem']),
                      'consumed=' + json.dumps([(c.get('artifact'), c.get('quality')) for c in r.get('consumed') or []], ensure_ascii=False)[:300])
    elif mode == 'friction':
        for r in recs:
            if r['kind'] == 'friction':
                print(r['_line'], r['id'], r['chunk'], r['skill'], r['step'], r.get('type'),
                      'untyped' if r.get('untyped') else '', json.dumps(r.get('impact')), '|', r.get('what'), '|', r.get('evidence') or '-')
    elif mode == 'problems':
        for r in recs:
            for i, p in enumerate(r['problem']):
                print(r['_line'], r['id'], r['chunk'], r['skill'], r['step'], i, json.dumps(p, ensure_ascii=False))
    elif mode == 'keys':
        print(collections.Counter(k for r in recs for k in r))
