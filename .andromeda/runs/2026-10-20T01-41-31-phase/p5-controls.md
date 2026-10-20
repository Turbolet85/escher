# P5 controls — 2026-10-20-load-wait

Every control was fired as its entry, `gate.py run --plan … --entry {n} --project-root {clone}`, with no run dir.
The clone is a `git clone --shared` of the project at HEAD `165cfe443f46551338dec46e1d7d7104fe2b1175`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/9146d674-49b9-405a-b666-2923bebf7da2/scratchpad/clone-load-wait`.
Each input was made in the clone and reverted. The script is `p5-controls.sh` beside it.

## Unmodified (the clone reads what the tree read)

| entry | reading |
|---|---|
| 3 | red: last line `client.rs host.rs mcp.rs` (no `session.rs`) |
| 4 | green: exit 1 |
| 5 | green: exit 0, last line 0 |
| 6 | green: last line 2 |
| 7 | green: exit 1 |
| 8 | red: exit 2, file absent |
| 9 | green: exit 1 |
| 10 | green: exit 1 |

## Must-pass (the subject as the plan builds it)

| entry | input | reading |
|---|---|---|
| 3 | `session.rs` gains `Instant::now` | green: exit 0, the four-file line |
| 5 | `session.rs` edited | green: last line 0 |
| 9 | `session.rs` edited | green: exit 1 |
| 9 | `stand_load_wait.rs` committed in the clone, as after the pre-CI commit | green: exit 1 |
| 8 | a silent `stand_load_wait.rs` minted | green: exit 1 |
| 10 | `stand_load_wait.rs` minted | green: exit 1 |

## Must-fail

| entry | input | reading |
|---|---|---|
| 3 | `execute.rs` gains `Instant::now` too | red: a five-file line |
| 4 | blitz-dom `document.rs` gains `Instant::now` | red: exit 0 |
| 5 | blitz-test-harness `settle.rs` edited | red: last line 1 |
| 6 | a third `info_span!` | red: last line 3 |
| 7 | a `tracing::info!(` site | red: exit 0 |
| 8 | a `println!` in the check | red: exit 0 |
| 9 | blitz-dom `lib.rs` edited | red: exit 0 |
| 10 | `extra.rs` minted | red: exit 0 |

A first must-fail for entry 9 edited the root `Cargo.toml` and read green. That is correct, not vacuous: the root
manifest lies outside entry 9's pathspecs (`packages examples tests apps wpt .github scripts`), and entry 5 reads
it. The control was re-aimed at a path inside entry 9's read (blitz-dom `lib.rs`) and read red.
