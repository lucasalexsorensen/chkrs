use bitvec::prelude::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chkrs::board::{BitSet, filter_approach, trick_approach};

const LEGAL_TILES_WITH_LEADING_ZEROS: u64 = 0b0000000000000000000000000000011111111011111111011111111011111111;
const LEGAL_TILES_WITH_TRAILING_ZEROS: u64 = LEGAL_TILES_WITH_LEADING_ZEROS.reverse_bits();


fn bench_iterators(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterators");
    group.sample_size(100).significance_level(0.05);

    let trailing_zeros_bitset = BitSet(LEGAL_TILES_WITH_TRAILING_ZEROS);
    let leading_zeros_bitset = BitSet(LEGAL_TILES_WITH_LEADING_ZEROS);
    
    group.bench_function("filter approach (trailing zeros)", |b| b.iter(|| {
        filter_approach(&trailing_zeros_bitset)
    }));
    group.bench_function("filter approach (leading zeros)", |b| b.iter(|| {
        filter_approach(&leading_zeros_bitset)
    }));

    group.bench_function("trick approach (trailing zeros)", |b| b.iter(|| {
        trick_approach(&trailing_zeros_bitset)
    }));
    group.bench_function("trick approach (leading zeros)", |b| b.iter(|| {
        trick_approach(&leading_zeros_bitset)
    }));

    let trailing_zeros_bitvec = LEGAL_TILES_WITH_TRAILING_ZEROS.view_bits::<Lsb0>();
    let leading_zeros_bitvec = LEGAL_TILES_WITH_LEADING_ZEROS.view_bits::<Lsb0>();
    group.bench_function("bitvec (trailing zeros)", |b| b.iter(|| {
        trailing_zeros_bitvec.iter_ones().count()
    }));
    group.bench_function("bitvec (leading zeros)", |b| b.iter(|| {
        leading_zeros_bitvec.iter_ones().count()
    }));



    // group.bench_function("with trick (trailing zeros)", |b| b.iter(|| {
    //     BitSet(LEGAL_TILES_WITH_TRAILING_ZEROS).into_iter().count()
    // }));

    // group.bench_function("with trick (leading zeros)", |b| b.iter(|| {
    //     BitSet(LEGAL_TILES_WITH_LEADING_ZEROS).into_iter().count()
    // }));

    // group.bench_function("without trick (trailing zeros)", |b| b.iter(|| {
    //     BitSet2(LEGAL_TILES_WITH_TRAILING_ZEROS).into_iter().count()
    // }));

    // group.bench_function("without trick (leading zeros)", |b| b.iter(|| {
    //     BitSet2(LEGAL_TILES_WITH_LEADING_ZEROS).into_iter().count()
    // }));


}

criterion_group!(benches, bench_iterators);
criterion_main!(benches);