# P5 review — 2026-11-04-dioxus-runtime-probe «Dioxus runtime probe»

Open the plan in an editor at `.andromeda/cache/docs/2026-11-04-dioxus-runtime-probe/plan.md`. It is a render of
`doc:2026-11-04-dioxus-runtime-probe/plan` (rev 3): no plan stands in the chunk's folder, and writing in that file
changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-04-dioxus-runtime-probe` shows a diff of the chunk's records.

## Goal
One kept check, `tests/blitz-tests/tests/dioxus_runtime_probe.rs`, measures on the headless stand what the Dioxus 0.7
runtime and the bridge expose of the UI graph's four links, with no fork-level change, and pins the MountedData hazard
as it stands at HEAD. One document for the founder, `evidence/probe-reading.md`, gives each link's verdict with its
evidence, the graph's cost class, the hazard, his answer and the forks «The UI graph» needs. No fix lands, and nothing
under `packages/`, `examples/` or the manifests moves.

## The verdicts the plan expects to measure
- **Link 1, node → component: exposed.** dioxus-core's public vdom walk (`VComponent::mounted_scope` · `VComponent.name`) and the bridge's public `vdom` / `vdom_state.try_element_to_node_id`.
- **Link 2, node → handlers: exposed.** These are the bridge's listener attributes (event name = `<rust func>`, plus `data-dioxus-id`). Cells' 677 reacting elements agree with this. Predicted limit, to be measured: a removed listener's attribute stays. The per-kind counts are `pub(crate)`.
- **Links 3 and 4.** Exposed for a signal the reader holds (`Readable::subscribers()` → `origin_scope()`), and as the handler's effect (the snapshot diff). For the stand's hook-local state: **not exposed without a fork-level change** (`Scope.hooks` is `pub(crate)`), read from the code and not run. The «exposed with a cost» route (an escher-side signal registry) is named, not built.
- **MountedData:** `#[should_panic(expected = "RefCell already borrowed")]`, both modes, its precondition asserted first, beside a synchronous contrast that acts. The port chunk inverts it.

## Top acceptance criteria
- The probe check passes in both layout modes, and `agent-run.sh run dioxus_runtime_probe` exits 0.
- Links 1, 2, 3 and 4 measured as above; the hazard pinned; no fix applied.
- Silence census 0; nothing under `packages/` / `examples/` / manifests / `Cargo.lock` / `deny.toml` moved, so the twelve verbs, thirteen node keys, eight causes and fifteen session errors stand.
- `ci-leg.sh fast` and `doc` pass; `evidence/probe-reading.md` and `evidence/counts.md` stand.

## Touchpoints
- New: `tests/blitz-tests/tests/dioxus_runtime_probe.rs`. Modified: none.

## Test commands (10 entries)
1 `cargo test -p blitz-tests --locked --test dioxus_runtime_probe` (new, baseline red: no target yet) · 2 silence census
grep (new, baseline red: file absent; clone controls green/red) · 3 preservation guard against base `fd2e99389` (new,
baseline green; clone controls red on a `packages/` edit and an untracked `examples/` file) · 4 `agent-run.sh boot` (seen
green) · 5 `agent-run.sh run dioxus_runtime_probe` (new, baseline red: usage) · 6 `ci-leg.sh fast` (seen green) · 7
`ci-leg.sh doc` (seen green) · 8 hygiene · 9 push · 10 `ci.py conclusion --wait 1800` (operator legs).

## Capabilities
None claimed. The pool's one candidate, v030-05, names the graph itself in its requirement, and «The UI graph» lists
v030-05 too: not provable by this chunk. After the word, one dated `notes` line on v030-05 records that this chunk
proved its «probe … reached the founder» half.

## The founder's answer, already given
«Порт, отдельный чанк (Рекомендую)» (by dialog in the overseer session, relayed; `inputs#I2`): port upstream's buffered
render onto the 0.7 bridge in a chunk of its own, after this probe and before «The UI graph».

## Sampler
- architecture: sampler: E carried 60 · the cascade lost 9 (conventions#2.1, conventions#5.1, conventions#6.1, cross-cutting-patterns#7.1, existing-scopes#1.22, inherited-defaults#3.1, stack-and-technologies#2.1, standard-contracts#29.6, standard-contracts#5.1)
- security-plan: sampler: E carried 30 · the cascade lost 6 (data-protection#1.1, dependency-security#2.1, input-validation#1.65, logging-monitoring#4.1, threat-model-summary#4.1, threat-model-summary#5.1)
- design-system: sampler: E carried 5 · the cascade lost 3 (motion#1.5, motion#4.3, typography#2.2)
- test-plan: sampler: E carried 40 · the cascade lost 11 (ci-integration#4.1, ci-integration#8.1, e2e-test-strategy#1.1, e2e-test-strategy#1.3, e2e-test-strategy#4.2, integration-test-strategy#5.1, mocking-stubbing-discipline#1.7, test-data-fixtures#3.1, test-data-fixtures#3.6, test-scope-summary#2.1, test-scope-summary#24.1)
- obs-plan: sampler: E carried 25 · the cascade lost 7 (ci-integration#1.3, ci-integration#1.6, ci-integration#1.9, metric-coverage#1.1, metric-coverage#1.9, pii-scrubbing-compliance#3.1, telemetry-strategy#3.1)
- a11y-plan: sampler: E carried 24 · the cascade lost 5 (cognitive-accessibility#1.2, keyboard-navigation#4.11, keyboard-navigation#5.8, visual-design-verification#1.4, visual-design-verification#1.8)
- The plan carries 11 of the 41 lost items as `(sampler)` cites, in its steps, constraints and notes. The other 30 are no-change bounds the plan's guard and census already hold (no dependency, crate, span, field, carrier or committed check added).

## Beyond the plan
| # | Proposal | Kind | Where |
|---|---|---|---|
| 1 | New working entry «MountedData buffered render»: port upstream `286874cc1`'s buffered render (render with the document unborrowed, then apply) onto the Dioxus 0.7 bridge, upstream's test `task_polled_during_render_can_access_document` its witness, inverting `dioxus_runtime_probe`'s hazard pin. An engine change for every Dioxus document, on the founder's word (`inputs#I2`). | route insertion | Epoch 3 — The UI graph, after «Dioxus runtime probe», before «The UI graph» |
| 2 | A `BLOCKED-ON:` on «The UI graph»: the founder's answer to the forks of this chunk's `evidence/probe-reading.md` (the operator's directive at take-up, `inputs#I1`: the reading reaches the founder before the UI graph is planned). | route annotation | the existing entry «The UI graph — per screen: node, component, handlers, written state, its readers; "reacts to a click" reading; values masked, never logged (v030-05)» |

Apply? (yes / review / cancel)
