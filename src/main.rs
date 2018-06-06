mod workspace;

#[macro_use]
extern crate serde_derive;
extern crate clap;

use workspace::Workspace;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::env;

fn main() {
    let matches = App::new("workspace")
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
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes a specified workspace in this directory, if present")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the workspace to delete")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        new(matches);
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        delete(matches);
    }
}

fn new(matches: &ArgMatches) {
    let ws = Workspace {
        name: matches.value_of("NAME").unwrap().to_string(),
        path: env::current_dir().expect("ERROR: Could not read current directory"),
    };
    if ws.exists() {
        eprintln!("ERROR: A workspace called \"{}\" already exists", ws.name);
        std::process::exit(1);
    }
    ws.write();
    println!("Created workspace \"{}\" in {:?}", ws.name, ws.path);
}

fn delete(matches: &ArgMatches) {
    let ws = Workspace {
        name: matches.value_of("NAME").unwrap().to_string(),
        path: env::current_dir().expect("ERROR: Could not read current directory"),
    };
    if !ws.exists() {
        eprintln!("ERROR: A workspace called \"{}\" does not exist", ws.name);
        std::process::exit(1);
    }
    ws.delete();
    println!("Deleted workspace \"{}\" in {:?}", ws.name, ws.path);
}