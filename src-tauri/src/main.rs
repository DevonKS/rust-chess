// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, sync::Mutex};

use uuid::Uuid;

use chess_rs::board;
use chess_rs::lookup_tables;

struct State<'a> {
    l: &'a lookup_tables::LookupTables,
    games: Mutex<HashMap<String, board::Board<'a>>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn new_game(state: tauri::State<State>) -> String {
    let id = Uuid::new_v4().to_string();
    let b = board::Board::start_pos(state.l);
    state.games.lock().unwrap().insert(id.clone(), b);

    id
}

fn main() {
    let l = lookup_tables::LookupTables::generate();
    let games = Mutex::new(HashMap::new());

    tauri::Builder::default()
        .manage(State { l: &l, games })
        .invoke_handler(tauri::generate_handler![greet, new_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
