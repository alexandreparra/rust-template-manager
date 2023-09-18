extern crate core;

#[cfg(target_os = "linux")]
mod env_var;

mod file;

use std::{env, fs};
use std::io;
use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
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
                return ExitCode::SUCCESS;
            }

            if flags[0] == "-ne" {
                create_file(file, true);
            } else {
                println!("Unknown {} flag", flags[0]);
                return ExitCode::FAILURE;
            }
        }
        ["delete", file] => delete_file(file),
        ["delete", files @ .. ] => delete_files(files),
        ["edit", file] => edit_file(file),
        ["help"] => help_message(),
        [] => help_message(),
        x => {
            println!("Command '{}' doesn't exist \nRun 'rtm help' to display commands", x[0]);
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
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
    println!("Your default template folder is: {:?}", file::unwrap_path());
}

fn copy_file(name: &str) {
    let mut default_path = file::unwrap_path();
    default_path.push(name);

    if let Err(e) = fs::copy(default_path, name) {
        println!("{e}");
    } else {
        println!("File copied!");
    }
}

fn create_file(file: &str, no_edit: bool) {
    let mut file_path = file::unwrap_path();
    file_path.push(file);

    if let Err(e) = fs::File::create(&file_path) {
        println!("{e}");
    } else {
        println!("File created.");

        #[cfg(not(target_os = "windows"))]
        if !no_edit && ask_user_to_open_editor(None) {
            file::open_editor(&file_path);
        }
    }
}

fn delete_files(files: &[&str]) {
    for file in files {
        delete_file(file);
    }
}

fn delete_file(file: &str) {
    let mut file_path = file::unwrap_path();
    file_path.push(file);

    if let Err(e) = fs::remove_file(file_path) {
        println!("{e}");
    } else {
        println!("File deleted");
    }
}

fn edit_file(file: &str) {
    let mut file_path = file::unwrap_path();
    file_path.push(file); file::open_editor(&file_path);
}

fn list_files() {
    let default_dir = file::unwrap_path();

    if let Ok(entries) = fs::read_dir(default_dir) {
        for entry in entries.flatten() {
            println!("{}", entry.file_name().to_str().unwrap());
        }
    } else {
        println!("No template files created.");
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
