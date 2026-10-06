// On Windows do NOT show a console window when opening the app
#![cfg_attr(all(not(test), target_os = "windows"), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    if let Err(error) = escher_telemetry::init(escher_telemetry::service_identity!()) {
        eprintln!("telemetry not installed: {error}");
    }
    dioxus_native::launch(seven_guis::app::app);
}

#[cfg(target_arch = "wasm32")]
fn main() {}
