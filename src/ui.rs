use crate::app::App;
use crate::app::Focus;
use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Min(10),
        Constraint::Length(6),
        Constraint::Length(3),
    ]);

    let chunks = vertical.split(frame.area());

    let top = Layout::horizontal([
        Constraint::Length(30),
        Constraint::Length(30),
        Constraint::Min(10),
    ]);

    let top_chunks = top.split(chunks[0]);

    render_files(frame, app, top_chunks[0]);
    render_functions(frame, app, top_chunks[1]);
    render_viewer(frame, app, top_chunks[2]);
    render_diagnostics(frame, app, chunks[1]);
    render_footer(frame, chunks[2]);
}

fn render_files(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .files
        .iter()
        .map(|f| ListItem::new(f.as_str()))
        .collect();

    let list = List::new(items)
    .highlight_symbol("▶ ")
    .highlight_style(Style::default().reversed())
    .block(
        Block::bordered()
            .title("Functions")
            .border_style(
                if app.focus == Focus::Functions {
                    Style::default().bold()
                } else {
                    Style::default()
                },
            ),
    );

    let mut state = ListState::default();
    state.select(Some(app.selected_file));

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_functions(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .functions
        .iter()
        .map(|f| ListItem::new(f.as_str()))
        .collect();

    let list = List::new(items)
        .highlight_symbol("▶ ")
       .block(
    Block::bordered()
        .title("Functions")
        .border_style(
            if app.focus == Focus::Functions {
                Style::default().bold()
            } else {
                Style::default()
            },
        ),
);

    let mut state = ListState::default();

    state.select(Some(app.selected_function));

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_viewer(frame: &mut Frame, app: &App, area: Rect) {
    let p = Paragraph::new(app.content.as_str())
        .block(Block::bordered().title("Viewer"))
        .wrap(Wrap { trim: false });

    frame.render_widget(p, area);
}

fn render_diagnostics(frame: &mut Frame, app: &App, area: Rect) {
    let text = app.diagnostics.join("\n");

    frame.render_widget(
        Paragraph::new(text).block(Block::bordered().title("Diagnostics")),
        area,
    );
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new("1 Source | 2 HIR | 3 MIR | 4 LLVM | 5 ASM | q Quit")
        .block(Block::bordered());

    frame.render_widget(footer, area);
}
