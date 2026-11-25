# Baseline controls — 2026-11-25-virtual-nodes-in-every-read (phase P5, check 4 (9))

Entry 1 (the manifest and lock probe, base `b0e2d1c31f543d77c6d1e6ba9f8c27f6e8d28620`), fired as the entry through
`gate.py run --entry 1 --project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/0c1d3523-0a76-447a-b21f-5f56554edaf1/scratchpad/probe-clone`
over a `git clone --shared` of the project at HEAD `b0e2d1c31f543d77c6d1e6ba9f8c27f6e8d28620`:

- the clone unmodified: `1 probe green · exit 0` — what the real tree read;
- input it must pass, a line appended to `packages/escher-driver/src/actions.rs`: `1 probe green · exit 0`;
- input it must fail, a line appended to `packages/escher-driver/Cargo.toml` as well: `1 probe red · exit 0 ✗ (exit 1)`.
