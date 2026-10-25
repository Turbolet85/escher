//! The `escher-session` binary: the driver's command line over six stand tasks — the four lean
//! tasks, Cells and the load page. As a client it runs one driver command against a session in a
//! state directory; in its host role (`serve <task> --session <dir>`) it boots one task
//! headlessly and serves it as that session until it is stopped or idle; in its `mcp` role it
//! speaks MCP on its standard streams, holding the apps it starts in its own process.

#[cfg(not(target_arch = "wasm32"))]
fn main() -> std::process::ExitCode {
    use escher_driver::{Session, SessionError, command_line};
    use seven_guis::stand::{self, LeanTask};

    /// A task the binary serves: a lean task, Cells or the load page, each its own boot.
    #[derive(Clone, Copy)]
    enum Served {
        Lean(LeanTask),
        Cells,
        Load,
    }

    /// The served tasks by the stand's own slugs.
    const TASKS: [(&str, Served); 6] = [
        ("counter", Served::Lean(LeanTask::Counter)),
        ("flight-booker", Served::Lean(LeanTask::FlightBooker)),
        ("timer", Served::Lean(LeanTask::Timer)),
        ("crud", Served::Lean(LeanTask::Crud)),
        ("cells", Served::Cells),
        ("load", Served::Load),
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
            Served::Lean(LeanTask::Timer) => {
                let (harness, handle) = stand::boot_timer(stand::options(true));
                ticks = Some(handle);
                harness
            }
            Served::Lean(task) => stand::boot(task, stand::options(true)),
            Served::Cells => stand::boot_cells(stand::options(true)),
            Served::Load => stand::boot_load(stand::options(true)),
        })?;
        Ok(match ticks {
            Some(handle) => session.with_time(stand::timer_step(handle)),
            None => session,
        })
    })
}

#[cfg(target_arch = "wasm32")]
fn main() {}
