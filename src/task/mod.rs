use clap::{arg, Command};
use std::fs;

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

pub fn task_init() {
    let already_initialized = fs::metadata("./.adjustant");

    if already_initialized.is_ok() {
        println!("already initialized");
        return;
    }

    let result = fs::create_dir_all("./.adjutant/tasks");

    match result {
        Ok(_) => {
            fs::File::create("./.adjutant/tasks/TASKS.dat").expect("Failed create task data.");
            println!("adjutant task initialized");
        }
        Err(err) => {
            println!("adjutant task initialize failed!");
            println!("Err: {}", err);
        }
    };
}
