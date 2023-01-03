use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::Spans,
    widgets::ListState,
    Frame,
};
use tui_textarea::TextArea;

use crate::{
    data_fetching::{GameParsed, ReservationParsed, User},
    MenuItem,
};

pub mod copyright;
pub mod home_page;
pub mod login;
pub mod menu;
pub mod order_tickets;
pub mod popup;
pub mod register;
pub mod show_tickets;

pub fn ui(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    active_menu_item: MenuItem,
    ticket_list_state: &mut ListState,
    order_textarea: &mut [TextArea],
    login_textarea: &mut [TextArea],
    register_textarea: &mut [TextArea],
    reservations: &Vec<ReservationParsed>,
    games: &Vec<GameParsed>,
    titles: &Vec<&str>,
    popup_show: &[bool],
    login_spans: &Vec<Spans>,
    register_spans: &Vec<Spans>,
    order_spans: &Vec<Spans>,
    show_spans: &Vec<Spans>,
    signed_in: bool,
    user: &User,
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

    menu::show(active_menu_item, titles, &chunks, f);
    copyright::show(&chunks, f);

    match active_menu_item {
        MenuItem::Home => home_page::show(&chunks, f, signed_in, user),
        MenuItem::Login => login::show(
            f,
            &chunks,
            login_textarea,
            popup_show[0],
            "Neuspešno prijavljivanje (Ctrl+C za zatvaranje prozora)",
            login_spans,
        ),
        MenuItem::Register => register::show(
            f,
            &chunks,
            register_textarea,
            popup_show[1],
            "Neuspešna registracija (Ctrl+C za zatvaranje prozora)",
            register_spans,
        ),
        MenuItem::ShowTickets => show_tickets::show(
            f,
            ticket_list_state,
            reservations,
            &chunks,
            popup_show[2],
            "Nema zakazanih karata",
            show_spans,
        ),
        MenuItem::OrderTicket => order_tickets::show(
            f,
            &chunks,
            games,
            order_textarea,
            popup_show[3],
            "Neuspešna rezervacija (Ctrl+C za zatvaranje prozora)",
            order_spans,
        ),
    }
}
