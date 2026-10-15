
## 2026-10-14-quality-gates — the floors and the flakiness budget, the new checks, the counts
**Section:** §1 (CI Python scripts · seven_guis · blitz-dom · escher-driver · tests/blitz-tests) · §4 escher-driver, CI workflows and leg script · §5 Session host · §6 Command line · §8 Real dependencies kept · §9 Legs, Cache, Package installs, Engine features by runner, Per-OS test tallies (new), Local baseline, Coverage · §10 · `§3 → Session lifecycle`
**Change:**
- §10: coverage floors escher-driver 97 % · dioxus-native-dom 83 % · blitz-test-harness 90 % (read 98.34 · 84.25 · 91.69 %), and a zero-failure flakiness budget on the stand checks (20 local rounds 0 failures). 67.06 % beside the first 53.23 % reading. NOT YET MEASURED narrowed to a performance budget (was a quality gate, threshold, flakiness or performance budget).
- §9: thirteen linux jobs; the `stand-repeat` leg; the audit, clippy and coverage legs widened; `stand-repeat` restore-only; ten install steps with the repair (was "dpkg … not handled"). The per-OS tallies of CI#38649787488 attributed. `seven_guis_native`'s stderr by level measured (was "Not measured"). Workspace 186 · 955 · 0 · 12 (was 185 · 951).
- §1 / §4: CI scripts 94 in seven files (was 79 in six), `InstallScriptTest` eight, `CoverageFloorTest` 7. escher-driver 73 (was 72), client.rs's per-read bound. `cli_commands` 12 (was 11) with the closed-stdout check. `font_face_sniffed_source` 2. `stand_act_spans` reads a not-settled call (was "`busy` … unit test only").
- §8: no `document.rs` font test skips — the skip was the one-argument `qual_name!` fixture, not missing system fonts (was "skip with an `eprintln!` … without `system-fonts`").
- Key file: `session-wire` and `session-host` name the per-read bound's pin; `cli_commands` 12.
**Why:** the floors and budget on the founder's «3 крейта, на 1 пункт ниже (Рекомендую)» and «Повторное задание, ноль сбоев (Рекомендую)», relayed verbatim by the overseer as his chosen option labels; the rest measured at this chunk.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/
