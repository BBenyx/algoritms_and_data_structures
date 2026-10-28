use std::{error::Error, time::Instant};
use sorting_algorithms::sorting::{
    bubble_sort::bubble_sort,
    shaker_sort::shaker_sort,
    insertion_sort::insertion_sort,
    selection_sort::selection_sort,
    heap_sort::heap_sort,
};

mod test_cases;
mod csv_writer;

type ListCreator = fn(usize) -> Vec<i32>;
type ListSorter<T> = fn(&mut [T]);

struct TestConfiguration<T> {
    range: std::ops::Range<usize>,
    step: usize, // TODO: needs validation (it cannot be smaller then 0)
    sample_size: usize, // TODO: needs validation (it cannot be smaller then 0)
    algorithms: Vec<(String, ListSorter<T>)>, // TODO: Try replacing String with &'static str
    distributions: Vec<(String, ListCreator)>, // TODO: Try replacing String with &'static str
}

impl<T> TestConfiguration<T> {
    fn header(&self) -> Vec<String> {
        [
            "Sorting algorithm",
            "Input size (n)",
            "Distribution",
            "Mean run time (ns)",
            "Sample size for mean"
        ].into_iter().map(String::from).collect()
    }
}

impl<T: Ord> Default for TestConfiguration<T> {
    fn default() -> Self {
        Self {
            range: 100..10000,
            step: 200,
            sample_size: 10,
            algorithms: vec![
                ("Bubble sort".to_string(), bubble_sort),
                ("Shaker sort".to_string(), shaker_sort),
                ("Insertion sort".to_string(), insertion_sort),
                ("Selection sort".to_string(), selection_sort),
                ("Heap sort".to_string(), heap_sort),
            ],
            distributions: vec![
                ("Ordered".to_string(), test_cases::ordered),
                ("Reverse".to_string(), test_cases::reverse_ordered),
                ("Random".to_string(), test_cases::randoms),
            ]
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    // Creates the results folder if it doesn't exists
    std::fs::create_dir_all("results")?;

    let csv_data = write_builder(TestConfiguration::default());

    csv_writer::write_to_csv(
        "results/Sorting_algorithms_runtimesN".to_string(),
        csv_data
    )?;

    Ok(())
}

// TODO: try to solve the too much clone
fn write_builder(configuration: TestConfiguration<i32>) -> Vec<Vec<String>> {
    let range = configuration.range.clone();
    let step = configuration.step;
    let sample_size = configuration.sample_size;

    let mut table = vec![configuration.header()];

    let distributions = configuration.distributions;
    let algrorithms = configuration.algorithms;

    for n in range.step_by(step) {

        for (distr_name, distr) in distributions.clone() {
            let test_list = distr(n);

            for (algr_name, algr) in algrorithms.clone() {
                let mut row = Vec::new();

                row.push(algr_name);
                row.push(n.to_string());
                row.push(distr_name.clone());

                let data: Vec<Vec<i32>> = (0..sample_size)
                    .map(|_| test_list.clone())
                    .collect();

                row.push(mean_run_time_of(algr, data).to_string());
                row.push(sample_size.to_string());

                table.push(row);
            }
        }
    }
    table
}

fn mean_run_time_of<T: Ord>(func: ListSorter<T>, list: Vec<Vec<T>>) -> u128 {
    assert!(!list.is_empty()); // TODO: try to solve this without panic (probably: Option)
    let mut runtimes: Vec<u128> = Vec::new();

    for mut elem in list {
        let start = Instant::now();
        func(&mut elem);
        runtimes.push(start.elapsed().as_nanos());
    }
    runtimes.sort();

    // Taking the trimmed mean

    let trim_val = if runtimes.len() >= 10 {
        (runtimes.len() as f64 * 0.25) as usize
    } else {
        0
    };

    let sum: u128 = runtimes[trim_val..runtimes.len() - trim_val].iter().sum();

    sum / (runtimes.len() - 2* trim_val) as u128
}



