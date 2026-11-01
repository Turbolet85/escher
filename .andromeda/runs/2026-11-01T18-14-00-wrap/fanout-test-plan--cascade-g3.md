proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 270
    section: "## 1. Test Scope Summary"
    change: "In the `stand_cells_served` clause, `stand::boot_cells` now reads a screen of 47,089 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`, in place of 47,949 bytes; the rest of the line stands."
    sidecar: "test-scope-summary: the served Cells screen's stale 47,949 bytes corrected to the measured 47,089 bytes, 740 lines and 5 parts unmoved."
    rationale: "Report, Spec claims disproved by measurement, `cells-bytes`: `test-scope-summary.md:270` states 47,949 flat; measured through the binary on a scratch clone at the base and on this tree, 47,089 bytes, 740 lines, 5 parts, so the take moves nothing and the reading is stale since an earlier chunk (`evidence/counts.md`); disposition, amend all three."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:270
note: "#37.1's add-to names no F{n} and no fact of F1-F7 fits either picked entity, so those seven go to the new-detail pass; the same Cells reading's other sites, test-data-fixtures.md:26 and architecture's existing-scopes.md:137, and the quarantine's two new lines, read by §7 and §4, lie outside this group."
