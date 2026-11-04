@apply_row n=1 ref_file=fanout-security-plan--cascade.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-security-plan--cascade.md ref_n=3 v=folded_into to_file=fanout-security-plan--cascade.md to_n=1
@apply_row n=3 ref_file=fanout-security-plan--cascade.md ref_n=5 v=folded_into to_file=fanout-security-plan--cascade.md to_n=1
@apply_row n=4 ref_file=fanout-security-plan--cascade.md ref_n=2 v=folded_into to_file=fanout-security-plan--cascade.md to_n=1
@apply_row n=5 ref_file=fanout-security-plan--cascade.md ref_n=4 v=applied edit=E2
@apply_row n=6 ref_file=fanout-security-plan--cascade.md ref_n=6 v=applied edit=E3
@apply_row n=7 ref_file=fanout-security-plan--cascade.md ref_n=7 v=folded_into to_file=fanout-security-plan--cascade.md to_n=6
@fact id=fct-a4qh6ioi expect_rev=1
old: Eight ignores stand.
new: Nine ignores stand.
old: leaves paste, owned by a `.andromeda/residuals.md` line (target: next)
new: leaves paste, owned by a `residual` row in the store (target: next)
old: a measured limit owned by a `.andromeda/residuals.md` line (target: next)
new: a measured limit owned by a `residual` row in the store (target: next)
old: RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369 (zip 2.4.2, each fixed in zip 9.0), since 2026-10-16-fast-failing-mutants,
new: RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369, since 2026-10-16-fast-failing-mutants, and RUSTSEC-2026-0429, `ZipArchive::extract` aborting the process on a symlink entry with a large declared size (zip 2.4.2, each fixed only in zip 9.0),
old: which reads archive entries `by_name` into memory and never extracts to disk;
new: whose `deserialize` opens a `ZipArchive` and reads entries `by_name` into memory and never calls `extract`;
old: (deny.toml:23-25) on the founder's
new: (deny.toml:23-25), the three on the founder's
old: and the ignores read green (CI#39014589486);
new: and the ignores read green (CI#39014589486), and RUSTSEC-2026-0429 with RUSTSEC-2026-0433 turned it red (CI#41911351081) over a lock the green CI#41346357110 read, its ignore reading green (CI#41914044942);
old: owned by a `.andromeda/residuals.md` line (target: next; `.andromeda/residuals.md:29`)
new: owned by the store's `residual` row `res-h7acz56c` (target: next)
old: owned by a `.andromeda/residuals.md` line (target: next; `.andromeda/residuals.md:30`)
new: owned by a `residual` row in the store (target: next)
old: as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md.
new: as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md. The lock resolves pulldown-cmark 0.13.6 by a `Cargo.lock`-only update, semver-compatible with no coupled pin and reached only through `rdme`, fixing RUSTSEC-2026-0433 (quadratic parsing of nested image wikilinks under `ENABLE_WIKILINKS`, which `rdme` never sets); the audit leg reads `advisories ok` and `licenses ok` locally and in CI#41914044942 — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md.

@fact after=fct-a4qh6ioi file=dependency-security gid=new:audit-published-advisories kind=values subject=deps
- **Audit tool, advisories published after a green audit:** a CI Dependency audit can go red on advisories published after the base's green audit with `Cargo.lock` and `deny.toml` unchanged (CI#41911351081 against the green CI#41346357110 on the same graph), and the local `fast` leg runs no audit — `bash .github/scripts/ci-leg.sh audit` is the local reproduction to run before a push — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@fact id=fct-7ni72vcj expect_rev=1
old: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in that task panics with `RefCell already borrowed` (packages/dioxus-native-dom/src/events.rs:162), any app able to meet it — measured headlessly through the harness's `poll(None)`, the windowed shell running the same lines, read and not run;
new: `DioxusDocument::poll` holds `inner.borrow_mut()` (packages/dioxus-native-dom/src/dioxus_document.rs:295) across `vdom.render_immediate` (packages/dioxus-native-dom/src/dioxus_document.rs:297), where dioxus-core 0.7.10 first polls a task spawned from a key handler that wrote a signal (`handle_task_wakeup`, dioxus-core `virtual_dom.rs:611`), so that task's `set_focus` (packages/dioxus-native-dom/src/events.rs:295), borrowing through `doc_mut` (packages/dioxus-native-dom/src/events.rs:162), panics `RefCell already borrowed`, any app able to meet it — measured headlessly in both layout modes and pinned by `dioxus_runtime_probe`'s two `should_panic` tests, one per layout mode, its fix routed to the route entry «MountedData buffered render», the windowed path read, not run; the same handler calling `set_focus` at once with no task moves focus and does not panic; `get_scroll_offset` / `get_scroll_size` take a shared borrow through `doc` (packages/dioxus-native-dom/src/events.rs:152), read from the code, not run;
old: — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
new: — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md, the Cells sentence at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md

@pattern id=sec-01
fixed: owned by a `.andromeda/residuals.md` line
@pattern id=sec-02 ignore_case=true
fixed: Eight ignores stand
@pattern id=sec-03
fixed: RUSTSEC-2026-0368 and RUSTSEC-2026-0369
