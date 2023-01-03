use regex::Regex;

use crate::data_fetching::{self, check_username, GameParsed};

pub fn validate_length(
    field: &str,
    field_name: &str,
    min_length: usize,
    max_length: usize,
) -> Option<String> {
    return if field.len() < min_length {
        Some(String::from(format!(
            "Dužina polja {} je manja od {}!",
            field_name, min_length
        )))
    } else if field.len() > max_length {
        Some(String::from(format!(
            "Dužina polja {} je veća od {}!",
            field_name, max_length
        )))
    } else {
        None
    };
}

pub fn validate_email(field: &str) -> Option<String> {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if !email_regex.is_match(field) {
        return Some(String::from(format!(
            "Email adresa nije u ispravnom formatu!"
        )));
    }

    None
}

pub fn validate_username(field: &str) -> Option<String> {
    if check_username(field) {
        return Some(String::from("Korisničko ime već postoji!"));
    }

    None
}

pub fn validate_jmbg(field: &str) -> Option<String> {
    if field.len() != 13 {
        return Some(String::from("JMBG mora da sadrži 13 karaktera"));
    }

    None
}

pub fn validate_basic_tickets(
    tickets_field: &str,
    id_field: &str,
    games: &Vec<GameParsed>,
    username: &String,
    password: &String,
) -> Option<String> {
    let reservations = data_fetching::get_reservations(games, username, password).unwrap();
    let mut current_num = 0;
    let mut max_amt = 0;
    for res in reservations {
        if res.game.game_id.to_string() == id_field {
            current_num += res.basic_tickets;
        }
    }

    for game in games {
        if game.game_id == id_field.parse::<i32>().unwrap_or_else(|_| -1) {
            max_amt = game.basic_tickets
        }
    }

    if current_num + tickets_field.parse::<i32>().unwrap_or_else(|_| 100) > 4 {
        return Some(String::from(
            "Broj rezervisanih običnih karata za izabranu utakmicu prevazilazi 4 dozvoljene karte!",
        ));
    }
    if max_amt - tickets_field.parse::<i32>().unwrap_or_else(|_| 100) < 0 {
        return Some(String::from(
            "Nema dovoljno običnih karata za izabranu utakmicu!",
        ));
    }

    None
}

pub fn validate_vip_tickets(
    tickets_field: &str,
    id_field: &str,
    games: &Vec<GameParsed>,
    username: &String,
    password: &String,
) -> Option<String> {
    let reservations = data_fetching::get_reservations(games, username, password).unwrap();
    let mut current_num = 0;
    let mut max_amt = 0;
    for res in reservations {
        if res.game.game_id.to_string() == id_field {
            current_num += res.vip_tickets;
        }
    }

    for game in games {
        if game.game_id == id_field.parse::<i32>().unwrap_or_else(|_| -1) {
            max_amt = game.basic_tickets
        }
    }

    if current_num + tickets_field.parse::<i32>().unwrap_or_else(|_| 100) > 1 {
        return Some(String::from(
            "Broj rezervisanih VIP karata za izabranu utakmicu prevazilazi 1 dozvoljenu kartu!",
        ));
    }
    if max_amt - tickets_field.parse::<i32>().unwrap_or_else(|_| 100) < 0 {
        return Some(String::from(
            "Nema dovoljno VIP karata za izabranu utakmicu!",
        ));
    }

    None
}
