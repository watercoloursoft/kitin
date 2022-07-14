use std::{path::PathBuf, str::FromStr};

use clap::{AppSettings, Arg, Command};

use crate::kitin::{self, KitinModule, KitinModuleSourceType, KitinProject};

pub fn subcommand() -> Command<'static> {
    return Command::new("project")
        .about("Kitin project related commmands")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(subcommand_init())
        .subcommand(subcommand_add());
}

pub fn subcommand_init() -> Command<'static> {
    return Command::new("init")
        .about("Initialise a new project")
        .arg(Arg::with_name("name").required(false).index(1));
}

pub fn subcommand_add() -> Command<'static> {
    return Command::new("add")
        .about("Add a module to a project")
        .arg(Arg::with_name("path").required(true).index(1))
        .arg(Arg::with_name("source").required(true).index(2))
        .arg(
            Arg::with_name("source_type")
                .default_value("Git")
                .required(false)
                .index(3),
        );
}

pub fn run(matches: &clap::ArgMatches) {
    let mut project = kitin::KitinProject::new();
    let cwd = std::env::current_dir().unwrap();

    match matches.subcommand() {
        Some(("init", sub_matches)) => run_init(&mut project, &cwd, sub_matches),
        Some(("add", sub_matches)) => run_add(&mut project, &cwd, sub_matches),
        _ => {}
    }
}

pub fn run_init(project: &mut KitinProject, cwd: &PathBuf, matches: &clap::ArgMatches) {
    if kitin::KitinProject::directory_has_project(cwd.as_path()) {
        println!("Kitin project already exists in this directory");
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

pub fn run_add(project: &mut KitinProject, cwd: &PathBuf, matches: &clap::ArgMatches) {
    if !kitin::KitinProject::directory_has_project(cwd.as_path()) {
        println!("No kitin project in current directory");
        return;
    };
    project.load_from_file();

    let path = matches.get_one::<String>("path").unwrap();
    let source = matches.get_one::<String>("source").unwrap();
    let source_type = matches.get_one::<String>("source_type").unwrap();
    let module = KitinModule::new(
        source.clone(),
        KitinModuleSourceType::from_str(&source_type).unwrap(),
    );
    project.add_module(path.clone(), module);
    project.save_to_file(cwd.as_path());
}
