# arch extract

## Relevance
relevant — the chunk moves the headless stand's contract (its served tasks, its offline boot), the driver command line's boot over the stand's slugs and the cold-agent pipe's CI contract, and may touch provider wiring and the resource registry.

## Constraints
- The stand boots offline: `options(incremental)` hands `net_provider: None` (through `DummyNetProvider`), installs no telemetry, reads no env var and names no `blitz_net` (per architecture §Standard Contracts → Headless stand). A load in flight on the stand must enter within that contract or move it as a recorded amendment at wrap. Whether a held stand-in can live in example code without breaking "names no `blitz_net`" is research's question.
- Embedder services, a fetch included, reach the DOM only through provider traits the embedder injects, with no-op defaults (per architecture §Design Philosophy). A stand load in flight therefore comes through a `NetProvider` in `DocumentConfig`, never through an internal HTTP client (per architecture §Standard Contracts → Provider traits).
- `escher-session` hands `escher_driver::command_line` the stand's slugs and its boot, and every served session starts through `seven_guis::stand` only. Today that is four slugs (`counter` · `flight-booker` · `timer` · `crud`), and a task that `start` serves, such as `cells`, joins through that same boot (per architecture §Standard Contracts → CLIs). The driver library depends on no app, emits no event, reads no env var and installs no subscriber (per architecture §Established Decisions → Driver session).
- The driver schema stays as stated: twelve verbs, eight closed causes, every call through `Session::run`, no `NodeId` crossing, and no id, name or value logged (per architecture §Standard Contracts → Driver session). A new carrier or exit widens the boundary (per architecture §Established Decisions → Driver session).
- A new listener, port, env var or workspace crate is a registration in architecture §Occupied Resources, never a silent addition. The one listener is the session socket, and the `mcp` role has no listener (per architecture §Occupied Resources → Network ports and listeners). Only the spawned `claude` client reaches the model provider, on the operator-driven live run only, never in CI, and no escher code makes that request (per architecture §Occupied Resources → Outbound hosts).
- The select model is fixed for every document: the choice is held as its options' `CHECKED` state, a controlled app writes `selected` from its `input` event's value or the choice reverts, no popup is painted, an option is not clicked, and `multiple` is not modelled (per architecture §Established Decisions → DOM semantics). The flight booker's select is shaped to that model. A need beyond it is an engine change, so it surfaces as a fork.
- Every element an agent can act on reads an author key. `DioxusDocument::unkeyed_actionable` lists each one whose id holds a `/` (per architecture §Standard Contracts → Dioxus DOM bridge). This applies to a served `cells` and to the new select and its options.

## Patterns to follow
- Stand boot shape: `boot(LeanTask, HarnessOptions)` over `stand::options(true)`, with a task that needs time carrying its step through `Session::with_time`, as `boot_timer` and `timer_step` do. The boot builds a fresh `VirtualDom` and renders `app::task_in_shell` (per architecture §Standard Contracts → Headless stand).
- The held network stand-in `HeldNet` is stated once in `tests/blitz-tests/tests/fixtures/mod.rs`, beside the fixture boot and hold. Shared stand tables live in `tests/blitz-tests/tests/common/mod.rs`. Each check covers one behaviour and runs in both layout modes (per architecture §Conventions → Tests).
- The cold-agent pipe is `scripts/cold-agent.sh`, with `scripts/cold-agent.ps1` a pass-through. `.github/scripts/test_cold_agent.py` pins it with a `claude` shim and a fake `escher-session` (no live model, credential or built binary), and the existing `ci-scripts` leg runs it. Its full contract is test-plan §3 (per architecture §Standard Contracts → CI contracts).
- Dioxus stand components follow the example conventions: `#[component]`, signal and store hooks, reactive state through `#[derive(Store)]`, and CSS in `const` raw strings (per architecture §Conventions → Apps and examples).

## Anti-patterns to avoid
- No real network on the stand: no `blitz_net::Provider`, no outbound host, no env read and no default path to carry a load in flight (per architecture §Standard Contracts → Headless stand · per architecture §Occupied Resources → Environment variables).
- No new driver verb, argument kind, cause, session error, carrier, exit, log field or span to reach the new stand screens (per architecture §Standard Contracts → Driver session).
- No CI job or escher code reaching the model provider. The live run stays operator-driven on the dev host (per architecture §Occupied Resources → Outbound hosts).

## Contract bindings
- arch §Standard Contracts → CI contracts ↔ test-plan §3 (the cold-agent pipe's full contract). Today the verdict's positive evidence includes "all four lean apps met" from the tool's own `snapshot` answers, and `wrong_calls` is recorded but decides nothing. New goals and a founder-set wrong-call bar move both. Whether the code ties the verdict to the four lean apps is research's question.
- arch §Occupied Resources → Outbound hosts ↔ security-plan §Surfaces. The live run carries stand screens to the model provider, and that carriage was ratified for one run. New screens (Cells' contents, the select) ride that crossing.
- arch §Standard Contracts → Headless stand ↔ a11y-plan and test-plan. The flight booker's select moves the stand's Tab-order, activation and contrast tables and the `stand-repeat` targets that architecture §Standard Contracts → CI contracts names.
- arch §Occupied Resources → Filesystem ↔ security-plan hygiene: the cold-agent state area `target/cold-agent/` (transcript and server log, never printed) is local only and uploaded by no CI job.

## Acceptance criteria contributions
- Every task `escher-session start` and the `mcp` role's `start` serve boots through `seven_guis::stand`, which still installs no telemetry and reads no env var. Any load source is a `NetProvider` stand-in with no outbound host (per architecture §Standard Contracts → Headless stand).
- `VERBS` still holds twelve verbs and the causes eight. No env var, listener, port or workspace crate lands unregistered in architecture §Occupied Resources (per architecture §Standard Contracts → Driver session · per architecture §Occupied Resources → Network ports and listeners).
- `DioxusDocument::unkeyed_actionable` reads empty on every task the driver serves, the extended flight booker and any served `cells` included (per architecture §Standard Contracts → Dioxus DOM bridge).
- The ci-scripts leg is green with `test_cold_agent.py` pinning the extended pipe through its shim, with no live model or credential in CI (per architecture §Standard Contracts → CI contracts).
