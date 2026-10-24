# Task — arch · 2026-10-24-a11y-re-read

## The chunk
A re-read of the stand's accessibility checks after 0.2.0's engine changes, with no engine change: the Tab sequences at boot and in enabled states, Enter and Space on each stand button, and every contrast row are read again in both layout modes.
A sequence or row that 0.2.0 moved is re-recorded with the change named. A stand control that 0.2.0 made keyboard-operable and that no row asserts gets its row; the candidate is the flight booker's single-choice select (arrow keys, Home, End, typeahead).
The `a11y` CI leg is widened from its six test targets to the 0.2.0 checks that witness an a11y requirement (accessibility states, inert subtree, labels, descriptions, merged states, active descendant, select interaction). The leg's pin in the CI-script tests moves with it, and the job keeps its tier.
A defect the re-read finds that needs engine behaviour changed for every document goes to the founder as a fork. No carrier, verb, field, log field, span, listener, env read or crate is added.

## Reached files
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/stand_contrast.rs
- .github/scripts/ci-leg.sh · merge surface
- .github/scripts/test_ci_workflows.py

## Needs
- Test conventions: each integration file covers one behaviour and opens with a `//!` doc naming it; scenarios run in both layout modes; stand helpers are shared through modules under `tests/` that are no test target; a test first asserts that its fixture produces the condition; blitz-tests keeps one test target per file, selected by `--test {name}`.
- The one incremental pipeline: incremental and non-incremental layout must read identically, so every re-asserted Tab, key and contrast row runs in both modes.
- The headless stand contract: `seven_guis::stand` boots the four lean tasks offline in a pinned 800x600 Light viewport with one bundled font, which is the ground every stand row reads.
- The test harness contract: input helpers pump and never settle; `text_contrast` reads a WCAG ratio from computed styles and changes nothing; the shared `VirtualClock` that `tick` and `advance_time` move. A select typeahead row reads the 1 s buffer on that clock.
- Keyboard activation and focus: Enter and a released Space on the focused button types dispatch a Click that is queued, so the app's handlers run before the default action; a click on a button focuses it; neither key acts on a disabled form control.
- The DOM semantics decision: `disabled` is read by its presence on a form control alone (`ElementData::is_disabled`) for focusability, the pointer and click gates and the activation keys. This bears on the enabled-state Tab sequences.
- The DOM semantics decision on `inert`: an inert element or one under an inert ancestor is not focusable, takes no hit and leaves the accessibility tree and the snapshot. HTML's focus fixup is not built, which is a known limit owned by a residuals line and not absorbed here.
- The select model: the choice is held as its options' `CHECKED` state; focused and enabled, the select moves by `ArrowDown` · `ArrowUp` · `Home` · `End` and by typeahead; every change goes through `apply_select_choice` and fires `input` and then `change`; a choice of the chosen option fires nothing; a click focuses it; no popup is painted and neither `multiple` nor `optgroup` is modelled. A controlled app writes `selected` from its `input` value or the choice reverts.
- The select's rules live in the crate-private `node::select` with no public type, so a stand row reads the choice through the public `BaseDocument::select_value` or the snapshot's `value`.
- The events contract: a focus change is dispatched as `blur`, `focusout`, `focus`, `focusin`; `change` is dispatched only after `input` on a select's choice.
- The accessibility-tree contract: `build_accessibility_tree` names nodes from text children, `aria-labelledby`, `aria-label`, a bound `<label>` and option labels, and writes the state attributes `aria-selected`, `aria-pressed`, `aria-checked` on the five checkable roles, and `aria-invalid`. Upstream's `aria-expanded`, `aria-disabled`, `aria-level` and description reads are included. These are what the checks the leg takes in witness.
- A measured limit of the accessibility tree: it states no native checkbox's or radio's checked state (owned by a residuals line). A widened leg asserts around that limit and does not fix it.
- The Dioxus DOM bridge: stable element ids (author key first) and the read-only snapshot's state readings (enabled, checked, value, focused, selected, pressed, invalid). Stand rows name controls by these ids and read their states.
- The default-feature decision: `accessibility` is a default feature of blitz-dom and gates the snapshot modules, and the a11y checks build under it.
- Invalidation and state integrity: the changed set and the shell's platform-tree refresh on a non-empty poll, held headlessly by `stand_accessibility_ids`, which stays in the leg.
- The document's clock contract: every engine time read, including the select typeahead window and the double-click window, goes through `DocumentConfig.clock`, which a harness-built stand document shares as a `VirtualClock`.
- The CI contracts: `.github/scripts/ci-leg.sh {leg}` is the one leg runner of CI and the dev host. The `a11y` leg's `--test` list is the surface this chunk widens.
- The CI/CD pattern: the `a11y` job is one of the Ubuntu leg jobs. It needs the fast four, installs `libfontconfig1-dev` through `apt-install.sh`, uploads its log on failure and keeps its tier with no required status check.
- CI infrastructure: the `a11y` job restores the test job's `workspace-test` cache with `save-if: false` and never saves, so the added targets must already build in the workspace test build.
- The stack note: the `ci-scripts` leg that runs `test_ci_workflows.py` needs Python 3.11 or later.
- The existing direct engine changes that are flagged upstreamable (range input, keyboard activation and focus, accessibility state writes, the select model, the clock) are what the re-read reads against. Adding another one is outside the chunk.

## Not needed
- The driver session's wire, command line, MCP server, recording, replay, refusal schema and spans. A driver check put into the leg runs unchanged, and nothing in escher-driver is edited.
- The telemetry bootstrap and its target allowlist, scrub sets and panic hook. No log site or field is added.
- Occupied resources (listeners, ports, outbound hosts, environment variables, filesystem state areas, names). No resource is registered or touched.
- Painting decisions, the password mask, the headless screenshot and the visual lint. Contrast is read from computed styles, not pixels.
- Scrolling, hit testing, `scroll_into_view`, `visible_region`, transformed client rects and the `contain: paint` hit stop. No stand row re-read here depends on them.
- Layout, CSS approximations, unsupported CSS features, inline SVG and fonts.
- HTML parsing, networking, resource loading, JavaScript on Boa and the high-level launch paths.
- The accessibility adapter (`accesskit_xplat`) and its windowed assistive-technology witness. Both are out of scope by the requirements and a residuals line.
- The browser app, rdme, the WPT runner and their outputs.
- Dependency pinning, build profiles, MSRV, release versioning, Nix packaging, cross-compilation and the upstream sync baselines in Project Intent.
- The coverage leg's per-crate floor and the mutation tier.
