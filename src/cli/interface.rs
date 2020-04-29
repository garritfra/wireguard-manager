use clap::{App, SubCommand};

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
  let files = std::fs::read_dir("/etc/wireguard").unwrap();

  for path in files {
    let file_string: &str = &path
      .unwrap()
      .file_name()
      .into_string()
      .unwrap_or(String::new());
    println!("{}", file_string.split('.').take(1).collect::<Vec<_>>()[0]);
  }
}
