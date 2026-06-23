use crate::compiler::*;

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Focus, View};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }

        KeyCode::Down => match app.focus {
            Focus::Files => {
                app.next_file();
            }

            Focus::Functions => {
                if app.selected_function + 1 < app.functions.len() {
                    app.selected_function += 1;
                }
            }
        },

        KeyCode::Up => match app.focus {
            Focus::Files => {
                app.previous_file();
            }

            Focus::Functions => {
                if app.selected_function > 0 {
                    app.selected_function -= 1;
                }
            }
        },

        KeyCode::Tab => {
            app.focus = match app.focus {
                Focus::Files => Focus::Functions,
                Focus::Functions => Focus::Files,
            };
        }

        KeyCode::F(1) => {
            app.current_view = View::Source;

            app.content = std::fs::read_to_string("src/main.rs").unwrap_or_default();
        }

        KeyCode::Char('2') => {
            app.current_view = View::Hir;

            if app.hir_cache.is_none() {
                match generate_hir() {
                    Ok(hir) => app.hir_cache = Some(hir),
                    Err(e) => {
                        app.content = e;
                        return;
                    }
                }
            }

            if let Some(name) = app.current_function() {
                if let Some(hir) = &app.hir_cache {
                    app.content = crate::compiler::hir_filter::extract_function_hir(hir, name);
                }
            }
        }
        KeyCode::Char('3') => {
            app.current_view = View::Mir;

            if app.mir_cache.is_none() {
                match generate_mir() {
                    Ok(mir) => app.mir_cache = Some(mir),
                    Err(e) => {
                        app.content = e;
                        return;
                    }
                }
            }

            if let Some(name) = app.current_function() {
                if let Some(mir) = &app.mir_cache {
                    app.content = extract_function_mir(mir, name);
                }
            }
        }

        KeyCode::Char('4') => {
            app.current_view = View::Llvm;

            if app.llvm_cache.is_none() {
                match generate_llvm() {
                    Ok(llvm) => app.llvm_cache = Some(llvm),
                    Err(e) => {
                        app.content = e;
                        return;
                    }
                }
            }

            app.content = app.llvm_cache.clone().unwrap();
        }

        KeyCode::Char('5') => {
            app.current_view = View::Asm;

            if let Some(name) = app.current_function() {
                match generate_asm_for_function(name) {
                    Ok(data) => app.content = data,
                    Err(e) => app.content = e,
                }
            }
        }

        KeyCode::Enter => {
            if app.focus == Focus::Files {
                app.open_selected_file();
            }

            if app.focus == Focus::Functions
                && let Some(name) = app.current_function()
                && let Some(path) = app.files.get(app.selected_file)
            {
                let source = std::fs::read_to_string(path).unwrap_or_default();

                if let Some(src) = crate::function_view::extract_function_source(&source, name) {
                    app.content = src;
                }
            }
        }

        KeyCode::PageDown => {
            app.viewer.scroll_down();
        }

        KeyCode::PageUp => {
            app.viewer.scroll_up();
        }

        KeyCode::Right => {
            app.viewer.scroll_right();
        }

        KeyCode::Left => {
            app.viewer.scroll_left();
        }

        _ => {}
    }
}
