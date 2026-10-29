use std::{error::Error, time::Instant, env};
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
    step: usize,
    sample_size: usize,
    algorithms: Vec<(&'static str, ListSorter<T>)>,
    distributions: Vec<(&'static str, ListCreator)>,
}

impl<T> TestConfiguration<T> {
    fn header(&self) -> Vec<String> {
        [
            "Sorting algorithm",
            "Input size (n)",
            "Distribution",
            "Mean runtime (ns)",
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


fn main() -> Result<(), Box<dyn Error>>{
    let arg = match env::args().nth(1) {
        Some(val) => val,
        None => return Err("No file name parameter was given!")?
    };
    let file_name = arg.trim().trim_matches('/').split('/').nth(0).unwrap();
    if file_name.len() == 0 { return Err("The given file name parameter cannot be a file name!")?}
    println!("{}", file_name);

    let csv_data = write_builder(TestConfiguration::default());

    // Creates the results folder if it doesn't exists
    std::fs::create_dir_all("data")?;
    csv_writer::write_to_csv(
        format!("data/{}", file_name),
        csv_data
    )?;

    Ok(())
}


fn write_builder(configuration: TestConfiguration<i32>) -> Vec<Vec<String>> {
    let step = configuration.step;
    let sample_size = configuration.sample_size;
    if step == 0 || step == 0 {
        return vec![Vec::new()]
    }

    let range = configuration.range.clone();

    let mut table = vec![configuration.header()];

    let distributions = configuration.distributions;
    let algrorithms = configuration.algorithms;

    for n in range.step_by(step) {

        for (distr_name, distr) in &distributions {
            let test_list = distr(n);

            for (algr_name, algr) in &algrorithms {
                let mut row = Vec::new();

                row.push(algr_name.to_string());
                row.push(n.to_string());
                row.push(distr_name.to_string());

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

fn mean_run_time_of<T: Ord>(func: &ListSorter<T>, list: Vec<Vec<T>>) -> u128 {
    assert!(!list.is_empty());
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