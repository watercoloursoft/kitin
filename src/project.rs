use std::path::PathBuf;

use clap::{AppSettings, Arg, Command};

use crate::kitin::{self, KitinProject};

pub fn subcommand() -> Command<'static> {
    return Command::new("project")
        .about("Kitin project related commmands")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(subcommand_init());
}

pub fn subcommand_init() -> Command<'static> {
    return Command::new("init")
        .about("Initialise a new project")
        .arg(Arg::with_name("name").required(false).index(1));
}

pub fn run(matches: &clap::ArgMatches) {
    let mut project = kitin::KitinProject::new();
    let cwd = std::env::current_dir().unwrap();

    match matches.subcommand() {
        Some(("init", sub_matches)) => run_init(&mut project, &cwd, sub_matches),
        _ => {}
    }
}

pub fn run_init(project: &mut KitinProject, cwd: &PathBuf, matches: &clap::ArgMatches) {
    if kitin::KitinProject::directory_has_project(cwd.as_path()) {
        return;
    };

    let default_name = cwd
        .file_name()
        .and_then(|s| s.to_str())
        .and_then(|s| Some(s.to_string()))
        .unwrap();
    let name = matches.get_one::<String>("name").unwrap_or(&default_name);
    project.set_name(name);
    project.save_to_file(cwd.as_path());
}
