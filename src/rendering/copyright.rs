use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn show(chunks: &Vec<Rect>, f: &mut Frame<CrosstermBackend<Stdout>>) {
    let copyright = Paragraph::new("karte-CLI 2022 - Sva prava zadržana")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(BorderType::Rounded),
        );
    f.render_widget(copyright, chunks[2]);
}
