use clap::{Arg, Command};

use crate::kit;

pub fn subcommand() -> Command<'static> {
    return Command::new("init")
        .about("Initialise a new kit project")
        .arg(Arg::with_name("name").required(false).index(1));
}

pub fn run(matches: &clap::ArgMatches) {
    let mut project = kit::KitProject::new();
    let cwd = std::env::current_dir().unwrap();
    if kit::KitProject::directory_has_project(cwd.as_path()) {
        project.load_from_file();
        println!("{:?}", project);
    };

    let default_name = cwd
        .file_name()
        .and_then(|s| s.to_str())
        .and_then(|s| Some(s.to_string()))
        .unwrap();
    let name = matches.get_one::<String>("name").unwrap_or(&default_name);
    print!("{}", name);
}
