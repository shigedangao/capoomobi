mod scenarios;
mod parser;
mod config;

pub fn bootstrap() {
  // scenarios::welcome::welcome_scn::bootstrap();
  // Parse the arguments and launch a scenario
  match parser::cli_parser::parse_arguments() {
    Err(err) => println!("{:?}", err),
    Ok(args) => scenarios::bootstrap::init(args.main, args.sub, args.options)
  }
}