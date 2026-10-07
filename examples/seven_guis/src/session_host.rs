//! The `escher-session` host: boots one lean stand task headlessly and serves it as a driver
//! session in a state directory until it is stopped.

#[cfg(not(target_arch = "wasm32"))]
fn main() -> std::process::ExitCode {
    use std::path::Path;
    use std::process::ExitCode;

    use escher_driver::{Session, serve};
    use seven_guis::stand::{self, LeanTask};

    /// The lean tasks by the stand's own slugs.
    const TASKS: [(&str, LeanTask); 4] = [
        ("counter", LeanTask::Counter),
        ("flight-booker", LeanTask::FlightBooker),
        ("timer", LeanTask::Timer),
        ("crud", LeanTask::Crud),
    ];

    if let Err(error) = escher_telemetry::init(escher_telemetry::service_identity!()) {
        eprintln!("telemetry not installed: {error}");
    }

    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let task = match args.as_slice() {
        [slug, _] => TASKS.iter().find(|(known, _)| slug == known),
        _ => None,
    };
    let (Some(&(slug, task)), Some(state_dir)) = (task, args.get(1)) else {
        eprintln!("usage: escher-session <counter|flight-booker|timer|crud> <state-dir>");
        return ExitCode::from(2);
    };

    // The timer's tick handle lives as long as the session does: its session holds it as the
    // step that moves the timer's time.
    let mut ticks = None;
    let served = Session::start(slug, || match task {
        LeanTask::Timer => {
            let (harness, handle) = stand::boot_timer(stand::options(true));
            ticks = Some(handle);
            harness
        }
        task => stand::boot(task, stand::options(true)),
    })
    .map(|session| match ticks {
        Some(handle) => session.with_time(stand::timer_step(handle)),
        None => session,
    })
    .and_then(|session| serve(Path::new(state_dir), session));

    match served {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {}
