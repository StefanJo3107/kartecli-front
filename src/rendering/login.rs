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
    login_textarea: &mut [TextArea],
    show_popup: bool,
    popup_title: &str,
    popup_content: &Vec<Spans>,
) {
    let login_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
            ]
            .as_ref(),
        );
    let login_chunks = login_layout.split(chunks[1]);
    f.render_widget(render_login_help(), login_chunks[2]);
    for (textarea, chunk) in login_textarea.iter().zip(login_chunks) {
        let widget = textarea.widget();
        f.render_widget(widget, chunk);
    }

    if show_popup {
        popup::show(f, popup_title, popup_content);
    }
}

fn render_login_help<'a>() -> Paragraph<'a> {
    let lock_ascii = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Pritisnite:")]),
        Spans::from(vec![Span::raw("Ctrl+X ili Enter za promenu polja za unos")]),
        Spans::from(vec![Span::raw("Ctrl+S za prijavljivanje na sistem")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "            .-\"\"-.                       ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "           / .--. \\                      ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "          / /    \\ \\                     ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "          | |    | |                     ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "          | |.-\"\"-.|                     ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "         ///`.::::.`\\                    ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "        ||| ::/  \\:: ;  8 8          ,o. ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "        ||; ::\\__/:: ; d8o8azzzzzzzzd   b",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "         \\\\\\ '::::' /                `o' ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "          `=':-..-'`                     ",
            Style::default().fg(Color::Yellow),
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
