#[macro_use]
extern crate criterion;
extern crate fudd;

use criterion::black_box;
use criterion::Criterion;

use fudd::get_fudd;

pub fn criterion_benchmark_rabbit_fire_script(c: &mut Criterion) {
    let rabbit_fire_elmer_script = include_str!("./rabbit_fire_elmer_script_unfudded.txt");
    c.bench_function("rabbit_fire_script", |b| {
        b.iter(|| get_fudd(black_box(rabbit_fire_elmer_script)))
    });
}

pub fn criterion_benchmark_dictionary(criterion: &mut Criterion) {
    let word_list = include_str!("./words.txt");
    criterion
        .sample_size(20)
        .bench_function("dictionary", |b| b.iter(|| get_fudd(black_box(word_list))));
}

criterion_group!(long_test, criterion_benchmark_dictionary);
criterion_group!(benches, criterion_benchmark_rabbit_fire_script);
criterion_main!(benches, long_test);
