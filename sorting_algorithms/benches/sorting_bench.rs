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

type ListSorter<T> = fn(&mut [T]);

fn main() -> Result<(), Box<dyn Error>>{
    // Creates the results folder if it doesn't exists
    std::fs::create_dir_all("results")?;

    let csv_data = write_builder(100, 1000, 200);

    csv_writer::write_to_csv(
        "results/Sorting_algorithms_runtimesN".to_string(),
        csv_data
    )?;

    Ok(())
}

fn write_builder(from: usize, to: usize , step: usize) -> Vec<Vec<String>> {
    assert!(from < to);
    let mut datas = vec![
        ["Input (n)","Ordered (bubble sort)","Ordered (shaker sort)","Ordered (insertion sort)","Ordered (selection sort)","Ordered (heap sort)",
        "Reverse ordered (bubble sort)","Reverse ordered (shaker sort)","Reverse ordered (insertion sort)","Reverse ordered (selection sort)", "Reverse ordered (heap sort)",
        "Random (bubble sort)","Random (shaker sort)","Random (insertion sort)","Random (selection sort)","Random (heap sort)"].
        into_iter().map(String::from).collect()];

    let list_creators = [test_cases::ordered, test_cases::reverse_ordered, test_cases::randoms];
    let sorters: [ListSorter<i32>; 5] = [bubble_sort, shaker_sort, insertion_sort, selection_sort, heap_sort];

    let to = to - ((to - from) % step);
    for n in (from..to).step_by(step) {
        let mut row = Vec::new();
        row.push(n.to_string());

        for creator in list_creators {
            let list = creator(n);
            let data: Vec<Vec<i32>> = (0..10)
                .map(|_| list.clone())
                .collect();

            for sorter in sorters {
                row.push(mean_run_time_of(sorter, data.clone()).to_string());
            }
        }
        datas.push(row);
    }
    datas
}

fn mean_run_time_of<T: PartialOrd>(func: ListSorter<T>, list: Vec<Vec<T>>) -> u128 {
    let mut runtimes: Vec<u128> = Vec::new();

    for mut elem in list {
        let start = Instant::now();
        func(&mut elem);
        runtimes.push(start.elapsed().as_nanos());
    }
    runtimes.sort();

    // Taking the trimmed mean

    let trim_val = if runtimes.len() >= 4 {
        (runtimes.len() as f64 * 0.25) as usize
    } else {
        0
    };
    
    let sum: u128 = runtimes[trim_val..runtimes.len() - trim_val].iter().sum();

    sum / (runtimes.len() - 2* trim_val) as u128
}



