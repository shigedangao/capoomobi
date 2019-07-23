mod cli;
mod docker;
mod errors;
mod kubernetes;

extern crate tokio;
#[macro_use] extern crate handlebars;

fn main() {
    cli::bootstrap();
}
