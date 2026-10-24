
## 2026-10-24-upstream-sync-polish-and-ship — the upstream take read at `f9cbb2756`: stack, conventions and project intent
**Section:** §Stack (body; stack-and-technologies.md) · §Conventions (conventions.md, Manifests) · §Project Intent (body; project-intent.md)
**Change:**
- Stack, body: the drift sentence is retired — was «the flake's "1.90.0" pin sits below the workspace `rust-version` "1.91.0"»; now mimalloc is the global allocator of two upstream-only binaries, the browser app (by default) and the WPT runner.
- stack-and-technologies: tokio "1.42" resolved 1.54.1, past RUSTSEC-2026-0386; `mimalloc` 0.1.48 (lock 0.1.52, with libmimalloc-sys 0.1.49) in the browser's `default` features (was optional); the WPT runner's direct `mimalloc` "0.1.48" and its `#[global_allocator]`; the flake's rust-bin "1.91.0" (was "1.90.0"), matching `rust-version` (was «sits below»).
- Conventions, Manifests: the WPT runner's direct `mimalloc = "0.1.48"` is named the one exception to «declared once in `[workspace.dependencies]`».
- Project Intent: upstream `main` last read at `f9cbb2756` (was `17de049b1`), thirteen of its commits taken by no-commit cherry-picks (was three), the ten of this sync named; the merge base `23b9b26a9` unmoved; git Stylo 0.23-pre, the six Taffy-main commits (was four) and Dioxus 0.8.0-alpha.3 wait for escher's next version, owned by a `.andromeda/residuals.md` line (target: next); Stylo stays "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", no git dependency.
**Why:** the take is the founder's own word «Десять коммитов (Рекомендую)» on the P4 take-shape fork, relayed verbatim by the overseer (2026-10-24, chunk inputs#I4) — ten commits and tokio 1.54.1, the pins unchanged. The left-out delta's residuals line is the operator's own word «yes — apply» on the phase's P5 card (inputs#I5).
**Ref:** .andromeda/runs/2026-10-24T12-21-29-wrap/
