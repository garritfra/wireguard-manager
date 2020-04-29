use clap::{App, SubCommand};

pub fn command<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("interfaces")
    .alias("if")
    .alias("interface")
    .subcommand(SubCommand::with_name("list"))
    .about("Interface tools")
}

pub fn run(args: &clap::ArgMatches) {
  println!("{:?}", args.is_present("list"));
}
