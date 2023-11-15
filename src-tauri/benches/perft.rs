use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_chess::board;
use rust_chess::lookup_tables;
use rust_chess::perft;

pub fn perft4_bench(c: &mut Criterion) {
    let l = lookup_tables::LookupTables::generate();
    let bd = board::Board::start_pos(&l);
    c.bench_function("perft 4", |b| b.iter(|| perft::perft(&bd, black_box(4))));
}

pub fn perft7_bench(c: &mut Criterion) {
    // FIXME: I can't get this to run fast enough. Need to tweek warmup and measurement time.
    // let l = lookup_tables::LookupTables::generate();
    // let bd = board::Board::start_pos(&l);
    // let mut group = c.benchmark_group("sample-size-example");
    // group.sample_size(10);
    // group.bench_function("perft 7", |b| b.iter(|| perft::perft(&bd, black_box(7))));
    // group.finish();
}

criterion_group!(benches, perft4_bench, perft7_bench);
criterion_main!(benches);
