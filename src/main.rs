use std::env;

use app::handler::Handler;

mod app;
mod entity;
mod infra;

fn main() {
    let args: Vec<String> = env::args().collect();
    let handler = Handler::new();
    let _ = handler.run(args).map_err(|err| println!("{}", err));
}
