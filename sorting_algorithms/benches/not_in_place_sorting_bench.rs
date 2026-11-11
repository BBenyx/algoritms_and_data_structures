use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod support;
use support::test_configuration::{SorterType, TestConfiguration};

fn not_in_place_sorter_tester(c: &mut Criterion) {
    let config = TestConfiguration::<u32>::non_in_place_sorters();

    let mut group = c.benchmark_group("Not in place sorting");
    group.sample_size(config.sample_size);

    for n in config.range.step_by(config.step) {
        for (dist_name, distribution) in &config.distributions {
            let test_list = distribution(n);

            for (alg_name, algorithm) in &config.algorithms {
                let benchmark_id = BenchmarkId::new(format!("{alg_name} - {dist_name}"), n);

                if let SorterType::NotInPlace(algorithm) = algorithm {
                    group.bench_function(benchmark_id, |b| {
                        b.iter_batched(
                            || &test_list,
                            |data| algorithm(black_box(data.as_slice())),
                            BatchSize::LargeInput,
                        )
                    });
                }
            }
        }
    }
    group.finish();
}

criterion_group!(benches, not_in_place_sorter_tester);
criterion_main!(benches);
