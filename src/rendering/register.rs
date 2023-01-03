use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use super::popup;

pub fn show<'a>(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    chunks: &Vec<Rect>,
    register_textarea: &mut [TextArea],
    show_popup: bool,
    popup_title: &str,
    popup_content: &Vec<Spans>,
) {
    let register_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
            ]
            .as_ref(),
        );
    let register_chunks = register_layout.split(chunks[1]);
    f.render_widget(render_register_help(), register_chunks[6]);
    for (textarea, chunk) in register_textarea.iter().zip(register_chunks) {
        let widget = textarea.widget();
        f.render_widget(widget, chunk);
    }

    if show_popup {
        popup::show(f, popup_title, popup_content);
    }
}

fn render_register_help<'a>() -> Paragraph<'a> {
    let lock_ascii = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(
            "     /|                                                  ",
        )]),
        Spans::from(vec![Span::raw(
            "    / |                                                  ",
        )]),
        Spans::from(vec![Span::raw(
            "   /__|______                                            ",
        )]),
        Spans::from(vec![Span::raw(
            "  |  __  __  |                                           ",
        )]),
        Spans::from(vec![Span::raw(
            "  | |  ||  | |               Pritisnite:                 ",
        )]),
        Spans::from(vec![Span::raw(
            "  | |__||__| |  Ctrl+X ili Enter za promenu polja za unos",
        )]),
        Spans::from(vec![Span::raw(
            "  |  __  __()|         Ctrl+S za registraciju            ",
        )]),
        Spans::from(vec![Span::raw(
            "  | |  ||  | |                                           ",
        )]),
        Spans::from(vec![Span::raw(
            "  | |  ||  | |                                           ",
        )]),
        Spans::from(vec![Span::raw(
            "  | |__||__| |                                           ",
        )]),
        Spans::from(vec![Span::raw(
            "  |__________|                                           ",
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Pomoć")
            .border_type(BorderType::Rounded),
    );

    lock_ascii
}
