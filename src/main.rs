use clap::Command;

mod kitin;
mod project;

fn cli() -> Command<'static> {
    Command::new("kitin")
        .about("A simple project manager")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(project::subcommand())
}

fn main() {
    let app = cli();
    let matches = app.try_get_matches().unwrap_or_else(|e| e.exit());

    match matches.subcommand() {
        Some(("project", sub_matches)) => {
            project::run(sub_matches);
        }
        _ => {}
    }
}
