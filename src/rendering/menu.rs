use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Tabs},
    Frame,
};

use crate::MenuItem;

pub fn show(
    active_menu_item: MenuItem,
    titles: &Vec<&str>,
    chunks: &Vec<Rect>,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) {
    let menu_titles = titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let tabs = Tabs::new(menu_titles)
        .select(active_menu_item.into())
        .block(
            Block::default()
                .title("Meni")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));
    f.render_widget(tabs, chunks[0]);
}
