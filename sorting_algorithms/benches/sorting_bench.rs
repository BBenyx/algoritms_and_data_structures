use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, BatchSize};
use std::hint::black_box;
use sorting_algorithms::sorting::{
    bubble_sort::bubble_sort,
    shaker_sort::shaker_sort,
    insertion_sort::insertion_sort,
    selection_sort::selection_sort,
    heap_sort::heap_sort,
};

mod test_cases;

type ListCreator = fn(usize) -> Vec<u32>;
type ListSorter<T> = fn(&mut [T]);

struct TestConfiguration<T> {
    range: std::ops::Range<usize>,
    step: usize,
    sample_size: usize,
    algorithms: Vec<(&'static str, ListSorter<T>)>,
    distributions: Vec<(&'static str, ListCreator)>,
}

impl<T: Ord> Default for TestConfiguration<T> {
    fn default() -> Self {
        Self {
            range: 100..4000,
            step: 300,
            sample_size: 10,
            algorithms: vec![
                ("Bubble sort", bubble_sort),
                ("Shaker sort", shaker_sort),
                ("Insertion sort", insertion_sort),
                ("Selection sort", selection_sort),
                ("Heap sort", heap_sort),
            ],
            distributions: vec![
                ("Ordered", test_cases::ordered),
                ("Reverse", test_cases::reverse_ordered),
                ("Random", test_cases::randoms),
            ]
        }
    }
}

fn algorithm_tester(c: &mut Criterion) {
    let config: TestConfiguration<u32> = TestConfiguration::default();

    let mut group = c.benchmark_group("sorting");
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