# By-level readings — 2026-10-07-sink-target-allowlist

What the two sink-installing seven_guis binaries write to stderr at each `RUST_LOG` setting, before and after the
sink's drop. This is implement's hand record of what the instrument beside it (`by-level.py`) printed. The instrument
prints counts and the spellings of its own fixed needle lists, never anything a binary wrote, and removes its captures.

- **Base commit:** `ebd7411f7c6d3908f9e1a7c68cf9a96ffa0b18d3`. The before-readings were taken on its sources, before
  any source edit of this chunk; the after-readings on the working tree with this chunk's edits.
- **Build, both times:** `cargo build -p seven_guis --bins --locked` — the package alone, so no engine `tracing`
  feature is on.
- **Host:** the dev host (Linux, `WAYLAND_DISPLAY=wayland-1`), 2026-10-07.
- **Form, host:** `python3 escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.py host` — per
  setting, `target/debug/escher-session crud {state-dir}`, one `hello v1` then `stop v1` over its socket.
- **Form, windowed:** the same instrument with `windowed` — per setting, `timeout 10 target/debug/seven_guis_native`,
  left on Home with nobody clicking; exit 124 is the stand still up when `timeout` stopped it.
- **Settings read:** `RUST_LOG` unset (the sink's default, `warn`) · `info` · `debug` · `trace`.
- **Each reading holds:** the exit, stderr bytes and lines, stdout bytes, the emitting targets by level with their
  line counts, and the id needles and name needles found with the number of lines holding each.
- **Needles:** the instrument's fixed lists — 15 author keys and 6 texts of the CRUD screen for the host, 11 author
  keys and 5 texts of Home for the windowed stand.
- **Not measured:** typed text. No command can type into the host's instance yet; that proof is carried on "Act by
  id". `log.file` occurrences were not counted by the instrument.
- **Not claimed:** no reading here is a windowed witness of the accessibility-tree refresh; the carry on "Stand a11y
  assertions" stands.

## host — before

Taken 2026-10-07T07:58:07Z; the instrument exited 0 and wrote nothing to its own stderr. It agrees with research.md's
table: 0 · 1 · 1165 · 1501 lines, 15 of 15 ids at `debug` and `trace`, 6 of 6 names at `trace` only.

```text
## host · RUST_LOG (unset) · exit 0 · answered True · 0 B before the answer · state dir left False
   stderr 0 B · 0 lines · stdout 0 B
   targets: none
   ids found 0 of 15: none
   names found 0 of 6: none
## host · RUST_LOG info · exit 0 · answered True · 124 B before the answer · state dir left False
   stderr 124 B · 1 lines · stdout 0 B
   targets: INFO escher_telemetry ×1
   ids found 0 of 15: none
   names found 0 of 6: none
## host · RUST_LOG debug · exit 0 · answered True · 856040 B before the answer · state dir left False
   stderr 892447 B · 1165 lines · stdout 0 B
   targets: DEBUG selectors::matching ×112 · DEBUG style::data ×4 · DEBUG style::invalidation::stylesheets ×24 · DEBUG style::rule_cache ×17 · DEBUG style::rule_tree::core ×731 · DEBUG style::sharing ×47 · DEBUG style::style_resolver ×125 · DEBUG style::stylesheet_set ×7 · DEBUG style::stylist ×4 · DEBUG style::traversal ×93 · INFO escher_telemetry ×1
   ids found 15 of 15: task-shell ×15 · task-header ×30 · task-body ×14 · back-btn ×16 · task-title ×13 · crud-filter ×28 · crud-list ×14 · crud-create ×15 · crud-update ×17 · crud-delete ×4 · crud-person-0 ×18 · crud-person-1 ×4 · crud-person-2 ×4 · crud-name ×30 · crud-surname ×8
   names found 0 of 6: none
## host · RUST_LOG trace · exit 0 · answered True · 1068492 B before the answer · state dir left False
   stderr 1104899 B · 1501 lines · stdout 0 B
   targets: DEBUG selectors::matching ×112 · DEBUG style::data ×4 · DEBUG style::invalidation::stylesheets ×24 · DEBUG style::rule_cache ×17 · DEBUG style::rule_tree::core ×731 · DEBUG style::sharing ×47 · DEBUG style::style_resolver ×125 · DEBUG style::stylesheet_set ×7 · DEBUG style::stylist ×4 · DEBUG style::traversal ×93 · INFO escher_telemetry ×1 · TRACE dioxus_core::diff::node ×12 · TRACE dioxus_signals::signal ×20 · TRACE style::sharing ×35 · TRACE style::style_resolver ×163 · TRACE style::traversal ×86 · TRACE warnings::warnings ×20
   ids found 15 of 15: task-shell ×19 · task-header ×37 · back-btn ×20 · task-title ×17 · task-body ×18 · crud-filter ×35 · crud-list ×18 · crud-name ×37 · crud-surname ×13 · crud-create ×19 · crud-update ×21 · crud-delete ×7 · crud-person-0 ×24 · crud-person-1 ×9 · crud-person-2 ×9
   names found 6 of 6: 'Emil, Hans' ×3 · 'Mustermann, Max' ×3 · 'Tisch, Roman' ×3 · 'Filter prefix: ' ×1 · 'Name: ' ×1 · 'Surname: ' ×1
```

## windowed — before

Taken 2026-10-07T07:58:13Z to 07:58:54Z. This is the instrument's first run in `windowed` mode: it ran as written and
needed no fix. Its one-shot control reads as required — Home ids at `trace` (11 of 11) and none with `RUST_LOG`
unset. New in this reading: a `WARN winit_wayland::window::state` line at every setting, unset included, and an
`INFO wgpu_hal::vulkan::adapter` line at `info`, `debug` and `trace`.

```text
## windowed · RUST_LOG (unset) · exit 124 · exit 124 = still up when timeout stopped it
   stderr 164 B · 1 lines · stdout 0 B
   targets: WARN winit_wayland::window::state ×1
   ids found 0 of 11: none
   names found 0 of 5: none
## windowed · RUST_LOG info · exit 124 · exit 124 = still up when timeout stopped it
   stderr 636 B · 3 lines · stdout 0 B
   targets: INFO escher_telemetry ×1 · INFO wgpu_hal::vulkan::adapter ×1 · WARN winit_wayland::window::state ×1
   ids found 0 of 11: none
   names found 0 of 5: none
## windowed · RUST_LOG debug · exit 124 · exit 124 = still up when timeout stopped it
   stderr 6522462 B · 12413 lines · stdout 0 B
   targets: DEBUG naga::front ×3185 · DEBUG naga::front::wgsl::lower ×450 · DEBUG naga::front::wgsl::lower::conversion ×1246 · DEBUG naga::proc::overloads::list ×3915 · DEBUG naga::proc::typifier ×1217 · DEBUG naga::valid::expression ×1111 · DEBUG naga::valid::function ×142 · DEBUG naga::valid::interface ×65 · DEBUG sctk ×13 · DEBUG selectors::matching ×91 · DEBUG style::data ×7 · DEBUG style::invalidation::stylesheets ×27 · DEBUG style::rule_cache ×14 · DEBUG style::rule_tree::core ×402 · DEBUG style::sharing ×67 · DEBUG style::style_resolver ×80 · DEBUG style::stylesheet_set ×5 · DEBUG style::stylist ×8 · DEBUG style::traversal ×161 · DEBUG wgpu_core::device::resource ×2 · DEBUG wgpu_core::instance ×6 · DEBUG wgpu_hal::gles::adapter ×6 · DEBUG wgpu_hal::gles::egl ×16 · DEBUG wgpu_hal::vulkan::adapter ×24 · DEBUG wgpu_hal::vulkan::instance ×150 · INFO escher_telemetry ×1 · INFO wgpu_hal::vulkan::adapter ×1 · WARN winit_wayland::window::state ×1
   ids found 11 of 11: home-header ×17 · task-grid ×14 · home-title ×15 · home-subtitle ×13 · task-card-counter ×29 · task-card-temp-converter ×4 · task-card-flight-booker ×4 · task-card-timer ×4 · task-card-crud ×4 · task-card-circle-drawer ×4 · task-card-cells ×4
   names found 0 of 5: none
## windowed · RUST_LOG trace · exit 124 · exit 124 = still up when timeout stopped it
   stderr 19458805 B · 47482 lines · stdout 0 B
   targets: DEBUG naga::front ×3185 · DEBUG naga::front::wgsl::lower ×450 · DEBUG naga::front::wgsl::lower::conversion ×1246 · DEBUG naga::proc::overloads::list ×3915 · DEBUG naga::proc::typifier ×1217 · DEBUG naga::valid::expression ×1111 · DEBUG naga::valid::function ×142 · DEBUG naga::valid::interface ×65 · DEBUG sctk ×13 · DEBUG selectors::matching ×91 · DEBUG style::data ×7 · DEBUG style::invalidation::stylesheets ×27 · DEBUG style::rule_cache ×14 · DEBUG style::rule_tree::core ×402 · DEBUG style::sharing ×67 · DEBUG style::style_resolver ×80 · DEBUG style::stylesheet_set ×5 · DEBUG style::stylist ×8 · DEBUG style::traversal ×161 · DEBUG wgpu_core::device::resource ×2 · DEBUG wgpu_core::instance ×6 · DEBUG wgpu_hal::gles::adapter ×6 · DEBUG wgpu_hal::gles::egl ×16 · DEBUG wgpu_hal::vulkan::adapter ×24 · DEBUG wgpu_hal::vulkan::instance ×150 · INFO escher_telemetry ×1 · INFO wgpu_hal::vulkan::adapter ×1 · TRACE calloop::loop_logic ×14 · TRACE calloop::sources ×11 · TRACE dioxus_core::diff::node ×20 · TRACE dioxus_signals::signal ×17 · TRACE naga::back::spv::writer ×622 · TRACE naga::compact ×1947 · TRACE naga::compact::expressions ×9870 · TRACE naga::compact::functions ×142 · TRACE naga::compact::handle_set_map ×20854 · TRACE naga::proc::constant_evaluator ×986 · TRACE naga::proc::type_methods ×4 · TRACE style::sharing ×58 · TRACE style::style_resolver ×102 · TRACE style::traversal ×156 · TRACE warnings::warnings ×17 · TRACE wgpu_core::command ×12 · TRACE wgpu_core::command::pass ×16 · TRACE wgpu_core::command::render ×12 · TRACE wgpu_core::device::global ×80 · TRACE wgpu_core::device::queue ×11 · TRACE wgpu_core::device::resource ×18 · TRACE wgpu_core::device::surface_config ×4 · TRACE wgpu_core::instance ×5 · TRACE wgpu_core::resource ×16 · TRACE wgpu_hal::gles::adapter ×1 · TRACE wgpu_hal::gles::egl ×70 · TRACE wgpu_hal::vulkan::instance ×4 · WARN winit_wayland::window::state ×1
   ids found 11 of 11: home-header ×22 · home-title ×20 · home-subtitle ×18 · task-grid ×19 · task-card-counter ×35 · task-card-temp-converter ×9 · task-card-flight-booker ×9 · task-card-timer ×9 · task-card-crud ×9 · task-card-circle-drawer ×9 · task-card-cells ×9
   names found 5 of 5: '7GUIs' ×1 · 'Seven benchmark tasks for GUI frameworks' ×1 · 'Temp Converter' ×3 · 'Flight Booker' ×3 · 'Circle Drawer' ×3
```

## host check — red before the fix

`cargo test -p seven_guis --locked --test host_log` against the unfixed sink, 2026-10-07T08:02Z, exit 101. The
failing message, whole:

```text
thread 'the_host_logs_no_id_and_no_name_at_trace' (3197811) panicked at examples/seven_guis/tests/host_log.rs:153:5:
16 of 16 id needles and 6 of 6 name needles found in 1501 stderr lines
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```

It is red because needles were found: the assertion that failed is the last one, after the host answered, stopped,
exited 0, wrote nothing to stdout, left no state directory and stamped every stderr line. No timeout, hang or spawn
error. Predicted 15 id needles, measured 16: the check reads its needles from the booted stand, not from the
instrument's fixed list. The sixteenth is not printed by the check (it prints counts only); the shell's source holds
one more author key with a `-` than the instrument lists, `task-header-spacer` (`examples/seven_guis/src/app.rs:174`).

## sink check — red before the fix

`cargo test -p blitz-tests --locked --test telemetry_drop` against the unfixed sink, 2026-10-07T08:02:11Z, exit 101,
`test result: FAILED. 0 passed; 1 failed; 1 ignored`. It failed on the first third-party sentinel found
(`ESCHER-SENTINEL-51d0 on stderr`). The child's stderr held five lines: the three third-party records under the
directive `warn,style=trace,dioxus_core=trace,selectors=trace` — `WARN style::traversal`, `TRACE
dioxus_core::diff::node` and, over the `log` bridge, `DEBUG selectors::matching` — then the `escher_stand_probe`
record with its sentinel and the `blitz_dom::mutator` record with `message=[redacted]`. Every sentinel is synthetic.

## unit tests — the red-before-green control

The five `outside_target` unit tests were written with the fix, so their control was taken by switching the fix off:
with `is_outside_target` temporarily returning `false`, `cargo test -p escher-telemetry --locked --lib outside_target`
exited 101 with `0 passed; 5 failed; 0 ignored; 0 measured; 5 filtered out`. The file was restored byte-identical
(`cmp`) and the same command read `5 passed; 0 failed`. 2026-10-07, between 08:03:46Z and 08:04:08Z.

## host — after

Taken 2026-10-07T08:04:08Z to 08:04:09Z, on the binaries rebuilt with the fix at 08:04:08Z. Predicted 0 · 1 · 1 · 1
lines, only `INFO escher_telemetry`, 0 ids, 0 names — measured exactly that. The bytes written before the socket
first answered fall from 856,040 (`debug`) and 1,068,492 (`trace`) to 124.

```text
## host · RUST_LOG (unset) · exit 0 · answered True · 0 B before the answer · state dir left False
   stderr 0 B · 0 lines · stdout 0 B
   targets: none
   ids found 0 of 15: none
   names found 0 of 6: none
## host · RUST_LOG info · exit 0 · answered True · 124 B before the answer · state dir left False
   stderr 124 B · 1 lines · stdout 0 B
   targets: INFO escher_telemetry ×1
   ids found 0 of 15: none
   names found 0 of 6: none
## host · RUST_LOG debug · exit 0 · answered True · 124 B before the answer · state dir left False
   stderr 124 B · 1 lines · stdout 0 B
   targets: INFO escher_telemetry ×1
   ids found 0 of 15: none
   names found 0 of 6: none
## host · RUST_LOG trace · exit 0 · answered True · 124 B before the answer · state dir left False
   stderr 124 B · 1 lines · stdout 0 B
   targets: INFO escher_telemetry ×1
   ids found 0 of 15: none
   names found 0 of 6: none
```

## windowed — after

Taken 2026-10-07T08:04:09Z to 08:04:49Z, same binaries. The stand stays up at every setting (exit 124), prints its
install line at `info`, `debug` and `trace`, and shows no Home id and no Home text at any level. With `RUST_LOG`
unset its stderr is now empty: the `winit_wayland` WARN no longer prints.

```text
## windowed · RUST_LOG (unset) · exit 124 · exit 124 = still up when timeout stopped it
   stderr 0 B · 0 lines · stdout 0 B
   targets: none
   ids found 0 of 11: none
   names found 0 of 5: none
## windowed · RUST_LOG info · exit 124 · exit 124 = still up when timeout stopped it
   stderr 124 B · 1 lines · stdout 0 B
   targets: INFO escher_telemetry ×1
   ids found 0 of 11: none
   names found 0 of 5: none
## windowed · RUST_LOG debug · exit 124 · exit 124 = still up when timeout stopped it
   stderr 124 B · 1 lines · stdout 0 B
   targets: INFO escher_telemetry ×1
   ids found 0 of 11: none
   names found 0 of 5: none
## windowed · RUST_LOG trace · exit 124 · exit 124 = still up when timeout stopped it
   stderr 124 B · 1 lines · stdout 0 B
   targets: INFO escher_telemetry ×1
   ids found 0 of 11: none
   names found 0 of 5: none
```

## What the drop loses

Directed at the P5 review: "In the report, list what the drop loses by target, level and count for both binaries,
WARN and ERROR separately." — the operator, 2026-10-07. The listings below are computed from the four readings above
by row, not written by hand. A third-party WARN or ERROR no longer reaches stderr at any level; the readings hold one
such row, on the windowed stand.

### Session host — `escher-session crud`

Every target·level row the before-reading holds and the after-reading lacks; a cell is that row's line count at that `RUST_LOG` setting in the before-reading (0 = the row did not emit there). After the fix every cell of every row below is 0.

#### WARN rows lost

none

#### ERROR rows lost

none

#### INFO rows lost

none

#### DEBUG rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `selectors::matching` | 0 | 0 | 112 | 112 |
| `style::data` | 0 | 0 | 4 | 4 |
| `style::invalidation::stylesheets` | 0 | 0 | 24 | 24 |
| `style::rule_cache` | 0 | 0 | 17 | 17 |
| `style::rule_tree::core` | 0 | 0 | 731 | 731 |
| `style::sharing` | 0 | 0 | 47 | 47 |
| `style::style_resolver` | 0 | 0 | 125 | 125 |
| `style::stylesheet_set` | 0 | 0 | 7 | 7 |
| `style::stylist` | 0 | 0 | 4 | 4 |
| `style::traversal` | 0 | 0 | 93 | 93 |
| **10 rows, lines** | **0** | **0** | **1164** | **1164** |

#### TRACE rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `dioxus_core::diff::node` | 0 | 0 | 0 | 12 |
| `dioxus_signals::signal` | 0 | 0 | 0 | 20 |
| `style::sharing` | 0 | 0 | 0 | 35 |
| `style::style_resolver` | 0 | 0 | 0 | 163 |
| `style::traversal` | 0 | 0 | 0 | 86 |
| `warnings::warnings` | 0 | 0 | 0 | 20 |
| **6 rows, lines** | **0** | **0** | **0** | **336** |

#### Lines off the sink's line shape

none — every stderr line of every before- and after-reading had the sink's line shape.

#### Rows kept

`INFO escher_telemetry`

### Windowed stand — `seven_guis_native`, on Home

Every target·level row the before-reading holds and the after-reading lacks; a cell is that row's line count at that `RUST_LOG` setting in the before-reading (0 = the row did not emit there). After the fix every cell of every row below is 0.

#### WARN rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `winit_wayland::window::state` | 1 | 1 | 1 | 1 |
| **1 rows, lines** | **1** | **1** | **1** | **1** |

#### ERROR rows lost

none

#### INFO rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `wgpu_hal::vulkan::adapter` | 0 | 1 | 1 | 1 |
| **1 rows, lines** | **0** | **1** | **1** | **1** |

#### DEBUG rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `naga::front` | 0 | 0 | 3185 | 3185 |
| `naga::front::wgsl::lower` | 0 | 0 | 450 | 450 |
| `naga::front::wgsl::lower::conversion` | 0 | 0 | 1246 | 1246 |
| `naga::proc::overloads::list` | 0 | 0 | 3915 | 3915 |
| `naga::proc::typifier` | 0 | 0 | 1217 | 1217 |
| `naga::valid::expression` | 0 | 0 | 1111 | 1111 |
| `naga::valid::function` | 0 | 0 | 142 | 142 |
| `naga::valid::interface` | 0 | 0 | 65 | 65 |
| `sctk` | 0 | 0 | 13 | 13 |
| `selectors::matching` | 0 | 0 | 91 | 91 |
| `style::data` | 0 | 0 | 7 | 7 |
| `style::invalidation::stylesheets` | 0 | 0 | 27 | 27 |
| `style::rule_cache` | 0 | 0 | 14 | 14 |
| `style::rule_tree::core` | 0 | 0 | 402 | 402 |
| `style::sharing` | 0 | 0 | 67 | 67 |
| `style::style_resolver` | 0 | 0 | 80 | 80 |
| `style::stylesheet_set` | 0 | 0 | 5 | 5 |
| `style::stylist` | 0 | 0 | 8 | 8 |
| `style::traversal` | 0 | 0 | 161 | 161 |
| `wgpu_core::device::resource` | 0 | 0 | 2 | 2 |
| `wgpu_core::instance` | 0 | 0 | 6 | 6 |
| `wgpu_hal::gles::adapter` | 0 | 0 | 6 | 6 |
| `wgpu_hal::gles::egl` | 0 | 0 | 16 | 16 |
| `wgpu_hal::vulkan::adapter` | 0 | 0 | 24 | 24 |
| `wgpu_hal::vulkan::instance` | 0 | 0 | 150 | 150 |
| **25 rows, lines** | **0** | **0** | **12410** | **12410** |

#### TRACE rows lost

| target | unset | `info` | `debug` | `trace` |
|---|---|---|---|---|
| `calloop::loop_logic` | 0 | 0 | 0 | 14 |
| `calloop::sources` | 0 | 0 | 0 | 11 |
| `dioxus_core::diff::node` | 0 | 0 | 0 | 20 |
| `dioxus_signals::signal` | 0 | 0 | 0 | 17 |
| `naga::back::spv::writer` | 0 | 0 | 0 | 622 |
| `naga::compact` | 0 | 0 | 0 | 1947 |
| `naga::compact::expressions` | 0 | 0 | 0 | 9870 |
| `naga::compact::functions` | 0 | 0 | 0 | 142 |
| `naga::compact::handle_set_map` | 0 | 0 | 0 | 20854 |
| `naga::proc::constant_evaluator` | 0 | 0 | 0 | 986 |
| `naga::proc::type_methods` | 0 | 0 | 0 | 4 |
| `style::sharing` | 0 | 0 | 0 | 58 |
| `style::style_resolver` | 0 | 0 | 0 | 102 |
| `style::traversal` | 0 | 0 | 0 | 156 |
| `warnings::warnings` | 0 | 0 | 0 | 17 |
| `wgpu_core::command` | 0 | 0 | 0 | 12 |
| `wgpu_core::command::pass` | 0 | 0 | 0 | 16 |
| `wgpu_core::command::render` | 0 | 0 | 0 | 12 |
| `wgpu_core::device::global` | 0 | 0 | 0 | 80 |
| `wgpu_core::device::queue` | 0 | 0 | 0 | 11 |
| `wgpu_core::device::resource` | 0 | 0 | 0 | 18 |
| `wgpu_core::device::surface_config` | 0 | 0 | 0 | 4 |
| `wgpu_core::instance` | 0 | 0 | 0 | 5 |
| `wgpu_core::resource` | 0 | 0 | 0 | 16 |
| `wgpu_hal::gles::adapter` | 0 | 0 | 0 | 1 |
| `wgpu_hal::gles::egl` | 0 | 0 | 0 | 70 |
| `wgpu_hal::vulkan::instance` | 0 | 0 | 0 | 4 |
| **27 rows, lines** | **0** | **0** | **0** | **35069** |

#### Lines off the sink's line shape

none — every stderr line of every before- and after-reading had the sink's line shape.

#### Rows kept

`INFO escher_telemetry`

## After the fix, by binary

- **Session host:** 0 lines with `RUST_LOG` unset; 1 line (`INFO escher_telemetry`, 124 B) at `info`, `debug` and
  `trace`; 0 of 15 ids and 0 of 6 names at every setting; stdout 0 B; exit 0; no state directory left.
- **Windowed stand:** 0 lines with `RUST_LOG` unset; 1 line (`INFO escher_telemetry`, 124 B) at `info`, `debug` and
  `trace`; 0 of 11 ids and 0 of 5 names at every setting; stdout 0 B; exit 124 (still up).
