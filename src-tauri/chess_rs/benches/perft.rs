use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};

use chess_rs::board;
use chess_rs::lookup_tables;
use chess_rs::perft;

pub fn perft4_bench(c: &mut Criterion) {
    let l = lookup_tables::LookupTables::generate();
    let bd = board::Board::start_pos(&l);
    c.bench_function("perft 4", |b| b.iter(|| perft::perft(&bd, black_box(4))));
}

pub fn perft7_bench(c: &mut Criterion) {
    let l = lookup_tables::LookupTables::generate();
    let bd = board::Board::start_pos(&l);
    let mut group = c.benchmark_group("perft 7");
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(10);
    group.bench_function("perft 7", |b| b.iter(|| perft::perft(&bd, black_box(7))));
    group.finish();
}

criterion_group!(benches, perft4_bench, perft7_bench);
criterion_main!(benches);
