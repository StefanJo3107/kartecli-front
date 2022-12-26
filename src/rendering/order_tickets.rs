use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render_match_ids<'a>() -> Paragraph<'a> {
    let match_ids = Paragraph::new(vec![
        Spans::from(vec![Span::raw("SRB - BRA (1)")]),
        Spans::from(vec![Span::raw("SRB - CAM (2)")]),
        Spans::from(vec![Span::raw("SRB - SUI (3)")]),
    ])
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

pub fn render_order_ticket_help<'a>() -> Paragraph<'a> {
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
