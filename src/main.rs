extern crate adjutant;

use adjutant::task::command_task;
use clap::Command;

fn cli() -> Command<'static> {
    Command::new("adjutant")
        .author("Astellar")
        .about("work manager cli")
        .version("0.0.1")
        .subcommand_required(true)
        .allow_external_subcommands(true)
        .subcommand(command_task())
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("task", sub_matches)) => {
            if sub_matches.contains_id("show") {
                // TODO : Shows tasks
                println!("SHOW: TASK_NAMES");
                return;
            }

            match sub_matches.subcommand() {
                Some(("init", _)) => {
                    println!("INIT");
                }
                _ => (),
            };

            let (add, done) = (
                sub_matches.get_one::<String>("add"),
                sub_matches.get_one::<String>("done"),
            );

            match add {
                Some(task_name) => {
                    // TODO : Add task
                    println!("ADD: {}", task_name);
                }
                _ => (),
            };

            match done {
                Some(task_name) => {
                    // TODO : Make done
                    println!("DONE: {}", task_name);
                }
                _ => (),
            };
        }
        _ => {
            println!("?");
        }
    };
}
