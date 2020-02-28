#![warn(clippy::all)]
use criterion::{black_box, criterion_group, Criterion};
use zkp_criterion_utils::{log_size_bench, log_thread_bench};
use zkp_macros_decl::field_element;
use zkp_primefield::{
    fft,
    fft::{fft2_permuted, fft_cofactor_permuted},
    transpose::{transpose, transpose_base},
    FieldElement,
};
use zkp_u256::U256;

const SIZES: [usize; 7] = [64, 1024, 16384, 262144, 4194304, 16777216, 33554432];

fn bench_size(crit: &mut Criterion) {
    log_size_bench(crit, "Transpose square size", &SIZES, move |bench, size| {
        let row_size = size / 2;
        let src: Vec<_> = (0..size).map(FieldElement::from).collect();
        let mut dst = src.clone();
        bench.iter(|| transpose(&src, &mut dst, row_size))
    });
}

fn bench_size_ref(crit: &mut Criterion) {
    log_size_bench(
        crit,
        "Transpose base square size",
        &SIZES,
        move |bench, size| {
            let row_size = size / 2;
            let src: Vec<_> = (0..size).map(FieldElement::from).collect();
            let mut dst = src.clone();
            bench.iter(|| transpose_base(&src, &mut dst, row_size))
        },
    );
}

criterion_group!(group, bench_size, bench_size_ref);
