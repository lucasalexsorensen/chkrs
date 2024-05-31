use bitvec::prelude::*;
use criterion::{criterion_group, criterion_main, Criterion};
use chkrs::board::fast::BitSet;

const LEGAL_TILES_WITH_LEADING_ZEROS: u64 = 0b0000000000000000000000000000011111111011111111011111111011111111;
const LEGAL_TILES_WITH_TRAILING_ZEROS: u64 = LEGAL_TILES_WITH_LEADING_ZEROS.reverse_bits();


fn bench_iterators(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterators");
    group.sample_size(100).significance_level(0.05);

    
    group.bench_function("own implementation (trailing zeros)", |b| b.iter(|| {
        LEGAL_TILES_WITH_TRAILING_ZEROS.iter_ones().count()
    }));
    group.bench_function("own implementation (leading zeros)", |b| b.iter(|| {
        LEGAL_TILES_WITH_LEADING_ZEROS.iter_ones().count()
    }));

    let trailing_zeros_bitvec = LEGAL_TILES_WITH_TRAILING_ZEROS.view_bits::<Lsb0>();
    let leading_zeros_bitvec = LEGAL_TILES_WITH_LEADING_ZEROS.view_bits::<Lsb0>();
    group.bench_function("bitvec (trailing zeros)", |b| b.iter(|| {
        trailing_zeros_bitvec.iter_ones().count()
    }));
    group.bench_function("bitvec (leading zeros)", |b| b.iter(|| {
        leading_zeros_bitvec.iter_ones().count()
    }));

}

criterion_group!(benches, bench_iterators);
criterion_main!(benches);