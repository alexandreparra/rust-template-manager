extern crate core;

mod env_var;
mod file_util;

use std::{env, fs};
use std::io;
use std::io::{Result, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use crate::file_util::un_path;

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
        ["create", file, flags @ ..] => {
            if flags.is_empty() {
                create_file(file, false);
            }

            if flags[0] == "-ne" {
                create_file(file, true);
            } else {
                println!("Unknown {} flag", flags[0]);
            }
        }
        ["delete", file] => delete_file(file),
        ["delete", files @ .. ] => delete_files(files),
        ["edit", file] => edit_file(file),
        ["help"] => help_message(),
        [] => help_message(),
        x => {
            println!("Command '{}' doesn't exist \nRun 'rtm help' to display commands", x[0]);
        }
    }
}

fn help_message() {
    println!("
rtm 0.1.0

Easily manage your template files through the CLI.

USAGE:
    rtm [COMMAND]

COMMAND:
    copy   [FILE]          Copy the desired template file inside the current folder.
    create [FILE] [FLAGS]  Create a file inside your default template folder.
    delete [FILE]...       Delete files inside your default template folder.
    edit   [FILE]          Edit a template file with your
    list                   List your template files.
    folder                 Display the path to your default template folder.
    help                   Prints this message.

FLAGS:
    -ne               No-edit, supress and ignore the choice to edit a newly created file.
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

fn create_file(file: &str, no_edit: bool) {
    let mut file_path = un_path();
    file_path.push(file);

    if let Err(e) = fs::File::create(&file_path) {
        println!("{e}");
    } else {
        println!("File created.");
        if !no_edit && ask_user_to_open_editor(None) {
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

fn edit_file(file: &str) {
    let mut file_path = un_path();
    file_path.push(file);
    open_editor(&file_path);
}

fn list_files() {
    let default_dir = un_path();

    if let Ok(entries) = fs::read_dir(default_dir) {
        for entry in entries.flatten() {
            println!("{}", entry.file_name().to_str().unwrap());
        }
    }
}

fn ask_user_to_open_editor(message: Option<&str>) -> bool {
    if let Some(m) = message {
        print!("{m}");
    } else {
        print!("Do you want to edit this file? (y/n) ");
    }

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

    if let Some(visual) = editor.visual {
        let result = open_file(visual, file_path);
        if result.is_ok() {
            return;
        }
    }

    if !ask_user_to_open_editor(
        Some("Failed to open visual editor, do you want to open the file on your terminal? (y/n)")
    ) {
        return;
    }

    if let Some(editor) = editor.editor {
        let result = open_file(editor, file_path);
        if result.is_err() {
            println!("Couldn't open any of your default editors");
        }
    }
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
