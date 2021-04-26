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
    #[structopt(name = "delete", about = "Delete file inside the default template folder.")]
    Delete {
        file: PathBuf,
    },
    #[structopt(name = "list", about = "List your template files inside your template folder.")]
    List,
}

fn main () {
    match Manager::from_args() {
        Manager::Folder => template_folder(),
        Manager::Copy { file_name } => copy_file(&file_name),
        Manager::Create { file } => create_file(file),
        Manager::Delete { file } => println!("TODO"),
        Manager::List => list_files(),
    }
}

// Displays your system's default folder.
fn template_folder() {
    println!("Your default template folder is: {:?}", un_path());    
}

// Copy a valid file from the template folder to your current directory.
fn copy_file(name: &PathBuf) {
    let mut default_path = un_path();
    default_path.push(name);

    if let Err(e) = fs::copy(default_path, name) {
        println!("{}", e);
    } else {
        println!("File copied!");
    }
}

// Create a file inside your default template folder.
fn create_file(file: PathBuf) {
    let mut file_path = un_path();
    file_path.push(file);

    if let Err(e) = std::fs::File::create(file_path) {
        println!("{}", e)
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

// Unwraps the default template folder to be used inside other functions.
fn un_path() -> PathBuf {
    let default_dir = match template_dir() {
        Some(default_dir) => default_dir,
        None => panic!("Could not open default template folder."),
    };
    default_dir
}