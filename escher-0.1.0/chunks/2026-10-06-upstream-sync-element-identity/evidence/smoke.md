# Windowed boot smoke (gate entries 19-20, WAYLAND_DISPLAY set)

cargo build -p seven_guis --bin seven_guis_native --locked → exit 0
RUST_LOG=info timeout 10 target/debug/seven_guis_native; test $? -eq 124 → exit 0 (ran the full 10 s, killed by timeout)

First log line:
2026-10-06T08:14:21.746766Z INFO escher_telemetry service.name=seven_guis service.version=0.1.0 message=telemetry installed
