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

use crate::data_fetching::GameParsed;

use super::popup;

pub fn show<'a>(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    chunks: &Vec<Rect>,
    games: &Vec<GameParsed<'a>>,
    order_textarea: &mut [TextArea],
    show_popup: bool,
    popup_title: &str,
    popup_content: &Vec<Spans>,
) {
    let ticket_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);
    let order_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
            ]
            .as_ref(),
        );

    f.render_widget(render_match_ids(games), ticket_chunks[0]);
    let order_chunks = order_layout.split(ticket_chunks[1]);
    f.render_widget(render_order_ticket_help(), order_chunks[3]);
    for (textarea, chunk) in order_textarea.iter().zip(order_chunks) {
        let widget = textarea.widget();
        f.render_widget(widget, chunk);
    }

    if show_popup {
        popup::show(f, popup_title, popup_content);
    }
}
fn render_match_ids<'a>(games: &Vec<GameParsed<'a>>) -> Paragraph<'a> {
    let mut games_info: Vec<Spans> = vec![];

    for game in games {
        games_info.push(Spans::from(vec![Span::raw(format!(
            "({}) {} - {} {}",
            game.game_id,
            game.home_team.name.to_uppercase().get(0..3).unwrap(),
            game.guest_team.name.to_uppercase().get(0..3).unwrap(),
            game.date.clone()
        ))]))
    }
    let match_ids = Paragraph::new(games_info)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("ID Utakmica")
                .border_type(BorderType::Rounded),
        );

    match_ids
}

fn render_order_ticket_help<'a>() -> Paragraph<'a> {
    let tickets_ascii = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "                @-_________________-@                  ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "          @-_____|   Katar 2022    |_____-@            ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "           |      Svetsko prvenstvo      |             ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "    _______|_____________________________|__________   ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "   |________________________________________________|  ",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "   |               -                -               |  ",
            Style::default().fg(Color::LightRed),
        )]),
        Spans::from(vec![Span::styled(
            "   |  Pritisnite Ctrl+X ili Enter za promenu polja  |  ",
            Style::default().fg(Color::LightRed),
        )]),
        Spans::from(vec![Span::styled(
            "   |     Pritisnite Ctrl+S za rezervaciju karte     |  ",
            Style::default().fg(Color::LightRed),
        )]),
        Spans::from(vec![Span::styled(
            "   |        ____    ______________-   ____          |  ",
            Style::default().fg(Color::LightRed),
        )]),
        Spans::from(vec![Span::styled(
            "   | -  -  |    |   |            |   |    | -       |  ",
            Style::default().fg(Color::LightRed),
        )]),
        Spans::from(vec![Span::styled(
            "   |       |    |   |   Karte    |   |    |         |  ",
            Style::default().fg(Color::LightRed),
        )]),
        Spans::from(vec![Span::styled(
            "   |  --   |____| - |_o___oo___o_| - |____|     -   |  ",
            Style::default().fg(Color::LightRed),
        )]),
        Spans::from(vec![Span::styled(
            "   | -     |    |  |     --       |  |    |         |  ",
            Style::default().fg(Color::LightRed),
        )]),
        Spans::from(vec![Span::styled(
            "   |    -  |    |- | -      -     |  |    | --      |  ",
            Style::default().fg(Color::LightRed),
        )]),
        Spans::from(vec![Span::styled(
            "  o  \\ o /    |_______|====|__|______________|__|====|_________|   \\ o /  o ",
            Style::default().fg(Color::White),
        )]),
        Spans::from(vec![Span::styled(
            " /|\\   |      /                                                 \\    |   /|\\",
            Style::default().fg(Color::White),
        )]),
        Spans::from(vec![Span::styled(
            " / \\  / \\    /___________________________________________________\\  / \\  / \\",
            Style::default().fg(Color::White),
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

    tickets_ascii
}
