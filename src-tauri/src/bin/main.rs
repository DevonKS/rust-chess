// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rust_chess::board;
use rust_chess::core::IN_CHECK_FEN;
use rust_chess::lookup_tables;

// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }
//
// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

fn main() {
    let l = lookup_tables::LookupTables::generate();

    let b = board::Board::from_fen(IN_CHECK_FEN, &l).unwrap();
    b.print();
    println!("{:?}", b.generate_moves());

    let b2 = board::Board::from_fen(
        "rnbqkbnr/2p1p1pp/1p1N4/5p1Q/p3P3/8/PPPP1PPP/RNB1KB1R b KQkq - 0 6",
        &l,
    )
    .unwrap();
    b2.print();
    println!("{:?}", b2.generate_moves());

    let b3 = board::Board::from_fen(
        "rnbqkbnr/ppp1pppp/3p4/8/Q7/2P5/PP1PPPPP/RNB1KBNR b KQkq - 1 2",
        &l,
    )
    .unwrap();
    b3.print();
    println!("{:?}", b3.generate_moves());

    let b4 = board::Board::from_fen("4k3/8/8/8/7q/8/5R2/r2RK3 w - - 0 1", &l).unwrap();
    b4.print();
    println!("{:?}", b4.generate_moves());

    let b5 = board::Board::from_fen(
        "rnbqkb1r/pppppppp/8/8/6n1/P4N2/1PPPPPPP/RNBQKB1R w KQkq - 1 3",
        &l,
    )
    .unwrap();
    b5.print();
    println!("{:?}", b5.generate_moves());

    let b6 = board::Board::from_fen("4k3/8/8/8/7q/8/5P2/r2RK3 w - - 0 1", &l).unwrap();
    b6.print();
    println!("{:?}", b6.generate_moves());

    let b7 = board::Board::from_fen(
        "r1bqkbnr/1ppp1Qpp/p1n5/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4",
        &l,
    )
    .unwrap();
    b7.print();
    println!("{:?}", b7.generate_moves());
    println!("{:?}", b7.is_valid());
}
