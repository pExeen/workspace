extern crate serde;
extern crate serde_yaml;

use std::fs;
use std::env;
use std::io::Write;
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

    pub fn exists(&self) -> bool {
        self.data_path().exists()
    }

    fn data_path(&self) -> PathBuf {
        let mut path = env::home_dir().expect("ERROR: Could not find home directory");
        path.push(".workspace");

        if !path.exists() {
            fs::create_dir(&path).expect(&format!("ERROR: Could not create directory {:?}", path));
        }

        path.push(&self.name);
        path.set_extension("yaml");

        path
    }
}
