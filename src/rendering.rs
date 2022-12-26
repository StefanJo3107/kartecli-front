use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Tabs},
    Frame,
};
use tui_textarea::TextArea;

use crate::MenuItem;

pub mod home_page;
pub mod login;
pub mod order_tickets;
pub mod register;
pub mod show_tickets;

pub fn ui(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    active_menu_item: MenuItem,
    ticket_list_state: &mut ListState,
    order_textarea: &mut [TextArea],
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let menu_titles = ["1. Početna", "2. Pregled karata", "3. Zakazivanje karata"]
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

    match active_menu_item {
        MenuItem::Home => f.render_widget(home_page::render_home(), chunks[1]),
        MenuItem::ShowTickets => {
            let ticket_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(chunks[1]);
            let detail_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(ticket_chunks[1]);
            let (left, rightu, rightb) = show_tickets::render_show_tickets(ticket_list_state);
            f.render_stateful_widget(left, ticket_chunks[0], ticket_list_state);
            f.render_widget(rightu, detail_chunks[0]);
            f.render_widget(rightb, detail_chunks[1]);
        }
        MenuItem::OrderTicket => {
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
                        Constraint::Min(3),
                    ]
                    .as_ref(),
                );

            f.render_widget(order_tickets::render_match_ids(), ticket_chunks[0]);
            let order_chunks = order_layout.split(ticket_chunks[1]);
            f.render_widget(order_tickets::render_order_ticket_help(), order_chunks[2]);
            for (textarea, chunk) in order_textarea.iter().zip(order_chunks) {
                let widget = textarea.widget();
                f.render_widget(widget, chunk);
            }
        }
    }
}
