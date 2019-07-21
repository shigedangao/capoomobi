mod cli;
mod docker;
mod errors;
mod kubernetes;

extern crate tokio;

fn main() {
    cli::bootstrap();
}
