mod cli;
mod docker;
mod errors;
mod kubernetes;
mod confiture;
mod bootstrap;

extern crate tokio;
extern crate handlebars;

fn main() {
    bootstrap::manager::start();
}
