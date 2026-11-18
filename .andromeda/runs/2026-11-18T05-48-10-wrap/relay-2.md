Wrap P2 cascade, two question dialogs to the operator, 2026-11-18.

Q1. No comparer flagged the three leaf clauses that say checks the a11y leg now runs (select_popup, select_popup_painted, stand_act_popup, key_press_after_container_lost_its_box) are not in that leg. They are .claude/rules/a11y.md:42, .claude/docs/tests-summary.md:21 @c28827 and @c31059. Which door?
Options: Hand write now (Recommended) · CARRY to next entry · Leave as is
Answer: Hand write now (Recommended)
Notes: Remove the false «not in the a11y leg» clauses and say nothing in their place beyond what the sentence needs to stay true — do not write the leg's file list into a leaf: ci-leg.sh holds it and a list retells the code. Land through rec.py harness land, --dry-run first, and name the three under Leaves as by hand.

Q2. Two more leaf clauses say stand_act_popup is «not among the `a11y` leg's named tests», which ci-leg.sh:41 contradicts: .claude/docs/commands.md:61 and .claude/docs/services/escher-driver.md:61. Your ruling covered the three named clauses. Should these two get the same by-hand removal?
Options: Same removal (Recommended) · CARRY them
Answer: Same removal (Recommended)
Notes: And the same for any further clause of this exact class you find in this wrap (a leaf saying a test is not in a leg that ci-leg.sh shows it in): remove it the same way and name it under Leaves, without asking again.
