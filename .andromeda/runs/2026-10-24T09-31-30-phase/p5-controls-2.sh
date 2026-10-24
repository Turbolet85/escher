#!/usr/bin/env bash
# P5 control, second pass: entry 4 (construct.rs against upstream's two commits) after its expect moved to
# ['exit 1', 'last line 6'] — fired AS THE ENTRY over a scratch clone of HEAD carrying the take. Run from the root.
set -u
ROOT=$(git rev-parse --show-toplevel)
G=~/.claude/skills/andromeda-tools/scripts/gate.py
PLAN=$ROOT/escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/plan.md
W=$(mktemp -d)
trap 'rm -rf "$W"' EXIT
C=$W/take
git clone --shared -q "$ROOT" "$C"
echo "clone take: HEAD $(git -C "$C" rev-parse HEAD)"
for s in 28f848d3bcf80dcb3b5af66ce34820172960d0c5 46d7cf993c0d8f5b5ba569ce3c0aa25af1b0043c; do
  git -C "$C" cherry-pick --no-commit "$s" > /dev/null 2>&1
  echo "pick $s exit $? unmerged: $(git -C "$C" diff --name-only --diff-filter=U | tr '\n' ' ')"
  if [ -n "$(git -C "$C" diff --name-only --diff-filter=U)" ]; then
    python3 - "$C/packages/blitz-dom/src/layout/construct.rs" <<'PY'
import re, sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
m = re.search(r"<<<<<<< [^\n]*\n(.*?)=======\n(.*?)>>>>>>> [^\n]*\n", s, re.S)
line = "node::{NodeFlags, NodeKind, SpecialElementData, TextInputData, TextLayout},"
fixed = m.group(2).replace(line, "node::{NodeFlags, NodeKind, RangeInputData, SpecialElementData, TextInputData, TextLayout},")
open(p, "w", encoding="utf-8", newline="\n").write(s[: m.start()] + fixed + s[m.end():])
print("construct.rs resolved: upstream line plus RangeInputData")
PY
    git -C "$C" add packages/blitz-dom/src/layout/construct.rs
  fi
done
echo "--- entry 4 · must-pass, the take's construct.rs"
python -X utf8 $G run --plan "$PLAN" --entry 4 --project-root "$C"
f=$C/packages/blitz-dom/src/layout/construct.rs
sed -i 's/node::{NodeFlags, NodeKind, RangeInputData, SpecialElementData, TextInputData, TextLayout},/node::{NodeFlags, NodeKind, SpecialElementData, TextInputData, TextLayout},/' "$f"
echo "--- entry 4 · must-fail, RangeInputData dropped from the resolved line"
python -X utf8 $G run --plan "$PLAN" --entry 4 --project-root "$C"
echo "controls done"
