use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table},
    Frame,
};

use crate::data_fetching::ReservationParsed;

use super::popup;

pub fn show(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    ticket_list_state: &mut ListState,
    reservations: &Vec<ReservationParsed>,
    chunks: &Vec<Rect>,
    show_popup: bool,
    popup_title: &str,
    popup_content: &Vec<Spans>,
) {
    if !show_popup {
        let ticket_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(chunks[1]);
        let detail_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(ticket_chunks[1]);
        let (left, rightu, rightb) = render_show_tickets(ticket_list_state, reservations);
        f.render_stateful_widget(left, ticket_chunks[0], ticket_list_state);
        f.render_widget(rightu, detail_chunks[0]);
        f.render_widget(rightb, detail_chunks[1]);
    }

    if show_popup {
        popup::show(f, popup_title, popup_content);
    }
}

fn render_show_tickets<'a>(
    ticket_list_state: &ListState,
    ticket_list: &Vec<ReservationParsed<'a>>,
) -> (List<'a>, Table<'a>, Paragraph<'a>) {
    let tickets = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Rezervacije")
        .border_type(BorderType::Rounded);

    let items: Vec<_> = ticket_list
        .iter()
        .map(|ticket| {
            ListItem::new(Spans::from(vec![Span::styled(
                ticket.reservation_id.to_string(),
                Style::default(),
            )]))
        })
        .collect();
    let selected_ticket = ticket_list
        .get(
            ticket_list_state
                .selected()
                .expect("there is always a selected ticket"),
        )
        .unwrap()
        .clone();

    let list = List::new(items).block(tickets).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let ticket_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_ticket.reservation_id.to_string())),
        Cell::from(Span::raw(selected_ticket.game.home_team.name.as_str())),
        Cell::from(Span::raw(selected_ticket.game.guest_team.name.as_str())),
        Cell::from(Span::raw(selected_ticket.game.date.as_str())),
        Cell::from(Span::raw(selected_ticket.basic_tickets.to_string())),
        Cell::from(Span::raw(selected_ticket.vip_tickets.to_string())),
    ])])
    .header(
        Row::new(vec![
            Cell::from(Span::styled(
                "ID",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Domaći tim",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Gostujući tim",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Datum",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Broj običnih karata",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Broj VIP karata",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ])
        .style(Style::default().fg(Color::Yellow)),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detalji")
            .border_type(BorderType::Rounded),
    )
    .widths(&[
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ]);

    let soccer_ascii = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Navigacija kroz listu sa kartama:")]),
        Spans::from(vec![Span::raw(
            "1. Za pregled gornjih karata pritisnuti strelicu na gore",
        )]),
        Spans::from(vec![Span::raw(
            "2. Za pregled donjih karata pritisnuti strelicu na dole",
        )]),
        Spans::from(vec![Span::raw(
            "3. Za brisanje trenutno izabrane karte pritisnuti Ctrl+d",
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(
            "  -   \\O                                     , ___",
        )]),
        Spans::from(vec![Span::raw(
            "  -     /\\                                   O/ |   |\\",
        )]),
        Spans::from(vec![Span::raw(
            " -   __/\\ `                                  /\\ |   |X\\",
        )]),
        Spans::from(vec![Span::raw(
            "     `    \\, ()                              ` <<|   |XX\\",
        )]),
        Spans::from(vec![Span::styled(
            "^^^^^^^^^^^^^^^`^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^",
            Style::default().fg(Color::Green),
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

    (list, ticket_detail, soccer_ascii)
}
