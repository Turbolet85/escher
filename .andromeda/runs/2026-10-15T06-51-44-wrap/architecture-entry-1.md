
## 2026-10-14-quality-gates — the quality gates: legs, the new job, the install repair, the code-quality row
**Section:** §Stack and Technologies → CI/CD, Code quality · §Conventions → Formatting and lints · §Standard Contracts → CI contracts · §Occupied Resources → CI infrastructure · §Infrastructure Patterns → CI/CD · §Inherited Defaults → Code quality
**Change:**
- The leg list gains `stand-repeat` (not a fast leg): three rounds of every `stand_*.rs` target, then seven_guis' seven process checks; the first red command ends the leg with its status, no retry; three green rounds print `stand-repeat: 3 of 3 rounds green`.
- `audit` is `cargo deny --locked --workspace --all-features check advisories licenses` (was `cargo deny check advisories`).
- `clippy` runs a second line, only when the workspace line is green: upstream's `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings`.
- `coverage` ends with `coverage_floor.py` over escher-driver 97 % · dioxus-native-dom 83 % · blitz-test-harness 90 % (read 98.34 · 84.25 · 91.69 %). Was "with no threshold".
- New contract `coverage_floor.py LCOV CRATE_DIR=PERCENT …`: exit 0 / 1 / 2, prints crate directories and counts only. The ci-scripts leg discovers seven test files (was six).
- `apt-install.sh`: every attempt after the first runs a bounded `dpkg --configure -a` first, in phase `repair`; the first runs no `dpkg`. Ten calls (was nine), `InstallScriptTest` eight (was five), `dpkg` among the runner tools. Was "dpkg interrupted, which the script does not handle".
- CI/CD: the job `Stand checks repeated`; thirteen linux leg jobs (was twelve); nine `libfontconfig1-dev` jobs (was eight); `a11y` and `stand-repeat` restore `workspace-test` and never save.
**Why:** this chunk's legs and job, on the founder's «3 крейта, на 1 пункт ниже (Рекомендую)», «Повторное задание, ноль сбоев (Рекомендую)», «Проверка лицензий (Рекомендую)», «Clippy blitz-dom, как в основном репозитории (Рекомендую)» and «Аудит всего workspace (Рекомендую)», relayed verbatim by the overseer as his chosen option labels; the repair on the operator's answer «Add a bounded repair» at P4.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/
