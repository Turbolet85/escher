# Check 4 (9) control — gate entry 7, the lock guard

The entry fired as itself through `gate.py run --chunk 2026-11-24-canvas-draw-commands --docs-root . --entry 7
--project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/77fb74c9-8f62-4423-845d-e5dfeb2dc9b3/scratchpad/lockctl`,
a `git clone --shared` of the project at HEAD `8520d77438661329b8a5e051589341b453d5e998`, no `--run-dir`.

| clone input | line read | verdict |
|---|---|---|
| unmodified (as on the real tree) | `7 probe green · exit 1 · 0.0s · 2 B` | green, as the baseline on the tree |
| must pass: `"anyrender",` and `"peniko",` added to escher-driver's `dependencies` list in `Cargo.lock` | `7 probe green · exit 1 · 0.01s · 2 B` | green |
| must fail: a `[[package]]` block `name = "control-probe"` · `version = "0.0.1"` appended to `Cargo.lock` | `7 probe red · exit 1 ✗ (exit 0) · exit 0 · 0.01s · 2 B` | red |

The guard reads a dependency-list edit green and an added package red.
