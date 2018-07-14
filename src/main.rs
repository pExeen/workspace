#[macro_use]
pub mod macros;
mod app;
pub mod exit;
mod shell;
mod workspace;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate clap;
extern crate colored;

use clap::ArgMatches;
use colored::*;
use exit::*;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path;
use std::process;
use workspace::Workspace;

pub static mut VERBOSE: bool = false;

fn main() {
    let matches = app::cli().get_matches();

    unsafe {
        VERBOSE = matches.is_present("verbose");
    }

    if let Some(matches) = matches.subcommand_matches("open") {
        open(matches);
    } else if let Some(matches) = matches.subcommand_matches("add") {
        add(matches);
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        delete(matches);
    } else if let Some(_matches) = matches.subcommand_matches("list") {
        list();
    } else if let Some(matches) = matches.subcommand_matches("shell") {
        shell(matches);
    }
}

fn open(matches: &ArgMatches) {
    let name: &str = matches.value_of("NAME").unwrap();
    let ws: Workspace = workspace::get(name)
        .unwrap_or_exit(&format!("A workspace called '{}' does not exist", name));
    if !ws.path.exists() {
        error!("The location of this workspace does not exist anymore");
        println!("The path '{}' was moved or deleted", ws.path.display());
        process::exit(1);
    }
    ws.open();
}

fn add(matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap().to_string();
    if workspace::exists(&name) {
        error!("A workspace called '{}' already exists", name);
        process::exit(1);
    }
    let ws = Workspace {
        path: env::current_dir().unwrap_or_exit("Could not read current directory"),
    };
    workspace::write(ws, &name);
    println!("Created workspace '{}' in {}", name, ws.path.display());
}

fn delete(matches: &ArgMatches) {
    let name: &str = matches.value_of("NAME").unwrap();
    let ws: Workspace = workspace::get(name)
        .unwrap_or_exit(&format!("A workspace called '{}' does not exist", name));

    if !matches.is_present("yes") {
        confirm!("delete the workspace '{}'", ws.name);
    }

    ws.delete();
    println!("Deleted workspace '{}' in {}", ws.name, ws.path.display());
}

fn list() {
    let all = workspace::all();

    if all.is_empty() {
        println!("No existing workspaces.\nRun `workspace add <NAME>` to create one.");
        return;
    }

    let rows: Vec<(&String, String, String)> = all
        .iter()
        .map(|(name, result)| {
            let path: String;
            let mut moved: String = String::default();
            match result {
                Ok(ws) => {
                    path = ws.path.display().to_string();
                    if !ws.path.exists() {
                        moved = format!("  {} path has moved", "warning:".bold().yellow());
                    }
                }
                Err(error) => {
                    path = format!("{} {}", "warning:".bold().yellow(), error.cause());
                }
            }
            (name, path, moved)
        })
        .collect();

    use std::cmp::max;
    let (longest_name_length, longest_path_length) = rows
        .iter()
        .map(|(name, path, _)| (name.len(), path.len()))
        .fold((0, 0), |(name1, path1), (name2, path2)| {
            (max(name1, name2), max(path1, path2))
        });

    for (name, path, moved) in rows {
        println!(
            "{0:<1$}  {2:<3$}{4}",
            name,
            longest_name_length,
            path.bright_black(),
            longest_path_length,
            moved
        );
    }
}

fn shell(matches: &ArgMatches) {
    if matches.subcommand_matches("bash").is_some() {
        println!("{}", shell::BASH);
    } else if matches.subcommand_matches("powershell").is_some() {
        println!("{}", shell::POWERSHELL)
    } else if let Some(matches) = matches.subcommand_matches("cmd") {
        let mut path: path::PathBuf = path_to_binary_or_arg(&matches);
        let mut file: fs::File = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .append(false)
            .truncate(true)
            .open(&path)
            .unwrap_or_exit(&format!(
                "Could not create batch file at {}",
                path.display()
            ));

        file.write_fmt(format_args!("{}", shell::CMD))
            .unwrap_or_exit("Could not write to batch file");

        println!("Wrote {}", path.display());
    }
}

fn path_to_binary_or_arg(matches: &ArgMatches) -> path::PathBuf {
    if let Some(path) = matches.value_of("PATH") {
        return path::Path::new(path)
            .with_file_name("ws")
            .with_extension("bat")
            .to_path_buf();
    } else {
        let mut path = env::current_exe().unwrap_or_exit("Could not determine path to binary");
        path.set_file_name("ws");
        path.set_extension("bat");
        return path;
    }
}
