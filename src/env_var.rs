use std::env;

static EDITOR: &str = "EDITOR";
static VISUAL: &str = "VISUAL";

pub struct Editors {
    pub editor: Option<String>,
    pub visual: Option<String>,
}

impl Editors {
    fn new() -> Editors {
        Editors {
            editor: None,
            visual: None,
        }
    }

    fn set_visual(&mut self, visual: String) {
        self.visual = Some(visual);
    }

    fn set_editor(&mut self, editor: String) {
        self.editor = Some(editor);
    }
}

pub fn get_editor_env_var() -> Editors {
    let mut editors = Editors::new();

    if let Ok(visual) = env::var(VISUAL) {
        editors.set_visual(visual);
    }

    if let Ok(editor) = env::var(EDITOR) {
        editors.set_editor(editor);
    }

    editors
}
