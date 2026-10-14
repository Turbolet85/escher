
## 2026-10-13-stand-keyboard-harness — `stand_keyboard`, the restated focus pins and the counts as measured
**Section:** §1 Test Scope Summary → Coverage scope (tests/blitz-tests · dioxus-native-dom) · §2 Directory pattern · §3 Crate-local test helpers (blitz-tests) · Input helpers · Agent-run contract → Proof · §5 Session ↔ held instance · §9 Pipeline facts · registry Session lifecycle
**Change:**
- §1 blitz-tests:
  - + `stand_keyboard` 11: the Tab walk with wrap, three focus readings agreeing; Enter and Space as a click on 20 stand rows; the fixture controls; a click focusing a button; `disabled` of any value skipped, the `<div disabled tabindex=0>` limit pinned; the CRUD list's keys; the Apple delete binding on every OS; no `inert`.
  - `stand_act_keys` 2 → 3: `press enter` and `press space` on a focused button count as its click.
  - `stand_act_obstructed`: after the accepted click the clicked button reads `focused` and is the tree's focus. Was: no element reads `focused`.
  - `stand_act_ids`: every accepted control changes the screen at boot, 15 control rows over `controls(task)`. Was 9.
  - `stand_diff`: the Book names seven bounds-only nodes and `flight-book` `[focused, bounds]`. Was eight `[bounds]`.
  - `dioxus_falsy_boolean_attrs`: 28 names, `inert` the 28th. Was 27, the falsy `inert` not measured.
  - `stand_accessibility_ids`: CRUD's Tab order restated, six stops. Was "unchanged".
  - `focusability_updates` +1: a bare `disabled` set after creation removes focusability.
- §1 dioxus-native-dom: the snapshot `disabled` pin reads not focusable as well. Was: "stays focusable".
- §2: `common` readers 18 → 19; blitz-tests 120 files · 119 targets → 121 · 120.
- §3 helpers: `tab_order` and `focused` moved into `common`, 19 readers; `tab_order` off the per-file list.
- §3 Input helpers: `apple_keybinding` exercised on every OS, green on linux, windows and macos. Was: no standing check calls it on Linux or Windows.
- §3 Proof: `run stand` 141 → 153 passed over 38 files.
- §5: `stand_act_obstructed`'s focus reading restated.
- §9: workspace 181 · 915 · 0 · 12 → 182 · 928 · 0 · 12.
- Registry Session lifecycle: `stand_act_keys` 3, the thirteen driver-action files 47 (was 46).
**Why:** this chunk's new check, the pins its engine changes restated (a click focuses a button; `disabled` by presence, both ratified by the founder) and the counts as measured.
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/
