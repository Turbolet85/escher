# Relay 2 — P4 forks of 2026-10-14-quality-gates (the dialog came back unanswered, 2026-10-15)

The question dialog returned no answer: "No answer from the driver within 58 min". Per the dialog fallback, the forks
are written here; the answer that arrives in a message is appended below, this file snapped, and P4 resumes on it.
Forks 1–3 are the founder's (each a new or widened CI gate, inputs#I1); fork 4 is the operator's. The recommendation
is marked on each.

## Fork 1 — Coverage floor on the driver crates (the founder's: a new CI gate)
Readings at HEAD, identical on the host and in CI#38649787488: escher-driver 98.20 %, dioxus-native-dom 84.25 %,
blitz-test-harness 91.69 %, escher-telemetry 96.48 %.
1. **3 crates, 1 pt under (Recommended)** — escher-driver ≥ 97 %, dioxus-native-dom ≥ 83 %, blitz-test-harness ≥ 90 %,
   each read per crate from `lcov.info` by a stdlib script inside the existing coverage leg (no new job): the three
   crates a driver call runs through (escher-driver's own dependency edges); the readings are deterministic, so 1 pt
   leaves room for small untested additions without inviting rot.
2. **4 crates, incl. telemetry** — the same three plus escher-telemetry ≥ 95 % (96.48 %): the sink the driver's binaries
   install, though escher-driver does not depend on it — a reach beyond "driver crates".
3. **escher-driver only** — ≥ 97 %; the narrowest reading of the entry, leaving the snapshot/diff and settle/screenshot
   code it serves unguarded.
4. **No floor yet — defer** — record the readings in test-plan §10, no threshold; a residuals line naming your label;
   the entry's "enforced on fork CI" is then not met in 0.1.0.

## Fork 2 — Flakiness budget for the stand checks (the founder's: a new CI gate)
`run stand` reads 168 · 0 · 7 over 40 files in 55 s warm; nothing retries a test in CI today.
1. **Repeat job, zero budget (Recommended)** — a new `stand-repeat` leg and slow-tier job: every `stand_*.rs` and the
   seven_guis process checks (`cli_*`, `host_*`, `mcp_*`) run 3× in sequence on linux; any failure in any repeat reds the
   job (budget = 0 failures), the budget first measured locally at 20 repeats. Cost: one more slow-tier job about the
   a11y job's size, restoring the test cache read-only. Catches an intermittent failure CI would otherwise meet by luck.
2. **No new job, zero budget** — budget 0 from 20 local repeats, recorded in §10; enforcement is the existing `test` leg
   running each check once with no retry, pinned by a ci-scripts invariant that no leg retries. Cheaper; detects a flake
   only when it fires on a push.
3. **Measure only — defer gate** — 20 local repeats recorded, no CI enforcement; a residuals line naming your label.

## Fork 3 — Which further gates fork CI runs (the founder's: each a new or widened CI gate; several may be chosen; each not chosen becomes a residuals line naming your label)
1. **Licence check (Recommended)** — `cargo deny --locked check licenses` beside advisories in the audit leg, over the
   audit's six-target graph; reads `licenses ok` today (two unmatched allowances, NCSA and Unicode-DFS-2016, warn).
   Upstream ran it with no target filter — not taken.
2. **Upstream blitz-dom clippy (Recommended)** — upstream's `cargo clippy -p blitz-dom --no-default-features --features
   svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets -- -D warnings` as a second command of the
   clippy leg; green at HEAD, 16 s warm; adds `--all-targets` (blitz-dom's tests and benches) and the build without
   `text-transform-icu`.
3. **Audit the whole workspace (Recommended)** — the audit leg runs `cargo deny --locked --workspace --all-features check
   advisories`: it reaches paste (RUSTSEC-2024-0436, unmaintained, no patched release), settled by one reasoned per-ID
   ignore naming its owner; memmap2's unsound advisory stays unreported at the defaults, recorded as a measured limit.

## Fork 4 — apt-install.sh and an interrupted dpkg (the operator's)
A bound firing inside `apt-get install` can leave dpkg interrupted; the script never repairs it.
1. **Add a bounded repair (Recommended)** — before every retried attempt, `sudo timeout … dpkg --configure -a` under the
   same bound and grace, pinned by an `InstallScriptTest` stand-in, the exit-2-before-anything rule kept; closes the
   named limit at no cost to a healthy run; its only witness stays the stand-in, as for the bound itself.
2. **Record only** — keep the named limit as test-plan §9 and arch state it; record that the eight-not-nine witness is
   expected (PyYAML on the runner image); no script change.

## Answer

Arrived in a message from the operator, 2026-10-15T05:06Z, verbatim:

> These are the answers to the timed-out dialog 1792020093389-3900480-0. Q1–Q3 are the FOUNDER's, relayed verbatim by
> the overseer as his chosen option labels; the English wording is the overseer's. Q1 «3 крейта, на 1 пункт ниже
> (Рекомендую)» = 3 crates, 1 pt under. Q2 «Повторное задание, ноль сбоев (Рекомендую)» = Repeat job, zero budget. Q3
> all three: «Проверка лицензий (Рекомендую)», «Clippy blitz-dom, как в основном репозитории (Рекомендую)», «Аудит всего
> workspace (Рекомендую)». Q4, the operator's: Add a bounded repair. Snap
> ~/dev/projects/additional/escher-overseer/relays/quality-gates-founder-answers.md as the record and continue the phase
> to P5.

Resolved: Fork 1 → option 1 · Fork 2 → option 1 · Fork 3 → options 1, 2 and 3 · Fork 4 → option 1. Forks 1–3 are the
founder's own answers, relayed verbatim by the overseer as his chosen option labels; fork 4 is the operator's.
