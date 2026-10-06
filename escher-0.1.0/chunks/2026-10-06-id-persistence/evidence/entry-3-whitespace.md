# Entry 3 (presentation-unchanged) — surfaced at implement, 2026-10-06T10:15Z

## Why the entry reads red
Person keys only give a keyed list once the filter moves out of the `for` body and into its iterator. In the old
form each `for` item was an unkeyed `if` wrapper around the keyed row `div`. Dioxus therefore diffed the items by
position, and when an inner key changed it replaced every shifted row. Measured: with `key: "{person.id}"` inside the
`if`, `stand_element_ids::text_and_removed_nodes_read_no_id` read `left: 3, right: 1` ("one row node left the list").
With the filtered iterator it reads green, because Delete drops only Hans's row node.

Removing the `if` level un-nests the row `div` by one level. Entry 3's `diff` compares the grepped lines with their
whitespace. Its only difference is 4 leading spaces on the row's `class:` line, and the class expression is
byte-identical:

```
7c7
<                                         class: if selected() == Some(i) { "list-item selected" } else { "list-item" },
---
>                                     class: if selected() == Some(i) { "list-item selected" } else { "list-item" },
```

## The same comparison with leading whitespace stripped
Run by hand on the working tree:

```
B=065295540ec0b80c0e40b7b186c7aecfba6f6121; F=examples/seven_guis/src/tasks/crud.rs; git show $B:$F | grep -E 'class:|style|id: "|^const CSS' | sed 's/^[[:space:]]*//' | diff - <(grep -E 'class:|style|id: "|^const CSS' $F | sed 's/^[[:space:]]*//') > /dev/null && git show $B:$F | sed -n '/^const CSS/,$p' | diff - <(sed -n '/^const CSS/,$p' $F) > /dev/null && echo presentation-unchanged
```

Result: exit 0, last line `presentation-unchanged`.

## Operator's word
Asked at implement P2 (2026-10-06). The answer was "Keep fix, surface entry 3". In the operator's note: "Finish this
run with entry 3 surfaced. After implement ends, entry 3 is to be changed in plan.md to the whitespace-insensitive form
of the same class-line comparison (the guard's intent is class strings unchanged, not indentation) and re-run; the
wrap's light gate then reads it."

## Entry 3 amended and re-run (2026-10-06T10:18Z)
On the operator's direction, `plan.md` entry 3 now strips leading whitespace from both grepped sides
(`| sed 's/^[[:space:]]*//'`). The CSS-block comparison is unchanged. Re-run alone with `gate.py run … --entry 3`:

```
  3 probe       green · exit 0 · 0.01s · 23 B → 3.log · git show 065295540ec0b80c0e40b7b186c7aecfba6f6121:examples… (519 chars)
entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
```
