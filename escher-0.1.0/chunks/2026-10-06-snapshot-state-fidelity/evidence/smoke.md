# P3 smoke: the windowed stand boots with the widened falsy-clear list

- **When:** 2026-10-06T22:14Z, at /implement P3, by hand (the plan names it P3's smoke, not a gate entry).
- **Why it fires:** `packages/dioxus-native-dom/src/mutation_writer.rs` is on the windowed app's boot path — every attribute of every element the app mounts goes through `set_attribute_inner`.
- **Form:** the plan names `just seven_guis` (`cargo run --release --package seven_guis --bin seven_guis_native`). No release build exists on the host, so the same binary was built in the dev profile with the recipe's own feature set: `cargo build -p seven_guis --bin seven_guis_native --locked`, exit 0, the binary's mtime 22:14:29Z (after this chunk's last source edit).
- **Boot:** `timeout -s TERM -k 5 15 ./target/debug/seven_guis_native`, started 22:14:36Z, exited 124 at 22:14:51Z — it stayed up for the whole bound. Stdout 0 bytes. Stderr one line: winit's Wayland `WARN` that `xdg_toplevel_icon_manager_v1` is not supported (the same line the earlier smokes read). 0 lines matching `panic`, `already borrowed` or `BorrowMutError`.
- **Process census:** no `seven_guis` process before the boot and none after it (`pgrep -a seven_guis`, both reads empty).
- **What it does not prove:** nothing was pressed in the window, so it proves the boot (Home mounted through the bridge) and not a task's toggling attributes; those are proved headlessly by `dioxus_falsy_boolean_attrs` and the stand checks. The binary names no `accessibility` feature, so the snapshot module and `MASKED_VALUE` are not compiled into it.
