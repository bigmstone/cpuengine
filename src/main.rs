use std::env;

use env_logger;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args[1] == "line" {
        cpuengine::render_line(&args);
    } else if args[1] == "object" {
        cpuengine::render_obj(&args);
    }
}
