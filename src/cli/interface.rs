use clap::{App, SubCommand};

use std::process::Command;

pub fn command<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("interfaces")
    .alias("if")
    .alias("interface")
    .subcommand(SubCommand::with_name("list").alias("ls"))
    .about("Interface tools")
}

pub fn run(args: &clap::ArgMatches) {
  match args.subcommand() {
    ("list", Some(_)) => list_interfaces(),
    _ => {}
  }
}

fn list_interfaces() {
  Command::new("wg")
    .args(&["show", "interfaces"])
    .output()
    .expect("failed to execute process");
}
