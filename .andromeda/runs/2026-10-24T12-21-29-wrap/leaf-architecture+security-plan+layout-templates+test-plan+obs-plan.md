group: architecture+security-plan+layout-templates+test-plan+obs-plan
read: CLAUDE.md · 150 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «— and passes through an element carrying `inert`; a scrolled box reads its own client rect where it stands»
  source: .andromeda/architecture.md:143 «a hit stops at the padding box of a box paint clips and skips a node dropped since the last resolve; `Node::absolute_position` subtracts only ancestors' scroll offsets»
  replace-with: «— passes through an element carrying `inert` and skips a node dropped since the last resolve; a scrolled box reads its own client rect where it stands»
