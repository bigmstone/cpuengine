mod logger;

use std::env;

use log::{LevelFilter, SetLoggerError};

static LOGGER: logger::SimpleLogger = logger::SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}

fn main() {
    init().expect("Error initializing render.");
    let args: Vec<String> = env::args().collect();

    if args[1] == "line" {
        cpuengine::render_line(&args);
    } else if args[1] == "object" {
        cpuengine::render_obj(&args);
    } else if args[1] == "triangle" {
        cpuengine::render_triangle(&args);
    }
}
