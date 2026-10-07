# Feature reach — `accessibility` named on `dioxus-native-dom` by `escher-driver`

Step 1's readings, taken by hand on the dev host (Linux) on 2026-10-07, once on the tree at
`3c58ce5c` and once after the one-line manifest edit
(`packages/escher-driver/Cargo.toml`: `dioxus-native-dom = { workspace = true, features = ["accessibility"] }`).
Workspace paths are written relative to the repository root.

## The five reads

| # | command | before | after |
|---|---|---|---|
| 1 | `cargo tree -p escher-driver -e features --locked -i dioxus-native-dom` | exit 0 · no feature line for `dioxus-native-dom` | exit 0 · `dioxus-native-dom feature "accessibility"` ← `escher-driver` |
| 2 | `cargo tree -p seven_guis -e normal --locked -i accesskit_xplat` | exit 101 · `did not match any packages` | exit 101 · `did not match any packages` |
| 3 | `cargo tree -p seven_guis -e normal --locked -i accesskit` | exit 0 · `accesskit v0.25.0` ← `blitz-dom` only | exit 0 · `accesskit v0.25.0` ← `blitz-dom` and `dioxus-native-dom` |
| 4 | `cargo tree -p seven_guis -e features --locked -i dioxus-native-dom` | exit 0 · no feature line for `dioxus-native-dom` | exit 0 · `dioxus-native-dom feature "accessibility"` ← `escher-driver` |
| 5 | `cargo tree -p seven_guis -e normal --locked -i accesskit_winit` | exit 101 · `did not match any packages` | exit 101 · `did not match any packages` |

Reads 1 and 2 are the two `cargo tree` entries of the plan's `## Test Commands`; read 3 is the
third reading step 1 names; reads 4 and 5 are the same question asked of the stand's feature graph
and of the winit adapter.

## Read 1 after the edit, whole

```text
dioxus-native-dom v0.7.0 (packages/dioxus-native-dom)
└── blitz-test-harness v0.3.0-beta.2 (packages/blitz-test-harness)
    └── blitz-test-harness feature "default"
        └── escher-driver v0.3.0-beta.2 (packages/escher-driver)
            └── escher-driver feature "default" (command-line)
└── dioxus-native-dom feature "accessibility"
    └── escher-driver v0.3.0-beta.2 (packages/escher-driver) (*)
```

## Read 4 after the edit, the lines the edit adds

```text
└── dioxus-native-dom feature "accessibility"
    └── escher-driver v0.3.0-beta.2 (packages/escher-driver) (*)
```

The stand's own features on `dioxus-native` are unchanged: `prelude`, `system-fonts`,
`vello-hybrid`, `woff`.

## Read 3 after the edit, the edge the edit adds

```text
accesskit v0.25.0
├── blitz-dom v0.3.0-beta.2 (packages/blitz-dom)
│   └── … (the 16 lines read before the edit, unchanged)
└── dioxus-native-dom v0.7.0 (packages/dioxus-native-dom) (*)
```

## What the readings measure

- The feature is on `dioxus-native-dom` in the driver's graph (read 1) and in the stand's (read 4),
  named by `escher-driver` in both. Predicted, measured.
- `accesskit 0.25.0` was already in both graphs through `blitz-dom`; the edit adds one edge to it —
  `dioxus-native-dom`'s own `dep:accesskit` — and no package. Predicted, measured.
- No platform adapter is in the stand's graph: `accesskit_xplat` and `accesskit_winit` each read
  `did not match any packages` at exit 101, before and after. Predicted, measured.
- `Cargo.lock` is byte-identical: sha256 `1e9a4587f631860f9af77e588d1abb4360b0a8d565b745888724dabdf1508ec2`
  before and after, and `git status --short Cargo.lock` prints nothing. Predicted, measured.

Not read here: any platform other than the dev host's. `cargo tree` resolves the host's target, so
the windows, macOS, iOS and android graphs are witnessed only by the fork's CI build legs.
