@proposal n=1 detector=D-cascade severity=warning file=body anchor=fct-jydhmppz basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-uzhqu5vc
section: ## 1. Test Scope Summary
change: dioxus-native-dom's unit-test list gains the reached record (the new ungated `reached` module, its `reached::tests` beside `bridge_tests`), after «the bridge and its dispatch record», with the rest of the line unchanged
sidecar: test-plan §1 dioxus-native-dom scope line names the reached record's unit tests (2026-11-08-handler-coverage)
rationale: the report adds the ungated module dioxus_native_dom::reached (fct-z4cdnfit) and states it is unit-tested by reached::tests (4) and bridge_tests +1 (fct-uzhqu5vc), but §1's per-crate list of what dioxus-native-dom's unit tests cover does not name it
@remark
F1-F9 fit no picked test-scope-summary entity (their slugs are integration/e2e/unit-test-strategy and test-data-fixtures), so the new-detail pass places them; the stand count fct-glytb3w3 and fct-6bs5asmb (sec-3mx24yx3, 244 over 52 -> 262 over 54) are harness-section facts and not this group's
