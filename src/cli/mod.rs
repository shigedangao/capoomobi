mod scenarios;
mod core;
mod configurator;

pub fn bootstrap() {
  // scenarios::welcome::welcome_scn::bootstrap();
  // Parse the arguments and launch a scenario
  match core::parser::cli_parser::parse_arguments() {
    Err(err) => println!("{:?}", err),
    Ok(args) => {
      let main = args.main.to_owned();
      let sub  = args.sub.to_owned(); 
      scenarios::bootstrap::init(main.as_str(), sub.as_str(), args.options)
    }
  }
}