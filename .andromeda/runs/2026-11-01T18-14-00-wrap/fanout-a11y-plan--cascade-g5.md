proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 13
    section: "## 9. CI Integration"
    change: "Keep the line and end it with a new sentence, Since 2026-11-01-upstream-sync-ui-graph `select_popup_painted` also runs as `embolden`, the last step of the `fast` leg (fmt, clippy, test, ci-scripts, embolden), by `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, with no CI job, macOS exercising the feature there; the a11y leg's fourteen files are unmoved and it stays outside them."
    sidecar: "ci-integration: `select_popup_painted` also runs as `embolden`, the last step of `fast`, beside the workspace `test` and `stand-repeat` legs the line names."
    rationale: "Report Harness / gate surface adds the leg `embolden`, in `LEGS` and last in `FAST_LEGS` (`ci-leg.sh:8-9`, `:28-29`), no CI job; Counts `fast-leg` reads fmt, clippy, test, ci-scripts, embolden; the line lists this list witness's legs and now omits one."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:64
