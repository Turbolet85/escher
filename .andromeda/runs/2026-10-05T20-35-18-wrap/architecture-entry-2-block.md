
## 2026-10-05-as-built-baseline — measured dev-host build environment
**Section:** §Stack and Technologies (Build environment row)
**Change:** the row now states that the flake's rust-bin "1.90.0" pin sits below the workspace `rust-version` "1.91.0" its own comment says to keep in sync with, and carries the dev-host reading as measured at the chunk's `evidence/baseline.md` (Omarchy 4.0.4, Arch-based, 2026-10-05): the flake is not used; host stable rustc/cargo 1.99.0; no `rust-toolchain*` file, no `.cargo/config*`; Arch packages fontconfig 2.18.3, openssl 3.6.4, pkgconf 3.0.7, python 3.14.7 stand in for CI's `libfontconfig1-dev` and build-time python3.
**Why:** the baseline is the first verified dev-host build; host-tool versions are readings on that host, not pins — no lockfile resolves them.
**Ref:** .andromeda/runs/2026-10-05T20-35-18-wrap/
