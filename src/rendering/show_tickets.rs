use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table},
};

use crate::data_fetching::read_db;

pub fn render_show_tickets<'a>(
    ticket_list_state: &ListState,
) -> (List<'a>, Table<'a>, Paragraph<'a>) {
    let tickets = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Karte")
        .border_type(BorderType::Rounded);

    let ticket_list = read_db().expect("cannot fetch ticket list");
    let items: Vec<_> = ticket_list
        .iter()
        .map(|ticket| {
            ListItem::new(Spans::from(vec![Span::styled(
                ticket.name.clone(),
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
        .expect("exists")
        .clone();

    let list = List::new(items).block(tickets).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let ticket_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_ticket.id.to_string())),
        Cell::from(Span::raw(selected_ticket.name)),
        Cell::from(Span::raw(selected_ticket.category)),
        Cell::from(Span::raw(selected_ticket.age.to_string())),
    ])])
    .header(
        Row::new(vec![
            Cell::from(Span::styled(
                "ID",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Name",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Category",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Age",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Created At",
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
        Constraint::Percentage(20),
        Constraint::Percentage(20),
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
            "3. Za brisanje trenutno izabrane karte pritisnuti taster d",
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
