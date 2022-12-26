use std::io;

use crossterm::terminal::enable_raw_mode;
use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, ListState},
    Terminal,
};
use tui_textarea::{Input, Key, TextArea};

use crate::data_fetching::get_user_data;

pub mod data_fetching;
pub mod rendering;

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    ShowTickets,
    OrderTicket,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::ShowTickets => 1,
            MenuItem::OrderTicket => 2,
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
            .title(format!("{} {}", title, "(^X to switch)")),
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

    let mut order_textarea = [TextArea::default(), TextArea::default()];
    let order_textarea_titles = ["Tip karte (Obična ili VIP)", "ID Utakmice"];
    let mut order_ta_index = 0;
    activate_area(&mut order_textarea[0], order_textarea_titles[0]);
    deactivate_area(&mut order_textarea[1], order_textarea_titles[1]);

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            rendering::ui(
                f,
                active_menu_item.clone(),
                &mut ticket_list_state,
                &mut order_textarea,
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
                ..
            } => active_menu_item = MenuItem::Home,
            Input {
                key: Key::Char('2'),
                ..
            } => active_menu_item = MenuItem::ShowTickets,
            Input {
                key: Key::Char('3'),
                ..
            } => active_menu_item = MenuItem::OrderTicket,
            Input { key: Key::Down, .. } => {
                if let Some(selected) = ticket_list_state.selected() {
                    let amount_tickets =
                        data_fetching::read_db().expect("can fetch pet list").len();
                    if selected >= amount_tickets - 1 {
                        ticket_list_state.select(Some(0));
                    } else {
                        ticket_list_state.select(Some(selected + 1));
                    }
                }
            }
            Input { key: Key::Up, .. } => {
                if let Some(selected) = ticket_list_state.selected() {
                    let amount_tickets =
                        data_fetching::read_db().expect("can fetch pet list").len();
                    if selected > 0 {
                        ticket_list_state.select(Some(selected - 1));
                    } else {
                        ticket_list_state.select(Some(amount_tickets - 1));
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
            } => {
                deactivate_area(
                    &mut order_textarea[order_ta_index],
                    order_textarea_titles[order_ta_index],
                );
                order_ta_index = (order_ta_index + 1) % 2;
                activate_area(
                    &mut order_textarea[order_ta_index],
                    order_textarea_titles[order_ta_index],
                );
            }
            input => {
                order_textarea[order_ta_index].input(input);
            }
        }
    }
    Ok(())
}
