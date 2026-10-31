proposals:
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract"
    change: "In the bullet on the third shared module `tests/blitz-tests/tests/fixtures/mod.rs`, the module has 28 readers, not 24; the rest of the bullet stands."
    sidecar: "§3: the readers of `tests/blitz-tests/tests/fixtures/mod.rs` move from 24 to 28 (2026-10-31-engine-debts)."
    rationale: "The report's Counts / qualifiers moved bullet reads Fixture readers 24 → 28 (`evidence/census.md`) and lists `obs-plan.md:92` among `fixture-readers`' 3 changed hits; Expected amendments names the same site. The parenthetical beside the count names the chunk of the old measure; this count's measure is the census named as basis."
    basis: "escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md"
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract"
    change: "In the silent-checks passage, `stand_keyboard` drives the four lean stand tasks, the load page and Cells, and its own fixtures, not the four lean stand tasks and its own fixtures alone."
    sidecar: "§3: `stand_keyboard`'s stated reach gains the load page and Cells (2026-10-31-engine-debts)."
    rationale: "The report's Coverage of new surfaces names `stand_keyboard` as a test of the `cells-grid` Tab stop and keys and of the load page rows; its target count moves 15 → 21, and New text, by line lists `the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space` and `cells_walks_its_stated_sequence`. The passage's reach is stale."
    basis: "tests/blitz-tests/tests/stand_keyboard.rs:158"
  - detector: D-new-detail
    severity: warning
    section: "§3 Observability Harness Contract"
    file: new
    near: "- The upstream take read at `ebacd68bd` is silent:"
    change: "Add after that bullet, inside the same absence · logs group: - The 0.3.0 engine-debts checks are silent: the five new checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event`, each in both layout modes, hold no log site, print, env read, listener or wall-clock read — the silence census and the added-lines census each exit 1, last line 0, `packages/blitz-shell/src/event.rs` held out of the added-lines census, its addition an inline test; the engine changes the chunk pins (`visible_region` through transforms, `hit_inner`'s non-invertible answer, the focus fixup, tree focus, native `toggled`, the `multiple` select, `optgroup`, the engine's `change` on a checkbox and a radio) and the driver's `focus_multiple_select` and moved fixed texts add no log site, span, field, env read or listener, escher-driver keeping its 2 `info_span!` sites and `host_log` and `mcp_log` reading 0 needles under both builds (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md)."
    sidecar: "§3: new silence row for the five new checks and the engine and driver changes of 2026-10-31-engine-debts; no log site, span or field added."
    rationale: "The report's Expected amendments carries obs-plan §3's census of the new checks; Schema / config reads no log site, print, env read, listener, wall-clock read or panic path on the added lines (entry 4 at 0, `blitz-shell/src/event.rs` held out); Outcome's closed-sets criterion reads 2 `info_span!`, entry 4 at 0, and `host_log` and `mcp_log` 0 needles under both builds. §3's silence rows state each earlier chunk's checks and no category file holds this section."
    basis: "escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md"
note: "D-obs-instrumentation, D-obs-pii and D-retired read no drift: no new log site, span or metric, no raw user data logged, and no retired item named in obs-plan. D-obs-stack's two proposals are §3 record corrections, no off-spec logger: the only dependency move is `pulldown-cmark`. `focussed node`'s re-read belongs to §6's log-coverage.md and is the cascade's. Not this chunk's move, so not proposed: `obs-plan.md:76` says twenty-nine `session_common` readers where the report reads 31, unmoved."
