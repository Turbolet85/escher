# Citation dispositions — architecture · part 3 of 3

changed .andromeda/architecture-detail/standard-contracts.md:190 packages/escher-driver/src/execute.rs:557-563 → ? — the start rewritten → claim false — raised at Validate (`focus_multiple_select` and `Target`'s `multiple` are gone: `focus_select` at 558-565 and `Target.select` take every select, not a `multiple` one alone, and it also closes any list that stands)
changed .andromeda/architecture-detail/standard-contracts.md:192 packages/escher-driver/src/schema.rs:261-263 → ? — both ends rewritten → holds — re-pointed by hand to 268-269
stretched .andromeda/architecture-detail/standard-contracts.md:195 packages/escher-driver/src/execute.rs:114-308 → 114-309 — changed inside → holds
stretched .andromeda/architecture-detail/standard-contracts.md:195 packages/escher-driver/src/execute.rs:362-602 → 363-604 — changed inside → holds
stretched .andromeda/architecture-detail/standard-contracts.md:211 packages/escher-driver/src/lint.rs:190-231 → 190-232 — changed inside → holds
changed .andromeda/architecture-detail/standard-contracts.md:213 packages/escher-driver/src/lint.rs:246-247 → ? — both ends rewritten → claim false — raised at Validate (`Entry.in_closed_select` is gone: the field is `in_select_box` at 247-249, and it also holds a row or a group of an open list)
changed .andromeda/architecture-detail/standard-contracts.md:213 packages/escher-driver/src/lint.rs:337-338 → ? — both ends rewritten → claim false — raised at Validate (`in_closed_select` no longer reads through `closed_select_of` alone: `in_select_box` at 339-342 reads it or `select_list_bounds`)
changed .andromeda/architecture-detail/standard-contracts.md:213 packages/escher-driver/src/lint.rs:347 → ? — rewritten → claim false — raised at Validate (the entry's field filled there is `in_select_box` at 351, no `in_closed_select` stands)
changed .andromeda/architecture-detail/standard-contracts.md:213 packages/escher-driver/src/lint.rs:364 → ? — rewritten → holds — re-pointed by hand to 368 (the earlier side's skip stands, on the field `in_select_box`)
changed .andromeda/architecture-detail/standard-contracts.md:213 packages/escher-driver/src/lint.rs:368 → ? — rewritten → holds — re-pointed by hand to 372 (the later side's skip stands, on the field `in_select_box`)
stretched .andromeda/architecture-detail/standard-contracts.md:263 packages/dioxus-native-dom/src/snapshot.rs:127-285 → 127-293 — changed inside → holds
stretched .andromeda/architecture-detail/standard-contracts.md:263 packages/dioxus-native-dom/src/lib.rs:34-38 → 34-38 — changed inside → holds
stretched .andromeda/architecture-detail/standard-contracts.md:279 packages/dioxus-native-dom/src/snapshot.rs:144-153 → 144-156 — changed inside → holds
stretched .andromeda/architecture-detail/standard-contracts.md:279 packages/dioxus-native-dom/src/snapshot.rs:241-250 → 244-258 — changed inside → holds
changed .andromeda/architecture-detail/standard-contracts.md:280 packages/dioxus-native-dom/src/snapshot.rs:242-252 → ? — the start rewritten → holds — re-pointed by hand to 245-260
stretched .andromeda/architecture-detail/standard-contracts.md:312 .github/scripts/test_ci_workflows.py:361-368 → 361-368 — changed inside → holds
stretched .andromeda/architecture-detail/standard-contracts.md:313 .github/scripts/ci-leg.sh:48-66 → 48-67 — changed inside → claim false — raised at Validate (the clause's count on line 312, fifteen named blitz-tests targets, is false: the leg names seventeen at 55-60, `select_popup` and `select_popup_painted` added)
stretched .andromeda/architecture-detail/standard-contracts.md:313 .github/scripts/ci-leg.sh:8-90 → 8-91 — changed inside → holds
stretched .andromeda/architecture-detail/standard-contracts.md:313 .github/scripts/test_ci_workflows.py:103-501 → 103-501 — changed inside → holds
