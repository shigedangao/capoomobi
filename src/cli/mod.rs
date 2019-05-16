mod scenarios;
mod parser;

pub fn lol() {
  println!("lol");
  scenarios::welcome::welcome_scn::bootstrap();
  match parser::cli_parser::parse_arguments() {
    Err(err) => println!("{:?}", err),
    Ok(args) => println!("{:?}", args.main)
  }
  
}