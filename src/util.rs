use std::io;
use std::io::Write;

pub fn ask_user_to_open_editor(message: Option<&str>) -> bool {
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
