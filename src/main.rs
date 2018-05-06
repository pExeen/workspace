extern crate clap;
use clap::App;

fn main() {
	App::new("workspace")
		.version("0.0.0")
		.about("Manages workspaces for all your projects!")
		.author("Matthias T. and Roma B.")
		.get_matches();
}