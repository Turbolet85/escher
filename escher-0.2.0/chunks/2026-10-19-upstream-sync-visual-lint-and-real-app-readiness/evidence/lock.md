# The lock — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

Read by /andromeda-implement, run `.andromeda/runs/2026-10-19T19-16-25-implement/`, 2026-10-19, after the take.
Lines quoted from the trail `gate-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness.json` of that run.

## No package enters, leaves or moves
`Cargo.toml`, `Cargo.lock` and `deny.toml` are byte-identical to the base `67f17b9d6`: the preservation guard names all
three and reads exit 0.

```
  2 probe       green · exit 0 · 0.01s · 0 B → 2.log · git diff --quiet 67f17b9d632ceb66f6c01b347d9f9ba036dd29b6 … (555 chars)
  3 probe       green · exit 1 · 0.0s · 2 B → 3.log · grep -c -E 'DioxusLabs/taffy|git = ' Cargo.toml
  4 probe       green · exit 0 · 0.0s · 2 B → 4.log · grep -c -E '^taffy = \{ version = "0\.15",' Cargo.toml
```

Entry 3's log reads `0`: no git dependency in the root manifest. Entry 4's log reads `1`: Taffy stays the registry 0.15.

## The four Taffy-main commits stay out
`1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6` are not taken. A merge of either taken commit would bring them,
and they put git Taffy `ea07c4c3` in the root manifest (`git show 17de049b1:Cargo.toml` line 100: `taffy = { git =
"https://github.com/DioxusLabs/taffy", rev = "ea07c4c304216c9b48bdd88c46153c1cbe3cae10", …`; inputs#I1), against the
founder's rule of 2026-10-13 that no git dependency is left in the root manifest. The reason and the answer — F1
answered «Cherry-pick two (Recommended)» by the operator as delegate, PROVISIONAL — are inputs#I2.

## The audit
```
 20 probe       green · exit 0 · 1.2s · 229 B → 20.log · bash .github/scripts/ci-leg.sh audit
```
Its log ends `advisories ok, licenses ok`, with the one standing `license-not-encountered` warning for the `NCSA`
allowance (`deny.toml:46`), as at the base. `deny.toml` is unchanged: no ignore was added.
