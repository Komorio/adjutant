extern crate adjutant;

use adjutant::task::{command_task, task_add, task_init, task_show};
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
                task_show();
                return;
            }

            match sub_matches.subcommand() {
                Some(("init", _)) => {
                    task_init();
                    return;
                }
                _ => (),
            };

            let add = sub_matches.get_one::<String>("add");

            match add {
                Some(task_name) => {
                    task_add(task_name.clone());
                    return;
                }
                _ => (),
            };
        }
        _ => {
            println!("?");
        }
    };
}
