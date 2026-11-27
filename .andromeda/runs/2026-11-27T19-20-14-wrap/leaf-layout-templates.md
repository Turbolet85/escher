@leaf_group group=layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=43
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/l
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=27 class=stale
leaf-text: never shrinking, provisional,
source: dec-bvhmgy6h «Circle Drawer canvas keeps its size — Circle Drawer's canvas keeps 700 x 400 and never shrinks, so on the 600 px stand view the dialog's Close lies below the fold and is scrolled into view before a click.»
replace-with: never shrinking, standing,
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=27 class=stale
leaf-text: is refused disabled; provisional)
source: dec-fltlgmy7 «Circle Drawer dialog disables other circles — While Circle Drawer's diameter dialog is open, every circle but the one being adjusted declares disabled and is refused disabled.»
replace-with: is refused disabled; standing)
