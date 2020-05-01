extern crate clap;

mod cli;
mod interface;
mod parser;
mod peer;

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
