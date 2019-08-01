mod cli;
mod docker;
mod errors;
mod kubernetes;
mod bootstrap;

extern crate tokio;
#[macro_use]
extern crate handlebars;

fn main() {
    bootstrap::manager::start();
}
