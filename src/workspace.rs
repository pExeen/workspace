extern crate serde;
extern crate serde_yaml;

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub name: String,
    pub path: PathBuf,
}

impl Workspace {
    pub fn write(&self) -> &Self {
        const ERR_MESSAGE: &str = "ERROR: Could not write workspace data";

        let path = self.data_path();
        let mut file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open(path)
            .expect(ERR_MESSAGE);

        let serialized = serde_yaml::to_string(self).unwrap();
        file.write_fmt(format_args!("{}", serialized))
            .expect(ERR_MESSAGE);

        self
    }

    pub fn delete(&self) -> &Self {
        let path = self.data_path();
        fs::remove_file(path).expect("ERROR: Could not delete workspace data");
        self
    }

    pub fn exists(&self) -> bool {
        self.data_path().exists()
    }

    fn data_path(&self) -> PathBuf {
        let mut path = data_path();
        path.push(&self.name);
        path.set_extension("yaml");

        path
    }
}

pub fn all() -> Vec<Workspace> {
    files().into_iter().map(parse).collect()
}

pub fn parse(mut file: fs::File) -> Workspace {
    const ERR_MESSAGE: &str = "ERROR: could not read workspace data";

    let mut content = String::new();
    file.read_to_string(&mut content).expect(ERR_MESSAGE);

    serde_yaml::from_str(&content).expect(ERR_MESSAGE)
}

pub fn files() -> Vec<fs::File> {
    const ERR_MESSAGE: &str = "ERROR: could not get workspace data";
    let mut files: Vec<fs::File> = Vec::new();

    for entry in fs::read_dir(data_path()).expect(ERR_MESSAGE) {
        let entry = entry.expect(ERR_MESSAGE);
        let file_type = entry.file_type();
        if file_type.is_ok() && file_type.unwrap().is_file() {
            let path = entry.path();
            let file = fs::OpenOptions::new()
                .read(true)
                .open(&path)
                .expect(ERR_MESSAGE);
            let extension = path.extension();
            if extension.is_some() && extension.unwrap().to_string_lossy() == "yaml" {
                files.push(file);
            }
        }
    }

    files
}

pub fn data_path() -> PathBuf {
    let mut path = env::home_dir().expect("ERROR: Could not find home directory");
    path.push(".workspace");

    if !path.exists() {
        fs::create_dir(&path)
            .unwrap_or_else(|_| panic!("ERROR: Could not create directory {}", path.display()))
    }

    path
}
