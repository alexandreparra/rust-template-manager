extern crate core;

mod env_var;

use dirs::{home_dir, template_dir};
use std::{env, fs};
use std::io;
use std::io::{Result, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut str_args: Vec<&str> = args
        .iter()
        .map(|s| { s.as_str() })
        .collect::<Vec<_>>();

    str_args.remove(0);

    match str_args.as_slice() {
        ["folder"] => template_folder(),
        ["list"] => list_files(),
        ["copy", file, ..] => copy_file(file),
        ["create", file, ..] => create_file(file),
        ["delete", file] => delete_file(file),
        ["delete", files @ .. ] => delete_files(files),
        [] => help_message(),
        _ => help_message()
    }
}

fn help_message() {
    println!("
rtm 0.1.0

Easily manage your template files through the CLI.

USAGE:
    rtm [COMMAND]

COMMAND:
    copy [FILE]       Copy the desired template file inside the current folder.
    create [FILE]     Create a file inside your default template folder.
    delete [FILE]...  Delete files inside your default template folder.
    list              List your template files.
    folder            Display the path to your default template folder.
    help              Prints this message.
");
}

fn template_folder() {
    println!("Your default template folder is: {:?}", un_path());
}

fn copy_file(name: &str) {
    let mut default_path = un_path();
    default_path.push(name);

    if let Err(e) = fs::copy(default_path, name) {
        println!("{e}");
    } else {
        println!("File copied!");
    }
}

fn create_file(file: &str) {
    let mut file_path = un_path();
    file_path.push(file);

    if let Err(e) = fs::File::create(&file_path) {
        println!("{e}");
    } else {
        println!("File created.");
        if ask_user_to_open_editor() {
            open_editor(&file_path);
        }
    }
}

fn delete_files(files: &[&str]) {
    for file in files {
       delete_file(file);
    }
}

fn delete_file(file: &str) {
    let mut file_path = un_path();
    file_path.push(file);

    if let Err(e) = fs::remove_file(file_path) {
        println!("{e}");
    } else {
        println!("File deleted");
    }
}

fn list_files() {
    let default_dir = un_path();

    if let Ok(entries) = fs::read_dir(default_dir) {
        for entry in entries.flatten() {
            println!("{}", entry.file_name().to_str().unwrap());
        }
    }
}

fn ask_user_to_open_editor() -> bool {
    print!("Do you want to edit this file? (y/n) ");
    io::stdout().flush().expect("Failed to print message");

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Couldn't read line");

        match line.trim().to_lowercase().as_str() {
            "yes" | "ye" | "y" => return true,
            "no" | "n" => return false,
            _ => continue,
        }
    }
}

fn open_editor(file_path: &PathBuf) {
    let editor = env_var::get_editor_env_var();
    open_file(editor, file_path).expect("Error while editing the file");
}

#[inline]
fn open_file(editor: String, file_path: &PathBuf) -> Result<()> {
    Command::new(editor)
        .arg(file_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

/// Unwraps the default template folder if it exists, otherwise use the fallback format.
fn un_path() -> PathBuf {
    match template_dir() {
        None => fallback_dir(),
        Some(dir) => dir
    }
}

fn fallback_dir() -> PathBuf {
    let default_dir = home_dir().expect("Couldn't find the default template folder");
    default_dir.join("Templates")
}
