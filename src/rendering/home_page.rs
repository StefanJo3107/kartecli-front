use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Dobrodošli u")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(" ___  __    ________  ________  _________  _______                  ________  ___       ___     ", Style::default().fg(Color::LightCyan))]),
            Spans::from(vec![Span::styled("|\\  \\|\\  \\ |\\   __  \\|\\   __  \\|\\___   ___\\\\  ___ \\                |\\   ____\\|\\  \\     |\\  \\    ", Style::default().fg(Color::LightCyan))]),
            Spans::from(vec![Span::styled("\\ \\  \\/  /|\\ \\  \\|\\  \\ \\  \\|\\  \\|___ \\  \\_\\ \\   __/|   ____________\\ \\  \\___|\\ \\  \\    \\ \\  \\   ",Style::default().fg(Color::LightCyan))]),
            Spans::from(vec![Span::styled(" \\ \\   ___  \\ \\   __  \\ \\   _  _\\   \\ \\  \\ \\ \\  \\_|/__|\\____________\\ \\  \\    \\ \\  \\    \\ \\  \\  ",Style::default().fg(Color::LightCyan))]),
            Spans::from(vec![Span::styled("  \\ \\  \\\\ \\  \\ \\  \\ \\  \\ \\  \\\\  \\|   \\ \\  \\ \\ \\  \\_|\\ \\|____________|\\ \\  \\____\\ \\  \\____\\ \\  \\ ",Style::default().fg(Color::LightCyan))]),
            Spans::from(vec![Span::styled("   \\ \\__\\\\ \\__\\ \\__\\ \\__\\ \\__\\\\ _\\    \\ \\__\\ \\ \\_______\\              \\ \\_______\\ \\_______\\ \\__\\", Style::default().fg(Color::LightCyan))]),
            Spans::from(vec![Span::styled("    \\|__| \\|__|\\|__|\\|__|\\|__|\\|__|    \\|__|  \\|_______|               \\|_______|\\|_______|\\|__|", Style::default().fg(Color::LightCyan))]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Pritisnite")]),
            Spans::from(vec![Span::raw("'1' za prikaz početne strane")]),
            Spans::from(vec![Span::raw("'2' za prikaz zakazanih karata")]),
            Spans::from(vec![Span::raw("'3' za zakazivanje nove karte")]),
            Spans::from(vec![Span::raw("'Escape' za izlazak iz programa.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Početna")
            .border_type(BorderType::Rounded),
    );
    home
}
