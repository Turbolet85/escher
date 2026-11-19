import json, re
C = '.andromeda/cache/evolve-diagnose/2026-11-18T21-35-38'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')  # a malformed ts is KEPT (the checkpoint fired) and listed
recs, skips, bad_ts = [], 0, []
for line in open(C + '/friction.ndjson', encoding='utf-8'):
    try: r = json.loads(line)
    except ValueError: skips += 1; continue
    if isinstance(r, dict) and 'record' in r and len(r) == 1:
        r = r['record']
        if isinstance(r, str):
            try: r = json.loads(r)
            except ValueError: skips += 1; continue
    if not ISO.fullmatch(str(r.get('ts'))): bad_ts.append([r.get('chunk'), r.get('skill'), r.get('step'), r.get('kind'), str(r.get('ts'))[:40]])
    recs.append(r)
by_id = {r['id']: r for r in recs if r.get('id')}
excl_ids, excl_probs, unres, clause_notes = set(), set(), [], {}
for r in recs:
    rl = r.get('retracts')
    if not rl: continue
    for t in (rl if isinstance(rl, list) else [rl]):
        if not isinstance(t, dict):
            unres.append([r.get('id'), 'prose-form-pre-boundary', t]); continue
        tid, scope = t.get('id'), t.get('scope')
        tgt = by_id.get(tid)
        if tid is None or tgt is None or scope not in ('record', 'problem', 'clause'):
            unres.append([r.get('id'), 'unknown-id-or-scope', t]); continue
        if scope == 'record':
            if tgt.get('kind') == 'step':
                unres.append([r.get('id'), 'record-scope-on-step', t]); continue
            excl_ids.add(tid)
        elif scope == 'clause':
            if tgt.get('kind') == 'step':
                unres.append([r.get('id'), 'clause-scope-on-step', t]); continue
            if not t.get('note'):
                unres.append([r.get('id'), 'clause-without-note', t]); continue
            clause_notes.setdefault(tid, []).append(t['note'])
        else:
            if tgt.get('kind') != 'step':
                unres.append([r.get('id'), 'problem-scope-off-step', t]); continue
            ix, facts = t.get('index'), tgt.get('problem')
            facts = facts if isinstance(facts, list) else ([facts] if facts else [])
            if ix is not None and (type(ix) is not int or not 0 <= ix < len(facts)):
                unres.append([r.get('id'), 'problem-index-out-of-range', t]); continue
            excl_probs.add((tid, ix))
retr_of_retr = sorted(r['id'] for r in recs if r.get('retracts') and r.get('id') in excl_ids)
out = {'records': len(recs), 'unparseable': skips, 'malformed_ts': bad_ts,
  'retracted_ids': sorted(excl_ids), 'retracted_problems': sorted(map(list, excl_probs), key=str),
  'clause_retracted': clause_notes,
  'unresolvable': unres, 'retraction_of_retraction': retr_of_retr}
open(C + '/q-retractions.json', 'w', encoding='utf-8').write(json.dumps(out, ensure_ascii=False, indent=1))
print(json.dumps({k: (v if not isinstance(v, list) or len(v) < 20 else len(v)) for k, v in out.items()}, ensure_ascii=False)[:4000])
