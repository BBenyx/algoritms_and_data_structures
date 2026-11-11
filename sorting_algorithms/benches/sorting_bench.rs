use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, BatchSize};
use std::hint::black_box;

mod support;

fn algorithm_tester(c: &mut Criterion) {
    let config = support::test_configuration::TestConfiguration::<u32>::in_place_sorters();

    let mut group = c.benchmark_group("In place sorting");
    group.sample_size(config.sample_size);

    for n in config.range.step_by(config.step) {
        for (dist_name, distribution) in &config.distributions {
            let test_list = distribution(n);

            for (alg_name, algorithm) in &config.algorithms {
                let benchmark_id = BenchmarkId::new(
                    format!("{alg_name} - {dist_name}"),
                    n,
                );

                group.bench_function(benchmark_id, |b| {
                    b.iter_batched(
                        || test_list.clone(),
                        |mut data| {
                            algorithm(black_box(data.as_mut_slice()))
                        },
                        BatchSize::LargeInput,
                    )
                });
            }
        }
    }
    group.finish();
}

criterion_group!(benches, algorithm_tester);
criterion_main!(benches);