extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    App::new("workspace")
        .version("0.0.0")
        .about("Manages workspaces for all your projects!")
        .author("Matthias T. and Roma B.")
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a new workspace in this directory")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the new workspace")
                        .required(true),
                ),
        )
        .get_matches();
}
