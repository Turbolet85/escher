#!/usr/bin/env bash
# P5 known-positive controls for 2026-10-20-load-wait: each entry fired through gate.py over a scratch clone at HEAD.
set -u
ROOT=.
C=$TMPDIR/{scratchpad}/clone-load-wait
GATE=~/.claude/skills/andromeda-tools/scripts/gate.py
PLAN=$ROOT/escher-0.2.0/chunks/2026-10-20-load-wait/plan.md

git clone -q --shared "$ROOT" "$C"
echo "clone HEAD $(git -C "$C" rev-parse HEAD)"

fire() {  # label entry
  echo "== $1"
  python -X utf8 "$GATE" run --plan "$PLAN" --entry "$2" --project-root "$C"
}
reset() { git -C "$C" checkout -q -- . ; git -C "$C" clean -fdq ; }

for n in 3 4 5 6 7 8 9 10; do fire "unmodified" $n; done

echo '// Instant::now' >> "$C/packages/escher-driver/src/session.rs"
fire "3 must-pass: session.rs gains Instant::now" 3
fire "5 must-pass: session.rs edited" 5
fire "9 must-pass: session.rs edited" 9
echo '// Instant::now' >> "$C/packages/escher-driver/src/execute.rs"
fire "3 must-fail: execute.rs gains Instant::now too" 3
reset

echo '// Instant::now' >> "$C/packages/blitz-dom/src/document.rs"
fire "4 must-fail: blitz-dom document.rs gains Instant::now" 4
reset

echo '// edited' >> "$C/packages/blitz-test-harness/src/settle.rs"
fire "5 must-fail: settle.rs edited" 5
reset

echo '// info_span!' >> "$C/packages/escher-driver/src/session.rs"
fire "6 must-fail: a third info_span!" 6
reset

echo '// tracing::info!(' >> "$C/packages/escher-driver/src/session.rs"
fire "7 must-fail: a tracing::info! site" 7
reset

printf '%s\n' '//! a silent check' 'mod fixtures;' '#[test]' 'fn t() {}' > "$C/tests/blitz-tests/tests/stand_load_wait.rs"
fire "8 must-pass: a silent stand_load_wait.rs" 8
fire "10 must-pass: stand_load_wait.rs minted" 10
git -C "$C" add tests/blitz-tests/tests/stand_load_wait.rs
git -C "$C" -c user.name=c -c user.email=c@c commit -qm control
fire "9 must-pass: stand_load_wait.rs committed, as after the pre-CI commit" 9
echo 'println!("x");' >> "$C/tests/blitz-tests/tests/stand_load_wait.rs"
fire "8 must-fail: a println! in the check" 8
git -C "$C" reset -q --hard "$(git -C "$ROOT" rev-parse HEAD)"
reset

echo '# edited' >> "$C/Cargo.toml"
fire "9 must-fail: Cargo.toml edited" 9
reset

printf '%s\n' '// extra' > "$C/tests/blitz-tests/tests/extra.rs"
fire "10 must-fail: extra.rs minted" 10
reset
