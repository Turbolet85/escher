@carried n=1 label=architecture/project-intent fact=fct-4ncs7geh
why: The probe must read the Dioxus runtime at the pinned 0.7.3 from the registry, with no move to 0.8.0-alpha.3 and no git dependency, because that move waits for escher's next version.
quote: Dioxus 0.8.0-alpha.3 (`286874cc1`), which wait for escher's next version, owned by a `.andromeda/residuals.md` line (target: next), so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3", with no git dependency
@carried n=2 label=architecture fact=fct-7lmrueug
why: The body's Upstream sync line holds the same pin, so every verdict the probe gives must be stated against Dioxus 0.7.3 and must never rest on a later Dioxus release.
quote: git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version, so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3".
@carried n=3 label=architecture fact=fct-uk5d6zx7
why: The probe serves the agent-first framing, so its readings must be measured on the 7GUIs stand that proves escher, and they must stay within the headless driver, stable ids and snapshot surfaces the UI graph will extend.
quote: escher is a fork of Blitz being turned into an agent-first UI framework — a headless driver (CLI + MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand.
@binding n=1
**Dioxus pin ↔ the chunk's no-fork boundary**: the scope's «no pin move — Dioxus stays 0.7.3» matches the category's pin (architecture/project-intent fct-4ncs7geh) and the body's Upstream sync line (architecture fct-7lmrueug), held in the body's Inherited Defaults too (architecture fct-656exhoj); a link that would need Dioxus 0.8 or a git Dioxus must read «not exposed without a fork-level change».
@binding n=2
**Agent-first framing ↔ the probe's measured verdicts**: the stand that proves escher (architecture fct-uk5d6zx7) is where each verdict must be measured, headlessly, on a lean task and a served task, in line with the body's headless and testable principle (architecture fct-wromimpb).
