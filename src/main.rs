use clap::Command;

mod initkit;
mod kit;

fn cli() -> Command<'static> {
    Command::new("kitin")
        .about("A simple project manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(initkit::subcommand())
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            initkit::run(sub_matches);
        }
        _ => unreachable!(),
    }
}
