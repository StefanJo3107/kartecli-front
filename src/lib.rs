use std::io;

use crossterm::terminal::enable_raw_mode;
use data_fetching::{NewReservation, NewUser, ReservationParsed, User};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListState},
    Terminal,
};
use tui_textarea::{Input, Key, TextArea};

pub mod data_fetching;
pub mod data_validation;
pub mod rendering;

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    ShowTickets,
    OrderTicket,
    Login,
    Register,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::ShowTickets => 1,
            MenuItem::OrderTicket => 2,
            MenuItem::Login => 1,
            MenuItem::Register => 2,
        }
    }
}

fn deactivate_area(textarea: &mut TextArea<'_>, title: &str) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    let b = textarea.block().cloned().unwrap_or_else(|| {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
    });
    textarea.set_block(
        b.style(Style::default().fg(Color::DarkGray))
            .title(format!("{} {}", title, "(^X ili Enter za promenu)")),
    );
}

fn activate_area(textarea: &mut TextArea<'_>, title: &str) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    let b = textarea.block().cloned().unwrap_or_else(|| {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
    });
    textarea.set_block(b.style(Style::default()).title(format!("{}", title)));
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut ticket_list_state = ListState::default();
    ticket_list_state.select(Some(0));
    let mut active_menu_item = MenuItem::Home;
    enable_raw_mode()?;

    let mut order_textarea = [
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
    ];
    let order_textarea_titles = ["ID Utakmice", "Broj običnih karata", "Broj VIP karata"];
    let mut order_ta_index = 0;
    activate_area(&mut order_textarea[0], order_textarea_titles[0]);
    deactivate_area(&mut order_textarea[1], order_textarea_titles[1]);
    deactivate_area(&mut order_textarea[2], order_textarea_titles[2]);

    let mut login_textarea = [TextArea::default(), TextArea::default()];
    let login_textarea_titles = ["Korisničko ime", "Lozinka"];
    let mut login_ta_index = 0;
    activate_area(&mut login_textarea[0], login_textarea_titles[0]);
    deactivate_area(&mut login_textarea[1], login_textarea_titles[1]);

    let mut register_textarea = [
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
    ];
    let register_textarea_titles = [
        "Ime",
        "Prezime",
        "JMBG",
        "Email",
        "Korisničko ime",
        "Lozinka",
    ];
    let mut register_ta_index = 0;
    activate_area(&mut register_textarea[0], register_textarea_titles[0]);
    for i in 1..6 {
        deactivate_area(&mut register_textarea[i], register_textarea_titles[i]);
    }

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let national_teams = data_fetching::get_national_teams().unwrap();
    let games = data_fetching::get_games(&national_teams).unwrap();
    let mut user: User = User {
        user_id: -1,
        username: String::from(""),
        password: String::from(""),
        email: String::from(""),
        identity_number: String::from(""),
        name: String::from(""),
        surname: String::from(""),
    };
    let mut reservations: Vec<ReservationParsed> = vec![];

    let mut signed_in = false;
    let mut show_popups = [false, false, false, false];
    let mut login_spans: Vec<Spans> = vec![];
    let mut register_spans: Vec<Spans> = vec![];
    let mut show_spans: Vec<Spans> = vec![];
    let mut order_spans: Vec<Spans> = vec![];
    let mut titles: Vec<&str>;

    loop {
        if signed_in {
            titles = vec![
                "1. Početna",
                "2. Pregled rezervacija",
                "3. Rezervacija karata",
                "4. Odjavi se",
            ];
        } else {
            titles = vec!["1. Početna", "2. Prijavi se", "3. Registruj se"];
        }

        terminal.draw(|f| {
            rendering::ui(
                f,
                active_menu_item.clone(),
                &mut ticket_list_state,
                &mut order_textarea,
                &mut login_textarea,
                &mut register_textarea,
                &reservations,
                &games,
                &titles,
                &show_popups,
                &login_spans,
                &register_spans,
                &order_spans,
                &show_spans,
                signed_in,
                &user,
            );
        })?;

        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,

            Input {
                key: Key::Char('m'),
                ctrl: true,
                ..
            } => {}

            Input {
                key: Key::Char('1'),
                ctrl: true,
                ..
            } => active_menu_item = MenuItem::Home,

            Input {
                key: Key::Char('2'),
                ctrl: true,
                ..
            } => {
                if signed_in {
                    show_spans = vec![];
                    show_popups[2] = false;
                    active_menu_item = MenuItem::ShowTickets;
                    if reservations.len() == 0 {
                        show_spans.push(Spans::from(vec![Span::styled(
                            "Ne postoje rezervisane karte za prijavljenog korisnika!",
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[2] = true;
                        continue;
                    }
                } else {
                    active_menu_item = MenuItem::Login;
                }
            }
            Input {
                key: Key::Char('3'),
                ctrl: true,
                ..
            } => {
                if signed_in {
                    active_menu_item = MenuItem::OrderTicket;
                } else {
                    active_menu_item = MenuItem::Register;
                }
            }
            Input {
                key: Key::Char('4'),
                ctrl: true,
                ..
            } => {
                signed_in = false;
                active_menu_item = MenuItem::Home;
            }

            Input {
                key: Key::Char('c'),
                ctrl: true,
                ..
            } => match active_menu_item {
                MenuItem::Login => show_popups[0] = false,
                MenuItem::Register => show_popups[1] = false,
                MenuItem::OrderTicket => show_popups[3] = false,
                MenuItem::Home | MenuItem::ShowTickets => {}
            },

            Input { key: Key::Down, .. } => {
                if matches!(active_menu_item, MenuItem::ShowTickets) {
                    if let Some(selected) = ticket_list_state.selected() {
                        let amount_tickets = reservations.len();
                        if selected >= amount_tickets - 1 {
                            ticket_list_state.select(Some(0));
                        } else {
                            ticket_list_state.select(Some(selected + 1));
                        }
                    }
                }
            }
            Input { key: Key::Up, .. } => {
                if matches!(active_menu_item, MenuItem::ShowTickets) {
                    if let Some(selected) = ticket_list_state.selected() {
                        let amount_tickets = reservations.len();
                        if selected > 0 {
                            ticket_list_state.select(Some(selected - 1));
                        } else {
                            ticket_list_state.select(Some(amount_tickets - 1));
                        }
                    }
                }
            }

            Input {
                key: Key::Char('x'),
                ctrl: true,
                ..
            }
            | Input {
                key: Key::Enter, ..
            } => match active_menu_item {
                MenuItem::OrderTicket => {
                    deactivate_area(
                        &mut order_textarea[order_ta_index],
                        order_textarea_titles[order_ta_index],
                    );
                    order_ta_index = (order_ta_index + 1) % order_textarea.len();
                    activate_area(
                        &mut order_textarea[order_ta_index],
                        order_textarea_titles[order_ta_index],
                    );
                }
                MenuItem::Login => {
                    deactivate_area(
                        &mut login_textarea[login_ta_index],
                        login_textarea_titles[login_ta_index],
                    );
                    login_ta_index = (login_ta_index + 1) % login_textarea.len();
                    activate_area(
                        &mut login_textarea[login_ta_index],
                        login_textarea_titles[login_ta_index],
                    );
                }
                MenuItem::Register => {
                    deactivate_area(
                        &mut register_textarea[register_ta_index],
                        register_textarea_titles[register_ta_index],
                    );
                    register_ta_index = (register_ta_index + 1) % register_textarea.len();
                    activate_area(
                        &mut register_textarea[register_ta_index],
                        register_textarea_titles[register_ta_index],
                    );
                }
                MenuItem::Home | MenuItem::ShowTickets => {}
            },
            Input {
                key: Key::Char('d'),
                ctrl: true,
                ..
            } => match active_menu_item {
                MenuItem::ShowTickets => {
                    if data_fetching::delete_reservation(
                        reservations[ticket_list_state.selected().unwrap()].reservation_id,
                        &user.username,
                        &user.password,
                    ) {
                        reservations =
                            data_fetching::get_reservations(&games, &user.username, &user.password)
                                .unwrap();
                        ticket_list_state.select(Some(0));
                    }
                }
                MenuItem::Home | MenuItem::Login | MenuItem::OrderTicket | MenuItem::Register => {}
            },
            Input {
                key: Key::Char('s'),
                ctrl: true,
                ..
            } => match active_menu_item {
                MenuItem::Login => {
                    show_popups[0] = false;
                    login_spans = vec![];
                    if let Some(m) = data_validation::validate_length(
                        login_textarea[0].lines().join("\n").as_str(),
                        "Korisničko ime",
                        1,
                        100,
                    ) {
                        login_spans.push(Spans::from(vec![Span::styled(
                            m,
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[0] = true;
                    }

                    if let Some(m) = data_validation::validate_length(
                        login_textarea[1].lines().join("\n").as_str(),
                        "Lozinka",
                        1,
                        100,
                    ) {
                        login_spans.push(Spans::from(vec![Span::styled(
                            m,
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[0] = true;
                    }

                    if show_popups[0] {
                        continue;
                    }

                    match data_fetching::get_user_data(
                        login_textarea[0].lines().join("\n"),
                        login_textarea[1].lines().join("\n"),
                    ) {
                        Some(u) => user = u,
                        None => {
                            login_spans.push(Spans::from(vec![Span::styled(
                                "Korisnik sa unetim kredencijalima ne postoji!",
                                Style::default().fg(Color::LightRed),
                            )]));
                            show_popups[0] = true;
                            continue;
                        }
                    }
                    signed_in = true;
                    active_menu_item = MenuItem::Home;
                    reservations =
                        data_fetching::get_reservations(&games, &user.username, &user.password)
                            .unwrap();
                }
                MenuItem::Register => {
                    show_popups[1] = false;
                    register_spans = vec![];
                    if let Some(m) = data_validation::validate_length(
                        register_textarea[0].lines().join("\n").as_str(),
                        "Ime",
                        1,
                        100,
                    ) {
                        register_spans.push(Spans::from(vec![Span::styled(
                            m,
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[1] = true;
                    }

                    if let Some(m) = data_validation::validate_length(
                        register_textarea[1].lines().join("\n").as_str(),
                        "Prezime",
                        1,
                        100,
                    ) {
                        register_spans.push(Spans::from(vec![Span::styled(
                            m,
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[1] = true;
                    }

                    if let Some(m) = data_validation::validate_jmbg(
                        register_textarea[2].lines().join("\n").as_str(),
                    ) {
                        register_spans.push(Spans::from(vec![Span::styled(
                            m,
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[1] = true;
                    }

                    if let Some(m) = data_validation::validate_email(
                        register_textarea[3].lines().join("\n").as_str(),
                    ) {
                        register_spans.push(Spans::from(vec![Span::styled(
                            m,
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[1] = true;
                    }

                    if let Some(m) = data_validation::validate_username(
                        register_textarea[4].lines().join("\n").as_str(),
                    ) {
                        register_spans.push(Spans::from(vec![Span::styled(
                            m,
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[1] = true;
                    }

                    if let Some(m) = data_validation::validate_length(
                        register_textarea[5].lines().join("\n").as_str(),
                        "Lozinka",
                        6,
                        100,
                    ) {
                        register_spans.push(Spans::from(vec![Span::styled(
                            m,
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[1] = true;
                    }

                    if show_popups[1] {
                        continue;
                    }

                    match data_fetching::post_user(NewUser {
                        name: register_textarea[0].lines().join("\n"),
                        surname: register_textarea[1].lines().join("\n"),
                        identity_number: register_textarea[2].lines().join("\n"),
                        email: register_textarea[3].lines().join("\n"),
                        username: register_textarea[4].lines().join("\n"),
                        password: register_textarea[5].lines().join("\n"),
                    }) {
                        Some(u) => user = u,
                        None => {
                            register_spans.push(Spans::from(vec![Span::styled(
                                "Greška prilikom pravljenja naloga!",
                                Style::default().fg(Color::LightRed),
                            )]));
                            show_popups[1] = true;
                            continue;
                        }
                    };
                    signed_in = true;
                    active_menu_item = MenuItem::Home;

                    reservations =
                        data_fetching::get_reservations(&games, &user.username, &user.password)
                            .unwrap();
                }
                MenuItem::OrderTicket => {
                    show_popups[3] = false;
                    order_spans = vec![];
                    if let Some(m) = data_validation::validate_length(
                        order_textarea[0].lines().join("\n").as_str(),
                        "ID Utakmice",
                        1,
                        100,
                    ) {
                        order_spans.push(Spans::from(vec![Span::styled(
                            m,
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[3] = true;
                    }

                    if let Some(m) = data_validation::validate_basic_tickets(
                        order_textarea[1].lines().join("\n").as_str(),
                        order_textarea[0].lines().join("\n").as_str(),
                        &games,
                        &user.username,
                        &user.password,
                    ) {
                        order_spans.push(Spans::from(vec![Span::styled(
                            m,
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[3] = true;
                    }

                    if let Some(m) = data_validation::validate_vip_tickets(
                        order_textarea[2].lines().join("\n").as_str(),
                        order_textarea[0].lines().join("\n").as_str(),
                        &games,
                        &user.username,
                        &user.password,
                    ) {
                        order_spans.push(Spans::from(vec![Span::styled(
                            m,
                            Style::default().fg(Color::LightRed),
                        )]));
                        show_popups[3] = true;
                    }

                    if show_popups[3] {
                        continue;
                    }

                    match data_fetching::post_reservation(
                        NewReservation {
                            game_id: order_textarea[0].lines().join("\n").parse::<i32>().unwrap(),
                            user_id: user.user_id,
                            basic_tickets: order_textarea[1]
                                .lines()
                                .join("\n")
                                .parse::<i32>()
                                .unwrap(),
                            vip_tickets: order_textarea[2]
                                .lines()
                                .join("\n")
                                .parse::<i32>()
                                .unwrap(),
                        },
                        &user.username,
                        &user.password,
                    ) {
                        Some(_) => {}
                        None => {
                            order_spans.push(Spans::from(vec![Span::styled(
                                "Greška prilikom rezervacije karata!",
                                Style::default().fg(Color::LightRed),
                            )]));
                            show_popups[3] = true;
                            continue;
                        }
                    };
                    reservations =
                        data_fetching::get_reservations(&games, &user.username, &user.password)
                            .unwrap();
                }
                MenuItem::Home | MenuItem::ShowTickets => {}
            },
            input => match active_menu_item {
                MenuItem::OrderTicket => {
                    order_textarea[order_ta_index].input(input);
                }
                MenuItem::Register => {
                    register_textarea[register_ta_index].input(input);
                }
                MenuItem::Login => {
                    login_textarea[login_ta_index].input(input);
                }
                MenuItem::Home | MenuItem::ShowTickets => {}
            },
        }
    }
    Ok(())
}
