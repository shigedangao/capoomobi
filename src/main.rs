mod core;
mod cli;
mod docker;
mod kubernetes;
mod confiture;
mod bootstrap;
mod assets;

extern crate handlebars;

fn main() {
    bootstrap::manager::start();
}
