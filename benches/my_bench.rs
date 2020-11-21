#[macro_use]
extern crate criterion;
use criterion::Criterion;
use shitamachi04::max;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("my benchmark", |b| b.iter(
        || {
            let mut input = std::iter::repeat(0)
                .take(10000)
                .collect::<Vec<i32>>();
            input.push(10);
            max(input);
        }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);