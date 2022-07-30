use clap::{arg, Command};
use std::{
    fs,
    io::{Read, Write},
};

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

pub fn task_add(task: String) {
    let data = format!("-[ ] {}\n", task);

    let mut task_data = load_editable_task_data();

    task_data.write(data.as_bytes()).expect("Failed task add");
    task_data.flush().expect("Failed task data flush");

    println!("ADDED: {}", task);
}

pub fn task_show() {
    let mut task_data = load_task_data();

    let mut data = String::new();
    task_data
        .read_to_string(&mut data)
        .expect("Failed read data.");

    let mut todo_tasks: Vec<String> = vec![];
    let mut done_tasks: Vec<String> = vec![];

    for line in data.lines() {
        if line.starts_with("-[ ]") {
            todo_tasks.push(String::from(line));
        } else if line.starts_with("-[X]") {
            done_tasks.push(String::from(line));
        }
    }

    println!("-- TODO --\n");

    for task in todo_tasks {
        println!("{}", task);
    }

    println!("\n-- DONE --\n");

    for task in done_tasks {
        println!("{}", task);
    }
}

pub fn load_task_data() -> fs::File {
    let file = fs::File::open("./.adjutant/tasks/TASKS.dat");

    let file = match file {
        Ok(f) => f,
        Err(_) => {
            println!("Couldn't load task data file.");
            std::process::exit(0);
        }
    };

    file
}

pub fn load_editable_task_data() -> fs::File {
    let file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./.adjutant/tasks/TASKS.dat");

    let file = match file {
        Ok(f) => f,
        Err(_) => {
            println!("Couldn't load task data file.");
            std::process::exit(0);
        }
    };
    file
}
