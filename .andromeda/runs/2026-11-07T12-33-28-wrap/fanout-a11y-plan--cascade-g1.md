@proposal n=1 detector=D-retired severity=warning file=keyboard-navigation anchor=fct-gmhtpdel basis_doc=doc:2026-11-07-mounteddata-buffered-render/report basis_fact=fct-lzssryef
section: ## 5. Keyboard Navigation
change: Re-say the hazard record closed: a `MountedData` call in a task spawned from a key handler that writes a signal acts and does not panic, since 2026-11-07-mounteddata-buffered-render, on every Dioxus document — `DioxusDocument::initial_build` and `Document::poll` render into a mutation buffer (`buffered_mutations`, a `MutationBuffer`) with the document unborrowed, then take `inner.borrow_mut()` once and replay it through one `MutationWriter`, so the task dioxus-core 0.7.10 first polls inside `render_immediate` reaches the document through `doc_mut`; before it `poll` held `inner.borrow_mut()` across `render_immediate` and the task's `set_focus` panicked with `RefCell already borrowed`; measured headlessly in both layout modes by `dioxus_runtime_probe`'s two pins `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}`, their precondition asserted first, each asserting after the spawned task's `set_focus` that the engine's focus, the snapshot's one `focused` node and the tree's focus name `focus-target`, both reading FAILED with the borrowed render restored; the windowed path read and not run; Cells still calls its handles synchronously, each acting at the call, no longer needed for the hazard and kept — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md, Cells' synchronous calls at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
sidecar: keyboard-navigation fct-gmhtpdel — the spawned-task MountedData hazard re-said fixed by the buffered render; the two should_panic pins replaced by the two …_spawned_acts_* pins asserting engine, snapshot and tree focus agree
rationale: the report's Retired (fct-lzssryef) retires the panic and the two should_panic pins; Symbols / APIs (fct-u4yklhu2) states the buffered render and the pins' assertions; Counts / qualifiers moved (fct-lz64wsmm) lists fct-gmhtpdel under hazard, buffer, title and pins, changed, with the synchronous Cells calls kept; Spec claims disproved (fct-jkkd7b4q) shows the pins discriminating
@proposal n=2 detector=D-new-detail severity=warning file=keyboard-navigation anchor=fct-gmhtpdel fact=F1 basis_doc=doc:2026-11-07-mounteddata-buffered-render/report basis_fact=fct-u4yklhu2
section: ## 5. Keyboard Navigation
change: Add after it: - The limit, by construction: a task polled during a Dioxus render acts on the document as the previous render left it, so `MountedData::set_focus` on a node created in the same render does not focus it (`set_focus_to` returns `false`); the probe's pins act on nodes mounted by an earlier render; the limit is owned by an open residual row (target: next) — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md
sidecar: keyboard-navigation — added the buffered render's limit: a node created in the same render takes no set_focus from a task polled during it, owned by an open residual row (target: next)
rationale: the report's Symbols / APIs «Limit, by construction» (fct-u4yklhu2) states it, and Addendum 1 (fct-zxiziztw, inputs#I4 item 4) records it as an open residual row, target next
@proposal n=3 detector=D-cascade severity=warning file=a11y-strategy anchor=fct-p7eqwwyx basis_doc=doc:2026-11-07-mounteddata-buffered-render/report basis_fact=fct-zi6q26ih
section: ## 2. A11y Strategy
change: The owner clause reads «… and is owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), the witness at an active assistive technology, which this version does not reach»; the rest unchanged
sidecar: a11y-strategy fct-p7eqwwyx — the res-llkgqygv owner clause re-said to its row's status, re-carried at 0.3.0 (target: next)
rationale: the report's Addendum (fct-zi6q26ih) lists fct-p7eqwwyx among res-llkgqygv's 17 sites, each naming the residual with no status, now owned by residual res-llkgqygv, re-carried at 0.3.0 (target: next)
@proposal n=4 detector=D-cascade severity=warning file=a11y-strategy anchor=fct-ngdl2hcv basis_doc=doc:2026-11-07-mounteddata-buffered-render/report basis_fact=fct-zi6q26ih
section: ## 2. A11y Strategy
change: The owner clause reads «… the windowed witness, at an active assistive technology, is owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), which this version does not reach …»; the rest unchanged
sidecar: a11y-strategy fct-ngdl2hcv — the res-llkgqygv owner clause re-said to its row's status, re-carried at 0.3.0 (target: next)
rationale: the report's Addendum (fct-zi6q26ih) lists fct-ngdl2hcv among res-llkgqygv's 17 sites, now owned by residual res-llkgqygv, re-carried at 0.3.0 (target: next)
@proposal n=5 detector=D-cascade severity=warning file=body anchor=fct-zyrpeqlm dependent_of=D-cascade basis_doc=doc:2026-11-07-mounteddata-buffered-render/report basis_fact=fct-zi6q26ih
section: ## 2. A11y Strategy
change: The closing clause reads «… its windowed witness at the platform adapter owed — owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), an active assistive technology this version does not reach.»
sidecar: a11y-plan §2 fct-zyrpeqlm — the res-llkgqygv owner re-said re-carried at 0.3.0 (target: next)
rationale: the body's broad line of a11y-strategy#1.1, the same res-llkgqygv site listed in the report's Addendum (fct-zi6q26ih)
@proposal n=6 detector=D-cascade severity=warning file=body anchor=fct-cixbakze dependent_of=D-cascade basis_doc=doc:2026-11-07-mounteddata-buffered-render/report basis_fact=fct-zi6q26ih
section: ## 2. A11y Strategy
change: The closing clause reads «… — the windowed AT witness owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), an active assistive technology this version does not reach.»
sidecar: a11y-plan §2 fct-cixbakze — the res-llkgqygv owner re-said re-carried at 0.3.0 (target: next)
rationale: the body's broad line of a11y-strategy#2.3, the same res-llkgqygv site listed in the report's Addendum (fct-zi6q26ih)
@proposal n=7 detector=D-cascade severity=warning file=body anchor=fct-5i3imozy basis_doc=doc:2026-11-07-mounteddata-buffered-render/report basis_fact=fct-zi6q26ih
section: ## 5. Keyboard Navigation
change: The closing clause reads «… role and cell semantics stay out, owned by residual `res-wvatzhpe`, re-carried at 0.3.0 (target: next).»
sidecar: a11y-plan §5 fct-5i3imozy — the res-wvatzhpe owner re-said re-carried at 0.3.0 (target: next)
rationale: the report's Addendum (fct-zi6q26ih) lists fct-5i3imozy among res-wvatzhpe's 5 sites, now owned by residual res-wvatzhpe, re-carried at 0.3.0 (target: next)
@remark
No new interactive UI element (Coverage of new surfaces: a11y n/a), so D-a11y-surface and D-a11y-obs-schema find no drift; the Addendum's other a11y-plan sites (fct-kbjv4usk for res-llkgqygv, fct-pofw2owv for res-wvatzhpe, whose «line 24» goes) were not among group 1's picks and need placing by the pass that holds them.
