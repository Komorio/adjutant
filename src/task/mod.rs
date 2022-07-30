use clap::{arg, Command};

fn option_add() -> clap::Arg<'static> {
    arg!(-a --add <TASK_NAME>).required(false)
}

fn option_done() -> clap::Arg<'static> {
    arg!(-d --done <TASK_NAME>).required(false)
}

fn option_show() -> clap::Arg<'static> {
    arg!(-s - -show).required(false)
}

pub fn command_task() -> Command<'static> {
    Command::new("task")
        .about("task")
        .args([option_add(), option_done(), option_show()])
        .arg_required_else_help(true)
        .subcommand(Command::new("init").about("initialize task manager"))
}
