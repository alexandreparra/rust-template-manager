use std::path::PathBuf;
use std::fs;
use structopt::StructOpt;
use dirs::template_dir;

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
    #[structopt(name = "list", about = "List your template files inside your template folder.")]
    List,
}

fn main () {
    match Manager::from_args() {
        Manager::Folder => template_folder(),
        Manager::Copy { file_name } => copy_file(&file_name),
        Manager::Create {file } => create_file(file),
        Manager::List => list_files(),
    }
}

// Displays the default folder if no arguments is given.
fn template_folder() {
    println!("Your default template folder is: {:?}", template_dir());    
}

// Copy a valid file from the template folder to your current directory.
fn copy_file(name: &PathBuf) {
    let mut default_path = un_path();
    default_path.push(name);

    if let Err(_) = fs::copy(default_path, name) {
        println!("Couldn't reach file.");
    } else {
        println!("File copied!");
    }
}

// Create a file inside your default template folder.
fn create_file(file: PathBuf) {
    let mut file_path = un_path();
    file_path.push(file);

    if let Err(_) = std::fs::File::create(file_path) {
        println!("Couldn't create file.")
    } else {
        println!("File created.")
    }
}

// List the files inside your template folder.
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

// Unwraps the default template folder from your system and return the PathBuf.
fn un_path() -> PathBuf {
    let default_dir = match template_dir() {
        Some(default_dir) => default_dir,
        None => panic!("Could not open default template folder."),
    };

    default_dir
}