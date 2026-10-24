#!/usr/bin/env bash
# P5 controls for 2026-10-24-upstream-sync-polish-and-ship: each guard fired AS ITS ENTRY over a scratch clone of
# HEAD carrying the take (must-pass), then with one planted input (must-fail). Run from the project root.
set -u
ROOT=$(git rev-parse --show-toplevel)
G=~/.claude/skills/andromeda-tools/scripts/gate.py
PLAN=$ROOT/escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/plan.md
W=$(mktemp -d)
trap 'rm -rf "$W"' EXIT
C=$W/take
git clone --shared -q "$ROOT" "$C"
echo "clone take: HEAD $(git -C "$C" rev-parse HEAD)"
fire() { echo "--- entry $1 · $2"; python -X utf8 $G run --plan "$PLAN" --entry "$1" --project-root "$C"; }

# the take, as step 2 and step 3 write it
for s in 28f848d3bcf80dcb3b5af66ce34820172960d0c5 9eeebb30255b8c72f1810b02964f3789a2c35d12 e4ca5b9b547cae93c033822746a19bac6d2e0249 46d7cf993c0d8f5b5ba569ce3c0aa25af1b0043c fb297a38c1f25f34ee290c0aba26c5094956afc9 50bf2f2356f588839977c5dc0df722a689a867d1 f8723f495c7c6bfb611a923a21ed5c8e0fe95d55 21edc92d90d8f8bedadd3f9ecb14b1ad950952eb f102663b81012a1aa9a30aecf25273348fb52db6 f9cbb275628b8f3017df8f76150f5f32df218620; do
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
( cd "$C" && cargo update -p tokio ) 2>&1 | tail -2

# must-pass: the take
for n in 1 2 3 4 5 6 8 9 10 13 22; do fire $n "must-pass, the take"; done
fire 25 "must-pass, the take (the audit)"

# must-fail: one planted input each, reverted after
f=packages/blitz-dom/src/mutator.rs
cp "$C/$f" "$W/keep"; sed -i '0,/self.doc.changed_nodes.insert(parent_id);/{//d}' "$C/$f"
fire 1 "must-fail, our first changed-set insert in mutator.rs dropped"; cp "$W/keep" "$C/$f"
f=.github/scripts/ci-leg.sh
cp "$C/$f" "$W/keep"; echo '# planted' >> "$C/$f"
fire 6 "must-fail, a line appended to ci-leg.sh"
sed -i "0,/--ignore-filename-regex 'keyboard_compat\\\\.rs\\\$'/s///" "$C/$f"
fire 15 "must-fail, one keyboard_compat exclusion dropped"; cp "$W/keep" "$C/$f"
f=Cargo.toml
cp "$C/$f" "$W/keep"; echo 'planted = { git = "https://example.invalid/planted" }' >> "$C/$f"
fire 7 "must-fail, a git dependency line planted"; cp "$W/keep" "$C/$f"
f=Cargo.lock
cp "$C/$f" "$W/keep"; printf '\n[[package]]\nname = "planted"\nversion = "0.0.1"\n' >> "$C/$f"
fire 9 "must-fail, a package entry planted in the lock"; cp "$W/keep" "$C/$f"
f=packages/blitz-dom/src/util.rs
cp "$C/$f" "$W/keep"; echo 'fn _planted() { let _ = std::time::Instant::now(); }' >> "$C/$f"
fire 12 "must-fail, an Instant::now planted in util.rs"; cp "$W/keep" "$C/$f"
touch "$C/tests/blitz-tests/tests/stand_planted.rs"
fire 30 "must-fail, a stand file minted"
echo "controls done"
