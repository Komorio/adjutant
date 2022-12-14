use super::App;
use super::UserCursor;
use super::Vec2;
use super::Widget;

use clap::{arg, Command};
use std::{
    fs,
    io::{Read, Write},
};

const DATA_PATH: &str = "./.adjutant/tasks/TASK.dat";

fn option_add() -> clap::Arg<'static> {
    arg!(-a --add <TASK_NAME>).required(false)
}

fn option_show() -> clap::Arg<'static> {
    arg!(-s - -show).required(false)
}

pub fn command_task() -> Command<'static> {
    Command::new("task")
        .about("task")
        .args([option_add(), option_show()])
        .arg_required_else_help(true)
        .subcommand(Command::new("init").about("initialize task manager"))
}

pub fn task_init() {
    let already_initialized = fs::metadata("./.adjutant");

    if already_initialized.is_ok() {
        println!("already initialized");
        return;
    }

    let result = fs::create_dir_all("./.adjutant/tasks");

    match result {
        Ok(_) => {
            fs::File::create(DATA_PATH).expect("Failed create task data.");
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

    let mut task_data = load_editable_task_data(true);

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

    let mut app = App::new(UserCursor::from(2, todo_tasks.len() as u16));

    const TODO_X: u16 = 5;
    const DONE_X: u16 = 45;

    loop {
        let mut widgets: Vec<Widget> = vec![];

        let todo_title = Widget {
            position: Vec2::from(TODO_X, 3),
            content: String::from("TODO"),
            has_highlight: app.cursor.position.x == 0,
        };

        let done_title = Widget {
            position: Vec2::from(DONE_X, 3),
            content: String::from("DONE"),
            has_highlight: app.cursor.position.x == 1,
        };

        widgets.push(todo_title);
        widgets.push(done_title);

        if todo_tasks.len() > 0 {
            for index in 0..todo_tasks.len() {
                let y = 3 + (index + 1) * 2;

                let widget = Widget {
                    position: Vec2::from(TODO_X, y as u16),
                    content: todo_tasks[index].clone(),
                    has_highlight: app.cursor.position.x == 0
                        && app.cursor.position.y == index as u16,
                };

                widgets.push(widget);
            }
        }

        if done_tasks.len() > 0 {
            for index in 0..done_tasks.len() {
                let y = 3 + (index + 1) * 2;

                let widget = Widget {
                    position: Vec2::from(DONE_X, y as u16),
                    content: done_tasks[index].clone(),
                    has_highlight: app.cursor.position.x == 1
                        && app.cursor.position.y == index as u16,
                };

                widgets.push(widget);
            }
        }

        app.render(widgets);

        let prev_x = app.cursor.position.x;
        let key = app.update();

        if app.is_ended {
            save_tasks(todo_tasks, done_tasks);
            break;
        }

        match key {
            'D' => {
                if app.cursor.position.x == 0 {
                    let index = app.cursor.position.y as usize;

                    done_tasks.push(todo_tasks[index].clone().replace("-[ ]", "-[X]"));
                    todo_tasks.remove(index);
                } else if app.cursor.position.x == 1 {
                    let index = app.cursor.position.y as usize;

                    todo_tasks.push(done_tasks[index].clone().replace("-[X]", "-[ ]"));
                    done_tasks.remove(index);
                }
            }
            _ => (),
        };

        if prev_x != app.cursor.position.x {
            if app.cursor.position.x == 0 {
                app.cursor.max_y = todo_tasks.len() as u16;
            } else if app.cursor.position.x == 1 {
                app.cursor.max_y = done_tasks.len() as u16;
            }

            if app.cursor.position.y >= app.cursor.max_y {
                if app.cursor.max_y > 0 {
                    app.cursor.position.y = app.cursor.max_y - 1;
                } else {
                    app.cursor.position.y = 0;
                }
            }
        }
    }
}

fn save_tasks(todo_tasks: Vec<String>, done_tasks: Vec<String>) {
    let mut file = load_editable_task_data(false);

    for index in 0..todo_tasks.len() {
        file.write(todo_tasks[index].as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }

    for index in 0..done_tasks.len() {
        file.write(done_tasks[index].as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }
}

pub fn load_task_data() -> fs::File {
    let file = fs::File::open(DATA_PATH);

    let file = match file {
        Ok(f) => f,
        Err(_) => {
            println!("Couldn't load task data file.");
            std::process::exit(0);
        }
    };

    file
}

pub fn load_editable_task_data(is_append: bool) -> fs::File {
    let file = fs::OpenOptions::new()
        .write(true)
        .append(is_append)
        .open(DATA_PATH);

    let file = match file {
        Ok(f) => f,
        Err(_) => {
            println!("Couldn't load task data file.");
            std::process::exit(0);
        }
    };
    file
}
