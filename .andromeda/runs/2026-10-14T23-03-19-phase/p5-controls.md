# P5 controls — 2026-10-14-quality-gates (check 4 (9))

Typed in the gate tool's shell (`/usr/bin/bash -o pipefail -c`), each differing from its entry's `run` by the path
operand alone, on the untouched tree at `eb5ba90c`, 2026-10-15T05:17Z.

- Entry 6 (the silence census, `command grep -c -E '…' {file}`):
  - must-pass — `tests/blitz-tests/tests/stand_counter.rs` (a stand check with no named exception): `0`, exit 1 → the
    entry's `['exit 1', 'last line 0']` holds.
  - must-fail — `tests/blitz-tests/tests/stand_act_spans.rs` (installs the sink in its children, a named exception):
    `5`, exit 0 → the entry reads red.
- Entry 13 (the flake-record reader, `command grep -c '^| [0-9]* | passed |' {file}`), over two files minted in the
  session scratchpad in step 9's row form:
  - must-pass — `$TMPDIR/{scratchpad}/flake-pass.md`, 20 rows `| R | passed | … |`: `20`, exit 0 → `last line 20` holds.
  - must-fail — `$TMPDIR/{scratchpad}/flake-fail.md`, round 7 written `failed`: `19`, exit 0 → `last line 20` fails.
