@task_prose part=session
A documentation-only session: the README now states 0.3.0 closed and adds a 0.4.0 planned section, in six stages, with six epoch bullets named as the 0.4.0 working route names its epochs.
The README's Later line was restated to name what lies past 0.4.0: the demo project and heavy modern HTML (0.5.0), a GPU-painted canvas, a web Canvas 2D context for page scripts, opt-in OpenTelemetry export, the platform accessibility adapter witnessed with a real assistive technology, and the next MCP protocol revision.
The stack summary re-derived five stale version lines to the existing pins (selectors 0.43.0, skrifa 0.49, usvg 0.50.0, svgtypes 0.16.2) and dropped arboard, which no manifest names; the windowing row now reads clipboard text through winit.
The README's 0.1.0, 0.2.0 and 0.3.0 sections are unchanged; the capability statement and its check are unchanged; no code, test, dependency, schema or CI gate changed.
The founder's forks F1 and F2 stay unanswered and are carried onto the Capability statement stage of Epoch 6 (accessibility reach stated by hand, the README's 0.4.0 done), nothing written presuming an answer.
@task_file n=1 path=README.md
what: the project README, the user-facing statement of each version's status and what it builds — now a 0.4.0 planned section in six stages and a Later line of out-of-version items, among them the platform accessibility adapter witnessed with a real assistive technology
@task_file n=2 path=.claude/docs/stack.md
what: the stack summary, a harness render of the pinned library versions and each layer's role (selectors, skrifa, usvg, svgtypes; clipboard text through winit in the windowing row)
@task_item n=1 part=touches
The README's Later line now names the platform accessibility adapter witnessed with a real assistive technology as past 0.4.0; it sits beside the absence of a platform accessibility adapter in any stand binary (no active assistive technology given a tree) and the decision that the stand binaries build no platform adapter — consistent, check the wording of the stated reach.
@task_item n=2 part=touches
The same Later item bears on accesskit_xplat as the platform adapter and on accessibility reach at tier 0 (accessibility tier 0): the README now states publicly that a real assistive-technology witness is not in 0.4.0.
@task_item n=3 part=touches
Accessibility reach edited by hand: F1 and F2 are carried onto the 0.4.0 Capability statement stage (Epoch 6), where the capability statement's accessibility reach and the README's 0.4.0 done are written by hand; the README's earlier version sections and the capability statement were left as they were.
@task_item n=4 part=touches
The README's 0.4.0 section is a user-facing statement of planned scope, including any epoch that brings accessibility work; rows that state what accessibility reach the project claims publicly may need to stay in step with it.
@task_item n=1 part=untouched
The accessibility tree, role mapping, ARIA state and property reads, accessible-name precedence and hidden or inert content rules: no engine or tree code changed.
@task_item n=2 part=untouched
Keyboard navigation, focus, select list and range input contracts, keyboard activation and Tab order: no input or focus behaviour changed.
@task_item n=3 part=untouched
Contrast measurement, the lint's contrast finding, WCAG thresholds, stand accent colour and visual design rows: nothing measured or styled changed.
@task_item n=4 part=untouched
Snapshot, UI graph, actions answer, driver refusals, MCP results and screenshot contracts: no driver or snapshot surface changed.
@task_item n=5 part=untouched
The a11y CI leg, accessibility checks gating CI and stand checks repeated three rounds: no CI leg, gate or test changed.
@task_item n=6 part=untouched
Accessibility feature defaults and Dioxus accessibility feature naming: no manifest, feature or dependency changed; the version lines re-derived name no accessibility crate.
