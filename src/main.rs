extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::Write;

extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct Workspace {
    name: String,
    path: PathBuf,
}

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
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        new(matches);
    }
}

fn new(matches: &ArgMatches) {
    let workspace = Workspace {
        name: matches.value_of("NAME").unwrap().to_string(),
        path: env::current_dir().expect("Could not read current directory"),
    };
    write(&workspace);
    println!(
        "Created workspace \"{}\" in {:?}",
        workspace.name, workspace.path
    );
}

fn write(workspace: &Workspace) {
    let serialized = serde_yaml::to_string(&workspace).unwrap();

    let mut path = data_path();
    path.push(&workspace.name);
    path.set_extension("yaml");

    let mut file: fs::File = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(path)
        .expect("Could not open workspace data");

    file.write_fmt(format_args!("{}", serialized))
        .expect("Could not write workspace data");
}

fn data_path() -> PathBuf {
    let mut path = env::home_dir().expect("Could not find home directory");
    path.push(".workspace");

    if !path.exists() {
        fs::create_dir(&path).expect(&format!("Could not create directory {:?}", path));
    }

    return path;
}
