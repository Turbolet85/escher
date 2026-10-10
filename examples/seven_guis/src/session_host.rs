//! The `escher-session` binary: the driver's command line over the four lean stand tasks. As a
//! client it runs one driver command against a session in a state directory; in its host role
//! (`serve <task> --session <dir>`) it boots one task headlessly and serves it as that session
//! until it is stopped or idle.

#[cfg(not(target_arch = "wasm32"))]
fn main() -> std::process::ExitCode {
    use escher_driver::{Session, SessionError, command_line};
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

    let slugs = TASKS.map(|(slug, _)| slug);
    command_line(std::env::args_os().skip(1), &slugs, |slug| {
        let Some(&(slug, task)) = TASKS.iter().find(|(known, _)| *known == slug) else {
            return Err(SessionError::InvalidLabel);
        };
        // The timer's tick handle lives as long as the session does: its session holds it as
        // the step that moves the timer's time.
        let mut ticks = None;
        let session = Session::start(slug, || match task {
            LeanTask::Timer => {
                let (harness, handle) = stand::boot_timer(stand::options(true));
                ticks = Some(handle);
                harness
            }
            task => stand::boot(task, stand::options(true)),
        })?;
        Ok(match ticks {
            Some(handle) => session.with_time(stand::timer_step(handle)),
            None => session,
        })
    })
}

#[cfg(target_arch = "wasm32")]
fn main() {}
