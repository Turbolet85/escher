@leaf_group group=architecture+security-plan+test-plan+obs-plan
@leaf_read leaf=CLAUDE.md lines=164
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=122 class=omission
leaf-text: no verb, role, argument, MCP tool, session socket call or kept check reaches it).
source: ctr-5uc2ynp6 «contract/driver-session/layout · escher-driver session · Session::layout — An in-process read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId; it resolves its id»
replace-with: no verb, role, argument, MCP tool, session socket call or kept check reaches it), and the layout explanation (built — `Session::layout`, an in-process read answering a `LayoutExplanation` of stable id strings, closed words and numbers, never a `NodeId`, from blitz-dom's read-only `BaseDocument::layout_reason` — the placing word, container, offset, shifts and a size reason per axis with its cause, `None` for an element with no box; it resolves its id as the run path does, refusing `stale` or `not-found`, runs no step, pump, tick or settle and opens no span; no verb, MCP tool, session socket call or kept check reaches it).
@leaf_edit n=2 leaf=CLAUDE.md line=36 class=omission
leaf-text: handler coverage and auto-exploration (`explore`, in process alone)
source: dec-j32bi6cn «decision/layout-read-in-process-only · Layout read stays in process — No verb, MCP tool, session socket call or kept check reaches the driver's layout read; it is answered in process only.»
replace-with: handler coverage, auto-exploration (`explore`, in process alone) and the layout read (`Session::layout`, in process alone)
@leaf_edit n=3 leaf=CLAUDE.md line=21 class=omission
leaf-text: the form controls' models and the document's clock
source: ctr-qixddwqw «contract/base-document/layout-reason · BaseDocument · layout_reason — Public, read-only: answers a LayoutReason (placing word, container, offset, shifts, a size reason per axis with its cause)»
replace-with: the form controls' models, the document's clock and the read-only layout reason (`layout_reason`)
