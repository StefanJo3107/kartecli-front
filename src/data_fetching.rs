use std::io;

use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ticket {
    pub id: usize,
    pub name: String,
    pub category: String,
    pub age: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub surname: String,
    pub identity_number: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub name: String,
    pub surname: String,
    pub identity_number: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reservation {
    pub reservation_id: i32,
    pub user_id: i32,
    pub game_id: i32,
    pub basic_tickets: i32,
    pub vip_tickets: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewReservation {
    pub user_id: i32,
    pub game_id: i32,
    pub basic_tickets: i32,
    pub vip_tickets: i32,
}

#[derive(Debug)]
pub struct ReservationParsed<'a> {
    pub reservation_id: i32,
    pub user_id: i32,
    pub game: &'a GameParsed<'a>,
    pub basic_tickets: i32,
    pub vip_tickets: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    pub game_id: i32,
    pub home_team_id: i32,
    pub guest_team_id: i32,
    pub date: String,
    pub basic_tickets: i32,
    pub vip_tickets: i32,
}

#[derive(Debug)]
pub struct GameParsed<'a> {
    pub game_id: i32,
    pub home_team: &'a NationalTeam,
    pub guest_team: &'a NationalTeam,
    pub date: String,
    pub basic_tickets: i32,
    pub vip_tickets: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NationalTeam {
    pub national_team_id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CheckUser {
    pub status: bool,
}

pub fn get_user_data(username: String, password: String) -> Option<User> {
    let res = reqwest::blocking::get(format!(
        "http://localhost:1207/user/{}/{}",
        username, password
    ))
    .unwrap();
    match res.json::<User>() {
        Ok(mut parsed) => {
            parsed.password = password;
            return Some(parsed);
        }
        Err(_) => None,
    }
}

pub fn check_username(username: &str) -> bool {
    let res;
    match reqwest::blocking::get(format!("http://localhost:1207/check/{}", username)) {
        Ok(r) => res = r,
        Err(_) => return false,
    };
    match res.json::<CheckUser>() {
        Ok(parsed) => {
            if parsed.status {
                return true;
            }
            return false;
        }
        Err(_) => false,
    }
}

pub fn get_national_teams() -> Option<Vec<NationalTeam>> {
    let res = reqwest::blocking::get("http://localhost:1207/teams").unwrap();
    match res.json::<Vec<NationalTeam>>() {
        Ok(parsed) => Some(parsed),
        Err(_) => None,
    }
}

pub fn find_team_with_id(
    national_teams: &Vec<NationalTeam>,
    team_id: i32,
) -> Option<&NationalTeam> {
    for team in national_teams {
        if team_id == team.national_team_id {
            return Some(team);
        }
    }
    None
}

pub fn get_games(national_teams: &Vec<NationalTeam>) -> Option<Vec<GameParsed>> {
    let res = reqwest::blocking::get("http://localhost:1207/games").unwrap();
    let parsed = match res.json::<Vec<Game>>() {
        Ok(parsed) => Some(parsed),
        Err(_) => None,
    };

    let mut games_parsed: Vec<GameParsed> = vec![];
    for game in parsed.unwrap_or_else(|| vec![]) {
        let home_team = find_team_with_id(national_teams, game.home_team_id).unwrap();
        let guest_team = find_team_with_id(national_teams, game.guest_team_id).unwrap();
        games_parsed.push(GameParsed {
            game_id: game.game_id,
            home_team: home_team,
            guest_team: guest_team,
            basic_tickets: game.basic_tickets,
            vip_tickets: game.vip_tickets,
            date: game.date,
        });
    }

    Some(games_parsed)
}

pub fn find_game_with_id<'a>(
    games: &'a Vec<GameParsed>,
    game_id: i32,
) -> Option<&'a GameParsed<'a>> {
    for game in games {
        if game_id == game.game_id {
            return Some(game);
        }
    }
    None
}
pub fn get_reservations<'a>(
    games: &'a Vec<GameParsed<'a>>,
    username: &String,
    password: &String,
) -> Option<Vec<ReservationParsed<'a>>> {
    let res = reqwest::blocking::get(format!(
        "http://localhost:1207/reservations/{}/{}",
        username, password
    ))
    .unwrap();
    let parsed = match res.json::<Vec<Reservation>>() {
        Ok(parsed) => Some(parsed),
        Err(_) => None,
    };

    let mut reservations_parsed: Vec<ReservationParsed<'a>> = vec![];
    for reservation in parsed.unwrap_or_else(|| vec![]) {
        let game = find_game_with_id(games, reservation.game_id).unwrap();
        reservations_parsed.push(ReservationParsed {
            user_id: reservation.user_id,
            game: game,
            reservation_id: reservation.reservation_id,
            basic_tickets: reservation.basic_tickets,
            vip_tickets: reservation.vip_tickets,
        });
    }

    Some(reservations_parsed)
}

pub fn post_user(new_user: NewUser) -> Option<User> {
    let client = reqwest::blocking::Client::new();
    let json_data = json!(new_user);

    let res = client
        .post("http://localhost:1207/user")
        .json(&json_data)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .unwrap();
    match res.json::<User>() {
        Ok(mut parsed) => {
            parsed.password = new_user.password;
            return Some(parsed);
        }
        Err(_) => None,
    }
}

pub fn post_reservation(
    new_reservation: NewReservation,
    username: &String,
    password: &String,
) -> Option<Reservation> {
    let client = reqwest::blocking::Client::new();
    let json_data = json!(new_reservation);

    let res = client
        .post(format!(
            "http://localhost:1207/reservations/{}/{}",
            username, password
        ))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&json_data)
        .send()
        .unwrap();

    match res.json::<Reservation>() {
        Ok(parsed) => Some(parsed),
        Err(_) => None,
    }
}

pub fn delete_reservation(reservation_id: i32, username: &String, password: &String) -> bool {
    let client = reqwest::blocking::Client::new();
    match client
        .delete(format!(
            "http://localhost:1207/reservations/{}/{}/{}",
            username, password, reservation_id
        ))
        .send()
    {
        Ok(_) => true,
        Err(err) => panic!("{}", err),
    }
}
