use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::data_fetching::User;

pub fn show(
    chunks: &Vec<Rect>,
    f: &mut Frame<CrosstermBackend<Stdout>>,
    signed_in: bool,
    user: &User,
) {
    f.render_widget(render_home(signed_in, user), chunks[1]);
}
fn render_home<'a>(signed_in: bool, user: &User) -> Paragraph<'a> {
    if signed_in {
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw(format!("Dobrodošli {} u", user.username))]),
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
            Spans::from(vec![Span::raw("'Ctrl+1' za prikaz početne strane")]),
            Spans::from(vec![Span::raw("'Ctrl+2' za prikaz rezervisanih karata")]),
            Spans::from(vec![Span::raw("'Ctrl+3' za rezervisanje nove karte")]),
            Spans::from(vec![Span::raw("'Ctrl+4' za odjavljivanje")]),
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
    } else {
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
            Spans::from(vec![Span::raw("'Ctrl+1' za prikaz početne strane")]),
            Spans::from(vec![Span::raw("'Ctrl+2' za prijavljivanje")]),
            Spans::from(vec![Span::raw("'Ctrl+3' za registraciju")]),
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
}
