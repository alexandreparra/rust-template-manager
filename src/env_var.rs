use std::env;

static EDITOR: &str = "EDITOR";
static VISUAL: &str = "VISUAL";

pub fn get_editor_env_var() -> String {
    if let Ok(visual) = env::var(VISUAL) {
        return visual;
    }

    return env::var(EDITOR).expect("Couldn't find any default editor to open the file with");
}
