# Mutation tier — POSTPONED (not declined)

Stopped 2026-11-18 on the operator's line, relaying the founder: the tier is not to start yet (the account limit is
nearly spent); it waits for his word and resumes by artifact presence on the operator's line.

- Confirmed at the confirm: the eight units at jobs 6 (escher-driver 96 · blitz-paint 43 · wpt 14 · seven_guis 13 ·
  browser 4 · wgpu_texture 2 · dioxus-native 2 · wasm_hello 1) and blitz-dom's 61 with the two FnValue sites excluded
  (`replace (BaseDocument::scroll_state|node_scroll_range) -> `; scrolling.rs:537 ×1000, :12 ×100; 1161 in-diff listed).
- What ran: `mutate.py` started, only seven_guis' hand pass on the UNMUTATED tree (cargo nextest, still compiling);
  stopped by SIGINT to the driver's own process group after 36 s. No `cargo mutants` invocation started, no mutant
  tested, no `mutants-{unit}/` dir, no `state-{unit}.json`, no `c-mutation-{unit}.json`. The torn hand-pass log removed.
- Resume: `python3 .andromeda/runs/2026-11-18T21-45-08-code-audit/mutate.py .andromeda/runs/2026-11-18T21-45-08-code-audit seven_guis browser wpt wgpu_texture dioxus-native wasm_hello blitz-paint escher-driver blitz-dom`
  (forms in `forms.json`; a unit with a `done` state file is skipped). Re-read load and MemAvailable first: the jobs 6
  fit was read 2026-11-18 at load 6.0 and 36.6 GB available.
- Moved 2026-11-19 (the operator's line, the raw dirs' absolute host paths kept out of git): every `mutants-{unit}/`, `rca/`, `jscpd-abs/` and `list/` now stand in the git-ignored `target/audit-raw-2026-11-18/`, the two escher-driver dirs among them (`mutants-escher-driver/`, `mutants-escher-driver.before-stop-2026-11-19/`); a resume moves them back into this run dir first.
