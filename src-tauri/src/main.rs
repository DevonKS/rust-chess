// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, sync::Mutex};

use serde::Serialize;
use uuid::Uuid;

use chess_rs::board;
use chess_rs::core::{Move, Piece, PieceKind, Square};
use chess_rs::lookup_tables;

struct MyState<'a> {
    l: &'a lookup_tables::LookupTables,
    games: Mutex<HashMap<String, board::Board<'a>>>,
}

#[derive(Serialize)]
struct GameState {
    game_id: String,
    pieces: Vec<(Piece, Square)>,
    valid_moves: Vec<Move>,
    moves: Vec<String>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn new_game(state: tauri::State<MyState>) -> GameState {
    let id = Uuid::new_v4().to_string();
    let b = board::Board::start_pos(state.l);
    let pieces = b.pieces();
    let valid_moves = b.generate_moves(board::Legality::Legal);
    let moves = b.san_moves.clone();
    state.games.lock().unwrap().insert(id.clone(), b);

    GameState {
        game_id: id,
        pieces,
        valid_moves,
        moves,
    }
}

#[tauri::command]
fn make_move(state: tauri::State<MyState>, id: String, m: String) -> Result<GameState, String> {
    let m = if m.len() == 4 {
        let source = Square::try_from(&m[0..2]).unwrap();
        let dest = Square::try_from(&m[2..]).unwrap();
        Move(source, dest, None)
    } else if m.len() == 5 {
        let source = Square::try_from(&m[0..2]).unwrap();
        let dest = Square::try_from(&m[2..4]).unwrap();
        let promotion = Some(PieceKind::try_from(&m[4..]).expect("cannot parse move"));
        Move(source, dest, promotion)
    } else {
        panic!("invalid move");
    };

    let mut games = state.games.lock().unwrap();
    let b: &mut board::Board = games.get_mut(&id).ok_or("cannot find game")?;
    let valid_moves_before = b.generate_moves(board::Legality::Legal);
    b.apply_move(m, &valid_moves_before);

    let valid_moves = b.generate_moves(board::Legality::Legal);
    let moves = b.san_moves.clone();

    Ok(GameState {
        game_id: id,
        pieces: b.pieces(),
        valid_moves,
        moves,
    })
}

fn main() {
    let l = Box::new(lookup_tables::LookupTables::generate());
    let games = Mutex::new(HashMap::new());

    tauri::Builder::default()
        .manage(MyState {
            l: Box::leak(l),
            games,
        })
        .invoke_handler(tauri::generate_handler![greet, new_game, make_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
