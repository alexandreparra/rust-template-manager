extern crate core;

#[cfg(not(target_os = "windows"))]
pub mod env_var;

mod file;
mod util;

use std::{env, fs};
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
                create_file(file, false, false);
                return ExitCode::SUCCESS;
            }

            match flags[0] {
                "-ne" => {
                    create_file(file, true, false);
                }
                #[cfg(not(target_os = "windows"))]
                "-pv" => {
                    create_file(file, false, true);
                }
                _ => {
                    println!("Unknown flag: '{}' for create command", flags[0]);
                    return ExitCode::FAILURE;
                }
            }
        }
        ["delete", file] => delete_file(file),
        ["delete", files @ .. ] => delete_files(files),
        ["edit", file, flags @ ..] => {
            if flags.is_empty() {
                edit_file(file, false);
                return ExitCode::SUCCESS;
            }

            match flags[0] {
                #[cfg(not(target_os = "windows"))]
                "-pv" => {
                    edit_file(file, true);
                }
                _ => {
                    println!("Unknown flag: '{}' for edit command", flags[0]);
                    return ExitCode::FAILURE;
                }
            }

        },
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
rtm 0.2.0

Easily manage your template files through the CLI.

USAGE:
    rtm [COMMAND]

COMMAND:
    copy   [FILE]          Copy the desired template file inside the current folder.
    create [FILE] [FLAGS]  Create a file inside your default template folder.
    delete [FILE]...       Delete files inside your default template folder.
    edit   [FILE] [FLAGS]  Edit a template file with your
    list                   List your template files.
    folder                 Display the path to your default template folder.
    help                   Prints this message.

FLAGS:
    -ne               No-edit, suppress the choice to edit a newly created file (invalidates other flags)
    -pv               Prefer visual, this will try to use $VISUAL env var on MacOS and Linux, if it doesn't succeed
                      then it'll use open or xdg-open, fall-backing yet again to $EDITOR.
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

fn create_file(file: &str, no_edit: bool, prefer_visual: bool) {
    let mut file_path = file::unwrap_path();
    file_path.push(file);

    if let Err(e) = fs::File::create(&file_path) {
        println!("{e}");
    } else {
        println!("File created.");

        #[cfg(target_os = "windows")]
        file::open_editor(&file_path);

        #[cfg(not(target_os = "windows"))]
        if !no_edit && util::ask_user_to_open_editor(None) {
            if prefer_visual {
                file::open_editor(&file_path)
            } else {
                file::open_terminal_editor(&file_path);
            }
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

fn edit_file(file: &str, prefer_visual: bool) {
    let mut file_path = file::unwrap_path();
    file_path.push(file);

    if prefer_visual {
        file::open_editor(&file_path);
    } else {
        file::open_terminal_editor(&file_path);
    }
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
