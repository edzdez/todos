use std::{fs, result};
use std::fs::{create_dir_all, File};
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use dirs::config_dir;
use crate::options::{AppMode, Options};
use crate::tasks::{Task, Tasks, Urgency};
use crate::error::Error;

mod options;
mod tasks;
mod error;

type Result<T, E = Error> = result::Result<T, E>;

fn open_file(opts: &Options) ->  Result<PathBuf> {
    match &opts.todo_list_location {
        Some(path) => Ok(path.clone()),
        None => {
            let mut default_dir = config_dir().expect("unable to access default config directory");
            default_dir.push("todos");
            if !default_dir.exists() {
                create_dir_all(&default_dir)?;
                    // .with_context(|| format!("could not create directory {:?}", default_dir))?;
            }

            default_dir.push("todos.json");
            if !default_dir.exists() {
                File::create(&default_dir)?;
                    // .with_context(|| format!("could not create file {:?}", default_dir))?;
            }

            Ok(default_dir)
        }
    }
}

fn read_todos(opts: &Options) -> Result<Tasks> {
    let file_path = open_file(opts)?;

    let contents = fs::read_to_string(&file_path)?;
        // .with_context(|| format!("file `{:?}` does not exist", file_path))?;
    match serde_json::from_str(&contents) {
        Ok(tasks) => Ok(tasks),
        Err(e) => {
            println!("{:?}, Initializing tasks as default", e);
            Ok(Tasks::new())
        }
    }
}

fn write_tasks(opts: &Options, tasks: &mut Tasks) -> Result<()> {
    let path = open_file(opts)?;
    tasks.sort_tasks();
    let mut file = File::create(&path)?;
        // .with_context(|| format!("Failed to open file `{:?}` for writing", path))?;

    let serialized = serde_json::to_string_pretty(tasks)?;
        // .with_context(|| format!("Failed to serialize tasks to json"))?;


    file.write_all(serialized.as_bytes())?;

    Ok(())
}

fn list_todos(opts: &Options, num_to_show: usize) -> Result<()> {
    let mut todos = read_todos(opts)?;
    println!("\x1b[1;4mYour Todos:\x1b[0m");

    todos.sort_tasks();
    match todos.tasks.len() {
        0 => println!("\x1b[32mAll Done!\x1b[0m"),
        _ => {
            for (idx, task) in todos.tasks.iter().enumerate() {
                if idx >= num_to_show {
                    break;
                }

                println!("{}: {}", idx + 1, task);
            }
        }
    }

    println!();
    Ok(())
}

fn add_todo(opts: &mut Options) -> Result<()> {
    let mut todos = read_todos(opts)?;
    println!("\x1b[1;4mAdd an Entry:\x1b[0m");

    println!("Enter contents:");
    let mut contents = String::new();
    stdout().flush()?;
    stdin().read_line(&mut contents).unwrap();

    println!("Enter urgency: (Low, Medium, High)");
    let mut urgency = String::new();
    stdout().flush()?;
    stdin().read_line(&mut urgency).unwrap();

    todos.tasks.push(Task::new(contents.trim().to_owned(), Urgency::from(urgency)));

    write_tasks(opts, &mut todos)?;
    println!();

    list_todos(opts, todos.tasks.len())
}

fn remove_todo(opts: &mut Options) -> Result<()> {
    let mut todos = read_todos(opts)?;
    println!("\x1b[1;4mRemove an Entry:\x1b[0m");
    list_todos(opts, todos.tasks.len())?;

    println!("Enter the number of the entry to remove:");
    let mut entry = String::new();
    stdout().flush()?;
    stdin().read_line(&mut entry).unwrap();
    entry = String::from(entry.trim());

    let entry = entry.parse::<usize>()?;
        // .with_context(|| format!("invalid entry: Please enter a number"))?;

    if entry > todos.tasks.len() {
        println!("Please enter the number of an entry: {} is too high", entry);
    } else {
        todos.tasks.remove(entry - 1);

        write_tasks(opts, &mut todos)?;
    }

    println!();
    list_todos(opts, todos.tasks.len())
}

fn main() -> Result<()> {
    let mut opts = Options::from_args();

    match opts.mode {
        AppMode::List { num_to_show } => list_todos(&opts, num_to_show)?,
        AppMode::Add => add_todo(&mut opts)?,
        AppMode::Remove => remove_todo(&mut opts)?,
    }

    Ok(())
}
