# Silence census — 2026-10-31-engine-debts

Token set (`.andromeda/obs-plan-detail/log-coverage.md:124`):
`println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`

Read by gate entry 2 over every blitz-tests check this chunk adds or edits, `common/mod.rs` and `fixtures/mod.rs`:
`exit 1 · last line 0`, 0 files holding a token. Not on that entry's list:

- `session_common/mod.rs`: not edited by this chunk. Its one hit is `std::env::current_exe` at `:345`, a path for
  `shot_prefix`'s re-run and no env read.
- `stand_non_lean_tasks.rs`: edited by this chunk (the moved pin, `scope-record.md`), and on no entry's list. Its diff adds
  a comment and rewords two doc lines, holding no token.

Entry 4, the added lines under `packages/`: `exit 1 · last line 0`, no log site, print, env read, listener, wall-clock
read or panic path. `packages/blitz-shell/src/event.rs` is held out by that entry, its addition an inline test. Entry 3:
escher-driver holds its 2 `info_span!` sites. Entry 5: no `Instant::now()` or `SystemTime::now()` in blitz-dom outside
`clock.rs`.

## Reader counts

| Shared module | Readers before | Readers now | Who joined |
|---|---:|---:|---|
| `tests/fixtures/mod.rs` (`mod fixtures;`) | 24 | 28 | `inert_subtree`, `accessibility_states`, `checkable_change_event`, `text_range_no_change_event` |
| `tests/session_common/mod.rs` (`mod session_common;`) | 31 | 31 | none |

`stand_act_transformed`, `stand_lint`, `stand_keyboard`, `stand_cells_served` and `stand_load_page` were already readers
of the modules they read. `stand_contrast` reads `common` and no fixture.
