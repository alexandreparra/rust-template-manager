use std::{env, io};
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use structopt::StructOpt;
use dirs::home_dir;

#[derive(StructOpt, Debug)]
#[structopt(name = "rtm", about = "Easily manage your template files through the CLI.")]
enum Manager {
    #[structopt(name = "folder", about = "Define the default folder for your template files.")]
    Folder,
    #[structopt(name = "copy", about = "Copy the desire template file inside the current folder.")]
    Copy {
        file_name: PathBuf,
    },
    #[structopt(name = "create", about = "Create a file inside your default template folder.")]
    Create {
        file: PathBuf,
    },
    #[structopt(name = "delete", about = "Delete file inside the default template folder.")]
    Delete {
        file_name: PathBuf,
    },
    #[structopt(name = "list", about = "List your template files inside your template folder.")]
    List,
}

fn main() {
    match Manager::from_args() {
        Manager::Folder => template_folder(),
        Manager::Copy { file_name } => copy_file(&file_name),
        Manager::Create { file } => create_file(file),
        Manager::Delete { file_name } => delete_file(&file_name),
        Manager::List => list_files(),
    }
}

/// Displays your system's default folder.
fn template_folder() {
    println!("Your default template folder is: {:?}", un_path());
}

/// Copy a valid file from the template folder to your current directory.
fn copy_file(name: &PathBuf) {
    let mut default_path = un_path();
    default_path.push(name);

    if let Err(e) = fs::copy(default_path, name) {
        println!("{}", e);
    } else {
        println!("File copied!");
    }
}

/// Create a file inside your default template folder.
fn create_file(file: PathBuf) {
    let mut file_path = un_path();
    file_path.push(file);

    if let Err(e) = fs::File::create(&file_path) {
        println!("{}", e)
    } else {
        println!("File created.")
    }

    ask_user_to_open_editor(&file_path);
}

/// Delete a file from your default template folder.
fn delete_file(file: &PathBuf) {
    let mut file_path = un_path();
    file_path.push(file);

    if let Err(e) = fs::remove_file(file_path) {
        println!("{}", e);
    } else {
        println!("File deleted");
    }
}

/// List the files inside your template folder.
fn list_files() {
    let default_dir = un_path();

    println!("Listing files inside: {:#?}", default_dir);

    if let Ok(entries) = fs::read_dir(default_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{:?}", entry.file_name());
            }
        }
    }
}

fn ask_user_to_open_editor(file: &PathBuf) {
    println!("Do you want to edit this file? y/N");

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Couldn't read line");

        match line.trim().to_lowercase().as_str() {
            "yes" | "ye" | "y" => open_editor(file),
            "no" | "n" => return,
            _ => continue
        }
    }
}

fn open_editor(file_path: &PathBuf) {
    let editor = "EDITOR";

    match env::var(editor) {
        Ok(editor) => {
            Command::new(editor)
                .arg(file_path)
                .status()
                .expect("Failed to edit your file.");
        }
        Err(_) => println!("Couldn't find your default editor, please set the environment variable $EDITOR")
    }
}

/// Unwraps the default template folder to be used inside other functions.
fn un_path() -> PathBuf {
    let default_dir = home_dir().expect("Couldn't find the default template folder");
    default_dir.join("Templates")
}