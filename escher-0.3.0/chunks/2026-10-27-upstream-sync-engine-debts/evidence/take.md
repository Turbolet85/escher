# The take — 2026-10-27-upstream-sync-engine-debts

The founder's «Одиннадцать коммитов (Рекомендую)» (2026-10-27, relayed verbatim by the overseer, inputs#I4). Each pick
`git -c rerere.enabled=false cherry-pick --no-commit {sha}` on `de85792f5`, in upstream order, on one stacked index,
between 2026-10-27T09:58:03Z (Setup's clock read) and 2026-10-27T10:00:46Z (the clock read after this file's first write). `.git/rr-cache` read 16 entries before and 16 after: nothing recorded or replayed.

| # | sha | exit | files |
|---|---|---|---|
| 1 | `d63ed2c8cbf359bf096c70e9b7dbb2b981f36c40` (#1268) | 0 | `apps/browser/src/main.rs` · `apps/browser/src/tab.rs` |
| 2 | `6587e7e79acfcc061540e44710bce373e5698713` (#1269) | 0 | `packages/blitz-shell/src/lib.rs` · `packages/blitz-shell/src/window.rs` |
| 3 | `e6f4615ecdf25e917e8d62c3dfc9a407a5781f81` (#1270) | 1 — `CONFLICT (content): Merge conflict in packages/blitz-vibey-script/src/runtime.rs` | `src/dom/element.rs` M, `src/dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs` A, `src/runtime.rs` M (conflicted), `src/runtime/{bootstrap,css,events,fetch,modules,timers,window}.rs` A, `src/runtime/bootstrap/*.js` A (nine) |
| 4 | `a7ea9631b249bd05a31b5c3a56695db278a9714f` (#1274) | 0 | `.github/workflows/publish-browser.yml` |
| 5 | `b5b163450344d631eda1735cb975c22e92b174d5` (#1275) | 0 | `.github/workflows/publish-browser.yml` |
| 6 | `7fc9ee3af156ead71ef4c09fb2d869381db3640e` | 1 — `CONFLICT (content): Merge conflict in packages/dioxus-native-dom/src/dioxus_document.rs` | `packages/blitz-dom/src/layout/construct.rs` (clean) · `packages/dioxus-native-dom/src/dioxus_document.rs` (conflicted) |
| 7 | `2963b65a2d3be98ce45ac6768ced9bd2c483d719` (#1280) | 0 | `packages/blitz-dom/src/layout/construct.rs` · `tests/blitz-tests/tests/all.rs` · `tests/blitz-tests/tests/pseudo_element_flow_classification.rs` A |
| 8 | `cee6b87310e57fe370832034fc6a61d58309d6d0` (#1283) | 0 | blitz-vibey-script `src/document.rs` · `src/fetch.rs` · `src/lib.rs` · `src/runtime/fetch.rs` · `src/runtime/modules.rs` · `tests/dom.rs` |
| 9 | `53e9d81c6ba24f3fdd57ad4b3abfb707eff133f6` (#1007) | 1 — `CONFLICT (content): Merge conflict in packages/blitz-dom/src/mutator.rs` | `packages/blitz-dom/src/mutator.rs` (conflicted) · `node/element.rs` · `stylo.rs` (clean) |
| 10 | `6fee0db257c5e263ac7047487e2f56791f89bddd` (#1295) | 0 | `wpt/WPT_COMMIT` |
| 11 | `efdc5472a070640d6053ca51340ee6146ae4575e` (#1297) | 0 | blitz-vibey-script `src/runtime/bootstrap.rs` · `src/runtime/bootstrap/style.js` · `src/runtime/css.rs` · `tests/dom.rs` |

After pick 11: no `CHERRY_PICK_HEAD`, no unmerged path; `git diff --cached --shortstat de85792f5` 41 files changed,
4604 insertions, 3631 deletions; `cargo fmt --all --check` exit 0, so no formatter pass changed a picked file.

## The three resolutions — the take's non-additive edits

Each resolved and `git add`ed before the next pick.

### 1. `e6f4615ec` — `packages/blitz-vibey-script/src/runtime.rs`

- **Before (the conflicted file):** 1079 lines, one hunk — `<<<<<<< HEAD` at 151, `=======` at 1077, `>>>>>>> e6f4615ec`
  at 1078. Our side (152–1076) held the code upstream moved out into `runtime/` (from `install_body_onload_attribute`
  on); upstream's side was empty. Our one change to the file since `23b9b26a9` is `14818317c`'s two-line select-by-id
  comment (`git diff 23b9b26a9 de85792f5 -- …/runtime.rs`: the `change`-synthesis comment at `:1669`).
- **After:** `git checkout --theirs -- packages/blitz-vibey-script/src/runtime.rs` — 151 lines, `cmp` equal to
  `git show e6f4615ec:packages/blitz-vibey-script/src/runtime.rs`.
- **Our comment carried** to its new home, `packages/blitz-vibey-script/src/runtime/events.rs:171-173`, above the
  checkbox/radio `change` synthesis:
  - before (upstream):
    ```
            // inputs. Blitz only generates `input` events, so synthesise the `change`
            // event here.
    ```
  - after (ours):
    ```
            // inputs. Blitz generates a `change` event for a `select` alone (dispatched
            // above like any event), so synthesise the checkbox/radio one here.
    ```

### 2. `7fc9ee3af` — `packages/dioxus-native-dom/src/dioxus_document.rs` (test module)

- **Before:** one hunk after `#[test]` at 725 — `<<<<<<< HEAD` at 726, `=======` at 727, `>>>>>>> 7fc9ee3af` at 804.
  Our side empty; upstream's side held `input_focused_on_mount_enables_ime` (728–761), then `#[test]`
  `task_polled_during_render_can_access_document` (763–801, `286874cc1`'s, not taken), then `#[test]` (803).
- **After:** ours plus `input_focused_on_mount_enables_ime` alone, under the `#[test]` at 725, then `#[test]`
  `fn mounted_data_of_removed_element_reports_error` (762–763). `task_polled_during_render_can_access_document`
  dropped with its hunk lines. `git diff de85792f5 -- …/dioxus_document.rs`: 0 removed lines — the file's change is
  exactly upstream's added test. Its helpers `rerender` and `poll_until_idle` are ours already. The engine hunk
  (`layout/construct.rs`, `create_text_editor`) picked clean.

### 3. `53e9d81c6` — `packages/blitz-dom/src/mutator.rs` `set_attribute`

- **Before:**
  ```
  <<<<<<< HEAD
          if name.local == local_name!("min")
              || name.local == local_name!("max")
              || name.local == local_name!("step")
          {
              element.resanitize_range_input();
  =======
          if name.local == local_name!("open") {
              element.flush_open_state();
  >>>>>>> 53e9d81c6 (Add support for `<details>` `:open` selector (#1007))
          }
  ```
- **After:** two independent `if` blocks, ours first:
  ```
          if name.local == local_name!("min")
              || name.local == local_name!("max")
              || name.local == local_name!("step")
          {
              element.resanitize_range_input();
          }

          if name.local == local_name!("open") {
              element.flush_open_state();
          }
  ```
  `remove_attribute`'s `open` arm picked clean.

## Further gate-driven edits

none
