# viola 0.2.0 — Incubator

**Status:** PRE-route material for **v0.2.0**, gathered while v0.1.0 is in flight. Later it is assembled into
`intent.md`.

> ⚠️ **This is NOT a version folder.** The `-incubator` suffix keeps it out of Andromeda's version cursor. New-session
> and phase read only the highest *strict* `viola-X.Y.Z/` dir, and `0.2.0-incubator` is not `X.Y.Z`, so the cursor
> stays at `viola-0.1.0`. The real `viola-0.2.0/` is created later by `/andromeda-route --version 0.2.0`, from
> the `intent.md` assembled here. Do not treat this folder as the active version. It is gitignored
> (`.gitignore`, as Pulse's `andromeda-pulse-0.4.0-incubator/` is) and stays local and untracked until it is
> promoted.

## Purpose
The pattern is **intent-driven**: one `intent.md` → `/andromeda-route` derives the vision, requirements, working
route and verification matrix → the loop builds and gates each cap. This folder is where the *material* for 0.2.0's
intent accumulates (research, rulings, sharp questions), so the intent is assembled from a measured base when 0.1.0
ships, not invented cold.

## The 0.2.0 direction (the frame)

> **Reframe pending (founder direction 2026-10-04):** the frame below was written for a Windows host. The founder
> wants the next versions rethought for a tiling host (Omarchy/Hyprland). See `open-questions.md` Q9 before reading
> Q1/Q2 as settled.

**0.2.0 makes the session hierarchy visible and reachable.**

- **One node graph per project:** `overseer1` → the project overseers → their builders and workers. It is drawn
  with React Flow on the React + TypeScript web front that 0.1.0 lands (the Lit → React switch is a 0.1.0 item,
  founder-ruled 2026-10-01, entering the route at the Epoch 2b corrective wrap).
- **The founder's pain behind it:** with ~10 overseers running at once, the problem is *finding the right terminal
  to type into*. A graph that only shows the sessions does not solve that. **RAISE is therefore the core of 0.2.0,
  not a nicety:** picking a node must bring that session's terminal to the foreground.
- **The hard fact under the frame:** on the founder's host the foreground-lock timeout is infinite
  (`SPI_GETFOREGROUNDLOCKTIMEOUT` = 2147483647 ms, *measured* 2026-09-30). A click in the graph reaches the
  background `viola ui` server, which therefore **cannot** raise a window by itself. So 0.2.0's real decision is
  the raise *trigger*, and every candidate trigger touches the security plan. The forks are in `open-questions.md`.
- **0.2.0 is also where sessions are CREATED, not only seen and reached** (the founder's direction,
  2026-10-01).
  - Every session runs under viola, the overseers included, and one registry holds every session id and every
    edge.
  - The parent is declared at launch; observed `link`s are drawn on top, styled apart.
  - The founder clicks an overseer's node, presses "add worker", and viola launches the session and records the
    edge at once, replacing today's hand-run `.ps1` launcher.
  - That makes the spawn button the GUI's first write route (code-bearing, so it needs full write auth), and it
    makes viola-opened named terminal windows (R3) central rather than opt-in. See `open-questions.md` Q5/Q6 and
    the re-weighed `raise-and-focus.md`.

**Already decided for 0.1.0, so not in 0.2.0's scope:** revive (`viola revive <name>`, the study's V2: resume in
place, no launch replay, with P4/P5/P6/P7/P12 as its measurement step) and the React + TypeScript toolchain chunk
at Epoch 8's head (frontend sources in the code-graph ts plane). Founder ruling, 2026-10-01.

## Contents
- **`open-questions.md`** — the forks the 0.2.0 intent must resolve: the raise trigger, the window identity model,
  the gating probes (P1/P2/P3/P9 + P13–P15), the security-plan entries, the graph registry (Q5, direction
  founder-ruled), spawn from the UI (Q6, direction founder-ruled), and the Unix legs. **Read this first.**
- **`raise-and-focus.md`** — the 0.2.0 half of the session-memory study, re-weighed 2026-10-01: R3 + R5 + V4 are
  core, R1 + R2 become a deferrable hand-started leg, R4 is not planned, with the comparison and size tables.
- **`deferred-from-0.1.0.md`** — the items deliberately kept out of 0.1.0: launch-spec replay, V3, V4, V5b, and
  V5a (rejected), each with its reason and its status after spawn (`launch.json`, V4 and V5b move to 0.2.0's
  core; V3 for spawned sessions only).
- **`research/`** — findings from probes and delegated research. Drop results here as they come.
- **The source study:** `refs/session-memory-options.md` (tracked, commit `4a3062d`) holds the full reasoning,
  the measured facts F1–F8 / M1–M9 and the probe list P1–P12. The notes here point into it rather than copying it.

## How this becomes the intent
When v0.1.0 ships and the material is rich enough: assemble `intent.md` from this folder (the frame plus the
resolved open questions, in OBSERVED → EXPECT shape), then `/andromeda-route --version 0.2.0` derives the version.

**Cap numbering:** 0.1.0 used `v1-01..v1-53` (*measured*: `viola-0.1.0/requirements.md` and
`verification-matrix.json`, 53 ids). 0.2.0's id scheme is the route's to pick: either `v1-54+` continuing the
series, or a new `v2-01` prefix. Settle it before the intent cites any cap id.
