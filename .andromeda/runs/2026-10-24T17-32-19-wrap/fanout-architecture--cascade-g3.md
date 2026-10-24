proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 181
    section: "## Standard Contracts"
    change: "the closing clause «a node both clipped or off-screen and holding text also reads `truncated`, asserted by no check (as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md)» becomes «a node both clipped or off-screen and holding text also reads `truncated`, asserted since 2026-10-24-stand-requirement-sweep by stand_lint's `a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it` in both layout modes on its in-file page `reading_page()` — `fx-cut-text` clipped and truncated, `fx-padded-text` clipped alone, `fx-off-text` truncated and off-screen — the `lint` verb answering the same (tests/blitz-tests/tests/stand_lint.rs:110-141; tests/blitz-tests/tests/stand_lint.rs:226-274) (as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md, the check at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md)»"
    sidecar: "Standard Contracts › Driver session (visual lint): the clipped-or-off-screen-and-truncated co-reading moves from «asserted by no check» to asserted by stand_lint in both layout modes, the lint verb answering the same."
    rationale: "Report Changes › Symbols (stand_lint.rs: reading_page 110-141, the test 226-274, both layout modes, the five findings, the lint verb answers the same) and Counts 4 (lint-coread: Moved `architecture-detail/standard-contracts.md:181`, «asserted by no check» → asserted by stand_lint)."
    basis: .andromeda/architecture-detail/standard-contracts.md:181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 256
    section: "## Standard Contracts"
    change: "in the `stand-repeat` leg's parenthesis, «running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs`, then …» becomes «running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` and, after that glob on the same line every round and since 2026-10-24-stand-requirement-sweep, ten named blitz-tests targets as literal `--test` words — `inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`, `transformed_client_rect`, `accessibility_active_descendant_unread`, `virtual_clock_engine`, `select_interaction` and `text_extent`, the files keeping their names, on the founder's word «Список в скрипте (Рекомендую)» (2026-10-24) (.github/scripts/ci-leg.sh:54-58) — then …», and «the script reads the floor script and the `stand_*.rs` set beside itself» becomes «the script reads the floor script and the `stand_*.rs` set beside itself and names the ten in its own text»; the leg's citation range in that parenthesis keeps its start and ends five lines later (five lines added at ci-leg.sh:54-58)"
    sidecar: "Standard Contracts › CI contracts: stand-repeat runs ten named blitz-tests targets as literal --test words after the stand_*.rs glob, by the founder's word of 2026-10-24; the leg's citation range ends five lines later."
    rationale: "Report Changes › Schema / config (ci-leg.sh stand-repeat 54-58, the ten `--test` words after the glob, same cargo line every round, the founder's word in a comment; the seven_guis half, --locked, no retry, no --ignored, no env var unchanged), Decisions › the founder's word (inputs#I1), Counts 1 (leg-select: Moved `architecture-detail/standard-contracts.md:256`, both clauses); task F1."
    basis: .andromeda/architecture-detail/standard-contracts.md:256
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 256
    section: "## Standard Contracts"
    change: "after the stand-repeat parenthesis's «three green rounds end `stand-repeat: 3 of 3 rounds green`», add «— its ten named `--test` words pinned by `test_stand_repeat_leg_runs_three_rounds` (.github/scripts/test_ci_workflows.py:359-362) and read from the script's own text by stand_requirement_sweep's `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` (tests/blitz-tests/tests/stand_requirement_sweep.rs:994-1069)»"
    sidecar: "Standard Contracts › CI contracts: test_stand_repeat_leg_runs_three_rounds pins stand-repeat's ten named --test words."
    rationale: "Report Changes › Files and Schema / config (`test_ci_workflows.py` `test_stand_repeat_leg_runs_three_rounds` (359-362) asserts the ten words), Symbols (the sweep test 994-1069 reads `--test {name}` for the names in the leg's text), Counts 2 (leg-pin); task F2."
    basis: .andromeda/architecture-detail/standard-contracts.md:256
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 256
    section: "## Standard Contracts"
    change: "the line's two other citations into the reached CI scripts move with the added lines: the leg runner's whole-script `.github/scripts/ci-leg.sh` range ends five lines later (five lines added at ci-leg.sh:54-58), and the `test_ci_workflows.py` range pinning the contract ends four lines later (four lines added at test_ci_workflows.py:359-362); every citation on the line ending at or before those added lines stays"
    sidecar: "Standard Contracts › CI contracts: the ci-leg.sh whole-script range and the test_ci_workflows.py pin range end five and four lines later."
    rationale: "Report New text, by line: ci-leg.sh added 54-58, test_ci_workflows.py added 359-362; both files reached whole, so a citation range running past the added lines moves (task Touches: ci-leg.sh past line 53 by five, test_ci_workflows.py past 358 by four)."
    basis: .andromeda/architecture-detail/standard-contracts.md:256
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 257
    section: "## Standard Contracts"
    change: "the apt-install contract's citation of `InstallScriptTest` and `test_o_package_installs_are_bounded` into `.github/scripts/test_ci_workflows.py` past the added lines moves four lines later (four lines added at test_ci_workflows.py:359-362); its earlier test_ci_workflows.py range and the apt-install.sh citations stay"
    sidecar: "Standard Contracts › CI contracts: the apt-install pin's later test_ci_workflows.py range moves four lines later."
    rationale: "Report New text, by line: test_ci_workflows.py added 359-362 — a reached file, so its range lying past those lines shifts by four (task Touches: test_ci_workflows.py past 358)."
    basis: .andromeda/architecture-detail/standard-contracts.md:257
note: "F4 (text_extent's three limits asserted) belongs beside standard-contracts.md:54 (Scrolling, selection, tree), which no group-3 entity carries — the new-detail pass places it; the shifted citation ranges are stated as offsets because the report lists only the added lines, not the new range ends."
