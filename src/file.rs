use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io;
use dirs::{template_dir, config_dir};

#[cfg(not(target_os = "windows"))]
use crate::env_var;

pub fn open_editor(file_path: &PathBuf) {
    let result = open_file(file_path);
    if result.is_err() {
        println!("Failed to open file on default text editor");
    }
}


// Windows
#[inline]
#[cfg(target_os = "windows")]
fn open_file(file_path: &PathBuf) -> io::Result<()> {
    Command::new("cmd")
        .args(["/c", "start"])
        .arg(file_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

// MacOS
#[inline]
#[cfg(target_os = "macos")]
fn open_file(file_path: &PathBuf) -> io::Result<()> {
    Command::new("open")
        .arg(file_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

// Linux
#[inline]
#[cfg(target_os = "linux")]
fn open_file(editor: String, file_path: &PathBuf) -> io::Result<()> {
    Command::new("xdg-open")
        .arg(file_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

// Linux/MacOS

/// Try to open an editor inside the terminal, if it fails, fallback to the visual ones.
#[cfg(not(target_os = "windows"))]
pub fn open_terminal_editor(file_path: &PathBuf) {
    let editor = env_var::get_editor_env_var();

    if let Some(editor) = editor.editor {
        let result = open_file_env(editor, file_path);
        if result.is_ok() {
            return;
        }
    }

    println!("Error opening terminal based editor set on $EDITOR env var, fallbacking to visual ones.");

    if let Some(visual) = editor.visual {
        let result = open_file_env(visual, file_path);
        if result.is_ok() {
            return;
        }
    }

    println!("Failed to open $VISUAL editor, fallbacking to system default.");

    let result = open_file(file_path);
    if result.is_ok() {
        return;
    }

    println!("Couldn't open any of your default editors.");
}

#[inline]
#[cfg(not(target_os = "windows"))]
fn open_file_env(editor: String, file_path: &PathBuf) -> io::Result<()> {
    Command::new(editor)
        .arg(file_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

/// Unwraps the default template folder if it exists, otherwise use the fallback format.
pub fn unwrap_path() -> PathBuf {
    match template_dir() {
        None => {
            let config_dir = config_dir().expect("Couldn't find the system default config directory, aborting.").join("rtm");
            fs::create_dir_all(&config_dir).expect("Couldn't create rtm config folder, aborting.");
            
            config_dir
        },
        Some(dir) => dir
    }
}
