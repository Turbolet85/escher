proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (subscriber installation)"
    change: "Add to the headless-stand silence census: since 2026-10-18-engine-debts the five new blitz-tests checks `accessibility_merged_states`, `inert_subtree`, `accessibility_active_descendant_unread`, `transformed_client_rect` and the driver check `stand_act_transformed` (a `session_common` reader), and the five rewritten `stand_keyboard`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box` and `password_caret_click`, each install no subscriber, read no env var and print nothing, their readings going into assertions only, the silence probe reading 0 over the ten and the shared module `common/mod.rs`; the chunk's engine changes (the `inert` read, the checkable-role `aria-checked` toggle read, the password click caret, the transformed client rect, `Node::absolute_position`, the `contain: paint` hit stop, `ElementData::is_disabled`) and the driver's reworded fixed texts add no log site, span, field, log field or env read (as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md)."
    sidecar: "obs-plan §3 Logging stack: the silence census gains 2026-10-18-engine-debts' five new and five rewritten checks (probe 0) and states the seven engine fixes and the driver's reworded texts add no log site, span or field."
    rationale: "The report lists five new and five rewritten checks (Files, report.md:17-18), states no new log field, span or env read with the census and silence probes both reading 0 (report.md:97, the silence-probe gate report.md:421, Outcome (obs) met report.md:397), and names obs-plan §3's silence census as the expected amendment (report.md:288-290); obs-plan §3's census today ends at earlier chunks' checks and names none of the ten as measured silent at this chunk."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:288-290"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (subscriber installation)"
    change: "«for the thirteen driver-action checks `stand_act_*` (…; the thirteenth, `stand_act_screenshot`, since 2026-10-12-headless-screenshot)» → «for the fourteen driver-action checks `stand_act_*` (…; the thirteenth, `stand_act_screenshot`, since 2026-10-12-headless-screenshot; the fourteenth, `stand_act_transformed`, since 2026-10-18-engine-debts)»."
    sidecar: "obs-plan §3 Logging stack: the census's driver-action checks thirteen → fourteen, `stand_act_transformed` the fourteenth (2026-10-18-engine-debts)."
    rationale: "The report moves `stand_act_*` files 13 → 14 (`ls …/stand_act_*.rs | wc -l` → 14) and lists obs-plan.md:70 as a stale row of pattern `stand-act-13` (report.md:118-121)."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:118-121"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (subscriber installation)"
    change: "«and for the module twenty-one checks read — four of the five `stand_session_*`, `stand_settle`, the thirteen `stand_act_*`, …» → «and for the module twenty-two checks read — four of the five `stand_session_*`, `stand_settle`, the fourteen `stand_act_*` (`stand_act_transformed` the twenty-second reader, since 2026-10-18-engine-debts), …» (`tests/blitz-tests/tests/session_common/mod.rs`)."
    sidecar: "obs-plan §3 Logging stack: `session_common` readers twenty-one → twenty-two and its `stand_act_*` thirteen → fourteen (2026-10-18-engine-debts)."
    rationale: "The report moves the `session_common` readers 21 → 22 (`grep -l '^mod session_common;'` → 22, report.md:116-117) with `stand_act_transformed` a `session_common` reader (report.md:222-224) and `stand_act_*` 13 → 14 (report.md:118-121); its `sc-readers` sweep named only test-plan.md:101 stale, missing this second statement of the reader count on obs-plan.md:70 worded «the module twenty-one checks read»."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:116-121"
    dependent-of: D-obs-instrumentation
note: "D-obs-stack and D-obs-pii read clean (no dependency added or bumped, report.md:100-101; no log site, field, log field or span added, report.md:97). The report does not say whether `stand_act_transformed` reads `tests/blitz-tests/tests/common/mod.rs`, so obs-plan.md:70's «21 of which read the shared module» count was left unproposed — unmeasured by the report. obs-plan §6 / §8 cite `packages/blitz-dom/src/events/pointer.rs:893-899`, `:894`, `:899` for the link log sites, and this chunk added 93 lines in pointer.rs, all above line 893 by the New-text ranges — the citation pass's to re-point, no claim moved."
