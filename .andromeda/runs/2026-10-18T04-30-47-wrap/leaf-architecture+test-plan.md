group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/services/blitz-test-harness.md:19   class: stale
  leaf-text: «read its position less its own scroll offset (built on `Node::absolute_position` — measured and pinned, not fixed)»
  source: .andromeda/test-plan.md:92 «for a box that is itself scrolled, `layout_rect`, `layout_rect_of` and `center_of` — built on `Node::absolute_position` — read where the box stands — a box at (40, 50) reads (40, 50) at every scroll offset»
  replace-with: «read where the box stands — a box at (40, 50) reads (40, 50) at every scroll offset of its own, in both layout modes — as `BaseDocument::get_client_bounding_rect` and the snapshot's `bounds` do (built on `Node::absolute_position`; `scrolled_box_absolute_position` its check, PROVISIONAL)»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «pinned where the real characters' widths put it, not the bullets'»
  source: .andromeda/test-plan.md:18 «one click at one offset into two values of one length, eight narrow and eight wide characters, lands on one index inside 1–7, the check printing no index — the caret placed by the bullets — and a double click selects»
  replace-with: «placed by the bullets — one click at one offset into two values of one length, eight narrow and eight wide characters, landing on one index — and a double click selecting the whole value (PROVISIONAL)»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «--test stand_act_screenshot --test stand_screenshot_repeat»
  source: .andromeda/test-plan.md:26 «`stand_act_transformed` 2, the fourteenth `stand_act_*` file, a stand check `run stand` picks up — a `click` on a translated button accepted and landing, its snapshot `bounds` read where it shows»
  replace-with: «--test stand_act_screenshot --test stand_act_transformed --test stand_screenshot_repeat»

- leaf: .claude/docs/commands.md:63   class: stale
  leaf-text: «`Node::absolute_position` pinned as it reads»
  source: .andromeda/test-plan.md:26 «`scrolled_box_absolute_position` 1 (`Node::absolute_position` of a scrolled box reads where it stands — a box at (40, 50) reads (40, 50) at every scroll offset, only its ancestors' scroll subtracted)»
  replace-with: «a scrolled box's `Node::absolute_position` read where it stands, only its ancestors' scroll offsets subtracted (PROVISIONAL)»

- leaf: .claude/docs/commands.md:63   class: omission
  leaf-text: «the hit stopped at a box that clips by `overflow`»
  source: .andromeda/test-plan.md:26 «a `contain: paint` box with a `visible` `overflow` stops the hit at its padding box»
  replace-with: «the hit stopped at a box that clips by `overflow` and, PROVISIONAL, at a `contain: paint` box's padding box»

- leaf: .claude/docs/commands.md:63   class: omission
  leaf-text: «read the bounds reader's other body (`writing-mode` on)»
  source: .andromeda/test-plan.md:26 «`transformed_client_rect` 4 — a translated box read moved by its translation, a scaled box scaled about its centre, a child of a translated parent moving with it while its offset rect does not, and a target inside»
  replace-with: «read the bounds reader's other body (`writing-mode` on); and `cargo test -p blitz-tests --locked --test transformed_client_rect` — the client rect read where an element's transforms move it (PROVISIONAL): a translated box, a scaled box, a translated parent's child while its offset rect does not move, and a target inside a translated scrolling box reading outside its `visible_region`, the measured `off-screen` limit, both layout modes»