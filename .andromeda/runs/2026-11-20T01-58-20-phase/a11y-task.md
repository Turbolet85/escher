@task_prose part=chunk
Moves the stand checks' repeated setup (booting a stand task or fixture, acting through a driver session, reading the answer: refusal, diff, caused, snapshot text) into shared test helpers that the checks call, so one copy replaces several.
The first family is the clone in the stand act-error-text check, also found in the stand act-errors and stand order-served checks; P3 reads further clone families from the duplication tool at HEAD.
Test code only. Every check stays one target per file and proves what it proved before, with the same functions, assertions and layout modes.
The duplication reading is measured before and after with the code audit's own tool and recipe, against 3.80 → 3.81 → 3.89 %, where the rise came from test-to-test pairs.
@task_file n=1 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand check of the driver's act refusals and their text, run as its own test target against the seven_guis stand — edits: its repeated boot / act / answer-read setup (the first candidate family), replaced by calls to the shared helpers
@task_file n=2 path=tests/blitz-tests/tests/stand_act_errors.rs mark=edits
what: the stand check of the driver's act refusal causes on the stand, run as its own test target — edits: its copy of the first candidate family's setup, replaced by calls to the shared helpers
@task_file n=3 path=tests/blitz-tests/tests/stand_order_served.rs mark=edits
what: the stand check that the stand's order is served and read back through a driver session, run as its own test target — edits: its copy of the first candidate family's setup, replaced by calls to the shared helpers
@task_file n=4 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the shared test module many blitz test targets include, home of shared harness test helpers — edits: new shared helpers for stand boot, driver-session act and answer read
@task_file n=5 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared module of driver-session test helpers the stand checks include, home of shared session setup — edits: shared act and answer-read helpers, where P3 finds that a family already includes it
@task_file n=6 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the shared test-fixture module (fixture documents and their boot) the checks include — edits: the shared fixture boot helper, where P3 finds that a family already includes it
@task_file n=7 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests target pin, the check that the list of blitz test targets (one target per file) is unchanged
@task_item n=1 part=needs
The restructure's invariant is that the boot of a stand task or fixture, the act through a driver session and the read of the answer (refusal, diff, caused, snapshot text) move into shared helpers and keep doing what they do. The same stand checks hold this: the same test functions under the same names, the same assertions on the same readings, in the same layout modes.
@task_item n=2 part=needs
A helper moves only setup and reading code, never the subject or bound of an assertion. No stand check is weakened, skipped or ignored, and expected roles, names, tab orders and other expectations stay stated once in their shared tables, never copied into a helper.
@task_item n=3 part=needs
Every check stays one target per file. No test file is merged, split or renamed, no test target is added or removed, and the targets named by the stand-repeat leg (the stand checks repeated three rounds) are unchanged.
@task_item n=4 part=needs
The shared helpers live under the blitz tests' shared modules (common, session_common, fixtures), which the checks include. P3 decides which module takes which helper, by what each family already includes.
@task_item n=5 part=needs
A shared helper in a module that non-stand targets also include must not break those targets' build and must not add a dead-code warning under the clippy leg. It follows the module's existing convention for helpers unused in some targets.
@task_item n=6 part=needs
The answer-read helpers must pass on the driver's answers exactly as the checks read them before: the driver refusal causes, the actions answer and its causes, the snapshot diff and its entries, and the parts and state tokens of the snapshot text form. They must not re-shape or re-parse any of these.
@task_item n=7 part=needs
Nothing a check asserts on leaves the repo differently. A helper writes no id, name, value or typed text into a test name or an assertion message, because a driver refusal holds nothing of the call.
@task_item n=8 part=needs
Test code only. Nothing changes in the engine, driver, harness, stand or product code, and no crate or dependency is added.
@task_item n=9 part=needs
The snapshot and accessibility-tree readers that the helpers call stay behind the accessibility feature, just as they were when the checks called them directly. A helper reaching them compiles only where the checks that use it already enable that feature.
@task_item n=10 part=needs
The duplication reading is taken before and after with the code audit's own tool (jscpd, Rust format, default minimum tokens, gitignore respected) over the audit's directories. It is summarized as the audit record does: the percentage to two places, duplicated and total lines, clones, the src / test / mixed split and the top ten.
@task_item n=11 part=needs
The readings it moves against are 3.80 % → 3.81 % → 3.89 %, with test pairs 266 → 291 → 307 while src pairs held at 223. P4 settles which earlier reading counts as "back under", and whether that is read as the whole percentage or as the test split.
@task_item n=12 part=needs
The moved setup still passes every CI gate it passed before: the accessibility test leg in the slow tier, the accessibility checks that gate CI and the stand checks repeated three rounds. Each check stays deterministic across the repeated rounds.
@task_standing n=1
convention: each shared helper that a stand check calls keeps expected roles, names and orders stated once in the shared expectation tables, not restated beside the helper
home: screen-reader-support · 7-screen-reader-support
@task_standing n=2
convention: each stand check reached through a helper stays an accessibility assertion written as a Rust test over the AccessKit tree, inside the accessibility test leg
home: 3-a11y-assertion-harness-contract
@task_standing n=3
convention: each stand check keeps its place in the three-round stand-repeat leg under its own named target
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
The rows on keyboard behaviour for the range input, single and multiple select, the CRUD list, the Cells grid and keyboard activation do not bind. The chunk changes no key or pointer behaviour; it only moves test setup.
@task_item n=2 part=not_needed
The rows that state how roles are mapped, how ARIA attributes are read and written, how accessible names take precedence and how hidden or inert content is excluded from the tree do not bind. No engine or tree-building code changes.
@task_item n=3 part=not_needed
The rows on the platform accessibility adapter, the accesskit_xplat adapter, AccessKit event routing and the absence of an adapter on the stand binaries do not bind. The chunk adds no adapter and changes none.
@task_item n=4 part=not_needed
The rows on the accessibility tier, on the accessibility feature being default-on or off in the browser, and on how accessibility reach is stated in the capability statement do not bind. The chunk changes no feature default or reach statement.
@task_item n=5 part=not_needed
The rows on visual design do not bind: contrast thresholds, the contrast lint and its finding, scrollbar and select-list colours, the stand accent colour, zoom and colour scheme. No paint or contrast code changes.
@task_item n=6 part=not_needed
The rows on cognitive accessibility, such as an invalid state not shown by colour or class alone, do not bind. No stand markup or styling changes.
@task_item n=7 part=not_needed
The rows on password masking, password caret placement, IME enabling and option labels leaving only as pixels do not bind. The chunk changes no product behaviour behind these rules.
@task_item n=8 part=not_needed
The rows on the UI graph state registration and the Dioxus document reached-handler record do not bind. These components are product code the chunk does not touch.
