mod cli;
mod docker;
mod errors;
mod kubernetes;

fn main() {
    cli::bootstrap();
}
