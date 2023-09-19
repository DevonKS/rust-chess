use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_chess::board;
use rust_chess::lookup_tables;
use rust_chess::perft;

pub fn criterion_benchmark(c: &mut Criterion) {
    let l = lookup_tables::LookupTables::new();
    let bd = board::Board::start_pos(&l);
    c.bench_function("perft 4", |b| b.iter(|| perft::perft(&bd, black_box(4))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
