# 1A tree — escher-0.4.0

## Epoch 1 — Foundation (the head's first entry)
- Build directory held (source: intent §1; v040-01; test-plan §9 CI Integration decision/local-baseline)

## Epoch 2 — Debts of 0.1.0–0.3.0 (opens with the head's sync and README)
- Upstream sync ahead of the debts (source: intent §2; v040-02; residuals res-7qgqygtj, res-ipcah32k, res-xpkmxnvl, res-xchmbfjr, res-vuvpa25q, res-q4s3p5ga, res-kk4tzmi5, res-derughab)
- README and records (source: intent §3; v040-03; arch §Project Intent decision/readme-is-escher-own)
- Stand-test setup shared (source: intent §1, the audit's rising duplication; v040-08)
- Codebase health (source: intent §4, the Epoch 6 code audit; v040-08; test-plan §10 decision/complexity-ceiling-host-only)
- Browser's surviving mutants (source: intent §4, `apps/browser/src/capture.rs:86:5`; v040-08)
- Select list debts (source: intent §4; v040-04; res-tym5wois, res-2iu7qn2e, res-caouqg3t, res-ysz67imk)
- Driver debts (source: intent §4; v040-05; res-hfvjj77l, res-7cc5bi4k, res-4dr3wt62)
- Engine debts (source: intent §4; v040-06; res-6k56erh6, res-433ttjb2, res-jqbadec2, res-k2zggoqx)
- Dependency and CI debts (source: intent §4; v040-07; res-j4x7iiqc, res-q2tllspz; test-plan §9 decision/dependency-audit-advisories-and-licenses)

## Epoch 3 — The canvas painted
- Upstream sync ahead of the canvas (source: intent §2; v040-02)
- Canvas in a headless shot (source: intent §5; v040-09)
- Canvas placement and text (source: intent §5, upstream #1578, v030-14 bundled fonts; v040-09)
- Canvas shot across hosts (source: intent §5; v040-09; arch §Existing Scopes decision/no-screenshot-in-cross-os-byte-check)
- Circle Drawer on a canvas (source: intent §5, §10; v040-09)

## Epoch 4 — Inside the canvas
- Upstream sync ahead of inside the canvas (source: intent §2; v040-02)
- Canvas draw commands (source: intent §6; v040-10; security-plan §Threat Model Summary — the provider crossing is the founder's fork)
- Canvas lint (source: intent §6; v040-10)
- Virtual nodes declared (source: intent §7; v040-11)
- Virtual nodes as elements (source: intent §7; v040-11)
- Virtual nodes in every read (source: intent §7; v040-11)
- Circle Drawer's circles as virtual nodes (source: intent §7, §10; v040-10, v040-11)

## Epoch 5 — Drag, pan and zoom
- Upstream sync ahead of drag, pan and zoom (source: intent §2; v040-02)
- The drag verb (source: intent §8; v040-12)
- Select list thumb dragged (source: intent §8; v040-12; res-ammkrycp)
- Pan and zoom under a transform (source: intent §9; v040-13)
- Node-graph stand app (source: intent §10; v040-14)

## Epoch 6 — Polish & ship
- Upstream sync ahead of polish and ship (source: intent §2; v040-02)
- Stand requirement sweep (source: intent §Principles; test-plan §10 decision/zero-failure-stand-repeat)
- A11y re-read (source: a11y-plan §9 decision/stand-checks-repeated-three-rounds; intent §7 roles and names)
- Capability statement (source: arch §Project Intent decision/capability-statement, decision/cold-agent-result-stated-measured; v040-03)
- Cold-agent bar (source: intent §11; v040-15)
- Cold-agent live run (source: intent §11; v040-15)

## Notes
- The head (build directory → upstream sync → README) runs unbroken across the Epoch 1/2 boundary. Foundation holds
  the build directory alone, so the sync can open Epoch 2 (intent §2: «an upstream-sync entry opens each epoch»)
  without a second sync near it. The placement is a proposal; the order beyond the head is the founder's (F2).
