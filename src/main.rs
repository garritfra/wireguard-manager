extern crate clap;

mod cli;
use clap::App;

fn main() {
    let matches = App::new("wireguard-manager")
        .subcommand(cli::interface::command())
        .get_matches();

    match matches.subcommand() {
        ("interfaces", Some(args)) => cli::interface::run(args),
        _ => {}
    }
}
