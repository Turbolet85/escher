"""Read-only cross-epoch lookback for Stage 4 recurrence (themes and signatures)."""
import json
import os
import re
import sys

RUN = os.path.dirname(os.path.abspath(__file__))
d = json.load(open(os.path.join(RUN, 'q-facts-raw.json'), encoding='utf-8'))
pat = re.compile(sys.argv[1], re.I)
sol = sys.argv[2] if len(sys.argv) > 2 else None
for r in d['prior_lookback_0_1_0']:
    if pat.search(r['note'] or '') and (sol is None or r['solution'] == sol):
        print('-', r['epoch'], r['chunk'], r['step'], r['nature'], r['solution'], '|', r['note'][:260])
