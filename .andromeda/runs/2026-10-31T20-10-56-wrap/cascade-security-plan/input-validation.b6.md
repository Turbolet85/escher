### input-validation#1.118 · values · input · ## Input Validation · lines 396–398
- **Boundary:** WPT runner
  - **What to validate:** `data:` URLs
  - **How (stack-specific):** Processing and base64 decoding errors record a request failure (wpt/runner/src/net_provider.rs:60-65)
### input-validation#1.119 · values · input · ## Input Validation · lines 399–401
- **Boundary:** WPT runner
  - **What to validate:** Page messages
  - **How (stack-specific):** Parsed as JSON and checked for `type` before use (wpt/runner/src/test_runners/harness_test.rs:231-250)
### input-validation#1.120 · values · input · ## Input Validation · lines 402–404
- **Boundary:** CI scripts
  - **What to validate:** PR lookup
  - **How (stack-specific):** Selects only open PRs whose head sha equals the workflow run's head sha, and skips when none is found (.github/workflows/wpt-post-results.yml:36-42)
### input-validation#1.121 · values · input · ## Input Validation · lines 405–407
- **Boundary:** CI scripts
  - **What to validate:** Comment splice
  - **How (stack-specific):** `splice` replaces only when both markers exist with end after start (.github/scripts/wpt_diff_to_pr.py:252-260)
### input-validation#1.122 · values · input · ## Input Validation · lines 408–410
- **Boundary:** Example apps
  - **What to validate:** 7GUIs inputs
  - **How (stack-specific):** Flight booker validates `dd.mm.yyyy` dates including month range and leap years, and return date not before start (examples/seven_guis/src/tasks/flight_booker.rs:3-41; examples/seven_guis/src/tasks/flight_booker.rs:50-59); cells tokenizer rejects unknown characters, detects reference cycles as `#CYCLE`, and treats division by zero as an error (examples/seven_guis/src/tasks/cells.rs:29-31; examples/seven_guis/src/tasks/cells.rs:147; examples/seven_guis/src/tasks/cells.rs:230-233); circle drawer clamps diameter to 5–100 (examples/seven_guis/src/tasks/circle_drawer.rs:111-114); the flight booker's mode `select#flight-type` writes its `flight_type` signal by a closed match on the `input` value — `return` a return flight, any other value one-way — over its two options (`one-way`, `return`) (examples/seven_guis/src/tasks/flight_booker.rs:66-89)
### input-validation#1.123 · values · input · ## Input Validation · lines 411–413
- **Boundary:** Example apps
  - **What to validate:** 7GUIs Cells, the `cells-grid` keys
  - **How (stack-specific):** `cells-grid` takes `tabindex` 0 and an `onkeydown` with a closed key match and no role: arrows move `current`, clamped, Enter opens the editor on `current`, focused, and the editor's Enter commits, stops propagation and returns focus to the grid; a click inside a cell's own open editor returns early (examples/seven_guis/src/tasks/cells.rs:425-433), ratified as to the grid — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
### input-validation#1.124 · values · input · ## Input Validation · lines 414–416
- **Boundary:** Example apps
  - **What to validate:** Widget and TodoMVC
  - **How (stack-specific):** The widget's `color` attribute is parsed with `parse_color`, falling back to black on failure (examples/custom_widget.rs:109-115); the TodoMVC page trims input and ignores empty text (examples/preact/index.html:60-63)
### input-validation#1.125 · values · input · ## Input Validation · lines 417–419
- **Boundary:** Example fixtures
  - **What to validate:** Form inputs
  - **How (stack-specific):** The graphite fixture's name and email inputs are required and the email input is type=email (examples/assets/graphite.html:2052; examples/assets/graphite.html:2054); its phone input has autocomplete=off and tabindex=-1, and its column is display:none (examples/assets/graphite.html:2053; examples/assets/graphite.html:293-295); the google fixture's query textarea is limited to maxlength 2048 (examples/assets/google.html:3181)
