extern crate clap;

mod cli;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("wireguard-manager")
        .subcommand(cli::interface::command())
        .get_matches();

    match matches.subcommand() {
        ("interfaces", Some(interfaces)) => cli::interface::run(interfaces),
        _ => {}
    }
}
