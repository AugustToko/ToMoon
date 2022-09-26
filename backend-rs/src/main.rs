mod api;
mod settings;
mod control;

use simplelog::{WriteLogger, LevelFilter};
use usdpl_back::Instance;

const PORT: u16 = 55555;
fn main() -> Result<(),()>  {
    WriteLogger::init(
        #[cfg(debug_assertions)]{LevelFilter::Debug},
        #[cfg(not(debug_assertions))]{LevelFilter::Info},
        Default::default(),
        std::fs::File::create("/tmp/clashdeck.log").unwrap()
    ).unwrap();

    log::info!("Starting back-end ({} v{})", api::NAME, api::VERSION);
    println!("Starting back-end ({} v{})", api::NAME, api::VERSION);

    let runtime = control::ControlRuntime::new();
    runtime.run();

    Instance::new(PORT)
    .register("set_clash_status", api::set_clash_status(&runtime))
    .register("get_clash_status", api::get_clash_status(&runtime))
    .run_blocking()
}