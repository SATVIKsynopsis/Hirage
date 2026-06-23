use crate::project::discover_files;
use crate::state::viewer::ViewerState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    Source,
    Hir,
    Mir,
    Llvm,
    Asm,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    Files,
    Functions,
}

pub struct App {
    pub files: Vec<String>,
    pub selected_file: usize,
    pub current_view: View,
    pub content: String,
    pub diagnostics: Vec<String>,
    pub should_quit: bool,

    pub viewer: ViewerState,
    pub functions: Vec<String>,
    pub selected_function: usize,

    pub focus: Focus,

    pub hir_cache: Option<String>,
    pub mir_cache: Option<String>,
    pub llvm_cache: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            files: discover_files(),
            selected_file: 0,
            current_view: View::Source,
            content: String::new(),
            diagnostics: vec!["No diagnostics".to_string()],
            should_quit: false,

            viewer: ViewerState::new(),

            functions: vec![],
            selected_function: 0,
            focus: Focus::Files,

            hir_cache: None,
            mir_cache: None,
            llvm_cache: None,
        }
    }

    pub fn next_file(&mut self) {
        if self.selected_file + 1 < self.files.len() {
            self.selected_file += 1;
            self.open_selected_file();
        }
    }

    pub fn previous_file(&mut self) {
        if self.selected_file > 0 {
            self.selected_file -= 1;
            self.open_selected_file();
        }
    }

    pub fn open_selected_file(&mut self) {
        if let Some(path) = self.files.get(self.selected_file) {
            self.content = std::fs::read_to_string(path).unwrap_or_default();

            self.functions = crate::parser::extract_functions(&self.content);

            self.selected_function = 0;

        }
    }

    pub fn current_function(&self) -> Option<&String> {
        self.functions.get(self.selected_function)
    }
}
