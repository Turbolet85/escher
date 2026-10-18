# Orchestrator measurement — Taffy's declared MSRV at both revs, /andromeda-phase P3, 2026-10-18T06:02Z

Read by the orchestrator on the dev host through `gh api repos/DioxusLabs/taffy/contents/Cargo.toml?ref={rev}` (read-only), base64-decoded:

- `ref=ea07c4c304216c9b48bdd88c46153c1cbe3cae10` (upstream blitz's git rev): `version = "0.15.0"` (Cargo.toml:3), `rust-version = "1.71"` (Cargo.toml:10).
- `ref=v0.15.0` (the registry release our lock holds): `version = "0.15.0"` (Cargo.toml:3), `rust-version = "1.71"` (Cargo.toml:10).

Taffy main at `ea07c4c3` declares the same package version and the same MSRV (1.71) as the 0.15.0 release, both under our workspace MSRV 1.91.
