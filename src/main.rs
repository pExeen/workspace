mod workspace;

#[macro_use]
extern crate serde_derive;
extern crate clap;

use workspace::Workspace;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::env;
use std::fs;

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
                .about("Deletes a specified workspace, if present")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the workspace to delete")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists all existing workspaces")
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        new(matches);
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        delete(matches);
    } else if let Some(_matches) = matches.subcommand_matches("list") {
        list();
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

fn list() {
    let dir_path = workspace::data_path();
    let paths = fs::read_dir(dir_path).unwrap();

    let mut workspaces: Vec<String> = Vec::new();
    for entry in paths {
        let path = entry.unwrap().path();
        // assuming all yaml files are workspaces
        // might need to change this if save other settings as yaml files
        if file_extension_from_path(&path) == "yaml" {
            let file_stem = file_stem_from_path(&path);
            workspaces.push(file_stem);
        }
    }
    if workspaces.len() == 0 {
        println!("No existing workspaces.\nRun `workspace new <NAME>` to create one.");
    } else {
        println!("Existing workspaces:");
        for ws in workspaces {
            println!("  {}", ws);
        }
    }
}

fn file_stem_from_path(path: &std::path::PathBuf) -> String {
    let file_stem = path.file_stem().unwrap();
    let os_string = file_stem.to_os_string();
    os_string.to_str().unwrap().to_string()
}

fn file_extension_from_path(path: &std::path::PathBuf) -> String {
    let file_stem = path.extension().unwrap();
    let os_string = file_stem.to_os_string();
    os_string.to_str().unwrap().to_string()
}
