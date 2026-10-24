proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 41
    section: "## 6. Log Coverage"
    change: "In the escher-driver `command` bullet, after \"— `host_log` and `mcp_log` each drive one part read at `trace` with 0 needles;\", add: since 2026-10-24-stand-requirement-sweep a paged part read is in the sink capture too — `stand_act_spans`' `info` child holds the shared fixture page `many_items` (`ITEMS`, 400 items) and reads `snapshot` part 2 of ≥ 2 (`Outcome::Part`), its 400 `item-NNN` ids joining the id needles and the `#items > button` texts the name needles, 0 on a line, the child's calls 35 → 36 per mode (tests/blitz-tests/tests/stand_act_spans.rs:243; tests/blitz-tests/tests/stand_act_spans.rs:685-693; tests/blitz-tests/tests/stand_act_spans.rs:727; as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md);"
    sidecar: "log-coverage §6 escher-driver: the paged part read (part 2 of ≥ 2 of `many_items`) measured in `stand_act_spans`' info-child sink capture, 0 id and name needles, 36 calls per mode"
    rationale: "The report's Symbols (stand_act_spans.rs 685-693, 243, 727) and Count 6 add a paged part read to the in-process sink capture; Expected amendments name obs-plan §6 for 'the paged part read in the sink capture'. Count 6 keeps line 41's part-read line shape unchanged, so this is added detail, not a correction."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:53-56
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 41
    section: "## 6. Log Coverage"
    change: "In the same bullet, beside the host's part read, add the stated limit: the `escher-session` host's and the `mcp` role's part read is part 1 of a lean-task screen that does not page — the host boots only the four lean tasks — so a paged `part` read through the host's own sink stays unmeasured, owned by the `.andromeda/residuals.md` line «a paged `part` read through the escher-session host's sink» (target: next) (as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md, inputs#I2's card row 1)"
    sidecar: "log-coverage §6 escher-driver: the host-role paged part read stated as an unmeasured limit with its residuals owner"
    rationale: "Count 7 and inputs#I2's Beyond-the-plan row 1 make the host-role paged read a stated limit with an owner; Expected amendments carry it to obs-plan §6/§3. Line 41's 'host_log and mcp_log each drive one part read at trace with 0 needles' reads as covering a paged read unless qualified."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:158-161
note: D-obs-instrumentation, D-obs-stack and D-obs-pii find no drift here (test files and one CI leg only, no dependency, no log site; the clients' own stderr at trace reads 0 needles); the chunk's new tests' silence (census 0, gate entry 8) has no entry in the body's absence block at .andromeda/obs-plan.md:86-96, which is outside this file.
